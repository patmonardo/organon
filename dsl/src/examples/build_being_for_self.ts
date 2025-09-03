import { evaluate } from '../engine/evaluator'
import { fact, sym } from '../core'
import { asProgram, bodyMind, part, center, body, mind, forSelf, one, repels, attracts } from '../domains/being_for_self'

const C = sym('SelfComplex')
const X = sym('Self')     // the “One” center
const B = sym('Body')
const M = sym('Mind')

const prog = asProgram([
  fact(bodyMind(C)),
  fact(body(B)), fact(mind(M)),
  fact(part(C, B)), fact(part(C, M)),
  fact(center(C, X))
])

const db = evaluate(prog)

type Tup = (string | number)[]
const rows = (pred: string): Tup[] => {
  const rel = db.rels.get(pred)
  if (!rel) return []
  return Array.from(rel.rows, (s: string) => JSON.parse(s) as Tup)
}

console.log('one:', rows('logic:one'))           // [ ['Self'] ]
console.log('forSelf:', rows('logic:forSelf'))   // [ ['Self'] ]
console.log('repels:', rows('logic:repels'))     // [ ['Self','Body'], ['Self','Mind'] ]
console.log('attracts:', rows('logic:attracts')) // [ ['Self','Body'], ['Self','Mind'] ]
