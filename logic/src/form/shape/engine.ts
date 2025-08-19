import type { Command, Event } from '../triad/message';
import type { EventBus } from '../triad/bus';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';
import type { Repository } from '../../repository/repo';
import { ShapeSchema, type Shape } from '../../schema/shape';
import { FormShape } from './shape';
import * as active from '../../schema/active';

type BaseState = Shape['shape']['state'];
type Signature = NonNullable<Shape['shape']['signature']>;
type Facets = Shape['shape']['facets'];

type ShapeCreateCmd = {
  kind: 'shape.create';
  payload: Parameters<typeof FormShape.create>[0];
  meta?: Record<string, unknown>;
};
type ShapeDeleteCmd = {
  kind: 'shape.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type ShapeSetCoreCmd = {
  kind: 'shape.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type ShapeSetSignatureCmd = {
  kind: 'shape.setSignature';
  payload: { id: string; signature?: Signature };
  meta?: Record<string, unknown>;
};
type ShapeMergeSignatureCmd = {
  kind: 'shape.mergeSignature';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};
type ShapeSetFacetsCmd = {
  kind: 'shape.setFacets';
  payload: { id: string; facets: Facets };
  meta?: Record<string, unknown>;
};
type ShapeMergeFacetsCmd = {
  kind: 'shape.mergeFacets';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};
type ShapeSetStateCmd = {
  kind: 'shape.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type ShapePatchStateCmd = {
  kind: 'shape.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};
type ShapeDescribeCmd = {
  kind: 'shape.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ShapeCommand =
  | ShapeCreateCmd
  | ShapeDeleteCmd
  | ShapeSetCoreCmd
  | ShapeSetSignatureCmd
  | ShapeMergeSignatureCmd
  | ShapeSetFacetsCmd
  | ShapeMergeFacetsCmd
  | ShapeSetStateCmd
  | ShapePatchStateCmd
  | ShapeDescribeCmd;

export class ShapeEngine {
  private readonly shapes = new Map<string, FormShape>();

  constructor(
    private readonly repo?: Repository<Shape>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'shape',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  getShape(id: string): FormShape | undefined {
    return this.shapes.get(id);
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private mustGet(id: string): FormShape {
    const s = this.getShape(id);
    if (!s) throw new Error(`Shape not found: ${id}`);
    return s;
  }

  private async persist(s: FormShape) {
    if (!this.repo) return;
    const id = s.id;
    const doc = ShapeSchema.parse(s.toSchema());
    const current = await this.repo.get(id);
    if (current) await this.repo.update(id, () => doc);
    else await this.repo.create(doc);
  }

  async handle(cmd: ShapeCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'ShapeEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'shape.create': {
        const { payload } = cmd as ShapeCreateCmd;
        const shape = FormShape.create(payload);
        this.shapes.set(shape.id, shape);
        await this.persist(shape);
        return [
          this.emit(base, 'shape.created', {
            id: shape.id,
            type: shape.type,
            name: shape.name ?? null,
          }),
        ];
      }

      case 'shape.delete': {
        const { id } = (cmd as ShapeDeleteCmd).payload;
        const existed = this.shapes.delete(id);
        if (this.repo) await this.repo.delete(id);
        return [this.emit(base, 'shape.deleted', { id, ok: existed })];
      }

      case 'shape.setCore': {
        const { id, name, type } = (cmd as ShapeSetCoreCmd).payload;
        const s = this.mustGet(id);
        if (name !== undefined) s.setName(name);
        if (type !== undefined) s.setType(type);
        await this.persist(s);
        return [
          this.emit(base, 'shape.core.set', {
            id,
            name: s.name ?? null,
            type: s.type,
          }),
        ];
      }

      case 'shape.setSignature': {
        const { id, signature } = (cmd as ShapeSetSignatureCmd).payload;
        const s = this.mustGet(id);
        s.setSignature(signature);
        await this.persist(s);
        return [this.emit(base, 'shape.signature.set', { id })];
      }

      case 'shape.mergeSignature': {
        const { id, patch } = (cmd as ShapeMergeSignatureCmd).payload;
        const s = this.mustGet(id);
        s.patchSignature(patch);
        await this.persist(s);
        return [this.emit(base, 'shape.signature.merged', { id })];
      }

      case 'shape.setFacets': {
        const { id, facets } = (cmd as ShapeSetFacetsCmd).payload;
        const s = this.mustGet(id);
        s.setFacets(facets);
        await this.persist(s);
        return [this.emit(base, 'shape.facets.set', { id })];
      }

      case 'shape.mergeFacets': {
        const { id, patch } = (cmd as ShapeMergeFacetsCmd).payload;
        const s = this.mustGet(id);
        s.mergeFacets(patch);
        await this.persist(s);
        return [this.emit(base, 'shape.facets.merged', { id })];
      }

      case 'shape.setState': {
        const { id, state } = (cmd as ShapeSetStateCmd).payload;
        const s = this.mustGet(id);
        s.setState(state);
        await this.persist(s);
        return [
          this.emit(base, 'shape.state.set', { id, status: s.state.status }),
        ];
      }

      case 'shape.patchState': {
        const { id, patch } = (cmd as ShapePatchStateCmd).payload;
        const s = this.mustGet(id);
        s.patchState(patch);
        await this.persist(s);
        return [
          this.emit(base, 'shape.state.patched', {
            id,
            status: s.state.status,
          }),
        ];
      }

      case 'shape.describe': {
        const { id } = (cmd as ShapeDescribeCmd).payload;
        const s = this.mustGet(id);
        const doc = s.toSchema();
        return [
          this.emit(base, 'shape.described', {
            id,
            type: s.type,
            name: s.name ?? null,
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

  // ADR 0002 ShapeEngine interface: process/commit
  async process(
    shapes: Array<active.ActiveShape>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    // Validate/normalize via Zod; this clamps confidence and checks ids
    const list = active.parseActiveShapes(shapes);
    // For now, produce deterministic actions that map to our command space
    const actions: any[] = [];
    for (const s of list) {
      // Upsert behavior based on revoked/active
      if (s.revoked === true) {
        if (s.id) actions.push({ type: 'shape.delete', id: s.id, sourceShapeId: s.id });
        continue;
      }
      actions.push({
        type: 'shape.upsert',
        id: s.id ?? childIdFromName(s.name),
        name: s.name,
        kind: s.kind ?? 'shape',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'shape.delete') {
        const [evt] = await this.handle({ kind: 'shape.delete', payload: { id: a.id } } as any);
        events.push(evt);
      } else if (a.type === 'shape.upsert') {
        // Upsert → create or set core
        const id = a.id as string;
        if (!this.getShape(id)) {
          const [evt] = await this.handle({ kind: 'shape.create', payload: { id, type: a.kind, name: a.name } } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({ kind: 'shape.setCore', payload: { id, type: a.kind, name: a.name } } as any);
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  if (!name) return `shape:${Math.random().toString(36).slice(2, 10)}`;
  return `shape:${name.toLowerCase().replace(/\s+/g, '-')}`;
}
