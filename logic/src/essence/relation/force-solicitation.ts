import crypto from "crypto";

export type ForceNode = {
  id: string;
  inwardScore?: number; // return-to-self tendency
  outwardScore?: number; // expressing/soliciting tendency
  tags?: string[];
  weight?: number; // optional force magnitude
};

export type SolicitationPair = {
  id: string; // stable id over a<->b
  a: string;
  b: string;
  symmetric: true;
  mediated: true;
  roles: { [nodeId: string]: "soliciting" | "solicited" | "both" };
  coupling: number; // 0..1 heuristic of mutual conditioning
  signature: string;
  evidence: string[];
};

function sha1(parts: string[]): string {
  return crypto.createHash("sha1").update(parts.map(String).sort().join("|")).digest("hex");
}

export function deriveSolicitationPairs(
  nodes: ForceNode[],
  opts?: { minCoupling?: number; tagAnyOf?: string[] }
): SolicitationPair[] {
  const pairs: SolicitationPair[] = [];
  for (let i = 0; i < nodes.length; i++) {
    for (let j = i + 1; j < nodes.length; j++) {
      const a = nodes[i];
      const b = nodes[j];
      const key = [a.id, b.id].sort();
      const id = `pair:${key[0]}::${key[1]}`;
      const signature = sha1([
        id,
        String(a.inwardScore ?? 0), String(a.outwardScore ?? 0), String(a.weight ?? 0),
        String(b.inwardScore ?? 0), String(b.outwardScore ?? 0), String(b.weight ?? 0),
        ...(a.tags || []), ...(b.tags || [])
      ]);
      // roles per text: soliciting vs solicited are mediated; when outward~inward both ways → "both"
      const aPush = (a.outwardScore ?? 0) - (a.inwardScore ?? 0);
      const bPush = (b.outwardScore ?? 0) - (b.inwardScore ?? 0);
      const aRole = Math.abs(aPush) < 0.1 ? "both" : aPush > 0 ? "soliciting" : "solicited";
      const bRole = Math.abs(bPush) < 0.1 ? "both" : bPush > 0 ? "soliciting" : "solicited";
      // coupling: higher when both outward≈inward and magnitudes align
      const norm = (v: number) => 1 / (1 + Math.abs(v));
      const align = 1 - Math.abs((a.outwardScore ?? 0) - (b.outwardScore ?? 0)) / (1 + Math.abs((a.outwardScore ?? 0)) + Math.abs((b.outwardScore ?? 0)));
      const inwardAlign = 1 - Math.abs((a.inwardScore ?? 0) - (b.inwardScore ?? 0)) / (1 + Math.abs((a.inwardScore ?? 0)) + Math.abs((b.inwardScore ?? 0)));
      const mag = Math.min(1, Math.abs(a.weight ?? 1) * Math.abs(b.weight ?? 1));
      const coupling = Math.max(0, (align + inwardAlign) / 2 * norm(aPush) * norm(bPush) * mag);
      // optional filters
      if (opts?.minCoupling && coupling < opts.minCoupling) continue;
      if (opts?.tagAnyOf && opts.tagAnyOf.length) {
        const t = new Set([...(a.tags || []).map(t=>t.toLowerCase()), ...(b.tags || []).map(t=>t.toLowerCase())]);
        if (!opts.tagAnyOf.some(x => t.has(x.toLowerCase()))) continue;
      }
      const evidence = [
        `pair:${id}`,
        "symmetric:true",
        "mediated:true",
        `roles:${a.id}:${aRole}`,
        `roles:${b.id}:${bRole}`,
        `coupling:${coupling.toFixed(4)}`,
        `sig:${signature}`,
      ];
      pairs.push({ id, a: key[0], b: key[1], symmetric: true, mediated: true, roles: { [a.id]: aRole, [b.id]: bRole }, coupling, signature, evidence });
    }
  }
  return pairs;
}

export type InfinityExpression = {
  pairId: string;
  identityScore: number; // 0..1 — closeness of outward to inward
  signature: string;
  evidence: string[];
};

export function expressForceInfinity(pair: SolicitationPair, a: ForceNode, b: ForceNode): InfinityExpression {
  const aIn = a.inwardScore ?? 0, aOut = a.outwardScore ?? 0;
  const bIn = b.inwardScore ?? 0, bOut = b.outwardScore ?? 0;
  // identity: outward ≈ inward across both nodes + pair coupling weight
  const diff = Math.abs(aIn - aOut) + Math.abs(bIn - bOut);
  const denom = 1 + Math.abs(aIn) + Math.abs(aOut) + Math.abs(bIn) + Math.abs(bOut);
  const base = Math.max(0, 1 - diff / denom);
  const identityScore = Math.max(0, Math.min(1, base * (0.5 + 0.5 * (pair.coupling ?? 0))));
  const signature = sha1([pair.id, pair.signature, String(identityScore)]);
  const evidence = [
    `infinity:${pair.id}`,
    `identity:outward=inward`,
    `coupling:${pair.coupling?.toFixed(4)}`,
    `sig:${signature}`,
  ];
  return { pairId: pair.id, identityScore, signature, evidence };
}
