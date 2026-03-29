# Principles Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the introductory and schematism side of the Analytic of Principles.
- In the current presentation, it should be read as the first judgment-side continuation of the deduction sequence.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-DISTILLATION.md`
- Upstream source authority: `principles.md`
- This workbook covers only the introductory and schematism material of the Analytic of Principles.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not turn the schemata into images or empirical habits.
- Do not extend the categories beyond the sensible conditions that schematism supplies.
- Do not confuse the categories' empirical significance with any supposed transcendental use once sensibility is removed.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook focused on the power of judgment and schematism rather than the later system of principles.
- Preserve the double role of schematism as both realization and restriction of the categories.
- Use empirical-significance and no-transcendental-use language where the chapter states the boundary sharply.
- Use the judgment-side framing explicitly, since this book is the transition from categories to valid application.

### Entry kant-trans-principles — Marker `Analytic of Principles`

- sourceFiles:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `principles.md:7-34`
  - `principles.md:40-98`
  - `principles.md:117-156`
  - `principles.md:198-278`
- summary: The Analytic of Principles is the canon of the transcendental power of judgment, and its first task is to show through schematism how pure concepts of understanding can be validly applied to appearances in possible experience.

Key points: (KeyPoint)

- k1. The analytic side of transcendental logic now centers on judgment rather than reason.
- k2. The power of judgment subsumes appearances under pure rules.
- k3. Schematism provides the mediating condition of application.
- k4. Schemata both give the categories empirical significance and limit them to possible experience.

Claims: (Claim)

- c1. id: kant-trans-principles-c1
  - subject: analytic_of_principles
  - predicate: functions_as
  - object: canon_of_transcendental_power_of_judgment_for_applying_categories_to_appearances
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles.md:19-34`
    - `principles.md:93-98`

- c2. id: kant-trans-principles-c2
  - subject: schematism
  - predicate: mediates_and_limits
  - object: application_of_categories_to_appearances_through_a_priori_time_determinations_that_yield_empirical_significance_only_within_possible_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles.md:126-139`
    - `principles.md:239-278`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-deduction
  - targetWorkbook: `DEDUCTION-TRANS-WORKBOOK.md`
  - note: once the categories are shown valid for possible experience, the next task is to explain how judgment can apply them.
  - sourceClaimIds: [`kant-trans-principles-c1`, `kant-trans-principles-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-deduction-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-system
  - targetWorkbook: `PRINCIPLES-SYSTEM-WORKBOOK.md`
  - note: once schematism has made categorical application possible, the systematic principles of pure understanding can be unfolded.
  - sourceClaimIds: [`kant-trans-principles-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-system-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the book-level shift from deduction to judgment-side application.

### Entry kant-trans-principles-judgment — `Analytic of Principles`: judgment as canon of application

- sourceFiles:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `principles.md:7-98`
- summary: Kant defines the Analytic of Principles as a doctrine of the power of judgment, because transcendental logic must secure the valid subsumption of appearances under pure concepts.

Key points: (KeyPoint)

- k1. Transcendental logic cannot give reason a place in its analytic side.
- k2. The power of judgment is the faculty of subsuming under rules.
- k3. Judgment cannot be mechanically taught by further rules.
- k4. Transcendental critique can indicate the a priori case of application.

Claims: (Claim)

- c1. id: kant-trans-principles-judgment-c1
  - subject: transcendental_logic
  - predicate: restricts_its_analytic_side_to
  - object: understanding_and_power_of_judgment_because_reason_in_transcendental_use_is_dialectical
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles.md:19-34`

- c2. id: kant-trans-principles-judgment-c2
  - subject: transcendental_power_of_judgment
  - predicate: needs
  - object: critical_guidance_for_subsuming_appearances_under_pure_concepts_of_understanding
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles.md:40-98`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: this entry fixes why the second book belongs to judgment and not to a mere continuation of concept-analysis.
  - sourceClaimIds: [`kant-trans-principles-judgment-c1`, `kant-trans-principles-judgment-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-c1`]

Review outcome:

- review_pending
- notes: the negative-critical role of philosophy is kept explicit because it governs the tone of the whole book.

### Entry kant-trans-principles-schema — `Schematism`: mediation between category and appearance

- sourceFiles:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `principles.md:108-193`
- summary: Schematism solves the heterogeneity between pure concepts and sensible appearances by introducing the schema as a pure, sensible-intellectual mediating rule of imagination.

Key points: (KeyPoint)

- k1. Pure concepts and appearances are otherwise heterogeneous.
- k2. A third mediating representation is therefore necessary.
- k3. The schema is a rule of imagination rather than an image.
- k4. Time makes the schema homogeneous with both category and appearance.

Claims: (Claim)

- c1. id: kant-trans-principles-schema-c1
  - subject: transcendental_schema
  - predicate: makes_possible
  - object: subsumption_of_appearances_under_categories_by_mediating_between_intellectual_and_sensible_sides
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles.md:117-139`

- c2. id: kant-trans-principles-schema-c2
  - subject: schema
  - predicate: is
  - object: rule_governed_product_of_imagination_and_not_a_particular_image
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles.md:154-193`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: this entry states the central mediating device by which judgment can apply the categories.
  - sourceClaimIds: [`kant-trans-principles-schema-c1`, `kant-trans-principles-schema-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-c2`]

Review outcome:

- review_pending
- notes: schema versus image is kept explicit because it is the chapter's main anti-misreading boundary.

### Entry kant-trans-principles-time — `Schematism`: time-determinations of the categories

- sourceFiles:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `principles.md:198-257`
- summary: Each category receives its schema as a specific a priori time-determination, so that quantity, quality, relation, and modality become rule-governed ways of determining appearances in time.

Key points: (KeyPoint)

- k1. Quantity is schematized as number.
- k2. Quality is schematized as the filling of time.
- k3. Relation is schematized as persistence, succession according to rule, and simultaneity according to rule.
- k4. Modality is schematized through relation to time as possible, actual, and necessary.

Claims: (Claim)

- c1. id: kant-trans-principles-time-c1
  - subject: schemata_of_categories
  - predicate: are
  - object: a_priori_time_determinations_according_to_rules
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles.md:198-248`

- c2. id: kant-trans-principles-time-c2
  - subject: schematism_of_understanding
  - predicate: comes_down_to
  - object: unity_of_manifold_of_intuition_in_inner_sense_and_indirectly_unity_of_apperception
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `principles.md:250-257`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-principles-boundary
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: once the schemas are shown as time-determinations, their restrictive function for the categories can be stated explicitly.
  - sourceClaimIds: [`kant-trans-principles-time-c1`, `kant-trans-principles-time-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-boundary-c1`, `kant-trans-principles-boundary-c2`]

Review outcome:

- review_pending
- notes: the temporal structure is kept explicit because it is the actual form of schematism rather than an optional illustration.

### Entry kant-trans-principles-boundary — `Schematism`: realization, empirical significance, and restriction

- sourceFiles:
  - `principles.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `principles.md:259-278`
- summary: Schematism simultaneously gives the categories empirical significance and restricts them to the sensible conditions under which alone objects can be given to us in possible experience.

Key points: (KeyPoint)

- k1. Schemata first realize the categories.
- k2. The same sensibility that realizes them also limits them.
- k3. Without schemata the categories retain only logical form, not determinate objective use.

Claims: (Claim)

- c1. id: kant-trans-principles-boundary-c1
  - subject: schema
  - predicate: restricts
  - object: categories_to_conditions_of_sensibility_even_as_it_realizes_them
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles.md:262-268`

- c2. id: kant-trans-principles-boundary-c2
  - subject: categories_without_schemata
  - predicate: retain_only
  - object: logical_form_of_unity_without_any_determinate_objective_use
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles.md:270-278`

Relations: (Relation)

- r1. type: bounds
  - targetEntryId: kant-trans-principles
  - targetWorkbook: `PRINCIPLES-WORKBOOK.md`
  - note: this entry fixes the critical limit that prevents schematized categories from being mistaken for concepts of things in themselves.
  - sourceClaimIds: [`kant-trans-principles-boundary-c1`, `kant-trans-principles-boundary-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-principles-c2`]

Review outcome:

- review_pending
- notes: the final restriction is held separately because it states the shift from empirical significance to no transcendental use.
