import type { GraphArtifact } from '../schema/projection'
import type { Facets } from '../schema/signature'
import { computeTermSignatures, facetWeightsFromFacets } from '../dataset/signatures'
import { findTerms } from '../topic/search'
import { createMockGDS, type GDSProvider } from './gds'

export type AgentContext = {
  artifact: GraphArtifact
  facets: Facets
}

export type AgentTools = {
  // Topic-like find over artifact terms (boosted by facet weights)
  find: (query: string, limit?: number) => Array<{ id: string; score: number; term: any }>
  // Simple facet summary (token -> weight)
  facetWeights: () => Record<string, number>
  // Experimental: run a Cypher-Lite AST (only if GDSL_QUERY_RULES=1)
  query: (ast: any) => Promise<any[]>
  // GDS facade (mock by default; pluggable)
  gds: GDSProvider
}

export function createAgentContext(artifact: GraphArtifact, opts?: { facets?: Facets }): AgentContext {
  return {
    artifact,
    facets: opts?.facets ?? computeTermSignatures(artifact),
  }
}

export function createAgentTools(ctx: AgentContext, opts?: { gds?: GDSProvider }): AgentTools {
  const weights = facetWeightsFromFacets(ctx.facets)
  const gds = opts?.gds ?? createMockGDS(ctx.artifact)

  // minimal Topic shape from artifact.terms
  const topic = {
    id: `topic:${ctx.artifact.dataset}`,
    title: ctx.artifact.dataset,
    terms: (ctx.artifact.terms ?? []).map((t: any) => ({
      id: String(t.id ?? t.term ?? ''),
      label: String(t.label ?? t.name ?? t.id ?? ''),
      aliases: t.aliases ?? [],
      desc: t.desc ?? t.description ?? '',
      tags: t.tags ?? {},
    })),
    edges: [],
  }

  return {
    find: (query: string, limit = 25) => findTerms(topic as any, query, { limit, facetWeights: weights }),
    facetWeights: () => weights,
    query: async (ast: any) => {
      if (process.env.GDSL_QUERY_RULES !== '1') {
        throw new Error('query engine disabled (set GDSL_QUERY_RULES=1 to enable)')
      }
      const { QueryEngine } = await import('../query/engine')
      const engine = new QueryEngine({ artifact: ctx.artifact })
      return engine.execute(ast)
    },
    gds,
  }
}
