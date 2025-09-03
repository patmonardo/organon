import { evaluate } from '../engine/evaluator'
import { fact, sym } from '../core'
import {
  triad, organicUnity, memberOf, duality, asProgram,
  emergesFrom, recognizes, rosary
} from '../domains/metaphysics'

const Being = sym('Being'), Nothing = sym('Nothing'), Becoming = sym('Becoming'), Existence = sym('Existence')
const Sattva = sym('Sattva'), Rajas = sym('Rajas'), Tamas = sym('Tamas')
const Gunas = sym('Gunas')
const D_BN = sym('Duality:Being~Nothing')
const Absolute = sym('Absolute')

// Add an organic unity for the triad’s members, so Existence can “emerge from” it
const BNB = sym('Unity:BNB') // organic unity containing Being/Nothing/Becoming

const facts = [
  fact(triad(Existence, Being, Nothing, Becoming)),

  // Organic unity for Gunas
  fact(organicUnity(Gunas)),
  fact(memberOf(Sattva, Gunas)),
  fact(memberOf(Tamas, Gunas)),
  fact(memberOf(Rajas, Gunas)),

  // Organic unity for B/N/B
  fact(organicUnity(BNB)),
  fact(memberOf(Being, BNB)),
  fact(memberOf(Nothing, BNB)),
  fact(memberOf(Becoming, BNB)),

  // A duality
  fact(duality(D_BN, Being, Nothing)),
]

const prog = asProgram(facts)
const db = evaluate(prog)

type Tup = (string | number)[]
const rows = (pred: string): Tup[] => {
  const rel = db.rels.get(pred)
  if (!rel) return []
  const arr = Array.from(rel.rows, (s: string) => JSON.parse(s) as Tup)
  return arr.sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)))
}

console.log('simpleUnity:', rows('meta:simpleUnity'))     // ['Existence']
console.log('presupposes:', rows('meta:presupposes'))
console.log('viyoga:', rows('meta:viyoga'))
console.log('organicUnity:', rows('meta:organicUnity'))
console.log('sanyoga:', rows('meta:sanyoga'))
console.log('emergesFrom:', rows('meta:emergesFrom'))     // ['Existence','Unity:BNB']
console.log('recognizes:', rows('meta:recognizes'))       // ['Absolute','Existence']
console.log('rosary:', rows('meta:rosary'))               // ['Existence']
