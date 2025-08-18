import { createHash } from 'crypto';
import { schemas } from '.';

// Focused ActiveContext driver — small, deterministic helpers used by Absolute
// to normalize inputs for the ContextEngine (ADR-0003).

export type ActiveContext = schemas.ActiveContext;

function stableHashOf(obj: unknown): string {
  function canon(v: unknown): unknown {
    if (v === null || typeof v !== 'object') return v;
    if (Array.isArray(v)) return v.map(canon);
    const keys = Object.keys(v as Record<string, unknown>).sort();
    const out: Record<string, unknown> = {};
    for (const k of keys) out[k] = canon((v as Record<string, unknown>)[k]);
    return out;
  }
  return createHash('sha1').update(JSON.stringify(canon(obj))).digest('hex');
}

export function computeContextSignature(ctx: unknown): string {
  try {
    const c = schemas.ActiveContextSchema.parse(ctx ?? {});
    if (c.id) return String(c.id);
    const seed = { type: c.kind ?? c.name ?? undefined, name: c.name, scope: c.scope };
    return stableHashOf(seed).slice(0, 12);
  } catch (err) {
    // fallback: hash raw input
    return stableHashOf(ctx).slice(0, 12);
  }
}

export function extractContextFacets(ctx: unknown) {
  const c = schemas.ActiveContextSchema.parse(ctx ?? {});
  const scope = (c as any).scope ?? {};
  const worldCount = Array.isArray(scope?.world) ? scope.world.length : undefined;
  const idsCount = Array.isArray(scope?.ids) ? scope.ids.length : undefined;
  const rulesCount = c.rules && typeof c.rules === 'object' ? Object.keys(c.rules as any).length : 0;
  return {
    active: c.active ?? true,
    revoked: c.revoked ?? false,
    confidence: c.confidence ?? undefined,
    weight: c.weight ?? undefined,
    worldCount,
    idsCount,
    rulesCount,
  };
}

export function toActiveContext(input: unknown): schemas.ActiveContext {
  // delegates to zod schema for validation and normalization
  const parsed = schemas.ActiveContextSchema.parse(input ?? {});
  // Ensure confidence is clamped (schemas.normalizeActivation exists but we keep local safety)
  if (parsed && typeof (parsed as any).confidence === 'number') {
    const v = (parsed as any).confidence as number;
    (parsed as any).confidence = Math.min(1, Math.max(0, v));
  }
  return parsed;
}

export function fromFormContext(doc: unknown): schemas.ActiveContext {
  // Accepts a canonical Form Context document and derives an ActiveContext
  const id = (doc as any)?.shape?.core?.id ?? undefined;
  const name = (doc as any)?.shape?.core?.name ?? undefined;
  const type = (doc as any)?.shape?.core?.type ?? undefined;
  const scope = (doc as any)?.shape?.scope ?? undefined;
  const rules = (doc as any)?.shape?.rules ?? undefined;
  return schemas.ActiveContextSchema.parse({ id, name, kind: type, scope, rules, active: true });
}

export default { computeContextSignature, extractContextFacets, toActiveContext, fromFormContext };

// --- Compatibility helpers expected by existing tests -------------------------------------------------
// Determine a minimal context facet for a single Thing (no spectrum computation)
export function determineThingContext(thing: any, properties: any[]) {
  const tid = thing?.id;
  const tessence = thing?.essence ?? undefined;
  const propCount = Array.isArray(properties)
    ? properties.filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && ent.id); return entId === tid; }).length
    : 0;
  const positing = { id: tid, type: thing?.type ?? undefined, essence: tessence ? Object.keys(tessence).length : 0 };
  const external = { propertyCount: propCount };
  const determining = { score: (external.propertyCount || 0) + (tessence ? 1 : 0) };
  return { positing, external, determining };
}

function stableHash(inputs: string[]) {
  // simple deterministic stable hash used by signatures
  return createHash('sha1').update(inputs.sort().join('|')).digest('hex');
}

function clamp01(n: number) {
  if (Number.isNaN(n) || !isFinite(n)) return 0;
  if (n <= 0) return 0;
  if (n >= 1) return 1;
  return n;
}

function computeSpectrum(thing: any, properties: any[], facetSig: string) {
  const propKeys = (properties || []).filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && ent.id); return entId === thing.id; }).map((p) => p.key || '');
  const propCount = propKeys.length;
  const essenceKeys = thing.essence ? Object.keys(thing.essence).length : 0;
  const raw = (0.45 * propCount + 0.65 * essenceKeys) / 5;
  const intensity = clamp01(raw);
  const aspects = new Set<string>();
  for (const k of propKeys) {
    if (!k) continue;
    const mk = String(k).toLowerCase();
    if (mk.includes('goal') || mk.includes('purpose')) aspects.add('teleology');
    if (mk.includes('color') || mk.includes('hue')) aspects.add('appearance.color');
    if (mk.includes('priority') || mk.includes('importance')) aspects.add('salience');
    if (mk.includes('state') || mk.includes('status')) aspects.add('state');
    if (mk.includes('value') || mk.includes('magnitude')) aspects.add('magnitude');
  }
  if (essenceKeys > 0) aspects.add('essence.present');
  const signature = stableHash([String(thing.id), facetSig, ...Array.from(aspects).sort(), String(Math.floor(intensity * 100))]).slice(0, 12);
  return { signature, intensity, aspects: Array.from(aspects), evidence: ['reflect:sig:' + signature] };
}

// reflectStage: main reflect entry used across the codebase and tests.
export async function reflectStage(things: any[] = [], properties: any[] = [], opts: any = {}) {
  const thingFacets: Record<string, any> = {};
  const propertyFacets: Record<string, any> = {};
  const thingSigs: Record<string, string> = {};
  const propSigs: Record<string, string> = {};
  const evidence: string[] = [];

  const mode = opts?.mode ?? 'full';

  for (const t of things) {
    const tid = t.id;
    const ttype = t.type ?? undefined;
    const tessence = t.essence ?? undefined;
    const positing = { id: tid, type: ttype, essence: tessence ? Object.keys(tessence).length : 0 };
    const external = { propertyCount: (properties || []).filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && ent.id); return entId === tid; }).length };
    const determining = { score: (external.propertyCount || 0) + (tessence ? 1 : 0) };
    const facet: any = { positing, external, determining, evidence: [] };
    thingFacets[tid] = facet;

    const propKeys = (properties || []).filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && ent.id); return entId === tid; }).map((p) => p.key ?? '').sort();
    const essenceKeys = tessence ? Object.keys(tessence).sort() : [];
    const sig = stableHash([tid, String(ttype), ...propKeys, ...essenceKeys]);
    thingSigs[tid] = sig;
    evidence.push(`thing:${tid}:sig:${sig}`);
    facet.evidence.push(`sig:${sig}`);

    if (mode !== 'context') {
      try {
        facet.spectrum = computeSpectrum(t, properties, sig);
      } catch (err) {
        // swallow
      }
    }
  }

  if (mode !== 'context') {
    for (const p of properties) {
      const pid = p.id;
      const pkey = p.key ?? '';
      const pEntity = p.entity;
      const pEntityId = typeof pEntity === 'string' ? pEntity : (pEntity && pEntity.id);
      const positing = { key: pkey, entity: pEntity };
      const external = { valueType: typeof p.value };
      const determining = { expressive: typeof p.value !== 'undefined' };
      propertyFacets[pid] = { positing, external, determining, evidence: [] };
      const sig = stableHash([pid, String(pEntityId), pkey, String(p.value)]);
      propSigs[pid] = sig;
      propertyFacets[pid].evidence.push(`sig:${sig}`);
      evidence.push(`prop:${pid}:sig:${sig}`);
    }
  }

  const signatures: any = { thing: thingSigs };
  if (mode !== 'context') signatures.property = propSigs;

  return { thingFacets, propertyFacets: mode === 'context' ? undefined : propertyFacets, signatures, evidence };
}
