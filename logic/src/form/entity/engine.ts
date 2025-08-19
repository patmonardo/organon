import type { Command, Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import { startTrace, childSpan } from '../triad/trace';
import type { Repository, Concurrency } from '../../repository/repo';
import {
  type Entity,
  EntitySchema,
  createEntity,
  updateEntity,
} from '../../schema/entity';
import * as active from '../../schema/active';

// Typed command union (unified verbs)
type EntityCreateCmd = {
  kind: 'entity.create';
  payload: Parameters<typeof createEntity>[0];
  meta?: Record<string, unknown>;
};
type EntityDeleteCmd = {
  kind: 'entity.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type EntityDescribeCmd = {
  kind: 'entity.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type EntitySetCoreCmd = {
  kind: 'entity.setCore';
  payload: {
    id: string;
    name?: string;
    type?: string;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntitySetStateCmd = {
  kind: 'entity.setState';
  payload: {
    id: string;
    state: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntityPatchStateCmd = {
  kind: 'entity.patchState';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntitySetFacetsCmd = {
  kind: 'entity.setFacets';
  payload: {
    id: string;
    facets: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntityMergeFacetsCmd = {
  kind: 'entity.mergeFacets';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntitySetSignatureCmd = {
  kind: 'entity.setSignature';
  payload: {
    id: string;
    signature?: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};
type EntityMergeSignatureCmd = {
  kind: 'entity.mergeSignature';
  payload: {
    id: string;
    patch: Record<string, unknown>;
    expectedRevision?: Concurrency['expectedRevision'];
  };
  meta?: Record<string, unknown>;
};

export type EntityCommand =
  | EntityCreateCmd
  | EntityDeleteCmd
  | EntityDescribeCmd
  | EntitySetCoreCmd
  | EntitySetStateCmd
  | EntityPatchStateCmd
  | EntitySetFacetsCmd
  | EntityMergeFacetsCmd
  | EntitySetSignatureCmd
  | EntityMergeSignatureCmd;

/**
 * EntityEngine — repo-backed engine for Entity docs.
 * Unified Command verbs (create/delete/describe/set/patch)
 */
export class EntityEngine {
  constructor(
    private readonly repo: Repository<Entity>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'entity',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  private emit(
    base: Record<string, any>,
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

  private async mustGet(id: string): Promise<Entity> {
    const doc = await this.repo.get(id);
    if (!doc) throw new Error(`entity not found: ${id}`);
    return EntitySchema.parse(doc);
  }

  private async save(
    id: string,
    next: Entity,
    expectedRevision?: Concurrency['expectedRevision'],
  ): Promise<Entity> {
    const opts = expectedRevision == null ? undefined : { expectedRevision };
    const saved = await this.repo.update(
      id,
      () => EntitySchema.parse(next),
      opts,
    );
    return EntitySchema.parse(saved);
  }

  private async persist(doc: Entity): Promise<void> {
    const id = (doc as any).shape.core.id as string;
    const existing = await this.repo.get(id);
    if (existing != null) {
      // update expects (id, mutateFn, [concurrency])
      await this.repo.update(id, () => EntitySchema.parse(doc));
    } else {
      await this.repo.create(EntitySchema.parse(doc));
    }
  }

  async handle(cmd: EntityCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'EntityEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'entity.create': {
        const doc = EntitySchema.parse(
          createEntity((cmd as EntityCreateCmd).payload as any),
        );
        await this.persist(doc); // use upsert helper
        const evt = this.emit(base, 'entity.created', {
          id: doc.shape.core.id,
          type: doc.shape.core.type,
          name: doc.shape.core.name ?? null,
        });
        return [evt];
      }

      case 'entity.delete': {
        const { id } = (cmd as EntityDeleteCmd).payload;
        const ok = await this.repo.delete(id);
        const evt = this.emit(base, 'entity.deleted', { id, ok });
        return [evt];
      }

      case 'entity.describe': {
        const { id } = (cmd as EntityDescribeCmd).payload;
        const doc = await this.mustGet(id);
        const payload = {
          id,
          type: doc.shape.core.type,
          name: doc.shape.core.name ?? null,
          state: doc.shape.state,
          signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
          facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
        };
        const evt = this.emit(base, 'entity.described', payload);
        return [evt];
      }

      case 'entity.setCore': {
        const { id, name, type, expectedRevision } = (cmd as EntitySetCoreCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateEntity(curr, {
          core: {
            ...(name !== undefined ? { name } : {}),
            ...(type !== undefined ? { type } : {}),
          },
        } as any);
        const saved = await this.save(id, next as any, expectedRevision);
        return [
          this.emit(base, 'entity.core.set', {
            id,
            name: saved.shape.core.name ?? null,
            type: saved.shape.core.type,
          }),
        ];
      }

      case 'entity.setState': {
        const { id, state, expectedRevision } = (cmd as EntitySetStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateEntity(curr, { state } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.state.set', { id })];
      }

      case 'entity.patchState': {
        const { id, patch, expectedRevision } = (cmd as EntityPatchStateCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateEntity(curr, { state: patch } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.state.patched', { id })];
      }

      case 'entity.setFacets': {
        const { id, facets, expectedRevision } = (cmd as EntitySetFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const next = updateEntity(curr, { facets } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.facets.set', { id })];
      }

      case 'entity.mergeFacets': {
        const { id, patch, expectedRevision } = (cmd as EntityMergeFacetsCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).facets ?? {}), ...patch };
        const next = updateEntity(curr, { facets: merged } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.facets.merged', { id })];
      }

      case 'entity.setSignature': {
        const { id, signature, expectedRevision } = (
          cmd as EntitySetSignatureCmd
        ).payload;
        const curr = await this.mustGet(id);
        // Convention: undefined clears signature (via null sentinel for schema helper, if required)
        const next = updateEntity(curr, {
          signature: signature === undefined ? (null as any) : signature,
        } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.signature.set', { id })];
      }

      case 'entity.mergeSignature': {
        const { id, patch, expectedRevision } = (cmd as EntityMergeSignatureCmd)
          .payload;
        const curr = await this.mustGet(id);
        const merged = { ...((curr.shape as any).signature ?? {}), ...patch };
        const next = updateEntity(curr, { signature: merged } as any);
        await this.save(id, next as any, expectedRevision);
        return [this.emit(base, 'entity.signature.merged', { id })];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-0005: EntityEngine interface — process/commit
  async process(
    entities: Array<active.ActiveEntity>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = active.parseActiveEntities(entities);
    const actions: any[] = [];
    for (const e of list) {
      if (e.revoked === true) {
        actions.push({ type: 'entity.delete', id: e.id });
        continue;
      }
      actions.push({
        type: 'entity.upsert',
        id: e.id,
        entityType: e.entityType,
        labels: e.labels,
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'entity.delete') {
        const [evt] = await this.handle({
          kind: 'entity.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'entity.upsert') {
        const id = a.id as string;
        const existing = await this.repo.get(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'entity.create',
            payload: { id, type: a.entityType },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'entity.setCore',
            payload: { id, type: a.entityType },
          } as any);
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}
