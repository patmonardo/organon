import { describe, it, expect } from 'vitest';
import { isWholeGround, computeGroundCardinality, computeRelationDegrees, suggestModalityForGround, classifyTruthOfGround, classifyTruthOfRelation, locateTranscendental } from '../../src/absolute/qualquant';

describe('Qualitative/Quantitative helpers', () => {
  it('detects whole ground and computes counts/degree', () => {
    const ground: any = {
      id: 'g:abs',
      contributingConditionIds: ['c1','c2','c3','c4'],
    };
    const relations: any[] = [
      { id: 'r1', sourceId: 'A', targetId: 'B', particularityOf: 'g:abs' },
      { id: 'r2', sourceId: 'A', targetId: 'C', particularityOf: 'g:abs' },
      { id: 'r3', sourceId: 'B', targetId: 'C' },
    ];

    expect(isWholeGround(ground)).toBe(true);
    const card = computeGroundCardinality(ground, relations);
    expect(card.parts).toBe(2);
    expect(card.conditions).toBe(4);

    const deg = computeRelationDegrees(relations);
    expect(deg.out['A']).toBe(2);
    expect(deg.in['C']).toBe(2);

    const mod = suggestModalityForGround(ground);
    expect(['actual','possible']).toContain(mod.kind);
  });

  it('classifies truth of relation (Mechanism/Chemism/Teleology) and locates transcendental', () => {
    const abs: any = { id: 'abs1', kind: 'absolute', type: 'related_to:absolute', provenance: { metaphysics: { intuition: 'intellectual' } } };
    const rel1: any = { id: 'rel1', sourceId: 'S', targetId: 'T', type: 'related_to', kind: 'essential', particularityOf: 'abs1', provenance: { metaphysics: { intuition: 'inner' } } };
    const rel2: any = { id: 'rel2', sourceId: 'S', targetId: 'U', type: 'related_to', kind: 'essential', particularityOf: 'abs1' };
    const rels = [abs, rel1, rel2];

    // With two particulars, classify relation as Mechanism by default threshold
    expect(classifyTruthOfRelation(rel1, rels)).toBe('Mechanism');
    // Ground-level classification falls back to Teleology without parts signal or expression
    expect(classifyTruthOfGround(abs, rels)).toBe('Teleology');

    // Add an expressive property provenance to mark Chemism
    const props = [{ id: 'p1', provenance: { viaRelation: 'rel1' } }];
    expect(classifyTruthOfRelation(rel1, rels, {}, { properties: props })).toBe('Chemism');

    // Locate transcendental
    const loc = locateTranscendental(rel1, rels, {}, { properties: props });
    expect(loc.locus.id).toBe('rel1');
    expect(loc.transcendental?.id).toBe('abs1');
    expect(['Mechanism','Chemism','Teleology']).toContain(loc.truth);
  });
});
