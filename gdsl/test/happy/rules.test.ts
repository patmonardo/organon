import { describe, it, expect } from 'vitest'
import { QueryEngine, type QueryAST } from '../../src/query/engine'
import type { Rule } from '../../src/query/ast'

describe('Rules + QueryEngine (happy path)', () => {
  it('derives edges via a simple rule then matches them', async () => {
    const artifact = {
      dataset: 'test.rules',
      nodes: [
        { id: 'a1', labels: ['A'], props: { label: 'Alpha One' } },
        { id: 'b1', labels: ['B'], props: { label: 'Beta One' } },
        { id: 'b2', labels: ['B'], props: { label: 'Beta Two' } },
      ],
      edges: [
        { type: 'LINK', from: 'a1', to: 'b1', props: {} },
        { type: 'LINK', from: 'a1', to: 'b2', props: {} },
      ],
      clauses: [],
      tokens: [],
      terms: [],
      counts: {},
    } as any

    // Rule: RELATED(a,b) :- edge LINK(a,b), label(a,'A'), label(b,'B'), propContains(b, ['props','label'], 'One')
    const rules: Rule[] = [
      {
        name: 'related-AB-one',
        head: { kind: 'edge', type: 'RELATED', from: 'a', to: 'b' },
        body: [
          { pred: { kind: 'edge', type: 'LINK', from: 'a', to: 'b' } },
          { pred: { kind: 'label', v: 'a', label: 'A' } },
          { pred: { kind: 'label', v: 'b', label: 'B' } },
          { pred: { kind: 'propContains', v: 'b', path: ['props', 'label'], value: 'One' } },
        ],
      },
    ]

    const engine = new QueryEngine({ artifact })
    const ast: QueryAST = {
      rules,
      match: [
        {
          nodeVars: [{ var: 'a', labels: ['A'] }, { var: 'b', labels: ['B'] }],
          edge: { from: 'a', type: 'RELATED', to: 'b' },
        },
      ],
      return: [{ expr: 'a.id', as: 'from' }, { expr: 'b.id', as: 'to' }],
      limit: 10,
    }

    const rows = await engine.execute(ast)
    // Only b1 has label 'Beta One', so RELATED(a1,b1) is derived and matched
    expect(rows).toEqual([{ from: 'a1', to: 'b1' }])
  })
})
