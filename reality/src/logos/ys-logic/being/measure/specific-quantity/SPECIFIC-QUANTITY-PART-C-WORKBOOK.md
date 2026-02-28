# Specific Quantity Part C Workbook

Part: `C. THE BEING-FOR-ITSELF IN MEASURE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/specific-quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `being-for-itself.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries must follow complete sentence groups (no mid-sentence start/end).

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-23 (initial scaffold)

Scope:

- file: `being-for-itself.txt`
- fixed range: lines `1-220`
- pass policy: establish first-order entries for moments `1-3`, with explicit handling of immediate vs specified measure and relation to empirical coefficient/exponent.

Decision:

- Use ID prefix `speci-spm` for Part C entries.
- Part C has no subentries.
- Section boundaries are the numerical labels `1`, `2`, `3` in `being-for-itself.txt`.
- Keep first pass source-restricted and line-anchored.
- Delay second-order integration into Becoming-Essence transitions until Part A/B/C first-order claims are stable.

## Decomposition lock

- Entry model: one entry per numbered section.
- Planned entries:
  - `speci-spm-001` -> section `1`
  - `speci-spm-002` -> section `2`
  - `speci-spm-003` -> section `3`

### Entry speci-spm-001 — Immediate qualitative subsistence against specified relation

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/being-for-itself.txt`
- lineStart: 4
- lineEnd: 51

Summary:

Specified measure presents qualitatively determined moments, yet the qualitative side initially subsists immediately and alongside a direct ratio, preserving an unresolved immediacy within measure.

Key points: (KeyPoint)

- k1. In specified measure, quantitative moments are qualitatively determined within one measure-determination.
- k2. Qualities still appear immediate and diverse, not yet exhausted by their quantitative relation.
- k3. Immediate quality remains tied to immediate quantum and allows an external arithmetical progression side.
- k4. Alongside power-determined specification, a direct and immediate ratio persists.

Claims: (Claim)

- c1. id: speci-spm-001-c1
  - subject: specified_measure
  - predicate: contains
  - object: qualitatively_determined_quantitative_moments_with_immediate_quality_residue
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-24] moments are qualitatively determined but quality also appears as immediate subsistence.

- c2. id: speci-spm-001-c2
  - subject: immediate_qualitative_moment
  - predicate: is_bound_to
  - object: immediate_quantum_and_empirical_external_alteration
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [33-43] immediate quality is bound with immediate quantum and an external arithmetical side.

- c3. id: speci-spm-001-c3
  - subject: immediate_qualities_within_measure
  - predicate: stand_in
  - object: direct_ratio_beside_power_specification
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [47-51] immediate qualities stand in a direct ratio outside the specified power-determination.

Claim ↔ key point map:

- speci-spm-001-c1 -> k1, k2
- speci-spm-001-c2 -> k3
- speci-spm-001-c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: speci-qua-c-002
  - note: details Part B's variable-measure outcome by showing persistence of immediate/direct-ratio moment.
  - sourceClaimIds: [`speci-spm-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`speci-qua-c-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: speci-spm-002
  - note: unresolved immediacy drives analysis of empirical coefficient/exponent in determinate relation.
  - sourceClaimIds: [`speci-spm-001-c2`, `speci-spm-001-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`speci-spm-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 1 stabilized as immediacy-within-specification moment.

### Entry speci-spm-002 — Empirical coefficient and conceptual exponent in the falling-body ratio

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/being-for-itself.txt`
- lineStart: 53
- lineEnd: 142

Summary:

In specific measure, the immediate quantum appears empirically as coefficient/unit, while the conceptually decisive determination remains the power-specifying relation.

Key points: (KeyPoint)

- k1. Immediate quantitative determinateness is posited as negation-return within qualitatively determined relation.
- k2. Exponent appears as immediate empirical unit in direct-ratio representation.
- k3. The formal velocity/direct-ratio moment lacks concrete existence as conceptually specific determination.
- k4. Conceptually, empirical coefficient is only a moment; measure's truth lies in power-specification of sides.

Claims: (Claim)

- c1. id: speci-spm-002-c1
  - subject: immediate_quantum_in_specific_measure
  - predicate: appears_as
  - object: empirical_unit_or_coefficient_of_direct_ratio
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [53-78] immediate quantum emerges as quotient/exponent in direct-ratio mode and empirical unit.
    - [88-92] same coefficient/unit functions in direct-ratio framing.

- c2. id: speci-spm-002-c2
  - subject: reflectively_formal_velocity_or_atomic_time_moment
  - predicate: lacks
  - object: concrete_existence_in_the_measure_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [93-104] formal velocity and atomic temporal moment are treated as merely imagined/assumed.

- c3. id: speci-spm-002-c3
  - subject: empirical_coefficient
  - predicate: is_only
  - object: immediate_moment_while_truth_is_power_specification_of_measure
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [126-142] empirical coefficient is only moment; determining truth lies in specific power relation.

Claim ↔ key point map:

- speci-spm-002-c1 -> k1, k2
- speci-spm-002-c2 -> k3
- speci-spm-002-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-spm-003
  - note: from moment-status of empirical unit to explicit unity where measure becomes real being-for-itself.
  - sourceClaimIds: [`speci-spm-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`speci-spm-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 2 stabilized around empirical-moment vs conceptual-determination distinction.

### Entry speci-spm-003 — Real being-for-itself as unity and repulsion of measure-determined somethings

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/being-for-itself.txt`
- lineStart: 144
- lineEnd: 191

Summary:

Measure is determined as specified quantitative relation whose immediate exponent and immanent determinateness form a negative unity, yielding real being-for-itself and the differentiation of self-subsisting somethings by measure.

Key points: (KeyPoint)

- k1. Specified relation makes quantum the determining exponent-moment of relation.
- k2. The unity of immediate/external and internally specified sides constitutes a further determination of measure.
- k3. This negative unity is real being-for-itself: unity of qualities in measure-relation as self-subsistence.
- k4. Real being-for-itself repels into distinct self-subsisting somethings grounded in determinate measure.

Claims: (Claim)

- c1. id: speci-spm-003-c1
  - subject: measure
  - predicate: is_determined_as
  - object: specified_quantitative_relation_with_exponent_as_determining_moment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [144-155] measure is specified relation where exponent is determining moment.

- c2. id: speci-spm-003-c2
  - subject: unity_of_immediate_and_internally_specified_measure_sides
  - predicate: constitutes
  - object: negative_unity_as_real_being_for_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [162-183] unity of two measure sides yields negative unity and real being-for-itself.

- c3. id: speci-spm-003-c3
  - subject: real_being_for_itself_of_measure
  - predicate: repels_into
  - object: distinct_self_subsisting_somethings_with_measure_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [184-191] self-subsisting whole differentiates into distinct somethings grounded in measure.

Claim ↔ key point map:

- speci-spm-003-c1 -> k1
- speci-spm-003-c2 -> k2, k3
- speci-spm-003-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: pending_next_chapter
  - note: Part C closes Specific Quantity by yielding differentiated self-subsisting somethings, preparing the next movement in Measure.
  - sourceClaimIds: [`speci-spm-003-c2`, `speci-spm-003-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 3 stabilized; Part C first-order pass complete.
