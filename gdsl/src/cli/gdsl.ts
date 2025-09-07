#!/usr/bin/env -S node
import { registerMode } from '../registry/modes'
import { register, resolveNoun, listNouns } from '../registry/cli'
import { loadExtensions } from './load-extensions'

// Load from config (paths or package names that export activate(api))
const extSpecs = process.env.GDSL_EXTENSIONS?.split(',').filter(Boolean) ?? []

async function main() {
  await loadExtensions({ registerMode, registerNoun: register }, extSpecs)
  const [noun, verb, ...args] = process.argv.slice(2)
  const handler = resolveNoun(noun, verb)
  if (!handler) {
    console.error('unknown command', noun, verb)
    process.exit(2)
  }
  await handler(args)
}
main().catch(err => { console.error(err); process.exit(1) })
