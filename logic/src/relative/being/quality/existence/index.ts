// Shared types + central registry + basic validators for Essence

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

import { CHUNKS } from '../essence';
// Module imports (append Existence parts here)
import {
  CANONICAL_CHUNKS as ECH0,
  LOGICAL_OPERATIONS as EOP0,
} from './existence';
import {
  CANONICAL_CHUNKS as ECH1,
  LOGICAL_OPERATIONS as EOP1,
} from './existence1';
import {
  CANONICAL_CHUNKS as ECH2,
  LOGICAL_OPERATIONS as EOP2,
} from './existence2';
import {
  CANONICAL_CHUNKS as FCH0,
  LOGICAL_OPERATIONS as FOP0,
} from './finitude';
import {
  CANONICAL_CHUNKS as FCH1,
  LOGICAL_OPERATIONS as FOP1,
} from './finitude1';
import {
  CANONICAL_CHUNKS as FCH2,
  LOGICAL_OPERATIONS as FOP2,
} from './finitude2';
import {
  CANONICAL_CHUNKS as ICH0,
  LOGICAL_OPERATIONS as IOP0,
} from './infinity';
import {
  CANONICAL_CHUNKS as ICH1,
  LOGICAL_OPERATIONS as IOP1,
} from './infinity1';
import {
  CANONICAL_CHUNKS as ICH2,
  LOGICAL_OPERATIONS as IOP2,
} from './infinity2';

// Existence registry
export const EXISTENCE_CHUNKS: Chunk[] = [
  ...ECH0,
  ...ECH1,
  ...ECH2,
  ...FCH0,
  ...FCH1,
  ...FCH2,
  ...ICH0,
  ...ICH1,
  ...ICH2,
];
export const EXISTENCE_HLOS: LogicalOperation[] = [
  ...EOP0,
  ...EOP1,
  ...EOP2,
  ...FOP0,
  ...FOP1,
  ...FOP2,
  ...IOP0,
  ...IOP1,
  ...IOP2,
];

// Validators (default to Existence registry)
export function validateUniqueIds(
  chunks = EXISTENCE_CHUNKS,
  ops = EXISTENCE_HLOS,
) {
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

export function validateOpChunkRefs(
  chunks = EXISTENCE_CHUNKS,
  ops = EXISTENCE_HLOS,
) {
  const chunkSet = new Set(chunks.map((c) => c.id));
  const missingRefs = ops
    .filter((o) => !chunkSet.has(o.chunkId))
    .map((o) => ({ opId: o.id, chunkId: o.chunkId }));
  return { missingRefs };
}

export function integrityReport() {
  return {
    uniqueIds: validateUniqueIds(),
    opChunkRefs: validateOpChunkRefs(),
  };
}
