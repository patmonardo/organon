import { describe, it, expect } from 'vitest'
import { computeTermSignatures } from '../../src/dataset/signatures'

describe('computeTermSignatures (happy path)', () => {
  it('produces facets for simple terms with tokens and aliases', () => {
    const artifact = {
      dataset: 'test.artifact',
      nodes: [],
      edges: [],
      clauses: [],
      tokens: [],
      terms: [
        { id: 't1', tokens: ['alpha', 'beta'], aliases: ['A'] },
        { id: 't2', tokens: ['beta', 'gamma'], aliases: [] },
      ],
      counts: {},
    } as any

    const facets = computeTermSignatures(artifact)
    expect(Object.keys(facets)).toEqual(expect.arrayContaining(['t1', 't2']))

    const t1 = facets['t1'].map((p: any) => p.token)
    expect(t1).toEqual(expect.arrayContaining(['alpha', 'beta', 'A']))
    const t2 = facets['t2'].map((p: any) => p.token)
    expect(t2).toEqual(expect.arrayContaining(['beta', 'gamma']))
  })
})
