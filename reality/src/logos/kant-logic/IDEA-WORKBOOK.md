# Idea Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file treats Idea as the architectonic container closure of General Logic.
- Life and Knowing are the two internal regions of this container.
- `LIFE-WORKBOOK.md` and `COGNITION-WORKBOOK.md` remain subordinate workbooks under this frame.
- It preserves a speculative reserve without importing later Hegelian Objectivity or Absolute Idea language.

## Quick orientation

- First question: what is Idea doing here?
  Answer: it recollects General Logic into the container of methodical Life and perfected Knowing.
- Second question: what is the first major operator split?
  Answer: Life fixes rule, method, and content-extension distinctness; Knowing fixes relation, clarity, faculties, and perfection.
- Third question: what caution governs this workbook?
  Answer: it must not force later Objectivity into Kant, even though the chapter already pressures toward speculative closure.

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

## Session: 2026-03-27 (container pass)

Scope:

- files:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- pass policy: 1 marker entry + expanded analytic entries by architectonic operator

Decision:

- Regenerate the derivative layers from `idea.md` without further revising the Kant source.
- Treat Idea as a container closure that recollects General Logic into Life and Knowing.
- Make the content-extension paragraph explicit as a decisive operator split inside Life.
- Preserve the chapter's closing role while naming its speculative reserve.
- Keep `LIFE-WORKBOOK.md` and `COGNITION-WORKBOOK.md` subordinate to this file.

### Entry kant-idea - Marker `Idea`

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:3-16`
  - `idea.md:21-29`
  - `idea.md:78-86`
  - `idea.md:459-465`
- summary: Idea functions as the closing container of General Logic, dividing into Life and Knowing while holding an explicit speculative reserve at the point of closure.

Key points: (KeyPoint)

- k1. Idea divides into Life and Knowing.
- k2. Idea is the architectonic container closure of General Logic.
- k3. Life and Knowing are internal regions of that container.
- k4. The chapter closes genuinely without yet explicitly unfolding speculation.

Claims: (Claim)

- c1. id: kant-idea-c1
  - subject: idea
  - predicate: divides_into
  - object: life_and_knowing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `idea.md:8-16`
    - `idea.md:21-29`
    - `idea.md:78-86`

- c2. id: kant-idea-c2
  - subject: idea
  - predicate: functions_as
  - object: architectonic_container_closure_of_general_logic
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:3-16`

- c3. id: kant-idea-c3
  - subject: idea
  - predicate: closes_with
  - object: speculative_reserve_without_explicit_speculative_unfolding
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `idea.md:459-465`

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
- notes: marker entry now fixes Idea as a container closure and names the speculative reserve without importing Objectivity.

### Entry kant-idea-life-method - `Idea`: life as ruled and methodical science

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:21-56`
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
    - `idea.md:21-35`

- c2. id: kant-idea-life-method-c2
  - subject: method
  - predicate: forms
  - object: science_as_systematic_connection_of_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `idea.md:38-56`

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

### Entry kant-idea-life-content-extension - `Idea`: life as operator of content and extension

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:57-76`
- summary: The doctrine of method advances cognition by targeting distinctness, thoroughness, and systematic ordering, then differentiates content from extension by assigning each its own clarifying operator.

Key points: (KeyPoint)

- k1. Method aims at distinctness, thoroughness, and systematic order.
- k2. Distinctness depends on concepts in respect of both content and extension.
- k3. Exposition and definition clarify what is contained in concepts.
- k4. Logical division clarifies what is contained under concepts.

Claims: (Claim)

- c1. id: kant-idea-life-content-extension-c1
  - subject: doctrine_of_method
  - predicate: aims_at
  - object: distinctness_thoroughness_and_systematic_ordering
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:57-66`

- c2. id: kant-idea-life-content-extension-c2
  - subject: exposition_and_definition
  - predicate: further
  - object: distinct_consciousness_of_conceptual_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:67-74`

- c3. id: kant-idea-life-content-extension-c3
  - subject: logical_division
  - predicate: furthers
  - object: distinct_consciousness_of_conceptual_extension
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:67-76`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry isolates the content-extension split through which life produces conceptual distinctness.
  - sourceClaimIds: [`kant-idea-life-content-extension-c1`, `kant-idea-life-content-extension-c2`, `kant-idea-life-content-extension-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-idea-c2`]

Review outcome:

- review_pending
- notes: the content-extension distinction is now explicit and no longer buried under a generic distinctness heading.

### Entry kant-idea-knowing-relation - `Idea`: knowing as relation, consciousness, and logical threshold

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:78-156`
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
    - `idea.md:96-107`

- c2. id: kant-idea-knowing-relation-c2
  - subject: logic
  - predicate: applies_to
  - object: clear_representations_with_distinct_manifold_consciousness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `idea.md:124-156`

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

### Entry kant-idea-knowing-distinctness - `Idea`: sensible and intellectual distinctness

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:203-234`
- summary: Knowing differentiates sensible from intellectual distinctness and fixes analysis as explication of what a concept already contains rather than material augmentation.

Key points: (KeyPoint)

- k1. Distinctness is sensible in intuition and intellectual in concepts.
- k2. Intellectual distinctness is produced by analysis of conceptual marks.
- k3. Analysis adds nothing to a concept materially.
- k4. Distinctness improves the concept only as to form.

Claims: (Claim)

- c1. id: kant-idea-knowing-distinctness-c1
  - subject: distinctness_of_cognition
  - predicate: divides_into
  - object: sensible_and_intellectual_forms
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:203-219`

- c2. id: kant-idea-knowing-distinctness-c2
  - subject: analysis_of_concepts
  - predicate: adds
  - object: no_new_matter
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:217-234`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry preserves the distinction between explicated content and added matter.
  - sourceClaimIds: [`kant-idea-knowing-distinctness-c1`, `kant-idea-knowing-distinctness-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-idea-c1`, `kant-idea-c2`]

Review outcome:

- review_pending
- notes: this entry now makes explicit why analysis can clarify content without materially producing it.

### Entry kant-idea-knowing-faculties - `Idea`: faculty split between sensibility and understanding

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:236-256`
- summary: Knowing organizes cognition by the logical difference between sensibility and understanding, distributing cognition into intuitions and concepts.

Key points: (KeyPoint)

- k1. All cognitions are either intuitions or concepts in this logical respect.
- k2. Sensibility is the faculty of intuitions.
- k3. Understanding is the faculty of concepts.

Claims: (Claim)

- c1. id: kant-idea-knowing-faculties-c1
  - subject: cognition
  - predicate: is_logically_distributed_by
  - object: sensibility_and_understanding
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `idea.md:236-246`

- c2. id: kant-idea-knowing-faculties-c2
  - subject: sensibility_and_understanding
  - predicate: provide
  - object: intuitions_and_concepts_respectively
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `idea.md:241-256`

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
- notes: the faculty split remains logical here rather than metaphysical.

### Entry kant-idea-perfection - `Idea`: logical and aesthetic perfection under regulated combination

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:258-378`
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
    - `idea.md:258-299`

- c2. id: kant-idea-perfection-c2
  - subject: combination_of_logical_and_aesthetic_perfection
  - predicate: is_governed_by
  - object: logical_priority_formal_beauty_and_caution_toward_stimulation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `idea.md:360-372`

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
  - `idea.md:374-456`
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
    - `idea.md:374-422`

- c2. id: kant-idea-four-moments-c2
  - subject: truth
  - predicate: is
  - object: principal_perfection_as_ground_of_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `idea.md:437-456`

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

### Entry kant-idea-speculative-reserve - `Idea`: closure without explicit speculative unfolding

- sourceFiles:
  - `idea.md`
  - `IDEA-DISTILLATION.md`
- lineSpans:
  - `idea.md:459-465`
- summary: The chapter explicitly reaches closure, but it still withholds a fully developed speculative moment, leaving that pressure registered rather than unfolded.

Key points: (KeyPoint)

- k1. The chapter reaches genuine closure.
- k2. Speculation is present here only as reserve.
- k3. The reserve should not be filled by importing later Objectivity language.

Claims: (Claim)

- c1. id: kant-idea-speculative-reserve-c1
  - subject: idea
  - predicate: reaches
  - object: genuine_closure
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - `idea.md:459-461`

- c2. id: kant-idea-speculative-reserve-c2
  - subject: idea
  - predicate: withholds
  - object: explicit_speculative_unfolding
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - `idea.md:461-465`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-idea
  - targetWorkbook: `IDEA-WORKBOOK.md`
  - note: this entry keeps the closing reserve visible without forcing a later architectonic into the chapter.
  - sourceClaimIds: [`kant-idea-speculative-reserve-c1`, `kant-idea-speculative-reserve-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-idea-c3`]

Review outcome:

- review_pending
- notes: this entry preserves the line between Kant's closure and a later speculative development.
