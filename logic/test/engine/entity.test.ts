import { describe, it, expect } from 'vitest';
import { EntityEngine } from '../../src/form/entity/engine';
import { InMemoryEventBus } from '../../src/form/triad/bus';
import { makeInMemoryRepository } from '../support/inMemoryRepo';
import type { Entity } from '../../src/schema/entity';

describe('EntityEngine', () => {
  it('creates, updates, gets, and deletes (events emitted)', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const bus = new InMemoryEventBus();

    const received: string[] = [];
    const tap = (k: string) => bus.subscribe(k, (e) => received.push(e.kind));
    [
      'entity.created',
      'entity.updated',
      'entity.got',
      'entity.deleted',
    ].forEach(tap);

    const engine = new EntityEngine(repo as any, bus);

    // create
    const [created] = await engine.handle({
      kind: 'entity.create',
      payload: { type: 'system.Entity', name: 'A' },
    } as any);
    expect(created.kind).toBe('entity.created');
    const id = (created.payload as any).id as string;
    expect(id).toBeTruthy();

    // update (schema patch shape may vary; keep loose here)
    const [updated] = await engine.handle({
      kind: 'entity.update',
      payload: { id, patch: { core: { name: 'B' } } },
    } as any);
    expect(updated.kind).toBe('entity.updated');

    // get
    const [got] = await engine.handle({
      kind: 'entity.get',
      payload: { id },
    } as any);
    expect(got.kind).toBe('entity.got');
    expect((got.payload as any).entity?.shape?.core?.id).toBe(id);

    // delete
    const [deleted] = await engine.handle({
      kind: 'entity.delete',
      payload: { id },
    } as any);
    expect(deleted.kind).toBe('entity.deleted');

    expect(received).toEqual([
      'entity.created',
      'entity.updated',
      'entity.got',
      'entity.deleted',
    ]);
  });
});
