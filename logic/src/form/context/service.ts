import type { Event, EventBus } from '../triad/message';
import { InMemoryEventBus } from '../triad/bus';
import type { Repository } from '../../repository/repo';
import {
  ContextSchema,
  type Context,
  createContext,
  updateContext,
} from '../../schema/context';

export type ContextId = string;

export class ContextService {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, Context>();

  constructor(private readonly repo?: Repository<Context>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Event API
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Reads
  async get(id: ContextId): Promise<Context | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? ContextSchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? ContextSchema.parse(doc) : undefined;
  }

  // SDK verbs

  async create(input: { type: string; name?: string }) {
    const doc = ContextSchema.parse(createContext(input as any));
    await this.persist(doc);
    this.bus.publish({
      kind: 'context.created',
      payload: {
        id: doc.shape.core.id,
        type: doc.shape.core.type,
        name: doc.shape.core.name ?? null,
      },
    });
    return doc.shape.core.id as ContextId;
  }

  async delete(id: ContextId) {
    const existed = await this.remove(id);
    this.bus.publish({ kind: 'context.deleted', payload: { id, ok: existed } });
  }

  async describe(id: ContextId) {
    const doc = await this.mustGet(id);
    const info = {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name ?? null,
      state: doc.shape.state,
    };
    this.bus.publish({ kind: 'context.described', payload: info });
    return info;
  }

  async setCore(id: ContextId, core: { name?: string; type?: string }) {
    const curr = await this.mustGet(id);
    const next = ContextSchema.parse(updateContext(curr, { core } as any));
    await this.persist(next);
    this.bus.publish({
      kind: 'context.core.set',
      payload: {
        id,
        name: next.shape.core.name ?? null,
        type: next.shape.core.type,
      },
    });
  }

  async setState(id: ContextId, state: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = ContextSchema.parse(updateContext(curr, { state } as any));
    await this.persist(next);
    this.bus.publish({ kind: 'context.state.set', payload: { id } });
  }

  async patchState(id: ContextId, patch: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = ContextSchema.parse(
      updateContext(curr, { state: patch } as any),
    );
    await this.persist(next);
    this.bus.publish({ kind: 'context.state.patched', payload: { id } });
  }

  // Internals

  private async mustGet(id: string): Promise<Context> {
    const found = await this.get(id);
    if (!found) throw new Error(`Context not found: ${id}`);
    return found;
  }

  private async persist(doc: Context): Promise<void> {
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
