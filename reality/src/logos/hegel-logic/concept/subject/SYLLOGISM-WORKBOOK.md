# Syllogism Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `syllogism/`, not a replacement for the local idea workbook, compiler workbook, or the more granular local distillation stack.
- Read it to follow the chapter-level spine from restored concept, through the middle term, into the three forms of mediation.
- Use the local `syllogism/SYLLOGISM-IDEA-WORKBOOK.md` and local distillations for denser analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the concept is restored from judgment and made explicit as mediation.
- Second question: what is the chapter's central operator?
  Answer: the middle term as unity of the extremes.
- Third question: where does the subfolder lead?
  Answer: to objectivity.

## Authority + format lock (must persist)

- Working extraction references: `syllogism/syllogism-idea.txt` and `SYLLOGISM-DISTILLATION.md`
- Upstream source authority: `syllogism/syllogism-idea.txt`
- This workbook covers the `syllogism/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook and distillation stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Syllogism-folder pass)

Scope:

- files:
  - `syllogism/syllogism-idea.txt`
  - `SYLLOGISM-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `syllogism/` subfolder.
- Preserve the older local workbook and distillation stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: restored concept, middle term, existence/reflection/necessity.

### Entry hegel-subject-syllogism — Marker `Syllogism`

- sourceFiles:
  - `syllogism/syllogism-idea.txt`
- lineSpans:
  - `syllogism/syllogism-idea.txt:1-151`
- summary: The `syllogism/` subfolder presents the syllogism as the restored concept, unfolds the middle term as its essence, and culminates in the passage to objectivity.

Key points: (KeyPoint)

- k1. Syllogism restores the concept out of judgment.
- k2. The chapter centers on the middle term as mediation.
- k3. It unfolds through existence, reflection, and necessity.
- k4. It culminates in objectivity.

Claims: (Claim)

- c1. id: hegel-subject-syllogism-c1
  - subject: syllogism_chapter
  - predicate: is
  - object: concept_restored_as_explicit_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:1-17`

- c2. id: hegel-subject-syllogism-c2
  - subject: syllogism_chapter
  - predicate: unfolds_through
  - object: middle_term_and_three_forms_of_mediation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism/syllogism-idea.txt:19-151`

- c3. id: hegel-subject-syllogism-c3
  - subject: syllogism_chapter
  - predicate: culminates_in
  - object: objectivity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism/syllogism-idea.txt:137-151`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-subject-middle-term
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: the chapter first clarifies the rational meaning of mediation.
  - sourceClaimIds: [`hegel-subject-syllogism-c1`, `hegel-subject-syllogism-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-subject-middle-term-c1`, `hegel-subject-middle-term-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-subject-syllogism-threshold
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: the three forms of mediation culminate in objectivity.
  - sourceClaimIds: [`hegel-subject-syllogism-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`hegel-subject-syllogism-threshold-c1`, `hegel-subject-syllogism-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the chapter over the already existing local part structure.

### Entry hegel-subject-middle-term — `Syllogism`: rational mediation and the middle term

- sourceFiles:
  - `syllogism/syllogism-idea.txt`
- lineSpans:
  - `syllogism/syllogism-idea.txt:1-113`
- summary: The syllogism is the rational because it explicitly posits the unity of the extremes in the middle term, which is not an external connector but their ground.

Key points: (KeyPoint)

- k1. The syllogism is the restored truth of concept and judgment.
- k2. The rational is this mediated unity.
- k3. The middle term is the unity and ground of the extremes.

Claims: (Claim)

- c1. id: hegel-subject-middle-term-c1
  - subject: syllogism
  - predicate: is
  - object: the_rational_as_fully_posited_concept
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:1-84`

- c2. id: hegel-subject-middle-term-c2
  - subject: middle_term
  - predicate: is
  - object: unity_and_ground_of_the_extremes
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:86-113`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subject-syllogism-threshold
  - targetWorkbook: `SYLLOGISM-WORKBOOK.md`
  - note: the middle term is concretized through existence, reflection, and necessity.
  - sourceClaimIds: [`hegel-subject-middle-term-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-syllogism-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the chapter's center on mediation rather than on formal figures alone.

### Entry hegel-subject-syllogism-threshold — `Syllogism`: existence, reflection, necessity, objectivity

- sourceFiles:
  - `syllogism/syllogism-idea.txt`
- lineSpans:
  - `syllogism/syllogism-idea.txt:115-151`
- summary: The syllogism unfolds through existence, reflection, and necessity until the mediating factor becomes objective universality and the chapter passes into objectivity.

Key points: (KeyPoint)

- k1. Existence gives immediate mediation.
- k2. Reflection makes mediation explicit.
- k3. Necessity gives objective universality.
- k4. The chapter turns into objectivity.

Claims: (Claim)

- c1. id: hegel-subject-syllogism-threshold-c1
  - subject: syllogism
  - predicate: unfolds_through
  - object: existence_reflection_and_necessity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:115-151`

- c2. id: hegel-subject-syllogism-threshold-c2
  - subject: syllogism
  - predicate: passes_into
  - object: objectivity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `syllogism/syllogism-idea.txt:145-151`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: objectivity-threshold
  - targetWorkbook: `../object/OBJECTIVITY-WORKBOOK.md`
  - note: the next architectonic band is objectivity.
  - sourceClaimIds: [`hegel-subject-syllogism-threshold-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: pending_cross_workbook


Review outcome:

- review_pending
- notes: this threshold entry keeps the objectivity handoff explicit without pre-writing the objectivity container.
