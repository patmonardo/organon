# Aesthetic Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the Transcendental Aesthetic as the sensibility-side condition for later transcendental logic.
- In the current presentation, it should be read as the givenness-side counterpart to the formal powers articulated on the General Logic side.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `AESTHETIC-DISTILLATION.md`
- Upstream source authority: `aesthetic.md`
- This workbook covers only the Transcendental Aesthetic module.

## Clean-room rules

- Keep the pass on the Kant Transcendental Aesthetic side.
- Do not turn space and time into properties of things in themselves.
- Do not collapse appearance into mere illusion or into confused intellectual cognition.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by major argumentative turn

Decision:

- Keep this workbook architectonic but fuller than the small seed modules.
- Preserve the sequence from sensibility and pure intuition, through space and time, to appearance, self-affection, and the final synthetic a priori limit.
- Make the Aesthetic readable beside the newer General Logic presentation without blurring the specifically transcendental claims.

### Entry kant-trans-aesthetic - Marker `Transcendental Aesthetic`

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:11-57`
  - `aesthetic.md:80-172`
  - `aesthetic.md:198-294`
  - `aesthetic.md:383-606`
- summary: The Transcendental Aesthetic isolates sensibility as the receptive source of intuition, discovers space and time as its pure a priori forms, and thereby explains both the possibility and the limit of synthetic a priori cognition.

Key points: (KeyPoint)

- k1. Sensibility gives objects through intuition, while understanding thinks them.
- k2. Space and time are the pure forms of sensible intuition.
- k3. Appearances are objectively valid for experience while remaining distinct from things in themselves.
- k4. Synthetic a priori cognition is possible only within the field of possible experience.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-c1
  - subject: transcendental_aesthetic
  - predicate: researches
  - object: a_priori_forms_of_sensibility_as_conditions_of_appearance
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `aesthetic.md:11-57`

- c2. id: kant-trans-aesthetic-c2
  - subject: transcendental_aesthetic
  - predicate: grounds
  - object: possibility_and_boundary_of_synthetic_a_priori_cognition_for_appearances
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `aesthetic.md:339-344`
    - `aesthetic.md:601-606`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: the Aesthetic fixes the sensibility-side conditions that transcendental logic will presuppose when it isolates pure understanding and reason.
  - sourceClaimIds: [`kant-trans-aesthetic-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-trans-intro-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the Aesthetic as the receptive and intuitional side of the transcendental machine before the logic of objects a priori is unfolded.

### Entry kant-trans-aesthetic-sensibility - `Aesthetic`: intuition, appearance, and pure form

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:11-57`
- summary: The opening of the Aesthetic distinguishes sensibility from understanding, divides appearance into matter and form, and isolates pure intuition as the a priori form of receptivity.

Key points: (KeyPoint)

- k1. Intuition is the immediate relation to an object.
- k2. Sensibility is receptivity, while understanding thinks through concepts.
- k3. Appearance divides into sensible matter and a priori form.
- k4. The Aesthetic isolates pure intuition by removing both conceptual determination and sensation.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-sensibility-c1
  - subject: sensibility
  - predicate: gives
  - object: objects_through_intuition_while_understanding_thinks_them_through_concepts
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `aesthetic.md:11-19`

- c2. id: kant-trans-aesthetic-sensibility-c2
  - subject: transcendental_aesthetic
  - predicate: isolates
  - object: pure_intuition_as_the_a_priori_form_of_appearance_disclosing_space_and_time
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `aesthetic.md:25-57`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-aesthetic
  - targetWorkbook: `AESTHETIC-WORKBOOK.md`
  - note: this entry fixes the methodological opening of the Aesthetic before the two pure forms are separately unfolded.
  - sourceClaimIds: [`kant-trans-aesthetic-sensibility-c1`, `kant-trans-aesthetic-sensibility-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-aesthetic-c1`]

Review outcome:

- review_pending
- notes: the opening distinction is kept strict so later claims about space and time stay tied to receptivity rather than thought.

### Entry kant-trans-aesthetic-space - `Aesthetic`: space as outer pure intuition

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:65-190`
- summary: The exposition of space shows that it is a non-empirical, necessary, singular, infinite pure intuition of outer sense, making geometry possible while remaining valid only for appearances.

Key points: (KeyPoint)

- k1. Space is not abstracted from outer experience but grounds it.
- k2. Space is a pure intuition rather than a discursive concept.
- k3. Geometry is possible only if space is given a priori in the subject.
- k4. Space is empirically real for outer appearances and transcendentally ideal with respect to things in themselves.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-space-c1
  - subject: space
  - predicate: is
  - object: necessary_singular_infinite_pure_intuition_of_outer_sense
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `aesthetic.md:80-107`

- c2. id: kant-trans-aesthetic-space-c2
  - subject: space
  - predicate: grounds
  - object: possibility_of_geometry_and_empirical_reality_of_outer_appearance_without_belonging_to_things_in_themselves
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `aesthetic.md:111-172`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-aesthetic
  - targetWorkbook: `AESTHETIC-WORKBOOK.md`
  - note: this entry fixes the outer sensible form and its mathematical consequence.
  - sourceClaimIds: [`kant-trans-aesthetic-space-c1`, `kant-trans-aesthetic-space-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-aesthetic-c1`, `kant-trans-aesthetic-c2`]

Review outcome:

- review_pending
- notes: space is anchored here as pure intuition so geometry remains synthetic a priori without becoming thing-in-itself metaphysics.

### Entry kant-trans-aesthetic-time - `Aesthetic`: time as inner pure intuition

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:198-367`
- summary: Time is the necessary pure intuition of inner sense, the condition of succession and alteration, and the mediate form under which all appearances are given, while remaining transcendentally ideal.

Key points: (KeyPoint)

- k1. Time is not empirical but grounds simultaneity and succession.
- k2. Time is a pure intuition rather than a general concept.
- k3. Alteration and motion are possible only in time.
- k4. Time is the form of inner sense and the mediate condition of outer appearances.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-time-c1
  - subject: time
  - predicate: is
  - object: necessary_singular_pure_intuition_grounding_temporal_relations_and_alteration
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `aesthetic.md:198-235`

- c2. id: kant-trans-aesthetic-time-c2
  - subject: time
  - predicate: functions_as
  - object: form_of_inner_sense_and_mediate_condition_of_all_appearances_with_empirical_reality_and_transcendental_ideality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `aesthetic.md:239-367`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-aesthetic
  - targetWorkbook: `AESTHETIC-WORKBOOK.md`
  - note: this entry fixes the inner sensible form and its wider scope over all appearances.
  - sourceClaimIds: [`kant-trans-aesthetic-time-c1`, `kant-trans-aesthetic-time-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-aesthetic-c1`, `kant-trans-aesthetic-c2`]

Review outcome:

- review_pending
- notes: time's wider reach is preserved here without collapsing it into an objective property of things in themselves.

### Entry kant-trans-aesthetic-appearance - `Aesthetic`: appearance, anti-Leibniz, and relation

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:383-497`
- summary: The general remarks clarify that sensibility yields appearances rather than confused knowledge of things in themselves, reject the Leibnizian reduction of the sensible/intellectual distinction to a merely logical one, and defend the certainty of synthetic a priori cognition by tying it to subjective forms of intuition.

Key points: (KeyPoint)

- k1. Appearance is not a confused thing in itself but a distinct mode of givenness.
- k2. The sensible/intellectual distinction is transcendental, not merely logical.
- k3. Space and time as subjective forms explain synthetic a priori geometry without collapsing into illusion.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-appearance-c1
  - subject: sensibility
  - predicate: yields
  - object: appearances_not_confused_cognition_of_things_in_themselves
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `aesthetic.md:386-442`

- c2. id: kant-trans-aesthetic-appearance-c2
  - subject: transcendental_ideality_of_space_and_time
  - predicate: secures
  - object: certainty_of_synthetic_a_priori_cognition_for_appearances_without_extending_it_to_things_in_themselves
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `aesthetic.md:453-497`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-trans-aesthetic-space
  - targetWorkbook: `AESTHETIC-WORKBOOK.md`
  - note: the general remarks defend the earlier doctrine of space and time against logical, empiricist, and dogmatic misreadings.
  - sourceClaimIds: [`kant-trans-aesthetic-appearance-c1`, `kant-trans-aesthetic-appearance-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-trans-aesthetic-space-c2`, `kant-trans-aesthetic-time-c2`]

Review outcome:

- review_pending
- notes: the anti-Leibnizian turn is kept explicit because it is one of the Aesthetic's decisive architectonic clarifications.

### Entry kant-trans-aesthetic-boundary - `Aesthetic`: self-affection, no illusion, and the limit of experience

- sourceFiles:
  - `aesthetic.md`
  - `AESTHETIC-DISTILLATION.md`
- lineSpans:
  - `aesthetic.md:501-606`
- summary: The closing remarks extend the doctrine to inner sense and self-affection, distinguish appearance from illusion, deny that space and time can be conditions of divine or thing-in-itself intuition, and conclude that pure intuition explains synthetic a priori cognition only for possible experience.

Key points: (KeyPoint)

- k1. Inner sense gives even the subject only as appearance.
- k2. Appearance is objectively valid for sensibility and must not be reduced to illusion.
- k3. Space and time cannot be carried over to intellectual intuition or to God.
- k4. The Aesthetic solves the synthetic a priori problem only within the field of possible experience.

Claims: (Claim)

- c1. id: kant-trans-aesthetic-boundary-c1
  - subject: inner_sense
  - predicate: presents
  - object: the_subject_to_itself_as_appearance_through_self_affection_not_as_thing_in_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `aesthetic.md:514-538`

- c2. id: kant-trans-aesthetic-boundary-c2
  - subject: pure_intuitions_of_space_and_time
  - predicate: make_possible
  - object: synthetic_a_priori_cognition_only_for_objects_of_possible_experience_and_not_for_things_in_themselves_or_divine_intuition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `aesthetic.md:542-597`
    - `aesthetic.md:601-606`

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: kant-trans-intro
  - targetWorkbook: `INTRO-LOGIC-WORKBOOK.md`
  - note: once sensible givenness and its bounds are fixed, transcendental logic can isolate the lawful object-related use of pure understanding and reason.
  - sourceClaimIds: [`kant-trans-aesthetic-boundary-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`kant-trans-intro-c1`, `kant-trans-intro-c2`]

Review outcome:

- review_pending
- notes: the closing entry keeps the theological and experiential boundary together because both serve the same restriction of synthetic a priori validity.
