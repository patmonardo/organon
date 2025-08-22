import type { Command, Event } from '../../../absolute/core/message';
import type { EventBus } from '../../../absolute/core/bus';
import { InMemoryEventBus } from '../../../absolute/core/bus';
import { startTrace, childSpan } from '../../../absolute/core/trace';
import type { Repository } from '../../../repository/repo';
import {
  type Context,
  ContextSchema,
  createContext,
  updateContext,
} from '../../../schema/context';
import * as active from '../../../schema/active';

type BaseState = Context['shape']['state'];

type ContextCreateCmd = {
  kind: 'context.create';
  payload: Parameters<typeof createContext>[0];
  meta?: Record<string, unknown>;
};
type ContextDeleteCmd = {
  kind: 'context.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type ContextSetCoreCmd = {
  kind: 'context.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type ContextSetStateCmd = {
  kind: 'context.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type ContextPatchStateCmd = {
  kind: 'context.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};
type ContextDescribeCmd = {
  kind: 'context.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ContextCommand =
  | ContextCreateCmd
  | ContextDeleteCmd
  | ContextSetCoreCmd
  | ContextSetStateCmd
  | ContextPatchStateCmd
  | ContextDescribeCmd;

class MemContext {
  constructor(public doc: Context) {}
  get id() {
    return this.doc.shape.core.id;
  }
  get type() {
    return this.doc.shape.core.type;
  }
  get name() {
    return this.doc.shape.core.name ?? null;
  }
  setCore(core: { name?: string; type?: string }) {
    this.doc = ContextSchema.parse(
      updateContext(this.doc, {
        core: {
          ...(core.name !== undefined ? { name: core.name } : {}),
          ...(core.type !== undefined ? { type: core.type } : {}),
        },
      } as any),
    );
  }
  setState(state: BaseState) {
    this.doc = ContextSchema.parse(updateContext(this.doc, { state } as any));
  }
  patchState(patch: Partial<BaseState>) {
    this.doc = ContextSchema.parse(updateContext(this.doc, { state: patch } as any));
  }
  toSchema() {
    return this.doc;
  }
}

export class ContextEngine {
  private readonly contexts = new Map<string, MemContext>();

  constructor(
    private readonly repo?: Repository<Context>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'context',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  getContext(id: string): MemContext | undefined {
    return this.contexts.get(id);
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private mustGet(id: string): MemContext {
    const c = this.getContext(id);
    if (!c) throw new Error(`Context not found: ${id}`);
    return c;
  }

  private async persist(c: MemContext) {
    if (!this.repo) return;
    const id = c.id;
    const doc = ContextSchema.parse(c.toSchema());
    const current = await this.repo.get(id);
    if (current) await this.repo.update(id, () => doc);
    else await this.repo.create(doc);
  }

  async handle(cmd: ContextCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'ContextEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'context.create': {
        const { payload } = cmd as ContextCreateCmd;
        const created = ContextSchema.parse(createContext(payload as any));
        const ctx = new MemContext(created);
        this.contexts.set(ctx.id, ctx);
        await this.persist(ctx);
        return [
          this.emit(base, 'context.create', {
            id: ctx.id,
            type: ctx.type,
            name: ctx.name,
          }),
        ];
      }

      case 'context.delete': {
        const { id } = (cmd as ContextDeleteCmd).payload;
        const existed = this.contexts.delete(id);
        if (this.repo) await this.repo.delete(id);
        return [this.emit(base, 'context.delete', { id, ok: existed })];
      }

      case 'context.setCore': {
        const { id, name, type } = (cmd as ContextSetCoreCmd).payload;
        const c = this.mustGet(id);
        c.setCore({ name, type });
        await this.persist(c);
        return [
          this.emit(base, 'context.setCore', {
            id,
            name: c.name,
            type: c.type,
          }),
        ];
      }

      case 'context.setState': {
        const { id, state } = (cmd as ContextSetStateCmd).payload;
        const c = this.mustGet(id);
        c.setState(state);
        await this.persist(c);
        return [this.emit(base, 'context.setState', { id })];
      }

      case 'context.patchState': {
        const { id, patch } = (cmd as ContextPatchStateCmd).payload;
        const c = this.mustGet(id);
        c.patchState(patch);
        await this.persist(c);
        return [this.emit(base, 'context.patchState', { id })];
      }

      case 'context.describe': {
        const { id } = (cmd as ContextDescribeCmd).payload;
        const c = this.mustGet(id);
        const doc = c.toSchema();
        return [
          this.emit(base, 'context.describe', {
            id,
            type: c.type,
            name: c.name,
            state: doc.shape.state,
            signatureKeys: Object.keys(
              (doc.shape as any).signature ?? ({} as Record<string, unknown>),
            ),
            facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-like interface to mirror ShapeEngine
  async process(
    contexts: Array<active.ActiveContext>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = active.parseActiveContexts(contexts);
    const actions: any[] = [];
    for (const c of list) {
      if (c.revoked === true) {
        if (c.id) actions.push({ type: 'context.delete', id: c.id });
        continue;
      }
      actions.push({
        type: 'context.upsert',
        id: c.id ?? childIdFromName(c.name),
        name: c.name,
        kind: c.kind ?? 'system.Context',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'context.delete') {
        const [evt] = await this.handle({
          kind: 'context.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'context.upsert') {
        const id = a.id as string;
        if (!this.getContext(id)) {
          const [evt] = await this.handle({
            kind: 'context.create',
            payload: { id, type: a.kind, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'context.setCore',
            payload: { id, type: a.kind, name: a.name },
          } as any);
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  if (!name) return `context:${Math.random().toString(36).slice(2, 10)}`;
  return `context:${name.toLowerCase().replace(/\s+/g, '-')}`;
}
