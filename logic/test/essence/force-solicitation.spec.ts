import { describe, it, expect } from 'vitest';
import { deriveSolicitationPairs, expressForceInfinity, type ForceNode } from '../../src/essence/relation/force-solicitation';

describe('essence/relation — Force: solicitation & infinity', () => {
  it('pairs are symmetric/mediated with stable signatures; infinity computes identity', () => {
    const nodes: ForceNode[] = [
      { id: 'A', inwardScore: 0.7, outwardScore: 0.6, tags: ['magnetic'], weight: 0.9 },
      { id: 'B', inwardScore: 0.65, outwardScore: 0.62, tags: ['electric'], weight: 1.0 }
    ];

    const pairs = deriveSolicitationPairs(nodes, { minCoupling: 0.1, tagAnyOf: ['electric'] });
    expect(pairs).toHaveLength(1);
    const p = pairs[0];
    expect(p.symmetric).toBe(true);
    expect(p.mediated).toBe(true);
    expect(p.roles['A']).toBeDefined();
    expect(p.roles['B']).toBeDefined();
    expect(p.coupling).toBeGreaterThan(0);

    const p2 = deriveSolicitationPairs(nodes)[0];
    expect(p2.signature).toEqual(p.signature);

  const inf = expressForceInfinity(p, nodes[0], nodes[1]);
    expect(inf.identityScore).toBeGreaterThan(0.5);
    expect(inf.signature).toBeDefined();
  });
});
