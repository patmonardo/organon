# Principles Ground Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the ground of the distinction between phenomena and noumena.
- In the current presentation, it should be read as the closure of the Analytic of Principles and as the main boundary-setting clarification for object in general, thing in itself, and noumenon.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-GROUND-DISTILLATION.md`
- Upstream source authority: `principles-ground.md`
- This workbook covers the ground of the phenomena/noumena distinction across the A and B versions, with special weight on the fuller B-edition clarification.
- A and B should be handled here as co-divisions of one boundary problem, not as duplicate chapters.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not turn object in general into a positively cognized noumenal object.
- Do not confuse transcendental significance with transcendental use.
- Do not flatten thing in itself, negative noumenon, and positive noumenon into one undifferentiated term.
- Do not posit a second knowable world of understanding for us.
- Do not treat A-text and B-text differences as arbitrary variants; they partition the same problem under different weights and clarifications.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Use the A-text to fix the empirical-limit and anti-ontology closure of the Analytic.
- Use the B-text as the main clarification of the object-in-general and noumenon problem.
- Use `transcendental object-thought` as the clean label for the first object-in-general register, so it stays distinct from noumenon proper.
- Keep the negative versus positive noumenon distinction explicit and central.
- Treat noumenon as a boundary-concept, not as a hidden object-stock waiting to be catalogued.
- Treat A and B as co-divisions of the same object-boundary graph: A secures the anti-ontology limit, B gives the fuller articulation of transcendental object-thought and noumenon.

### Entry kant-trans-principles-ground — Marker `Ground of the Distinction into Phenomena and Noumena`

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- lineSpans:
  - `principles-ground.md:11-98`
  - `principles-ground.md:160-194`
  - `principles-ground.md:241-344`
  - `principles-ground.md:346-485`
- summary: The ground chapter closes the Analytic by showing that the categories and principles of understanding have objective validity only for possible experience, that object in general beyond sensible intuition is only indeterminately thought and not cognized, and that noumenon has for us only a negative, boundary-setting significance rather than a positive knowable domain; the A and B versions act here as co-divisions of one boundary problem rather than as unrelated treatments.

Key points: (KeyPoint)

- k1. Pure understanding is valid only for experience.
- k2. Categories without sensible conditions lose determinate objective use.
- k3. Object in general beyond sensible intuition is only indeterminately thought.
- k4. Noumenon is a negative boundary-concept, not positive cognition.
- k5. A and B divide the same problem under different explanatory weights rather than generating two separate doctrines.

Claims: (Claim)

- c1. id: kant-trans-principles-ground-c1
  - subject: pure_concepts_and_principles_of_understanding
  - predicate: have_objective_validity_only_for
  - object: possible_experience_and_never_for_things_in_general_taken_apart_from_our_mode_of_intuition
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:54-98`
    - `principles-ground.md:160-170`
    - `principles-ground.md:241-275`
    - `principles-ground.md:310-320`

- c2. id: kant-trans-principles-ground-c2
  - subject: concept_of_noumenon
  - predicate: functions_as
  - object: necessary_negative_boundary_concept_limiting_sensibility_without_yielding_positive_cognition_of_a_supersensible_domain
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:371-395`
    - `principles-ground.md:408-440`
    - `principles-ground.md:482-485`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-iv
  - targetWorkbook: `PRINCIPLES-IV-WORKBOOK.md`
  - note: the postulates and the system note already restricted all principles to possible experience; this chapter states that restriction at the level of phenomena, things in themselves, and noumena.
  - sourceClaimIds: [`kant-trans-principles-ground-c1`, `kant-trans-principles-ground-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iv-c1`, `kant-trans-principles-iv-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-reflection
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: once the boundary between appearance and non-appearance is fixed, the next move is to examine the reflective misunderstandings that arise when that boundary is mishandled.
  - sourceClaimIds: [`kant-trans-principles-ground-c1`, `kant-trans-principles-ground-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-c1`]

Review outcome:

- review_pending
- notes: marker entry treats the chapter as a boundary-clarification, not as a new noumenal doctrine, and keeps the A/B relation intelligible as a co-division within one graph-region.

### Entry kant-trans-principles-ground-limit — `Ground`: categories and ontology restricted to experience

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- lineSpans:
  - `principles-ground.md:54-98`
  - `principles-ground.md:160-170`
  - `principles-ground.md:241-275`
  - `principles-ground.md:310-320`
- summary: The chapter first reasserts that concepts require intuition for objective sense, so the categories and their principles can never ground a transcendental ontology of things in general but only the analytic of possible experience.

Key points: (KeyPoint)

- k1. A concept without an object given in intuition is empty.
- k2. Even mathematics must exhibit its concepts in appearance.
- k3. Categories lose significance when sensible conditions are removed.
- k4. Ontology must give way to an analytic of the pure understanding.

Claims: (Claim)

- c1. id: kant-trans-principles-ground-limit-c1
  - subject: categories
  - predicate: require
  - object: descent_to_conditions_of_sensibility_for_any_determinate_objective_significance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:83-98`
    - `principles-ground.md:270-275`

- c2. id: kant-trans-principles-ground-limit-c2
  - subject: transcendental_analytic
  - predicate: reduces
  - object: proud_ontology_to_mere_analytic_of_pure_understanding_bounded_by_possible_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:164-170`
    - `principles-ground.md:314-320`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: this entry secures the anti-ontology thesis before the noumenon distinction is introduced.
  - sourceClaimIds: [`kant-trans-principles-ground-limit-c1`, `kant-trans-principles-ground-limit-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c1`]

Review outcome:

- review_pending
- notes: the anti-ontology line is kept because it frames the whole noumenon discussion negatively from the outset.

### Entry kant-trans-principles-ground-object — `Ground`: object in general, transcendental significance, and no transcendental use

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- lineSpans:
  - `principles-ground.md:172-194`
  - `principles-ground.md:322-344`
  - `principles-ground.md:397-406`
- summary: When intuition is left unspecified, the understanding can still think an object in general as a transcendental object-thought, but this does not determine any actual object; it leaves only transcendental significance and no genuine use of the categories.

Key points: (KeyPoint)

- k1. Thinking relates intuitions to an object.
- k2. If the manner of intuition is omitted, the object is merely transcendental.
- k3. Categories then express only the thought of an object in general.
- k4. Without schema, subsumption and object-determination disappear.

Claims: (Claim)

- c1. id: kant-trans-principles-ground-object-c1
  - subject: object_in_general_beyond_given_sensible_intuition
  - predicate: is
  - object: merely_transcendental_object_thought_and_not_determined_as_cognized_object
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:172-185`
    - `principles-ground.md:322-335`

- c2. id: kant-trans-principles-ground-object-c2
  - subject: pure_categories_without_schema
  - predicate: retain_only
  - object: transcendental_significance_and_not_any_transcendental_use_or_determinate_objective_application
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:187-194`
    - `principles-ground.md:337-344`
    - `principles-ground.md:397-406`

Relations: (Relation)

- r1. type: clarifies
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: this entry isolates the specific point where empty object-thought must not be mistaken for noumenal cognition.
  - sourceClaimIds: [`kant-trans-principles-ground-object-c1`, `kant-trans-principles-ground-object-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c1`, `kant-trans-principles-ground-c2`]

Review outcome:

- review_pending
- notes: this is the main anti-confusion entry for object in general and transcendental object; later reflection material should be read as a co-division of the same higher object-problem, not as a replacement for this entry.

### Entry kant-trans-principles-ground-noumenon — `Ground`: ambiguity, negative noumenon, positive noumenon

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- lineSpans:
  - `principles-ground.md:346-395`
- summary: Kant identifies the ambiguity that tempts the understanding to treat the beyond of appearance as a cognizable being of understanding, then resolves it by distinguishing negative noumenon from positive noumenon.

Key points: (KeyPoint)

- k1. The understanding is tempted to oppose phenomena to a determinate class of noumena.
- k2. This begins from an ambiguity in the thought of object in itself.
- k3. Negative noumenon means only not-object-of-our-senses.
- k4. Positive noumenon would require intellectual intuition, which we do not have.

Claims: (Claim)

- c1. id: kant-trans-principles-ground-noumenon-c1
  - subject: ambiguity_in_noumenon_talk
  - predicate: consists_in
  - object: mistaking_entirely_undetermined_being_of_understanding_for_determinate_object_cognizable_by_understanding
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:363-369`

- c2. id: kant-trans-principles-ground-noumenon-c2
  - subject: noumenon
  - predicate: splits_into
  - object: legitimate_negative_sense_and_unavailable_positive_sense_requiring_intellectual_intuition
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:371-395`

Relations: (Relation)

- r1. type: clarifies
  - targetEntryId: kant-trans-principles-ground-object
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: the noumenon distinction resolves the error of taking empty object-thought as a knowable supersensible object.
  - sourceClaimIds: [`kant-trans-principles-ground-noumenon-c1`, `kant-trans-principles-ground-noumenon-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-object-c1`, `kant-trans-principles-ground-object-c2`]

- r2. type: explicates
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: this entry gives the central vocabulary by which the chapter sorts the appearance/noumenon distinction.
  - sourceClaimIds: [`kant-trans-principles-ground-noumenon-c1`, `kant-trans-principles-ground-noumenon-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c2`]

Review outcome:

- review_pending
- notes: negative and positive noumenon are kept explicit because this is the chapter's main terminological pivot.

### Entry kant-trans-principles-ground-boundary — `Ground`: noumenon as problematic boundary-concept and negative expansion

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-DISTILLATION.md`
- lineSpans:
  - `principles-ground.md:408-485`
- summary: The concept of noumenon is admissible only problematically, as a boundary-concept that limits sensibility without opening a second positive domain of knowledge or a world of understanding accessible to our categories.

Key points: (KeyPoint)

- k1. Noumenon is not contradictory but remains problematic.
- k2. It is needed to limit the objective validity of sensible cognition.
- k3. The domain beyond appearance is empty for us because no suitable intuition is available.
- k4. Synthetic transcendental principles about noumena are impossible.

Claims: (Claim)

- c1. id: kant-trans-principles-ground-boundary-c1
  - subject: concept_of_noumenon
  - predicate: is
  - object: merely_problematic_boundary_concept_of_negative_use_limiting_sensibility
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:408-440`

- c2. id: kant-trans-principles-ground-boundary-c2
  - subject: alleged_positive_world_of_understanding_for_us
  - predicate: is_denied_because
  - object: categories_without_possible_non_sensible_intuition_cannot_ground_any_synthetic_transcendental_principles_about_noumena
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:424-485`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: this entry states the final boundary function of noumenon and closes the chapter's negative result.
  - sourceClaimIds: [`kant-trans-principles-ground-boundary-c1`, `kant-trans-principles-ground-boundary-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c2`]

Review outcome:

- review_pending
- notes: negative expansion is kept as boundary-language only, not as acquisition of a new object-domain.
