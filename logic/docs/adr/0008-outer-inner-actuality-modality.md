# ADR 0008 — Outer/Inner → Actuality (Modality) mapping

Date: 2025-08-15

Status: Proposed

Authors: automated-design-assistant + project

Context
-------

We are building a Form Processor that must mediate between two registers: the transactional/occurrence world of Conditions (Kriya) and the explanatory, stable world of Grounds (Nishkriya). The Hegelian excerpt "Relation of Outer and Inner" (see `logic/src/essence/relation/sources/outer-inner.txt`) provides the conceptual glue: the inner (essence) and outer (appearance) are formally identical but only become a determinate, content-rich unity through a mediation that is both reflective and material.

We need a lightweight, operational model for when an Essential relation becomes Actual — that is, when the form-determination (outer) coincides with its inner content and attains a modality (actual / possible / necessary) that the logic engine can use for downstream inference, indexing, and explanation.

Decision
--------

1. Introduce an explicit notion of Actuality (a modality) as a first-class metadata property attached to derived artifacts (relations, properties, grounds). Actuality is the system-level signal that the identity of Inner and Outer has been realized sufficiently to treat a relation as explanatory (not merely transactional).

2. Keep the existing Ground/Condition split. Conditions (Kriya) remain the occurrent, transaction-level artifacts produced by triggers and morphs. Grounds (Nishkriya) are stable, often Absolute containers that collect, generalize, and justify Conditions. Actuality is expressed at the boundary where a Ground and its Particular Conditions cohere.

3. Encode modality as optional provenance metadata rather than a rigid schema change. This preserves schema stability while enabling programmatic discrimination:

- provenance.modality: { kind: 'actual' | 'possible' | 'necessary' | 'dispositional', confidence?: number }
- provenance.metaphysics.role: existing 'condition' | 'ground' | 'both'
- GroundSchema.contributingConditionIds — proof-tree for the Ground

4. Add a `synthStage` in the Kriya pipeline (immediately after `groundStage`) which:
- clusters Conditions by a canonical signature (morph id, source/target types, context signature, predicate shape)
- when cluster evidence meets a policy threshold (configurable: count N, confidence, or explicit synth flag), creates or reuses a Ground (Absolute) that generalizes the cluster
- computes a modality for the resulting Ground/Essential relation using heuristics (cluster size, provenance strength, source trust, timestamp stability)
- attaches modality and `contributingConditionIds` to the Ground; sets each Condition.groundedBy → groundId
- ensures idempotence: signature-hash determinism yields stable ground ids or versioning metadata when recomputed with different inputs

Consequences
------------

- Actual relations (provenance.modality.kind === 'actual') are eligible for higher-confidence inference, indexing, and caching. They form the basis of non-transient reasoning (absolute supersensibility).

- Conditions remain the engine of change and discovery. The system will not mutate existing Conditions when creating Grounds; links are append-only to preserve historical traceability.

- Modality is an emergent, computed property, not an a priori label. It depends on cluster heuristics and provenance signals. By default items are 'possible' until synthesized into a Ground with sufficient evidence to become 'actual'.

Implementation: concrete artifacts
---------------------------------

Files and changes (proposed PR contents):
- `logic/src/schema/ground.ts` — (already added) add/ensure `contributingConditionIds: string[]` and optional `modality` shape in provenance if we formalize it.
- `logic/src/schema/condition.ts` — (already added) ensure `groundedBy?: string` is present.
- `logic/src/absolute/ground.ts` — add `synthStage` and helper utilities:
  - `clusterConditions(conditions, opts) -> clusters` (signature hashing)
  - `synthesizeGround(cluster) -> Ground + {conditionIds}`
  - `computeModality(cluster) -> { kind, confidence }`
  - `commitSynthResults(triad, synthResults)` — persist Grounds and update Conditions.groundedBy
- `logic/src/absolute/index.ts` — export `findGroundFor`, `findConditionsFor`, and `synthStage`.
- `logic/src/absolute/kriya/orchestrator.ts` — call `synthStage` after `groundStage` when `opts.synth === true`.
- Tests: `logic/test/absolute/ground.synthesis.test.ts` covering
  - single-ground-from-many-conditions (idempotence)
  - modality assignment rules (threshold/confidence)
  - versioning when a ground is recomputed with stronger evidence

Heuristics / Example modality policy
-----------------------------------
- clusterSize >= 2 → candidate 'possible'
- clusterSize >= 3 and average provenance.confidence >= 0.7 → 'actual' with confidence score
- if provenance sources include a trusted origin (configurable), boost confidence
- If a new cluster supersedes an old ground, create new ground with `previousVersionId` and leave old ground immutable

Queries / API
-------------
Expose small helpers to the FormProcessor and consumers:
- `findGroundFor(conditionId): Ground | null`
- `findConditionsFor(groundId): string[]`
- `findActualRelations(filter?): Relation[]`
- `assertActualityInvariant(relations)` — check that every relation claiming role 'ground' and `modality.kind === 'actual'` has `contributingConditionIds.length > 0` and that each contributing condition points back to the ground.

Testing & Quality Gates
-----------------------
- Add unit tests for clustering and synthesis (happy path + edge cases)
- Run integration tests (existing `@organon/logic` suite) to ensure no regressions
- Validate idempotence by re-running `synthStage` with the same inputs — no duplicate grounds
- Validate versioning by running synth with expanded input and confirm new ground created with `previousVersionId`

Risks & mitigations
-------------------
- Overfitting: heuristics may be brittle. Keep policy configurable and conservative by default (require minimal cluster size and provenance confidence).
- Semantic drift: modality labels are philosophical; prefer conservative names ('actual'|'possible'|'necessary') and treat them as operational signals, not metaphysical verdicts.
- Performance: clustering at scale requires indexing; implement signature hashing and use repo.byTag or byType for candidate retrievals before full scans.

Notes on theory → code mapping
-----------------------------
- The Hegelian "inner outer" move is mapped to a computational mediation: Conditions are the repeated externality; Grounds are the unity that reflects and contains that externality as inner content.
- Actuality is the moment where the reflected unity is sufficiently evidenced to be treated as non-transient — the FormProcessor operationalizes this as a modality attached to Grounds/Relations.

Open questions
--------------
- Should `modality` influence automatic rule firing (e.g., only allow certain inferences from 'actual' relations)? (Recommend configurable policy.)
- Should Grounds live in a separate repository or partition to simplify immutability guarantees? (Recommend experiment in `triad` commit helper.)

Decision log
------------
- This ADR proposes a design that keeps metaphysical labels in provenance + a synthStage to determine Actuality. It avoids intrusive schema changes and preserves traceability.


-- end of ADR 0008
