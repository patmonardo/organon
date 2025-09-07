import { describe, it, expect } from 'vitest'
import {
  applyCanonRules,
  ruleHypothesizeFromConjunction,
  ruleResolveDisjunction,
} from '../../src/dataset/canon-rules'

describe('Logogenesis cycle — Conjunction → Hypothetical → Disjunction → Categorical', () => {
  it('derives ONE_OF from HAS∧IMPLIES, then resolves to ACCEPT with provenance', () => {
    const artifact = {
      dataset: 'logo.cycle',
      nodes: [{ id: 's1' }, { id: 'fA' }, { id: 'fB' }, { id: 'A' }, { id: 'B' }],
      edges: [
        // Conjunctive evidence about subject
        { type: 'HAS', from: 's1', to: 'fA' },
        { type: 'HAS', from: 's1', to: 'fB' },
        // Hypothetical mappings (feature ⇒ class)
        { type: 'IMPLIES', from: 'fA', to: 'A' },
        { type: 'IMPLIES', from: 'fB', to: 'B' },
        // Disjunctive elimination evidence
        { type: 'NEGATED', from: 's1', to: 'A' },
      ],
      terms: [],
    } as any

    // Step 1: Hypothesize ONE_OF from conjunctions
    const afterHyp = applyCanonRules(artifact, [ruleHypothesizeFromConjunction])
    const oneOf = (afterHyp.edges ?? []).filter(e => e.type === 'ONE_OF')
    expect(oneOf).toEqual(
      expect.arrayContaining([
        expect.objectContaining({ from: 's1', to: 'A' }),
        expect.objectContaining({ from: 's1', to: 'B' }),
      ]),
    )
    // Provenance carried
    expect(oneOf[0]?.props?.provenance?.kind).toBe('hypothetical')

    // Step 2: Resolve disjunction to categorical ACCEPT
    const afterDisj = applyCanonRules(afterHyp, [ruleResolveDisjunction])
    const accept = (afterDisj.edges ?? []).filter(e => e.type === 'ACCEPT')
    expect(accept).toHaveLength(1)
    expect(accept[0]).toMatchObject({
      type: 'ACCEPT',
      from: 's1',
      to: 'B',
      props: { modality: 'inferred', provenance: { kind: 'disjunctive', rule: 'resolve-disjunction' } },
    })

    // Idempotent across the cycle
    const again = applyCanonRules(afterDisj, [ruleHypothesizeFromConjunction, ruleResolveDisjunction])
    const accept2 = (again.edges ?? []).filter(e => e.type === 'ACCEPT')
    expect(accept2).toHaveLength(1)
  })
})
