# Idea Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Idea as the architectonic closure of General Logic.
- Life and Cognition are subordinate workbooks under this architectonic frame.
- It does not import later Hegelian Objectivity or Absolute Idea language.

## Quick orientation

- First question: what is Idea doing here?
  Answer: it gathers logic into methodical life and perfected knowing.
- Second question: what is the first major operator split?
  Answer: Life fixes rule, method, and the means of distinctness; Knowing fixes cognition's relation, faculties, and perfections.
- Third question: why does this file matter for the earlier modules?
  Answer: it reassembles method, cognition, and the four moments into one architectonic close, then delegates the fuller detail to the subordinate Life and Cognition workbooks.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `idea.md` and `IDEA-DISTILLATION.md`
- Upstream source authority: `idea.md`
- This workbook covers only the idea module.

## Clean-room rules

- Keep the pass on the General Logic side.
- Do not import later Hegelian Idea structures into this file.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (operator pass)

Scope:

- files:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by architectonic operator

Decision:

- Rebuild Idea directly from `idea.md` rather than inherit the older Life/Cognition seed framing.
- Keep Life and Knowing as internal divisions of the Idea chapter.
- Treat `LIFE-WORKBOOK.md` and `COGNITION-WORKBOOK.md` as subordinate workbooks under this file.
- Preserve the chapter's closing role as architectonic completion of General Logic.
- Make the four moments of perfection explicit so the chapter reconnects with the operator grammar already fixed in Judgment.

### Entry kant-idea - Marker `Idea`

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:7-172`
- summary: Idea closes General Logic by dividing logic into methodical life and perfected knowing, then recombining them in one schema of cognitive perfection.

Key points: (KeyPoint)

- k1. Idea divides into Life and Knowing.
- k2. Life fixes the methodical ordering of science.
- k3. Knowing fixes the relation, faculties, and perfection of cognition.

Claims: (Claim)

- c1. id: kant-idea-c1
  - subject: idea
  - predicate: divides_into
  - object: life_and_knowing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `IDEA-DISTILLATION.md:7-17`

- c2. id: kant-idea-c2
  - subject: idea
  - predicate: functions_as
  - object: architectonic_closure_of_general_logic
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:7-17`
    - `IDEA-DISTILLATION.md:162-172`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: kant-life
  - targetWorkbook: `LIFE-WORKBOOK.md`
  - note: Life is the subordinate workbook for the method-side of Idea.
  - sourceClaimIds: [`kant-idea-c1`, `kant-idea-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-life-c1`, `kant-life-c2`]

- r2. type: unfolds_to
  - targetEntryId: kant-cognition
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: Cognition is the subordinate workbook for the content-bearing knowing-side of Idea.
  - sourceClaimIds: [`kant-idea-c1`, `kant-idea-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`kant-cognition-c1`, `kant-cognition-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes the whole chapter before the internal operators are separated and delegated to the subordinate Life and Cognition workbooks.

### Entry kant-idea-life-method - `Idea`: life as ruled and methodical science

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:21-59`
- summary: Life begins by subordinating cognition to rule and culminates in method as the connective form of science.

Key points: (KeyPoint)

- k1. All cognition must conform to a rule.
- k2. Manner and method are differentiated as free and compelled procedure.
- k3. Method gives science its systematic form by connecting manifold cognition.

Claims: (Claim)

- c1. id: kant-idea-life-method-c1
  - subject: life_of_logic
  - predicate: begins_with
  - object: rule_and_the_distinction_between_manner_and_method
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:21-31`

- c2. id: kant-idea-life-method-c2
  - subject: method
  - predicate: forms
  - object: science_as_systematic_connection_of_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `IDEA-DISTILLATION.md:35-59`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry fixes the life-side operator of rule-governed scientific ordering.
  - sourceClaimIds: [`kant-idea-life-method-c1`, `kant-idea-life-method-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: method is treated here as operator of composition, not as a loose pedagogical add-on.

### Entry kant-idea-life-distinctness - `Idea`: life as operator of conceptual clarification

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:63-87`
- summary: The doctrine of method advances cognition by targeting distinctness, thoroughness, and systematic ordering through exposition, definition, and division.

Key points: (KeyPoint)

- k1. Method aims at distinctness, thoroughness, and systematic order.
- k2. Distinctness depends on concepts in respect of both content and extension.
- k3. Exposition, definition, and division are the clarifying operators of life.

Claims: (Claim)

- c1. id: kant-idea-life-distinctness-c1
  - subject: doctrine_of_method
  - predicate: aims_at
  - object: distinctness_thoroughness_and_systematic_ordering
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:63-73`

- c2. id: kant-idea-life-distinctness-c2
  - subject: exposition_definition_and_logical_division
  - predicate: further
  - object: distinct_consciousness_of_content_and_extension
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:77-87`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry isolates the methodical means through which life produces conceptual distinctness.
  - sourceClaimIds: [`kant-idea-life-distinctness-c1`, `kant-idea-life-distinctness-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c2`]

Review outcome:

- review_pending
- notes: the clarifying operators are kept explicit so this file stays aligned with the concept pass.

### Entry kant-idea-knowing-relation - `Idea`: knowing as relation, consciousness, and logical threshold

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:91-115`
- summary: Knowing begins from cognition's relation to object and subject, then fixes clarity and distinctness as the threshold for logical treatment.

Key points: (KeyPoint)

- k1. Cognition has a twofold relation to object and subject.
- k2. Matter and form restate this relation inside cognition itself.
- k3. Logic concerns clear representations and their distinct articulation of a manifold.

Claims: (Claim)

- c1. id: kant-idea-knowing-relation-c1
  - subject: cognition
  - predicate: has_twofold_relation
  - object: object_representation_and_subject_consciousness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `IDEA-DISTILLATION.md:91-101`

- c2. id: kant-idea-knowing-relation-c2
  - subject: logic
  - predicate: applies_to
  - object: clear_representations_with_distinct_manifold_consciousness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `IDEA-DISTILLATION.md:105-115`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry fixes the relational opening of knowing and the threshold at which logical form begins.
  - sourceClaimIds: [`kant-idea-knowing-relation-c1`, `kant-idea-knowing-relation-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: this restores consciousness as the formal threshold instead of treating knowing as a loose epistemic survey.

### Entry kant-idea-knowing-faculties - `Idea`: sensible distinctness, intellectual distinctness, and faculty split

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:119-143`
- summary: Knowing differentiates sensible from intellectual distinctness and organizes cognition by the logical split between sensibility and understanding.

Key points: (KeyPoint)

- k1. Distinctness is sensible in intuition and intellectual in concepts.
- k2. Intellectual distinctness is produced by analysis of conceptual marks.
- k3. Cognition divides logically into intuitions from sensibility and concepts from understanding.

Claims: (Claim)

- c1. id: kant-idea-knowing-faculties-c1
  - subject: distinctness_of_cognition
  - predicate: divides_into
  - object: sensible_and_intellectual_forms
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:119-129`

- c2. id: kant-idea-knowing-faculties-c2
  - subject: cognition
  - predicate: is_logically_distributed_by
  - object: sensibility_and_understanding
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `IDEA-DISTILLATION.md:133-143`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry preserves the faculty grammar that organizes knowing internally.
  - sourceClaimIds: [`kant-idea-knowing-faculties-c1`, `kant-idea-knowing-faculties-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: the file now keeps the intuition-concept split tied to the sensible-intellectual distinction.

### Entry kant-idea-perfection - `Idea`: logical and aesthetic perfection under regulated combination

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:147-158`
- summary: Logical and aesthetic perfection are heterogeneous norms that must be combined only under rules preserving logical priority.

Key points: (KeyPoint)

- k1. Logical perfection concerns the object under understanding.
- k2. Aesthetic perfection concerns the subject under sensibility.
- k3. Their combination is possible only when logical priority and caution toward stimulation are maintained.

Claims: (Claim)

- c1. id: kant-idea-perfection-c1
  - subject: perfection_of_cognition
  - predicate: divides_into
  - object: logical_and_aesthetic_perfection
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `IDEA-DISTILLATION.md:147-158`

- c2. id: kant-idea-perfection-c2
  - subject: combination_of_logical_and_aesthetic_perfection
  - predicate: is_governed_by
  - object: logical_priority_formal_beauty_and_caution_toward_stimulation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `IDEA-DISTILLATION.md:147-158`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry keeps the understanding-sensibility conflict explicit without sacrificing the chapter's practical synthesis.
  - sourceClaimIds: [`kant-idea-perfection-c1`, `kant-idea-perfection-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c2`]

Review outcome:

- review_pending
- notes: this entry preserves the rules of combination rather than flattening perfection into taste or truth alone.

### Entry kant-idea-four-moments - `Idea`: perfection through quantity, quality, relation, and modality

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `IDEA-DISTILLATION.md:162-172`
- summary: Idea ends by measuring perfection through the four moments of cognition and their aesthetic counterparts, with truth grounding unity as the principal perfection.

Key points: (KeyPoint)

- k1. Logical perfection is measured by universality, distinctness, truth, and certainty.
- k2. Aesthetic perfection has parallel moments for those same four headings.
- k3. Truth is principal because it grounds unity through relation to the object.

Claims: (Claim)

- c1. id: kant-idea-four-moments-c1
  - subject: perfection_of_cognition
  - predicate: is_measured_by
  - object: quantity_quality_relation_and_modality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `IDEA-DISTILLATION.md:162-172`

- c2. id: kant-idea-four-moments-c2
  - subject: truth
  - predicate: is
  - object: principal_perfection_as_ground_of_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `IDEA-DISTILLATION.md:162-172`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry closes the file by reconnecting Idea to the four-moment grammar already explicit in judgment.
  - sourceClaimIds: [`kant-idea-four-moments-c1`, `kant-idea-four-moments-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c2`]

Review outcome:

- review_pending
- notes: this entry preserves the final architectonic recapitulation instead of letting the chapter end as a loose appendix on taste.
