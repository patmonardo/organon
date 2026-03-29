# Principles III Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the analogies of experience as the dynamical principles of temporal existence.
- In the current presentation, it should be read as the transition from mathematical principles to rules of duration, succession, and simultaneity.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-III-DISTILLATION.md`
- Upstream source authority: `principles-III.md`
- This workbook covers the general analogies framework plus the three analogies.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not turn the analogies into constitutive constructions like the mathematical principles.
- Do not collapse subjective sequence in apprehension into objective succession in the object.
- Do not detach persistence, causality, or community from possible experience.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep the marker entry on the general character of the analogies as regulative principles.
- Split the three analogies into persistence, causality, and community entries.
- Preserve the house/ship and simultaneity-through-interaction logic explicitly, since those are the most decisive argumentative turns.

### Entry kant-trans-principles-iii — Marker `Analogies of Experience`

- sourceFiles:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- lineSpans:
  - `principles-III.md:1-126`
  - `principles-III.md:128-289`
  - `principles-III.md:291-793`
  - `principles-III.md:795-985`
- summary: The third family of principles establishes that experience is possible only through necessary temporal connection of perceptions, and it determines the three modes of time through persistence of substance, causal succession, and reciprocal community.

Key points: (KeyPoint)

- k1. The analogies concern existence rather than mere possibility.
- k2. They are regulative rather than constitutive.
- k3. Persistence, succession, and simultaneity correspond to three temporal rules.
- k4. The unity of one possible experience depends on substance, cause, and community.

Claims: (Claim)

- c1. id: kant-trans-principles-iii-c1
  - subject: experience
  - predicate: is_possible_only_through
  - object: representation_of_necessary_connection_of_perceptions_in_time
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-III.md:1-26`

- c2. id: kant-trans-principles-iii-c2
  - subject: analogies_of_experience
  - predicate: determine
  - object: duration_succession_and_simultaneity_through_persistence_causality_and_community
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-III.md:27-126`
    - `principles-III.md:128-985`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-ii
  - targetWorkbook: `PRINCIPLES-II-WORKBOOK.md`
  - note: after the mathematical principles determine intuition and perception, the dynamical principles determine existence in time.
  - sourceClaimIds: [`kant-trans-principles-iii-c1`, `kant-trans-principles-iii-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ii-c1`, `kant-trans-principles-ii-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-trans-principles-iv
  - targetWorkbook: `PRINCIPLES-IV-WORKBOOK.md`
  - note: once existence is determined through temporal relations, modality can specify possibility, actuality, and necessity within experience.
  - sourceClaimIds: [`kant-trans-principles-iii-c1`, `kant-trans-principles-iii-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iv-c1`]

Review outcome:

- review_pending
- notes: marker entry fixes the analogies as the dynamical center of the system.

### Entry kant-trans-principles-iii-regulative — `Analogies`: necessary connection and regulative status

- sourceFiles:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- lineSpans:
  - `principles-III.md:1-126`
- summary: Kant distinguishes the analogies from the mathematical principles by showing that they concern existence and temporal relation, so they can provide only regulative rules for empirical synthesis rather than constitutive constructions.

Key points: (KeyPoint)

- k1. Experience is cognition through connected perceptions.
- k2. Necessity is not given in perception itself.
- k3. Time-relations must be determined through rules.
- k4. Analogies are philosophical analogies, not mathematical proportions.

Claims: (Claim)

- c1. id: kant-trans-principles-iii-regulative-c1
  - subject: analogies_of_experience
  - predicate: concern
  - object: existence_and_relation_of_appearances_with_respect_to_time
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-III.md:1-55`

- c2. id: kant-trans-principles-iii-regulative-c2
  - subject: analogies_of_experience
  - predicate: are
  - object: merely_regulative_principles_of_empirical_use_and_not_constitutive_principles_of_things_in_themselves
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-III.md:57-126`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: this entry fixes the analogies' distinctive status before the three individual principles are split out.
  - sourceClaimIds: [`kant-trans-principles-iii-regulative-c1`, `kant-trans-principles-iii-regulative-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c1`, `kant-trans-principles-iii-c2`]

Review outcome:

- review_pending
- notes: the mathematical-versus-dynamical contrast is preserved here instead of repeating it under each analogy.

### Entry kant-trans-principles-iii-persistence — `First Analogy`: persistence of substance

- sourceFiles:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- lineSpans:
  - `principles-III.md:128-289`
- summary: The first analogy argues that all time-determination in appearance presupposes a persistent substratum, so substance as appearance grounds duration and makes alteration intelligible as change of states rather than origination or annihilation of the substratum.

Key points: (KeyPoint)

- k1. Time cannot be perceived by itself.
- k2. A persistent correlate in appearance is required.
- k3. Substance fulfills that role as substratum of time-determination.
- k4. Alteration concerns determinations of substance, not substance's own arising or perishing.

Claims: (Claim)

- c1. id: kant-trans-principles-iii-persistence-c1
  - subject: substance_in_appearance
  - predicate: persists_as
  - object: substratum_of_all_time_determination_and_condition_of_duration
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-III.md:134-205`

- c2. id: kant-trans-principles-iii-persistence-c2
  - subject: alteration
  - predicate: is
  - object: succession_of_determinations_of_what_persists_and_not_arising_or_perishing_of_substance_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-III.md:241-289`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: the first analogy secures duration by providing the persistent correlate of time.
  - sourceClaimIds: [`kant-trans-principles-iii-persistence-c1`, `kant-trans-principles-iii-persistence-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c2`]

Review outcome:

- review_pending
- notes: persistence is kept as the condition of all later temporal determination, not just one doctrine among others.

### Entry kant-trans-principles-iii-causality — `Second Analogy`: objective succession through cause

- sourceFiles:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- lineSpans:
  - `principles-III.md:291-793`
- summary: The second analogy argues that objective succession cannot be read off mere successive apprehension, but requires a rule according to which the later state must follow the earlier, and this rule is the law of causality.

Key points: (KeyPoint)

- k1. Mere apprehension is always successive.
- k2. House-order can vary, while ship-motion cannot.
- k3. Objective eventhood requires a necessary order.
- k4. Causality also grounds the continuity of alteration through time.

Claims: (Claim)

- c1. id: kant-trans-principles-iii-causality-c1
  - subject: objective_succession_of_appearances
  - predicate: requires
  - object: rule_by_which_later_state_necessarily_follows_earlier_state_as_effect_from_cause
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-III.md:297-520`

- c2. id: kant-trans-principles-iii-causality-c2
  - subject: alteration_under_causality
  - predicate: proceeds_through
  - object: continuous_generation_of_new_state_across_time_of_change_and_not_by_unmediated_leap
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-III.md:716-793`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: the second analogy secures objective succession and the lawful structure of events.
  - sourceClaimIds: [`kant-trans-principles-iii-causality-c1`, `kant-trans-principles-iii-causality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c1`, `kant-trans-principles-iii-c2`]

Review outcome:

- review_pending
- notes: the house/ship contrast and continuity of alteration are kept together because both serve the same objective-sequence argument.

### Entry kant-trans-principles-iii-community — `Third Analogy`: simultaneity through interaction

- sourceFiles:
  - `principles-III.md`
  - `PRINCIPLES-III-DISTILLATION.md`
- lineSpans:
  - `principles-III.md:795-985`
- summary: The third analogy argues that simultaneity of substances can be cognized only if they stand in reciprocal interaction, since only community allows each to determine the other's temporal position within one shared time.

Key points: (KeyPoint)

- k1. Reciprocal order of perceptions is the mark of simultaneity.
- k2. Time itself cannot be perceived as the basis of that simultaneity.
- k3. Interaction among substances is required.
- k4. Community secures coexistence in one possible experience.

Claims: (Claim)

- c1. id: kant-trans-principles-iii-community-c1
  - subject: simultaneity_of_substances
  - predicate: can_be_cognized_only_under
  - object: presupposition_of_thoroughgoing_interaction_or_community
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-III.md:801-900`

- c2. id: kant-trans-principles-iii-community-c2
  - subject: community_of_substances
  - predicate: makes_possible
  - object: empirical_relation_of_coexistence_and_whole_of_simultaneous_experience
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-III.md:901-985`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-iii
  - targetWorkbook: `PRINCIPLES-III-WORKBOOK.md`
  - note: the third analogy completes the temporal system by determining simultaneity through reciprocal influence.
  - sourceClaimIds: [`kant-trans-principles-iii-community-c1`, `kant-trans-principles-iii-community-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-iii-c2`]

Review outcome:

- review_pending
- notes: community is kept in the commercium sense because Kant explicitly wants reciprocal influence, not mere coexistence.
