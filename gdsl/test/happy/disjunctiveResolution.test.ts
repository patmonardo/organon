import { describe, it, expect } from 'vitest'
import { applyCanonRules, ruleResolveDisjunction } from '../../src/dataset/canon-rules'

describe('Logogenesis (happy path) — Disjunctive resolution', () => {
  it('(A ∨ B) ∧ ¬A ⇒ B → derive Categorical ACCEPT with provenance', () => {
    const artifact = {
      dataset: 'logo.disj',
      nodes: [{ id: 's1' }, { id: 'A' }, { id: 'B' }],
      edges: [
        { type: 'ONE_OF', from: 's1', to: 'A' },
        { type: 'ONE_OF', from: 's1', to: 'B' },
        { type: 'NEGATED', from: 's1', to: 'A' }, // evidence against A
      ],
      terms: [],
    } as any

    const once = applyCanonRules(artifact, [ruleResolveDisjunction])
    const acc = (once.edges ?? []).filter(e => e.type === 'ACCEPT')
    expect(acc).toHaveLength(1)
    expect(acc[0]).toMatchObject({
      type: 'ACCEPT',
      from: 's1',
      to: 'B',
      props: {
        modality: 'inferred',
        provenance: {
          kind: 'disjunctive',
          rule: 'resolve-disjunction',
        },
      },
    })
    expect(acc[0]?.props?.provenance?.sources).toEqual(
      expect.arrayContaining([
        { type: 'ONE_OF', from: 's1', to: 'A' },
        { type: 'ONE_OF', from: 's1', to: 'B' },
        { type: 'NEGATED', from: 's1', to: 'A' },
      ]),
    )

    // Idempotent
    const twice = applyCanonRules(once, [ruleResolveDisjunction])
    const acc2 = (twice.edges ?? []).filter(e => e.type === 'ACCEPT')
    expect(acc2).toHaveLength(1)
  })
})
