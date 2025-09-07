import { describe, it, expect } from 'vitest'
import { canonicalizeManifest } from '../../src/dataset/canonicalize'

describe('canonicalizeManifest (happy path)', () => {
  it('produces a validated artifact and applies default rules', async () => {
    const manifest = {
      id: 'example.ds',
      hlos: [
        {
          id: 'h1',
          label: 'Hello',
          tokens: ['hello'],
          clauses: ['assert(Alpha(x))', 'tag(topic,"greeting")'],
          links: [{ type: 'LINK', from: 'h1', to: 'h2', props: {} }],
        },
        { id: 'h2', label: 'World', tokens: ['world'], clauses: [] },
      ],
      terms: [
        { id: 't1', tokens: ['alpha'], aliases: ['a'] },
        { id: 't2', tokens: ['beta'], aliases: [] },
      ],
      signatureTokens: ['sigA', 'sigB'],
    }

    const artifact = await canonicalizeManifest(manifest, { write: false, outDir: 'dist/test' })
    expect(artifact.dataset).toBe('example.ds')
    // nodes include HLO nodes
    expect(artifact.nodes.length).toBe(2)
    // edges include LINK (and deduped)
    expect(artifact.edges.length).toBe(1)
    // clauses parsed and counted
    expect(artifact.counts?.clauses).toBe(2)
    expect(artifact.counts?.asserts).toBe(1)
    expect(artifact.counts?.tags).toBe(1)
    // signatures present from signatureTokens
    expect(artifact.signatures && artifact.signatures['default']?.length).toBe(2)
    // tokens present (original + inferred by rules if needed)
    expect(Array.isArray(artifact.tokens)).toBe(true)
  })

  it('dedupes duplicate edges and recomputes counts', async () => {
    const manifest = {
      id: 'dedupe.ds',
      hlos: [
        {
          id: 'h1',
          label: 'One',
          links: [
            { type: 'LINK', from: 'h1', to: 'h2' },
            { type: 'LINK', from: 'h1', to: 'h2' }, // duplicate
          ],
          clauses: [],
        },
        { id: 'h2', label: 'Two', clauses: [] },
      ],
    }
    const artifact = await canonicalizeManifest(manifest, { write: false, outDir: 'dist/test' })
    expect(artifact.edges.length).toBe(1)
    expect(artifact.counts?.edges).toBe(1)
    expect(artifact.counts?.nodes).toBe(2)
  })
})
