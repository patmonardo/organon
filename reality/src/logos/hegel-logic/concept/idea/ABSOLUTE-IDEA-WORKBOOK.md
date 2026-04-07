# Absolute Idea Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic chapter-level workbook for the Absolute Idea.
- The local dense source stack for this chapter lives under `speculation/`; this workbook layers over that stack rather than replacing it.
- Read it to follow the chapter-level spine from reconciliation of truth and good, through method, into the transition to nature.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the Idea finally knows itself as the unity of truth and good.
- Second question: what is the chapter's central operator?
  Answer: method as the concept's own self-movement.
- Third question: where does the chapter lead?
  Answer: to nature.

## Authority + format lock (must persist)

- Working extraction references: `speculation/absolute-idea.txt`, `speculation/speculation.txt`, `speculation/method-advance.txt`, and `ABSOLUTE-IDEA-DISTILLATION.md`
- Upstream source authority: `speculation/absolute-idea.txt`, `speculation/speculation.txt`, `speculation/method-advance.txt`
- This workbook covers the third chapter of the Idea, whose dense local stack lives under `speculation/`.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Absolute-Idea chapter pass)

Scope:

- files:
  - `speculation/absolute-idea.txt`
  - `speculation/speculation.txt`
  - `speculation/method-advance.txt`
  - `ABSOLUTE-IDEA-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Expose the readable chapter at the root as `ABSOLUTE-IDEA-*`.
- Preserve the `speculation/` folder as the dense local stack carrying the chapter's detailed artifacts.
- Keep the emphasis on the chapter sequence: absolute idea, method, nature-transition.

### Entry hegel-idea-absolute — Marker `Absolute Idea`

- sourceFiles:
  - `speculation/absolute-idea.txt`
  - `speculation/speculation.txt`
  - `speculation/method-advance.txt`
- lineSpans:
  - `speculation/absolute-idea.txt:1-296`
  - `speculation/speculation.txt:1-889`
  - `speculation/method-advance.txt:1-735`
- summary: The third chapter presents the Absolute Idea as the unity of theoretical and practical Idea, unfolds method as the concept's own self-movement, and culminates in the Idea's free release into nature.

Key points: (KeyPoint)

- k1. The Absolute Idea unifies truth and good.
- k2. Method is the self-movement of the concept.
- k3. The chapter culminates in the transition to nature.

Claims: (Claim)

- c1. id: hegel-idea-absolute-c1
  - subject: absolute_idea_chapter
  - predicate: captures
  - object: unity_of_theoretical_and_practical_idea
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `speculation/absolute-idea.txt:1-96`

- c2. id: hegel-idea-absolute-c2
  - subject: absolute_idea_chapter
  - predicate: unfolds_through
  - object: method_as_self_movement_of_the_concept
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `speculation/absolute-idea.txt:97-239`
    - `speculation/speculation.txt:1-889`

- c3. id: hegel-idea-absolute-c3
  - subject: absolute_idea_chapter
  - predicate: culminates_in
  - object: free_transition_to_nature
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `speculation/absolute-idea.txt:240-296`
    - `speculation/method-advance.txt:1-735`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-idea-method
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: reconciliation becomes explicit as method.
  - sourceClaimIds: [`hegel-idea-absolute-c1`, `hegel-idea-absolute-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-idea-method-c1`, `hegel-idea-method-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-idea-nature-threshold
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: the method culminates in the free discharge into nature.
  - sourceClaimIds: [`hegel-idea-absolute-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-nature-threshold-c1`, `hegel-idea-nature-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the third Idea chapter at readable scale while preserving the local `speculation/` stack.

### Entry hegel-idea-method — `Absolute Idea`: method as concept's self-movement

- sourceFiles:
  - `speculation/speculation.txt`
  - `speculation/method-advance.txt`
- lineSpans:
  - `speculation/speculation.txt:1-889`
  - `speculation/method-advance.txt:1-735`
- summary: Method is not an external technique but the concept's own self-determining movement, analytic and synthetic at once because it takes determination from the subject matter itself.

Key points: (KeyPoint)

- k1. Method is the concept's own activity.
- k2. It is analytic and synthetic together.
- k3. Dialectic is the immanent advance of the subject matter.

Claims: (Claim)

- c1. id: hegel-idea-method-c1
  - subject: method
  - predicate: is
  - object: self_movement_of_the_concept
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `speculation/absolute-idea.txt:97-239`
    - `speculation/speculation.txt:1-155`

- c2. id: hegel-idea-method-c2
  - subject: method
  - predicate: unfolds_as
  - object: analytic_synthetic_and_dialectical_advance
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `speculation/method-advance.txt:1-260`
    - `speculation/speculation.txt:156-889`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-nature-threshold
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: the method culminates in the free discharge of the Idea.
  - sourceClaimIds: [`hegel-idea-method-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-nature-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the method section centered on immanent movement rather than external procedure.

### Entry hegel-idea-nature-threshold — `Absolute Idea`: free release into nature

- sourceFiles:
  - `speculation/absolute-idea.txt`
  - `speculation/method-advance.txt`
- lineSpans:
  - `speculation/absolute-idea.txt:240-296`
  - `speculation/method-advance.txt:680-735`
- summary: The Absolute Idea completes itself by freely determining itself as external idea, so that nature is the Idea's own self-release rather than a fall into alien being.

Key points: (KeyPoint)

- k1. The transition to nature is free discharge.
- k2. The external idea remains transparent to the Idea.
- k3. Completion is the beginning of another sphere.

Claims: (Claim)

- c1. id: hegel-idea-nature-threshold-c1
  - subject: absolute_idea
  - predicate: freely_releases_itself_as
  - object: nature
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `speculation/absolute-idea.txt:261-296`

- c2. id: hegel-idea-nature-threshold-c2
  - subject: transition_to_nature
  - predicate: is
  - object: beginning_of_another_science_not_a_loss_of_the_idea
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `speculation/method-advance.txt:680-735`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: nature-threshold
  - targetWorkbook: `pending_nature_upgrade`
  - note: the next sphere is nature.
  - sourceClaimIds: [`hegel-idea-nature-threshold-c1`, `hegel-idea-nature-threshold-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: pending_cross_workbook

Review outcome:

- review_pending
- notes: this entry keeps the nature handoff explicit without prewriting the next sphere.
