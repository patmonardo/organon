import { CHUNKS as CONCEPT_CHUNKS, OPS as CONCEPT_OPS } from './index'
import type { Chunk, LogicalOperation } from './index'

type IdSet = Record<string, number>

function countWords(s: string): number {
  return (s?.trim().match(/\S+/g) ?? []).length
}
function uniq<T>(arr: T[]): T[] {
  return Array.from(new Set(arr))
}

export function integrityReport() {
  const chunks: Chunk[] = CONCEPT_CHUNKS
  const ops: LogicalOperation[] = CONCEPT_OPS

  // ID maps and duplicate detection
  const chunkIdCounts: IdSet = {}
  const opIdCounts: IdSet = {}
  for (const c of chunks) chunkIdCounts[c.id] = (chunkIdCounts[c.id] ?? 0) + 1
  for (const o of ops) opIdCounts[o.id] = (opIdCounts[o.id] ?? 0) + 1
  const duplicateChunkIds = Object.entries(chunkIdCounts).filter(([, n]) => n > 1).map(([id]) => id)
  const duplicateOpIds = Object.entries(opIdCounts).filter(([, n]) => n > 1).map(([id]) => id)

  // Cross-linking
  const chunkMap = new Map(chunks.map(c => [c.id, c]))
  const opsWithMissingChunk = ops.filter(o => !chunkMap.has(o.chunkId)).map(o => o.id)
  const chunksReferenced = new Set(ops.map(o => o.chunkId))
  const orphanChunks = chunks.filter(c => !chunksReferenced.has(c.id)).map(c => c.id)

  // Basic op sanity
  const opsMissingPredicates = ops.filter(o => !o.predicates || o.predicates.length === 0).map(o => o.id)
  const opsLongSummaries = ops.filter(o => (o.candidateSummary ?? '').length > 300).map(o => o.id)

  // Compression/length metrics
  const perChunk = chunks.map(c => {
    const textWords = countWords(c.text)
    const conciseWords = countWords(c.concise ?? '')
    const ratio = textWords ? +(conciseWords / textWords).toFixed(3) : 0
    return {
      id: c.id,
      title: c.title,
      textLines: (c.text?.match(/\n/g) ?? []).length + 1,
      textWords,
      conciseWords,
      compressionRatio: ratio
    }
  })
  const totals = perChunk.reduce(
    (acc, r) => {
      acc.textWords += r.textWords
      acc.conciseWords += r.conciseWords
      return acc
    },
    { textWords: 0, conciseWords: 0 }
  )
  const overallCompression = totals.textWords ? +(totals.conciseWords / totals.textWords).toFixed(3) : 0

  // Clause DSL quick lint (very loose)
  const badClauses = ops.flatMap(o =>
    (o.clauses ?? [])
      .map((cl, idx) => ({ id: o.id, idx, cl }))
      .filter(({ cl }) => !/^(assert|tag|annotate)\(/.test(cl))
  )

  // Tag/coverage snapshots
  const allTags = uniq(
    ops.flatMap(o => (o.clauses ?? [])
      .filter(cl => cl.startsWith('tag('))
      .map(cl => cl.replace(/^tag\(|\)$/g, '')))
  ).sort()

  return {
    totals: {
      chunks: chunks.length,
      operations: ops.length,
      textWords: totals.textWords,
      conciseWords: totals.conciseWords,
      overallCompression
    },
    duplicates: {
      chunkIds: duplicateChunkIds,
      opIds: duplicateOpIds
    },
    linkage: {
      opsWithMissingChunk,
      orphanChunks
    },
    opsQuality: {
      opsMissingPredicates,
      opsLongSummaries,
      badClauses
    },
    perChunk
,    tags: allTags
  }
}
