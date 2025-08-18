import type { Command, Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';
import type { Repository, Concurrency } from '../../repository/repo';
import {
  type Context,
  ContextSchema,
  createContext,
  updateContext,
} from '../../schema/context';
import { EntityRef } from '../../schema/entity';
import { schemas } from '../../absolute/essence';

// Core CRUD and mutation commands
export type ContextCreateCmd = {
  kind: 'context.create';
  payload: Parameters<typeof createContext>[0];
  meta?: Record<string, unknown>;
};
export type ContextDeleteCmd = {
  kind: 'context.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type ContextDescribeCmd = {
  kind: 'context.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type ContextSetCoreCmd = {
  kind: 'context.setCore';
  payload: {
    id: string;
    name?: string;
    type?: string;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextSetStateCmd = {
  kind: 'context.setState';
  payload: {
    id: string;
    state: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextPatchStateCmd = {
  kind: 'context.patchState';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextSetFacetsCmd = {
  kind: 'context.setFacets';
  payload: {
    id: string;
    facets: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextMergeFacetsCmd = {
  kind: 'context.mergeFacets';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextSetSignatureCmd = {
  kind: 'context.setSignature';
  payload: {
    id: string;
    signature?: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
export type ContextMergeSignatureCmd = {
  kind: 'context.mergeSignature';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};

// Membership: entities
export type ContextAddEntityCmd = {
  kind: 'context.addEntity';
  payload: { id: string; ref: unknown };
  meta?: Record<string, unknown>;
};
export type ContextAddEntitiesCmd = {
  kind: 'context.addEntities';
  payload: { id: string; refs: unknown[] };
  meta?: Record<string, unknown>;
};
export type ContextRemoveEntityCmd = {
  kind: 'context.removeEntity';
  payload: { id: string; ref: unknown };
  meta?: Record<string, unknown>;
};
export type ContextClearEntitiesCmd = {
  kind: 'context.clearEntities';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

// Membership: relations (by id string)
export type ContextAddRelationCmd = {
  kind: 'context.addRelation';
  payload: { id: string; relationId: string };
  meta?: Record<string, unknown>;
};
export type ContextAddRelationsCmd = {
  kind: 'context.addRelations';
  payload: { id: string; relationIds: string[] };
  meta?: Record<string, unknown>;
};
export type ContextRemoveRelationCmd = {
  kind: 'context.removeRelation';
  payload: { id: string; relationId: string };
  meta?: Record<string, unknown>;
};
export type ContextClearRelationsCmd = {
  kind: 'context.clearRelations';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ContextCommand =
  | ContextCreateCmd
  | ContextDeleteCmd
  | ContextDescribeCmd
  | ContextSetCoreCmd
  | ContextSetStateCmd
  | ContextPatchStateCmd
  | ContextSetFacetsCmd
  | ContextMergeFacetsCmd
  | ContextSetSignatureCmd
  | ContextMergeSignatureCmd
  | ContextAddEntityCmd
  | ContextAddEntitiesCmd
  | ContextRemoveEntityCmd
  | ContextClearEntitiesCmd
  | ContextAddRelationCmd
  | ContextAddRelationsCmd
  | ContextRemoveRelationCmd
  | ContextClearRelationsCmd;

export class ContextEngine {
  constructor(
    private readonly repo: Repository<Context>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'context',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  private emit(
    base: Record<string, unknown>,
    kind: Event['kind'],
    payload: Event['payload'],
    extraMeta?: Record<string, unknown>,
  ): Event {
    const meta = childSpan(base as any, {
      action: kind,
      scope: this.scope,
      ...(extraMeta ?? {}),
    });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
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
        const created = await this.repo.create(
          ContextSchema.parse(createContext(payload as any)),
        );
        return [
          this.emit(base, 'context.created', {
            id: created.shape.core.id,
            type: created.shape.core.type,
            name: created.shape.core.name ?? null,
          }),
        ];
      }

      case 'context.delete': {
        const { id } = (cmd as ContextDeleteCmd).payload;
        const ok = await this.repo.delete(id);
        return [this.emit(base, 'context.deleted', { id, ok: !!ok })];
      }

      case 'context.describe': {
        const { id } = (cmd as ContextDescribeCmd).payload;
        const doc = await this.mustGet(id);
        return [
          this.emit(base, 'context.described', {
            id,
            type: doc.shape.core.type,
            name: doc.shape.core.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
            facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
            entitiesCount: (doc.shape.entities ?? []).length,
            relationsCount: (doc.shape.relations ?? []).length,
          }),
        ];
      }

      case 'context.setCore': {
        const { id, name, type, expectedRevision } = (cmd as ContextSetCoreCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateContext(curr, {
          core: {
            ...(name !== undefined ? { name } : {}),
            ...(type !== undefined ? { type } : {}),
          },
        } as any);
        const saved = await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [
          this.emit(base, 'context.core.set', {
            id,
            name: saved.shape.core.name ?? null,
            type: saved.shape.core.type,
          }),
        ];
      }

      case 'context.setState': {
        const { id, state, expectedRevision } = (cmd as ContextSetStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateContext(curr, { state } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.state.set', { id })];
      }

      case 'context.patchState': {
        const { id, patch, expectedRevision } = (cmd as ContextPatchStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateContext(curr, { state: patch } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.state.patched', { id })];
      }

      case 'context.setFacets': {
        const { id, facets, expectedRevision } = (cmd as ContextSetFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateContext(curr, { facets } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.facets.set', { id })];
      }

      case 'context.mergeFacets': {
        const { id, patch, expectedRevision } = (cmd as ContextMergeFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).facets ?? {}), ...patch };
        const next = updateContext(curr, { facets: merged } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.facets.merged', { id })];
      }

      case 'context.setSignature': {
        const { id, signature, expectedRevision } = (
          cmd as ContextSetSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const next = updateContext(curr, {
          signature: signature === undefined ? (null as any) : signature,
        } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.signature.set', { id })];
      }

      case 'context.mergeSignature': {
        const { id, patch, expectedRevision } = (
          cmd as ContextMergeSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).signature ?? {}), ...patch };
        const next = updateContext(curr, { signature: merged } as any);
        await this.repo.update(
          id,
          () => ContextSchema.parse(next),
          expectedRevision !== undefined ? { expectedRevision } : undefined,
        );
        return [this.emit(base, 'context.signature.merged', { id })];
      }

      // Entities
      case 'context.addEntity': {
        const { id, ref } = (cmd as ContextAddEntityCmd).payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const exists = new Set(
          current.shape.entities.map((e) => `${e.type}:${e.id}`),
        );
        const next = exists.has(`${parsed.type}:${parsed.id}`)
          ? current
          : updateContext(current, {
              entities: [...current.shape.entities, parsed],
            } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [
          this.emit(base, 'context.entity.added', { id, entity: parsed }),
        ];
      }

      case 'context.addEntities': {
        const { id, refs } = (cmd as ContextAddEntitiesCmd).payload;
        const current = await this.mustGet(id);
        const exists = new Set(
          current.shape.entities.map((e) => `${e.type}:${e.id}`),
        );
        const toAdd = (refs ?? [])
          .map((r: unknown) => EntityRef.parse(r))
          .filter((r) => !exists.has(`${r.type}:${r.id}`));
        const next =
          toAdd.length === 0
            ? current
            : updateContext(current, {
                entities: [...current.shape.entities, ...toAdd],
              } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [
          this.emit(base, 'context.entities.added', {
            id,
            count: toAdd.length,
          }),
        ];
      }

      case 'context.removeEntity': {
        const { id, ref } = (cmd as ContextRemoveEntityCmd).payload;
        const current = await this.mustGet(id);
        const parsed = EntityRef.parse(ref);
        const nextList = current.shape.entities.filter(
          (e) => !(e.id === parsed.id && e.type === parsed.type),
        );
        const next =
          nextList.length === current.shape.entities.length
            ? current
            : updateContext(current, { entities: nextList } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [
          this.emit(base, 'context.entity.removed', { id, entity: parsed }),
        ];
      }

      case 'context.clearEntities': {
        const { id } = (cmd as ContextClearEntitiesCmd).payload;
        const current = await this.mustGet(id);
        const next =
          current.shape.entities.length === 0
            ? current
            : updateContext(current, { entities: [] } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [this.emit(base, 'context.entities.cleared', { id })];
      }

      // Relations by id
      case 'context.addRelation': {
        const { id, relationId } = (cmd as ContextAddRelationCmd).payload;
        const current = await this.mustGet(id);
        const set = new Set<string>(current.shape.relations);
        const next = set.has(relationId)
          ? current
          : updateContext(current, {
              relations: [...current.shape.relations, relationId],
            } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [this.emit(base, 'context.relation.added', { id, relationId })];
      }

      case 'context.addRelations': {
        const { id, relationIds } = (cmd as ContextAddRelationsCmd).payload;
        const current = await this.mustGet(id);
        const set = new Set<string>(current.shape.relations);
        const toAdd = (relationIds ?? []).filter(
          (rid: string) => !set.has(rid),
        );
        const next =
          toAdd.length === 0
            ? current
            : updateContext(current, {
                relations: [...current.shape.relations, ...toAdd],
              } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [
          this.emit(base, 'context.relations.added', {
            id,
            count: toAdd.length,
          }),
        ];
      }

      case 'context.removeRelation': {
        const { id, relationId } = (cmd as ContextRemoveRelationCmd).payload;
        const current = await this.mustGet(id);
        const nextList = current.shape.relations.filter(
          (r: string) => r !== relationId,
        );
        const next =
          nextList.length === current.shape.relations.length
            ? current
            : updateContext(current, { relations: nextList } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [
          this.emit(base, 'context.relation.removed', { id, relationId }),
        ];
      }

      case 'context.clearRelations': {
        const { id } = (cmd as ContextClearRelationsCmd).payload;
        const current = await this.mustGet(id);
        const next =
          current.shape.relations.length === 0
            ? current
            : updateContext(current, { relations: [] } as any);
        await this.repo.update(id, () => ContextSchema.parse(next));
        return [this.emit(base, 'context.relations.cleared', { id })];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-0003: ContextEngine process/commit interface (class methods)
  async process(
    contexts: Array<schemas.ActiveContext>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = schemas.parseActiveContexts(contexts);
    const actions: any[] = [];
    for (const c of list) {
      if (c.revoked === true) {
        actions.push({ type: 'context.delete', id: c.id });
        continue;
      }
      actions.push({
        type: 'context.upsert',
        id: c.id,
        name: c.name,
        kind: c.kind,
        scope: c.scope,
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
        const id = a.id as string | undefined;
        if (!id) continue; // skip malformed
        const existing = await this.repo.get(id);
        if (!existing) {
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

  private async mustGet(id: string): Promise<Context> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`context not found: ${id}`);
    return ContextSchema.parse(doc);
  }
}
