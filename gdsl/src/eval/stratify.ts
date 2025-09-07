import type { Program, Rule, Atom, Literal } from '../core'

type Edge = { from: string; to: string; neg: boolean }

function headPred(r: Rule) { return r.head.pred }
function deps(r: Rule): Edge[] {
  const edges: Edge[] = []
  const from = headPred(r)
  for (const lit of r.body) {
    if ((lit as any).pred) {
      const a = lit as Atom
      edges.push({ from, to: a.pred, neg: false })
    } else if ((lit as any).kind === 'neg') {
      const a = (lit as any).atom as Atom
      edges.push({ from, to: a.pred, neg: true })
    }
  }
  return edges
}

export function stratify(p: Program) {
  const preds = new Set<string>()
  p.rules.forEach(r => preds.add(headPred(r)))
  p.facts.forEach(f => preds.add(f.atom.pred))
  const edges = p.rules.flatMap(deps)

  // Kahn-style topo by increasing stratum; neg edges force higher stratum
  const nodes = Array.from(preds)
  const idx = new Map(nodes.map((n, i) => [n, i]))
  const stratum = new Map<string, number>(nodes.map(n => [n, 0]))

  let changed = true
  while (changed) {
    changed = false
    for (const e of edges) {
      const sFrom = stratum.get(e.from) ?? 0
      const sTo = stratum.get(e.to) ?? 0
      const needed = sTo + (e.neg ? 1 : 0)
      if (sFrom < needed) {
        stratum.set(e.from, needed)
        changed = true
      }
    }
  }

  // detect cycles through negation: if any neg edge e: from==to or cycle with neg raising stratum inconsistently
  // Minimal check: if a strongly connected component has a neg edge inside, reject.
  const adj = new Map<string, string[]>()
  nodes.forEach(n => adj.set(n, []))
  for (const e of edges) adj.get(e.from)!.push(e.to)

  // Kosaraju SCC
  const order: string[] = []
  const seen = new Set<string>()
  function dfs1(n: string) {
    if (seen.has(n)) return
    seen.add(n)
    for (const m of adj.get(n) ?? []) dfs1(m)
    order.push(n)
  }
  nodes.forEach(dfs1)
  const radj = new Map<string, string[]>()
  nodes.forEach(n => radj.set(n, []))
  for (const e of edges) radj.get(e.to)!.push(e.from)
  const comp: string[] = []
  function dfs2(n: string) {
    if (seen.has(n)) return
    seen.add(n)
    comp.push(n)
    for (const m of radj.get(n) ?? []) dfs2(m)
  }
  seen.clear()
  for (let i = order.length - 1; i >= 0; i--) {
    comp.length = 0
    dfs2(order[i])
    if (comp.length > 1) {
      const inside = new Set(comp)
      if (edges.some(e => e.neg && inside.has(e.from) && inside.has(e.to))) {
        throw new Error('Unstratified negation: neg edge inside SCC')
      }
    }
  }

  const maxStratum = Math.max(...Array.from(stratum.values()))
  const strata: string[][] = Array.from({ length: maxStratum + 1 }, () => [])
  for (const [pred, s] of stratum) strata[s].push(pred)
  return { strata, stratum }
}
