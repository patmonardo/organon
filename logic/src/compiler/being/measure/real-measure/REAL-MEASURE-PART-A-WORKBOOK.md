# Real Measure Part A Workbook

Part: `A. THE RELATION OF INDEPENDENT MEASURES`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/real-measure/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `measures.txt` as authority.
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

- file: `measures.txt`
- pass policy: concept-part extraction for Part A using locked subsection decomposition.

Decision:

- Use ID prefix `rm-a` for Part A entries.
- Concept-part-first workflow is active.
- Part A uses subentries labeled by letters (`a`, `b`, `c`).
- Subsection `a` is sectioned by three core paragraphs followed by one transition paragraph.
- Subsection `b` is sectioned by numerical labels (`1`, `2`, `3`).
- Subsection `c` is sectioned by lettered subentries.
- Defer IDEA-special pass and compiler-normalization pass until Part A/B/C first-order claims are stabilized.

## Decomposition lock

- Entry model: subsection-specific decomposition.
- Planned entries:
  - For `a. Combination of two measures`:
    - `rm-a-a-001` -> paragraph 1 (`37-63`)
    - `rm-a-a-002` -> paragraph 2 (`64-97`)
    - `rm-a-a-003` -> paragraph 3 (`98-139`)
    - `rm-a-a-004` -> transition paragraph (`140-146`)
  - For `b. Measure as a series of measure-relations`:
    - `rm-a-b-001` -> `1` (`150-172`)
    - `rm-a-b-002` -> `2` (`174-271`)
    - `rm-a-b-003` -> `3` (`272-325`)
  - For `c. Elective affinity` (lettered decomposition):
    - `rm-a-c-a` -> first movement (`328-370`)
    - `rm-a-c-b` -> second movement (`371-405`)
    - `rm-a-c-c` -> third movement (`406-431`)

### Entry rm-a-a-001 — Two-sided material relation as direct specific gravity ratio

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 37
- lineEnd: 63

Summary:

The material something is constituted by the quantitative relation of in-itselfness and externality, now expressed as a direct-ratio exponent rather than a power-ratio form.

Key points: (KeyPoint)

- k1. A measure-relation thing is a connection of two qualitative sides.
- k2. These sides are material in-itselfness and idealized externality (space).
- k3. Their quantitative relation constitutes specific gravity.
- k4. In this immediate return of being-for-itself, determination is by direct ratio exponent.

Claims: (Claim)

- c1. id: rm-a-a-001-c1
  - subject: material_something_as_measure_relation
  - predicate: consists_of
  - object: in_itselfness_and_externality_in_quantitative_relation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [37-50] thing is connection of two sides whose quantitative relation defines qualitative nature.

- c2. id: rm-a-a-001-c2
  - subject: specific_gravity_relation
  - predicate: is_structured_as
  - object: volume_as_unit_and_weight_as_amount
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [51-55] volume is unit; intensive side appears as amount.

- c3. id: rm-a-a-001-c3
  - subject: immediate_measure_form_of_material_being
  - predicate: replaces
  - object: power_ratio_with_direct_ratio_exponent
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [56-63] pure qualitative power relation disappears; immediacy restores direct-ratio exponent.

Claim ↔ key point map:

- rm-a-a-001-c1 -> k1, k2
- rm-a-a-001-c2 -> k3
- rm-a-a-001-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-a-002
  - note: direct-ratio exponent now appears as immediate yet alterable specific quantum.
  - sourceClaimIds: [`rm-a-a-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`rm-a-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a paragraph 1 stabilized.

### Entry rm-a-a-002 — Immediate exponent as inner measure yet external alterability

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 64
- lineEnd: 97

Summary:

The exponent determines inner measure only comparatively and thus remains externally alterable, so two preserving measures reciprocally specify each other in composition.

Key points: (KeyPoint)

- k1. The specific exponent is immediate and known only through comparison with other exponents.
- k2. Inner measure resting on quantum is externally indifferent and alterable.
- k3. Composition of two different inner measures preserves each while reciprocally specifying measure.

Claims: (Claim)

- c1. id: rm-a-a-002-c1
  - subject: specific_exponent_of_something
  - predicate: is
  - object: immediate_quantum_determined_comparatively
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [64-70] exponent is immediate and determined by comparison with like ratios.

- c2. id: rm-a-a-002-c2
  - subject: inner_measure_resting_on_quantum
  - predicate: is
  - object: externally_indifferent_and_subject_to_alteration
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [71-81] quantum basis makes measure externally alterable despite intrinsic determination.

- c3. id: rm-a-a-002-c3
  - subject: composition_of_two_inner_measures
  - predicate: yields
  - object: reciprocal_specification_through_self_preserving_negation_of_quantum
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [82-97] each measure preserves itself yet specifies the other through altered ratio.

Claim ↔ key point map:

- rm-a-a-002-c1 -> k1
- rm-a-a-002-c2 -> k2
- rm-a-a-002-c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-a-003
  - note: reciprocal specification becomes explicit in unequal additive behavior of weight and volume.
  - sourceClaimIds: [`rm-a-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`rm-a-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a paragraph 2 stabilized.

### Entry rm-a-a-003 — Additive permanence and idealized-side alteration in composition

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 98
- lineEnd: 139

Summary:

In compound formation, additive permanence belongs to weight while qualitative specification shifts the exponent on the idealized side, where volume can diminish as space is revealed as idealization.

Key points: (KeyPoint)

- k1. Purely quantitative expectation predicts additive sums on both sides of the compound.
- k2. In fact, additive permanence is retained on weight/material-parts side.
- k3. Alteration appears in exponents expressing qualitative being-for-itself of the compound.
- k4. The idealized side (space/volume) is where this immanent alteration is manifested.

Claims: (Claim)

- c1. id: rm-a-a-003-c1
  - subject: compound_additivity
  - predicate: is_realized_as
  - object: weight_sum_not_volume_sum
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [98-115] expected dual additivity is denied; weight alone persists as sum.

- c2. id: rm-a-a-003-c2
  - subject: qualitative_specification_in_composition
  - predicate: appears_in
  - object: altered_exponents_and_idealized_side
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [116-126] alteration falls in exponents, not weight, and turns up on idealized side.

- c3. id: rm-a-a-003-c3
  - subject: space_in_mixed_material_relation
  - predicate: is_posited_as
  - object: idealization_and_alterable_non_self_subsistent_side
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [127-139] volume alteration/diminution exhibits space as idealization.

Claim ↔ key point map:

- rm-a-a-003-c1 -> k1, k2
- rm-a-a-003-c2 -> k3
- rm-a-a-003-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-a-004
  - note: alteration of one qualitative side prepares the explicit transition claim about measure's instability.
  - sourceClaimIds: [`rm-a-a-003-c2`, `rm-a-a-003-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`rm-a-a-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a paragraph 3 stabilized.

### Entry rm-a-a-004 — Transition: measure determinateness shifts to other measure-relations

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 140
- lineEnd: 146

Summary:

The transition paragraph concludes that not only one qualitative side but measure itself is alterable, so determinateness no longer remains stable within a single relation.

Key points: (KeyPoint)

- k1. Alterability extends from a side of the relation to measure itself.
- k2. The qualitative determinateness based on that measure is not stable within.
- k3. Measure determinateness shifts into other measure-relations.

Claims: (Claim)

- c1. id: rm-a-a-004-c1
  - subject: measure_determinateness_in_combination
  - predicate: is_not_stable_within_single_relation
  - object: but_moves_into_other_measure_relations
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [140-146] text explicitly states measure and its qualitative determinateness pass into other measure-relations.

Claim ↔ key point map:

- rm-a-a-004-c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-b-001
  - note: subsection a transition opens subsection b, where measure is developed as a series of measure-relations.
  - sourceClaimIds: [`rm-a-a-004-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`rm-a-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a completed as 3 core paragraphs plus transition paragraph.

### Entry rm-a-b-001 — Neutralizing union preserves independent measure as specifying moment

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 150
- lineEnd: 172

Summary:

When something is itself a measure-relation, combination with another preserves independence through quantitative indifference while functioning as specification of a new relation.

Key points: (KeyPoint)

- k1. Simple qualitative compounding alone would mutually sublate both terms.
- k2. As measure-relations, terms preserve independence in union via quantitative indifference.
- k3. In union each becomes specifying moment of a new measure-relation.
- k4. Specificity is not exhausted by one bilateral neutralization.

Claims: (Claim)

- c1. id: rm-a-b-001-c1
  - subject: measure_relation_something_in_union
  - predicate: preserves_itself_as
  - object: quantitatively_indifferent_specifying_moment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [154-160] each independent measure preserves itself and behaves as specifying moment in new relation.

- c2. id: rm-a-b-001-c2
  - subject: specific_property_of_measure_relation_something
  - predicate: is_not_expressed_in
  - object: one_single_neutralization_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [165-172] indifference shows same type of neutralizations with others; one relation does not exhaust specificity.

Claim ↔ key point map:

- rm-a-b-001-c1 -> k2, k3
- rm-a-b-001-c2 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-b-002
  - note: non-exhaustive bilateral relation opens the plurality and series structure of exponents.
  - sourceClaimIds: [`rm-a-b-001-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`rm-a-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b.1 stabilized.

### Entry rm-a-b-002 — Series-comparison structure and common unit of independent measures

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 174
- lineEnd: 271

Summary:

Independent measure is expressed through a series of exponents across multiple combinations, and comparison of independent measures requires a common self-subsistent unit in the ratio-of-series relation.

Key points: (KeyPoint)

- k1. Plural combinations generate diverse exponents and a characteristic series for independent measure.
- k2. Qualitative determination appears in the relation among these series.
- k3. Comparison of independent measures requires a common self-subsistent unit.
- k4. This unit is found in the constant ratio among exponents and reciprocal series structure.

Claims: (Claim)

- c1. id: rm-a-b-002-c1
  - subject: independent_measure
  - predicate: displays_itself_as
  - object: series_of_exponents_across_plural_relations
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [174-197] plurality yields diverse exponents; qualitative aspect lies in relation of series.

- c2. id: rm-a-b-002-c2
  - subject: comparison_of_independent_measures
  - predicate: requires
  - object: common_unit_existing_for_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [212-229] determinate ratio requires common unit found in ratio of series exponents.

- c3. id: rm-a-b-002-c3
  - subject: reciprocal_series_relation
  - predicate: constitutes
  - object: mutual_unit_amount_and_comparative_number_structure
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [232-271] each side is unit/exponent/comparative-number relative to the other series.

Claim ↔ key point map:

- rm-a-b-002-c1 -> k1, k2
- rm-a-b-002-c2 -> k3
- rm-a-b-002-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-b-003
  - note: reciprocal series structure returns to degree-like externality and develops exclusive qualitative unity.
  - sourceClaimIds: [`rm-a-b-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`rm-a-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b.2 stabilized.

### Entry rm-a-b-003 — Degree-like externality turns into exclusive elective unity

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 272
- lineEnd: 325

Summary:

Measure's self-reference becomes externalized as series-relating quantum, yet the reciprocal specification of independent measures produces a negation-of-negation as exclusive qualitative unit, i.e., elective affinity.

Key points: (KeyPoint)

- k1. Independent measure returns to a degree-like self-externality in series-form.
- k2. Specific determination is quantified by relation to external series while preserving self-reference.
- k3. Reciprocal negative positing yields an exclusive qualitative unity.
- k4. Indifferent plurality of affinities is sublated into elective affinity.

Claims: (Claim)

- c1. id: rm-a-b-003-c1
  - subject: independent_measure_self_reference
  - predicate: appears_as
  - object: series_externality_of_quantitative_relating
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [272-299] being-for-itself lies in totality of numerical-ratio series; quality falls into external relating.

- c2. id: rm-a-b-003-c2
  - subject: connection_of_two_specific_measures_in_exponent
  - predicate: yields
  - object: exclusive_qualitative_unity_as_negation_of_negation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [300-313] reciprocal negative positing and preserved selfhood produce exclusive unit specifying difference qualitatively.

- c3. id: rm-a-b-003-c3
  - subject: affinity_of_independent_measure_to_plurality
  - predicate: becomes
  - object: elective_affinity_not_indifferent_connection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [314-325] quantitative basis is sublated into exclusive unit; affinity becomes elective.

Claim ↔ key point map:

- rm-a-b-003-c1 -> k1, k2
- rm-a-b-003-c2 -> k3
- rm-a-b-003-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-c-a
  - note: subsection b culminates in elective affinity, opening subsection c's explicit treatment.
  - sourceClaimIds: [`rm-a-b-003-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`rm-a-c-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b completed by numeric labels `1`, `2`, `3`.

### Entry rm-a-c-a — Elective affinity generalized across chemical and tonal circles

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 328
- lineEnd: 370

Summary:

Elective affinity names a qualitative non-indifference that is structurally tied to quantitative series-relations, exemplified both chemically and musically, while its governing measure principle remains a higher-order problem.

Key points: (KeyPoint)

- k1. Elective affinity is introduced as chemical non-indifference constituted in relation to others.
- k2. This specificity is tied to quantity and to a series of opposing matters.
- k3. The same structure appears in musical systems of notes and harmonies.
- k4. The principle of measure for such elective affinities is deferred as a higher-order question.

Claims: (Claim)

- c1. id: rm-a-c-a-c1
  - subject: elective_affinity
  - predicate: denotes
  - object: qualitative_non_indifference_structured_by_series_relations
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [328-342] chemical specificity is concrete non-indifference in series-affinity context.

- c2. id: rm-a-c-a-c2
  - subject: elective_affinity_structure
  - predicate: is_exemplified_in
  - object: chemical_and_musical_combination_circles
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [343-359] note relations/harmonies parallel chemical affinity structure.

- c3. id: rm-a-c-a-c3
  - subject: measure_principle_for_elective_affinities
  - predicate: is
  - object: deferred_higher_order_problem
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [360-370] principle is postponed and tied to higher-order qualitative science.

Claim ↔ key point map:

- rm-a-c-a-c1 -> k1, k2
- rm-a-c-a-c2 -> k3
- rm-a-c-a-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-c-b
  - note: from general framing to quantitative test of exclusiveness within manifold affinity.
  - sourceClaimIds: [`rm-a-c-a-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`rm-a-c-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection c movement (a) stabilized.

### Entry rm-a-c-b — Quantitative manifold and failed derivation of exclusiveness

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 371
- lineEnd: 405

Summary:

When manifold affinity is treated through quantitative proportions and extensive/intensive conversion, exclusiveness is not produced; combinations remain formally open under proportional partition.

Key points: (KeyPoint)

- k1. In manifold affinity, the more specific determination first appears only quantitative.
- k2. Elective exclusiveness initially seems derivable from quantitative proportion/intensity transformation.
- k3. This conversion leaves the same quantitative basis intact and fails to ground exclusiveness.
- k4. Result: proportional combinations of many members remain possible.

Claims: (Claim)

- c1. id: rm-a-c-b-c1
  - subject: manifold_affinity_determinateness
  - predicate: appears_as
  - object: quantitative_difference_prior_to_exclusive_connection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [371-378] member unity over opposite series is first only quantitative.

- c2. id: rm-a-c-b-c2
  - subject: extensive_to_intensive_conversion_strategy
  - predicate: does_not_establish
  - object: true_exclusiveness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [379-397] proposed quantitative-to-intensive conversion changes no fundamental determination.

- c3. id: rm-a-c-b-c3
  - subject: quantitative_basis_without_exclusive_principle
  - predicate: permits
  - object: multiple_proportional_combinations
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [398-405] more-than-one-member combinations remain possible under proportional quantum shares.

Claim ↔ key point map:

- rm-a-c-b-c1 -> k1
- rm-a-c-b-c2 -> k2, k3
- rm-a-c-b-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-a-c-c
  - note: failure of quantitative derivation leads to explicit measure-exponent exclusiveness and leap structure.
  - sourceClaimIds: [`rm-a-c-b-c2`, `rm-a-c-b-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`rm-a-c-c-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection c movement (b) stabilized.

### Entry rm-a-c-c — Exclusive exponent and reciprocal leap between quantitative and qualitative relation

Span:

- sourceFile: `src/compiler/being/measure/real-measure/measures.txt`
- lineStart: 406
- lineEnd: 431

Summary:

Neutralization as measure-determination introduces an exclusive exponent relation that both resists and admits quantitative incursion, yielding reciprocal sudden transitions between quantitative and qualitative modes.

Key points: (KeyPoint)

- k1. Neutralization is not merely intensity but exclusive measure-determination.
- k2. Under exclusiveness, number continuity and combinability are negatively transformed.
- k3. Quantitative indifference still re-enters through divisible neutralizing shares.
- k4. The result is reciprocal sudden conversion between quantitative relation and specific qualitative measure.

Claims: (Claim)

- c1. id: rm-a-c-c-c1
  - subject: neutralization_exponent
  - predicate: is
  - object: exclusive_measure_determination
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [406-415] exponent is measure-determination; exclusive relation negates numeric continuity.

- c2. id: rm-a-c-c-c2
  - subject: exclusive_relation
  - predicate: remains_vulnerable_to
  - object: quantitative_indifference_in_multi_source_neutralization
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [417-424] exclusiveness is exposed to quantitative incursion via divisible opposite moments.

- c3. id: rm-a-c-c-c3
  - subject: relation_dynamics_in_elective_affinity
  - predicate: consists_in
  - object: sudden_two_way_conversion_between_quantitative_and_qualitative_measure_relations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [425-431] explicit sudden conversion both ways between indifferent quantity and specific relation.

Claim ↔ key point map:

- rm-a-c-c-c1 -> k1, k2
- rm-a-c-c-c2 -> k3
- rm-a-c-c-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rm-b-001
  - note: subsection c closes Part A by opening Part B (Nodal lines) where leap-structure and alternation are developed explicitly.
  - sourceClaimIds: [`rm-a-c-c-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection c completed by lettered movements (a, b, c); Part A first-order pass complete.
