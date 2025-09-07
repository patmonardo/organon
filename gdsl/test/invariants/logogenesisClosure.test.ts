import { describe, it, expect } from 'vitest'
import { applyCanonRules, ruleHypothesizeFromConjunction, ruleResolveDisjunction } from '../../src/dataset/canon-rules'
import { explainDerivedEdge, artifactFromSources } from '../../src/dataset/explain'

describe('Logogenesis cycle closure — Truth reveals Cycle; Cycle reveals Truth', () => {
  it('ACCEPT provenance reconstructs sufficient sources that re-derive the same ACCEPT', () => {
    // Base truths (Conjunction + Hypothetical map + Disjunctive evidence)
    const base = {
      dataset: 'logo.closure',
      nodes: [{ id: 's1' }, { id: 'fA' }, { id: 'fB' }, { id: 'A' }, { id: 'B' }],
      edges: [
        { type: 'HAS', from: 's1', to: 'fA' },
        { type: 'HAS', from: 's1', to: 'fB' },
        { type: 'IMPLIES', from: 'fA', to: 'A' },
        { type: 'IMPLIES', from: 'fB', to: 'B' },
        { type: 'NEGATED', from: 's1', to: 'A' },
      ],
      terms: [],
    } as any

    // Derive ONE_OF then ACCEPT (Cycle forward)
    const afterHyp = applyCanonRules(base, [ruleHypothesizeFromConjunction])
    const afterDisj = applyCanonRules(afterHyp, [ruleResolveDisjunction])
    const accept = (afterDisj.edges ?? []).find((e: any) => e.type === 'ACCEPT')
    expect(accept).toBeTruthy()
    expect(accept).toMatchObject({ from: 's1', to: 'B' })

    // Explain ACCEPT back to sources (Truth extraction from Knowledge)
    const sources = explainDerivedEdge(accept as any)
    expect(sources.length).toBeGreaterThan(0)

    // Rebuild minimal artifact from sources and re-derive (Cycle closure)
    const minimal = artifactFromSources('logo.closure.replay', sources)
    const replay = applyCanonRules(
      applyCanonRules(minimal, [ruleHypothesizeFromConjunction]),
      [ruleResolveDisjunction],
    )
    const accept2 = (replay.edges ?? []).find((e: any) => e.type === 'ACCEPT')

    // Oneness: same categorical result
    expect(accept2).toBeTruthy()
    expect(accept2).toMatchObject({ from: 's1', to: 'B' })
  })
})
