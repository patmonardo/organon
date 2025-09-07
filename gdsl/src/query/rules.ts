import type { GraphArtifact } from '../schema/projection'
import type { Rule, BodyPred, EdgePred, LabelPred, PropContainsPred } from './ast'

type NodeRow = { id: string; labels?: string[]; props?: Record<string, any> }
type EdgeRow = { type: string; from: string; to: string; props?: Record<string, any> }
type Binding = Record<string, any>

function getNodeByIdMap(nodes: NodeRow[]) {
  const m = new Map<string, NodeRow>()
  for (const n of nodes) m.set(String(n.id), n)
  return m
}

function matchEdgePred(edges: EdgeRow[], nodesById: Map<string, NodeRow>, pred: EdgePred): Binding[] {
  const out: Binding[] = []
  for (const e of edges) {
    if (pred.type && String(e.type) !== pred.type) continue
    const a = nodesById.get(String(e.from))
    const b = nodesById.get(String(e.to))
    if (!a || !b) continue
    out.push({ [pred.from]: a, [pred.to]: b, [`${pred.from}__edge__${pred.to}`]: e })
  }
  return out
}

function matchLabelPred(nodes: NodeRow[], pred: LabelPred): Binding[] {
  const out: Binding[] = []
  for (const n of nodes) {
    const ok = (n.labels ?? []).includes(pred.label)
    if (ok) out.push({ [pred.v]: n })
  }
  return out
}

function matchPropContains(nodes: NodeRow[], pred: PropContainsPred): Binding[] {
  const out: Binding[] = []
  for (const n of nodes) {
    let cur: any = n
    for (const p of pred.path) {
      cur = cur?.[p]
      if (cur === undefined || cur === null) break
    }
    const hay = String(cur ?? '').toLowerCase()
    if (hay.includes(String(pred.value).toLowerCase())) out.push({ [pred.v]: n })
  }
  return out
}

function evalPred(nodes: NodeRow[], edges: EdgeRow[], nodesById: Map<string, NodeRow>, p: BodyPred): Binding[] {
  switch (p.kind) {
    case 'edge': return matchEdgePred(edges, nodesById, p)
    case 'label': return matchLabelPred(nodes, p)
    case 'propContains': return matchPropContains(nodes, p)
    default: return []
  }
}

function joinBindings(a: Binding[], b: Binding[]) {
  if (a.length === 0) return b
  if (b.length === 0) return a
  const out: Binding[] = []
  for (const x of a) {
    for (const y of b) {
      let ok = true
      for (const k of Object.keys(y)) {
        if (x[k] !== undefined && x[k] !== y[k]) { ok = false; break }
      }
      if (ok) out.push({ ...x, ...y })
    }
  }
  return out
}

export function evaluateRules(artifact: GraphArtifact, rules: Rule[], maxIterations = 8): EdgeRow[] {
  if (!rules?.length) return []
  const nodes = (artifact.nodes ?? []) as any as NodeRow[]
  const baseEdges = (artifact.edges ?? []) as any as EdgeRow[]
  const nodesById = getNodeByIdMap(nodes)

  const derived: EdgeRow[] = []
  const hasEdge = (type: string, from: string, to: string) => {
    return baseEdges.some(e => e.type === type && String(e.from) === from && String(e.to) === to) ||
           derived.some(e => e.type === type && String(e.from) === from && String(e.to) === to)
  }

  let changed = false
  let iter = 0
  do {
    changed = false
    iter++
    const allEdges = baseEdges.concat(derived)
    for (const rule of rules) {
      // evaluate body to bindings
      let binds: Binding[] = []
      for (const lit of rule.body) {
        if (lit.negated) continue // reserved: ignore for MVP
        const bs = evalPred(nodes, allEdges, nodesById, lit.pred)
        binds = joinBindings(binds, bs)
        if (binds.length === 0) break
      }
      // project head and add edges
      for (const b of binds) {
        const fromNode = b[rule.head.from]
        const toNode = b[rule.head.to]
        if (!fromNode || !toNode) continue
        const from = String(fromNode.id)
        const to = String(toNode.id)
        if (hasEdge(rule.head.type, from, to)) continue
        derived.push({ type: rule.head.type, from, to, props: { derivedBy: rule.name ?? 'rule' } })
        changed = true
      }
    }
  } while (changed && iter < maxIterations)

  return derived
}
