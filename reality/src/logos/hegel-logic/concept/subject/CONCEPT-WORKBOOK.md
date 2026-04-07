# Concept Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `concept/`, not a replacement for the local idea workbook, compiler workbook, or the more granular local distillation stack.
- Read it to follow the chapter-level spine from universality, through particularity, into singularity.
- Use the local `concept/CONCEPT-IDEA-WORKBOOK.md` and part workbooks for denser local analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which the concept first articulates itself as universality, particularity, and singularity.
- Second question: what is the chapter's central operator?
  Answer: self-differentiation from within.
- Third question: where does the subfolder lead?
  Answer: to judgment.

## Authority + format lock (must persist)

- Working extraction references: `concept/concept-idea.txt` and `CONCEPT-DISTILLATION.md`
- Upstream source authority: `concept/concept-idea.txt`
- This workbook covers the `concept/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Concept side.
- Do not duplicate the detailed local workbook and distillation stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-06 (Concept-folder pass)

Scope:

- files:
  - `concept/concept-idea.txt`
  - `CONCEPT-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `concept/` subfolder.
- Preserve the older local workbook and distillation stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: universality, particularity, singularity.

### Entry hegel-subject-concept — Marker `Concept`

- sourceFiles:
  - `concept/concept-idea.txt`
- lineSpans:
  - `concept/concept-idea.txt:1-70`
- summary: The `concept/` subfolder presents the concept as such through universality, particularity, and singularity, culminating in the turn to judgment.

Key points: (KeyPoint)

- k1. The concept is treated as living self-determining totality.
- k2. The chapter unfolds through universality, particularity, and singularity.
- k3. Singularity yields judgment.

Claims: (Claim)

- c1. id: hegel-subject-concept-c1
  - subject: concept_chapter
  - predicate: unfolds_through
  - object: universality_particularity_and_singularity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concept/concept-idea.txt:1-70`

- c2. id: hegel-subject-concept-c2
  - subject: concept_chapter
  - predicate: culminates_in
  - object: judgment
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concept/concept-idea.txt:66-70`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-subject-universality-particularity
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: the chapter begins with the triadic articulation of the concept.
  - sourceClaimIds: [`hegel-subject-concept-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-subject-universality-particularity-c1`, `hegel-subject-universality-particularity-c2`]

- r2. type: transitions_to
  - targetEntryId: hegel-subject-singularity-threshold
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: singularity is the judgment threshold.
  - sourceClaimIds: [`hegel-subject-concept-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-singularity-threshold-c1`, `hegel-subject-singularity-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the chapter at the proper scale over the already strong local stack.

### Entry hegel-subject-universality-particularity — `Concept`: universality and particularity

- sourceFiles:
  - `concept/concept-idea.txt`
- lineSpans:
  - `concept/concept-idea.txt:1-64`
- summary: The concept first appears as living universality, but because this universality is totality, it determines itself into particularity from within.

Key points: (KeyPoint)

- k1. The concept is not a dead determination of the understanding.
- k2. Universality, particularity, and singularity are the concept's own moments.
- k3. Universality immediately particularizes itself.

Claims: (Claim)

- c1. id: hegel-subject-universality-particularity-c1
  - subject: concept
  - predicate: contains
  - object: universality_particularity_and_singularity_as_its_own_moments
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concept/concept-idea.txt:1-43`

- c2. id: hegel-subject-universality-particularity-c2
  - subject: universality
  - predicate: determines_itself_as
  - object: particularity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `concept/concept-idea.txt:45-64`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subject-singularity-threshold
  - targetWorkbook: `CONCEPT-WORKBOOK.md`
  - note: the triadic articulation completes itself in singularity.
  - sourceClaimIds: [`hegel-subject-universality-particularity-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-singularity-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the chapter's first movement centered on self-differentiation.

### Entry hegel-subject-singularity-threshold — `Concept`: singularity and the turn to judgment

- sourceFiles:
  - `concept/concept-idea.txt`
- lineSpans:
  - `concept/concept-idea.txt:66-70`
- summary: Singularity is the concept reflecting itself out of difference into absolute negativity, and in that movement it becomes judgment.

Key points: (KeyPoint)

- k1. Singularity is reflective return from difference.
- k2. This return is absolute negativity.
- k3. The concept becomes judgment here.

Claims: (Claim)

- c1. id: hegel-subject-singularity-threshold-c1
  - subject: singularity
  - predicate: is
  - object: concept_reflecting_itself_into_absolute_negativity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concept/concept-idea.txt:66-69`

- c2. id: hegel-subject-singularity-threshold-c2
  - subject: concept
  - predicate: becomes
  - object: judgment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concept/concept-idea.txt:69-70`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-subject-judgment
  - targetWorkbook: `JUDGMENT-WORKBOOK.md`
  - note: the next architectonic field is judgment.
  - sourceClaimIds: [`hegel-subject-singularity-threshold-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-subject-judgment-c1`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the chapter from appearing as a closed triad rather than a passage.
