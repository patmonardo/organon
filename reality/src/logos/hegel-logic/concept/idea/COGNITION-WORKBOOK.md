# Cognition Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `cognition/`, not a replacement for the local idea workbook, compiler workbook, or the part workbooks.
- Read it to follow the chapter-level spine from cognition as judgment, through the true, into the good.
- Use the local `cognition/COGNITION-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the Idea leaves immediate life and becomes explicit as cognition.
- Second question: what is the chapter's central operator?
  Answer: the doubled relation of subjective and objective concept.
- Third question: where does the subfolder lead?
  Answer: to the Absolute Idea.

## Authority + format lock (must persist)

- Working extraction references: `cognition/cognition-idea.txt`, `cognition/idea-of-the-true.txt`, `cognition/idea-of-the-good.txt`, and `COGNITION-DISTILLATION.md`
- Upstream source authority: `cognition/cognition-idea.txt`, `cognition/idea-of-the-true.txt`, `cognition/idea-of-the-good.txt`
- This workbook covers the `cognition/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Cognition-folder pass)

Scope:

- files:
  - `cognition/cognition-idea.txt`
  - `cognition/idea-of-the-true.txt`
  - `cognition/idea-of-the-good.txt`
  - `COGNITION-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `cognition/` subfolder.
- Preserve the older local workbook stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: cognition-opening, true, good.

### Entry hegel-idea-cognition — Marker `Cognition`

- sourceFiles:
  - `cognition/cognition-idea.txt`
  - `cognition/idea-of-the-true.txt`
  - `cognition/idea-of-the-good.txt`
- lineSpans:
  - `cognition/cognition-idea.txt:1-531`
  - `cognition/idea-of-the-true.txt:1-2197`
  - `cognition/idea-of-the-good.txt:1-364`
- summary: The `cognition/` subfolder presents the Idea in judgment, unfolds the theoretical and practical Idea, and culminates in the transition to the Absolute Idea.

Key points: (KeyPoint)

- k1. Cognition is the Idea in judgment.
- k2. The chapter unfolds through true and good.
- k3. The chapter culminates in the Absolute Idea.

Claims: (Claim)

- c1. id: hegel-idea-cognition-c1
  - subject: cognition_chapter
  - predicate: captures
  - object: idea_as_judgmental_doubling
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `cognition/cognition-idea.txt:1-41`

- c2. id: hegel-idea-cognition-c2
  - subject: cognition_chapter
  - predicate: unfolds_through
  - object: idea_of_the_true_and_idea_of_the_good
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `cognition/idea-of-the-true.txt:1-2197`
    - `cognition/idea-of-the-good.txt:1-364`

- c3. id: hegel-idea-cognition-c3
  - subject: cognition_chapter
  - predicate: culminates_in
  - object: absolute_idea
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/idea-of-the-good.txt:329-364`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-idea-true
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: cognition first appears as the idea of the true.
  - sourceClaimIds: [`hegel-idea-cognition-c1`, `hegel-idea-cognition-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-idea-true-c1`, `hegel-idea-true-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-idea-good
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: theoretical cognition requires practical completion.
  - sourceClaimIds: [`hegel-idea-cognition-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-idea-good-c1`, `hegel-idea-good-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-idea-absolute
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: the practical Idea opens the Absolute Idea.
  - sourceClaimIds: [`hegel-idea-cognition-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-absolute-c1`]

Review outcome:

- review_pending
- notes: this marker fixes Cognition at readable chapter scale over the strong true/good files.

### Entry hegel-idea-true — `Cognition`: the idea of the true

- sourceFiles:
  - `cognition/idea-of-the-true.txt`
- lineSpans:
  - `cognition/idea-of-the-true.txt:1-2197`
- summary: The idea of the true gives cognition's theoretical movement, passing through analytic and synthetic forms toward a more adequate grasp of objectivity.

Key points: (KeyPoint)

- k1. The true is the theoretical Idea.
- k2. Analytic cognition is necessary but limited.
- k3. Synthetic cognition carries the advance.

Claims: (Claim)

- c1. id: hegel-idea-true-c1
  - subject: idea_of_the_true
  - predicate: is
  - object: theoretical_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/idea-of-the-true.txt:46-50`

- c2. id: hegel-idea-true-c2
  - subject: idea_of_the_true
  - predicate: unfolds_through
  - object: analytic_and_synthetic_cognition
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/idea-of-the-true.txt:223-225`
    - `cognition/idea-of-the-true.txt:678-678`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-good
  - targetWorkbook: `COGNITION-WORKBOOK.md`
  - note: theoretical cognition requires practical completion.
  - sourceClaimIds: [`hegel-idea-true-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-good-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the theoretical side at readable scale over the massive local file.

### Entry hegel-idea-good — `Cognition`: the idea of the good and absolute-idea threshold

- sourceFiles:
  - `cognition/idea-of-the-good.txt`
- lineSpans:
  - `cognition/idea-of-the-good.txt:1-364`
- summary: The idea of the good gives cognition's practical side, reveals the finitude of the ought, and drives cognition beyond itself into the Absolute Idea.

Key points: (KeyPoint)

- k1. The good is the practical Idea.
- k2. The good remains finite as ought while opposed to actuality.
- k3. The practical Idea passes into the Absolute Idea.

Claims: (Claim)

- c1. id: hegel-idea-good-c1
  - subject: idea_of_the_good
  - predicate: is
  - object: practical_idea
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `cognition/idea-of-the-good.txt:43-50`

- c2. id: hegel-idea-good-c2
  - subject: idea_of_the_good
  - predicate: passes_into
  - object: absolute_idea
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `cognition/idea-of-the-good.txt:154-162`
    - `cognition/idea-of-the-good.txt:329-364`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-idea-absolute
  - targetWorkbook: `ABSOLUTE-IDEA-WORKBOOK.md`
  - note: practical completion opens the Absolute Idea.
  - sourceClaimIds: [`hegel-idea-good-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-idea-absolute-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the cognition-to-absolute-idea handoff explicit.
