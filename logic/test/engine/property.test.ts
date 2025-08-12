import { describe, it, expect } from 'vitest';
import { PropertyEngine } from '../../src/form/property/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import { type Property } from '../../src/schema/property';
import { type Entity, createEntity } from '../../src/schema/entity';

describe('PropertyEngine', () => {
  it('creates, binds to entity, sets/clears value, and deletes', async () => {
    const repo = makeInMemoryRepository<Property>();
    const bus = new InMemoryEventBus();
    const received: string[] = [];
    [
      'property.created',
      'property.bound.entity',
      'property.value.set',
      'property.value.cleared',
      'property.deleted',
    ].forEach((k) => bus.subscribe(k, (e) => received.push(e.kind)));

    const engine = new PropertyEngine(repo as any, bus);

    const [created] = await engine.handle({
      kind: 'property.create',
      payload: {
        type: 'system.Property',
        key: 'p0',
        contextId: 'ctx:test', // required by schema
      },
    } as any);
    const id = (created.payload as any).id as string;

    const ent: Entity = createEntity({ type: 'system.Entity', name: 'E0' });
    const [bound] = await engine.handle({
      kind: 'property.bindEntity',
      payload: {
        id,
        ref: { id: ent.shape.core.id, type: ent.shape.core.type },
      },
    } as any);
    expect(bound.kind).toBe('property.bound.entity');

    const [setValue] = await engine.handle({
      kind: 'property.setValue',
      payload: { id, value: 42, valueType: 'number' },
    } as any);
    expect(setValue.kind).toBe('property.value.set');

    const [cleared] = await engine.handle({
      kind: 'property.clearValue',
      payload: { id },
    } as any);
    expect(cleared.kind).toBe('property.value.cleared');

    const [deleted] = await engine.handle({
      kind: 'property.delete',
      payload: { id },
    } as any);
    expect(deleted.kind).toBe('property.deleted');

    expect(received).toEqual([
      'property.created',
      'property.bound.entity',
      'property.value.set',
      'property.value.cleared',
      'property.deleted',
    ]);
  });
});
