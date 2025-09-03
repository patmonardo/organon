import type { LogicalOperation } from '../logic/existence/index'
import { parseClauses, type Clause } from './parse_clauses'

export type Judgment =
  | { id: string; type: 'Affirmative'; subject: string; predicate: string; object: string; source: string }
  | { id: string; type: 'Negative'; subject: string; predicate: string; object: string; source: string }
  | { id: string; type: 'Relational'; subject: string; relation: string; object: string; source: string }

let auto = 0
const jid = (prefix: string) => `${prefix}-${++auto}`

function jAff(source: string, s: string, p: string, o: string): Judgment {
  return { id: jid('J'), type: 'Affirmative', subject: s, predicate: p, object: o, source }
}
function jNeg(source: string, s: string, p: string, o: string): Judgment {
  return { id: jid('J'), type: 'Negative', subject: s, predicate: p, object: o, source }
}
function jRel(source: string, s: string, r: string, o: string): Judgment {
  return { id: jid('J'), type: 'Relational', subject: s, relation: r, object: o, source }
}

// Very small pattern set; extend as needed.
function judgmentsFrom(op: LogicalOperation, clauses: Clause[]): Judgment[] {
  const J: Judgment[] = []
  for (const c of clauses) {
    if (c.kind !== 'assert') continue
    const a = c.args
    switch (c.name) {
      case 'equals': {
        if (a.length === 2) J.push(jAff(op.id, a[0], 'is', a[1]))
        break
      }
      case 'otherOf': {
        if (a.length === 2) J.push(jRel(op.id, a[0], 'otherOf', a[1]))
        break
      }
      case 'principleOf':
      case 'elementOf':
      case 'produces':
      case 'pointsTo':
      case 'momentOf':
      case 'belongsTo':
      case 'determinationOf': {
        if (a.length === 2) J.push(jRel(op.id, a[0], c.name, a[1]))
        break
      }
      case 'notMerelyRelativeTo':
      case 'notProduces': {
        if (a.length === 2) J.push(jNeg(op.id, a[0], c.name, a[1]))
        break
      }
      case 'identityOf': {
        // identityOf(["Existence","Limit"]) -> pairwise Affirmatives
        try {
          const arr = JSON.parse(a[0]) as string[]
          for (let i = 1; i < arr.length; i++) J.push(jAff(op.id, arr[0], 'is', arr[i]))
        } catch {}
        break
      }
      case 'noAbsolute': {
        try {
          const arr = JSON.parse(a[0]) as string[]
          for (const x of arr) J.push(jNeg(op.id, x, 'is', 'absolute'))
        } catch {}
        break
      }
    }
  }
  return J
}

export function extractJudgments(ops: LogicalOperation[]): Judgment[] {
  const out: Judgment[] = []
  for (const op of ops) out.push(...judgmentsFrom(op, parseClauses(op)))
  return out
}
