// Shared types + central registry + basic validators for Essence

// Provenance/evidence (IR) metadata
export interface Provenance {
  sourceChunk?: string
  sourceOp?: string
  extractor?: string
  ts?: string | number
  deps?: string[]
  evidenceIds?: string[]
}

// Canonical text chunk + concise IR summary
export interface Chunk {
  id: string
  title: string
  text: string
  concise?: string
  [k: string]: unknown
}

// Logical operation (HLO) with IR fields
export interface LogicalOperation {
  id: string
  chunkId: string
  label: string
  clauses: string[]
  predicates?: { name: string; args: unknown[] }[]
  relations?: { predicate: string; from: string; to: string }[]
  candidateSummary?: string
  provenance?: Provenance
  evidence?: unknown[]
  [k: string]: unknown
}

import { CHUNKS } from '../essence'
// Module imports (consolidated Being module)
import { CANONICAL_CHUNKS as BEING_CHUNKS, LOGICAL_OPERATIONS as BEING_HLOS } from './being'

export { BEING_CHUNKS, BEING_HLOS }

// Validators
export function validateUniqueIds(chunks = BEING_CHUNKS, ops = BEING_HLOS) {
  const dup = (ids: string[]) =>
    Object.entries(ids.reduce<Record<string, number>>((m, id) => ((m[id] = (m[id] ?? 0) + 1), m), {}))
      .filter(([, n]) => n > 1)
      .map(([id, n]) => ({ id, count: n }))

  const chunkIds = chunks.map(c => c.id)
  const opIds = ops.map(o => o.id)
  return {
    duplicateChunkIds: dup(chunkIds),
    duplicateOpIds: dup(opIds)
  }
}

export function validateOpChunkRefs(chunks = BEING_CHUNKS, ops = BEING_HLOS) {
  const chunkSet = new Set(chunks.map(c => c.id))
  const missingRefs = ops
    .filter(o => !chunkSet.has(o.chunkId))
    .map(o => ({ opId: o.id, chunkId: o.chunkId }))
  return { missingRefs }
}

// Convenience: one-shot integrity report
export function integrityReport() {
  return {
    uniqueIds: validateUniqueIds(),
    opChunkRefs: validateOpChunkRefs()
  }
}
