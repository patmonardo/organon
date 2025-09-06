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

export const CANONICAL_CHUNKS: Chunk[] = []
export const LOGICAL_OPERATIONS: LogicalOperation[] = []

/* minimal, stable accessors */
export function getChunk(oneBasedIndex: number): Chunk | null {
  return CANONICAL_CHUNKS[oneBasedIndex - 1] ?? null
}

export function getChunkById(id: string): Chunk | null {
  return CANONICAL_CHUNKS.find((c) => c.id === id) ?? null
}

export function getLogicalOpsForChunkId(chunkId: string): LogicalOperation[] {
  return LOGICAL_OPERATIONS.filter((op) => op.chunkId === chunkId)
}

export function getAllChunks(): Chunk[] {
  return CANONICAL_CHUNKS
}

export function getLogicalOperations(): LogicalOperation[] {
  return LOGICAL_OPERATIONS
}
