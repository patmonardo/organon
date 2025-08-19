import { describe, it, expect } from 'vitest';
import { ContextEngine } from '../../src/form/context/engine';
import { InMemoryEventBus } from '../../src/absolute/core/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import {
  type Context,
  createContext,
  ContextSchema,
} from '../../src/schema/context';
import { type Entity, createEntity } from '../../src/schema/entity';

describe('ContextEngine', () => {
  it('creates, adds/removes membership, and deletes', async () => {
    const repo = makeInMemoryRepository<Context>();
    const bus = new InMemoryEventBus();
    const received: string[] = [];

    // Subscribe to emitted kinds (singular addEntity emits 'context.entity.added')
    [
      'context.created',
      'context.entity.added',
      'context.entities.added', // in case addEntities is used in future
      'context.entity.removed',
      'context.deleted',
    ].forEach((k) => bus.subscribe(k, (e) => received.push(e.kind)));

    const engine = new ContextEngine(repo as any, bus);

    const [created] = await engine.handle({
      kind: 'context.create',
      payload: { type: 'system.Context', name: 'C0' },
    } as any);
    const id = (created.payload as any).id as string;

    // add entity (singular)
    const ent: Entity = createEntity({ type: 'system.Entity', name: 'E0' });
    const [added] = await engine.handle({
      kind: 'context.addEntity',
      payload: {
        id,
        ref: { id: ent.shape.core.id, type: ent.shape.core.type },
      },
    } as any);
    expect(added.kind).toBe('context.entity.added');

    // remove entity
    const [removed] = await engine.handle({
      kind: 'context.removeEntity',
      payload: {
        id,
        ref: { id: ent.shape.core.id, type: ent.shape.core.type },
      },
    } as any);
    expect(removed.kind).toBe('context.entity.removed');

    // delete
    const [deleted] = await engine.handle({
      kind: 'context.delete',
      payload: { id },
    } as any);
    expect(deleted.kind).toBe('context.deleted');

    expect(received).toEqual([
      'context.created',
      'context.entity.added',
      'context.entity.removed',
      'context.deleted',
    ]);
  });
});
