import { describe, it, expect } from 'vitest';
import { runCycle, StageFns } from '../../../logic/src/absolute/core/kriya';
import { reflectStage } from '../../../logic/src/absolute/essence/reflect';
import { groundStage } from '../../../logic/src/absolute/essence/ground';

describe('reflect -> ground integration (spectrum propagation)', () => {
  it('attaches spectrum metadata to derived relations via groundStage', async () => {
    const principles = {
      shapes: [],
      contexts: [],
      morphs: [
        // minimal morph with id and ruleSpec to derive a relation from thing:1 -> thing:2
        {
          id: 'm1',
          ruleSpec: {
            id: 'rs1',
            kind: 'deriveRelation',
            relationType: 'related_to',
            source: { byId: 'thing:1' },
            target: { kind: 'fixed', targetEntityId: 'thing:2' },
            idempotent: true,
            setProperty: { key: 'derived', value: true },
          },
        },
      ],
    } as any;

    // StageFns minimal implementations
    const fns: StageFns = {
      seed: async (p) => {
        return { entities: [{ id: 'thing:1', type: 'Example', essence: { soul: true } }, { id: 'thing:2', type: 'Example' }] } as any;
      },
      contextualize: async (p, g) => {
        return { properties: [
          { id: 'prop:1', entityId: 'thing:1', key: 'goal', value: 'be' },
          { id: 'prop:2', entityId: 'thing:1', key: 'color', value: 'red' }
        ] } as any;
      },
      reflect: async (things, props, opts) => {
        return reflectStage(things as any, props as any, opts as any) as any;
      },
      ground: async (p, g, opts) => {
        return groundStage({ morphs: principles.morphs }, { entities: (g as any).entities, properties: (g as any).properties }, opts as any) as any;
      },
      model: async (g) => ({ indexes: {}, views: {} }),
      control: async (g, proj) => ({ actions: [] }),
      plan: async (ctrl) => ({ tasks: [], workflow: { nodes: [], edges: [] } }),
    };

    const res = await runCycle(principles as any, fns, { reflectOpts: { contextId: 'ctx:1' } });

    // should have relations and at least one absolute + essential pair
    const rels = res.graph.relations;
    expect(rels.length).toBeGreaterThanOrEqual(1);

    // find the essential relation that was derived
    const essential = rels.find((r) => (r as any).kind === 'essential');
    const absolute = rels.find((r) => (r as any).kind === 'absolute');
    expect(essential).toBeDefined();
    expect(absolute).toBeDefined();

    // spectrum should have been attached from reflect facets (advisory)
    const spectrumOnEssential = (essential as any).meta?.spectrum || (essential as any).provenance?.spectrum;
    const spectrumOnAbsolute = (absolute as any).meta?.spectrum || (absolute as any).provenance?.spectrum;

    expect(spectrumOnEssential || spectrumOnAbsolute).toBeDefined();
    if (spectrumOnEssential) {
      expect(typeof spectrumOnEssential.signature).toBe('string');
      expect(spectrumOnEssential.intensity).toBeGreaterThanOrEqual(0);
      expect(spectrumOnEssential.intensity).toBeLessThanOrEqual(1);
    }
    if (spectrumOnAbsolute) {
      expect(typeof spectrumOnAbsolute.signature).toBe('string');
    }
  });
});
