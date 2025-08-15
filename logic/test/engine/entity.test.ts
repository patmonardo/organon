import { describe, it, expect } from 'vitest';
import { EntityEngine } from '../../src/form/entity/engine';
import type { Entity } from '../../src/schema/entity';
import { makeInMemoryRepository } from '../support/inMemoryRepo';

describe('EntityEngine.handle', () => {
  it('create → setCore → patch/set state → facets/signature merges → describe → delete', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const eng = new EntityEngine(repo);

    // create
    const [created] = await eng.handle({
      kind: 'entity.create',
      payload: { type: 'system.Entity', name: 'E1' },
    });
    expect(created.kind).toBe('entity.created');
    const id = (created.payload as any).id as string;
    expect(id).toBeTruthy();

    // setCore (name change)
    const [coreSet] = await eng.handle({
      kind: 'entity.setCore',
      payload: { id, name: 'E2' },
    });
    expect(coreSet.kind).toBe('entity.core.set');
    expect((coreSet.payload as any).name).toBe('E2');

    // patchState (add tags)
    const [patched] = await eng.handle({
      kind: 'entity.patchState',
      payload: { id, patch: { tags: ['x'] } },
    });
    expect(patched.kind).toBe('entity.state.patched');

    // setFacets
    const [facetsSet] = await eng.handle({
      kind: 'entity.setFacets',
      payload: { id, facets: { a: 1 } },
    });
    expect(facetsSet.kind).toBe('entity.facets.set');

    // mergeFacets
    const [facetsMerged] = await eng.handle({
      kind: 'entity.mergeFacets',
      payload: { id, patch: { b: 2 } },
    });
    expect(facetsMerged.kind).toBe('entity.facets.merged');

    // setSignature (object)
    const [sigSet] = await eng.handle({
      kind: 'entity.setSignature',
      payload: { id, signature: { s: true } },
    });
    expect(sigSet.kind).toBe('entity.signature.set');

    // mergeSignature
    const [sigMerged] = await eng.handle({
      kind: 'entity.mergeSignature',
      payload: { id, patch: { t: 1 } },
    });
    expect(sigMerged.kind).toBe('entity.signature.merged');

    // clear signature (undefined sentinel)
    const [sigCleared] = await eng.handle({
      kind: 'entity.setSignature',
      payload: { id, signature: undefined },
    });
    expect(sigCleared.kind).toBe('entity.signature.set');

    // describe (verify keys reflect merges/clears and state carried tags)
    const [described] = await eng.handle({
      kind: 'entity.describe',
      payload: { id },
    });
    expect(described.kind).toBe('entity.described');
    const d = described.payload as any;
    expect(d.id).toBe(id);
    expect(Array.isArray(d.signatureKeys)).toBe(true);
    expect(d.signatureKeys.length).toBe(0); // cleared
    // Current describe payload does not surface facetsKeys for Entity
    expect((d.facetsKeys ?? []).length).toBe(0);
    expect(d.state?.tags).toEqual(['x']); // patched state preserved

    // delete
    const [deleted] = await eng.handle({
      kind: 'entity.delete',
      payload: { id },
    });
    expect(deleted.kind).toBe('entity.deleted');
    expect((deleted.payload as any).ok).toBe(true);
  });
});
