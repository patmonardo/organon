import type { LogicalOperation } from './index'
import { parseClause } from '../parser/clause-parser'

export type PredicateStats = {
  count: number
  examples: { hloId: string; clause: string }[]
}

export type AnalysisReport = {
  totals: { hlos: number; clauses: number; asserts: number; tags: number; unknown: number }
  predicates: Record<string, PredicateStats>
  tags: Record<string, PredicateStats>
  unknownClauses: { hloId: string; clause: string }[]
}

export function analyzePredicates(hlos: LogicalOperation[]): AnalysisReport {
  const report: AnalysisReport = {
    totals: { hlos: 0, clauses: 0, asserts: 0, tags: 0, unknown: 0 },
    predicates: {},
    tags: {},
    unknownClauses: []
  }
  report.totals.hlos = hlos.length
  for (const h of hlos) {
    for (const c of h.clauses ?? []) {
      report.totals.clauses++
      const ast = parseClause(c)
      if (ast.verb === 'assert') {
        report.totals.asserts++
        const name = ast.fn
        const entry = report.predicates[name] ?? { count: 0, examples: [] }
        entry.count++
        if (entry.examples.length < 3) entry.examples.push({ hloId: h.id, clause: c })
        report.predicates[name] = entry
      } else if (ast.verb === 'tag') {
        report.totals.tags++
        const name = `${ast.key}:${JSON.stringify(ast.value)}`
        const entry = report.tags[name] ?? { count: 0, examples: [] }
        entry.count++
        if (entry.examples.length < 3) entry.examples.push({ hloId: h.id, clause: c })
        report.tags[name] = entry
      } else {
        report.totals.unknown++
        report.unknownClauses.push({ hloId: h.id, clause: c })
      }
    }
  }
  return report
}
