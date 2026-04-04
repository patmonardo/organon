# Intro Logic Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes what transcendental logic is and how its field is divided before later analytic and dialectic modules are split further.
- In the current presentation, the distinction from General Logic should be read as generating a demand for reunion rather than as a permanent estrangement.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `INTRO-LOGIC-DISTILLATION.md`
- Upstream source authority: `intro-logic.md`
- This workbook covers only the introduction to transcendental logic.

## Clean-room rules

- Keep the pass on the Kant Transcendental Logic side.
- Do not confuse formal logic with transcendental, object-related cognition.
- Do not turn the critique of illusion into a positive ontology of objects beyond experience.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-26 (first pass)

Scope:

- files:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- pass policy: 1 marker entry + 1 analytic entry

Decision:

- Keep this workbook introductory and architectonic, but upgrade it to a fuller first pass.
- Fix the sequence from sensibility and general logic through transcendental logic, truth, and dialectic.
- Keep the analytic-dialectic split visible before later component workbooks are added.
- Make explicit, at the derivative level, that this distinction pressures a higher unity with General Logic.

### Entry kant-trans-intro — Marker `Transcendental Logic`

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:13-42`
  - `intro-logic.md:54-79`
  - `intro-logic.md:93-124`
  - `intro-logic.md:193-216`
- summary: The introduction differentiates sensibility from understanding, distinguishes general from transcendental logic, and installs the analytic-dialectic architectonic of object-related a priori thought.

Key points: (KeyPoint)

- k1. Logic presupposes the distinction between sensibility and understanding.
- k2. General logic treats the formal use of understanding, while transcendental logic treats pure object-related cognition.
- k3. Transcendental logic concerns a priori cognition as related to objects.
- k4. The introduction divides transcendental logic into analytic and dialectic.

Claims: (Claim)

- c1. id: kant-trans-intro-c1
  - subject: transcendental_logic
  - predicate: studies
  - object: laws_of_pure_understanding_and_reason_as_related_to_objects_a_priori
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `intro-logic.md:117-124`

- c2. id: kant-trans-intro-c2
  - subject: transcendental_logic
  - predicate: divides_into
  - object: transcendental_analytic_and_transcendental_dialectic
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `intro-logic.md:193-216`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: kant-trans-concepts
  - targetWorkbook: `CONCEPTS-WORKBOOK.md`
  - note: the analytic side first opens through the discovery of the pure concepts of understanding.
  - sourceClaimIds: [`kant-trans-intro-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`kant-trans-concepts-c1`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-ideas
  - targetWorkbook: `INTRO-IDEAS-WORKBOOK.md`
  - note: the dialectical side later opens through the critique of transcendental illusion.
  - sourceClaimIds: [`kant-trans-intro-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`kant-trans-ideas-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the architectonic role of the introduction before later analytic and dialectic workbooks are populated.

### Entry kant-trans-intro-faculties — `Transcendental Logic`: sensibility, understanding, and the split of sciences

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:13-42`
- summary: The introduction begins from the difference and cooperation of sensibility and understanding, establishing why aesthetic and logic must be separately treated.

Key points: (KeyPoint)

- k1. Intuition gives and concept thinks.
- k2. Without sensibility nothing is given; without understanding nothing is thought.
- k3. Aesthetic and logic are separated because the faculties do not exchange roles.

Claims: (Claim)

- c1. id: kant-trans-intro-faculties-c1
  - subject: cognition
  - predicate: requires
  - object: union_of_sensible_intuition_and_conceptual_thought
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `intro-logic.md:13-39`

- c2. id: kant-trans-intro-faculties-c2
  - subject: distinction_between_aesthetic_and_logic
  - predicate: rests_on
  - object: irreducible_difference_between_sensibility_and_understanding
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `intro-logic.md:40-42`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: this entry fixes the two-faculty opening that the rest of the introduction presupposes.
  - sourceClaimIds: [`kant-trans-intro-faculties-c1`, `kant-trans-intro-faculties-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`]

Review outcome:

- review_pending
- notes: the faculties are kept distinct here so transcendental logic never floats free of intuition.

### Entry kant-trans-intro-general-logic — `Transcendental Logic`: general, pure, and applied logic

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:44-89`
- summary: Before transcendental logic is introduced, Kant fixes the scope of general logic as formal thinking, distinguishing its pure canonical form from its applied, empirically conditioned use.

Key points: (KeyPoint)

- k1. General logic abstracts from differences among objects.
- k2. Pure logic is formal and a priori.
- k3. Applied logic concerns subjective empirical conditions and is not strict science.

Claims: (Claim)

- c1. id: kant-trans-intro-general-logic-c1
  - subject: general_logic
  - predicate: abstracts_from
  - object: difference_of_objects_in_order_to_set_out_necessary_rules_of_thinking
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `intro-logic.md:44-79`

- c2. id: kant-trans-intro-general-logic-c2
  - subject: applied_logic
  - predicate: concerns
  - object: contingent_empirical_conditions_of_understanding_and_is_not_a_pure_canon
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `intro-logic.md:81-89`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the introduction can define transcendental logic only after the purely formal scope of general logic is delimited.
  - sourceClaimIds: [`kant-trans-intro-general-logic-c1`, `kant-trans-intro-general-logic-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the pure and applied distinction explicit because it prepares the contrast with transcendental logic.

### Entry kant-trans-intro-transcendental — `Transcendental Logic`: object-related a priori cognition

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:93-124`
- summary: Kant defines transcendental logic as the science of pure understanding and reason insofar as they relate to objects a priori, clarifying that not every a priori representation is therefore transcendental.

Key points: (KeyPoint)

- k1. Transcendental logic does not abstract from all content of cognition.
- k2. The transcendental concerns the possibility and use of a priori representations.
- k3. Object-related pure concepts of thought motivate the science of transcendental logic.

Claims: (Claim)

- c1. id: kant-trans-intro-transcendental-c1
  - subject: transcendental_logic
  - predicate: concerns
  - object: pure_thinking_of_objects_insofar_as_its_origin_and_relation_to_objects_are_a_priori
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `intro-logic.md:93-103`
    - `intro-logic.md:117-124`

- c2. id: kant-trans-intro-transcendental-c2
  - subject: transcendental_cognition
  - predicate: means
  - object: cognition_of_how_a_priori_representations_or_concepts_are_possible_and_apply_to_objects
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `intro-logic.md:105-115`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: this entry fixes the exact meaning of the transcendental before the later analytic and dialectic are split.
  - sourceClaimIds: [`kant-trans-intro-transcendental-c1`, `kant-trans-intro-transcendental-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`]

Review outcome:

- review_pending
- notes: the warning that not every a priori representation is transcendental is kept central because it governs the rest of the critique.

### Entry kant-trans-intro-critique — `Transcendental Logic`: truth, critique, and illusion

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:128-189`
- summary: The introduction argues that general logic supplies only a negative criterion of truth and becomes dialectical when it is misused as an organon for objective claims.

Key points: (KeyPoint)

- k1. General logic judges only formal consistency.
- k2. Formal consistency is not sufficient for objective truth.
- k3. Dialectic begins when logic is misused as an organon of objective cognition.

Claims: (Claim)

- c1. id: kant-trans-intro-critique-c1
  - subject: general_logic
  - predicate: provides
  - object: negative_criterion_of_formal_truth
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `intro-logic.md:139-156`

- c2. id: kant-trans-intro-critique-c2
  - subject: misuse_of_logic_as_organon
  - predicate: generates
  - object: dialectical_illusion_by_treating_formal_logic_as_productive_of_objective_truth
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `intro-logic.md:158-189`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the analytic entry shows why the introduction must divide logic into truth-guiding analytic and illusion-critiquing dialectic.
  - sourceClaimIds: [`kant-trans-intro-critique-c1`, `kant-trans-intro-critique-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`, `kant-trans-intro-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the truth-question and the critique of illusion together at seed scale.

### Entry kant-trans-intro-analytic-dialectic — `Transcendental Logic`: analytic and dialectic

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:193-216`
- summary: Transcendental logic divides into analytic and dialectic because pure understanding is lawful only under the condition of intuition and becomes illusory when it is used beyond possible experience; in the current presentation, this is treated as Kant's dualized presentation of one logical life under finite conditions.

Key points: (KeyPoint)

- k1. Pure understanding requires given intuition if cognition is to have objects.
- k2. Transcendental analytic is a logic of truth.
- k3. Transcendental dialectic critiques the hyperphysical misuse of pure understanding and reason.

Claims: (Claim)

- c1. id: kant-trans-intro-analytic-dialectic-c1
  - subject: transcendental_analytic
  - predicate: expounds
  - object: pure_elements_and_principles_without_which_no_object_can_be_thought
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `intro-logic.md:193-200`

- c2. id: kant-trans-intro-analytic-dialectic-c2
  - subject: transcendental_dialectic
  - predicate: critiques
  - object: illusion_arising_when_pure_understanding_or_reason_is_used_beyond_possible_experience
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `intro-logic.md:201-216`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: this entry fixes the governing architectonic division announced by the introduction.
  - sourceClaimIds: [`kant-trans-intro-analytic-dialectic-c1`, `kant-trans-intro-analytic-dialectic-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c2`]

Review outcome:

- review_pending
- notes: the analytic-dialectic split is held here under the condition of intuition so the later critique never drifts into empty formalism; derivative reuse reads this as a dualized presentation rather than as a denial that logic pressures toward higher unity.

### Entry kant-trans-intro-bridge — `Transcendental Logic`: distinction that demands reunion

- sourceFiles:
  - `intro-logic.md`
  - `INTRO-LOGIC-DISTILLATION.md`
- lineSpans:
  - `intro-logic.md:13-42`
  - `intro-logic.md:93-124`
  - `intro-logic.md:193-216`
- summary: On a derivative reading, the distinction between general and transcendental logic generates a demand for reunion, because the formal powers mapped in general logic call for transcendental completion if they are to relate to objects a priori; this is the pressure by which a Kantian dualized presentation can be recollected monistically as one logic.

Key points: (KeyPoint)

- k1. General and transcendental logic are distinct without being foreign to one another.
- k2. The distinction pressures a higher architectonic unity.
- k3. This pressure can be read as a demand of pure reason itself.
- k4. In a stronger derivative register, dialectic can be read as an internal moment of a single logic rather than as a merely external second half.

Claims: (Claim)

- c1. id: kant-trans-intro-bridge-c1
  - subject: distinction_between_general_and_transcendental_logic
  - predicate: generates
  - object: demand_for_higher_unity_of_the_thinking_machine
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `intro-logic.md:13-42`
    - `intro-logic.md:93-124`
    - `intro-logic.md:193-216`

- c2. id: kant-trans-intro-bridge-c2
  - subject: pure_reason
  - predicate: can_be_read_as_demanding
  - object: reunion_of_formal_and_transcendental_logic
  - modality: interpreted
  - confidence: 0.84
  - evidence:
    - `intro-logic.md:117-124`
    - `intro-logic.md:193-216`

Relations: (Relation)

- r1. type: interprets
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: this bridge entry states the larger architectonic consequence of the introduction for the current presentation.
  - sourceClaimIds: [`kant-trans-intro-bridge-c1`, `kant-trans-intro-bridge-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-intro-c1`, `kant-trans-intro-c2`]

Review outcome:

- review_pending
- notes: the unity-demand is kept explicitly interpretive so the source distinction is preserved while the present architectonic is made legible; the dualism-versus-monism axis is held explicit here without rewriting Kant as though he had already made the monistic move.
