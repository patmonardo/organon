# Principles Reflection Workbook

Status: active
Authority: distillation-first, source-aware

## Reading note

- This is a seed workbook, not a final graph.
- Read the marker entry first.
- This file fixes the amphiboly of the concepts of reflection and the appendix's final treatment of object in general and nothing.
- In the current presentation, it should be read as the reflective policing of the boundary just established in the ground chapter.

## Authority + format lock (must persist)

- Contract reference: `TRANSCENDENTAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Immediate extraction authority: `PRINCIPLES-REFLECTION-DISTILLATION.md`
- Upstream source authority: `principles-reflection.md`
- This workbook covers transcendental reflection, the four concepts of reflection, the Leibniz diagnosis, the renewed object-in-general problem, and the four senses of nothing.

## Clean-room rules

- Keep the pass on the Kant Transcendental Analytic side.
- Do not treat the concepts of reflection as if they were categories of objects.
- Do not let Leibniz's pure-understanding comparison of appearances stand as a valid transcendental method.
- Keep the appendix's first object-in-general register aligned with the ground chapter's transcendental object-thought.
- Do not collapse the chapter's formal “object in general” into a positively cognized noumenon.
- If mechanism or chemism are invoked, mark them as derivative hypotheses unless the source text itself warrants them.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-03-29 (first pass)

Scope:

- files:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- pass policy: 1 marker entry + source-aligned analytic entries by argumentative turn

Decision:

- Keep the marker entry on transcendental reflection as source-discrimination.
- Split the chapter into four major zones: reflection itself, the four reflection-pairs, the Leibniz/amphiboly diagnosis, and the renewed object-in-general/nothing problem.
- Preserve the object-in-general issue explicitly, since this is the main continuity point with the ground chapter.
- Treat the appendix's first object-in-general register as the same transcendental object-thought fixed in the ground chapter, not as a new noumenal content.
- Treat the user's mechanism/chemism line as a derivative research hypothesis rather than a direct source claim of this appendix.

### Entry kant-trans-principles-reflection — Marker `Amphiboly of the Concepts of Reflection`

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:5-42`
  - `principles-reflection.md:44-112`
  - `principles-reflection.md:114-398`
  - `principles-reflection.md:399-437`
- summary: The appendix argues that objective comparison of representations requires transcendental reflection on whether they belong to sensibility or understanding, shows that failure of this reflection produces the Leibnizian intellectualization of appearances, and closes by re-situating the transcendental object-thought and noumenon as merely problematic boundary notions rather than positive intelligible cognition.

Key points: (KeyPoint)

- k1. Transcendental reflection discriminates cognitive source before objective judgment.
- k2. The four concepts of reflection change meaning between sensibility and understanding.
- k3. Amphiboly produces the false intellectual system of Leibniz.
- k4. Object in general and noumenon remain formal or problematic at the boundary of cognition.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-c1
  - subject: transcendental_reflection
  - predicate: is_required_for
  - object: correct_objective_comparison_of_representations_by_determining_whether_they_belong_to_sensibility_or_understanding
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-reflection.md:5-42`
    - `principles-reflection.md:114-138`

- c2. id: kant-trans-principles-reflection-c2
  - subject: appendix_on_amphiboly
  - predicate: shows
  - object: that_misplacing_reflection_concepts_generates_false_intellectual_cognition_and_that_noumenon_object_in_general_remain_only_problematic_boundary_notions_for_us
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:140-158`
    - `principles-reflection.md:343-398`
    - `principles-reflection.md:399-437`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-ground
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: once the ground chapter limits categories and noumenon, the reflection appendix diagnoses the comparative mistakes that arise when that limit is ignored.
  - sourceClaimIds: [`kant-trans-principles-reflection-c1`, `kant-trans-principles-reflection-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-c1`, `kant-trans-principles-ground-c2`]

Review outcome:

- review_pending
- notes: marker entry keeps the appendix attached to the ground chapter's boundary work rather than treating it as a separate ontology.

### Entry kant-trans-principles-reflection-source — `Reflection`: source-discrimination before comparison

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:5-42`
- summary: Reflection is the state of mind in which one determines the source of representations, and transcendental reflection is the act of deciding whether representations belong to sensibility or understanding before comparing them objectively.

Key points: (KeyPoint)

- k1. Reflection does not directly derive concepts from objects.
- k2. It prepares the subjective conditions of conceptual use.
- k3. The comparison must ask which faculty the representations belong to.
- k4. Without this, objective determination of their relation is impossible.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-source-c1
  - subject: reflection
  - predicate: is
  - object: consciousness_of_relation_of_given_representations_to_their_sources_of_cognition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:5-10`

- c2. id: kant-trans-principles-reflection-source-c2
  - subject: transcendental_reflection
  - predicate: determines
  - object: objective_relation_of_representations_by_discriminating_pure_understanding_from_pure_intuition
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-reflection.md:16-42`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-reflection
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: this entry fixes the operative method of the appendix before the four reflection-pairs are applied.
  - sourceClaimIds: [`kant-trans-principles-reflection-source-c1`, `kant-trans-principles-reflection-source-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-c1`]

Review outcome:

- review_pending
- notes: this is the methodological hinge of the whole appendix.

### Entry kant-trans-principles-reflection-pairs — `Reflection`: the four concepts of reflection

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:44-112`
- summary: Identity/difference, agreement/opposition, inner/outer, and matter/form each receive different objective meaning depending on whether the compared objects are taken as appearances or as objects of pure understanding.

Key points: (KeyPoint)

- k1. Numerical difference in appearance can arise from spatial place.
- k2. Phenomenal realities can stand in real opposition.
- k3. Matter as appearance is known through relational forces, not noumenal simplicity.
- k4. In sensible intuition form precedes matter.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-pairs-c1
  - subject: four_concepts_of_reflection
  - predicate: receive_different_objective_meaning_in
  - object: pure_understanding_and_sensibility_so_that_same_comparison_title_cannot_be_used_univocally_across_both
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:44-112`

- c2. id: kant-trans-principles-reflection-pairs-c2
  - subject: matter_as_appearance
  - predicate: has
  - object: only_comparatively_internal_relational_determinations_and_not_the_noumenal_simple_inner_postulated_by_intellectualism
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-reflection.md:71-83`
    - `principles-reflection.md:105-112`

Relations: (Relation)

- r1. type: supports
  - targetEntryId: kant-trans-principles-reflection
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: this entry gives the chapter's concrete comparative cases before the larger Leibniz diagnosis is drawn.
  - sourceClaimIds: [`kant-trans-principles-reflection-pairs-c1`, `kant-trans-principles-reflection-pairs-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-c1`, `kant-trans-principles-reflection-c2`]

Review outcome:

- review_pending
- notes: the inner/outer and matter/form sections are kept close because they feed directly into the monad critique.

### Entry kant-trans-principles-reflection-leibniz — `Remark`: amphiboly and the intellectualization of appearances

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:114-279`
  - `principles-reflection.md:289-342`
- summary: Kant argues that Leibniz's entire intellectual system results from comparing appearances merely through concepts without determining their transcendental place, thereby turning analytic rules of concept-comparison into false principles about nature and things.

Key points: (KeyPoint)

- k1. Transcendental topic would have blocked the error.
- k2. Leibniz treated sensibility as confused understanding.
- k3. Identity of indiscernibles, harmony, monadology, and intellectualized space/time arise from that mistake.
- k4. The resulting principles are not laws of nature but only analytic rules of concept-comparison.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-leibniz-c1
  - subject: transcendental_amphiboly
  - predicate: consists_in
  - object: confusing_pure_object_of_understanding_with_appearance_by_omitting_transcendental_reflection_on_their_cognitive_place
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-reflection.md:125-158`

- c2. id: kant-trans-principles-reflection-leibniz-c2
  - subject: leibnizian_intellectual_system
  - predicate: arises_from
  - object: extension_of_pure_conceptual_comparison_to_objects_of_sense_and_resulting_intellectualization_of_appearances
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:159-279`
    - `principles-reflection.md:289-342`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-trans-principles-reflection
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: this entry states the chapter's main historical-philosophical diagnosis.
  - sourceClaimIds: [`kant-trans-principles-reflection-leibniz-c1`, `kant-trans-principles-reflection-leibniz-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-c2`]

Review outcome:

- review_pending
- notes: this entry fixes the chapter as a critique of method, not just of a few isolated theses.

### Entry kant-trans-principles-reflection-object — `Remark`: two object-in-general registers and the limit of noumenon

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:257-267`
  - `principles-reflection.md:343-398`
  - `principles-reflection.md:399-405`
- summary: The appendix reopens object in general in two linked but distinct registers: first as the same transcendental object-thought fixed in the ground chapter, where lack of sensible determination makes comparison contradictory or empty, and then as object in general taken problematically as the highest heading prior to the division between something and nothing.

Key points: (KeyPoint)

- k1. The first object-in-general register repeats the ground chapter's transcendental object-thought and is unusable without sensible or intellectual specification.
- k2. Purely intelligible objects through categories alone are impossible for us.
- k3. This transcendental object-thought remains indeterminate for us and does not become a positively cognized noumenon.
- k4. Object in general is also the highest problematic concept before the something/nothing division.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-object-c1
  - subject: object_in_general_in_transcendental_sense
  - predicate: is
  - object: indeterminate_transcendental_object_thought_that_becomes_contradictory_or_empty_if_not_thought_under_sensible_conditions_of_intuition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:257-267`
    - `principles-reflection.md:375-385`
    - `principles-reflection.md:387-398`

- c2. id: kant-trans-principles-reflection-object-c2
  - subject: concept_of_object_in_general_taken_problematically
  - predicate: functions_as
  - object: highest_formal_heading_prior_to_division_of_something_and_nothing_and_not_as_positive_cognition_of_noumenal_object
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-reflection.md:399-405`

Relations: (Relation)

- r1. type: follows_from
  - targetEntryId: kant-trans-principles-ground-object
  - targetWorkbook: `PRINCIPLES-GROUND-WORKBOOK.md`
  - note: this entry extends the ground chapter's analysis of transcendental object-thought by showing how the same indeterminate object in general reappears inside the critique of reflection.
  - sourceClaimIds: [`kant-trans-principles-reflection-object-c1`, `kant-trans-principles-reflection-object-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-ground-object-c1`, `kant-trans-principles-ground-object-c2`]

- r2. type: clarifies
  - targetEntryId: kant-trans-principles-reflection
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: this entry isolates the double appearance of object in general that can otherwise blur into noumenon-talk.
  - sourceClaimIds: [`kant-trans-principles-reflection-object-c1`, `kant-trans-principles-reflection-object-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-c2`]

Review outcome:

- review_pending
- notes: this entry keeps the first register tied to transcendental object-thought and the second to the highest problematic heading; the mechanism/chemism mapping is not asserted because the appendix itself does not name that pair.

### Entry kant-trans-principles-reflection-nothing — `The Four Senses of Nothing`

- sourceFiles:
  - `principles-reflection.md`
  - `PRINCIPLES-REFLECTION-DISTILLATION.md`
- lineSpans:
  - `principles-reflection.md:407-437`
- summary: Kant closes the appendix by dividing “nothing” under the highest problematic heading of object in general, distinguishing empty concept, privative absence, empty intuition-form, and impossible contradiction.

Key points: (KeyPoint)

- k1. Noumena without corresponding intuition exemplify an empty concept without object.
- k2. Privation differs from contradiction.
- k3. Pure space and time are forms, not themselves intuited objects.
- k4. Contradictory concepts are impossible objects rather than merely empty ones.

Claims: (Claim)

- c1. id: kant-trans-principles-reflection-nothing-c1
  - subject: fourfold_division_of_nothing
  - predicate: orders
  - object: empty_concept_without_object_privative_nothing_empty_form_of_intuition_and_self_contradictory_impossible_object
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `principles-reflection.md:407-437`

- c2. id: kant-trans-principles-reflection-nothing-c2
  - subject: noumenon_in_this_closing_table
  - predicate: exemplifies
  - object: concept_without_object_that_is_not_on_that_ground_asserted_to_be_absolutely_impossible
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `principles-reflection.md:409-412`

Relations: (Relation)

- r1. type: supports
  - targetEntryId: kant-trans-principles-reflection-object
  - targetWorkbook: `PRINCIPLES-REFLECTION-WORKBOOK.md`
  - note: the table of nothing clarifies how object in general is still being handled problematically rather than positively.
  - sourceClaimIds: [`kant-trans-principles-reflection-nothing-c1`, `kant-trans-principles-reflection-nothing-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-trans-principles-reflection-object-c1`, `kant-trans-principles-reflection-object-c2`]

Review outcome:

- review_pending
- notes: the appendix ends in a negative taxonomy, not a new ontology.
