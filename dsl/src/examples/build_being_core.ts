import { evaluate } from '../engine/evaluator'
import { fact, v, sym } from '../core'
import { being as qualBeing, quality } from '../domains/quality'
import { beingRules } from '../domains/being'

// Subjects
const A = v('a')
const B = v('b')

// Facts:
// A: only being (no qualities) → pure/nothing/becoming
// B: being + one quality → determinate (existence via quality domain)
const facts = [
  fact(qualBeing(A)),
  fact(qualBeing(B)),
  // use Sym Term, not raw string
  fact(quality(B, sym('clarity')))
]

// Program: just being rules + quality facts (existence is derived by quality.ts)
const prog = { facts, rules: [...beingRules()], meta: { tags: ['demo:being'] } }
const db = evaluate(prog)

type Tup = (string | number)[]

function rows(pred: string): Tup[] {
  const r = db.rels.get(pred)
  if (!r) return []
  // don’t use .map(JSON.parse); use explicit mapper
  return Array.from(r.rows, (s: string) => JSON.parse(s) as Tup)
}

console.log('pure:', rows('being:pure'))
console.log('nothing:', rows('being:nothing'))
console.log('becoming:', rows('being:becoming'))
console.log('existence:', rows('qual:existence'))
