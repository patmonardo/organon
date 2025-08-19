import { describe, it, expect } from 'vitest';
import { processAbsoluteConcept } from '../../src/absolute/concept';
import { ActiveRelationSchema, type ActiveRelation } from '../../src/schema/active';

function makeActiveRelation(): ActiveRelation {
  return ActiveRelationSchema.parse({
    id: 'r1',
    kind: 'relation',
    particularityOf: 'ess:r1',
    source: { id: 'e1', type: 'system.Thing' },
    target: { id: 'e2', type: 'system.Thing' },
    type: 'related_to',
    active: true,
  });
}

describe('AbsoluteConcept.process — minimal Qual/Quant integration', () => {
  it('produces a Judgment from an ActiveRelation and no derived edges yet', () => {
    const rel = makeActiveRelation();
    const out = processAbsoluteConcept({
      relations: [rel],
      qualquant: { r1: { tags: ['good'], score: 0.9, weight: 1 } },
      intentRegistry: { intents: [{ id: 'i1', morphId: 'm1', goal: 'demo', priority: 2 }] },
    });

    // judgments
    expect(out.judgments.length).toBe(1);
    const j = out.judgments[0];
    expect(j.shape.subject.id).toBe('e1');
    expect(j.shape.predicate).toBe('related_to');
    expect(j.shape.object?.id).toBe('e2');
    expect(j.shape.polarity).toBe('affirm');
    expect(j.shape.modality).toBe('actual');

    // derived placeholders are empty for now
    expect(out.mechanism.length).toBe(0);
    expect(out.chemism.length).toBe(0);
    expect(out.teleology.length).toBe(0);

    // intent candidates present and ordered by priority
    expect(out.intentCandidates.length).toBeGreaterThanOrEqual(1);
    expect(out.intentCandidates[0].intentId).toBe('i1');
  });
});
