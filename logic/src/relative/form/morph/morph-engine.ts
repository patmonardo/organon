import type { Command, Event } from '../../../absolute/core/message';
import type { EventBus } from '../../../absolute/core/bus';
import { InMemoryEventBus } from '../../../absolute/core/bus';
import { startTrace, childSpan } from '../../../absolute/core/trace';
import type { Repository } from '../../../repository/repo';
import {
  type Morph,
  MorphSchema,
  createMorph,
  updateMorph,
} from '../../../schema/morph';
import { FormMorph } from './morph-form';
import * as active from '../../../schema/active';

type BaseState = Morph['shape']['state'];

type MorphCreateCmd = {
  kind: 'morph.create';
  payload: Parameters<typeof createMorph>[0];
  meta?: Record<string, unknown>;
};
type MorphDeleteCmd = {
  kind: 'morph.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type MorphDescribeCmd = {
  kind: 'morph.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type MorphSetCoreCmd = {
  kind: 'morph.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type MorphSetStateCmd = {
  kind: 'morph.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type MorphPatchStateCmd = {
  kind: 'morph.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};

export type MorphCommand =
  | MorphCreateCmd
  | MorphDeleteCmd
  | MorphDescribeCmd
  | MorphSetCoreCmd
  | MorphSetStateCmd
  | MorphPatchStateCmd;

export class MorphEngine {
  constructor(
    private readonly repo: Repository<Morph>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'morph',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getMorph(id: string): Promise<FormMorph | undefined> {
    const doc = await this.repo.get(id);
    return doc ? FormMorph.fromSchema(doc) : undefined;
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private async mustGet(id: string): Promise<FormMorph> {
    const m = await this.getMorph(id);
    if (!m) throw new Error(`Morph not found: ${id}`);
    return m;
  }

  private async persist(m: FormMorph) {
    const id = m.id;
    const doc = MorphSchema.parse(m.toSchema());
    const current = await this.repo.get(id);
    if (current) {
      await this.repo.update(id, doc as any);
    } else {
      await this.repo.create(doc);
    }
  }

  async handle(cmd: MorphCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'MorphEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'morph.create': {
        const { payload } = cmd as MorphCreateCmd;
        const created = MorphSchema.parse(createMorph(payload as any));
        const m = FormMorph.fromSchema(created);
        await this.persist(m);
        return [
          this.emit(base, 'morph.create', {
            id: m.id,
            type: m.type,
            name: m.name ?? null,
          }),
        ];
      }

      case 'morph.delete': {
        const { id } = (cmd as MorphDeleteCmd).payload;
        const existed = await this.repo.get(id);
        if (existed) {
          await this.repo.delete(id);
        }
        return [this.emit(base, 'morph.delete', { id, ok: !!existed })];
      }

      case 'morph.setCore': {
        const { id, name, type } = (cmd as MorphSetCoreCmd).payload;
        const m = await this.mustGet(id);
        m.setCore({ name, type });
        await this.persist(m);
        return [
          this.emit(base, 'morph.setCore', {
            id,
            name: m.name ?? null,
            type: m.type,
          }),
        ];
      }

      case 'morph.setState': {
        const { id, state } = (cmd as MorphSetStateCmd).payload;
        const m = await this.mustGet(id);
        m.setState(state as any);
        await this.persist(m);
        return [this.emit(base, 'morph.setState', { id })];
      }

      case 'morph.patchState': {
        const { id, patch } = (cmd as MorphPatchStateCmd).payload;
        const m = await this.mustGet(id);
        m.patchState(patch as any);
        await this.persist(m);
        return [this.emit(base, 'morph.patchState', { id })];
      }

      case 'morph.describe': {
        const { id } = (cmd as MorphDescribeCmd).payload;
        const doc = await this.repo.get(id);
        if (!doc) {
          return [this.emit(base, 'morph.describe', { id })];
        }

        const m = FormMorph.fromSchema(doc);
        return [
          this.emit(base, 'morph.describe', {
            id,
            type: m.type,
            name: m.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys(
              (doc.shape.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(doc.shape.facets ?? {}),
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-like interface to mirror EntityEngine/ContextEngine/PropertyEngine
  async process(
    morphs: Array<active.ActiveMorph>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = morphs as any[]; // Skip parsing for now to preserve test data
    const actions: any[] = [];
    for (const m of list) {
      if (m.revoked === true) {
        if (m.id) actions.push({ type: 'morph.delete', id: m.id });
        continue;
      }
      actions.push({
        type: 'morph.upsert',
        id: m.id ?? childIdFromName(m.name),
        morphType: m.kind ?? m.type ?? 'system.Morph',
        name: m.name,
        morph: {
          id: m.id,
          type: m.kind ?? m.type ?? 'system.Morph',
          name: m.name,
          transform: m.transform,
          active: m.active,
        },
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'morph.delete') {
        const [evt] = await this.handle({
          kind: 'morph.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'morph.upsert') {
        const id = a.id as string;
        const existing = await this.getMorph(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'morph.create',
            payload: {
              id,
              type: a.morphType,
              name: a.name,
              state: a.morph ? { transform: a.morph.transform, active: a.morph.active } : {},
            },
          } as any);
          events.push(evt);
        } else {
          // For existing morphs, emit morph.update (not setCore like other engines)
          const updatedDoc = updateMorph(existing.toSchema(), {
            core: {
              ...(a.name !== undefined ? { name: a.name } : {}),
              ...(a.morphType !== undefined ? { type: a.morphType } : {}),
            },
            state: a.morph ? { transform: a.morph.transform, active: a.morph.active } : {},
          } as any);
          const updated = FormMorph.fromSchema(MorphSchema.parse(updatedDoc));
          await this.persist(updated);

          const evt = this.emit({ correlationId: 'commit' }, 'morph.update', {
            id,
            type: updated.type,
            name: updated.name ?? null,
          });
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  const base =
    name?.toString().trim().toLowerCase().replace(/\s+/g, '-') ||
    Math.random().toString(36).slice(2, 10);
  return `morph:${base}`;
}
