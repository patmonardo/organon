import type { LogicalOperation } from '../logic/existence/index'

// AST
export type Clause =
  | { kind: 'tag'; name: string; args: string[] }
  | { kind: 'assert'; name: string; args: string[] }
  | { kind: 'annotate'; name: string; props: Record<string, unknown> }

const trim = (s: string) => s.trim()
const unquote = (s: string) => s.replace(/^["'`]|["'`]$/g, '')
const splitArgs = (s: string) =>
  s.split(',').map(a => unquote(a.trim())).filter(Boolean)

function parseOne(clause: string): Clause | null {
  const c = clause.trim()
  if (c.startsWith('tag(') && c.endsWith(')')) {
    const inner = c.slice(4, -1)
    const args = splitArgs(inner)
    return { kind: 'tag', name: 'tag', args }
  }
  if (c.startsWith('assert(') && c.endsWith(')')) {
    const inner = c.slice(7, -1).trim()
    // name(arg1,...)
    const m = inner.match(/^([a-zA-Z_]\w*)\s*\(([\s\S]*)\)$/)
    if (!m) return null
    const name = m[1]
    const rawArgs = m[2].trim()
    const args =
      rawArgs.startsWith('[') && rawArgs.endsWith(']')
        ? JSON.parse(rawArgs).map((x: unknown) => String(x))
        : splitArgs(rawArgs)
    return { kind: 'assert', name, args }
  }
  if (c.startsWith('annotate(') && c.endsWith(')')) {
    const inner = c.slice(9, -1).trim()
    // annotate(Subject,{...})
    const i = inner.indexOf(',')
    const subject = inner.slice(0, i).trim()
    const json = inner.slice(i + 1).trim()
    try {
      const props = JSON.parse(json)
      return { kind: 'annotate', name: unquote(subject), props }
    } catch {
      return { kind: 'annotate', name: unquote(subject), props: {} }
    }
  }
  return null
}

export function parseClauses(op: LogicalOperation): Clause[] {
  const list = op.clauses ?? []
  return list.map(parseOne).filter((x): x is Clause => !!x)
}

export function parseAll(ops: LogicalOperation[]): Record<string, Clause[]> {
  const out: Record<string, Clause[]> = {}
  for (const op of ops) out[op.id] = parseClauses(op)
  return out
}
