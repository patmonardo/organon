/*
  REFLECT — The Determinations of Reflection

  This module aggregates the three Determinations of Reflection:
  - A. Identity: Essence as simple self-reference, pure identity
  - B. Difference: The hard machinery of reflection (absolute difference, diversity, opposition)
  - C. Contradiction: Self-exclusion resolving to ground

  REFLECTION AS THE IDEA:CONCEPT:
  Reflection is the idea:concept itself — the shining of essence within itself.
  The Determinations of Reflection (Identity, Difference, Contradiction) are the
  systematic self-determination of essence through its own internal movement.

  TERMINOLOGY:
  The folder is named "reflect" (not "reflection") to avoid conflicts with
  reserved keywords in some languages (e.g., Rust). The parent module is "reflection"
  and its children/concepts are:
  - essence (the Essential and the Unessential)
  - reflect (the Determinations of Reflection: Identity, Difference, Contradiction)
  - ground (the resolved contradiction)
*/

// Module imports
import {
  CANONICAL_CHUNKS as IDENTITY_CHUNKS,
  LOGICAL_OPERATIONS as IDENTITY_OPS
} from './identity'

import {
  CANONICAL_CHUNKS as DIFFERENCE_CHUNKS,
  LOGICAL_OPERATIONS as DIFFERENCE_OPS
} from './difference'

import {
  CANONICAL_CHUNKS as CONTRADICTION_CHUNKS,
  LOGICAL_OPERATIONS as CONTRADICTION_OPS
} from './contradiction'

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

// Central registry: all Determinations of Reflection
export const CANONICAL_CHUNKS: Chunk[] = [
  ...IDENTITY_CHUNKS,
  ...DIFFERENCE_CHUNKS,
  ...CONTRADICTION_CHUNKS
]

export const LOGICAL_OPERATIONS: LogicalOperation[] = [
  ...IDENTITY_OPS,
  ...DIFFERENCE_OPS,
  ...CONTRADICTION_OPS
]

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
