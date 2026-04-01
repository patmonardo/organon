# Deduction Trans A Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the transcendental-deduction problem as the question of how the categories can lawfully relate to objects through possible experience.
- In the current presentation, it should be read as the entitlement, possible-experience, and synthesis-side architectonic of the deduction before the explicit B-deduction execution is unfolded downstream.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `deduction-trans-a-DISTILLATION.md`
- Upstream source authority: `deduction-trans-a.md`
- This workbook covers only the deduction-in-general, possible-experience, and preparatory synthesis/objectivity side of the transcendental deduction.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not reduce deduction to empirical concept-acquisition.
- Do not treat the object as given independently of the unity of possible experience.
- Do not let the explicit B-deduction presentation of original synthetic unity of apperception swallow the A-side problem-form.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (upgrade pass)

Scope:

- files:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep this workbook on the entitlement-question and the architectonic of possible experience.
- Split the argument into lawful entitlement, possible experience, threefold synthesis, objectivity, and affinity.
- Keep apperception here only insofar as it is needed to prepare the objectivity problem; reserve its explicit supreme-principle presentation for deduction-trans-b.
- Leave the fuller execution of the deduction to the paired deduction-trans-b workbook.

### Entry kant-trans-deduction-trans-a — Marker `Transcendental Deduction`

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:11-56`
  - `deduction-trans-a.md:71-87`
  - `deduction-trans-a.md:151-177`
  - `deduction-trans-a.md:183-223`
  - `deduction-trans-a.md:431-485`
  - `deduction-trans-a.md:504-521`
  - `deduction-trans-a.md:576-636`
- summary: The transcendental deduction justifies the categories by showing that they are a priori conditions of possible experience and of the lawful unity through which objects can be cognized.

Key points: (KeyPoint)

- k1. Deduction asks for lawful right, not merely psychological origin.
- k2. Categories must be conditions of possible experience to have objective validity.
- k3. Experience requires unity of synthesis under apperception.
- k4. Categories are not empirical habits but formal and synthetic principles of experience.

Claims: (Claim)

- c1. id: kant-trans-deduction-trans-a-c1
  - subject: transcendental_deduction
  - predicate: justifies
  - object: categories_as_a_priori_conditions_of_possible_experience
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `deduction-trans-a.md:28-30`
    - `deduction-trans-a.md:165-177`
    - `deduction-trans-a.md:504-508`

- c2. id: kant-trans-deduction-trans-a-c2
  - subject: categories
  - predicate: are_not
  - object: empirical_products_of_association_or_induction_but_conditions_of_lawful_experience
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:52-56`
    - `deduction-trans-a.md:523-550`
    - `deduction-trans-a.md:635-636`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: once the categories are identified, their entitlement must be deduced.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-trans-concepts-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-b-deduction
  - targetWorkbook: `deduction-trans-b-WORKBOOK.md`
  - note: the trans-a pass opens into the more explicit execution of the deduction through apperception and synthesis.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-b-deduction-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the lawful claim of the categories before the more explicit deduction sequence is unfolded.

### Entry kant-trans-deduction-entitlement — `Deduction`: lawful title versus empirical origin

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:11-56`
- summary: Kant distinguishes a transcendental deduction from empirical concept-genealogy and argues that pure a priori concepts need a lawful title to objective use.

Key points: (KeyPoint)

- k1. Deduction concerns quid juris rather than quid facti.
- k2. Empirical concepts can rely on experience for proof of objective reality.
- k3. Pure a priori concepts require a transcendental birth certificate.

Claims: (Claim)

- c1. id: kant-trans-deduction-entitlement-c1
  - subject: transcendental_deduction
  - predicate: explains
  - object: way_in_which_pure_concepts_can_relate_to_objects_a_priori
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `deduction-trans-a.md:11-30`

- c2. id: kant-trans-deduction-entitlement-c2
  - subject: empirical_derivation_of_pure_concepts
  - predicate: cannot_supply
  - object: lawful_justification_of_their_future_a_priori_use
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:39-56`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: this entry fixes the juridical form of the problem before the argument turns to possible experience.
  - sourceClaimIds: [`kant-trans-deduction-entitlement-c1`, `kant-trans-deduction-entitlement-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`]

Review outcome:

- review_pending
- notes: the entitlement question is held distinct from origin so the whole chapter does not collapse into psychology.

### Entry kant-trans-deduction-necessity — `Deduction`: why the categories require proof

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:58-134`
- summary: Categories require a special deduction because, unlike space and time, they do not give objects in intuition, and concepts such as cause cannot be grounded in empirical regularity alone.

Key points: (KeyPoint)

- k1. The deduction is unavoidable but difficult.
- k2. Categories do not provide conditions under which objects are given in intuition.
- k3. Cause cannot be derived from custom because necessity is not empirical.

Claims: (Claim)

- c1. id: kant-trans-deduction-necessity-c1
  - subject: pure_concepts_of_understanding
  - predicate: require
  - object: special_transcendental_deduction_because_their_objective_validity_is_not_immediately_evident
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:71-87`
    - `deduction-trans-a.md:94-111`

- c2. id: kant-trans-deduction-necessity-c2
  - subject: concept_of_cause
  - predicate: cannot_arise_from
  - object: empirical_regularities_because_it_contains_necessity_and_strict_universality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:102-134`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-deduction-trans-a-experience
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: the difficulty of the categories explains why the argument must turn to conditions of possible experience.
  - sourceClaimIds: [`kant-trans-deduction-necessity-c1`, `kant-trans-deduction-necessity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-experience-c1`, `kant-trans-deduction-trans-a-experience-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the special burden of proof for the categories visible before the positive deduction begins.

### Entry kant-trans-deduction-trans-a-experience — `Deduction`: possible experience as principle

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:136-177`
  - `deduction-trans-a.md:244-279`
- summary: The transcendental deduction must show that the categories are a priori conditions of possible experience, since objects of experience require both intuition and concept.

Key points: (KeyPoint)

- k1. Representation and object can meet either empirically or through a priori conditions of cognition.
- k2. Every object of experience requires intuition and concept together.
- k3. Categories have objective validity only insofar as they make experience thinkable.

Claims: (Claim)

- c1. id: kant-trans-deduction-trans-a-experience-c1
  - subject: categories
  - predicate: have_objective_validity_if_and_only_if
  - object: they_are_conditions_under_which_objects_of_experience_can_be_thought
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:151-177`

- c2. id: kant-trans-deduction-trans-a-experience-c2
  - subject: pure_concepts_of_understanding
  - predicate: must_contain
  - object: formal_and_objective_conditions_of_possible_experience_rather_than_empirical_content
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:244-279`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: this entry states the governing principle toward which the entire deduction is directed.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-experience-c1`, `kant-trans-deduction-trans-a-experience-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`]

Review outcome:

- review_pending
- notes: possible experience is kept as the central criterion because everything else in the deduction unfolds from it.

### Entry kant-trans-deduction-trans-a-syntheses — `Experience`: three sources and three syntheses

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:183-223`
  - `deduction-trans-a.md:280-369`
- summary: Experience depends on the coordinated activity of sense, imagination, and apperception, which unfold as synopsis and the three syntheses of apprehension, reproduction, and recognition.

Key points: (KeyPoint)

- k1. Sense, imagination, and apperception are original sources of experience.
- k2. Apprehension and reproduction are necessary even for pure intuitions.
- k3. Experience requires synthesis rather than isolated impressions.

Claims: (Claim)

- c1. id: kant-trans-deduction-trans-a-syntheses-c1
  - subject: possibility_of_experience
  - predicate: depends_on
  - object: three_original_sources_of_sense_imagination_and_apperception
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:183-197`

- c2. id: kant-trans-deduction-trans-a-syntheses-c2
  - subject: cognition
  - predicate: requires
  - object: synthesis_of_apprehension_reproduction_and_recognition_of_the_manifold
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:280-369`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-deduction-trans-a-objectivity
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: the syntheses make possible the later account of objectivity and apperception.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-syntheses-c1`, `kant-trans-deduction-trans-a-syntheses-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-objectivity-c1`, `kant-trans-deduction-trans-a-objectivity-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the transcendental side of synthesis in view without preempting the later deduction-trans-b treatment.

### Entry kant-trans-deduction-trans-a-objectivity — `Deduction`: object, apperception, and unity

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:370-487`
- summary: The object of cognition is the necessary unity of the manifold under rules, and this objectivity rests on transcendental apperception rather than on empirical self-consciousness.

Key points: (KeyPoint)

- k1. Recognition in the concept requires identity of consciousness.
- k2. The object = X names the correlate of necessary unity, not a separately intuited thing.
- k3. Transcendental apperception grounds the objective unity of cognition.

Claims: (Claim)

- c1. id: kant-trans-deduction-trans-a-objectivity-c1
  - subject: object_of_cognition
  - predicate: is_thought_as
  - object: necessary_unity_of_the_manifold_under_rules_of_synthesis
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:390-430`

- c2. id: kant-trans-deduction-trans-a-objectivity-c2
  - subject: transcendental_apperception
  - predicate: grounds
  - object: necessary_unity_of_consciousness_required_for_all_objects_of_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:431-487`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: this entry states the chapter's decisive move from synthesis to objectivity.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-objectivity-c1`, `kant-trans-deduction-trans-a-objectivity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`]

Review outcome:

- review_pending
- notes: the object is held here as a unity-condition so the deduction does not relapse into naïve object-givenness.

### Entry kant-trans-deduction-trans-a-affinity — `Deduction`: affinity, nature, and categories

- sourceFiles:
  - `deduction-trans-a.md`
  - `deduction-trans-a-DISTILLATION.md`
- lineSpans:
  - `deduction-trans-a.md:488-565`
  - `deduction-trans-a.md:576-700`
- summary: The affinity of appearances, the unity of experience, and the lawfulness of nature are grounded in the transcendental unity of apperception and the categories as pure principles of experience.

Key points: (KeyPoint)

- k1. Empirical association presupposes transcendental affinity.
- k2. There is one experience because there is one lawful synthetic unity of perception.
- k3. Categories are the formal and synthetic principles of all experience.

Claims: (Claim)

- c1. id: kant-trans-deduction-trans-a-affinity-c1
  - subject: affinity_of_appearances
  - predicate: rests_on
  - object: transcendental_unity_of_apperception_and_not_on_empirical_association_alone
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `deduction-trans-a.md:523-565`
    - `deduction-trans-a.md:665-699`

- c2. id: kant-trans-deduction-trans-a-affinity-c2
  - subject: categories
  - predicate: function_as
  - object: formal_and_synthetic_principles_of_all_possible_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `deduction-trans-a.md:504-508`
    - `deduction-trans-a.md:619-636`

Relations: (Relation)

- r1. type: bounds
  - targetEntryId: kant-trans-deduction-trans-a
  - targetWorkbook: `deduction-trans-a-WORKBOOK.md`
  - note: this closing entry turns lawful synthesis into the lawfulness of experience and nature.
  - sourceClaimIds: [`kant-trans-deduction-trans-a-affinity-c1`, `kant-trans-deduction-trans-a-affinity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-deduction-trans-a-c1`, `kant-trans-deduction-trans-a-c2`]

Review outcome:

- review_pending
- notes: the closing movement is kept on affinity and nature so the deduction ends as an account of law-governed experience rather than mere concept ownership.
