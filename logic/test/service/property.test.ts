import { describe, it, expect } from 'vitest';
import { PropertyService } from '../../src/form/property/service';
import { makeInMemoryRepository } from '../support/inMemoryRepo';

describe('PropertyService', () => {
  it('creates, updates, describes, and deletes properties with in-memory engine (no repo)', async () => {
    const svc = new PropertyService();

    const received: string[] = [];
    const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
    [
      'property.created',
      'property.core.set',
      'property.state.set',
      'property.state.patched',
      'property.described',
      'property.deleted',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Property', name: 'P', key: 'p', contextId: 'ctx' });
    expect(id).toBeTruthy();

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Property');
    expect(info1.name).toBe('P');

    // update core
    await svc.setCore(id, { name: 'P2', type: 'system.Property.Updated' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('P2');
    expect(info2.type).toBe('system.Property.Updated');

    // state
    await svc.setState(id, { status: 'active' } as any);
    const info3 = await svc.describe(id);
    expect((info3.state as any).status).toBe('active');
    await svc.patchState(id, { meta: { ok: true } } as any);
    const info4 = await svc.describe(id);
    expect((info4.state as any).meta.ok).toBe(true);

    // delete
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();

    // event sequence sanity: first is created, last is deleted
    expect(received[0]).toBe('property.created');
    expect(received[received.length - 1]).toBe('property.deleted');

    // contains expected mutation events at least once
    const has = (k: string) => received.includes(k);
    [
      'property.core.set',
      'property.state.set',
      'property.state.patched',
    ].forEach((k) => expect(has(k)).toBe(true));

    // number of describes equals number of describe() calls we made
    const describedCount = received.filter(
      (k) => k === 'property.described',
    ).length;
    expect(describedCount).toBe(4);
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository<any>();
    const svc = new PropertyService(repo as any);

    const id = await svc.create({ type: 'system.Property', name: 'Repo', key: 'repo', contextId: 'repo-ctx' });

    // get via repo-backed path
    const doc1 = await svc.get(id);
    expect(doc1?.shape.core.name).toBe('Repo');

    // mutate core and verify persisted
    await svc.setCore(id, { name: 'Repo2' });
    const doc2 = await svc.get(id);
    expect(doc2?.shape.core.name).toBe('Repo2');

    // delete and ensure gone
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();
  });
});
