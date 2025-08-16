import { describe, it, expect } from 'vitest';
import { groundStage } from '../../src/absolute/ground';
import type { Morph } from '../../src/schema/morph';

// Minimal entities/properties as plain objects to drive groundStage
const e1 = { id: 'E1' } as any;
const e2 = { id: 'E2' } as any;

describe('Ground — idempotence', () => {
  it('re-running groundStage with same inputs produces the same derived artifacts (no duplicates)', async () => {
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
          contextId: 'ctx',
          contextVersion: 1,
          status: 'valid',
        },
      ],
    };

    const res1 = await groundStage(principles as any, graph as any, { fixpointMaxIters: 8 });
    const res2 = await groundStage(principles as any, graph as any, { fixpointMaxIters: 8 });

    // basic expectations
    expect(res1.relations.length).toBeGreaterThan(0);
    expect(res1.properties.length).toBeGreaterThanOrEqual(0);

    // same counts across runs
    expect(res1.relations.length).toBe(res2.relations.length);
    expect(res1.properties.length).toBe(res2.properties.length);

    // ensure no duplicate ids in the first result
    const relIds = res1.relations.map((r) => r.id);
    expect(new Set(relIds).size).toBe(relIds.length);
    const propIds = res1.properties.map((p) => p.id);
    expect(new Set(propIds).size).toBe(propIds.length);
  });
});
