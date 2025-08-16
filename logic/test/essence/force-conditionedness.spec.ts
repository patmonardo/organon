import { describe, it, expect } from 'vitest';
import { analyzeForceCondition } from '../../src/essence/relation/force-conditionedness';

describe('essence/relation/analyzeForceCondition — Part 1: Conditionedness', () => {
  it('computes a report and stable signature', () => {
    const candidate = {
      id: 'F:1',
      thingId: 'T:1',
      immediacy: true,
      reflection: true,
      externalityTags: ['magnetic', 'ETHER']
    };

    const r1 = analyzeForceCondition(candidate);
    expect(r1.presupposesThing).toBe(true);
    expect(r1.appearsAsPositedness).toBe(true);
    expect(r1.immediacyTransient).toBe(true);
    expect(r1.designatedAsMatter).toEqual(['magnetic', 'ether']);
    const sig1 = r1.signature;

    const r2 = analyzeForceCondition(candidate);
    expect(r2.signature).toEqual(sig1);
    expect(r2.evidence.find(e => e.startsWith('sig:'))).toBeDefined();
    expect(r2.negativeUnityScore).toBeGreaterThan(0);
  });
});
