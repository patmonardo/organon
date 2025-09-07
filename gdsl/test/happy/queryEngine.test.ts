import { describe, it, expect } from 'vitest';
import { QueryEngine, type QueryAST } from '../../src/query/engine';

describe('QueryEngine (happy path)', () => {
  it('MATCH one-hop with typed edge, WHERE, RETURN, LIMIT', async () => {
    // minimal GraphArtifact fixture
    const artifact = {
      dataset: 'test.ds',
      nodes: [
        { id: 't1', labels: ['Term'], props: { label: 'Alpha Beta' } },
        { id: 't2', labels: ['Term'], props: { label: 'Gamma Delta' } },
        { id: 'x1', labels: ['Other'], props: { label: 'Irrelevant' } },
      ],
      edges: [
        { type: 'ALIGNS_WITH', from: 't1', to: 't2', props: {} },
        { type: 'NOISE', from: 'x1', to: 't1', props: {} },
      ],
      clauses: [],
      tokens: [],
      terms: [],
      counts: {},
    } as any;

    const engine = new QueryEngine({ artifact });

    const ast: QueryAST = {
      match: [
        {
          nodeVars: [
            { var: 'a', labels: ['Term'] },
            { var: 'b', labels: ['Term'] },
          ],
          edge: { from: 'a', type: 'ALIGNS_WITH', to: 'b' },
        },
      ],
      // simple function predicate (safe, local)
      where: (binding) =>
        String(binding.a?.props?.label ?? '').includes('Alpha'),
      return: [
        { expr: 'a.id', as: 'from' },
        { expr: 'b.id', as: 'to' },
      ],
      limit: 10,
    };

    const rows = await engine.execute(ast);
    expect(rows).toEqual([{ from: 't1', to: 't2' }]);
  });

  it('respects label constraints and LIMIT', async () => {
    const artifact = {
      dataset: 'test.ds',
      nodes: [
        { id: 'n1', labels: ['A'], props: { label: 'A1' } },
        { id: 'n2', labels: ['B'], props: { label: 'B1' } },
        { id: 'n3', labels: ['B'], props: { label: 'B2' } },
      ],
      edges: [
        { type: 'LINK', from: 'n1', to: 'n2', props: {} },
        { type: 'LINK', from: 'n1', to: 'n3', props: {} },
      ],
      clauses: [],
      tokens: [],
      terms: [],
      counts: {},
    } as any;

    const engine = new QueryEngine({ artifact });
    const ast: QueryAST = {
      match: [
        {
          nodeVars: [
            { var: 'a', labels: ['A'] },
            { var: 'b', labels: ['B'] },
          ],
          edge: { from: 'a', type: 'LINK', to: 'b' },
        },
      ],
      return: [{ expr: 'a.id' }, { expr: 'b.id' }],
      limit: 1,
    };

    const rows = await engine.execute(ast);
    expect(rows.length).toBe(1);
    expect(rows[0]).toHaveProperty('a.id');
    expect(rows[0]).toHaveProperty('b.id');
  });
});
