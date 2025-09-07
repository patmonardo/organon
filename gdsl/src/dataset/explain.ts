export type EdgeRow = { type: string; from: string; to: string; props?: Record<string, any> }

/**
 * Read provenance sources from a derived edge (if present).
 * Expected shape placed by canon rules:
 *   edge.props.provenance.sources: Array<{ type, from, to }>
 */
export function explainDerivedEdge(edge: EdgeRow): EdgeRow[] {
  const srcs = edge?.props?.provenance?.sources
  if (!Array.isArray(srcs)) return []
  return srcs.map((s: any) => ({
    type: String(s.type ?? ''),
    from: String(s.from ?? ''),
    to: String(s.to ?? ''),
  }))
}

/**
 * Given a set of source edges, build a minimal artifact to re-run derivations.
 */
export function artifactFromSources(dataset: string, sources: EdgeRow[]) {
  return {
    dataset,
    nodes: [], // nodes not required for current disjunctive/hypothetical rules
    edges: sources.slice(),
    terms: [],
    clauses: [],
  } as any
}
