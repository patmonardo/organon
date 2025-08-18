import { describe, it } from 'vitest';
import { groundStage, toActiveFromGround, commitGroundResults } from '../../src/absolute/essence/ground';
import { RelationEngine } from '../../src/form/relation/engine';

// ANSI color helpers for lively logs
const C = {
  reset: '\u001b[0m',
  bright: '\u001b[1m',
  dim: '\u001b[2m',
  red: '\u001b[31m',
  green: '\u001b[32m',
  yellow: '\u001b[33m',
  blue: '\u001b[34m',
  magenta: '\u001b[35m',
  cyan: '\u001b[36m',
};

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

describe('demo/form-processing', () => {
  it('runs a tiny ground -> process -> commit flow and prints world', async () => {
    const principles = {
      morphs: [
        // simple link when thing:1 is active
        {
          id: 'rs-demo-1',
          ruleSpec: {
            id: 'rs-demo-1',
            kind: 'deriveRelation',
            relationType: 'linked_to',
            directed: true,
            idempotent: true,
            source: { byId: 'thing:1' },
            target: { kind: 'fixed', targetEntityId: 'thing:2' },
            condition: { op: 'eq', key: 'status', value: 'active' },
            setProperty: { key: 'recognized', value: true },
          },
        },
        // praise morph: if target has recognized=true, derive a praise relation
        {
          id: 'rs-demo-2',
          ruleSpec: {
            id: 'rs-demo-2',
            kind: 'deriveRelation',
            relationType: 'praises',
            directed: true,
            idempotent: true,
            source: { byId: 'thing:2' },
            target: { kind: 'fixed', targetEntityId: 'thing:1' },
            condition: { op: 'eq', key: 'recognized', value: true },
            setProperty: { key: 'admired', value: 9001 },
          },
        },
        // absolute aggregator morph to bump an absolute container when both exist
        {
          id: 'rs-demo-3',
          ruleSpec: {
            id: 'rs-demo-3',
            kind: 'deriveProperty',
            setProperty: { key: 'celebrated', value: 'spectacular', status: 'derived' },
            source: { all: true },
            target: { kind: 'select', selector: { all: true } },
            idempotent: true,
          },
        },
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

  console.log(C.cyan, C.bright, '\n⟡ Running Ground Stage — forging relations and properties...', C.reset);
  const results = await groundStage(principles as any, graph as any, { fixpointMaxIters: 4 });
  console.log(C.green, C.bright, '\n→ GROUND RESULTS:', C.reset, JSON.stringify(results, null, 2));

    const relRepo = makeInMemoryRepo();

    const relEngine = new RelationEngine(relRepo as any);

  const { relations: activeRelations } = toActiveFromGround(results as any);

  const { actions, snapshot } = await relEngine.process(activeRelations as any);
  console.log(C.yellow, C.bright, '\n→ REL ACTIONS:', C.reset, JSON.stringify(actions, null, 2));

    const res = await relEngine.commit(actions, snapshot as any);
  console.log(C.magenta, C.bright, '\n→ COMMIT RESULT:', C.reset, JSON.stringify(res, null, 2));

  console.log(C.blue, C.bright, '\n⚑ REPO CONTENTS:', C.reset, JSON.stringify(relRepo.dump(), null, 2));

  // persist derived properties too via commitGroundResults to show the property repo
  const propRepo = makeInMemoryRepo();
  await commitGroundResults({ relation: relRepo as any, property: propRepo as any, bus: undefined }, results as any);
  console.log(C.green, C.bright, '\n☼ PROPERTIES PERSISTED:', C.reset, JSON.stringify(propRepo.dump(), null, 2));
  });
});
