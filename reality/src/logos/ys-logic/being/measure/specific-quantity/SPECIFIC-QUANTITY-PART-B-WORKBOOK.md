# Specific Quantity Part B Workbook

Part: `B. SPECIFYING MEASURE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/specific-quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `specifying-measure.txt` as authority.
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

- file: `specifying-measure.txt`
- fixed range: lines `1-246`
- pass policy: establish first-order entries for `a. The rule`, `b. Specifying measure`, and `c. Relation of the two sides as qualities`.

Decision:

- Use ID prefix `speci-qua` for Part B entries.
- Part B uses subentries by internal sectioning rules:
  - `a. The rule` is analyzed in three entries.
  - `b. Specifying measure` is sectioned by its three paragraphs.
  - `c. Relation of the two sides as qualities` is sectioned by numerical labels.
- Keep first pass source-restricted and line-anchored.
- Preserve explicit tie from measure as ratio/exponent to realized measure without adding second-order synthesis claims yet.

## Decomposition lock

- Entry model: section-specific decomposition.
- Planned entries:
  - For `a. The rule`:
    - `speci-qua-a-001`
    - `speci-qua-a-002`
    - `speci-qua-a-003`
  - For `b. Specifying measure` (three paragraphs):
    - `speci-qua-b-001`
    - `speci-qua-b-002`
    - `speci-qua-b-003`
  - For `c. Relation of the two sides as qualities` (numerical labels):
    - `speci-qua-c-001` -> `1`
    - `speci-qua-c-002` -> `2`

### Entry speci-qua-a-001 — Rule as determinate unit for external comparison

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 16
- lineEnd: 24

Summary:

The rule first appears as a magnitude determinate in itself that functions as a unit for measuring another concrete quantum.

Key points: (KeyPoint)

- k1. The rule is first presented as a magnitude determinate in itself.
- k2. This determinate magnitude functions as a unit with respect to another concrete existence.
- k3. Measurement determines the other concrete quantum as an amount of that unit.

Claims: (Claim)

- c1. id: speci-qua-a-001-c1
  - subject: rule
  - predicate: is_first_determined_as
  - object: magnitude_determinate_in_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [16-19] the rule is first a magnitude determinate in itself.

- c2. id: speci-qua-a-001-c2
  - subject: determinate_rule_magnitude
  - predicate: functions_as
  - object: unit_for_measuring_other_concrete_quantum
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [19-24] the other concrete existence is measured and determined as amount of the unit.

Claim ↔ key point map:

- speci-qua-a-001-c1 -> k1
- speci-qua-a-001-c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-a-002
  - note: from unit-function to the explicit externality and arbitrariness of comparison.
  - sourceClaimIds: [`speci-qua-a-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`speci-qua-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: speci-bfr-003
  - note: operationalizes Part A's transition from immediate measure to specifying measure.
  - sourceClaimIds: [`speci-qua-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`speci-bfr-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a/entry 1 stabilized as first determination of rule.

### Entry speci-qua-a-002 — External act and arbitrariness of the measuring unit

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 25
- lineEnd: 28

Summary:

The comparison is an external act, and the unit can be arbitrarily re-posited as a further amount.

Key points: (KeyPoint)

- k1. Comparison in this register is explicitly external.
- k2. The measuring unit is arbitrary as magnitude.
- k3. The unit can itself be re-expressed as an amount (for example, foot in inches).

Claims: (Claim)

- c1. id: speci-qua-a-002-c1
  - subject: measurement_comparison
  - predicate: is
  - object: external_act
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [25] the comparison is stated to be external.

- c2. id: speci-qua-a-002-c2
  - subject: measuring_unit
  - predicate: is
  - object: arbitrary_magnitude_reposable_as_amount
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [26-28] the unit is arbitrary and can be posited as an amount (foot/inches).

Claim ↔ key point map:

- speci-qua-a-002-c1 -> k1
- speci-qua-a-002-c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-a-003
  - note: external arbitrariness is immediately restricted by measure's intrinsic relation to its other quantum.
  - sourceClaimIds: [`speci-qua-a-002-c1`, `speci-qua-a-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`speci-qua-a-003-c1`]
  - logicalOperator: contrastive_negation
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a/entry 2 stabilized as external-comparison moment.

### Entry speci-qua-a-003 — Rule sublated into specific measure

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 29
- lineEnd: 31

Summary:

Measure is not merely external rule; as specific measure it intrinsically relates to an other that is quantum.

Key points: (KeyPoint)

- k1. Measure exceeds the status of merely external rule.
- k2. Specific measure is intrinsically relational.
- k3. The intrinsic relation is to an other that is quantum.

Claims: (Claim)

- c1. id: speci-qua-a-003-c1
  - subject: specific_measure
  - predicate: is_not
  - object: merely_external_rule
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [29] measure is not only an external rule.

- c2. id: speci-qua-a-003-c2
  - subject: specific_measure
  - predicate: intrinsically_relates_to
  - object: other_as_quantum
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [30-31] intrinsic nature is relation to its other as quantum.

Claim ↔ key point map:

- speci-qua-a-003-c1 -> k1
- speci-qua-a-003-c2 -> k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-b-001
  - note: subsection a concludes by opening subsection b's explicit account of specifying the external quantum.
  - sourceClaimIds: [`speci-qua-a-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`speci-qua-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection a completed in three entries per decomposition lock.

### Entry speci-qua-b-001 — Specific measure as immanent assimilation of external alteration

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 35
- lineEnd: 58

Summary:

Specific measure is the qualitative side that actively assimilates externally imposed quantitative alteration, demonstrating for-itself determinacy within externality.

Key points: (KeyPoint)

- k1. Measure specifies an otherwise indifferent external magnitude.
- k2. Internal measure reacts against externally aggregated alteration rather than merely receiving it.
- k3. The reaction is qualitative assimilation that transforms external quantum.
- k4. This specifying reaction demonstrates for-itselfness in externality.

Claims: (Claim)

- c1. id: speci-qua-b-001-c1
  - subject: specific_measure
  - predicate: determines
  - object: indifferent_external_quantum_qualitatively
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [35-43] the measure-side is qualitative and determines the indifferent external quantum.

- c2. id: speci-qua-b-001-c2
  - subject: internal_measure
  - predicate: reacts_to
  - object: externally_imposed_aggregate_as_intensive_measure
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [48-54] measure reacts against aggregate alteration and behaves as intensive measure.

- c3. id: speci-qua-b-001-c3
  - subject: specifying_function
  - predicate: demonstrates
  - object: for_itselfness_through_assimilative_transformation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [55-58] externally imposed alteration is transformed; this shows for-itself in externality.

Claim ↔ key point map:

- speci-qua-b-001-c1 -> k1
- speci-qua-b-001-c2 -> k2
- speci-qua-b-001-c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-b-002
  - note: assimilated aggregate is now considered as stable specified quantum with ratio/exponent character.
  - sourceClaimIds: [`speci-qua-b-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`speci-qua-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: speci-qua-a-003
  - note: elaborates subsection a's claim that specific measure intrinsically relates to quantum as its other.
  - sourceClaimIds: [`speci-qua-b-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`speci-qua-a-003-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b/entry 1 stabilized as assimilation moment.

### Entry speci-qua-b-002 — Specified aggregate as ratio and exponent

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 59
- lineEnd: 80

Summary:

The assimilated aggregate remains alterable yet specified in a constant manner, yielding measure's determinate existence as ratio whose specificity is exponent.

Key points: (KeyPoint)

- k1. The specifically assimilated aggregate remains quantum-dependent on an external other.
- k2. This aggregate is not mere quantum but external quantum specified in a constant manner.
- k3. Measure's determinate existence is ratio and specificity appears as exponent.
- k4. Unlike extensive/intensive form-switching, specifying measure alters amount through the exponentiated relation.

Claims: (Claim)

- c1. id: speci-qua-b-002-c1
  - subject: specified_aggregate
  - predicate: is
  - object: alterable_external_quantum_specified_constantly
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [59-64] aggregate is alterable yet not mere quantum; it is specified external quantum.

- c2. id: speci-qua-b-002-c2
  - subject: measure
  - predicate: has_determinate_existence_as
  - object: ratio_with_specificity_as_exponent
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [65-66] measure exists as ratio; specificity is exponent.

- c3. id: speci-qua-b-002-c3
  - subject: specifying_measure
  - predicate: differs_from
  - object: mere_intensive_extensive_form_difference_by_real_amount_shift
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [68-80] unlike simple form-difference, exponent relation yields another amount.

Claim ↔ key point map:

- speci-qua-b-002-c1 -> k1, k2
- speci-qua-b-002-c2 -> k3
- speci-qua-b-002-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-b-003
  - note: ratio/exponent is clarified against misreading exponent as fixed external quotient.
  - sourceClaimIds: [`speci-qua-b-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`speci-qua-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b/entry 2 stabilized as ratio-exponent determination.

### Entry speci-qua-b-003 — Exponent as immanent qualitative determination and power-law relation

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 81
- lineEnd: 105

Summary:

The exponent is not an external fixed quotient but the immanent qualitative determination of quantum, producing a power-determined and numerically incommensurable relation against arithmetical progression.

Key points: (KeyPoint)

- k1. Exponent must not be reduced to an external fixed quotient.
- k2. Exponent is the immanent qualitative determination that constitutes the ratio.
- k3. External quantum changes arithmetically by additive numerical one.
- k4. Specifying qualitative reaction yields a distinct, power-like and incommensurable relation.

Claims: (Claim)

- c1. id: speci-qua-b-003-c1
  - subject: exponent
  - predicate: is_not
  - object: merely_external_fixed_quotient
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [81-86] exponent is not just fixed external quantum/quotient.

- c2. id: speci-qua-b-003-c2
  - subject: exponent
  - predicate: constitutes
  - object: immanent_qualitative_determination_of_ratio
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [87-92] exponent is the strictly immanent qualitative determination constituting ratio.

- c3. id: speci-qua-b-003-c3
  - subject: specifying_reaction
  - predicate: produces
  - object: power_determined_numerically_incommensurable_series_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [93-105] additive arithmetical progression is answered by qualitatively specified, incommensurable power relation.

Claim ↔ key point map:

- speci-qua-b-003-c1 -> k1
- speci-qua-b-003-c2 -> k2
- speci-qua-b-003-c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-c-001
  - note: subsection b culminates in ratio/exponent determination that opens subsection c's relational unity of qualities.
  - sourceClaimIds: [`speci-qua-b-003-c2`, `speci-qua-b-003-c3`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`speci-qua-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection b completed in three paragraph entries per decomposition lock.

### Entry speci-qua-c-001 — Relational unity of qualitative sides under one measure

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 109
- lineEnd: 142

Summary:

The qualitative and quantitative sides are reciprocally determined within one measure-determinateness, where the exponent expresses their immanent quantitative relation as a qualitative unity.

Key points: (KeyPoint)

- k1. The qualitative side exists only in reference to the external quantitative side and sublates its externality.
- k2. The two sides are qualitatively self-standing yet bound in one measure-determination.
- k3. Their determination is mediated by exponent within the double-being of quantum (external and specific).
- k4. Measure is the immanent quantitative relating of two qualities.

Claims: (Claim)

- c1. id: speci-qua-c-001-c1
  - subject: qualitative_side_of_quantum
  - predicate: exists_as
  - object: reference_to_external_quantitative_side_that_it_sublates
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [109-117] qualitative side refers to and sublates external quantitative side.

- c2. id: speci-qua-c-001-c2
  - subject: two_measure_sides
  - predicate: are_bound_by
  - object: one_measure_determination_with_exponent
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [118-140] sides are interwoven under one determination of measure and exponent.

- c3. id: speci-qua-c-001-c3
  - subject: measure
  - predicate: is
  - object: immanent_quantitative_relation_of_two_qualities
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [141-142] explicit concluding determination of measure.

Claim ↔ key point map:

- speci-qua-c-001-c1 -> k1
- speci-qua-c-001-c2 -> k2, k3
- speci-qua-c-001-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-qua-c-002
  - note: from unity of two qualities to explicit variable magnitude and realized measure.
  - sourceClaimIds: [`speci-qua-c-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`speci-qua-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: speci-qua-b-003
  - note: subsection c.1 secures subsection b's exponent determination as immanent rather than external.
  - sourceClaimIds: [`speci-qua-c-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`speci-qua-b-003-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection c/entry 1 stabilized by numerical label `1`.

### Entry speci-qua-c-002 — Variable magnitude, realized measure, and power-determined relation

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/specifying-measure.txt`
- lineStart: 144
- lineEnd: 245

Summary:

Measure determines variable magnitude as qualitatively structured by power relations, yielding realized measure and clarifying the role of intensive/extensive sides beyond merely formal direct ratio.

Key points: (KeyPoint)

- k1. Measure as sublated quantum introduces essential variability where quantum is determined by qualitative otherness.
- k2. Specifying measure posits plurality of measures in one external quantum and yields realized measure.
- k3. Genuine variable magnitude is qualitatively determined through ratio of powers, not mere formal alteration.
- k4. Intensive/extensive moments orient unit and amount roles, while higher power relations better realize quantitative qualification.

Claims: (Claim)

- c1. id: speci-qua-c-002-c1
  - subject: measure
  - predicate: determines
  - object: variable_magnitude_as_qualitatively_othered_quantum
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [144-159] variable magnitude follows from sublated quantum and qualitative moment in specifying measure.

- c2. id: speci-qua-c-002-c2
  - subject: differentiation_of_quantum_in_common_external_medium
  - predicate: yields
  - object: realized_measure
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [160-170] differentiated quantitative existence in one medium is identified as real measure.

- c3. id: speci-qua-c-002-c3
  - subject: genuine_variable_magnitude
  - predicate: is_determined_by
  - object: ratio_of_powers_with_qualitative_priority_over_formal_direct_ratio
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [172-189] genuine alteration is of quantum as such and is qualitative ratio of powers.
    - [210-245] intensive/extensive assignment and higher power relations articulate fuller realization.

Claim ↔ key point map:

- speci-qua-c-002-c1 -> k1
- speci-qua-c-002-c2 -> k2
- speci-qua-c-002-c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: speci-spm-001
  - note: Part B closes by handing off realized, power-determined measure to Part C's being-for-itself in measure.
  - sourceClaimIds: [`speci-qua-c-002-c2`, `speci-qua-c-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`speci-spm-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection c completed by numerical labels `1` and `2`; Part B first-order pass complete.
