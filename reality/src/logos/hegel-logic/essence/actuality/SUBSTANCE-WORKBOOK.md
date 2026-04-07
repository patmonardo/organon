# Substance Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic subfolder-level workbook for `substance/`, not a replacement for the local part workbooks or the existing local distillation stack.
- Read it to follow the chapter-level spine from substantiality, through causality, into reciprocity.
- Use the local `substance/SUBSTANCE-PART-*.md` files and existing local distillations for more detailed analysis.

## Quick orientation

- First question: what is being fixed here?
  Answer: the movement in which absolute relation unfolds as substantiality, causality, and reciprocity, and thereby opens the concept.
- Second question: what is the chapter's central operator?
  Answer: necessity becoming explicit as self-mediation.
- Third question: where does the subfolder lead?
  Answer: to the concept.

## Authority + format lock (must persist)

- Working extraction references: `substance/substance-idea.txt`, `substance/relation-substantiality.txt`, `substance/relation-causality.txt`, `substance/reciprocity-action.txt`, and `SUBSTANCE-DISTILLATION.md`
- Upstream source authority: `substance/substance-idea.txt`, `substance/relation-substantiality.txt`, `substance/relation-causality.txt`, `substance/reciprocity-action.txt`
- This workbook covers the `substance/` subfolder only.

## Clean-room rules

- Keep the pass on the Hegel Essence side.
- Do not duplicate the detailed local workbook and distillation stack entry by entry.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-05 (Substance-folder pass)

Scope:

- files:
  - `substance/substance-idea.txt`
  - `substance/relation-substantiality.txt`
  - `substance/relation-causality.txt`
  - `substance/reciprocity-action.txt`
  - `SUBSTANCE-DISTILLATION.md`
- pass policy: 1 marker entry + 3 analytic entries

Decision:

- Preserve the folder-driven pattern by treating this file as the readable architectonic surface for the `substance/` subfolder.
- Preserve the older local workbook and distillation stack as detailed KG artifacts.
- Keep the emphasis on the chapter sequence: substantiality, causality, reciprocity.

### Entry hegel-actuality-substance — Marker `Substance`

- sourceFiles:
  - `substance/substance-idea.txt`
  - `substance/relation-substantiality.txt`
  - `substance/relation-causality.txt`
  - `substance/reciprocity-action.txt`
- lineSpans:
  - `substance/substance-idea.txt:1-64`
  - `substance/relation-substantiality.txt:2-180`
  - `substance/relation-causality.txt:2-780`
  - `substance/reciprocity-action.txt:2-164`
- summary: The `substance/` subfolder presents absolute relation through substantiality, causality, and reciprocity, culminating in the concept.

Key points: (KeyPoint)

- k1. Substance is the first shape of absolute relation.
- k2. The chapter unfolds through substantiality, causality, and reciprocity.
- k3. Reciprocity opens the concept.

Claims: (Claim)

- c1. id: hegel-actuality-substance-c1
  - subject: substance_chapter
  - predicate: unfolds_through
  - object: substantiality_causality_and_reciprocity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `substance/substance-idea.txt:1-64`
    - `substance/relation-substantiality.txt:2-180`
    - `substance/relation-causality.txt:2-780`
    - `substance/reciprocity-action.txt:2-164`

- c2. id: hegel-actuality-substance-c2
  - subject: substance_chapter
  - predicate: is
  - object: absolute_relation_as_truth_of_necessity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/relation-substantiality.txt:2-35`

- c3. id: hegel-actuality-substance-c3
  - subject: substance_chapter
  - predicate: culminates_in
  - object: concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/reciprocity-action.txt:132-164`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: hegel-actuality-substantiality
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: the chapter begins from substantiality.
  - sourceClaimIds: [`hegel-actuality-substance-c1`, `hegel-actuality-substance-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-actuality-substantiality-c1`, `hegel-actuality-substantiality-c2`]

- r2. type: unfolds_to
  - targetEntryId: hegel-actuality-causality
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: substantiality passes into causality.
  - sourceClaimIds: [`hegel-actuality-substance-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`hegel-actuality-causality-c1`, `hegel-actuality-causality-c2`]

- r3. type: transitions_to
  - targetEntryId: hegel-actuality-reciprocity-threshold
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: reciprocity is the concept-threshold of the chapter.
  - sourceClaimIds: [`hegel-actuality-substance-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-reciprocity-threshold-c1`, `hegel-actuality-reciprocity-threshold-c2`]

Review outcome:

- review_pending
- notes: this marker fixes the chapter-level path over a subfolder that already had strong local work.

### Entry hegel-actuality-substantiality — `Substance`: relation of substantiality

- sourceFiles:
  - `substance/relation-substantiality.txt`
- lineSpans:
  - `substance/relation-substantiality.txt:2-180`
- summary: Substantiality presents substance as the being in all being, with accidents as its shining and positedness, but this first shape still passes into causality.

Key points: (KeyPoint)

- k1. Substance is unity of being and reflection.
- k2. Accidentality is the shining of substance.
- k3. This first shape passes into causality.

Claims: (Claim)

- c1. id: hegel-actuality-substantiality-c1
  - subject: substantiality
  - predicate: is
  - object: unity_of_substance_and_accidentality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/relation-substantiality.txt:2-92`

- c2. id: hegel-actuality-substantiality-c2
  - subject: substantiality
  - predicate: passes_into
  - object: causality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/relation-substantiality.txt:149-180`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-causality
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: substantiality yields causality.
  - sourceClaimIds: [`hegel-actuality-substantiality-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`hegel-actuality-causality-c1`]

Review outcome:

- review_pending
- notes: this entry keeps substantiality tied to actuosity rather than static metaphysical stuff.

### Entry hegel-actuality-causality — `Substance`: relation of causality

- sourceFiles:
  - `substance/relation-causality.txt`
- lineSpans:
  - `substance/relation-causality.txt:2-780`
- summary: Causality develops substance as active power through formal causality, determinate causality, and action and reaction, while showing that cause and effect are only themselves in one another.

Key points: (KeyPoint)

- k1. Substance becomes cause.
- k2. Cause and effect are mutually implicative.
- k3. Finite causality undermines its own one-sidedness.

Claims: (Claim)

- c1. id: hegel-actuality-causality-c1
  - subject: causality
  - predicate: is
  - object: substance_as_active_power
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/relation-causality.txt:2-149`

- c2. id: hegel-actuality-causality-c2
  - subject: cause_and_effect
  - predicate: are
  - object: mutually_implicative_and_self_sublating
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `substance/relation-causality.txt:149-780`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: hegel-actuality-reciprocity-threshold
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: action and reaction culminate in reciprocity.
  - sourceClaimIds: [`hegel-actuality-causality-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`hegel-actuality-reciprocity-threshold-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the enormous causality file at chapter scale.

### Entry hegel-actuality-reciprocity-threshold — `Substance`: reciprocity and concept threshold

- sourceFiles:
  - `substance/reciprocity-action.txt`
- lineSpans:
  - `substance/reciprocity-action.txt:2-164`
- summary: Reciprocity sublates externally related substances and shows necessity manifesting itself as freedom; in that movement universal, particular, and singular emerge and the concept comes on the scene.

Key points: (KeyPoint)

- k1. Reciprocity sublates external mechanism.
- k2. Necessity manifests itself as freedom.
- k3. Universal, particular, and singular emerge.
- k4. The chapter turns into the concept.

Claims: (Claim)

- c1. id: hegel-actuality-reciprocity-threshold-c1
  - subject: reciprocity
  - predicate: is
  - object: sublation_of_external_causality_into_self_mediation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `substance/reciprocity-action.txt:2-84`

- c2. id: hegel-actuality-reciprocity-threshold-c2
  - subject: reciprocity
  - predicate: opens
  - object: concept
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `substance/reciprocity-action.txt:85-164`

Relations: (Relation)

- r1. type: gathers
  - targetEntryId: hegel-actuality-causality
  - targetWorkbook: `SUBSTANCE-WORKBOOK.md`
  - note: reciprocity gathers and overcomes one-sided causality.
  - sourceClaimIds: [`hegel-actuality-reciprocity-threshold-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`hegel-actuality-causality-c2`]

Review outcome:

- review_pending
- notes: this threshold entry keeps the concept handoff explicit.
