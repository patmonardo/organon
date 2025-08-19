import type { Event, EventBus } from "../../absolute/core/triad/message";
import { InMemoryEventBus } from "../../absolute/core/triad/bus";
import type { Repository } from "../../repository/repo";
import {
  PropertySchema,
  type Property,
  createProperty,
  updateProperty,
} from "../../schema/property";

export type PropertyId = string;

export class PropertyService {
  private readonly bus: EventBus;
  private readonly mem = new Map<string, Property>();

  constructor(private readonly repo?: Repository<Property>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
  }

  // Event API
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Reads
  async get(id: PropertyId): Promise<Property | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? PropertySchema.parse(doc) : undefined;
    }
    const doc = this.mem.get(id);
    return doc ? PropertySchema.parse(doc) : undefined;
  }

  // SDK verbs

  async create(input: { type: string; name?: string; key: string; contextId: string }) {
    const doc = PropertySchema.parse(createProperty(input as any));
    await this.persist(doc);
    this.bus.publish({
      kind: "property.created",
      payload: {
        id: doc.shape.core.id,
        type: doc.shape.core.type,
        name: doc.shape.core.name ?? null,
        key: doc.shape.core.key,
        contextId: doc.shape.contextId,
      },
    });
    return doc.shape.core.id as PropertyId;
  }

  async delete(id: PropertyId) {
    const existed = await this.remove(id);
    this.bus.publish({ kind: "property.deleted", payload: { id, ok: existed } });
  }

  async describe(id: PropertyId) {
    const doc = await this.mustGet(id);
    const info = {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name ?? null,
      state: doc.shape.state,
    };
    this.bus.publish({ kind: "property.described", payload: info });
    return info;
  }

  async setCore(id: PropertyId, core: { name?: string; type?: string; key?: string }) {
    const curr = await this.mustGet(id);
    const next = PropertySchema.parse(updateProperty(curr, { core } as any));
    await this.persist(next);
    this.bus.publish({
      kind: "property.core.set",
      payload: {
        id,
        name: next.shape.core.name ?? null,
        type: next.shape.core.type,
        key: next.shape.core.key,
        contextId: next.shape.contextId,
      },
    });
  }

  async setState(id: PropertyId, state: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = PropertySchema.parse(updateProperty(curr, { state } as any));
    await this.persist(next);
    this.bus.publish({ kind: "property.state.set", payload: { id } });
  }

  async patchState(id: PropertyId, patch: Record<string, unknown>) {
    const curr = await this.mustGet(id);
    const next = PropertySchema.parse(updateProperty(curr, { state: patch } as any));
    await this.persist(next);
    this.bus.publish({ kind: "property.state.patched", payload: { id } });
  }

  // Internals

  private async mustGet(id: string): Promise<Property> {
    const found = await this.get(id);
    if (!found) throw new Error(`Property not found: ${id}`);
    return found;
  }

  private async persist(doc: Property): Promise<void> {
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
