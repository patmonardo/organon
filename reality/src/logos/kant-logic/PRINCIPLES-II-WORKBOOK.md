# Principles II Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the second mathematical principle: the anticipations of perception.
- In the current presentation, it should be read as the quality-side counterpart to the axioms of intuition.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-II-DISTILLATION.md`
- Upstream source authority: `principles-II.md`
- This workbook covers only the anticipations of perception.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not confuse intensive magnitude with extensive magnitude.
- Do not treat the anticipation as anticipation of empirical quality itself.
- Do not turn the anti-void argument into a positive physics thesis stronger than Kant states.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-II.md`
  - `PRINCIPLES-II-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook focused on degree, continuity, and the anti-metaphysical use of the principle.
- Preserve the contrast between sensation's empirical quality and the a priori knowability of degree.
- Keep the transition from quantity to quality explicit.

### Entry kant-trans-principles-ii — Marker `Anticipations of Perception`

- sourceFiles:
  - `principles-II.md`
  - `PRINCIPLES-II-DISTILLATION.md`
- lineSpans:
  - `principles-II.md:1-52`
  - `principles-II.md:54-146`
  - `principles-II.md:154-231`
- summary: The second family of principles establishes that the real in appearance, as object of sensation, has intensive magnitude or degree, so that while empirical quality itself is never anticipated a priori, the formal gradation of sensible reality is.

Key points: (KeyPoint)

- k1. Sensation gives the real of perception.
- k2. The real in appearance has intensive magnitude.
- k3. Degree implies continuity between reality and negation.
- k4. The principle rules against dogmatic inferences from different fillings to emptiness.

Claims: (Claim)

- c1. id: kant-trans-principles-ii-c1
  - subject: real_in_appearance
  - predicate: has
  - object: intensive_magnitude_or_degree_even_though_it_has_no_extensive_magnitude_in_apprehension
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-II.md:1-84`

- c2. id: kant-trans-principles-ii-c2
  - subject: anticipation_of_perception
  - predicate: anticipates
  - object: formal_degree_of_sensation_and_not_empirical_quality_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-II.md:29-52`
    - `principles-II.md:211-231`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-i
  - targetWorkbook: `PRINCIPLES-I-WORKBOOK.md`
  - note: after quantity is secured as extensive magnitude of intuition, the second mathematical principle treats quality as degree of sensation.
  - sourceClaimIds: [`kant-trans-principles-ii-c1`, `kant-trans-principles-ii-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-principles-i-c1`, `kant-trans-principles-i-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: once the mathematical principles fix intuition and perception, the dynamical principles turn to existence and temporal connection.
  - sourceClaimIds: [`kant-trans-principles-ii-c1`, `kant-trans-principles-ii-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes quality as degree rather than empirical content.

### Entry kant-trans-principles-ii-degree — `Anticipations`: sensation, reality, and degree

- sourceFiles:
  - `principles-II.md`
  - `PRINCIPLES-II-DISTILLATION.md`
- lineSpans:
  - `principles-II.md:1-95`
- summary: Kant argues that perception contains sensation as its real element, and this real can rise from zero to any measure, so every reality in appearance has an intensive magnitude.

Key points: (KeyPoint)

- k1. Perception is empirical consciousness with sensation.
- k2. Sensation is not itself space or time.
- k3. It can increase from 0 to any given measure.
- k4. Intensive magnitude is apprehended at once rather than part-by-part.

Claims: (Claim)

- c1. id: kant-trans-principles-ii-degree-c1
  - subject: sensation
  - predicate: has
  - object: magnitude_through_growth_of_empirical_consciousness_even_without_extensive_parts
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-II.md:1-22`
    - `principles-II.md:54-84`

- c2. id: kant-trans-principles-ii-degree-c2
  - subject: intensive_magnitude
  - predicate: is
  - object: unity_apprehended_instantaneously_with_multiplicity_represented_as_approximation_to_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-II.md:72-95`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-ii
  - targetWorkbook: `PRINCIPLES-II-WORKBOOK.md`
  - note: this entry states the core transcendental claim of the second principle family.
  - sourceClaimIds: [`kant-trans-principles-ii-degree-c1`, `kant-trans-principles-ii-degree-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ii-c1`, `kant-trans-principles-ii-c2`]

Review outcome:

- review_pending
- notes: the 0-to-degree structure is kept explicit because it carries the whole anticipation thesis.

### Entry kant-trans-principles-ii-continuity — `Anticipations`: continuity, quantum, and aggregate

- sourceFiles:
  - `principles-II.md`
  - `PRINCIPLES-II-DISTILLATION.md`
- lineSpans:
  - `principles-II.md:85-146`
- summary: The doctrine of degree yields continuity in the real of appearance and allows Kant to state that appearances are continuous magnitudes intensively as well as extensively, unlike aggregates that arise only from repeated synthesis.

Key points: (KeyPoint)

- k1. No degree of reality is absolutely smallest.
- k2. Space and time are continua.
- k3. Appearance is a quantum when synthesis is continuous.
- k4. Mere aggregates do not have that same unity.

Claims: (Claim)

- c1. id: kant-trans-principles-ii-continuity-c1
  - subject: reality_and_negation_in_appearance
  - predicate: are_connected_by
  - object: continuous_series_of_intermediate_possible_degrees
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-II.md:85-95`

- c2. id: kant-trans-principles-ii-continuity-c2
  - subject: appearances
  - predicate: are
  - object: continuous_magnitudes_extensively_in_intuition_and_intensively_in_perception
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-II.md:97-146`

Relations: (Relation)

- r1. type: supports
  - targetEntryId: kant-trans-principles-ii
  - targetWorkbook: `PRINCIPLES-II-WORKBOOK.md`
  - note: this entry shows how the second principle complements and extends the first mathematical principle.
  - sourceClaimIds: [`kant-trans-principles-ii-continuity-c1`, `kant-trans-principles-ii-continuity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ii-c1`]

Review outcome:

- review_pending
- notes: quantum versus aggregate is kept because it clarifies why continuity matters transcendentally.

### Entry kant-trans-principles-ii-void — `Anticipations`: anti-void and limits of anticipation

- sourceFiles:
  - `principles-II.md`
  - `PRINCIPLES-II-DISTILLATION.md`
- lineSpans:
  - `principles-II.md:154-231`
- summary: Kant uses degree to block experience-based proofs of empty space or empty time and to deny that differences of filling must be explained through metaphysical assumptions about voids, while still leaving empirical qualitative determination entirely to experience.

Key points: (KeyPoint)

- k1. Experience cannot prove absolute absence of the real in appearance.
- k2. Equal spaces can be filled with different intensive degrees.
- k3. The principle removes the necessity of metaphysical void hypotheses.
- k4. Empirical quality itself is never anticipated.

Claims: (Claim)

- c1. id: kant-trans-principles-ii-void-c1
  - subject: principle_of_degree
  - predicate: excludes
  - object: inference_from_experience_to_absolute_empty_space_or_empty_time
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-II.md:154-188`

- c2. id: kant-trans-principles-ii-void-c2
  - subject: understanding
  - predicate: anticipates_only
  - object: intensive_quantity_of_sensation_and_not_its_empirical_quality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-II.md:211-231`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-ii
  - targetWorkbook: `PRINCIPLES-II-WORKBOOK.md`
  - note: this entry fixes both the critical payoff and the limit of the anticipation thesis.
  - sourceClaimIds: [`kant-trans-principles-ii-void-c1`, `kant-trans-principles-ii-void-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ii-c2`]

Review outcome:

- review_pending
- notes: the chapter is kept non-dogmatic by separating transcendental permission from physical assertion.
