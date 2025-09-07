import { describe, it, expect } from 'vitest'
import { applyCanonRules, ruleDeriveRelatedFromLinkToLabel } from '../../src/dataset/canon-rules'

describe('Logogenesis (happy path)', () => {
  it('Ignorance → Knowledge: derives RELATED via hypothetical syllogism with provenance', () => {
    const artifact = {
      dataset: 'logo.ds',
      nodes: [
        { id: 'a1', labels: ['A'], props: { label: 'Alpha' } },
        { id: 'b1', labels: ['B'], props: { label: 'Beta' } },
        { id: 'c1', labels: ['C'], props: { label: 'Gamma' } },
      ],
      edges: [
        { type: 'LINK', from: 'a1', to: 'b1' },
        { type: 'LINK', from: 'a1', to: 'c1' }, // will not qualify (to has no 'B' label)
      ],
      terms: [],
    } as any

    // Before: ignorance (no RELATED edges)
    expect((artifact.edges ?? []).filter(e => e.type === 'RELATED').length).toBe(0)

    const once = applyCanonRules(artifact, [ruleDeriveRelatedFromLinkToLabel])
    const related = (once.edges ?? []).filter(e => e.type === 'RELATED')
    expect(related.length).toBe(1)
    expect(related[0]).toMatchObject({
      type: 'RELATED',
      from: 'a1',
      to: 'b1',
      props: {
        modality: 'inferred',
        provenance: {
          kind: 'hypothetical',
          rule: 'related-from-link-to-label',
        },
      },
    })
    // Knowledge → Truth: provenance lists source LINK edge(s)
    expect(related[0]?.props?.provenance?.sources).toEqual([{ type: 'LINK', from: 'a1', to: 'b1' }])

    // Idempotent: applying again does not duplicate
    const twice = applyCanonRules(once, [ruleDeriveRelatedFromLinkToLabel])
    const relatedTwice = (twice.edges ?? []).filter(e => e.type === 'RELATED')
    expect(relatedTwice.length).toBe(1)
  })
})
