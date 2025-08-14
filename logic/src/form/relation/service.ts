import type { Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import type { Repository } from '../../repository/repo';
import {
  RelationSchema,
  type Relation,
  createRelation,
  updateRelation,
} from '../../schema/relation';

export type RelationId = string;

type NodeRef = { id: string; type: string };

export class RelationService {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, Relation>();

  constructor(private readonly repo?: Repository<Relation>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Event API
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Reads
  async get(id: RelationId): Promise<Relation | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? RelationSchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? RelationSchema.parse(doc) : undefined;
  }

  // SDK verbs

  async create(input: {
    type: string;
    name?: string;
    kind: string;
    source: NodeRef;
    target: NodeRef;
    direction?: 'directed' | 'bidirectional';
  }) {
    const doc = RelationSchema.parse(createRelation(input as any));
    await this.persist(doc);
    this.bus.publish({
      kind: 'relation.created',
      payload: {
        id: doc.shape.core.id,
        type: doc.shape.core.type,
        name: doc.shape.core.name ?? null,
        kind:
          (doc.shape as any).core?.kind ??
          (doc.shape as any).kind ??
          input.kind,
        direction: (doc.shape as any).direction,
        sourceId: (doc.shape as any).source?.id,
        targetId: (doc.shape as any).target?.id,
      },
    });
    return doc.shape.core.id as RelationId;
  }

  async delete(id: RelationId) {
    const existed = await this.remove(id);
    this.bus.publish({
      kind: 'relation.deleted',
      payload: { id, ok: existed },
    });
  }

  async describe(id: RelationId) {
    const doc = await this.mustGet(id);
    const payload = {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name ?? null,
      kind: (doc.shape as any).core?.kind ?? (doc.shape as any).kind,
      direction: (doc.shape as any).direction,
      source: (doc.shape as any).source,
      target: (doc.shape as any).target,
      state: doc.shape.state,
    };
    this.bus.publish({ kind: 'relation.described', payload });
    return payload;
  }

  async setCore(
    id: RelationId,
    core: { name?: string; type?: string; kind?: string },
  ) {
    const curr = await this.mustGet(id);
    const next = RelationSchema.parse(updateRelation(curr, { core } as any));
    await this.persist(next);
    this.bus.publish({
      kind: 'relation.core.set',
      payload: {
        id,
        name: next.shape.core.name ?? null,
        type: next.shape.core.type,
        kind: (next.shape as any).core?.kind ?? (next.shape as any).kind,
      },
    });
  }

  async setEndpoints(
    id: RelationId,
    endpoints: { source: NodeRef; target: NodeRef },
  ) {
    if (!endpoints.source?.type || !endpoints.target?.type) {
      throw new Error('Relation endpoints require { id, type }');
    }
    const curr = await this.mustGet(id);
    const next = RelationSchema.parse(
      updateRelation(curr, {
        source: endpoints.source,
        target: endpoints.target,
      } as any),
    );
    await this.persist(next);
    this.bus.publish({
      kind: 'relation.endpoints.set',
      payload: {
        id,
        sourceId: (next.shape as any).source?.id,
        targetId: (next.shape as any).target?.id,
      },
    });
  }

  async setDirection(id: RelationId, direction: 'directed' | 'bidirectional') {
    const curr = await this.mustGet(id);
    const next = RelationSchema.parse(
      updateRelation(curr, { direction } as any),
    );
    await this.persist(next);
    this.bus.publish({
      kind: 'relation.direction.set',
      payload: { id, direction: (next.shape as any).direction },
    });
  }

  async setState(id: RelationId, state: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = RelationSchema.parse(updateRelation(curr, { state } as any));
    await this.persist(next);
    this.bus.publish({ kind: 'relation.state.set', payload: { id } });
  }

  async patchState(id: RelationId, patch: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = RelationSchema.parse(
      updateRelation(curr, { state: patch } as any),
    );
    await this.persist(next);
    this.bus.publish({ kind: 'relation.state.patched', payload: { id } });
  }

  // Internals

  private async mustGet(id: string): Promise<Relation> {
    const found = await this.get(id);
    if (!found) throw new Error(`Relation not found: ${id}`);
    return found;
  }

  private async persist(doc: Relation): Promise<void> {
    const id = doc.shape.core.id;
    if (this.repo) {
      const r: any = this.repo as any;
      const existing = await r.get(id);
      if (existing && typeof r.delete === 'function') {
        await r.delete(id);
      }
      await r.create(doc);
    } else {
      this.mem.set(id, doc);
    }
  }

  private async remove(id: string): Promise<boolean> {
    if (this.repo) {
      const existing = await this.repo.get(id);
      if (!existing) return false;
      await this.repo.delete(id);
      return true;
    }
    return this.mem.delete(id);
  }
}
