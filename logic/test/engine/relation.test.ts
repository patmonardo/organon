import { describe, it, expect } from 'vitest';
import { RelationEngine } from '../../src/form/relation/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import { type Relation } from '../../src/schema/relation';
import { createEntity } from '../../src/schema/entity';

describe('RelationEngine', () => {
  it('creates, sets endpoints, inverts, sets direction, deletes', async () => {
    const repo = makeInMemoryRepository<Relation>();
    const bus = new InMemoryEventBus();
    const received: string[] = [];
    [
      'relation.created',
      'relation.endpoints.set',
      'relation.inverted',
      'relation.direction.set',
      'relation.deleted',
    ].forEach((k) => bus.subscribe(k, (e) => received.push(e.kind)));

    const engine = new RelationEngine(repo as any, bus);

    // prepare endpoints first
    const e1 = createEntity({ type: 'system.Entity', name: 'A' });
    const e2 = createEntity({ type: 'system.Entity', name: 'B' });
    const source = { id: e1.shape.core.id, type: e1.shape.core.type };
    const target = { id: e2.shape.core.id, type: e2.shape.core.type };

    // create with required fields (including endpoints)
    const [created] = await engine.handle({
      kind: 'relation.create',
      payload: {
        type: 'system.Relation',
        kind: 'related_to',
        direction: 'directed',
        source,
        target,
      },
    } as any);
    const id = (created.payload as any).id as string;
    expect(created.kind).toBe('relation.created');
    expect(id).toBeTruthy();

    // setEndpoints (idempotent update)
    const [setEndpoints] = await engine.handle({
      kind: 'relation.setEndpoints',
      payload: { id, source, target },
    } as any);
    expect(setEndpoints.kind).toBe('relation.endpoints.set');

    // invert
    const [inverted] = await engine.handle({
      kind: 'relation.invert',
      payload: { id },
    } as any);
    expect(inverted.kind).toBe('relation.inverted');

    // set direction
    const [setDirection] = await engine.handle({
      kind: 'relation.setDirection',
      payload: { id, direction: 'bidirectional' },
    } as any);
    expect(setDirection.kind).toBe('relation.direction.set');

    // delete
    const [deleted] = await engine.handle({
      kind: 'relation.delete',
      payload: { id },
    } as any);
    expect(deleted.kind).toBe('relation.deleted');

    expect(received).toEqual([
      'relation.created',
      'relation.endpoints.set',
      'relation.inverted',
      'relation.direction.set',
      'relation.deleted',
    ]);
  });
});
