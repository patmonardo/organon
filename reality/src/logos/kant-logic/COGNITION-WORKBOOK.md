# Cognition Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Cognition as the content-bearing expansion of Knowing inside the Idea chapter.
- It should be read after Life, since Life supplies the method-discipline that Cognition then fills out under the four moments.
- This pass keeps coverage broad, but more compact than the older Cognition artifacts.

## Quick orientation

- First question: what does Cognition add beyond Life?
  Answer: Life gives the methodical discipline; Cognition gives the fuller doctrine of perfection under quantity, relation, quality, and modality.
- Second question: what is the main organizational law here?
  Answer: each of the four moments becomes a substantial logical field rather than a short heading.
- Third question: why does this matter for the larger study?
  Answer: it completes the Knowing side of Idea and gives the truth, mark, and certainty doctrines that frame later CPR-oriented reading.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `cognition.md` and `COGNITION-DISTILLATION.md`
- Upstream source authority: `cognition.md`
- This workbook covers only the cognition module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Treat Cognition as an internal division of Idea's Knowing side, not as a warrant to import later ontological systems.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-27 (compact four-moment pass)

Scope:

- files:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- pass policy: 1 marker entry + compact analytic entries by major moment and operator cluster

Decision:

- Regenerate the derivative layers from `cognition.md` without touching the Kant source.
- Keep the four-moment architecture explicit.
- Make the file more compact than the previous Cognition pass while preserving coverage of horizon, truth, marks, distinctness, certainty, proof, and science.
- Keep the transition from Life to Cognition visible: method first, then content-bearing perfection.

### Entry kant-cognition - Marker `Cognition`

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:1-1265`
- summary: Cognition is the content-bearing expansion of Knowing inside Idea, unfolding the perfection of cognition through the four moments of quantity, relation, quality, and modality.

Key points: (KeyPoint)

- k1. Cognition is organized by quantity, relation, quality, and modality.
- k2. Each moment expands into a substantial logical field rather than a short formal heading.
- k3. Cognition completes the broader doctrine of perfection on the Knowing side of Idea.

Claims: (Claim)

- c1. id: kant-cognition-c1
  - subject: cognition
  - predicate: is_ordered_by
  - object: quantity_relation_quality_and_modality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `cognition.md:1-304`
    - `cognition.md:307-668`
    - `cognition.md:671-1013`
    - `cognition.md:1015-1265`

- c2. id: kant-cognition-c2
  - subject: cognition
  - predicate: functions_as
  - object: content_bearing_expansion_of_knowing_within_idea
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:1-1265`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: cognition is the fuller development of the Knowing side inside Idea.
  - sourceClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes the chapter as the broad four-moment expansion of Knowing.

### Entry kant-cognition-quantity - `Cognition`: horizon, ignorance, architectonic, and importance

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:3-304`
- summary: Quantity begins as extensive and intensive cognition, becomes horizon as purposive self-measurement, and closes in learned ignorance, architectonic planning, and importance measured by consequences.

Key points: (KeyPoint)

- k1. Quantity divides into extensive and intensive forms.
- k2. Horizon aligns cognition with capacities, interests, and ends.
- k3. Ignorance is the negative imperfection opposed to quantitative perfection.
- k4. Architectonic and consequence-importance govern the systematic expansion of cognition.

Claims: (Claim)

- c1. id: kant-cognition-quantity-c1
  - subject: quantity_of_cognition
  - predicate: divides_into
  - object: extensive_and_intensive_forms
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition.md:3-14`

- c2. id: kant-cognition-quantity-c2
  - subject: horizon
  - predicate: functions_as
  - object: purposive_self_measurement_of_cognition_by_capacity_and_end
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:15-173`

- c3. id: kant-cognition-quantity-c3
  - subject: ignorance
  - predicate: is
  - object: negative_imperfection_of_quantitative_cognition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:174-244`

- c4. id: kant-cognition-quantity-c4
  - subject: architectonic_and_logical_importance
  - predicate: order
  - object: systematic_expansion_of_cognition_by_plan_and_consequence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:245-304`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry gathers the quantity moment as horizon rather than mere amount.
  - sourceClaimIds: [`kant-cognition-quantity-c1`, `kant-cognition-quantity-c2`, `kant-cognition-quantity-c3`, `kant-cognition-quantity-c4`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the quantity-side perfection summarized in Idea.
  - sourceClaimIds: [`kant-cognition-quantity-c1`, `kant-cognition-quantity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-idea-four-moments-c1`]

Review outcome:

- review_pending
- notes: quantity is preserved as horizon, finitude, and system, not just extension.

### Entry kant-cognition-relation - `Cognition`: truth, criteria, error, and touchstones

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:309-668`
- summary: Relation opens from truth, denies a universal material criterion, secures formal criteria, and turns to error, illusion, and the maxims that discipline judgment.

Key points: (KeyPoint)

- k1. Truth is the essential condition of all perfection of cognition.
- k2. No universal material criterion of truth is possible, but a universal formal criterion is.
- k3. Formal truth is governed by contradiction, sufficient reason, and excluded middle.
- k4. Error arises through illusion and is checked by common understanding and the three maxims of thinking.

Claims: (Claim)

- c1. id: kant-cognition-relation-c1
  - subject: truth
  - predicate: is
  - object: essential_condition_of_all_perfection_of_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition.md:309-412`

- c2. id: kant-cognition-relation-c2
  - subject: formal_criteria_of_truth
  - predicate: are
  - object: contradiction_sufficient_reason_and_excluded_middle
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition.md:413-480`

- c3. id: kant-cognition-relation-c3
  - subject: error
  - predicate: arises_from
  - object: illusion_and_illicit_subjective_judgment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:481-668`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes relation as the full truth-and-error doctrine of the chapter.
  - sourceClaimIds: [`kant-cognition-relation-c1`, `kant-cognition-relation-c2`, `kant-cognition-relation-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the relation-side perfection of cognition as truth.
  - sourceClaimIds: [`kant-cognition-relation-c1`, `kant-cognition-relation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-four-moments-c1`, `kant-idea-four-moments-c2`]

Review outcome:

- review_pending
- notes: the source's diagnostic emphasis remains explicit: illusion must be disclosed, not merely refuted downstream.

### Entry kant-cognition-quality-marks - `Cognition`: discursive cognition and the grammar of marks

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:673-850`
- summary: Quality begins from discursive cognition through marks and unfolds a grammar of internal and external uses, kinds of marks, and logical essence.

Key points: (KeyPoint)

- k1. Human cognition is discursive and proceeds through marks.
- k2. Marks have internal use in derivation and external use in comparison.
- k3. Marks divide into several logical types.
- k4. Logic can determine only the logical essence of things.

Claims: (Claim)

- c1. id: kant-cognition-quality-marks-c1
  - subject: human_cognition
  - predicate: proceeds_as
  - object: discursive_cognition_through_marks
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition.md:673-715`

- c2. id: kant-cognition-quality-marks-c2
  - subject: marks
  - predicate: form
  - object: grammar_of_internal_external_and_multiple_determinative_types
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:716-850`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes quality first as the doctrine of marks and logical essence.
  - sourceClaimIds: [`kant-cognition-quality-marks-c1`, `kant-cognition-quality-marks-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: quality begins from the determinative units of discursive thought, not from a vague doctrine of clarity alone.

### Entry kant-cognition-quality-distinctness - `Cognition`: clarity, distinctness, adequacy, and the analysis-synthesis split

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:851-1013`
- summary: Quality rises from clarity to distinctness, differentiates logical and aesthetic forms, unfolds adequacy, and distinguishes analytic clarification from synthetic content-expansion before closing in the degrees of cognition.

Key points: (KeyPoint)

- k1. Distinctness is a higher degree of clarity.
- k2. Logical and aesthetic distinctness must be distinguished.
- k3. Exhaustiveness, profundity, precision, and adequacy structure complete distinctness.
- k4. Analysis makes concepts distinct, while synthesis makes distinct concepts.
- k5. Cognition ascends by degrees from representation to comprehension.

Claims: (Claim)

- c1. id: kant-cognition-quality-distinctness-c1
  - subject: quality_perfection_of_cognition
  - predicate: culminates_in
  - object: adequacy_through_exhaustiveness_profundity_and_precision
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:851-917`

- c2. id: kant-cognition-quality-distinctness-c2
  - subject: analysis_and_synthesis
  - predicate: differ_as
  - object: making_a_concept_distinct_and_making_a_distinct_concept
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:918-964`

- c3. id: kant-cognition-quality-distinctness-c3
  - subject: degrees_of_cognition
  - predicate: ascend_from
  - object: representation_to_comprehension
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:965-1013`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-quality-marks
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry develops the mark-doctrine into distinctness, adequacy, and the final ladder of cognition.
  - sourceClaimIds: [`kant-cognition-quality-distinctness-c1`, `kant-cognition-quality-distinctness-c2`, `kant-cognition-quality-distinctness-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`kant-cognition-quality-marks-c1`, `kant-cognition-quality-marks-c2`]

- r2. type: part_of
  - targetEntryId: kant-idea-four-moments
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry develops the quality-side perfection of cognition as distinctness.
  - sourceClaimIds: [`kant-cognition-quality-distinctness-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-four-moments-c1`]

Review outcome:

- review_pending
- notes: the distinction between analytic clarification and synthetic content-expansion is kept explicit because it is central to the chapter's logic.

### Entry kant-cognition-modality-holding - `Cognition`: holding-to-be-true, opinion, and belief

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:1017-1171`
- summary: Modality opens from holding-to-be-true as the subjective side of truth and differentiates opinion and belief by their forms of insufficiency and domain.

Key points: (KeyPoint)

- k1. Holding-to-be-true is the subjective relation of judgment to truth.
- k2. It divides into opinion, belief, and knowledge.
- k3. Opinion is provisional and empirically restricted.
- k4. Belief is practically necessary and morally grounded without becoming knowledge.

Claims: (Claim)

- c1. id: kant-cognition-modality-holding-c1
  - subject: holding_to_be_true
  - predicate: is
  - object: subjective_relation_of_judgment_to_truth
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:1017-1072`

- c2. id: kant-cognition-modality-holding-c2
  - subject: opinion_and_belief
  - predicate: function_as
  - object: empirically_provisional_and_practically_moral_modes_of_insufficient_warrant
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:1073-1171`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry fixes modality first as a doctrine of subjective warrant.
  - sourceClaimIds: [`kant-cognition-modality-holding-c1`, `kant-cognition-modality-holding-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: modality begins from stance toward truth, not from abstract modal vocabulary.

### Entry kant-cognition-modality-knowledge - `Cognition`: knowledge, proof, science, and conviction

- sourceFiles:
  - `cognition.md`
  - `COGNITION-DISTILLATION.md`
- lineSpans:
  - `cognition.md:1172-1265`
- summary: Knowledge is sufficient on both objective and subjective sides, divides by source and kind of certainty, and culminates in proof, science, and the distinction between logical and practical conviction.

Key points: (KeyPoint)

- k1. Knowledge divides into empirical and rational certainty.
- k2. Rational certainty divides into mathematical and philosophical forms.
- k3. Certainty may be immediate or mediated by proof.
- k4. Science is cognition as system rather than aggregate.
- k5. Conviction divides into logical and practical forms.

Claims: (Claim)

- c1. id: kant-cognition-modality-knowledge-c1
  - subject: knowledge
  - predicate: divides_into
  - object: empirical_and_rational_certainty_with_further_internal_divisions
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `cognition.md:1172-1211`

- c2. id: kant-cognition-modality-knowledge-c2
  - subject: certainty_and_science
  - predicate: culminate_in
  - object: proof_system_and_logical_practical_conviction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `cognition.md:1212-1265`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-cognition-modality-holding
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: this entry completes modality by moving from insufficient to sufficient warrant.
  - sourceClaimIds: [`kant-cognition-modality-knowledge-c1`, `kant-cognition-modality-knowledge-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`]
  - targetClaimIds: [`kant-cognition-modality-holding-c1`, `kant-cognition-modality-holding-c2`]

Review outcome:

- review_pending
- notes: the chapter closes by joining certainty, proof, system, and conviction into one final modality field.
