import type { Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import type { Repository } from '../../repository/repo';
import {
  EntitySchema,
  type Entity,
  createEntity,
  updateEntity,
} from '../../schema/entity';

export type EntityId = string;

export class EntityService {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, Entity>();

  constructor(private readonly repo?: Repository<Entity>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Event API
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Reads
  async get(id: EntityId): Promise<Entity | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? EntitySchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? EntitySchema.parse(doc) : undefined;
  }

  // SDK conveniences (unified verbs)

  async create(input: { type: string; name?: string }) {
    const doc = EntitySchema.parse(createEntity(input as any));
    await this.persist(doc);
    this.bus.publish({
      kind: 'entity.created',
      payload: {
        id: doc.shape.core.id,
        type: doc.shape.core.type,
        name: doc.shape.core.name ?? null,
      },
    });
    return doc.shape.core.id as EntityId;
  }

  async delete(id: EntityId) {
    const existed = await this.remove(id);
    this.bus.publish({ kind: 'entity.deleted', payload: { id, ok: existed } });
  }

  async describe(id: EntityId) {
    const doc = await this.mustGet(id);
    const info = {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name ?? null,
      state: doc.shape.state,
    };
    this.bus.publish({ kind: 'entity.described', payload: info });
    return info;
  }

  async setCore(id: EntityId, core: { name?: string; type?: string }) {
    const curr = await this.mustGet(id);
    const next = EntitySchema.parse(updateEntity(curr, { core } as any));
    await this.persist(next);
    this.bus.publish({
      kind: 'entity.core.set',
      payload: {
        id,
        name: next.shape.core.name ?? null,
        type: next.shape.core.type,
      },
    });
  }

  async setState(id: EntityId, state: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = EntitySchema.parse(updateEntity(curr, { state } as any));
    await this.persist(next);
    this.bus.publish({ kind: 'entity.state.set', payload: { id } });
  }

  async patchState(id: EntityId, patch: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = EntitySchema.parse(
      updateEntity(curr, { state: patch } as any),
    );
    await this.persist(next);
    this.bus.publish({ kind: 'entity.state.patched', payload: { id } });
  }

  // Internals

  private async mustGet(id: string): Promise<Entity> {
    const found = await this.get(id);
    if (!found) throw new Error(`Entity not found: ${id}`);
    return found;
  }

  private async persist(doc: Entity): Promise<void> {
    const id = doc.shape.core.id;
    if (this.repo) {
      const existing = await this.repo.get(id);
      if (existing) {
        await this.repo.update(id, () => doc);
      } else {
        await this.repo.create(doc);
      }
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
