import { describe, it, expect } from 'vitest'
import { findTerms } from '../../src/topic/search'

describe('topic search (happy path)', () => {
  it('finds and ranks a term by label and alias', () => {
    const topic = {
      id: 'topic:1',
      title: 'Test topic',
      terms: [
        { id: 't1', label: 'Alpha Beta', aliases: ['A'] },
        { id: 't2', label: 'Gamma Delta', aliases: ['G'] },
      ],
      edges: [],
    }

    const results = findTerms(topic as any, 'alpha')
    expect(results.length).toBeGreaterThan(0)
    expect(results[0].id).toBe('t1')
    expect(results[0].score).toBeGreaterThan(0)
  })
})
