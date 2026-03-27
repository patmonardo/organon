# Deduction Trans Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the B-deduction around apperception, categories, imagination, and the lawful form of experience.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `DEDUCTION-TRANS-DISTILLATION.md`
- Upstream source authority: `deduction-trans.md`
- This workbook covers only the apperception-centered and execution-stage side of the transcendental deduction.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not treat apperception as empirical psychology.
- Do not extend the categories beyond possible experience.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (first pass)

Scope:

- files:
  - `deduction-trans.md`
  - `DEDUCTION-TRANS-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Keep this workbook centered on apperception as the supreme principle.
- Hold judgment, category, imagination, and nature together as one executed deduction.
- Preserve the experience-bound limit of the categories.

### Entry kant-trans-deduction — Marker `Original Synthetic Unity of Apperception`

- sourceFiles:
  - `deduction-trans.md`
  - `DEDUCTION-TRANS-DISTILLATION.md`
- lineSpans:
  - `DEDUCTION-TRANS-DISTILLATION.md:7-42`
- summary: The B-deduction grounds all use of the understanding in the original synthetic unity of apperception and shows that categories determine the manifold only as objects of possible experience.

Key points: (KeyPoint)

- k1. Combination is an act of spontaneity rather than sense.
- k2. Original synthetic unity of apperception is the supreme principle of understanding.
- k3. Categories determine sensible intuition only under the condition of objective unity.

Claims: (Claim)

- c1. id: kant-trans-deduction-c1
  - subject: original_synthetic_unity_of_apperception
  - predicate: grounds
  - object: supreme_principle_of_all_use_of_understanding
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `DEDUCTION-TRANS-DISTILLATION.md:19-29`

- c2. id: kant-trans-deduction-c2
  - subject: categories
  - predicate: determine
  - object: manifold_of_sensible_intuition_as_objects_of_possible_experience
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `DEDUCTION-TRANS-DISTILLATION.md:31-42`
    - `DEDUCTION-TRANS-DISTILLATION.md:44-54`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-deduction-meta
  - targetWorkbook: `DEDUCTION-META-WORKBOOK.md`
  - note: the B-deduction executes the lawful claim that the meta-deduction sets up.
  - sourceClaimIds: [`kant-trans-deduction-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-trans-deduction-meta-c1`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: once the categories are shown valid for experience, the next task is their schematized use in principles.
  - sourceClaimIds: [`kant-trans-deduction-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-trans-principles-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the apperception-centered core of the deduction before the power-of-judgment material takes over.

### Entry kant-trans-deduction-application — `Categories`: imagination, experience, and nature

- sourceFiles:
  - `deduction-trans.md`
  - `DEDUCTION-TRANS-DISTILLATION.md`
- lineSpans:
  - `DEDUCTION-TRANS-DISTILLATION.md:44-68`
- summary: The execution of the deduction shows that categories acquire objective reality only through figurative synthesis and that they legislate the lawful form of experience.

Key points: (KeyPoint)

- k1. Imagination mediates between understanding and sensibility.
- k2. Categories are empty beyond possible experience.
- k3. Nature's lawful form arises from the categories as conditions of experience.

Claims: (Claim)

- c1. id: kant-trans-deduction-application-c1
  - subject: transcendental_imagination
  - predicate: mediates
  - object: application_of_categories_to_appearances
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `DEDUCTION-TRANS-DISTILLATION.md:44-54`

- c2. id: kant-trans-deduction-application-c2
  - subject: categories
  - predicate: legislate
  - object: lawful_form_of_nature_as_possible_experience
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `DEDUCTION-TRANS-DISTILLATION.md:56-68`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-deduction
  - targetWorkbook: `DEDUCTION-TRANS-WORKBOOK.md`
  - note: the analytic entry shows how apperception becomes experience through imagination and lawful synthesis.
  - sourceClaimIds: [`kant-trans-deduction-application-c1`, `kant-trans-deduction-application-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-c1`, `kant-trans-deduction-c2`]

Review outcome:

- review_pending
- notes: figurative synthesis, inner sense, and the legislation of nature remain grouped here at seed scale.
