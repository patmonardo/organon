#!/usr/bin/env -S node
import { fileURLToPath } from 'node:url'
import path from 'node:path'
import { register, resolveNoun, listNouns } from '../registry/registry'

function toFileUrl(p: string) { return new URL('file://' + p).href }

async function loadCoreNouns() {
  const base = path.dirname(fileURLToPath(import.meta.url))
  for (const f of ['dataset.js', 'form.js']) {
    try {
      const m = await import(toFileUrl(path.join(base, 'nouns', f)))
      register(m.default || m)
    } catch {}
  }
}

function usage() {
  console.log(`Usage: gdsl <noun> <verb> [args]
Nouns: ${listNouns().sort().join(', ') || '(dataset, form)'}
Examples:
  gdsl dataset pack reality/src/packages/vitarka.pkg.ts
  gdsl dataset validate reality/src/packages/vitarka.pkg.ts --json`)
}

async function main() {
  const [, , noun = '', verb = '', ...rest] = process.argv
  await loadCoreNouns()
  if (!noun || !verb) return usage()
  const mod = resolveNoun(noun)
  if (!mod) { console.error(`unknown noun: ${noun}`); process.exit(1) }
  const fn = mod.verbs[verb]
  if (!fn) { console.error(`unknown verb for ${mod.noun}: ${verb}`); process.exit(1) }
  await Promise.resolve(fn(rest))
}
main().catch(e => { console.error(e); process.exit(1) })
