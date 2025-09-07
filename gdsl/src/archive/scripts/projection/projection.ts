 /* Compact Projection tool: DatasetManifest -> GraphArtifact -> GraphStore
   Usage (quick):
     tsx src/scripts/projection/projection.ts path/to/being-dataset.ts
*/

import fs from 'fs';
import path from 'path';
import { pathToFileURL, fileURLToPath } from 'url';
import type {
  DatasetManifest,
  GraphArtifact,
  NodeRow,
  EdgeRow,
  ClauseRow,
  Token,
  Term,
  Chunk as ChunkT,
  HLO as HLOT,
} from '../../model/types';
import { GraphStore, MemoryGraphStore, NodeKey } from '../../api/graph-store';

// --- Helpers ---
function ensureDir(d: string) {
  fs.mkdirSync(d, { recursive: true });
}
function writeJson(file: string, obj: unknown) {
  fs.writeFileSync(file, JSON.stringify(obj, null, 2), 'utf8');
}
async function loadModule(fileRel: string) {
  const abs = path.resolve(process.cwd(), fileRel);
  const url = pathToFileURL(abs).href;
  return await import(url);
}

function extractSymbolNamesFromAssert(raw: string): string[] {
  const out: string[] = [];
  const re = /assert\(\s*(?:not\()?([A-Za-z_][A-Za-z0-9_]*)\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(raw)) !== null) out.push(m[1]);
  return out;
}
function clauseKind(raw: string): ClauseRow['kind'] {
  if (/^\s*assert\(/.test(raw)) return 'assert';
  if (/^\s*tag\(/.test(raw)) return 'tag';
  if (/^\s*annotate\(/.test(raw)) return 'annotate';
  return 'unknown';
}

// --- Canonicalizer: manifest -> GraphArtifact (JSON-first) ---
export async function canonicalizeManifest(
  manifestPath: string | DatasetManifest,
): Promise<GraphArtifact> {
  const manifest: DatasetManifest =
    typeof manifestPath === 'string'
      ? (await loadModule(manifestPath)).DATASET ??
        (await loadModule(manifestPath)).default
      : manifestPath;

  const chunks: ChunkT[] = [];
  const hlos: HLOT[] = [];
  const clauses: ClauseRow[] = [];
  const tokens = new Set<string>();
  const terms = new Map<string, Term>();

  // seed terms if present in manifest
  for (const t of (manifest as any).terms ?? []) terms.set(t.id, t);

  // load chunk modules referenced in manifest
  for (const cRef of manifest.chunks ?? []) {
    try {
      const mod = await loadModule(cRef.source ?? '');
      const exportedChunks = mod.CANONICAL_CHUNKS ?? mod.CHUNKS ?? [];
      const exportedHLOs = mod.LOGICAL_OPERATIONS ?? mod.HLOS ?? [];
      const pushChunks = Array.isArray(exportedChunks)
        ? exportedChunks
        : [exportedChunks];
      const pushHLOs = Array.isArray(exportedHLOs)
        ? exportedHLOs
        : [exportedHLOs];
      for (const cc of pushChunks) {
        cc.source = cRef.source;
        chunks.push(cc as ChunkT);
      }
      for (const hh of pushHLOs) hlos.push(hh as HLOT);
    } catch (err) {
      // silent fail for missing modules
      console.warn(
        '[projection] module load failed',
        cRef.source,
        (err as any)?.message ?? err,
      );
    }
  }

  // accept shallow HLO declarations in manifest
  for (const h of manifest.hlos ?? [])
    if (!hlos.find((x) => x.id === h.id)) hlos.push(h as HLOT);

  // build clauses/tokens/terms
  let cid = 0;
  for (const h of hlos) {
    for (const raw of h.clauses ?? []) {
      const kind = clauseKind(raw);
      const id = `${h.id}::clause::${++cid}`;
      clauses.push({ id, hloId: h.id, raw, kind });
      if (kind === 'assert') {
        for (const sym of extractSymbolNamesFromAssert(raw)) tokens.add(sym);
        const termRe = /\(([A-Za-z][A-Za-z0-9_]*)[,\)]/g;
        let m: RegExpExecArray | null;
        while ((m = termRe.exec(raw)) !== null) {
          const name = m[1];
          if (/^[A-Z]/.test(name)) terms.set(name, { id: name, label: name });
        }
      }
    }
  }

  // nodes/edges
  const nodeRows: NodeRow[] = [];
  const edgeRows: EdgeRow[] = [];

  nodeRows.push({
    id: manifest.id,
    labels: ['Dataset'],
    props: { title: manifest.title, provenance: manifest.provenance },
  });
  for (const c of chunks)
    nodeRows.push({ id: c.id, labels: ['Chunk'], props: c });
  for (const h of hlos)
    nodeRows.push({
      id: h.id,
      labels: ['HLO'],
      props: { ...h, clauses: undefined },
    });
  for (const cl of clauses)
    nodeRows.push({
      id: cl.id,
      labels: ['Clause'],
      props: { raw: cl.raw, kind: cl.kind, hloId: cl.hloId },
    });
  for (const tok of Array.from(tokens))
    nodeRows.push({
      id: `token:${tok}`,
      labels: ['Token'],
      props: { token: tok },
    });
  for (const t of terms.values())
    nodeRows.push({ id: `term:${t.id}`, labels: ['Term'], props: t });

  for (const c of chunks)
    edgeRows.push({ type: 'DATASET_HAS_CHUNK', from: manifest.id, to: c.id });
  for (const h of hlos)
    edgeRows.push({
      type: 'CHUNK_HAS_HLO',
      from: h.chunkId ?? 'unknown',
      to: h.id,
    });
  for (const cl of clauses)
    edgeRows.push({ type: 'HLO_HAS_CLAUSE', from: cl.hloId, to: cl.id });
  for (const cl of clauses) {
    if (cl.kind === 'assert') {
      for (const sym of extractSymbolNamesFromAssert(cl.raw))
        edgeRows.push({
          type: 'CLAUSE_ASSERTS_SYMBOL',
          from: cl.id,
          to: `token:${sym}`,
        });
    } else if (cl.kind === 'tag') {
      const m = cl.raw.match(
        /tag\(\s*['"]([^'"]+)['"]\s*,\s*['"]?([^'"\)]+)['"]?\s*\)/,
      );
      if (m) {
        const key = m[1],
          val = m[2];
        if (terms.has(val))
          edgeRows.push({
            type: 'HLO_TAGS_TERM',
            from: cl.hloId,
            to: `term:${val}`,
            props: { key },
          });
        else
          edgeRows.push({
            type: 'HLO_TAGS',
            from: cl.hloId,
            to: cl.id,
            props: { key, value: val },
          });
      } else {
        edgeRows.push({ type: 'HLO_TAGS', from: cl.hloId, to: cl.id });
      }
    }
  }

  for (const h of hlos) {
    for (const r of (h as any).witnessEdges ??
      (h as any).witnesses ??
      (h as any).relations ??
      []) {
      const type = (r.type ?? r.predicate ?? 'UNKNOWN')
        .toString()
        .toUpperCase();
      const from = r.from ?? r.subject ?? 'unknown';
      const to = r.to ?? r.object ?? r.target ?? 'unknown';
      const fromId = terms.has(from) ? `term:${from}` : from;
      const toId = terms.has(to) ? `term:${to}` : to;
      edgeRows.push({
        type: `WITNESS_${type}`,
        from: fromId,
        to: toId,
        props: r.props ?? {},
      });
    }
  }

  const artifact: GraphArtifact = {
    dataset: manifest.id,
    nodes: nodeRows,
    edges: edgeRows,
    clauses,
    tokens: Array.from(tokens),
    terms: Array.from(terms.values()),
    counts: {
      chunks: chunks.length,
      hlos: hlos.length,
      clauses: clauses.length,
      tokens: tokens.size,
    },
  };

  return artifact;
}

// --- Upsert artifact into GraphStore ---
export async function upsertArtifactToStore(
  store: GraphStore,
  artifact: GraphArtifact,
) {
  // upsert nodes
  for (const n of artifact.nodes) {
    // use first label as key label, and id as key value
    const key = { label: n.labels?.[0] ?? 'Node', key: 'id', value: n.id };
    await store.upsertNode({
      labels: n.labels ?? ['Node'],
      key,
      props: n.props,
    });
  }
  // upsert edges (best-effort: map from/to strings to NodeKey by label 'id')
  for (const e of artifact.edges) {
    const fromKey = { label: 'Node', key: 'id', value: e.from };
    const toKey = { label: 'Node', key: 'id', value: e.to };
    await store.upsertEdge({
      type: e.type,
      from: fromKey,
      to: toKey,
      props: e.props,
    });
  }
}

// --- CLI entrypoint for quick runs (ESM-safe) ---
if (fileURLToPath(import.meta.url) === process.argv[1]) {
  (async () => {
    const argv = process.argv.slice(2)
    if (!argv[0]) {
      console.error('usage: tsx projection.ts path/to/manifest.ts')
      process.exit(2)
    }
    const manifestPath = argv[0]
    const artifact = await canonicalizeManifest(manifestPath)
    const outDir = path.resolve(process.cwd(), 'dist', 'datasets')
    ensureDir(outDir)
    const outFile = path.join(outDir, `${artifact.dataset.replace(/[:\/]/g, '_')}.json`)
    writeJson(outFile, artifact)
    console.log('[projection] wrote artifact', outFile, 'counts=', artifact.counts)

    const store = new MemoryGraphStore()
    await upsertArtifactToStore(store, artifact)

    // print a fully expanded, readable dump (avoid util.inspect shorthand like [Array]/[Object])
    console.log('[projection] memory store load:')
    console.log(JSON.stringify(store.dump(), null, 2))
  })().catch((err) => {
    console.error(err)
    process.exit(1)
  })
}
