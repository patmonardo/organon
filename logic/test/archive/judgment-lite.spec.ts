import { describe, it, expect } from 'vitest';
import { DefaultJudgmentEngineLite as J } from '../../src/absolute/concept/judgment';

describe('JudgmentEngineLite — QualQuant → Judgment', () => {
  it('maps ActiveRelation into schema-backed Judgment with modality/polarity', () => {
    const r = {
      id: 'rel:1',
      kind: 'relation',
      particularityOf: 'abs:1',
      source: { id: 'A', type: 'system.Thing' },
      target: { id: 'B', type: 'system.Thing' },
      type: 'supports',
      active: true,
      confidence: 0.9,
    } as any;

    const j = J.toJudgmentFromRelation(r, { tags: ['demo'], weight: 1 });
    expect(j.shape.core.type).toBe('system.Judgment');
    expect(j.shape.subject.id).toBe('A');
    expect(j.shape.predicate).toBe('supports');
    expect(j.shape.object?.id).toBe('B');
    expect(j.shape.polarity).toBe('affirm');
    expect(j.shape.modality).toBe('actual');
  });
});
