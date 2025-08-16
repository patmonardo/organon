import crypto from "crypto";

export type Thing = { id: string; type: string; essence?: any; properties?: any };
export type Property = { id: string; entity: { id: string; type: string }; key: string; value: any };

export type ReflectFacet = {
  positing: Record<string, unknown>;
  external: Record<string, unknown>;
  determining: Record<string, unknown>;
  evidence: string[];
};

export type ReflectResult = {
  thingFacets?: Record<string, ReflectFacet>;
  propertyFacets?: Record<string, ReflectFacet>;
  signatures?: { thing?: Record<string, string>; property?: Record<string, string> };
  evidence?: string[];
};

function stableHash(inputs: string[]): string {
  const data = inputs.sort().join("|");
  return crypto.createHash("sha1").update(data).digest("hex");
}

export async function reflectStage(
  things: Thing[] = [],
  properties: Property[] = [],
  opts?: { contextId?: string }
): Promise<ReflectResult> {
  const thingFacets: Record<string, ReflectFacet> = {};
  const propertyFacets: Record<string, ReflectFacet> = {};
  const thingSigs: Record<string, string> = {};
  const propSigs: Record<string, string> = {};
  const evidence: string[] = [];

  for (const t of things) {
    // positing: identity + essence id
    const positing = { id: t.id, type: t.type, essence: t.essence ? Object.keys(t.essence).length : 0 };
    // external: count of properties present
    const external = { propertyCount: (properties || []).filter((p) => p.entity.id === t.id).length };
    // determining: simple heuristic — if many properties and an essence, it's more determining
    const determining = { score: (external.propertyCount || 0) + (t.essence ? 1 : 0) };

    const facet: ReflectFacet = { positing, external, determining, evidence: [] };
    thingFacets[t.id] = facet;

    // signature: hash of id + type + sorted property keys + essence keys
    const propKeys = (properties || []).filter((p) => p.entity.id === t.id).map((p) => p.key).sort();
    const essenceKeys = t.essence ? Object.keys(t.essence).sort() : [];
    const sig = stableHash([t.id, t.type, ...propKeys, ...essenceKeys]);
    thingSigs[t.id] = sig;
    evidence.push(`thing:${t.id}:sig:${sig}`);
    facet.evidence.push(`sig:${sig}`);
  }

  for (const p of properties) {
    const positing = { key: p.key, entity: p.entity };
    const external = { valueType: typeof p.value };
    const determining = { expressive: typeof p.value !== 'undefined' };
    propertyFacets[p.id] = { positing, external, determining, evidence: [] };
    const sig = stableHash([p.id, p.entity.id, p.key, String(p.value)]);
    propSigs[p.id] = sig;
    propertyFacets[p.id].evidence.push(`sig:${sig}`);
    evidence.push(`prop:${p.id}:sig:${sig}`);
  }

  return { thingFacets, propertyFacets, signatures: { thing: thingSigs, property: propSigs }, evidence };
}
