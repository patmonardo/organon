/**
 * ActiveShape (Container driver) — ADR 0002 alignment
 * - Deterministic helpers for signature and facet extraction
 * - Zod-backed creator for ActiveShape from raw input
 */
import { createHash } from 'crypto';
import { schemas } from '.';

export type RawShape = Record<string, any>;

export type Facet = {
  key: string;
  type?: string;
  value?: any;
};

// Stable hash: deterministic across property orderings by stringifying with sorted keys
function stableHash(value: unknown): string {
  function canonicalize(v: unknown): any {
    if (v === null || typeof v !== 'object') return v;
    if (Array.isArray(v)) return v.map(canonicalize);
    const keys = Object.keys(v as Record<string, any>).sort();
    const out: Record<string, any> = {};
    for (const k of keys) out[k] = canonicalize((v as Record<string, any>)[k]);
    return out;
  }
  const s = JSON.stringify(canonicalize(value));
  return createHash('sha256').update(s).digest('hex');
}

/**
 * Compute a deterministic signature for a Shape. Prefer shape.id if present
 * but fall back to hashing selected shape fields.
 */
export function computeSignature(shape: RawShape): string {
  if (!shape) return '';
  if (typeof shape === 'object' && shape.id) return String(shape.id);
  // choose a small, stable selection of fields
  const seed = {
    type: shape.type,
    name: shape.name,
    props: extractShapeKeys(shape).slice(0, 50),
  };
  return stableHash(seed);
}

/**
 * Extract salient keys (property names) from a shape definition. This uses
 * common conventions but is intentionally permissive to accept different
 * schema shapes present in the repository.
 */
export function extractShapeKeys(shape: RawShape): string[] {
  if (!shape || typeof shape !== 'object') return [];
  // common locations where properties might live in FormShape
  const candidates: string[] = [];
  if (Array.isArray(shape.properties)) {
    for (const p of shape.properties) {
      if (typeof p === 'string') candidates.push(p);
      else if (p && typeof p === 'object' && p.name)
        candidates.push(String(p.name));
    }
  }
  if (
    shape.properties &&
    typeof shape.properties === 'object' &&
    !Array.isArray(shape.properties)
  ) {
    candidates.push(...Object.keys(shape.properties));
  }
  if (shape.fields && typeof shape.fields === 'object') {
    candidates.push(...Object.keys(shape.fields));
  }
  // fallback: any top-level keys except common metadata
  const meta = new Set(['id', 'type', 'name', 'description', 'title']);
  for (const k of Object.keys(shape)) if (!meta.has(k)) candidates.push(k);
  // unique and stable ordering
  return Array.from(new Set(candidates)).sort();
}

/**
 * Extract lightweight facets for the engine to use during reflectStage.
 * Facets are intentionally small and deterministic.
 */
export function extractFacets(shape: RawShape): Facet[] {
  const keys = extractShapeKeys(shape);
  const facets: Facet[] = [];
  for (const k of keys) {
    const v =
      shape.properties && shape.properties[k]
        ? shape.properties[k]
        : shape.fields && shape.fields[k]
        ? shape.fields[k]
        : undefined;
    facets.push({
      key: k,
      type: typeof v,
      value: typeof v === 'string' || typeof v === 'number' ? v : undefined,
    });
  }
  // add a count facet
  facets.push({ key: '__propCount', value: keys.length, type: 'number' });
  return facets;
}

/**
 * Build a lightweight ActiveShape wrapper from a canonical FormShape object.
 */
export type ActiveShape = {
  id?: string;
  type?: string;
  name?: string;
  signature: string;
  facets: Facet[];
  raw: RawShape;
};

/**
 * Build a Zod-validated ActiveShape-like driver view from arbitrary input.
 * If `input` already follows ADR 0002 ActiveShape fields, it passes through
 * after confidence clamping; otherwise, it attempts a tolerant mapping.
 */
export function toActiveShape(input: unknown): schemas.ActiveShape {
  // Allow passing through schema-like objects while ensuring confidence clamp
  const res = schemas.ActiveShapeSchema.parse(input ?? {});
  return res;
}

/** From a canonical FormShape document (raw), assemble an engine-friendly view */
export function fromFormShape(shape: RawShape): ActiveShape {
  const signature = computeSignature(shape);
  return {
    id: (shape as any)?.id,
    type: (shape as any)?.type,
    name: (shape as any)?.name,
    signature,
    facets: extractFacets(shape),
    raw: shape,
  };
}

/**
 * Instantiate an engine-facing Thing from a Shape + optional property values.
 * The result is a small object consumed by reflect/ground stages.
 */
export default { computeSignature, extractFacets, fromFormShape, toActiveShape };
