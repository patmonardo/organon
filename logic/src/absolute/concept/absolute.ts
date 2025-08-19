/**
 * AbsoluteConcept — a light, noumenal integrator for Qual/Quant logic.
 *
 * Pure functions only. No bus, no persistence. Intended for tests/demos and
 * higher-order control flows that sit above the Form (phenomenal) engines.
 */

import { DefaultJudgmentEngineLite } from "./judgment";
import type { Judgment } from "../../schema/judgment";
import type { ActiveProperty, ActiveRelation } from "../../schema/active";
import { intentsForGround, type IntentRegistry } from "./concept";

// Optional: future derivations (placeholders for now)
import { deriveMechanisticEdges, type MechanismEdge } from "./mechanism";
import { deriveChemisticEdges, type ChemismEdge } from "./chemism";
import { planTeleology, type TeleologyGoal } from "./teleology";
import { computeAspectConflicts } from "../form/projection.conflict";

export type QualQuantInput = {
  // per relation id qualitative/quantitative hints
  [relationId: string]: { tags?: string[]; score?: number; weight?: number };
};

export type AbsoluteConceptInputs = {
  relations?: ActiveRelation[];
  properties?: ActiveProperty[];
  qualquant?: QualQuantInput;
  intentRegistry?: IntentRegistry;
  // future: syllogisms, mechanisms, chemisms inputs
};

export type AbsoluteConceptOutputs = {
  judgments: Judgment[];
  // derived edges (optional, empty until implemented)
  mechanism: MechanismEdge[];
  chemism: ChemismEdge[];
  teleology: TeleologyGoal[];
  // controller/planning helpers from intents
  intentCandidates: Array<{ intentId: string; score: number; payload: unknown }>;
};

export function processAbsoluteConcept(input: AbsoluteConceptInputs): AbsoluteConceptOutputs {
  const relations = input.relations ?? [];
  const properties = input.properties ?? [];
  const qualquant = { ...(input.qualquant ?? {}) } as QualQuantInput;

  // Pre-pass: derive conflicts and annotate qualquant with negative tags
  const conflicts = computeAspectConflicts({ relations });
  for (const id of conflicts.conflictedIds) {
    const entry = (qualquant[id] = qualquant[id] ?? {});
    entry.tags = Array.from(new Set([...(entry.tags ?? []), "contradict"]));
    entry.weight = Math.min(entry.weight ?? 0, -1);
    entry.score = Math.min(entry.score ?? 0, 0);
  }

  // 1) First-order: Aspects → Judgments (S–P–O with polarity/modality)
  const judgments = DefaultJudgmentEngineLite.toJudgments({ relations, properties, qualquant });

  // 2) Optional derivations (mechanism/chemism/teleology) — placeholders
  const mechanism = deriveMechanisticEdges({ relations, properties, judgments } as any);
  const chemism = deriveChemisticEdges({ relations, properties, judgments } as any);
  const teleology = planTeleology({ relations, properties, judgments } as any);

  // 3) Intent candidates for planning/control
  const intentCandidates = intentsForGround(
    { relations: relations as unknown[], properties: properties as unknown[] },
    input.intentRegistry ?? { intents: [] },
  );

  return { judgments, mechanism, chemism, teleology, intentCandidates };
}

export default processAbsoluteConcept;
