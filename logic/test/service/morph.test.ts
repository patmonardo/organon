import { describe, it, expect } from 'vitest';
import { MorphService } from '../../src/form/morph/service';
import { makeInMemoryRepository } from '../support/inMemoryRepo';

describe('MorphService', () => {
  it('creates, updates, describes, and deletes morphs with in-memory engine (no repo)', async () => {
    const svc = new MorphService();

    const received: string[] = [];
    const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
    [
      'morph.created',
      'morph.core.set',
      'morph.state.set',
      'morph.state.patched',
      'morph.described',
      'morph.deleted',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Morph', name: 'M' });
    expect(id).toBeTruthy();

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Morph');
    expect(info1.name).toBe('M');

    // update core
    await svc.setCore(id, { name: 'M2', type: 'system.Morph.Updated' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('M2');
    expect(info2.type).toBe('system.Morph.Updated');

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
    expect(received[0]).toBe('morph.created');
    expect(received[received.length - 1]).toBe('morph.deleted');

    // contains expected mutation events at least once
    const has = (k: string) => received.includes(k);
    ['morph.core.set', 'morph.state.set', 'morph.state.patched'].forEach((k) =>
      expect(has(k)).toBe(true),
    );

    // number of describes equals number of describe() calls we made
    const describedCount = received.filter(
      (k) => k === 'morph.described',
    ).length;
    expect(describedCount).toBe(4);
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository<any>();
    const svc = new MorphService(repo as any);

    const id = await svc.create({ type: 'system.Morph', name: 'Repo' });

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
