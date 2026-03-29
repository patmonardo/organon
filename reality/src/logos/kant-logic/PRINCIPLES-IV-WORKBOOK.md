# Principles IV Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the postulates of empirical thinking in general, together with the refutation of idealism and the concluding general note.
- In the current presentation, it should be read as the modality-side completion of the four principle families.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-IV-DISTILLATION.md`
- Upstream source authority: `principles-IV.md`
- This workbook covers the postulates, refutation of idealism, and general note.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not read modality as adding object-content rather than cognitive standing.
- Do not let possibility collapse into mere non-contradiction.
- Do not treat the refutation of idealism as proving that every seeming outer perception is veridical.
- Keep the closing note tied to possible experience and intuition.
- Do not let empirical use slide into any supposed transcendental use.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-IV.md`
  - `PRINCIPLES-IV-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep the marker entry on the empirical-use restriction of modality.
- Separate possibility/actuality/necessity from the idealism theorem and the final system note.
- Use empirical-use-only language consistently, since this chapter states the restriction more explicitly than the earlier families do.
- Preserve the outer-experience condition for inner time-determination as the chapter's major architectonic payoff.

### Entry kant-trans-principles-iv — Marker `Postulates of Empirical Thinking`

- sourceFiles:
  - `principles-IV.md`
  - `PRINCIPLES-IV-DISTILLATION.md`
- lineSpans:
  - `principles-IV.md:1-163`
  - `principles-IV.md:164-260`
  - `principles-IV.md:261-434`
  - `principles-IV.md:435-546`
- summary: The fourth family of principles states that possibility, actuality, and necessity concern an object's relation to our cognition within possible experience and empirical use, culminates in the refutation of idealism by grounding inner time-determination in outer experience, and closes by reaffirming that all principles of pure understanding are principles only of possible experience.

Key points: (KeyPoint)

- k1. Modality does not add content to the object-concept.
- k2. Possibility, actuality, and necessity are fixed only for empirical use.
- k3. Outer experience is required for determinate inner experience.
- k4. The whole system of principles is limited to possible experience.

Claims: (Claim)

- c1. id: kant-trans-principles-iv-c1
  - subject: postulates_of_empirical_thinking
  - predicate: define
  - object: possibility_actuality_and_necessity_only_as_relations_of_objects_to_cognition_within_possible_experience_and_empirical_use
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-IV.md:7-163`

- c2. id: kant-trans-principles-iv-c2
  - subject: system_of_principles
  - predicate: culminates_in
  - object: thesis_that_inner_experience_requires_outer_experience_and_that_all_synthetic_principles_concern_only_possible_experience
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:164-260`
    - `principles-IV.md:435-546`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: after persistence, causality, and community determine existence in time, modality specifies how objects count as possible, actual, and necessary for cognition.
  - sourceClaimIds: [`kant-trans-principles-iv-c1`, `kant-trans-principles-iv-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c1`, `kant-trans-principles-iii-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: once all four principle families are complete, the next move is to state the common ground of their possibility more explicitly.
  - sourceClaimIds: [`kant-trans-principles-iv-c1`, `kant-trans-principles-iv-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes modality as empirical standing rather than added content.

### Entry kant-trans-principles-iv-modal — `Postulates`: empirical modality and its limits

- sourceFiles:
  - `principles-IV.md`
  - `PRINCIPLES-IV-DISTILLATION.md`
- lineSpans:
  - `principles-IV.md:7-163`
  - `principles-IV.md:261-434`
- summary: Kant argues that objective possibility depends on formal conditions of experience, actuality on perception or lawful connection to perception, and necessity on lawful causal connection within experience rather than on mere concepts.

Key points: (KeyPoint)

- k1. Possibility requires more than non-contradiction.
- k2. Actuality requires perception or empirical connection to it.
- k3. Necessity is only hypothetical within experience.
- k4. Modality does not transcend empirical use.

Claims: (Claim)

- c1. id: kant-trans-principles-iv-modal-c1
  - subject: objective_possibility
  - predicate: requires
  - object: agreement_of_concept_with_formal_conditions_of_possible_experience_and_not_mere_logical_noncontradiction
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:20-125`

- c2. id: kant-trans-principles-iv-modal-c2
  - subject: actuality_and_necessity
  - predicate: are_cognized_only_through
  - object: perception_and_lawful_empirical_connection_within_one_experience_especially_causal_connection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:127-163`
    - `principles-IV.md:261-364`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iv
  - targetWorkbook: `PRINCIPLES-IV-WORKBOOK.md`
  - note: this entry states the modal core before the idealism and system-level consequences are drawn.
  - sourceClaimIds: [`kant-trans-principles-iv-modal-c1`, `kant-trans-principles-iv-modal-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iv-c1`]

Review outcome:

- review_pending
- notes: the modal entries are kept empirical to block any slide into transcendental realism.

### Entry kant-trans-principles-iv-idealism — `Refutation of Idealism`: outer experience as condition of inner time-determination

- sourceFiles:
  - `principles-IV.md`
  - `PRINCIPLES-IV-DISTILLATION.md`
- lineSpans:
  - `principles-IV.md:164-260`
- summary: Kant argues that determinately conscious inner existence in time presupposes something persistent in outer perception, so outer experience is immediate in the relevant sense and inner experience is possible only through it.

Key points: (KeyPoint)

- k1. Problematic idealism doubts outer existence while treating inner existence as immediate.
- k2. Time-determination requires something persistent.
- k3. Mere inner awareness does not provide that persistent correlate.
- k4. Outer experience is therefore the condition of determinate inner experience.

Claims: (Claim)

- c1. id: kant-trans-principles-iv-idealism-c1
  - subject: empirically_determined_consciousness_of_my_own_existence
  - predicate: proves
  - object: existence_of_objects_in_space_outside_me
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-IV.md:187-202`

- c2. id: kant-trans-principles-iv-idealism-c2
  - subject: inner_experience
  - predicate: is_possible_only_through
  - object: outer_experience_because_time_determination_requires_persistent_perception_not_found_in_mere_inner_sense
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:203-260`

Relations: (Relation)

- r1. type: supports
  - targetEntryId: kant-trans-principles-iv
  - targetWorkbook: `PRINCIPLES-IV-WORKBOOK.md`
  - note: this entry supplies the sharpest architectonic consequence of the postulates for the relation of inner and outer experience.
  - sourceClaimIds: [`kant-trans-principles-iv-idealism-c1`, `kant-trans-principles-iv-idealism-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iv-c2`]

Review outcome:

- review_pending
- notes: veridicality of each seeming perception is not asserted here; only the condition of possible inner experience is.

### Entry kant-trans-principles-iv-system-note — `General Note`: categories need intuition and experience

- sourceFiles:
  - `principles-IV.md`
  - `PRINCIPLES-IV-DISTILLATION.md`
- lineSpans:
  - `principles-IV.md:435-546`
- summary: The final note restates that categories alone are mere forms of thought, that their objective reality requires intuition and often outer intuition, and that all principles of pure understanding therefore concern only the possibility of experience.

Key points: (KeyPoint)

- k1. Categories alone do not yield synthetic cognition.
- k2. Objective reality requires intuition.
- k3. Several relation-categories require outer intuition for exhibition.
- k4. All principles of pure understanding are principles of possible experience alone.

Claims: (Claim)

- c1. id: kant-trans-principles-iv-system-note-c1
  - subject: categories
  - predicate: are
  - object: mere_forms_of_thought_until_intuition_displays_their_objective_reality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:435-505`

- c2. id: kant-trans-principles-iv-system-note-c2
  - subject: all_principles_of_pure_understanding
  - predicate: are
  - object: a_priori_principles_of_possibility_of_experience_and_nothing_beyond_it
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-IV.md:541-546`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iv
  - targetWorkbook: `PRINCIPLES-IV-WORKBOOK.md`
  - note: this entry closes the whole principles system by restating its experiential limit.
  - sourceClaimIds: [`kant-trans-principles-iv-system-note-c1`, `kant-trans-principles-iv-system-note-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iv-c2`]

Review outcome:

- review_pending
- notes: this closing boundary is kept explicit because it governs every earlier principle family as well.
