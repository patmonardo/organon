import test from 'node:test'
import assert from 'node:assert/strict'
import { evaluate } from '../engine/evaluator'
import { fact, sym } from '../core'
import { triad, organicUnity, memberOf, duality, asProgram } from '../domains/metaphysics'

test('metaphysics basics', () => {
  const [Being, Nothing, Becoming, Existence] = ['Being','Nothing','Becoming','Existence'].map(sym)
  const [Sattva, Rajas, Tamas, Gunas] = ['Sattva','Rajas','Tamas','Gunas'].map(sym)
  const D_BN = sym('Duality:Being~Nothing')

  const prog = asProgram([
    fact(triad(Existence, Being, Nothing, Becoming)),
    fact(organicUnity(Gunas)),
    fact(memberOf(Sattva, Gunas)),
    fact(memberOf(Tamas, Gunas)),
    fact(memberOf(Rajas, Gunas)),
    fact(duality(D_BN, Being, Nothing))
  ])
  const db = evaluate(prog)
  const rows = (p: string) => Array.from(db.rels.get(p)?.rows ?? [], (s: string) => JSON.parse(s) as (string | number)[])

  assert.deepEqual(rows('meta:simpleUnity'), [[ 'Existence' ]])
  assert.deepEqual(rows('meta:organicUnity'), [[ 'Gunas' ]])
  assert.ok(rows('meta:sanyoga').length >= 3)
})
