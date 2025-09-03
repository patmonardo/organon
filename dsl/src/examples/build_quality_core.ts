import { evaluate } from '../engine/evaluator'
import { atom, fact, sym, v, str, type Program } from '../core'
import { asProgram, quality, dominates, opposes } from '../domains/quality'

const S = v('s1') // subject

const facts: Program['facts'] = [
  fact(quality(S, 'clarity')),
  fact(quality(S, 'activity')),
  fact(dominates(S, 'clarity')),
  fact(opposes('clarity', 'confusion')),
  fact(opposes('activity', 'inertia'))
]

const prog = asProgram(facts)
const db = evaluate(prog)

type Tup = (string | number)[]

function rows(pred: string): Tup[] {
  const rel = db.rels.get(pred)
  if (!rel) return []
  return Array.from(rel.rows, (s: string) => JSON.parse(s) as Tup)
}

console.log('existence:', rows('qual:existence'))
console.log('self:', rows('qual:self'))
console.log('qualities:', rows('qual:quality'))
console.log('tags:', rows('tag'))
