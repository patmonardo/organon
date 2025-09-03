import type { Chunk, LogicalOperation } from './index'
import { parseClause } from './clause-parser'

export type ValidationIssue =
  | { kind: 'duplicate-chunk-id'; id: string }
  | { kind: 'duplicate-hlo-id'; id: string }
  | { kind: 'unknown-chunkId'; hloId: string; chunkId: string }
  | { kind: 'unparsed-clause'; hloId: string; clause: string }

export function validate(chunks: Chunk[], hlos: LogicalOperation[]) {
  const issues: ValidationIssue[] = []
  const seenC = new Set<string>(), seenH = new Set<string>()
  for (const c of chunks) {
    if (seenC.has(c.id)) issues.push({ kind: 'duplicate-chunk-id', id: c.id })
    seenC.add(c.id)
  }
  const chunkSet = new Set(chunks.map(c => c.id))
  for (const h of hlos) {
    if (seenH.has(h.id)) issues.push({ kind: 'duplicate-hlo-id', id: h.id })
    seenH.add(h.id)
    if (!chunkSet.has(h.chunkId)) issues.push({ kind: 'unknown-chunkId', hloId: h.id, chunkId: h.chunkId })
    for (const cl of h.clauses ?? []) {
      if (parseClause(cl).verb === 'unknown') issues.push({ kind: 'unparsed-clause', hloId: h.id, clause: cl })
    }
  }
  return { ok: issues.length === 0, issues }
}
