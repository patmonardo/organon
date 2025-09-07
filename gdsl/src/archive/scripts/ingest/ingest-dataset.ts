/* Minimal ingest scaffold: manifest -> canonical objects -> MemoryGraphStore -> JSON artifact
   Run with: tsx src/scripts/ingest/ingest-dataset.ts
*/
import fs from 'fs'
import path from 'path'
import { pathToFileURL } from 'url'
import { DATASET } from '../../data/datasets/being-dataset'

// --- Lightweight types ---
type Chunk = { id: string; title?: string; text?: string; summary?: string; source?: string }
type HLO = { id: string; chunkId: string; label?: string; digest?: string; clauses?: string[]; essentialRelations?: any[] }
type ClauseRow = { id: string; hloId: string; raw: string; kind: 'assert' | 'tag' | 'annotate' | 'unknown' }
type NodeRow = { id: string; label: string; props?: Record<string, unknown> }
type EdgeRow = { id?: string; type: string; from: string; to: string; props?: Record<string, unknown> }

// --- Simple MemoryGraphStore for quick QA ---
class MemoryGraphStore {
  nodes = new Map<string, NodeRow>()
  edges: EdgeRow[] = []
  upsertNode(n: NodeRow) { this.nodes.set(n.id, n); return n.id }
  upsertEdge(e: EdgeRow) { this.edges.push(e) }
}

// --- helpers ---
function ensureDir(d: string) { fs.mkdirSync(d, { recursive: true }) }
function writeJson(file: string, obj: unknown) { fs.writeFileSync(file, JSON.stringify(obj, null, 2), 'utf8') }

function extractSymbolNamesFromAssert(raw: string): string[] {
  // matches assert(functor(...) or assert(not(functor(...)))
  const out: string[] = []
  const re = /assert\(\s*(?:not\()?([A-Za-z_][A-Za-z0-9_]*)\s*\(/g
  let m: RegExpExecArray | null
  while ((m = re.exec(raw)) !== null) out.push(m[1])
  return out
}

function clauseKind(raw: string): ClauseRow['kind'] {
  if (/^\s*assert\(/.test(raw)) return 'assert'
  if (/^\s*tag\(/.test(raw)) return 'tag'
  if (/^\s*annotate\(/.test(raw)) return 'annotate'
  return 'unknown'
}

// --- ingest flow ---
async function loadModule(fileRel: string) {
  const abs = path.resolve(process.cwd(), fileRel)
  const url = pathToFileURL(abs).href
  return await import(url)
}

async function main() {
  const outDir = path.resolve(process.cwd(), 'dist', 'datasets')
  ensureDir(outDir)

  // canonical containers
  const chunks: Chunk[] = []
  const hlos: HLO[] = []
  const clauses: ClauseRow[] = []
  const tokens = new Set<string>()
  const terms = new Map<string, { id: string; label?: string; aliases?: string[] }>()

  // seed terms from manifest if present
  for (const t of (DATASET as any).terms ?? []) terms.set(t.id, t)

  // load referenced chunk modules (manifest points to source files)
  for (const cRef of (DATASET as any).chunks ?? []) {
    // try to import the module and collect exported chunk/hlo arrays
    try {
      const mod = await loadModule(cRef.source)
      const exportedChunks = mod.CANONICAL_CHUNKS ?? mod.CHUNKS ?? mod.CHUNK ?? mod.default ?? []
      const exportedHLOs = mod.LOGICAL_OPERATIONS ?? mod.HLOS ?? mod.DEFAULT_HLOS ?? mod.default ?? []
      // normalize arrays (some modules export single objects)
      const pushChunks = Array.isArray(exportedChunks) ? exportedChunks : [exportedChunks]
      const pushHLOs = Array.isArray(exportedHLOs) ? exportedHLOs : [exportedHLOs]
      for (const cc of pushChunks) {
        cc.source = cRef.source
        chunks.push(cc)
      }
      for (const hh of pushHLOs) {
        hlos.push(hh)
      }
    } catch (err) {
      console.error('[ingest] failed to import', cRef.source, err)
    }
  }

  // also allow manifest-specified HLOs (shallow entries) in DATASET.hlos
  for (const h of (DATASET as any).hlos ?? []) {
    // if already present by id skip
    if (!hlos.find(x => x.id === h.id)) hlos.push(h as HLO)
  }

  // build clauses + tokens + canonical terms from HLO clauses
  let clauseSeq = 0
  for (const h of hlos) {
    const rawClauses = h.clauses ?? []
    for (const raw of rawClauses) {
      const kind = clauseKind(raw)
      const cid = `${h.id}::clause::${++clauseSeq}`
      clauses.push({ id: cid, hloId: h.id, raw, kind })
      if (kind === 'assert') {
        for (const sym of extractSymbolNamesFromAssert(raw)) {
          tokens.add(sym)
        }
        // also pick up constants/terms by heuristic: words with initial capital inside parens
        const termRe = /\(([A-Za-z][A-Za-z0-9_]*)[,\)]/g
        let m: RegExpExecArray | null
        while ((m = termRe.exec(raw)) !== null) {
          const name = m[1]
          if (/^[A-Z]/.test(name)) terms.set(name, { id: name, label: name })
        }
      }
    }
  }

  // nodes/edges collections
  const nodeRows: NodeRow[] = []
  const edgeRows: EdgeRow[] = []

  // populate nodes: dataset meta, chunks, hlos, clauses, tokens, terms
  nodeRows.push({ id: (DATASET as any).id, label: 'Dataset', props: { title: (DATASET as any).title, provenance: (DATASET as any).provenance } })
  for (const c of chunks) nodeRows.push({ id: c.id, label: 'Chunk', props: c })
  for (const h of hlos) nodeRows.push({ id: h.id, label: 'HLO', props: { ...h, clauses: undefined } })
  for (const cl of clauses) nodeRows.push({ id: cl.id, label: 'Clause', props: { raw: cl.raw, kind: cl.kind, hloId: cl.hloId } })
  for (const tok of Array.from(tokens)) nodeRows.push({ id: `token:${tok}`, label: 'Token', props: { token: tok } })
  for (const t of terms.values()) nodeRows.push({ id: `term:${t.id}`, label: 'Term', props: t })

  // edges: dataset contains chunks; chunk->hlo; hlo->clause; clause->token (asserts); hlo->term via tags/annotations; essentialRelations -> edges
  for (const c of chunks) edgeRows.push({ type: 'DATASET_HAS_CHUNK', from: (DATASET as any).id, to: c.id })
  for (const h of hlos) edgeRows.push({ type: 'CHUNK_HAS_HLO', from: h.chunkId ?? 'unknown', to: h.id })
  for (const cl of clauses) edgeRows.push({ type: 'HLO_HAS_CLAUSE', from: cl.hloId, to: cl.id })
  for (const cl of clauses) {
    if (cl.kind === 'assert') {
      for (const sym of extractSymbolNamesFromAssert(cl.raw)) {
        edgeRows.push({ type: 'CLAUSE_ASSERTS_SYMBOL', from: cl.id, to: `token:${sym}` })
      }
    } else if (cl.kind === 'tag') {
      // best-effort: extract tag key/value for edges to terms if values are term ids
      const m = cl.raw.match(/tag\(\s*['"]([^'"]+)['"]\s*,\s*['"]?([^'"\)]+)['"]?\s*\)/)
      if (m) {
        const key = m[1], val = m[2]
        if (terms.has(val)) edgeRows.push({ type: 'HLO_TAGS_TERM', from: cl.hloId, to: `term:${val}`, props: { key } })
        else edgeRows.push({ type: 'HLO_TAGS', from: cl.hloId, to: cl.id, props: { key, value: val } })
      } else {
        edgeRows.push({ type: 'HLO_TAGS', from: cl.hloId, to: cl.id })
      }
    }
  }

  // essentialRelations from HLOs (if present)
  for (const h of hlos) {
    for (const r of (h as any).essentialRelations ?? (h as any).relations ?? []) {
      // map legacy shape { predicate, from, to } -> { type, from, to }
      const type = (r.type ?? r.predicate ?? 'UNKNOWN').toString().toUpperCase()
      const from = r.from ?? r.subject ?? 'unknown'
      const to = r.to ?? r.object ?? r.target ?? 'unknown'
      // normalize to term node ids if terms map contains them
      const fromId = terms.has(from) ? `term:${from}` : from
      const toId = terms.has(to) ? `term:${to}` : to
      edgeRows.push({ type: `ESSENTIAL_${type}`, from: fromId, to: toId, props: r.props ?? {} })
    }
  }

  // write artifact (JSON first)
  const artifact = { dataset: DATASET.id, nodes: nodeRows, edges: edgeRows, tokens: Array.from(tokens), terms: Array.from(terms.values()), counts: { chunks: chunks.length, hlos: hlos.length, clauses: clauses.length, tokens: tokens.size } }
  const outFile = path.join(outDir, `${(DATASET as any).id.replace(/[:\/]/g, '_')}.json`)
  writeJson(outFile, artifact)
  console.log('[ingest] wrote', outFile)

  // quick MemoryGraphStore load for immediate QA
  const store = new MemoryGraphStore()
  for (const n of nodeRows) store.upsertNode(n)
  for (const e of edgeRows) store.upsertEdge(e)
  console.log('[ingest] memory store: nodes=%d edges=%d', store.nodes.size, store.edges.length)
  console.log('[ingest] summary:', artifact.counts)
}

main().catch(err => { console.error(err); process.exit(1) })
