import { GDSL_OPERATOR_SET } from './operators'

export interface ClauseNode {
  raw: string
  operator?: string
  args?: string[]
  valid: boolean
  error?: string
}

const OP_RE = /^([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*)\)\s*$/

export function parseClause(raw: string): ClauseNode {
  const trimmed = raw.trim().replace(/,$/,'')
  const m = trimmed.match(OP_RE)
  if (!m) return { raw, valid: false, error: 'NoOperatorPattern' }
  const op = m[1]
  if (!GDSL_OPERATOR_SET.has(op)) return { raw, operator: op, valid: false, error: 'UnknownOperator' }
  const inner = m[2].trim()
  const args = inner.length ? smartSplit(inner) : []
  return { raw, operator: op, args, valid: true }
}

// naive split respecting simple parentheses and quotes (improvable)
function smartSplit(s: string): string[] {
  const out: string[] = []
  let buf = ''
  let depth = 0
  let inQuote: string | null = null
  for (let i=0;i<s.length;i++) {
    const c = s[i]
    if (inQuote) {
      buf += c
      if (c === inQuote && s[i-1] !== '\\') inQuote = null
    } else {
      if (c === '"' || c === "'") { inQuote = c; buf += c }
      else if (c === '(') { depth++; buf += c }
      else if (c === ')') { depth--; buf += c }
      else if (c === ',' && depth === 0) { out.push(buf.trim()); buf = '' }
      else buf += c
    }
  }
  if (buf.trim()) out.push(buf.trim())
  return out
}

export function parseClauses(list: string[]): ClauseNode[] {
  return list.map(parseClause)
}
