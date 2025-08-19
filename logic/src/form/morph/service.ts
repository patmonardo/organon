import type { Event, EventBus } from '../../absolute/core/triad/message';
import { InMemoryEventBus } from '../../absolute/core/triad/bus';
import type { Repository } from '../../repository/repo';
import {
  MorphSchema,
  type Morph,
  createMorph,
  updateMorph,
} from '../../schema/morph';

export type MorphId = string;

export class MorphService {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, Morph>();

  constructor(private readonly repo?: Repository<Morph>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Event API
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Reads
  async get(id: MorphId): Promise<Morph | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? MorphSchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? MorphSchema.parse(doc) : undefined;
  }

  // SDK verbs

  async create(input: { type: string; name?: string }) {
    const doc = MorphSchema.parse(createMorph(input as any));
    await this.persist(doc);
    this.bus.publish({
      kind: 'morph.created',
      payload: {
        id: doc.shape.core.id,
        type: doc.shape.core.type,
        name: doc.shape.core.name ?? null,
      },
    });
    return doc.shape.core.id as MorphId;
  }

  async delete(id: MorphId) {
    const existed = await this.remove(id);
    this.bus.publish({ kind: 'morph.deleted', payload: { id, ok: existed } });
  }

  async describe(id: MorphId) {
    const doc = await this.mustGet(id);
    const info = {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name ?? null,
      state: doc.shape.state,
    };
    this.bus.publish({ kind: 'morph.described', payload: info });
    return info;
  }

  async setCore(id: MorphId, core: { name?: string; type?: string }) {
    const curr = await this.mustGet(id);
    const next = MorphSchema.parse(updateMorph(curr, { core } as any));
    await this.persist(next);
    this.bus.publish({
      kind: 'morph.core.set',
      payload: {
        id,
        name: next.shape.core.name ?? null,
        type: next.shape.core.type,
      },
    });
  }

  async setState(id: MorphId, state: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = MorphSchema.parse(updateMorph(curr, { state } as any));
    await this.persist(next);
    this.bus.publish({ kind: 'morph.state.set', payload: { id } });
  }

  async patchState(id: MorphId, patch: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = MorphSchema.parse(updateMorph(curr, { state: patch } as any));
    await this.persist(next);
    this.bus.publish({ kind: 'morph.state.patched', payload: { id } });
  }

  // Internals

  private async mustGet(id: string): Promise<Morph> {
    const found = await this.get(id);
    if (!found) throw new Error(`Morph not found: ${id}`);
    return found;
  }

  private async persist(doc: Morph): Promise<void> {
    const id = doc.shape.core.id;
    if (this.repo) {
      const existing = await this.repo.get(id);
      if (existing) await this.repo.update(id, () => doc);
      else await this.repo.create(doc);
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
