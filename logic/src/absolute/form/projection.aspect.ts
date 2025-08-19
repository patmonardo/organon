import { ActiveAspectSchema, type ActiveAspect } from "../../schema/active";

export type AspectProjectionInputs = {
  apec?: ActiveAspect[];
};

export type AspectProjectionResult = {
  upserts: ActiveAspect[];
  deletes: string[];
};

/**
 * computeAspectProjection — Higher-Aspect projection over base aspects.
 * Deterministic & idempotent: given the same inputs, produces the same output set.
 *
 * Strategy (starter): for each base relation R(S, P, O), derive a higher aspect
 * with type `has_mark:${R.type}` and id `proj:${name}:${R.id}`.
 */
export function computeAspectProjection(
  name: string,
  input: AspectProjectionInputs,
): AspectProjectionResult {
  const base = [...(input.apec ?? [])]
    .filter((r) => r && r.source && r.target)
    .sort((a, b) => a.id.localeCompare(b.id));

  const upserts: ActiveAspect[] = base.map((r) =>
    ActiveAspectSchema.parse({
      id: `proj:${name}:${r.id}`,
      kind: "relation",
      particularityOf: r.particularityOf ?? `ess:proj:${name}`,
      source: r.source,
      target: r.target,
      type: `has_mark:${r.type ?? r.kind ?? "related_to"}`,
      active: r.revoked ? false : true,
    }),
  );

  return { upserts, deletes: [] };
}

export default { computeAspectProjection };
