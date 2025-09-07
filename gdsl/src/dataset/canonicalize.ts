import fs from 'fs';
import path from 'path';
import { pathToFileURL } from 'url';
import { GraphArtifactSchema, type GraphArtifact } from '../schema/projection'
import { validateFacets, type Facets } from '../schema/signature' // <- import signature helpers
import { DatasetManifestSchema, type DatasetManifest } from '../schema/dataset';
import { applyCanonRules, DEFAULT_CANON_RULES, type CanonRule } from './canon-rules'

function ensureDir(d: string) {
  fs.mkdirSync(d, { recursive: true });
}
function writeJson(file: string, obj: unknown) {
  fs.writeFileSync(file, JSON.stringify(obj, null, 2), 'utf8');
}

// Helpers kept here (no utils/ folder)
function extractSymbolNamesFromAssert(raw: string): string[] {
  const out: string[] = [];
  const re = /assert\(\s*(?:not\()?([A-Za-z_][A-Za-z0-9_]*)\s*\(/g;
  let m: RegExpExecArray | null;
  while ((m = re.exec(raw)) !== null) out.push(m[1]);
  return out;
}
function clauseKind(raw: string) {
  if (/^\s*assert\(/.test(raw)) return 'assert';
  if (/^\s*tag\(/.test(raw)) return 'tag';
  if (/^\s*annotate\(/.test(raw)) return 'annotate';
  return 'unknown';
}

async function loadManifest(spec: string | object): Promise<DatasetManifest> {
  if (typeof spec === 'object') return DatasetManifestSchema.parse(spec);
  const abs = path.resolve(process.cwd(), spec);
  const mod = await import(pathToFileURL(abs).href);
  const manifest = mod.default ?? mod.DATASET ?? mod.manifest ?? mod;
  return DatasetManifestSchema.parse(manifest);
}

export type CanonicalizeOptions = {
  outDir?: string
  write?: boolean
  rules?: CanonRule[]
}

export async function canonicalizeManifest(
  spec: string | object,
  outDirOrOptions: string | CanonicalizeOptions = 'dist/datasets',
): Promise<GraphArtifact> {
  const options: CanonicalizeOptions =
    typeof outDirOrOptions === 'string' ? { outDir: outDirOrOptions, write: true } : outDirOrOptions
  const outDir = options.outDir ?? 'dist/datasets'
  const write = options.write !== false

  const manifest = await loadManifest(spec);
  if (!manifest.id) throw new Error('canonicalizeManifest: manifest.id is required');
  ensureDir(outDir);

  // --- build artifact from manifest ---
  const nodes: Array<{
    id: string;
    labels?: string[];
    props?: Record<string, any>;
  }> = [];
  const edges: Array<{
    type: string;
    from: string;
    to: string;
    props?: Record<string, any>;
  }> = [];
  const clauses: Array<{
    id: string;
    hloId: string;
    raw: string;
    kind: string;
  }> = [];
  const tokenSet = new Set<string>();
  const terms = manifest.terms ?? [];

  // HLO nodes + clauses + tokens extracted from clauses
  const hlos = manifest.hlos ?? [];
  for (let i = 0; i < hlos.length; i++) {
    const h = hlos[i];
    const nodeId = h.id ?? `hlo:${i}`;
    nodes.push({
      id: nodeId,
      labels: ['HLO'],
      props: {
        label: h.label,
        chunkId: h.chunkId,
        digest: h.digest,
        tokens: h.tokens ?? [],
      },
    });

    // clauses
    for (
      let j = 0;
      (h.clauses ?? []).length && j < (h.clauses ?? []).length;
      j++
    ) {
      const raw = h.clauses![j];
      const kind = clauseKind(raw);
      clauses.push({ id: `${nodeId}:clause:${j}`, hloId: nodeId, raw, kind });
      if (kind === 'assert') {
        for (const sym of extractSymbolNamesFromAssert(raw)) tokenSet.add(sym);
      }
    }

    // tokens declared on HLO
    for (const t of h.tokens ?? []) tokenSet.add(t);

    // links (preferred) or legacy witnessEdges
    if (Array.isArray(h.links)) {
      for (const L of h.links) {
        const type =
          L.kind === 'witness'
            ? `WITNESS_${String(L.type).toUpperCase()}`
            : String(L.type);
        edges.push({
          type,
          from: String(L.from),
          to: String(L.to),
          props: L.props ?? {},
        });
      }
    }
    if (Array.isArray(h.witnessEdges)) {
      for (const e of h.witnessEdges) {
        const type = `WITNESS_${String(e.type).toUpperCase()}`;
        edges.push({
          type,
          from: String(e.from),
          to: String(e.to),
          props: e.props ?? {},
        });
      }
    }
  }

  // tokens: combine manifest.signatureTokens and extracted tokens
  for (const t of manifest.signatureTokens ?? []) tokenSet.add(t);
  const tokens = Array.from(tokenSet.values());

  // ensure terms array is present (may be manifest.terms)
  const artifactTerms = terms;

  // counts
  const counts = {
    hlos: hlos.length,
    clauses: clauses.length,
    asserts: clauses.filter((c) => c.kind === 'assert').length,
    tags: clauses.filter((c) => c.kind === 'tag').length,
    unknown: clauses.filter((c) => c.kind === 'unknown').length,
  };

  // simple signatures: put signatureTokens under a default signature if present
  const rawSignatures: Record<string, Array<{ token: string; weight?: number }>> = {}
  if ((manifest.signatureTokens ?? []).length > 0) {
    rawSignatures['default'] = (manifest.signatureTokens ?? []).map(
      (t: string) => ({ token: t, weight: 1 }),
    )
  }

  const validatedSignatures: Facets | undefined =
    Object.keys(rawSignatures).length > 0 ? validateFacets(rawSignatures) : undefined

  const artifact: GraphArtifact = {
    dataset: manifest.id,
    nodes: nodes as any,
    edges: edges as any,
    clauses: clauses as any,
    tokens,
    terms: artifactTerms as any,
    counts: counts as any,
    signatures: validatedSignatures,
  }

  // Apply dataset-time canonicalization rules (pure, deterministic)
  const rules = options.rules ?? DEFAULT_CANON_RULES
  const transformed = applyCanonRules(artifact as any, rules) as GraphArtifact

  const validated = GraphArtifactSchema.parse(transformed)
  const outFile = path.join(outDir, `${validated.dataset.replace(/[:\/]/g, '_')}.json`);
  ensureDir(path.dirname(outFile));
  if (write) writeJson(outFile, validated);
  return validated;
}
