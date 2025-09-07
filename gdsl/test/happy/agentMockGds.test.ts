import { describe, it, expect } from 'vitest'
import { createAgentContext, createAgentTools } from '../../src/agent/kit'

describe('AgentTools.gds (happy path)', () => {
  it('neighbors and degree from mock GDS', () => {
    const artifact = {
      dataset: 'gds.mock',
      nodes: [{ id: 'a' }, { id: 'b' }, { id: 'c' }],
      edges: [
        { type: 'LINK', from: 'a', to: 'b' },
        { type: 'LINK', from: 'a', to: 'c' },
        { type: 'BACK', from: 'b', to: 'a' },
      ],
      terms: [],
    } as any
    const ctx = createAgentContext(artifact)
    const tools = createAgentTools(ctx)

    expect(tools.gds.neighbors('a')).toEqual(expect.arrayContaining(['b', 'c']))
    // filter to LINK to match the comment's intent
    expect(tools.gds.neighbors('a', { dir: 'in', type: 'LINK' })).toEqual([])
    expect(tools.gds.neighbors('a', { dir: 'both' })).toEqual(expect.arrayContaining(['b', 'c']))
    expect(tools.gds.degree('a')).toBe(2)
    expect(tools.gds.neighbors('a', { type: 'LINK' })).toEqual(expect.arrayContaining(['b', 'c']))
  })
})
