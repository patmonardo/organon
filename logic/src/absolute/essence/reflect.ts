import crypto from "crypto";

// Runtime reflect shapes (loose, engine-level)
export type ThingLike = { id: string; type?: string; essence?: Record<string, unknown> } & Record<string, unknown>;
export type PropertyLike = { id: string; entity?: { id: string } | string; key?: string; value?: unknown } & Record<string, unknown>;

export type ReflectFacet = {
  positing: Record<string, unknown>;
  external: Record<string, unknown>;
  determining: Record<string, unknown>;
  evidence: string[];
  spectrum?: Spectrum;
};

export type ReflectResult = {
  thingFacets?: Record<string, ReflectFacet>;
  propertyFacets?: Record<string, ReflectFacet>;
  signatures?: { thing?: Record<string, string>; property?: Record<string, string> };
  evidence?: string[];
};

// Engine-only runtime spectrum model (deterministic, advisory)
export type Spectrum = {
  signature: string; // stable short hash
  intensity: number; // 0..1
  aspects: string[]; // human-readable tags
  evidence?: string[];
};

function stableHash(inputs: string[]): string {
  const data = inputs.sort().join("|");
  return crypto.createHash("sha1").update(data).digest("hex");
}

function clamp01(n: number) {
  if (Number.isNaN(n) || !isFinite(n)) return 0;
  if (n <= 0) return 0;
  if (n >= 1) return 1;
  return n;
}

function computeSpectrum(thing: ThingLike, properties: PropertyLike[], facetSig: string): Spectrum {
  const propKeys = properties.filter((p) => ((p as any).entity?.id ?? (p as any).entity) === (thing as any).id).map((p) => (p as any).key || "");
  const propCount = propKeys.length;
  const essenceKeys = (thing as any).essence ? Object.keys((thing as any).essence).length : 0;

  // simple deterministic intensity heuristic
  // base = (0.45 * propCount + 0.65 * essenceKeys) normalized by a soft cap (5)
  const raw = (0.45 * propCount + 0.65 * essenceKeys) / 5;
  const intensity = clamp01(raw);

  // derive aspects from conserved key names
  const aspects = new Set<string>();
  for (const k of propKeys) {
    if (!k) continue;
    const mk = String(k).toLowerCase();
    if (mk.includes("goal") || mk.includes("purpose")) aspects.add("teleology");
    if (mk.includes("color") || mk.includes("hue")) aspects.add("appearance.color");
    if (mk.includes("priority") || mk.includes("importance")) aspects.add("salience");
    if (mk.includes("state") || mk.includes("status")) aspects.add("state");
    if (mk.includes("value") || mk.includes("magnitude")) aspects.add("magnitude");
  }
  // include a generic tag when essence is present
  if (essenceKeys > 0) aspects.add("essence.present");

  const signature = stableHash([String((thing as any).id), facetSig, ...Array.from(aspects).sort(), String(Math.floor(intensity * 100))]).slice(0, 12);

  return { signature, intensity, aspects: Array.from(aspects), evidence: ["reflect:sig:" + signature] };
}

export async function reflectStage(
  things: ThingLike[] = [],
  properties: PropertyLike[] = [],
  opts?: { contextId?: string }
): Promise<ReflectResult> {
  const thingFacets: Record<string, ReflectFacet> = {};
  const propertyFacets: Record<string, ReflectFacet> = {};
  const thingSigs: Record<string, string> = {};
  const propSigs: Record<string, string> = {};
  const evidence: string[] = [];

  for (const t of things) {
    // helpers to access runtime fields in a safer way
    const tid = (t as ThingLike).id;
    const ttype = (t as ThingLike).type ?? undefined;
    const tessence = (t as ThingLike).essence ?? undefined;

    // positing: identity + essence id
    const positing = { id: tid, type: ttype, essence: tessence ? Object.keys(tessence).length : 0 };
    // external: count of properties present
    const external = { propertyCount: (properties || []).filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && (ent as any).id); return entId === tid; }).length };
    // determining: simple heuristic — if many properties and an essence, it's more determining
    const determining = { score: (external.propertyCount || 0) + (tessence ? 1 : 0) };

    const facet: ReflectFacet = { positing, external, determining, evidence: [] };
    thingFacets[tid] = facet;

    // signature: hash of id + type + sorted property keys + essence keys
    const propKeys = (properties || []).filter((p) => { const ent = p.entity; const entId = typeof ent === 'string' ? ent : (ent && (ent as any).id); return entId === tid; }).map((p) => p.key ?? '').sort();
    const essenceKeys = tessence ? Object.keys(tessence).sort() : [];
    const sig = stableHash([tid, String(ttype), ...propKeys, ...essenceKeys]);
    thingSigs[tid] = sig;
    evidence.push(`thing:${tid}:sig:${sig}`);
    facet.evidence.push(`sig:${sig}`);
    // compute and attach runtime spectrum (engine-only)
    try {
      facet.spectrum = computeSpectrum(t, properties, sig);
    } catch (err) {
      // swallow errors; spectrum is advisory
    }
  }

  for (const p of properties) {
    const pid = (p as PropertyLike).id;
    const pkey = (p as PropertyLike).key ?? '';
    const pEntity = (p as PropertyLike).entity;
    const pEntityId = typeof pEntity === 'string' ? pEntity : (pEntity && (pEntity as any).id);
    const positing = { key: pkey, entity: pEntity };
    const external = { valueType: typeof (p as PropertyLike).value };
    const determining = { expressive: typeof (p as PropertyLike).value !== 'undefined' };
    propertyFacets[pid] = { positing, external, determining, evidence: [] };
    const sig = stableHash([pid, String(pEntityId), pkey, String((p as PropertyLike).value)]);
    propSigs[pid] = sig;
    propertyFacets[pid].evidence.push(`sig:${sig}`);
    evidence.push(`prop:${pid}:sig:${sig}`);
  }

  return { thingFacets, propertyFacets, signatures: { thing: thingSigs, property: propSigs }, evidence };
}
