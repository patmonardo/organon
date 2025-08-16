import { describe, it, expect } from 'vitest';
import { groundStage } from '../../src/absolute/essence/ground';
import type { Morph } from '../../src/schema/morph';

const e1 = { id: 'E1' } as any;
const e2 = { id: 'E2' } as any;

describe('Ground — context propagation', () => {
  it('ensures derived relations/properties inherit contextId/contextVersion from triggering property', async () => {
    const morph: Morph = {
      id: 'm:copula-A-to-R',
      // @ts-ignore minimal ruleSpec used by ground.applyMorphRule
      ruleSpec: {
        id: 'm:copula-A-to-R',
        kind: 'deriveRelation',
        condition: { op: 'exists', key: 'A' },
        source: { byId: 'E1' },
        target: { kind: 'fixed', targetEntityId: 'E2' },
        relationType: 'related_to',
        setProperty: { key: 'B', value: 1 },
        idempotent: true,
      },
    } as any;

    const principles = { morphs: [morph] };
    const graph = {
      entities: [e1, e2],
      properties: [
        {
          id: 'p:E1:A',
          entityId: 'E1',
          key: 'A',
          value: true,
          contextId: 'context-42',
          contextVersion: 'v42',
          status: 'valid',
        },
      ],
    };

    const res = await groundStage(principles as any, graph as any, { fixpointMaxIters: 8 });

    const rel = res.relations.find((r) => r.sourceId === 'E1' && r.targetId === 'E2');
    expect(rel).toBeDefined();
    // derived relation should carry context provenance (in our scaffold this may be contextId/contextVersion fields)
    expect((rel as any).contextId ?? null).toBe('context-42');
    expect((rel as any).contextVersion ?? null).toBe('v42');

    const derivedProp = res.properties.find((p) => p.entityId === 'E2' && p.key === 'B');
    expect(derivedProp).toBeDefined();
    expect((derivedProp as any).contextId ?? null).toBe('context-42');
    expect((derivedProp as any).contextVersion ?? null).toBe('v42');
  });
});
