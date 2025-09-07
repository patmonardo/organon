import { CHUNKS, OPS } from './index'
import type { Chunk, LogicalOperation } from './index'

type Tag = { name: string; value?: string }

function parseTagClause(cl: string): Tag | null {
  // e.g., tag(Singularity,"negation-of-negation")
  const m = cl.match(/^tag\(\s*([^,\s)]+)\s*(?:,\s*(?:"([^"]*)"|([^)]+)))?\s*\)$/)
  if (!m) return null
  const name = m[1]
  const value = m[2] ?? (m[3]?.trim() || undefined)
  return { name, value }
}

function collectTagsByChunk(ops: LogicalOperation[]) {
  const map = new Map<string, Tag[]>()
  for (const op of ops) {
    const tags = (op.clauses || [])
      .map(parseTagClause)
      .filter(Boolean) as Tag[]
    if (!tags.length) continue
    const prev = map.get(op.chunkId) ?? []
    map.set(op.chunkId, prev.concat(tags))
  }
  // dedupe
  for (const [k, arr] of map) {
    const seen = new Set<string>()
    const dedup = []
    for (const t of arr) {
      const key = `${t.name}:${t.value ?? ''}`
      if (!seen.has(key)) {
        seen.add(key)
        dedup.push(t)
      }
    }
    map.set(k, dedup)
  }
  return map
}

function byPrefix(prefix: string) {
  return (c: Chunk) => c.id.startsWith(prefix)
}

function mdEscape(s: string) {
  return s.replace(/[<>]/g, c => (c === '<' ? '&lt;' : '&gt;'))
}

function renderSection(title: string, chunks: Chunk[], tagsByChunk: Map<string, Tag[]>) {
  const lines: string[] = []
  lines.push(`## ${title}`)
  for (const c of chunks) {
    const tags = tagsByChunk.get(c.id) ?? []
    lines.push(`### ${mdEscape(c.title)}`)
    if (c.concise) lines.push(c.concise)
    const tagList = tags.map(t => (t.value ? `${t.name}: ${t.value}` : t.name))
    const opsForChunk = OPS.filter(o => o.chunkId === c.id)
    lines.push(
      `- id: ${c.id}`,
      tagList.length ? `- tags: ${tagList.join(', ')}` : `- tags: —`,
      opsForChunk.length ? `- HLOs: ${opsForChunk.map(o => o.label).join(' | ')}` : `- HLOs: —`,
      ''
    )
  }
  return lines.join('\n')
}

function main() {
  const tagsByChunk = collectTagsByChunk(OPS)
  const universals = CHUNKS.filter(byPrefix('con-univ'))
  const particulars = CHUNKS.filter(byPrefix('con-part'))
  const singulars = CHUNKS.filter(byPrefix('con-sing'))

  const parts: string[] = []
  parts.push('# Concept Glossary (Universal • Particular • Singular)\n')
  parts.push(renderSection('Universal', universals, tagsByChunk))
  parts.push(renderSection('Particular', particulars, tagsByChunk))
  parts.push(renderSection('Singular', singulars, tagsByChunk))
  console.log(parts.join('\n'))
}

main()
