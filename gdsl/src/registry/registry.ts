export type VerbFn = (args: string[]) => Promise<void> | void

export interface NounModule {
  noun: string
  aliases?: string[]
  verbs: Record<string, VerbFn>
  help?: string
}

const nouns = new Map<string, NounModule>()
const alias = new Map<string, string>()

export function register(mod: NounModule) {
  nouns.set(mod.noun, mod)
  for (const a of mod.aliases || []) alias.set(a, mod.noun)
}

export function resolveNoun(n: string) {
  return nouns.get(n) || nouns.get(alias.get(n) || '')
}

export function listNouns() {
  return [...nouns.keys()]
}
