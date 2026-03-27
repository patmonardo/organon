# Cognition Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Cognition as the content-bearing expansion of Knowing inside the Idea chapter.
- It covers quantity, relation, quality, and modality as large operator families.

## Quick orientation

- First question: what does Cognition add beyond Life?
  Answer: Life gives the methodological operators; Cognition gives the contentful perfection of cognition under the four moments.
- Second question: why is this file larger than the earlier ones?
  Answer: this chapter expands each moment into a substantial doctrine rather than a narrow formal inventory.
- Third question: how should it be read?
  Answer: use the marker and the four moment-entries first, then the subordinate entries for the operator detail.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `cognition.md` and `COGNITION-DISTILLATION.md`
- Upstream source authority: `cognition.md`
- This workbook covers only the cognition module.

## Clean-room rules

- Keep the pass on the General Logic side, even where the chapter exceeds a stricter formal-logic boundary.
- Treat Cognition as an internal division of Idea's Knowing side, not as a license to import later ontological systems.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (expanded operator pass)

Scope:

- files:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by the four moments and their internal operator families

Decision:

- Keep Cognition long-form because the source itself unfolds a broad doctrine rather than a short appendix.
- Preserve the four-moment organization explicitly.
- Treat this as the content-bearing extension of Knowing beyond the methodological side fixed in Life.
- Expand entries rather than compressing horizon, truth, marks, and modality into overview language.

### Entry kant-cognition - Marker `Cognition`

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:7-479`
- summary: Cognition expands the four moments of perfection into a large content-bearing logic of horizon, truth, marks, certainty, and science.

Key points: (KeyPoint)

- k1. Cognition is organized by quantity, relation, quality, and modality.
- k2. Each moment expands into a substantial operator-family rather than a short formal heading.
- k3. Cognition is the strongest internal extension of Idea's Knowing side.

Claims: (Claim)

- c1. id: kant-cognition-c1
  - subject: cognition
  - predicate: is_ordered_by
  - object: quantity_relation_quality_and_modality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `COGNITION-DISTILLATION.md:7-17`

- c2. id: kant-cognition-c2
  - subject: cognition
  - predicate: functions_as
  - object: content_bearing_perfection_of_knowing_within_idea
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:7-17`
    - `COGNITION-DISTILLATION.md:469-479`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: cognition is the fuller, content-bearing development of the knowing side inside Idea.
  - sourceClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes the scope of the chapter before the four moments are unpacked.

### Entry kant-cognition-quantity-horizon - `Cognition`: quantity as horizon and self-measurement

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:21-101`
- summary: Quantity begins as extensive and intensive magnitude, then becomes horizon as the operator that measures cognition by ends, capacities, and differentiated limits.

Key points: (KeyPoint)

- k1. Quantity divides into extensive and intensive magnitudes.
- k2. Horizon aligns cognition with capacities and ends.
- k3. Horizon differentiates logical, aesthetic, practical, objective, and subjective determinations.

Claims: (Claim)

- c1. id: kant-cognition-quantity-horizon-c1
  - subject: quantity_of_cognition
  - predicate: divides_into
  - object: extensive_and_intensive_magnitudes
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `COGNITION-DISTILLATION.md:21-31`

- c2. id: kant-cognition-quantity-horizon-c2
  - subject: horizon
  - predicate: functions_as
  - object: quantity_operator_for_purposive_self_measurement_of_cognition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:35-101`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes the first moment as horizon rather than bare amount.
  - sourceClaimIds: [`kant-cognition-quantity-horizon-c1`, `kant-cognition-quantity-horizon-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the quantity-side perfection of cognition summarized in the Idea workbook.
  - sourceClaimIds: [`kant-cognition-quantity-horizon-c1`, `kant-cognition-quantity-horizon-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-four-moments-c1`]

Review outcome:

- review_pending
- notes: quantity is preserved here as an operator of horizon, not just a count of cognitions.

### Entry kant-cognition-quantity-ignorance - `Cognition`: ignorance and the boundaries of horizon

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:105-115`
- summary: Ignorance is the negative imperfection opposed to quantitative perfection and divides into objective and subjective forms.

Key points: (KeyPoint)

- k1. Ignorance is inseparable from finite cognition.
- k2. Objective ignorance divides into material and formal kinds.
- k3. Subjective ignorance divides into learned and common forms.

Claims: (Claim)

- c1. id: kant-cognition-quantity-ignorance-c1
  - subject: ignorance
  - predicate: is
  - object: negative_imperfection_opposed_to_quantitative_perfection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:105-115`

- c2. id: kant-cognition-quantity-ignorance-c2
  - subject: ignorance
  - predicate: divides_into
  - object: objective_material_formal_and_subjective_learned_common_forms
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:105-115`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-quantity-horizon
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: learned ignorance is the negative boundary-consciousness that belongs to horizon.
  - sourceClaimIds: [`kant-cognition-quantity-ignorance-c1`, `kant-cognition-quantity-ignorance-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-quantity-horizon-c2`]

Review outcome:

- review_pending
- notes: this entry keeps ignorance tied to quantitative finitude rather than scattering it into a generic epistemology.

### Entry kant-cognition-quantity-architectonic - `Cognition`: plan, architectonic, and logical importance

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:119-143`
- summary: Quantity culminates in architectonic planning of the sciences and in the measurement of importance by consequences rather than difficulty.

Key points: (KeyPoint)

- k1. Cognition has inner worth beyond immediate utility.
- k2. Architectonic orders cognitions systematically by kinship and plan.
- k3. Intensive quantity measures importance by consequence, not difficulty.

Claims: (Claim)

- c1. id: kant-cognition-quantity-architectonic-c1
  - subject: architectonic
  - predicate: orders
  - object: sciences_by_kinship_and_systematic_connection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:119-129`

- c2. id: kant-cognition-quantity-architectonic-c2
  - subject: logical_importance
  - predicate: is_measured_by
  - object: fertility_of_consequences_rather_than_difficulty
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:133-143`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry closes quantity by shifting from horizon-limits to purposive systematic expansion.
  - sourceClaimIds: [`kant-cognition-quantity-architectonic-c1`, `kant-cognition-quantity-architectonic-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: the chapter's anti-utilitarian and architectonic turn is kept explicit.

### Entry kant-cognition-relation-truth - `Cognition`: truth and its formal criteria

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:147-199`
- summary: Relation opens with truth, rejects a universal material criterion, and secures formal criteria through contradiction, sufficient reason, and excluded middle.

Key points: (KeyPoint)

- k1. Truth is the essential condition of perfection.
- k2. No universal material criterion of truth is possible.
- k3. Formal truth is governed by contradiction, sufficient reason, and excluded middle.

Claims: (Claim)

- c1. id: kant-cognition-relation-truth-c1
  - subject: truth
  - predicate: is
  - object: essential_condition_of_all_perfection_of_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `COGNITION-DISTILLATION.md:147-157`

- c2. id: kant-cognition-relation-truth-c2
  - subject: formal_criteria_of_truth
  - predicate: are
  - object: contradiction_sufficient_reason_and_excluded_middle
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `COGNITION-DISTILLATION.md:161-185`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes the relation-moment first as a doctrine of truth and formal validity.
  - sourceClaimIds: [`kant-cognition-relation-truth-c1`, `kant-cognition-relation-truth-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the relation-side perfection of cognition as truth.
  - sourceClaimIds: [`kant-cognition-relation-truth-c1`, `kant-cognition-relation-truth-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-four-moments-c1`, `kant-idea-four-moments-c2`]

Review outcome:

- review_pending
- notes: relation is preserved here as a truth-doctrine, not only as a property of propositions.

### Entry kant-cognition-relation-error - `Cognition`: grounds, error, illusion, and epistemic responsibility

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:189-241`
- summary: Relation tests truth through grounds and consequences, then explains error through illusion and the subject's misuse of judgment.

Key points: (KeyPoint)

- k1. Grounds and consequences provide positive and negative tests of truth.
- k2. Error is falsehood taken for truth through illusion.
- k3. Ignorance comes from limitation, but error comes from our own judgment.

Claims: (Claim)

- c1. id: kant-cognition-relation-error-c1
  - subject: truth_testing
  - predicate: proceeds_by
  - object: negative_apagogic_and_positive_grounding_relations_of_consequence
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:189-199`

- c2. id: kant-cognition-relation-error-c2
  - subject: error
  - predicate: arises_from
  - object: illusion_and_illicit_subjective_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:203-241`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-relation-truth
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry operationalizes the truth-doctrine through testing and diagnosis of error.
  - sourceClaimIds: [`kant-cognition-relation-error-c1`, `kant-cognition-relation-error-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-relation-truth-c1`, `kant-cognition-relation-truth-c2`]

Review outcome:

- review_pending
- notes: the source's strongest move is preserved here: refuting error is secondary to exposing illusion.

### Entry kant-cognition-relation-touchstones - `Cognition`: common understanding and the maxims of thinking

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:245-255`
- summary: Relation closes by supplying external touchstones of truth and the three maxims of enlightened, extended, and coherent thought.

Key points: (KeyPoint)

- k1. Others' judgments can expose subjective illusion.
- k2. Common understanding is a touchstone for speculative reason.
- k3. Thinking for oneself, from others' standpoints, and consistently form the three maxims.

Claims: (Claim)

- c1. id: kant-cognition-relation-touchstones-c1
  - subject: common_understanding_and_others_judgments
  - predicate: function_as
  - object: external_touchstones_of_truth_and_error_detection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:245-255`

- c2. id: kant-cognition-relation-touchstones-c2
  - subject: maxims_of_thought
  - predicate: are
  - object: enlightened_extended_and_coherent_modes_of_thought
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:245-255`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-relation-error
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: these touchstones are the practical disciplines for detecting the illusions isolated in the prior entry.
  - sourceClaimIds: [`kant-cognition-relation-touchstones-c1`, `kant-cognition-relation-touchstones-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-relation-error-c2`]

Review outcome:

- review_pending
- notes: the relation-moment ends here as a discipline of judgment, not just a list of truth tests.

### Entry kant-cognition-quality-marks - `Cognition`: discursive cognition and the grammar of marks

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:259-311`
- summary: Quality begins by defining discursive cognition as cognition through marks and then classifies the functional kinds of marks.

Key points: (KeyPoint)

- k1. Human cognition is discursive and proceeds through marks.
- k2. Marks have internal and external uses.
- k3. Marks divide into analytic/synthetic, coordinate/subordinate, negative/affirmative, important, sufficient, and necessary forms.

Claims: (Claim)

- c1. id: kant-cognition-quality-marks-c1
  - subject: human_cognition
  - predicate: proceeds_as
  - object: discursive_cognition_through_marks
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `COGNITION-DISTILLATION.md:259-269`

- c2. id: kant-cognition-quality-marks-c2
  - subject: marks
  - predicate: form
  - object: a_grammar_of_internal_external_and_multiple_determinative_types
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:273-311`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes the quality-moment first as the doctrine of marks.
  - sourceClaimIds: [`kant-cognition-quality-marks-c1`, `kant-cognition-quality-marks-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: this restores the full mark-grammar instead of reducing quality to mere clarity.

### Entry kant-cognition-quality-essence - `Cognition`: logical essence, distinctness, and adequacy

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:315-353`
- summary: Quality derives logical essence from necessary marks and then develops distinctness into lucidity, exhaustiveness, profundity, precision, and adequacy.

Key points: (KeyPoint)

- k1. Logic treats logical essence, not real essence.
- k2. Distinctness is the higher degree of clarity and divides logically and aesthetically.
- k3. Complete distinctness unfolds into exhaustiveness, profundity, precision, and adequacy.

Claims: (Claim)

- c1. id: kant-cognition-quality-essence-c1
  - subject: logical_essence
  - predicate: is
  - object: first_basic_concept_of_the_necessary_marks_of_a_thing
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:315-325`

- c2. id: kant-cognition-quality-essence-c2
  - subject: quality_perfection_of_cognition
  - predicate: culminates_in
  - object: adequacy_through_exhaustiveness_precision_and_profundity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:329-353`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-quality-marks
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: logical essence and adequacy are the higher-order organization of the mark-doctrine.
  - sourceClaimIds: [`kant-cognition-quality-essence-c1`, `kant-cognition-quality-essence-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-quality-marks-c1`, `kant-cognition-quality-marks-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the quality-side perfection of cognition as distinctness.
  - sourceClaimIds: [`kant-cognition-quality-essence-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`kant-idea-four-moments-c1`]

Review outcome:

- review_pending
- notes: logical and aesthetic distinctness are kept in productive tension rather than merged.

### Entry kant-cognition-quality-analysis - `Cognition`: analytic distinctness and the degrees of cognition

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:357-381`
- summary: Quality distinguishes analysis from synthesis in making concepts distinct and ends with the graded ascent from representation to comprehension.

Key points: (KeyPoint)

- k1. Analysis makes a given concept distinct, while synthesis makes a distinct concept.
- k2. Logic is concerned primarily with analytic distinctness.
- k3. Cognition advances through seven degrees from representation to comprehension.

Claims: (Claim)

- c1. id: kant-cognition-quality-analysis-c1
  - subject: analysis_and_synthesis_of_distinctness
  - predicate: differ_as
  - object: formal_clarification_of_given_concepts_and_content_expansion_of_constructed_concepts
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:357-367`

- c2. id: kant-cognition-quality-analysis-c2
  - subject: degrees_of_cognition
  - predicate: ascend_from
  - object: representation_to_comprehension
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:371-381`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-quality-essence
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry shows how the doctrine of distinctness becomes both a method of clarification and a ladder of perfected cognition.
  - sourceClaimIds: [`kant-cognition-quality-analysis-c1`, `kant-cognition-quality-analysis-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-quality-essence-c2`]

Review outcome:

- review_pending
- notes: the source's distinction between making a concept distinct and making a distinct concept is preserved explicitly.

### Entry kant-cognition-modality-holding - `Cognition`: holding-to-be-true and its three modi

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:385-409`
- summary: Modality begins from the subjective side of truth and divides holding-to-be-true into opinion, belief, and knowledge.

Key points: (KeyPoint)

- k1. Holding-to-be-true is the subjective side of truth.
- k2. Certainty and uncertainty differ by consciousness of necessity or contingency.
- k3. Opinion, belief, and knowledge are the three basic modi of warrant.

Claims: (Claim)

- c1. id: kant-cognition-modality-holding-c1
  - subject: holding_to_be_true
  - predicate: is
  - object: subjective_relation_of_judgment_to_truth
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:385-395`

- c2. id: kant-cognition-modality-holding-c2
  - subject: holding_to_be_true
  - predicate: divides_into
  - object: opinion_belief_and_knowledge
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `COGNITION-DISTILLATION.md:399-409`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes modality first as a doctrine of warrant and subjective stance.
  - sourceClaimIds: [`kant-cognition-modality-holding-c1`, `kant-cognition-modality-holding-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: modality is preserved as judgmental stance rather than mere modal vocabulary.

### Entry kant-cognition-modality-opinion-belief - `Cognition`: provisional opinion and practical belief

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:413-437`
- summary: Opinion is provisional empirical insufficiency, while belief is free but practically necessary moral holding-to-be-true.

Key points: (KeyPoint)

- k1. Opinion is provisional and restricted to empirical cognition.
- k2. Belief is practically necessary despite objective insufficiency.
- k3. Belief is grounded in moral interest rather than theoretical proof.

Claims: (Claim)

- c1. id: kant-cognition-modality-opinion-belief-c1
  - subject: opinion
  - predicate: is
  - object: provisional_empirically_restricted_holding_to_be_true
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:413-423`

- c2. id: kant-cognition-modality-opinion-belief-c2
  - subject: belief
  - predicate: is
  - object: practical_morally_grounded_subjectively_sufficient_holding_to_be_true
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:427-437`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-modality-holding
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry expands the first two modes of holding-to-be-true in their proper domains.
  - sourceClaimIds: [`kant-cognition-modality-opinion-belief-c1`, `kant-cognition-modality-opinion-belief-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-modality-holding-c2`]

Review outcome:

- review_pending
- notes: belief is kept practical here and not collapsed into testimony or loose plausibility.

### Entry kant-cognition-modality-knowledge-proof - `Cognition`: knowledge, certainty, and proof

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:441-465`
- summary: Knowledge is sufficient on both objective and subjective sides, divides by source and mode of certainty, and culminates in the doctrine of proof.

Key points: (KeyPoint)

- k1. Knowledge divides into empirical and rational certainty.
- k2. Rational certainty divides into mathematical and philosophical forms.
- k3. Certainty may be immediate or mediated through proof.

Claims: (Claim)

- c1. id: kant-cognition-modality-knowledge-proof-c1
  - subject: knowledge
  - predicate: divides_into
  - object: empirical_and_rational_certainty_with_further_internal_divisions
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:441-451`

- c2. id: kant-cognition-modality-knowledge-proof-c2
  - subject: certainty
  - predicate: culminates_in
  - object: immediate_starting_points_and_direct_indirect_proof
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:455-465`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-modality-holding
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry expands the strongest mode of holding-to-be-true into its internal topology of certainty and proof.
  - sourceClaimIds: [`kant-cognition-modality-knowledge-proof-c1`, `kant-cognition-modality-knowledge-proof-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-modality-holding-c2`]

Review outcome:

- review_pending
- notes: demonstration and acroamatic proof are preserved as distinct proof-modes rather than footnotes.

### Entry kant-cognition-modality-science - `Cognition`: science and conviction

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `COGNITION-DISTILLATION.md:469-479`
- summary: Modality closes by converting knowledge into science as system and by distinguishing logical from practical conviction.

Key points: (KeyPoint)

- k1. Science is cognition as system rather than aggregate.
- k2. System rests on an idea of the whole.
- k3. Conviction divides into logical and practical forms.

Claims: (Claim)

- c1. id: kant-cognition-modality-science-c1
  - subject: science
  - predicate: is
  - object: systematic_complex_of_cognition_ordered_by_an_idea_of_the_whole
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `COGNITION-DISTILLATION.md:469-479`

- c2. id: kant-cognition-modality-science-c2
  - subject: conviction
  - predicate: divides_into
  - object: logical_and_practical_forms
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `COGNITION-DISTILLATION.md:469-479`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-modality-knowledge-proof
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: science and conviction are the systematic and existential closure of the certainty-doctrine.
  - sourceClaimIds: [`kant-cognition-modality-science-c1`, `kant-cognition-modality-science-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-cognition-modality-knowledge-proof-c1`, `kant-cognition-modality-knowledge-proof-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-knowing-relation
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry completes the knowing-side expansion from cognition to science and conviction.
  - sourceClaimIds: [`kant-cognition-modality-science-c1`, `kant-cognition-modality-science-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-knowing-relation-c1`, `kant-idea-knowing-relation-c2`]

Review outcome:

- review_pending
- notes: the chapter ends in system and conviction, which is why it feels stronger than a narrow formal-logic appendix.
