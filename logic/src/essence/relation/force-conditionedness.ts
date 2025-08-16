import crypto from "crypto";

export type ForceCandidate = {
  id: string;
  thingId?: string;
  immediacy?: boolean; // has concrete/external immediacy
  reflection?: boolean; // appears as positedness
  externalityTags?: string[]; // e.g., ["magnetic", "electric", "ether"]
  hints?: Record<string, unknown>;
};

export type ForceConditionReport = {
  id: string;
  presupposesThing: boolean;
  appearsAsPositedness: boolean;
  immediacyTransient: boolean;
  designatedAsMatter: string[];
  negativeUnityScore: number;
  signature: string;
  evidence: string[];
};

function sha1(parts: string[]): string {
  return crypto.createHash("sha1").update(parts.map(String).sort().join("|")).digest("hex");
}

export function analyzeForceCondition(candidate: ForceCandidate): ForceConditionReport {
  const presupposesThing = Boolean(candidate.thingId);
  const appearsAsPositedness = Boolean(candidate.reflection);
  const hasImmediacy = Boolean(candidate.immediacy);
  const immediacyTransient = hasImmediacy && appearsAsPositedness;
  const tags = (candidate.externalityTags || []).map((t) => t.toLowerCase());
  const designatedAsMatter = tags.filter((t) => ["magnetic", "electric", "ether", "gravitic", "thermal"].includes(t));

  // negative unity heuristic: internal reflection + some immediacy + at least one tag
  const negativeUnityScore = (appearsAsPositedness ? 1 : 0) + (hasImmediacy ? 1 : 0) + (tags.length > 0 ? 1 : 0);

  const signature = sha1([
    candidate.id,
    candidate.thingId ?? "",
    String(hasImmediacy),
    String(appearsAsPositedness),
    ...tags,
  ]);

  const evidence: string[] = [];
  if (presupposesThing) evidence.push(`presupposes:${candidate.thingId}`);
  if (appearsAsPositedness) evidence.push("appears:positedness");
  if (hasImmediacy) evidence.push("moment:immediacy");
  if (immediacyTransient) evidence.push("immediacy:transient");
  for (const t of designatedAsMatter) evidence.push(`designated-as-matter:${t}`);
  evidence.push(`sig:${signature}`);

  return {
    id: candidate.id,
    presupposesThing,
    appearsAsPositedness,
    immediacyTransient,
    designatedAsMatter,
    negativeUnityScore,
    signature,
    evidence,
  };
}

export default analyzeForceCondition;
