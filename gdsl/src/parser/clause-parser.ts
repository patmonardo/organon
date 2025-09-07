export type ClauseAst =
  | { verb: 'assert'; fn: string; args: any[]; raw: string }
  | { verb: 'tag'; key: string; value: any; raw: string }
  | { verb: 'unknown'; raw: string }

const OUTER = /^(assert|tag)\((.*)\)$/s

export function parseClause(raw: string): ClauseAst {
  const m = raw.trim().match(OUTER)
  if (!m) return { verb: 'unknown', raw }
  const [, verb, innerRaw] = m
  if (verb === 'tag') {
    // tag(Key, Value)
    const [k, v] = splitTop(innerRaw, 2)
    return { verb: 'tag', key: stripOuter(k), value: parseValue(v), raw }
  }
  // assert(fn(args...))
  const fnName = innerRaw.slice(0, innerRaw.indexOf('(')).trim()
  const argsRaw = innerRaw.slice(innerRaw.indexOf('(') + 1, innerRaw.lastIndexOf(')'))
  const args = splitTop(argsRaw).map(parseValue)
  return { verb: 'assert', fn: fnName, args, raw }
}

// Split by commas at top-level (no split inside [...], (...), or "...")
function splitTop(s: string, limit = Infinity): string[] {
  const out: string[] = []
  let cur = ''
  let depthSq = 0, depthPar = 0, inStr = false, prev = ''
  for (let i = 0; i < s.length; i++) {
    const ch = s[i]
    if (inStr) {
      cur += ch
      if (ch === '"' && prev !== '\\') inStr = false
    } else {
      if (ch === '"') { inStr = true; cur += ch }
      else if (ch === '[') { depthSq++; cur += ch }
      else if (ch === ']') { depthSq--; cur += ch }
      else if (ch === '(') { depthPar++; cur += ch }
      else if (ch === ')') { depthPar--; cur += ch }
      else if (ch === ',' && depthSq === 0 && depthPar === 0) {
        out.push(cur.trim()); cur = ''
        if (out.length >= limit - 1) {
          cur += s.slice(i + 1).trim()
          break
        }
      } else cur += ch
    }
    prev = ch
  }
  if (cur.trim().length) out.push(cur.trim())
  return out
}

function stripOuter(x: string) {
  return x.trim().replace(/^["']|["']$/g, '')
}

function parseValue(x: string): any {
  const t = x.trim()
  if (t === 'true') return true
  if (t === 'false') return false
  if (/^-?\d+(\.\d+)?$/.test(t)) return Number(t)
  if (t.startsWith('"') && t.endsWith('"')) return JSON.parse(t)
  if (t.startsWith('[') && t.endsWith(']')) {
    const inner = t.slice(1, -1).trim()
    return inner ? splitTop(inner).map(parseValue) : []
  }
  // identifiers become strings
  return t
}
