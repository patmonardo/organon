# Principles I Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the first mathematical principle: the axioms of intuition.
- In the current presentation, it should be read as the quantity-side opening of the four principle families.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-I-DISTILLATION.md`
- Upstream source authority: `principles-I.md`
- This workbook covers only the axioms of intuition.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not collapse extensive magnitude into a merely empirical measurement practice.
- Do not treat numerical formulas as if they were the same kind of axiom as geometry.
- Keep the applicability of mathematics tied to appearance, not to things in themselves.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-I.md`
  - `PRINCIPLES-I-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook tightly on quantity and construction.
- Preserve the difference between extensive magnitude, geometrical axioms, and numerical formulas.
- Make the mathematics-to-appearance payoff explicit, since that is the architectonic point of the chapter.

### Entry kant-trans-principles-i — Marker `Axioms of Intuition`

- sourceFiles:
  - `principles-I.md`
  - `PRINCIPLES-I-DISTILLATION.md`
- lineSpans:
  - `principles-I.md:1-15`
  - `principles-I.md:17-29`
  - `principles-I.md:31-55`
  - `principles-I.md:57-73`
- summary: The first family of principles establishes that all intuitions of appearance are extensive magnitudes, because apprehension of objects in space and time requires successive synthesis of a homogeneous manifold, and this same synthesis grounds the valid application of pure mathematics to experience.

Key points: (KeyPoint)

- k1. Appearance is apprehended only through synthesis in space and time.
- k2. Extensive magnitude is generated from parts to whole.
- k3. Geometry formulates the proper axioms of this construction.
- k4. The principle secures the applicability of mathematics to appearances.

Claims: (Claim)

- c1. id: kant-trans-principles-i-c1
  - subject: intuitions_of_appearance
  - predicate: are
  - object: extensive_magnitudes_because_their_objective_representation_requires_successive_synthesis_of_homogeneous_manifold
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-I.md:1-29`

- c2. id: kant-trans-principles-i-c2
  - subject: transcendental_principle_of_axioms_of_intuition
  - predicate: grounds
  - object: applicability_of_pure_mathematics_to_objects_of_experience_as_appearances
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-I.md:57-73`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-system-table
  - targetWorkbook: `PRINCIPLES-SYSTEM-WORKBOOK.md`
  - note: this entry instantiates the first family in the system's mathematical side.
  - sourceClaimIds: [`kant-trans-principles-i-c1`, `kant-trans-principles-i-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-system-table-c1`, `kant-trans-principles-system-table-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-ii
  - targetWorkbook: `PRINCIPLES-II-WORKBOOK.md`
  - note: after extensive magnitude on the side of intuition, the system turns to intensive magnitude on the side of sensation.
  - sourceClaimIds: [`kant-trans-principles-i-c1`, `kant-trans-principles-i-c2`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ii-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes quantity as the first mathematical principle-family.

### Entry kant-trans-principles-i-magnitude — `Axioms of Intuition`: extensive magnitude and successive synthesis

- sourceFiles:
  - `principles-I.md`
  - `PRINCIPLES-I-DISTILLATION.md`
- lineSpans:
  - `principles-I.md:1-29`
- summary: Kant defines appearances as extensive magnitudes because intuition in space and time is generated through a synthesis in which the parts must be represented prior to the whole.

Key points: (KeyPoint)

- k1. Space and time ground the form of all appearances.
- k2. Apprehension requires composition of the homogeneous manifold.
- k3. A magnitude-concept arises with consciousness of that synthetic unity.
- k4. Line and time illustrate part-priority in extensive magnitude.

Claims: (Claim)

- c1. id: kant-trans-principles-i-magnitude-c1
  - subject: extensive_magnitude
  - predicate: is_defined_by
  - object: parts_whose_representation_makes_possible_and_precedes_representation_of_the_whole
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-I.md:17-29`

- c2. id: kant-trans-principles-i-magnitude-c2
  - subject: appearances_as_intuitions
  - predicate: can_be_cognized_only_through
  - object: successive_synthesis_from_part_to_part_in_apprehension
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-I.md:1-29`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-i
  - targetWorkbook: `PRINCIPLES-I-WORKBOOK.md`
  - note: this entry fixes the quantity-form that makes the whole principle intelligible.
  - sourceClaimIds: [`kant-trans-principles-i-magnitude-c1`, `kant-trans-principles-i-magnitude-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-i-c1`]

Review outcome:

- review_pending
- notes: line/time examples are preserved because they show construction rather than mere measurement.

### Entry kant-trans-principles-i-geometry — `Axioms of Intuition`: geometry, formulas, and mathematical scope

- sourceFiles:
  - `principles-I.md`
  - `PRINCIPLES-I-DISTILLATION.md`
- lineSpans:
  - `principles-I.md:31-55`
- summary: Geometry provides proper axioms because it constructs outer intuition a priori, whereas arithmetic propositions are synthetic singular formulas rather than universal axioms in the same sense.

Key points: (KeyPoint)

- k1. Geometry arises from productive imagination in the generation of figures.
- k2. Geometrical axioms concern quanta as such.
- k3. Numerical propositions are synthetic but singular.
- k4. Arithmetic should not be folded into axioms proper without distinction.

Claims: (Claim)

- c1. id: kant-trans-principles-i-geometry-c1
  - subject: geometry
  - predicate: expresses
  - object: a_priori_conditions_of_sensible_construction_under_which_outer_appearance_can_be_schematized
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-I.md:31-36`

- c2. id: kant-trans-principles-i-geometry-c2
  - subject: numerical_propositions
  - predicate: are
  - object: synthetic_singular_formulas_and_not_proper_axioms_in_the_geometrical_sense
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `principles-I.md:38-55`

Relations: (Relation)

- r1. type: supports
  - targetEntryId: kant-trans-principles-i
  - targetWorkbook: `PRINCIPLES-I-WORKBOOK.md`
  - note: the chapter's mathematical claims depend on distinguishing constructional geometry from mere numerical formula.
  - sourceClaimIds: [`kant-trans-principles-i-geometry-c1`, `kant-trans-principles-i-geometry-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-i-c1`, `kant-trans-principles-i-c2`]

Review outcome:

- review_pending
- notes: the geometry/arithmetic distinction is kept because it fixes Kant's precise use of axiom.

### Entry kant-trans-principles-i-application — `Axioms of Intuition`: mathematics valid for appearances

- sourceFiles:
  - `principles-I.md`
  - `PRINCIPLES-I-DISTILLATION.md`
- lineSpans:
  - `principles-I.md:57-73`
- summary: The principle justifies the objective application of pure mathematics to experience because appearances are given only through pure intuition and are not things in themselves.

Key points: (KeyPoint)

- k1. Appearances are not things in themselves.
- k2. Empirical intuition is possible only through pure intuition of space and time.
- k3. What geometry proves of space and time is valid for appearances.
- k4. Attempts to exempt appearances from those rules dissolve objective validity itself.

Claims: (Claim)

- c1. id: kant-trans-principles-i-application-c1
  - subject: pure_mathematics
  - predicate: applies_to
  - object: objects_of_experience_because_appearances_are_given_only_under_pure_intuition_of_space_and_time
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-I.md:57-73`

- c2. id: kant-trans-principles-i-application-c2
  - subject: denial_of_geometrical_validity_for_appearances
  - predicate: collapses
  - object: objective_validity_of_space_and_with_it_application_of_mathematics_to_experience
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-I.md:63-73`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-i
  - targetWorkbook: `PRINCIPLES-I-WORKBOOK.md`
  - note: this entry states the critical payoff of the entire first principle family.
  - sourceClaimIds: [`kant-trans-principles-i-application-c1`, `kant-trans-principles-i-application-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-i-c2`]

Review outcome:

- review_pending
- notes: the anti-things-in-themselves boundary is kept explicit because it is the condition of mathematical application.
