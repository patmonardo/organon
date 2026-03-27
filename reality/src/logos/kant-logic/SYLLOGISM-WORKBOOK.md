# Syllogism Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file now fixes the operator families inside understanding, reason, and the power of judgment.
- Use the distillation as the fast commentary path out of this workbook.
- Return to the source text when checking whether the workbook remains complete and correct.

## Quick orientation

- First question: what is the core processor split here?
  Answer: understanding transforms form, reason mediates necessity, and the power of judgment ascends reflectively from particulars.
- Second question: what is new in this pass?
  Answer: the file now treats chains, compression, and inferential defects as part of the machinery rather than as appendices.
- Third question: how should this workbook be read?
  Answer: read the workbook for logical commitments, the distillation for quick commentary, and the source text for verification.
- Fourth question: what can be deferred on the first return?
  Answer: the figure reductions, mixed forms, sorites, and defect taxonomy; marker plus understanding plus judgment-power are enough to fix the faculty architecture.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `syllogism.md` and `SYLLOGISM-DISTILLATION.md`
- Distillation role: commentary on workbooked logic and structural moves.
- Upstream source authority: `syllogism.md`
- This workbook covers only the syllogism module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not flatten immediate inference, reason, and reflection into one undifferentiated process.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (faculty pass)

Scope:

- files:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- pass policy: expanded analytic entries by inferential operator family

Decision:

- Keep the faculty split explicit across understanding, reason, and the power of judgment.
- Expand beyond overview language so the inferential operators are not reduced to species labels.
- Preserve a staged-entry path so later study can begin with the faculty architecture before the categorical technicalities.

### Entry kant-syllogism — Marker `Syllogism`

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `SYLLOGISM-DISTILLATION.md:16-336`
- summary: Syllogism provides a faculty-differentiated logic of cognitive transition whose essence lies in inferential operator families rather than bare species lists.

Key points: (KeyPoint)

- k1. Immediate inference transforms form while preserving matter.
- k2. Reason mediates necessity through rules, marks, and operator families.
- k3. The power of judgment generalizes from particulars through reflective operators.

Claims: (Claim)

- c1. id: kant-syllogism-c1
  - subject: syllogism
  - predicate: layers
  - object: understanding_reason_and_power_of_judgment_transition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:16-336`

- c2. id: kant-syllogism-c2
  - subject: syllogism
  - predicate: functions_as
  - object: formal_operations_manual_for_extending_cognition
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:326-336`

Relations: (Relation)

- r1. type: part_of
  - targetEntryId: kant-subj-triad
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: syllogism is the third moment in the subjectivity triad.
  - sourceClaimIds: [`kant-syllogism-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`kant-subj-triad-c1`]

Review outcome:

- review_pending
- notes: marker entry now frames syllogism as operator machinery rather than a sequence of enumerated kinds.

### Entry kant-syllogism-understanding — `Syllogism`: understanding as formal transformation

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `SYLLOGISM-DISTILLATION.md:16-95`
- summary: Inferences of the understanding operate by transforming judgment-form while preserving matter across quantity, quality, relation, and modality.

Key points: (KeyPoint)

- k1. Immediate inference alters form while keeping matter fixed.
- k2. The understanding distributes this across subalternation, opposition, conversion, and contraposition.
- k3. Understanding is the operator-family of formal transformation.

Claims: (Claim)

- c1. id: kant-syllogism-understanding-c1
  - subject: immediate_inference
  - predicate: transforms
  - object: form_without_new_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:16-26`

- c2. id: kant-syllogism-understanding-c2
  - subject: understanding_inference
  - predicate: distributes_into
  - object: subalternation_opposition_conversion_and_contraposition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:29-95`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry fixes the understanding-side operator family inside syllogism.
  - sourceClaimIds: [`kant-syllogism-understanding-c1`, `kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

- r2. type: specializes
  - targetEntryId: kant-judgment-quantity
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: subalternation reworks judgment-quantity as an immediate inference operator.
  - sourceClaimIds: [`kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-judgment-quantity-c2`]

- r3. type: specializes
  - targetEntryId: kant-judgment-quality
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: opposition reworks judgment-quality as a truth-transfer operator.
  - sourceClaimIds: [`kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-judgment-quality-c1`]

- r4. type: specializes
  - targetEntryId: kant-judgment-relation
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: conversion reworks judgment-relation as lawful transposition within immediate inference.
  - sourceClaimIds: [`kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-judgment-relation-c2`]

- r5. type: specializes
  - targetEntryId: kant-judgment-modality
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: contraposition reworks judgment-modality as modal intensification inside inference.
  - sourceClaimIds: [`kant-syllogism-understanding-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-judgment-modality-c2`]

Review outcome:

- review_pending
- notes: immediate inference is now treated as an operator-family rather than a prelude.

### Entry kant-syllogism-reason — `Syllogism`: reason as mediated closure

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `SYLLOGISM-DISTILLATION.md:114-218`
- summary: Inference of reason operates by mediated closure through rules, premises, consequentia, and the three operator families of major premise.

Key points: (KeyPoint)

- k1. Reason subsumes a case under a universal rule.
- k2. Premises are the matter and consequentia in the conclusion is the form.
- k3. Categorical, hypothetical, and disjunctive are distinct reason-operators selected by the major premise.

Claims: (Claim)

- c1. id: kant-syllogism-reason-c1
  - subject: inference_of_reason
  - predicate: has_form
  - object: mediated_closure_through_consequentia
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:114-136`

- c2. id: kant-syllogism-reason-c2
  - subject: inference_of_reason
  - predicate: distributes_into
  - object: categorical_hypothetical_and_disjunctive_operator_families
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:140-218`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry fixes reason as an operator-family of mediated necessity rather than a single generic step.
  - sourceClaimIds: [`kant-syllogism-reason-c1`, `kant-syllogism-reason-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

- r2. type: specializes
  - targetEntryId: kant-judgment-categorical
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: categorical syllogism turns categorical predicative binding into rule-mediated term mediation.
  - sourceClaimIds: [`kant-syllogism-reason-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-judgment-categorical-c1`, `kant-judgment-categorical-c2`]

- r3. type: specializes
  - targetEntryId: kant-judgment-hypothetical
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: hypothetical syllogism extends consequentia from judgment-form into inferential closure.
  - sourceClaimIds: [`kant-syllogism-reason-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-judgment-hypothetical-c2`]

- r4. type: specializes
  - targetEntryId: kant-judgment-disjunctive
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: disjunctive syllogism extends exhaustive division from judgment-form into inferential exclusion and affirmation.
  - sourceClaimIds: [`kant-syllogism-reason-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-judgment-disjunctive-c1`, `kant-judgment-disjunctive-c2`]

Review outcome:

- review_pending
- notes: the figure-system and mixed forms are preserved as internal reason-side machinery.

### Entry kant-syllogism-judgment-power — `Syllogism`: power of judgment as reflective ascent

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `SYLLOGISM-DISTILLATION.md:224-248`
- summary: The power of judgment operates by reflective ascent from many particulars toward empirical universals through induction and analogy.

Key points: (KeyPoint)

- k1. Reflective judgment proceeds from particular to universal.
- k2. Its principle is common ground across the many.
- k3. Induction and analogy are its two operator modes.

Claims: (Claim)

- c1. id: kant-syllogism-judgment-power-c1
  - subject: power_of_judgment_inference
  - predicate: proceeds_by
  - object: reflective_ascent_from_particular_to_universal
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:224-234`

- c2. id: kant-syllogism-judgment-power-c2
  - subject: power_of_judgment_inference
  - predicate: operates_through
  - object: induction_and_analogy
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:238-248`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry fixes the reflective operator-family distinct from both understanding and reason.
  - sourceClaimIds: [`kant-syllogism-judgment-power-c1`, `kant-syllogism-judgment-power-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`]

Review outcome:

- review_pending
- notes: reflective judgment is now preserved as a distinct inferential operator-family rather than a closing appendix.

### Entry kant-syllogism-chain-defect — `Syllogism`: chains, compression, and defects of mediation

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `SYLLOGISM-DISTILLATION.md:254-320`
- summary: Syllogism extends beyond single inferences into chains, compressed chains, and the formal defects that corrupt mediated reasoning.

Key points: (KeyPoint)

- k1. Composite reasoning chains multiple inferences by subordination.
- k2. Sorites compresses chained mediation into shortened inferential paths.
- k3. Fallacies, leaps, and bad proofs are failures in the economy of mediation.

Claims: (Claim)

- c1. id: kant-syllogism-chain-defect-c1
  - subject: composite_inference
  - predicate: extends_into
  - object: polysyllogism_prosyllogism_episyllogism_and_sorites
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:254-278`

- c2. id: kant-syllogism-chain-defect-c2
  - subject: inferential_defect
  - predicate: consists_in
  - object: failure_of_mediation_or_proof_proportion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `SYLLOGISM-DISTILLATION.md:282-320`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: this entry preserves the extended processor behavior of chains, compression, and inferential failure.
  - sourceClaimIds: [`kant-syllogism-chain-defect-c1`, `kant-syllogism-chain-defect-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-syllogism-c1`, `kant-syllogism-c2`]

Review outcome:

- review_pending
- notes: the late syllogism material is now treated as operator extension and anti-operator failure, not as miscellaneous leftovers.
