import { describe, it, expect } from 'vitest';
import { RelationService } from '../../src/form/relation/service';
import { makeInMemoryRepository } from '../support/inMemoryRepo';

describe('RelationService', () => {
  it('creates, updates, describes, and deletes relations with in-memory engine (no repo)', async () => {
    const svc = new RelationService();

    const received: string[] = [];
    const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
    [
      'relation.created',
      'relation.core.set',
      'relation.endpoints.set',
      'relation.direction.set',
      'relation.state.set',
      'relation.state.patched',
      'relation.described',
      'relation.deleted',
    ].forEach(tap);

    // create
    const id = await svc.create({
      type: 'system.Relation',
      name: 'R',
      kind: 'related_to',
      source: { id: 's1', type: 'system.Entity' },
      target: { id: 't1', type: 'system.Entity' },
      direction: 'directed',
    });
    expect(id).toBeTruthy();

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Relation');
    expect(info1.name).toBe('R');
    expect(info1.kind).toBe('related_to');
    expect(info1.direction).toBe('directed');
    expect(info1.source.id).toBe('s1');
    expect(info1.target.id).toBe('t1');

    // update core (name/type/kind)
    await svc.setCore(id, {
      name: 'R2',
      type: 'system.Relation.Updated',
      kind: 'contains',
    });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('R2');
    expect(info2.type).toBe('system.Relation.Updated');
    expect(info2.kind).toBe('contains');

    // endpoints
    await svc.setEndpoints(id, {
      source: { id: 's2', type: 'system.Entity' },
      target: { id: 't2', type: 'system.Entity' },
    });
    const info3 = await svc.describe(id);
    expect(info3.source.id).toBe('s2');
    expect(info3.target.id).toBe('t2');

    // direction
    await svc.setDirection(id, 'bidirectional');
    const info4 = await svc.describe(id);
    expect(info4.direction).toBe('bidirectional');

    // state
    await svc.setState(id, { status: 'active' } as any);
    const info5 = await svc.describe(id);
    expect((info5.state as any).status).toBe('active');
    await svc.patchState(id, { meta: { ok: true } } as any);
    const info6 = await svc.describe(id);
    expect((info6.state as any).meta.ok).toBe(true);

    // delete
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();

    // event sequence sanity
    expect(received[0]).toBe('relation.created');
    expect(received[received.length - 1]).toBe('relation.deleted');

    // contains expected mutation events at least once
    const has = (k: string) => received.includes(k);
    [
      'relation.core.set',
      'relation.endpoints.set',
      'relation.direction.set',
      'relation.state.set',
      'relation.state.patched',
    ].forEach((k) => expect(has(k)).toBe(true));

    // number of describes equals number of describe() calls we made
    const describedCount = received.filter(
      (k) => k === 'relation.described',
    ).length;
    expect(describedCount).toBe(6);
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository<any>();
    const svc = new RelationService(repo as any);

    const id = await svc.create({
      type: 'system.Relation',
      name: 'Repo',
      kind: 'owns',
      source: { id: 'a1', type: 'system.Entity' },
      target: { id: 'b1', type: 'system.Entity' },
      direction: 'directed',
    });

    // get via repo-backed path
    const doc1 = await svc.get(id);
    expect(doc1?.shape.core.name).toBe('Repo');

    // mutate core and verify persisted
    await svc.setCore(id, { name: 'Repo2' });
    const doc2 = await svc.get(id);
    expect(doc2?.shape.core.name).toBe('Repo2');

    // endpoints persisted
    await svc.setEndpoints(id, {
      source: { id: 'a2', type: 'system.Entity' },
      target: { id: 'b2', type: 'system.Entity' },
    });
    const doc3 = await svc.get(id);
    expect((doc3 as any).shape.source.id).toBe('a2');
    expect((doc3 as any).shape.target.id).toBe('b2');

    // direction persisted
    await svc.setDirection(id, 'bidirectional');
    const doc4 = await svc.get(id);
    expect((doc4 as any).shape.direction).toBe('bidirectional');

    // delete and ensure gone
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();
  });
});
