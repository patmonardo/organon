import {
  createWorld,
  updateWorld,
  type World,
  type WorldEdge,
} from '../../schema/world';
import type { ProcessorInputs } from '../core/contracts';
import crypto from 'crypto';
import { schemas } from '.';

const THING = 'system.Thing';
type ThingRef = { id: string; type: typeof THING };

// Extract entity id across mixed shapes/refs
function pickEntityId(e: any): string | undefined {
  const raw =
    e?.shape?.core?.id ??
    e?.core?.id ??
    e?.id ??
    e?.shape?.id ??
    undefined;
  return raw !== undefined && raw !== null ? String(raw) : undefined;
}

// Helpers to extract relation data resiliently across minor schema shifts
function pickKind(r: any): string {
  return (
    r?.shape?.core?.kind ??
    r?.shape?.kind ??
    r?.core?.kind ??
    r?.kind ??
    'related_to'
  );
}

function pickDirection(r: any): 'directed' | 'bidirectional' {
  const d =
    r?.shape?.direction ??
    r?.shape?.core?.direction ??
    r?.direction ??
    r?.core?.direction ??
    'directed';
  return d === 'bidirectional' ? 'bidirectional' : 'directed';
}

function pickSource(r: any): ThingRef | undefined {
  const s = r?.shape?.source ?? r?.source;
  if (!s?.id) return undefined;
  return { id: String(s.id), type: THING };
}

function pickTarget(r: any): ThingRef | undefined {
  const t = r?.shape?.target ?? r?.target;
  if (!t?.id) return undefined;
  return { id: String(t.id), type: THING };
}

function ensureThing(world: World, ref: ThingRef): World {
  const things = world.shape.things ?? [];
  if (things.some((t) => t.id === ref.id)) return world;
  return updateWorld(world, { things: [...things, ref] });
}

function edgeKey(e: WorldEdge): string {
  if (e.direction === 'bidirectional') {
    const a = [e.source.id, e.target.id].sort();
    return `undir|${e.kind}|${a[0]}|${a[1]}`;
  }
  return `dir|${e.kind}|${e.source.id}|${e.target.id}`;
}

export function assembleWorld(input: ProcessorInputs): World {
  let world = createWorld({ type: 'system.World', name: 'World' });

  // Entities → Things
  for (const e of input.entities ?? []) {
    const id = pickEntityId(e);
    if (!id) continue;
    world = ensureThing(world, { id, type: THING });
  }

  // Relations → Edges (normalized + deduped)
  const seen = new Set<string>();
  const edges: WorldEdge[] = [];

  for (const r of input.relations ?? []) {
    const source = pickSource(r);
    const target = pickTarget(r);
    if (!source || !target) continue;

    world = ensureThing(ensureThing(world, source), target);

    const edge: WorldEdge = {
      kind: pickKind(r),
      direction: pickDirection(r),
      source,
      target,
    };

    const key = edgeKey(edge);
    if (!seen.has(key)) {
      seen.add(key);
      edges.push(edge);
    }
  }

  // Stable ordering
  const sortedThings = [...(world.shape.things ?? [])].sort((a, b) =>
    a.id.localeCompare(b.id),
  );
  const sortedEdges = edges.sort((a, b) => {
    const k = a.kind.localeCompare(b.kind);
    if (k !== 0) return k;
    const s = a.source.id.localeCompare(b.source.id);
    if (s !== 0) return s;
    const t = a.target.id.localeCompare(b.target.id);
    if (t !== 0) return t;
    return a.direction.localeCompare(b.direction);
  });

  // Horizon — scope from Contexts if present, else Absolute World
  const firstCtx = (input.contexts ?? [])[0];
  const horizonScope =
    firstCtx?.shape?.core?.name ??
    firstCtx?.shape?.core?.id ??
    'Absolute World';

  world = updateWorld(world, {
    relations: sortedEdges,
    things: sortedThings,
    horizon: { scope: String(horizonScope) },
  });

  return world;
}

// Content index: subtle mapped to World (global), gross mapped to Thing (by entity id)
export function indexContent(input: ProcessorInputs): {
  subtleWorldTotal: number;
  grossByThing: Record<string, number>;
} {
  let subtleWorldTotal = 0;
  const grossByThing: Record<string, number> = {};

  for (const c of input.content) {
    if (c.shape.kind === 'subtle') {
      subtleWorldTotal += 1;
    } else {
      const id = c.shape.of.id;
      grossByThing[id] = (grossByThing[id] ?? 0) + 1;
    }
  }
  return { subtleWorldTotal, grossByThing };
}

// --- Property analytics (mates with reflect.ts property facet logic) ---

export type WorldProperty = {
  id: string;
  entity?: { id: string } | string;
  key?: string;
  value?: unknown;
} & Record<string, unknown>;

export type WorldPropertyFacet = {
  positing: Record<string, unknown>;
  external: Record<string, unknown>;
  determining: Record<string, unknown>;
  evidence: string[];
};

function stableHash(inputs: string[]): string {
  const data = inputs.sort().join('|');
  return crypto.createHash('sha1').update(data).digest('hex');
}

/**
 * Compute world-scope property facets and stable signatures.
 * Mirrors reflect.ts property facet/signature logic (no spectrum).
 */
export function computePropertyFacets(properties: WorldProperty[] = []): {
  propertyFacets: Record<string, WorldPropertyFacet>;
  signatures: Record<string, string>;
} {
  const propertyFacets: Record<string, WorldPropertyFacet> = {};
  const signatures: Record<string, string> = {};
  for (const p of properties) {
    const pid = String((p as any).id);
    const pkey = (p as any).key ?? '';
    const pEntity = (p as any).entity;
    const pEntityId = typeof pEntity === 'string' ? pEntity : pEntity && (pEntity as any).id;
    const positing = { key: pkey, entity: pEntity };
    const external = { valueType: typeof (p as any).value };
    const determining = { expressive: typeof (p as any).value !== 'undefined' };
    propertyFacets[pid] = { positing, external, determining, evidence: [] };
    const sig = stableHash([pid, String(pEntityId), String(pkey), String((p as any).value)]);
    signatures[pid] = sig;
    propertyFacets[pid].evidence.push(`sig:${sig}`);
  }
  return { propertyFacets, signatures };
}

// World -> ActiveProperty driver (first pass)
// Produces ActiveProperty instances from world-scoped property-like inputs.

export function toActiveProperty(input: unknown): schemas.ActiveProperty {
  // Validate via schema (will throw on invalid)
  return schemas.ActivePropertySchema.parse(input ?? {});
}

export function fromWorldProperty(
  p: WorldProperty,
): schemas.ActiveProperty {
  const subjectId = typeof p.entity === 'string' ? p.entity : p.entity?.id;
  const id = String(p.id ?? `${subjectId}:${p.key}`);
  // Attach provenance using computed facets/signature for traceability
  const { propertyFacets, signatures } = computePropertyFacets([p]);
  const prov = {
    world: true,
    signature: signatures[id] ?? signatures[String(p.id)] ?? null,
    facets: propertyFacets[id] ?? undefined,
  } as any;
  const candidate = {
    id,
    subjectId: String(subjectId ?? 'unknown'),
    key: String(p.key ?? 'unknown'),
    value: p.value,
    dtype:
      typeof p.value === 'string'
        ? 'string'
        : typeof p.value === 'number'
        ? 'number'
        : undefined,
    active: true,
    provenance: prov,
  } as any;
  return schemas.ActivePropertySchema.parse(candidate);
}

export function batchFromWorld(properties: WorldProperty[] = []) {
  return properties.map(fromWorldProperty);
}

export default { toActiveProperty, fromWorldProperty, batchFromWorld };
