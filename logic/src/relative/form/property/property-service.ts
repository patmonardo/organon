import { InMemoryEventBus, type EventBus } from '../../../absolute/core/bus';
import { makeInMemoryRepository } from '../../../repository/memory';
import { PropertySchema, type Property } from '../../../schema/property';
import { PropertyEngine } from './property-engine';
import type { Repository } from '../../../repository/repo';
import type { Event } from '../../../absolute/core/message';

export type PropertyId = string;

export interface PropertyCreateInput {
  id?: string;
  type: string;
  name: string;
  key?: string;
  contextId?: string;
  state?: Record<string, unknown>;
}

export interface PropertyDescribeResult {
  id: string;
  type: string;
  name: string;
  state: Record<string, unknown>;
  signatureKeys: string[];
  facetsKeys: string[];
}

export class PropertyService {
  private readonly engine: PropertyEngine;
  private readonly bus: EventBus;

  constructor(repo?: Repository<Property>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    const defaultRepo =
      repo ??
      (makeInMemoryRepository(
        PropertySchema as any,
      ) as unknown as Repository<Property>);
    this.engine = new PropertyEngine(defaultRepo, this.bus);
  }

  // Event subscription with proper typing
  on(
    kind: string,
    handler: (event: { kind: string; payload: any; meta?: any }) => void,
  ): void {
    const anyBus = this.bus as any;
    if (typeof anyBus.subscribe === 'function') {
      anyBus.subscribe(kind, handler);
    } else if (typeof anyBus.on === 'function') {
      anyBus.on(kind, handler);
    }
  }

  // Direct engine access
  async send(cmd: {
    kind: string;
    payload?: any;
    meta?: Record<string, unknown>;
  }): Promise<Event[]> {
    return this.engine.handle(cmd as any);
  }

  // Safe ID extraction helper - properly typed
  private extractId(events: Event[], expectedKind: string): PropertyId {
    const event = events.find((e) => e?.kind === expectedKind);
    if (!event) {
      throw new Error(
        `No ${expectedKind} event found in response. Got: ${events
          .map((e) => e?.kind)
          .join(', ')}`,
      );
    }

    if (!event.payload) {
      throw new Error(`${expectedKind} event has no payload`);
    }

    // Properly extract and validate the id from payload
    const rawId = (event.payload as any)?.id;
    if (typeof rawId !== 'string' && typeof rawId !== 'number') {
      throw new Error(
        `${expectedKind} event payload.id must be string or number. Got: ${typeof rawId}, Payload: ${JSON.stringify(
          event.payload,
        )}`,
      );
    }

    return String(rawId) as PropertyId;
  }

  // CRUD operations with proper error handling
  async create(input: PropertyCreateInput): Promise<PropertyId> {
    const state = {
      ...input.state,
      ...(input.key ? { key: input.key } : {}),
      ...(input.contextId ? { contextId: input.contextId } : {}),
    };

    const events = await this.engine.handle({
      kind: 'property.create',
      payload: {
        id: input.id,
        type: input.type,
        name: input.name,
        state,
      },
    } as any);

    return this.extractId(events, 'property.create');
  }

  async get(id: PropertyId): Promise<Property | undefined> {
    const formProperty = await this.engine.getProperty(id);
    return formProperty?.toSchema();
  }

  async describe(id: PropertyId): Promise<PropertyDescribeResult> {
    const events = await this.engine.handle({
      kind: 'property.describe',
      payload: { id },
    } as any);

    const event = events.find((e) => e?.kind === 'property.describe');
    if (!event?.payload) {
      throw new Error(`property.describe failed for id: ${id}`);
    }

    return event.payload as PropertyDescribeResult;
  }

  async setCore(
    id: PropertyId,
    patch: { name?: string; type?: string },
  ): Promise<void> {
    await this.engine.handle({
      kind: 'property.setCore',
      payload: { id, ...patch },
    } as any);
  }

  async setState(
    id: PropertyId,
    state: Record<string, unknown>,
  ): Promise<void> {
    await this.engine.handle({
      kind: 'property.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(
    id: PropertyId,
    patch: Record<string, unknown>,
  ): Promise<void> {
    await this.engine.handle({
      kind: 'property.patchState',
      payload: { id, patch },
    } as any);
  }

  async delete(id: PropertyId): Promise<void> {
    await this.engine.handle({
      kind: 'property.delete',
      payload: { id },
    } as any);
  }

  async createAndDescribe(
    input: PropertyCreateInput,
  ): Promise<{ id: PropertyId; info: PropertyDescribeResult }> {
    const id = await this.create(input);
    const info = await this.describe(id);
    return { id, info };
  }
}
