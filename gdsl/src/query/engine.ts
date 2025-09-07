export type Row = Record<string, any>

import type { GraphArtifact } from '../schema/projection'
import type { Rule } from './ast'
import { evaluateRules } from './rules'

export type GraphArtifactLike = GraphArtifact

export type MatchPattern = {
  nodeVars: Array<{ var: string; labels?: string[] }>
  edge?: { from: string; type?: string; to: string } // var names
}

export type ReturnExpr = { expr: string; as?: string }

export type QueryAST = {
  match: MatchPattern[]
  where?: ((binding: Record<string, any>) => boolean) | { var: string; prop?: string; op: '==' | 'in' | 'contains'; value: any }
  return: ReturnExpr[]
  limit?: number
  rules?: Rule[] // <- new: optional rule set to derive edges pre-match
}

export type ExecContext = {
  artifact: GraphArtifactLike
  facets?: Record<string, Array<{ token: string; weight?: number }>>
}

/**
 * Minimal, serverless query engine over an in-memory GraphArtifact.
 * - supports simple MATCH patterns with optional typed edge expansion
 * - WHERE can be a predicate function or a simple clause {var,prop,op,value}
 * - RETURN expressions are simple var.property paths like "t.id" or "t.label"
 *
 * This is intentionally small and synchronous for MVP. It is a foundation
 * for a planner/optimizer and a parser later.
 */
export class QueryEngine {
  ctx: ExecContext
  private nodeById: Map<string, any>
  private nodesByLabel: Map<string, any[]>
  private outEdges: Map<string, any[]>
  private inEdges: Map<string, any[]>
  private edgesByType: Map<string, any[]>

  constructor(ctx: ExecContext) {
    this.ctx = ctx
    const art = ctx.artifact
    this.nodeById = new Map()
    this.nodesByLabel = new Map()
    this.outEdges = new Map()
    this.inEdges = new Map()
    this.edgesByType = new Map()

    for (const n of art.nodes ?? []) {
      const id = String((n as any).id)
      this.nodeById.set(id, n)
      for (const l of (n as any).labels ?? []) {
        const arr = this.nodesByLabel.get(l) ?? []
        arr.push(n)
        this.nodesByLabel.set(l, arr)
      }
    }

    for (const e of art.edges ?? []) {
      const from = String((e as any).from)
      const to = String((e as any).to)
      const type = String((e as any).type ?? '')
      const out = this.outEdges.get(from) ?? []
      out.push(e)
      this.outEdges.set(from, out)
      const inp = this.inEdges.get(to) ?? []
      inp.push(e)
      this.inEdges.set(to, inp)
      const byType = this.edgesByType.get(type) ?? []
      byType.push(e)
      this.edgesByType.set(type, byType)
    }
  }

  private matchNodesByLabel(labels?: string[]) {
    if (!labels || labels.length === 0) {
      return Array.from(this.nodeById.values())
    }
    const results: any[] = []
    for (const l of labels) {
      const arr = this.nodesByLabel.get(l)
      if (arr) results.push(...arr)
    }
    // de-dup by id
    const seen = new Set<string>()
    return results.filter((n) => {
      const id = String((n as any).id)
      if (seen.has(id)) return false
      seen.add(id)
      return true
    })
  }

  private simpleWhereEval(binding: Record<string, any>, clause: { var: string; prop?: string; op: '==' | 'in' | 'contains'; value: any }) {
    const v = binding[clause.var]
    if (v === undefined) return false
    const field = clause.prop ? (v as any)[clause.prop] : v
    switch (clause.op) {
      case '==':
        return field === clause.value
      case 'in':
        if (Array.isArray(field)) return field.includes(clause.value)
        if (typeof field === 'string') return String(field) === String(clause.value)
        return false
      case 'contains':
        if (typeof field === 'string') return field.toLowerCase().includes(String(clause.value).toLowerCase())
        if (Array.isArray(field)) return field.includes(clause.value)
        return false
      default:
        return false
    }
  }

  private evalReturnExpr(binding: Record<string, any>, expr: string) {
    // expr like "t.id" or "t.props.label"
    const parts = expr.split('.')
    if (parts.length === 0) return undefined
    const v = parts[0]
    let cur: any = binding[v]
    for (let i = 1; i < parts.length; i++) {
      if (cur == null) return undefined
      cur = (cur as any)[parts[i]]
    }
    return cur
  }

  async execute(ast: QueryAST): Promise<Row[]> {
    // new: materialize derived edges from rules before pattern matching
    const derivedEdges = ast.rules?.length ? evaluateRules(this.ctx.artifact, ast.rules) : []
    // rebuild edge indexes for this execution using base + derived
    const allEdges = ([] as any[]).concat(this.ctx.artifact.edges ?? [], derivedEdges)
    const outEdges = new Map<string, any[]>()
    const inEdges = new Map<string, any[]>()
    const edgesByType = new Map<string, any[]>()
    for (const e of allEdges) {
      const from = String(e.from), to = String(e.to), type = String(e.type ?? '')
      const o = outEdges.get(from) ?? []; o.push(e); outEdges.set(from, o)
      const i = inEdges.get(to) ?? []; i.push(e); inEdges.set(to, i)
      const t = edgesByType.get(type) ?? []; t.push(e); edgesByType.set(type, t)
    }

    // helper to close over these per-exec indexes
    const getOut = (id: string) => outEdges.get(id) ?? []
    const getEdgesByType = (t: string) => edgesByType.get(t) ?? []

    // 1) for each MATCH pattern produce bindings (array of maps var->node)
    const patternBindings: Array<Record<string, any>[]> = []

    for (const pat of ast.match) {
      const vars = pat.nodeVars
      if (vars.length === 1 && !pat.edge) {
        const v = vars[0]
        const candidates = this.matchNodesByLabel(v.labels)
        const binds = candidates.map((n) => ({ [v.var]: n }))
        patternBindings.push(binds)
        continue
      }

      if (vars.length >= 2 && pat.edge) {
        const fromVar = pat.edge.from
        const toVar = pat.edge.to
        const type = pat.edge.type
        const fromSpec = vars.find((x) => x.var === fromVar)
        const toSpec = vars.find((x) => x.var === toVar)

        const fromCandidates = this.matchNodesByLabel(fromSpec?.labels)
        const binds: Record<string, any>[] = []

        for (const f of fromCandidates) {
          const fId = String((f as any).id)
          const outs = type ? getEdgesByType(type).filter(e => String(e.from) === fId) : getOut(fId)
          for (const e of outs) {
            const tgt = this.nodeById.get(String((e as any).to))
            if (!tgt) continue
            if (toSpec?.labels && toSpec.labels.length > 0) {
              const ok = (toSpec.labels ?? []).some((lab) => ((tgt as any).labels ?? []).includes(lab))
              if (!ok) continue
            }
            binds.push({ [fromVar]: f, [toVar]: tgt, [`${fromVar}__edge__${toVar}`]: e })
          }
        }
        patternBindings.push(binds)
        continue
      }

      // fallback: expand each var independently and do cartesian product
      const listPerVar = vars.map((v) => this.matchNodesByLabel(v.labels).map((n) => ({ [v.var]: n })))
      // cartesian join
      const joined: Record<string, any>[] = []
      function cartesianJoin(acc: Record<string, any>[], next: Record<string, any>[]) {
        if (acc.length === 0) return next
        const out: Record<string, any>[] = []
        for (const a of acc) for (const b of next) out.push({ ...a, ...b })
        return out
      }
      let acc: Record<string, any>[] = []
      for (const list of listPerVar) acc = cartesianJoin(acc, list)
      patternBindings.push(acc)
    }

    // 2) join bindings from multiple patterns
    let bindings: Record<string, any>[] = patternBindings.length ? patternBindings[0].slice() : [{}]
    for (let i = 1; i < patternBindings.length; i++) {
      const next = patternBindings[i]
      const joined: Record<string, any>[] = []
      for (const a of bindings) {
        for (const b of next) {
          // ensure consistency on overlapping vars
          let ok = true
          for (const k of Object.keys(b)) {
            if (a[k] !== undefined && a[k] !== b[k]) { ok = false; break }
          }
          if (ok) joined.push({ ...a, ...b })
        }
      }
      bindings = joined
    }

    // 3) apply WHERE filter
    const where = ast.where
    const filtered = bindings.filter((binding) => {
      if (!where) return true
      if (typeof where === 'function') {
        try { return Boolean(where(binding)) } catch { return false }
      }
      return this.simpleWhereEval(binding, where)
    })

    // 4) project RETURN expressions
    const rows: Row[] = []
    for (const b of filtered) {
      const row: Row = {}
      for (const r of ast.return) {
        const val = this.evalReturnExpr(b, r.expr)
        row[r.as ?? r.expr] = val
      }
      rows.push(row)
      if (ast.limit && rows.length >= ast.limit) break
    }

    return rows
  }
}
