import { describe, it, expect } from 'vitest';
import { groundStage } from '../../src/absolute/essence/ground';
import type { Morph } from '../../src/schema/morph';

// Minimal entities/properties as plain objects to drive groundStage
const e1 = { id: 'E1' } as any;
const e2 = { id: 'E2' } as any;

describe('Ground — copula (X -> Y) as deriveRelation', () => {
  it('fires a copula rule: property A on E1 produces relation E1->E2 and derived property on E2', async () => {
    const morph: Morph = {
      id: 'm:copula-A-to-R',
      // @ts-ignore - minimal ruleSpec shape used by ground.applyMorphRule
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

    const res = await groundStage(principles as any, graph as any, { fixpointMaxIters: 8 });

    // Expect one derived relation and one derived property for E2
    const rel = res.relations.find((r) => r.sourceId === 'E1' && r.targetId === 'E2' && r.type === 'related_to');
    expect(rel).toBeDefined();

    const derivedProp = res.properties.find((p) => p.entityId === 'E2' && p.key === 'B' && p.status === 'derived');
    expect(derivedProp).toBeDefined();

    // Provenance checks (rule id propagated)
    expect(rel?.ruleId ?? rel?.provenance?.ruleId).toBe('m:copula-A-to-R');
    expect(derivedProp?.provenance?.ruleId).toBe('m:copula-A-to-R');
  });
});
