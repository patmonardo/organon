# Deduction Meta Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the transcendental-deduction problem as the question of how the categories can lawfully relate to objects through possible experience.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `DEDUCTION-META-DISTILLATION.md`
- Upstream source authority: `deduction-meta.md`
- This workbook covers only the deduction-in-general and meta-architectonic side of the transcendental deduction.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not reduce deduction to empirical concept-acquisition.
- Do not treat the object as given independently of the unity of possible experience.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (first pass)

Scope:

- files:
  - `deduction-meta.md`
  - `DEDUCTION-META-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Keep this workbook on the entitlement-question and the architectonic of experience.
- Center the three sources and the three syntheses without splitting them into separate entries.
- Leave the B-deduction apperception argument to the paired deduction-trans workbook.

### Entry kant-trans-deduction-meta — Marker `Transcendental Deduction`

- sourceFiles:
  - `deduction-meta.md`
  - `DEDUCTION-META-DISTILLATION.md`
- lineSpans:
  - `DEDUCTION-META-DISTILLATION.md:7-29`
- summary: The transcendental deduction justifies the categories by showing that they are conditions of possible experience rather than empirical products.

Key points: (KeyPoint)

- k1. Deduction asks for lawful right, not merely psychological origin.
- k2. Categories must be conditions of possible experience to have objective validity.
- k3. The deduction opposes both empirical derivation and empty rationalism.

Claims: (Claim)

- c1. id: kant-trans-deduction-meta-c1
  - subject: transcendental_deduction
  - predicate: justifies
  - object: categories_as_a_priori_conditions_of_possible_experience
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `DEDUCTION-META-DISTILLATION.md:7-29`

- c2. id: kant-trans-deduction-meta-c2
  - subject: categories
  - predicate: are_not
  - object: empirical_products_of_association_or_induction
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `DEDUCTION-META-DISTILLATION.md:7-16`
    - `DEDUCTION-META-DISTILLATION.md:55-64`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: once the categories are identified, their entitlement must be deduced.
  - sourceClaimIds: [`kant-trans-deduction-meta-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-deduction
  - targetWorkbook: `DEDUCTION-TRANS-WORKBOOK.md`
  - note: the meta-deduction opens into the apperception-centered execution of the deduction.
  - sourceClaimIds: [`kant-trans-deduction-meta-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-trans-deduction-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the lawful claim of the categories before the more explicit B-deduction sequence is unfolded.

### Entry kant-trans-deduction-meta-sources — `Experience`: synthesis, object, and affinity

- sourceFiles:
  - `deduction-meta.md`
  - `DEDUCTION-META-DISTILLATION.md`
- lineSpans:
  - `DEDUCTION-META-DISTILLATION.md:31-64`
- summary: Experience becomes possible only through the coordinated activity of sense, imagination, and apperception, whose syntheses yield objectivity and lawful affinity.

Key points: (KeyPoint)

- k1. Experience requires apprehension, reproduction, and recognition.
- k2. Objectivity arises from lawful unity under apperception.
- k3. Nature's affinity rests on transcendental rather than merely empirical connection.

Claims: (Claim)

- c1. id: kant-trans-deduction-meta-sources-c1
  - subject: experience
  - predicate: depends_on
  - object: threefold_synthesis_of_sense_imagination_and_apperception
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `DEDUCTION-META-DISTILLATION.md:31-40`

- c2. id: kant-trans-deduction-meta-sources-c2
  - subject: transcendental_apperception
  - predicate: grounds
  - object: objectivity_affinity_and_lawfulness_of_appearances
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `DEDUCTION-META-DISTILLATION.md:43-64`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-deduction-meta
  - targetWorkbook: `DEDUCTION-META-WORKBOOK.md`
  - note: the analytic entry shows how possible experience is built from synthesis into lawful objectivity.
  - sourceClaimIds: [`kant-trans-deduction-meta-sources-c1`, `kant-trans-deduction-meta-sources-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-meta-c1`]

Review outcome:

- review_pending
- notes: Locke, Hume, object = X, and affinity remain grouped here as one architectonic line.
