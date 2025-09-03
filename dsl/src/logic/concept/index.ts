// Shared types + central registry + basic validators for Concept

// Provenance/evidence (IR) metadata
export interface Provenance {
  sourceChunk?: string;
  sourceOp?: string;
  extractor?: string;
  ts?: string | number;
  deps?: string[];
  evidenceIds?: string[];
}

// Canonical text chunk + concise IR summary
export interface Chunk {
  id: string;
  title: string;
  text: string;
  concise?: string;
  [k: string]: unknown;
}

// Logical operation (HLO) with IR fields
export interface LogicalOperation {
  id: string;
  chunkId: string;
  label: string;
  clauses: string[];
  predicates?: { name: string; args: unknown[] }[];
  relations?: { predicate: string; from: string; to: string }[];
  candidateSummary?: string;
  provenance?: Provenance;
  evidence?: unknown[];
  [k: string]: unknown;
}

// Module imports (Concept parts)
import {
  CANONICAL_CHUNKS as UNIVERSAL_CHUNKS,
  LOGICAL_OPERATIONS as UNIVERSAL_OPS,
} from './concept_universal';
import {
  CANONICAL_CHUNKS as PARTICULAR_CHUNKS,
  LOGICAL_OPERATIONS as PARTICULAR_OPS,
} from './concept_particular';
import {
  CANONICAL_CHUNKS as SINGULAR_CHUNKS,
  LOGICAL_OPERATIONS as SINGULAR_OPS,
} from './concept_singular';

// If you add an intro/pure-concept module later, append here:
// import { CANONICAL_CHUNKS as PURE_CONCEPT_CHUNKS, LOGICAL_OPERATIONS as PURE_CONCEPT_OPS } from './concept_pure'

// Central registry
export const CHUNKS: Chunk[] = [
  // ...PURE_CONCEPT_CHUNKS, // uncomment when available
  ...UNIVERSAL_CHUNKS,
  ...PARTICULAR_CHUNKS,
  ...SINGULAR_CHUNKS,
];

export const OPS: LogicalOperation[] = [
  // ...PURE_CONCEPT_OPS, // uncomment when available
  ...UNIVERSAL_OPS,
  ...PARTICULAR_OPS,
  ...SINGULAR_OPS,
];

// Validators
export function validateUniqueIds(chunks = CHUNKS, ops = OPS) {
  const dup = (ids: string[]) =>
    Object.entries(
      ids.reduce<Record<string, number>>(
        (m, id) => ((m[id] = (m[id] ?? 0) + 1), m),
        {},
      ),
    )
      .filter(([, n]) => n > 1)
      .map(([id, n]) => ({ id, count: n }));

  const chunkIds = chunks.map((c) => c.id);
  const opIds = ops.map((o) => o.id);
  return {
    duplicateChunkIds: dup(chunkIds),
    duplicateOpIds: dup(opIds),
  };
}

export function validateOpChunkRefs(chunks = CHUNKS, ops = OPS) {
  const chunkSet = new Set(chunks.map((c) => c.id));
  const missingRefs = ops
    .filter((o) => !chunkSet.has(o.chunkId))
    .map((o) => ({ opId: o.id, chunkId: o.chunkId }));
  return { missingRefs };
}

// Convenience: one-shot integrity report
export function integrityReport() {
  return {
    uniqueIds: validateUniqueIds(),
    opChunkRefs: validateOpChunkRefs(),
  };
}
