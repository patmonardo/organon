import { describe, it } from 'vitest';
import { groundStage, commitGroundResults } from '../../src/absolute/essence/ground';
import { RelationEngine } from '../../src/form/relation/engine';
import { schemas as essenceSchemas } from '../../src/absolute/essence';

// Very small in-memory repo used for demoing commit flows
function makeInMemoryRepo() {
  const store = new Map<string, any>();
  return {
    async get(id: string) {
      return store.get(id) ?? null;
    },
    async create(doc: any) {
      store.set(doc.id ?? doc.shape?.core?.id ?? Math.random().toString(), doc);
      return doc;
    },
    async update(id: string, mut: (c: any) => any) {
      const curr = store.get(id) ?? null;
      const next = mut(curr);
      store.set(id, next);
      return next;
    },
    async delete(id: string) {
      return store.delete(id);
    },
    dump() {
      return Array.from(store.values());
    },
  } as const;
}

describe.skip('demo/form-processing', () => {
  it('runs a tiny ground -> process -> commit flow and prints world', async () => {
    // principles: a single trivial morph that connects two things
    const principles = {
      morphs: [
        // a debug morph: we intentionally keep applyMorphRule default behavior in ground
      ],
    };

    const graph = {
      entities: [
        { id: 'thing:1', type: 'system.Thing' },
        { id: 'thing:2', type: 'system.Thing' },
      ],
      properties: [
        { id: 'p:1', entityId: 'thing:1', key: 'status', value: 'active' },
      ],
    };

    // run groundStage (uses applyMorphRule which may be a noop depending on morphs present)
    const results = await groundStage(principles as any, graph as any, { fixpointMaxIters: 2 });

    console.log('GROUND RESULTS:', JSON.stringify(results, null, 2));

    // create in-memory triad
    const relRepo = makeInMemoryRepo();
    const propRepo = makeInMemoryRepo();

    // use RelationEngine to process ActiveRelation carriers if any
    const relEngine = new RelationEngine(relRepo as any);

    // Map ground relations -> ActiveRelation if any
    const activeRelations = (results.relations ?? []).map((r) => ({
      id: r.id,
      source: { id: r.sourceId, type: 'system.Thing' },
      target: { id: r.targetId, type: 'system.Thing' },
      type: r.type,
      kind: r.kind,
    })) as any[];

    const { actions, snapshot } = await relEngine.process(activeRelations as any);
    console.log('REL ACTIONS:', JSON.stringify(actions, null, 2));

    const res = await relEngine.commit(actions, snapshot as any);
    console.log('COMMIT RESULT:', JSON.stringify(res, null, 2));

    console.log('REPO CONTENTS:', JSON.stringify(relRepo.dump(), null, 2));
  });
});
