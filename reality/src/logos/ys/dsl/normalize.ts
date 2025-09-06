const repl: Array<[RegExp, string]> = [
  [/[–—]/g, '-'],
  [/⇒|=>|→|⟶|->/g, '⇒'],
  [/⟷|<->|↔/g, '↔'],
  [/≠|!=/g, '≠'],
  [/==/g, '='],
  [/∥/g, '||'],
  [/\s+/g, ' '],
]

export function normalizeClause(s: string): string {
  let t = s.normalize('NFC').trim()
  for (const [rx, to] of repl) t = t.replace(rx, to)
  return t
}

export function normalizeClauses(cs: string[]): string[] {
  return cs.map(normalizeClause)
}
