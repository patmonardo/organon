import { describe, it, expect } from 'vitest';
import { runKriya } from '../../src/absolute/core/orchestrator';

// Minimal smoke input: one entity, one morph that derives a simple relation
const sampleInput = {
  shapes: [],
  entities: [
    {
      shape: { core: { id: 'thing:1', type: 'system.Thing' }, state: {}, essence: { foo: 'bar' } },
      revision: 1,
    },
  ],
  properties: [
    { id: 'prop:1', entity: 'thing:1', key: 'status', value: 'active' },
  ],
  morphs: [
    {
      id: 'morph:derive-simple',
      shape: { core: { id: 'morph:derive-simple', type: 'system.Morph' } },
      ruleSpec: {
        id: 'rule:1',
        kind: 'deriveRelation',
        relationType: 'related_to',
        source: { all: true },
        target: { kind: 'fixed', targetEntityId: 'thing:1' },
        idempotent: true,
      },
    },
  ],
  content: [],
};

describe('kriya smoke', () => {
  it('runs runKriya and returns expected shape', async () => {
    const res = await runKriya(sampleInput as any, { projectContent: false });
    expect(res).toHaveProperty('world');
    expect(res).toHaveProperty('ground');
    if (res.ground) {
      expect(Array.isArray(res.ground.relations)).toBe(true);
    }
  });
});
