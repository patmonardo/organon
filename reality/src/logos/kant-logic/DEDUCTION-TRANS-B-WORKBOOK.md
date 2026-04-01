# Deduction Trans B Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the B-deduction proper around original synthetic unity of apperception, categories, imagination, and the lawful form of experience.
- In the current presentation, it should be read as the execution of the entitlement-and-possible-experience route established in the deduction-trans-a pass.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `deduction-trans-b-DISTILLATION.md`
- Upstream source authority: `deduction-trans-b.md`
- This workbook covers only the apperception-centered and execution-stage side of the transcendental deduction.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not treat apperception as empirical psychology.
- Do not extend the categories beyond possible experience.
- Do not absorb the A-side lawful-entitlement problem into a mere preface; this workbook presupposes that route and executes it.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (upgrade pass)

Scope:

- files:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook centered on the canonical B-presentation of original synthetic unity of apperception as the supreme principle, and unfold the full B-deduction sequence.
- Split the execution into combination, apperception, judgment and category, experience-bound use, figurative synthesis and inner sense, and the final legislation of nature.
- Preserve the experience-bound limit of the categories as a standing critical boundary.

### Entry kant-trans-b-deduction — Marker `Original Synthetic Unity of Apperception`

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:19-49`
  - `deduction-trans-b.md:57-110`
  - `deduction-trans-b.md:118-149`
  - `deduction-trans-b.md:207-241`
  - `deduction-trans-b.md:308-316`
  - `deduction-trans-b.md:363-370`
  - `deduction-trans-b.md:538-541`
  - `deduction-trans-b.md:611-628`
- summary: The B-deduction grounds all use of the understanding in the original synthetic unity of apperception and shows that categories have objective reality only as conditions of possible experience.

Key points: (KeyPoint)

- k1. Combination is an act of spontaneity rather than sense.
- k2. Original synthetic unity of apperception is the supreme principle of understanding.
- k3. Categories determine sensible intuition only under the condition of objective unity.
- k4. Their legitimate use extends only to possible experience.

Claims: (Claim)

- c1. id: kant-trans-b-deduction-c1
  - subject: original_synthetic_unity_of_apperception
  - predicate: grounds
  - object: supreme_principle_of_all_use_of_understanding_and_objective_validity_of_cognition
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `deduction-trans-b.md:57-110`
    - `deduction-trans-b.md:118-149`

- c2. id: kant-trans-b-deduction-c2
  - subject: categories
  - predicate: determine
  - object: manifold_of_sensible_intuition_as_objects_of_possible_experience_only
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:231-241`
    - `deduction-trans-b.md:311-316`
    - `deduction-trans-b.md:363-370`
    - `deduction-trans-b.md:538-541`
    - `deduction-trans-b.md:611-628`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: the B-deduction executes the lawful claim and possible-experience route that deduction-trans-a sets up.
  - sourceClaimIds: [`kant-trans-b-deduction-c1`, `kant-trans-b-deduction-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`, `kant-trans-deduction-trans-a-c2`]

Review outcome:

- review_pending
- notes: marker entry fixes the apperception-centered core of the deduction and the experience-bound validity of the categories.

### Entry kant-trans-b-combination — `Deduction`: combination and higher unity

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:19-49`
- summary: The deduction begins by arguing that combination is never given by sense, but is an act of spontaneity that already presupposes a higher unity than the category of unity.

Key points: (KeyPoint)

- k1. Sensibility gives a manifold but not its conjunction.
- k2. Combination is an act of self-activity.
- k3. The unity of combination must be sought above the categories.

Claims: (Claim)

- c1. id: kant-trans-b-combination-c1
  - subject: combination_of_manifold
  - predicate: is
  - object: action_of_understanding_and_not_something_given_through_the_senses
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `deduction-trans-b.md:19-31`

- c2. id: kant-trans-b-combination-c2
  - subject: unity_of_combination
  - predicate: must_be_sought_in
  - object: higher_ground_of_unity_than_category_of_unity_already_presupposes
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `deduction-trans-b.md:38-49`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-b-apperception
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: the search for the source of unity leads directly to original apperception.
  - sourceClaimIds: [`kant-trans-b-combination-c1`, `kant-trans-b-combination-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-b-apperception-c1`, `kant-trans-b-apperception-c2`]

Review outcome:

- review_pending
- notes: this entry preserves the opening shift from sensible givenness to spontaneous synthesis.

### Entry kant-trans-b-apperception — `Deduction`: original synthetic unity

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:57-110`
  - `deduction-trans-b.md:118-189`
- summary: The I think must be able to accompany all representations, and this original synthetic unity of apperception is the supreme condition under which any manifold can belong to one cognition and one object.

Key points: (KeyPoint)

- k1. The I think must be able to accompany all representations.
- k2. Analytical unity of apperception presupposes synthetic unity.
- k3. Objective unity differs from merely empirical unity of consciousness.
- k4. Original synthetic unity of apperception is the supreme principle of understanding.

Claims: (Claim)

- c1. id: kant-trans-b-apperception-c1
  - subject: pure_apperception
  - predicate: requires
  - object: synthetic_unity_of_manifold_as_condition_of_identity_of_consciousness
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `deduction-trans-b.md:57-110`

- c2. id: kant-trans-b-apperception-c2
  - subject: transcendental_unity_of_apperception
  - predicate: is
  - object: objective_and_supreme_condition_of_all_cognition_for_human_understanding
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:118-149`
    - `deduction-trans-b.md:174-189`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-b-deduction
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: this entry states the central principle from which the rest of the B-deduction proceeds.
  - sourceClaimIds: [`kant-trans-b-apperception-c1`, `kant-trans-b-apperception-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-b-deduction-c1`]

Review outcome:

- review_pending
- notes: apperception is kept rigorously transcendental here, not psychological.

### Entry kant-trans-b-judgment — `Deduction`: judgment, objective unity, and categories

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:197-279`
- summary: Judgment is the way cognitions are brought to objective unity of apperception, and the categories are those same judging functions insofar as they determine the manifold of intuition.

Key points: (KeyPoint)

- k1. Judgment brings cognitions to objective unity of apperception.
- k2. The copula marks objective rather than merely associative unity.
- k3. Categories are functions for judging insofar as they determine intuition.
- k4. The categories are rules only for a discursive understanding given intuition from elsewhere.

Claims: (Claim)

- c1. id: kant-trans-b-judgment-c1
  - subject: judgment
  - predicate: is
  - object: way_of_bringing_given_cognitions_to_objective_unity_of_apperception
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:197-223`

- c2. id: kant-trans-b-judgment-c2
  - subject: categories
  - predicate: are
  - object: same_functions_for_judging_insofar_as_they_determine_manifold_of_given_intuition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:231-241`
    - `deduction-trans-b.md:249-279`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-b-boundary
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: once categories are identified as rules of objective unity, Kant can delimit their valid use.
  - sourceClaimIds: [`kant-trans-b-judgment-c1`, `kant-trans-b-judgment-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-b-boundary-c1`, `kant-trans-b-boundary-c2`]

Review outcome:

- review_pending
- notes: this entry keeps judgment and category together because the B-deduction explicitly joins them at the point of objective validity.

### Entry kant-trans-b-boundary — `Deduction`: experience-bound use of the categories

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:287-347`
- summary: Categories yield cognition only when joined to sensible intuition and therefore have no legitimate cognitive use beyond objects of possible experience.

Key points: (KeyPoint)

- k1. To think an object and to cognize it are not the same.
- k2. Cognition requires both concept and intuition.
- k3. For us, categories have sense only in relation to empirical intuition.
- k4. Beyond experience they remain empty forms of thought.

Claims: (Claim)

- c1. id: kant-trans-b-boundary-c1
  - subject: cognition_of_object
  - predicate: requires
  - object: category_plus_corresponding_intuition_and_not_mere_thought_of_object_in_general
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:287-306`

- c2. id: kant-trans-b-boundary-c2
  - subject: categories
  - predicate: have_no_cognitive_use_beyond
  - object: objects_of_possible_experience_given_in_sensible_intuition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:308-347`

Relations: (Relation)

- r1. type: bounds
  - targetEntryId: kant-trans-b-deduction
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: this entry fixes the decisive critical limit that accompanies the deduction's positive result.
  - sourceClaimIds: [`kant-trans-b-boundary-c1`, `kant-trans-b-boundary-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-b-deduction-c2`]

Review outcome:

- review_pending
- notes: the boundary is kept separate so the positive validity of the categories is never read as transcendent license.

### Entry kant-trans-b-figurative — `Deduction`: figurative synthesis, imagination, and inner sense

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:356-499`
- summary: Categories acquire objective reality in relation to appearances through figurative synthesis, the productive imagination, and the understanding's determination of inner sense, which still yields only appearance rather than intellectual self-knowledge.

Key points: (KeyPoint)

- k1. Categories become objectively real only for appearances that can be given to us.
- k2. Figurative synthesis differs from merely intellectual synthesis.
- k3. Productive imagination is the first application of understanding to sensibility.
- k4. Inner self-intuition remains appearance-bound.

Claims: (Claim)

- c1. id: kant-trans-b-figurative-c1
  - subject: transcendental_synthesis_of_imagination
  - predicate: mediates
  - object: objective_reality_of_categories_for_sensible_appearances
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-b.md:356-398`

- c2. id: kant-trans-b-figurative-c2
  - subject: self_consciousness
  - predicate: does_not_yield
  - object: intellectual_self_cognition_but_only_appearance_through_inner_sense
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `deduction-trans-b.md:409-467`
    - `deduction-trans-b.md:475-499`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-b-deduction
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: this entry shows how apperception reaches sensibility without collapsing into intellectual intuition.
  - sourceClaimIds: [`kant-trans-b-figurative-c1`, `kant-trans-b-figurative-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-b-deduction-c1`, `kant-trans-b-deduction-c2`]

Review outcome:

- review_pending
- notes: the paradox of inner sense and the productive imagination are kept together because Kant uses them to explain one and the same application-problem.

### Entry kant-trans-b-result — `Deduction`: nature, result, and the brief concept

- sourceFiles:
  - `deduction-trans-b.md`
  - `deduction-trans-b-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-b.md:508-658`
- summary: In perception and nature alike, the categories function as a priori conditions of connected experience, so that no a priori cognition is possible for us except of possible experience, whose lawfulness the understanding makes possible from its own side.

Key points: (KeyPoint)

- k1. Perception itself stands under categorical synthesis.
- k2. Categories prescribe laws a priori to appearances as nature in general.
- k3. Experience is possible because categories make it possible, not the reverse.
- k4. The deduction rejects both empirical origin and preformationist compromise.

Claims: (Claim)

- c1. id: kant-trans-b-result-c1
  - subject: categories
  - predicate: prescribe
  - object: a_priori_laws_to_appearances_as_nature_in_general_from_side_of_understanding
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:522-541`
    - `deduction-trans-b.md:571-603`

- c2. id: kant-trans-b-result-c2
  - subject: deduction_result
  - predicate: states
  - object: only_objects_of_possible_experience_are_available_for_our_a_priori_cognition_because_categories_ground_possibility_of_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-b.md:611-628`
    - `deduction-trans-b.md:631-658`

Relations: (Relation)

- r1. type: bounds
  - targetEntryId: kant-trans-b-deduction
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: the closing result turns the deduction into a complete account of experience-bound lawfulness.
  - sourceClaimIds: [`kant-trans-b-result-c1`, `kant-trans-b-result-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-b-deduction-c2`]

Review outcome:

- review_pending
- notes: the forward move into principles is implicit here, but no workbook cross-link is added until that artifact exists.
