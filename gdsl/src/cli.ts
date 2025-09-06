import { readFileSync } from 'node:fs'
import { evaluate } from './engine/evaluator'
import { validate, type Program } from './core'
import { emitSouffle } from './emit/souffle'
import { emitClingo } from './emit/clingo'

const file = process.argv[2]
if (!file) {
  console.error('usage: tsx logic/dsl/cli.ts path/to/program.json')
  process.exit(1)
}
const prog = JSON.parse(readFileSync(file, 'utf8')) as Program
console.log('validate:', validate(prog))
const db = evaluate(prog)
console.log('relations:', Array.from(db.rels.entries()).map(([k, v]) => [k, v.rows.size]))
const { dl } = emitSouffle(prog)
console.log('\n;; Soufflé\n', dl)
console.log('\n;; Clingo\n', emitClingo(prog))
