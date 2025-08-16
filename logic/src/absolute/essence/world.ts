import {
  createWorld,
  updateWorld,
  type World,
  type WorldEdge,
} from '../../schema/world';
import type { ProcessorInputs } from '../core/contracts';

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
