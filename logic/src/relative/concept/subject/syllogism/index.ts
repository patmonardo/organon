export type Chunk = { id: string; title?: string; text: string }

export type Predicate = { name: string; args?: string[] }
export type Relation = { predicate: string; from: string; to: string }

export type LogicalOperation = {
  id: string
  label?: string
  clauses: string[]
  predicates?: Predicate[]
  relations?: Relation[]
  chunkId?: string
}

export const CANONICAL_CHUNKS: Chunk[] = [
  // ... populate with canonical chunks for syllogism when ready ...
];

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  // ... populate with HLOs for syllogism when ready ...
];

/**
 * Accessors for canonical syllogism corpus
 */

// return entire list
export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS
}

export function getChunkByIndex(index: number): Chunk | null {
  if (!Number.isInteger(index) || index < 0) return null
  return CANONICAL_CHUNKS[index] ?? null
}

// lookup by id (recommended)
export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null
}

// return all HLOs
export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}

// return HLOs for a chunk id (can be multiple)
export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId)
}
