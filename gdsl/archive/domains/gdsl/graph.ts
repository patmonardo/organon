import { parseClauses } from './parser'

export interface GraphNode { id: string; type: string; data?: any }
export interface GraphEdge { from: string; to: string; rel: string; data?: any }
export interface ExtractResult { nodes: GraphNode[]; edges: GraphEdge[]; errors: string[] }

let idCounter = 0
function gid(prefix: string) { return `${prefix}_${++idCounter}` }

export function extractFromHLO(unitId: string, hloId: string, clauses: string[]): ExtractResult {
  const parsed = parseClauses(clauses)
  const nodes: GraphNode[] = []
  const edges: GraphEdge[] = []
  const errors: string[] = []

  nodes.push({ id: unitId, type: 'Unit' })
  nodes.push({ id: hloId, type: 'HLO', data: { unit: unitId } })
  edges.push({ from: unitId, to: hloId, rel: 'hasHLO' })

  for (const c of parsed) {
    if (!c.valid) { errors.push(`${hloId}: ${c.error} :: ${c.raw}`); continue }
    const clauseNodeId = gid('clause')
    nodes.push({ id: clauseNodeId, type: 'Clause', data: { op: c.operator, raw: c.raw } })
    edges.push({ from: hloId, to: clauseNodeId, rel: 'hasClause' })
    if (c.args?.length) {
      c.args.forEach(arg => {
        const argId = gid('arg')
        nodes.push({ id: argId, type: 'Symbol', data: { label: arg } })
        edges.push({ from: clauseNodeId, to: argId, rel: 'arg' })
      })
    }
  }
  return { nodes, edges, errors }
}
