export function extractSymbolNamesFromAssert(raw: string): string[] {
  const out: string[] = []
  const re = /assert\(\s*(?:not\()?([A-Za-z_][A-Za-z0-9_]*)\s*\(/g
  let m: RegExpExecArray | null
  while ((m = re.exec(raw)) !== null) out.push(m[1])
  return out
}
export function clauseKind(raw: string) {
  if (/^\s*assert\(/.test(raw)) return 'assert'
  if (/^\s*tag\(/.test(raw)) return 'tag'
  if (/^\s*annotate\(/.test(raw)) return 'annotate'
  return 'unknown'
}
