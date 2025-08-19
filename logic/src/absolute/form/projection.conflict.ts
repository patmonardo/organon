import type { ActiveRelation } from "../../schema/active";
import { ActiveRelationSchema } from "../../schema/active";

// A tiny incompatibility table between aspect types
const INCOMPATIBLE: Record<string, Set<string>> = {
  subset_of: new Set(["disjoint_from", "not_subset_of"]),
  disjoint_from: new Set(["subset_of", "overlaps"]),
  overlaps: new Set(["disjoint_from"]),
  not_subset_of: new Set(["subset_of"]),
};

function equalRespect(a: ActiveRelation, b: ActiveRelation): boolean {
  return (
    a.source?.id === b.source?.id &&
    a.target?.id === b.target?.id
    // extend later with context/time/modality as available
  );
}

function isIncompatible(aType?: string, bType?: string): boolean {
  if (!aType || !bType) return false;
  if (aType === bType) return false;
  const set = INCOMPATIBLE[aType] || new Set<string>();
  return set.has(bType);
}

export type ConflictProjectionInputs = { relations?: ActiveRelation[] };
export type ConflictProjectionResult = {
  upserts: ActiveRelation[];
  deletes: string[];
  conflictedIds: string[]; // base relation ids that participate in conflicts
};

/**
 * computeAspectConflicts — derive contradiction markers for pairs of base aspects
 * that conflict within the same respect (same S,O) and incompatible predicates.
 *
 * Derived relation schema:
 * - id: proj:conflict:<idA+idB> (lexicographically sorted)
 * - type: "conflicts_with"
 * - source/target: same as the base respect (S,O)
 */
export function computeAspectConflicts(
  input: ConflictProjectionInputs,
): ConflictProjectionResult {
  const rels = [...(input.relations ?? [])].sort((a, b) => a.id.localeCompare(b.id));
  const upserts: ActiveRelation[] = [];
  const conflicted = new Set<string>();

  for (let i = 0; i < rels.length; i++) {
    for (let j = i + 1; j < rels.length; j++) {
      const a = rels[i];
      const b = rels[j];
      if (!equalRespect(a, b)) continue;
      if (!isIncompatible(a.type ?? a.kind, b.type ?? b.kind)) continue;
      const [id1, id2] = [a.id, b.id].sort();
      const id = `proj:conflict:${id1}+${id2}`;
      upserts.push(
        ActiveRelationSchema.parse({
          id,
          kind: "relation",
          particularityOf: a.particularityOf ?? b.particularityOf ?? "ess:proj:conflict",
          source: a.source,
          target: a.target,
          type: "conflicts_with",
          active: true,
        }),
      );
      conflicted.add(a.id);
      conflicted.add(b.id);
    }
  }

  return { upserts, deletes: [], conflictedIds: Array.from(conflicted.values()) };
}

export default { computeAspectConflicts };
