// Being-for-Self: shared types + central registry + basic validators

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
