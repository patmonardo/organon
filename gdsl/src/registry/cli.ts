type Verb = (args: string[]) => unknown | Promise<unknown>
export type NounDef = { noun: string; aliases?: string[]; help?: string; verbs: Record<string, Verb> }

const nouns = new Map<string, NounDef>()

export function register(def: NounDef) {
  nouns.set(def.noun, def)
  for (const a of def.aliases ?? []) nouns.set(a, def)
}

export function resolveNoun(noun?: string, verb?: string): Verb | undefined {
  const n = noun ? nouns.get(noun) : undefined
  if (!n) return undefined
  const v = verb ?? 'help'
  return n.verbs[v]
}

export function listNouns(): string[] {
  return Array.from(new Set(Array.from(nouns.values()).map(n => n.noun))).sort()
}
