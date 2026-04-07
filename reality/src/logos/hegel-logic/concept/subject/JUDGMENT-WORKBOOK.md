# Judgment Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `judgment/`, not a replacement for the local idea workbook, compiler workbook, or the more granular local distillation stack.
- Read it to follow the chapter-level spine from self-diremption, through subject-predicate polarity, into the transition to syllogism.
- Use the local `judgment/JUDGMENT-IDEA-WORKBOOK.md` and local distillations for denser analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the concept realizes itself as judgment and drives that unstable realization into syllogism.
- Second question: what is the chapter's central operator?
  Answer: the contradiction of asserted identity and preserved separation.
- Third question: where does the subfolder lead?
  Answer: to syllogism.

## Authority + format lock (must persist)

- Working extraction references: `judgment/judgment-idea.txt` and `JUDGMENT-DISTILLATION.md`
- Upstream source authority: `judgment/judgment-idea.txt`
- This workbook covers the `judgment/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook and distillation stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Judgment-folder pass)

Scope:

- files:
  - `judgment/judgment-idea.txt`
  - `JUDGMENT-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `judgment/` subfolder.
- Preserve the older local workbook and distillation stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: self-division, subject-predicate polarity, syllogistic transition.

### Entry hegel-subject-judgment — Marker `Judgment`

- sourceFiles:
  - `judgment/judgment-idea.txt`
- lineSpans:
  - `judgment/judgment-idea.txt:1-451`
- summary: The `judgment/` subfolder presents judgment as the concept's self-diremption, unfolds subject and predicate as opposed totalities, and culminates in the transition to syllogism.

Key points: (KeyPoint)

- k1. Judgment is the concept's own self-diremption.
- k2. The chapter unfolds through the polarity of subject and predicate.
- k3. Judgment culminates in syllogism.

Claims: (Claim)

- c1. id: hegel-subject-judgment-c1
  - subject: judgment_chapter
  - predicate: is
  - object: concept_as_self_diremption
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `judgment/judgment-idea.txt:1-49`

- c2. id: hegel-subject-judgment-c2
  - subject: judgment_chapter
  - predicate: unfolds_through
  - object: subject_predicate_polarity_and_copula
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:50-227`

- c3. id: hegel-subject-judgment-c3
  - subject: judgment_chapter
  - predicate: culminates_in
  - object: syllogism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:229-451`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-subject-subject-predicate
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: the chapter first unfolds the polarity of its terms.
  - sourceClaimIds: [`hegel-subject-judgment-c1`, `hegel-subject-judgment-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-subject-subject-predicate-c1`, `hegel-subject-subject-predicate-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-subject-judgment-threshold
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: the contradiction of the copula drives judgment into syllogism.
  - sourceClaimIds: [`hegel-subject-judgment-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-judgment-threshold-c1`, `hegel-subject-judgment-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the chapter at a readable architectonic scale over the older local breakdown.

### Entry hegel-subject-subject-predicate — `Judgment`: subject, predicate, and copula

- sourceFiles:
  - `judgment/judgment-idea.txt`
- lineSpans:
  - `judgment/judgment-idea.txt:50-227`
- summary: Subject and predicate are first only indeterminate poles, and the copula expresses not mere grammatical combination but the belonging of predicate to the being of the subject.

Key points: (KeyPoint)

- k1. Subject and predicate are first only names or poles.
- k2. The predicate first gives the subject conceptual truth.
- k3. The copula states ontological belonging, not mere grammar.

Claims: (Claim)

- c1. id: hegel-subject-subject-predicate-c1
  - subject: subject_and_predicate
  - predicate: are_initially
  - object: indeterminate_poles_or_names
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:50-120`

- c2. id: hegel-subject-subject-predicate-c2
  - subject: copula
  - predicate: expresses
  - object: predicate_belonging_to_the_being_of_the_subject
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:149-178`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subject-judgment-threshold
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: the contradiction of this asserted unity drives the chapter onward.
  - sourceClaimIds: [`hegel-subject-subject-predicate-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-judgment-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the chapter anchored in the subject-predicate problem rather than dispersing into local taxonomies.

### Entry hegel-subject-judgment-threshold — `Judgment`: contradiction and transition to syllogism

- sourceFiles:
  - `judgment/judgment-idea.txt`
- lineSpans:
  - `judgment/judgment-idea.txt:229-451`
- summary: Judgment moves by raising the singular into universality and actualizing the universal in the singular, but because its asserted unity remains unfulfilled, the chapter passes into syllogism.

Key points: (KeyPoint)

- k1. The singular is raised into universality.
- k2. The universal becomes actual in the singular.
- k3. Judgment asserts identity without fully positing it.
- k4. The chapter turns into syllogism.

Claims: (Claim)

- c1. id: hegel-subject-judgment-threshold-c1
  - subject: judgment
  - predicate: moves_by
  - object: reciprocal_raise_of_singular_and_actualization_of_universal
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `judgment/judgment-idea.txt:229-365`

- c2. id: hegel-subject-judgment-threshold-c2
  - subject: contradiction_of_the_copula
  - predicate: drives
  - object: judgment_into_syllogism
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `judgment/judgment-idea.txt:367-451`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subject-syllogism
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: the next architectonic field is syllogism.
  - sourceClaimIds: [`hegel-subject-judgment-threshold-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`hegel-subject-syllogism-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the unity problem of judgment explicit at the chapter close.
