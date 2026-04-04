# Kantian Logogenesis Workbook

Status: active
Authority: source-guided, distillation-assisted

## Reading note

- This is a synthetic workbook, not a replacement for the component workbooks.
- Read it to follow one Kantian path from the fourfold judged object, through Object(), and then through concept, judgment, and syllogism.
- Its task is to preserve the architectonic spine of that path: the cross-component hinges without duplicating every local doctrine from the component workbooks.
- Its main task is to track the object first as quantity, quality, relation, and modality, then as the higher Object() division into phenomenon and noumenon, and then to track matter and form as they develop into syllogistic mediation.
- Keep this file inside the Kantian graph of `kant-logic`; do not merge it with a Hegelian graph here.
- Any later dialectical comparison with Hegel belongs in a separate workbook surface.
- Module-specific detail such as the full doctrine of immediate inferences, the figures, and their reductions should stay in the component workbooks unless one of those details must be surfaced as a spine-level transition.

## Quick orientation

- First question: what is this workbook naming here?
  Answer: a Kantian logogenetic path, namely the movement from the fourfold judged object, through the Object() boundary of phenomenon and noumenon, into concept, judgment, and syllogistic mediation.
- Second question: what is being tracked most carefully?
  Answer: the judged object under quantity, quality, relation, and modality, then its Object() division into phenomenon and noumenon, and then the development of matter and form across concept, judgment, and inference.
- Third question: what is excluded from this pass?
  Answer: any Hegelian objectivity-workbook or fused comparative graph.

## Authority + format lock (must persist)

- Contract reference: `GENERAL-LOGIC-WORKBOOK-CONTRACT-V1.md`
- Working extraction references: `idea.md`, `concepts.md`, `subjectivity.md`, `concept.md`, `judgment.md`, `syllogism.md`, `PRINCIPLES-DISTILLATION.md`, `PRINCIPLES-BOUNDARY-ARGUMENT.md`, `PRINCIPLES-GROUND-WORKBOOK.md`, and `LOGOGENESIS-DISTILLATION.md`
- Upstream source authority: `idea.md`, `concepts.md`, `subjectivity.md`, `concept.md`, `judgment.md`, `syllogism.md`, `principles-ground.md`, `principles-III.md`, `principles-IV.md`
- This workbook is a cross-component synthesis scaffold for a Kantian path up to syllogism.

## Clean-room rules

- Keep the pass on the Kantian General Logic side.
- Do not collapse the sequence into a Hegelian objectivity-workbook within this file.
- Do not let this workbook become a duplicate of the Concept, Judgment, or Syllogism workbooks.
- Track matter and form separately wherever the source explicitly distinguishes them.
- Preserve the difference between immediate inference, inference of reason, and reflective inference.
- Pull in local doctrine only when it changes the architectonic spine of the path.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending`.

## Session: 2026-04-02 (Kantian logogenesis pass)

Scope:

- files:
  - `idea.md`
  - `concepts.md`
  - `subjectivity.md`
  - `concept.md`
  - `judgment.md`
  - `syllogism.md`
  - `PRINCIPLES-DISTILLATION.md`
  - `PRINCIPLES-BOUNDARY-ARGUMENT.md`
  - `PRINCIPLES-GROUND-WORKBOOK.md`
  - `principles-ground.md`
  - `principles-III.md`
  - `principles-IV.md`
  - `LOGOGENESIS-DISTILLATION.md`
- pass policy: 1 marker entry + analytic entries by matter/form transition

Decision:

- Treat this file as a Kantian Logogenesis workbook rather than as a merged Kant-Hegel artifact.
- Front-load the object as a fourfold judged object under quantity, quality, relation, and modality.
- Insert Object() as the higher boundary section that divides object in general into phenomenon and noumenon before Subjectivity proper begins.
- Preserve the object-to-concept, concept-to-judgment, and judgment-to-syllogism sequence explicitly.
- Read judgment first as the object being taken into conceptual articulation, while retaining the formal role of unity of consciousness.
- Keep the judgment families distinct rather than flattening them into categorical form alone.
- Treat the syllogism chapter as the derivational completion of the present Kantian pass.
- Reserve any dialectical recombination with Hegel for a separate workbook.

### Entry kant-subjpath-fourfold — Marker `Fourfold Object Judged Into Concept`

- sourceFiles:
  - `idea.md`
  - `concepts.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `idea.md:349-370`
  - `concepts.md:120-220`
  - `concepts.md:280-321`
  - `concepts.md:381-431`
  - `PRINCIPLES-DISTILLATION.md:60-70`
- summary: The Kantian Logogenesis path can be front-loaded by taking the object as a fourfold judged object under quantity, quality, relation, and modality; because the same understanding gives unity both to judgment and to synthesis of intuition, these four titles are the first way the object is judged into concept.

Key points: (KeyPoint)

- k1. The table of judgment gives four titles, each with three moments.
- k2. The same unity-functions first displayed in judgment also generate the categories for object-cognition.
- k3. Schematism gives the four titles temporal determinability in experience.
- k4. Relation and modality most directly prepare the later subjective path because they determine the object as lawful bearer, sequence, whole, and cognitive standing.

Claims: (Claim)

- c1. id: kant-subjpath-fourfold-c1
  - subject: judged_object
  - predicate: is_first_taken_under
  - object: quantity_quality_relation_and_modality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `idea.md:349-370`
    - `concepts.md:120-220`

- c2. id: kant-subjpath-fourfold-c2
  - subject: categories
  - predicate: arise_from
  - object: same_unity_functions_first_displayed_in_judgment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:280-321`

- c3. id: kant-subjpath-fourfold-c3
  - subject: fourfold_categorical_front_end
  - predicate: receives
  - object: temporal_schema_as_number_degree_relation_and_modality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `PRINCIPLES-DISTILLATION.md:60-70`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: kant-subjpath-quantity
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: quantity gives the first numerical unity of the judged object.
  - sourceClaimIds: [`kant-subjpath-fourfold-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`kant-subjpath-quantity-c1`]

- r2. type: unfolds_to
  - targetEntryId: kant-subjpath-quality
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: quality gives the object determinacy through reality, negation, and limitation.
  - sourceClaimIds: [`kant-subjpath-fourfold-c1`, `kant-subjpath-fourfold-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`kant-subjpath-quality-c1`]

- r3. type: unfolds_to
  - targetEntryId: kant-subjpath-relation
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: relation gives the strongest genetic bridge from judged object toward conceptual articulation.
  - sourceClaimIds: [`kant-subjpath-fourfold-c1`, `kant-subjpath-fourfold-c2`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`kant-subjpath-relation-c1`]

- r4. type: unfolds_to
  - targetEntryId: kant-subjpath-modality
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: modality fixes how the judged object stands for cognition within experience.
  - sourceClaimIds: [`kant-subjpath-fourfold-c1`, `kant-subjpath-fourfold-c3`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`kant-subjpath-modality-c1`]

Review outcome:

- review_pending
- notes: this marker inserts the fourfold judged object directly at the front of the Kantian path.

### Entry kant-subjpath-quantity — `Fourfold Object`: quantity as unity, plurality, totality

- sourceFiles:
  - `concepts.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `concepts.md:120-220`
  - `concepts.md:389-398`
  - `PRINCIPLES-DISTILLATION.md:60-70`
- summary: Quantity takes the judged object as unity, plurality, and totality; schematically it becomes number, the successive synthesis of homogeneous units, so the object is already gathered for conceptive universality.

Key points: (KeyPoint)

- k1. Quantity divides into unity, plurality, and totality.
- k2. Totality is plurality considered as a unity.
- k3. The schema of quantity is number.

Claims: (Claim)

- c1. id: kant-subjpath-quantity-c1
  - subject: quantity_of_the_judged_object
  - predicate: divides_into
  - object: unity_plurality_and_totality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:120-220`
    - `concepts.md:389-395`

- c2. id: kant-subjpath-quantity-c2
  - subject: quantity_schema
  - predicate: is
  - object: number_as_successive_synthesis_of_homogeneous_units
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `PRINCIPLES-DISTILLATION.md:63-66`

Relations: (Relation)

- r1. type: prepares
  - targetEntryId: kant-subjpath-concept
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: quantity gives the object the first gatherable shape required for universality.
  - sourceClaimIds: [`kant-subjpath-quantity-c1`, `kant-subjpath-quantity-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-subjpath-concept-c1`]

Review outcome:

- review_pending
- notes: quantity is inserted here as the numerical front edge of the judged object.

### Entry kant-subjpath-quality — `Fourfold Object`: quality as reality, negation, limitation

- sourceFiles:
  - `concepts.md`
  - `PRINCIPLES-II-DISTILLATION.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `concepts.md:120-220`
  - `concepts.md:393-399`
  - `PRINCIPLES-II-DISTILLATION.md:35-44`
  - `PRINCIPLES-DISTILLATION.md:63-66`
- summary: Quality takes the judged object as reality, negation, and limitation; limitation combines the first two, and the schema of quality gives this combination sensible force as degree or filling of time by sensation.

Key points: (KeyPoint)

- k1. Quality divides into reality, negation, and limitation.
- k2. Limitation combines reality with negation.
- k3. The real in appearance admits degree and intermediate gradations.

Claims: (Claim)

- c1. id: kant-subjpath-quality-c1
  - subject: quality_of_the_judged_object
  - predicate: divides_into
  - object: reality_negation_and_limitation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:120-220`
    - `concepts.md:393-399`

- c2. id: kant-subjpath-quality-c2
  - subject: quality_schema
  - predicate: gives
  - object: degree_of_reality_through_continuous_passage_between_reality_and_negation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `PRINCIPLES-II-DISTILLATION.md:35-44`
    - `PRINCIPLES-DISTILLATION.md:63-66`

Relations: (Relation)

- r1. type: prepares
  - targetEntryId: kant-subjpath-concept
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: quality gives the object bounded determinacy before concept takes it up as matter.
  - sourceClaimIds: [`kant-subjpath-quality-c1`, `kant-subjpath-quality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-subjpath-concept-c1`]

Review outcome:

- review_pending
- notes: quality is kept genetic here by stressing degree and limitation rather than a bare list of titles.

### Entry kant-subjpath-relation — `Fourfold Object`: relation as substance, causality, community

- sourceFiles:
  - `concepts.md`
  - `principles-III.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `concepts.md:120-220`
  - `concepts.md:381-431`
  - `principles-III.md:128-985`
  - `PRINCIPLES-DISTILLATION.md:63-66`
- summary: Relation is the richest genetic division of the judged object. It takes the object as persistence of substance, objective succession through causality, and reciprocal coexistence through community; these yield the strongest Kantian bridge from the judged object toward the later subjective path.

Key points: (KeyPoint)

- k1. Relation divides into inherence-subsistence, causality-dependence, and community.
- k2. Schematically relation becomes persistence, succession according to rule, and simultaneity according to rule.
- k3. Community is the reciprocal whole already prefigured in disjunctive division.
- k4. Relation gives the object as bearer, lawful transition, and coordinated whole.

Claims: (Claim)

- c1. id: kant-subjpath-relation-c1
  - subject: relation_of_the_judged_object
  - predicate: divides_into
  - object: substance_causality_and_community
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `concepts.md:120-220`
    - `concepts.md:381-406`

- c2. id: kant-subjpath-relation-c2
  - subject: relation_schema
  - predicate: determines
  - object: persistence_succession_according_to_rule_and_simultaneity_according_to_rule
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `PRINCIPLES-DISTILLATION.md:63-66`
    - `principles-III.md:128-985`

- c3. id: kant-subjpath-relation-c3
  - subject: community
  - predicate: corresponds_to
  - object: reciprocal_coordination_of_members_within_one_whole
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `concepts.md:408-431`
    - `principles-III.md:795-985`

Relations: (Relation)

- r1. type: prepares
  - targetEntryId: kant-subjpath-judgment
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: relation most strongly anticipates the later judgmental and syllogistic articulation of subject, consequence, and divided whole.
  - sourceClaimIds: [`kant-subjpath-relation-c1`, `kant-subjpath-relation-c2`, `kant-subjpath-relation-c3`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-subjpath-judgment-c2`]

Review outcome:

- review_pending
- notes: this initial pass keeps the substance-causality-community sequence explicitly Kantian while still showing why relation is the strongest genetic bridge.

### Entry kant-subjpath-modality — `Fourfold Object`: modality as possibility, actuality, necessity

- sourceFiles:
  - `concepts.md`
  - `principles-IV.md`
  - `PRINCIPLES-DISTILLATION.md`
- lineSpans:
  - `concepts.md:204-220`
  - `principles-IV.md:20-163`
  - `PRINCIPLES-DISTILLATION.md:63-66`
- summary: Modality does not add content to the judged object. It determines how the object stands for cognition as possible, actual, and necessary within experience, thereby closing the fourfold front-end without transcending the Kantian limit of possible experience.

Key points: (KeyPoint)

- k1. Modality divides into possibility, actuality, and necessity.
- k2. Modality concerns the value of the copula in relation to thinking rather than new object-content.
- k3. Objective possibility requires more than non-contradiction.
- k4. Actuality and necessity are fixed only within empirical connection.

Claims: (Claim)

- c1. id: kant-subjpath-modality-c1
  - subject: modality_of_the_judged_object
  - predicate: divides_into
  - object: possibility_actuality_and_necessity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `concepts.md:204-220`

- c2. id: kant-subjpath-modality-c2
  - subject: objective_modality
  - predicate: is_fixed_only_through
  - object: formal_conditions_of_possible_experience_perception_and_lawful_empirical_connection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-IV.md:20-163`

Relations: (Relation)

- r1. type: prepares
  - targetEntryId: kant-subjpath-judgment
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: modality fixes how the judged object counts for cognition before judgment differentiates its relation-forms explicitly.
  - sourceClaimIds: [`kant-subjpath-modality-c1`, `kant-subjpath-modality-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-subjpath-judgment-c1`]

Review outcome:

- review_pending
- notes: modality is kept Kantian here by stressing cognitive standing within experience rather than extra object-content.

### Entry kant-subjpath-object-boundary — `Object()`: grounds of division of object in general into phenomenon and noumenon

- sourceFiles:
  - `principles-ground.md`
  - `PRINCIPLES-GROUND-WORKBOOK.md`
  - `PRINCIPLES-BOUNDARY-ARGUMENT.md`
- lineSpans:
  - `principles-ground.md:172-194`
  - `principles-ground.md:364-440`
  - `PRINCIPLES-GROUND-WORKBOOK.md:174-330`
  - `PRINCIPLES-BOUNDARY-ARGUMENT.md:59-67`
- summary: Before the Kantian path enters Subjectivity proper, Object() must be fixed as the grounds of a division of object in general. Under sensible givenness the object is phenomenon for us; apart from that givenness, what remains is only indeterminate object-thought, and noumenon is admissible only problematically as a negative boundary-concept rather than as a positively cognized domain.

Key points: (KeyPoint)

- k1. If the manner of intuition is omitted, what remains is only the thought of an object in general.
- k2. That object-thought has transcendental significance but no genuine transcendental use.
- k3. Phenomenon names the object insofar as it is given under sensible intuition.
- k4. Noumenon is legitimate only in the negative sense for us; positive noumenon would require intellectual intuition.

Claims: (Claim)

- c1. id: kant-subjpath-object-boundary-c1
  - subject: object_in_general_when_intuition_is_unspecified
  - predicate: is
  - object: merely_transcendental_object_thought_and_not_determinate_cognition_of_any_actual_object
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `principles-ground.md:172-194`
    - `PRINCIPLES-BOUNDARY-ARGUMENT.md:59-61`

- c2. id: kant-subjpath-object-boundary-c2
  - subject: division_of_object_in_general_for_us
  - predicate: yields
  - object: phenomenon_under_sensible_intuition_and_noumenon_only_as_negative_problematic_boundary_concept
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `principles-ground.md:364-440`
    - `PRINCIPLES-GROUND-WORKBOOK.md:230-330`
    - `PRINCIPLES-BOUNDARY-ARGUMENT.md:61-67`

Relations: (Relation)

- r1. type: prepares
  - targetEntryId: kant-subjpath
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: Object() fixes the higher boundary division that must be in place before the Subjectivity path can begin cleanly.
  - sourceClaimIds: [`kant-subjpath-object-boundary-c1`, `kant-subjpath-object-boundary-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-subjpath-c1`]

Review outcome:

- review_pending
- notes: this entry keeps Object() as a Kantian boundary division rather than a positive noumenal science.

### Entry kant-subjpath — Marker `Kantian Logogenesis Path`

- sourceFiles:
  - `subjectivity.md`
  - `concept.md`
  - `judgment.md`
  - `syllogism.md`
- lineSpans:
  - `subjectivity.md:1-69`
  - `concept.md:1-76`
  - `judgment.md:1-93`
  - `syllogism.md:1-136`
- summary: The Kantian Logogenesis path names the movement in which the fourfold judged object passes through the Object() boundary division into phenomenon and noumenon, becomes conceptive matter, is taken into judgmental articulation, and is finally carried into lawful derivation by syllogism.

This means the present Logogenesis framework joins Kant's Transcendental Logic and General Logic in one path. The Transcendental side supplies the category-front and the Object() boundary of phenomenon and noumenon; the General Logic side supplies concept, judgment, and syllogism as the explicit judged-into-concept movement.

Because Kant derives the categories from the functions of judgment, General Logic is not an optional appendix to this path but a presupposition of it. The transcendental derivation exceeds General Logic, but it does not begin without it.

Key points: (KeyPoint)

- k1. Concept takes object as matter and universality as form.
- k1a. Object() fixes the higher division of object in general into phenomenon and noumenon prior to Subjectivity proper.
- k1b. Logogenesis here joins Kant's Transcendental Logic and General Logic in one judged-into-concept framework.
- k1c. General Logic is a presupposition of the transcendental derivation because the categories are derived from the functions of judgment.
- k2. Judgment takes the judged object into conceptual articulation and differentiates its matter and form.
- k3. Syllogism completes the subjective path by turning judgment into derivation.

Claims: (Claim)

- c1. id: kant-subjpath-c1
  - subject: subjectivity_path
  - predicate: names
  - object: kantian_logogenesis_from_object_to_syllogistic_mediation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `subjectivity.md:1-69`
    - `syllogism.md:1-136`

- c2. id: kant-subjpath-c2
  - subject: subjectivity_path
  - predicate: tracks
  - object: development_of_fourfold_object_object_boundary_matter_and_form_across_concept_judgment_and_syllogism
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - `subjectivity.md:10-69`
    - `judgment.md:24-78`
    - `syllogism.md:109-136`

- c3. id: kant-subjpath-c3
  - subject: present_logogenesis_framework
  - predicate: joins
  - object: transcendental_logic_and_general_logic_in_one_judged_into_concept_path
  - modality: interpreted
  - confidence: 0.93
  - evidence:
    - `principles-ground.md:172-194`
    - `subjectivity.md:1-69`
    - `judgment.md:1-93`
    - `syllogism.md:1-136`

Relations: (Relation)

- r1. type: unfolds_to
  - targetEntryId: kant-subjpath-concept
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: the first explicit stage is concept.
  - sourceClaimIds: [`kant-subjpath-c1`, `kant-subjpath-c3`]
  - sourceKeyPointIds: [`k1`, `k1b`]
  - targetClaimIds: [`kant-subjpath-concept-c1`]

- r2. type: unfolds_to
  - targetEntryId: kant-subjpath-judgment
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: judgment makes the conceptive unity explicit.
  - sourceClaimIds: [`kant-subjpath-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`kant-subjpath-judgment-c1`]

- r3. type: unfolds_to
  - targetEntryId: kant-subjpath-syllogism
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: syllogism completes the derivational region of the path.
  - sourceClaimIds: [`kant-subjpath-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`kant-subjpath-syllogism-c1`]

Review outcome:

- review_pending
- notes: marker entry now begins from the fourfold judged object and the Object() boundary division before the later concept-judgment-syllogism sequence.

### Entry kant-subjpath-concept — `Kantian Logogenesis Path`: object as matter, universality as form

- sourceFiles:
  - `subjectivity.md`
  - `concept.md`
- lineSpans:
  - `subjectivity.md:10-34`
  - `concept.md:1-76`
- summary: The Kantian path begins where concept takes object as matter and universality as form, then articulates that form through content and extension, subordination, and logical ascent and descent.

Key points: (KeyPoint)

- k1. The matter of concept is object.
- k2. The form of concept is universality.
- k3. The form side becomes operational through content and extension, higher and lower concepts, genus and species, and abstraction-determination.

Claims: (Claim)

- c1. id: kant-subjpath-concept-c1
  - subject: concept
  - predicate: has_matter_and_form
  - object: object_and_universality
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - `subjectivity.md:10-13`

- c2. id: kant-subjpath-concept-c2
  - subject: conceptive_form
  - predicate: develops_through
  - object: content_extension_subordination_and_generation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `concept.md:1-76`

Relations: (Relation)

- r1. type: grounds
  - targetEntryId: kant-subjpath
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: concept provides the first matter-form articulation of the path.
  - sourceClaimIds: [`kant-subjpath-concept-c1`, `kant-subjpath-concept-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`kant-subjpath-c2`]

- r2. type: transitions_to
  - targetEntryId: kant-subjpath-judgment
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: the conceptive object is carried forward into the unity of judgment.
  - sourceClaimIds: [`kant-subjpath-concept-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`kant-subjpath-judgment-c1`]

Review outcome:

- review_pending
- notes: this entry keeps the conceptive matter/form split explicit and treats the rest of the file as development of the form-side in use.

### Entry kant-subjpath-judgment — `Kantian Logogenesis Path`: judgmental unity and its three relation-families

- sourceFiles:
  - `subjectivity.md`
  - `judgment.md`
- lineSpans:
  - `subjectivity.md:35-56`
  - `judgment.md:1-78`
- summary: Judgment restates the path at a higher level: the judged object is now explicitly taken into conceptual articulation; matter becomes given representations combined in one consciousness, form becomes their belonging-to-one-consciousness, and the relation-side then differentiates that matter-form unity into categorical, hypothetical, and disjunctive families.

Key points: (KeyPoint)

- k1. Judgment in general has matter as combined representations and form as their belonging to one consciousness.
- k2. Categorical judgment specifies matter as subject and predicate and form as copula.
- k3. Hypothetical judgment specifies matter as antecedent and consequent judgments and form as consequentia.
- k4. Disjunctive judgment specifies matter as member-judgments and form as disjunction itself.

Claims: (Claim)

- c1. id: kant-subjpath-judgment-c1
  - subject: judgment
  - predicate: has_matter_and_form
  - object: combined_representations_and_belonging_to_one_consciousness
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - `subjectivity.md:35-49`

- c2. id: kant-subjpath-judgment-c2
  - subject: relation_forms_of_judgment
  - predicate: differentiate
  - object: judgmental_matter_and_form_as_categorical_hypothetical_and_disjunctive
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - `judgment.md:24-78`

Relations: (Relation)

- r1. type: specifies
  - targetEntryId: kant-subjpath-concept
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: judgment carries forward conceptive matter into explicit unity.
  - sourceClaimIds: [`kant-subjpath-judgment-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`kant-subjpath-concept-c1`]

- r2. type: transitions_to
  - targetEntryId: kant-subjpath-syllogism
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: syllogism takes these differentiated judgments into derivation.
  - sourceClaimIds: [`kant-subjpath-judgment-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`kant-subjpath-syllogism-c1`]

Review outcome:

- review_pending
- notes: this entry now reads judgment first as object judged into conceptual articulation while retaining Kant's formal account of one consciousness.

### Entry kant-subjpath-syllogism — `Kantian Logogenesis Path`: derivation and mediation

- sourceFiles:
  - `syllogism.md`
  - `SYLLOGISM-DISTILLATION.md`
- lineSpans:
  - `syllogism.md:1-136`
  - `syllogism.md:150-323`
- summary: Syllogism completes the present Kantian pass by letting judgment pass into further judgment through immediate transformation, mediate necessity, and reflective ascent; in the present reconstruction it also gathers the matter/form lifecycle of judgment into explicit rational mediation, including the explicit statement of the principle of ground on the hypothetical side and the formal ladder from copula to consequentia to disjunctio.
- summary: Syllogism completes the present Kantian pass by carrying judgment from formal mobility into explicit rational mediation; for the LG spine, what matters is the matter/form ladder from subject-predicate to ground-consequence to membra of a whole, and the formal ladder from copula to consequentia to disjunctio.

Key points: (KeyPoint)

- k1. Immediate inference alters only the form of judgments while preserving their matter.
- k2. Inference of reason has premises as matter and conclusion as form insofar as it contains the consequentia.
- k3. Categorical, hypothetical, and disjunctive syllogisms mediate through middle mark, ground-conditioned sequence, and exhaustive division respectively.
- k4. Reflective inference searches for common ground from particulars toward a universal.
- k5. On the present reconstruction, syllogistic matter advances from subject-predicate, to ground-consequence, to membra of a whole, while syllogistic form advances from copula, to consequentia, to disjunctio.
- k6. Jasche's General Logic states the principle of ground explicitly in hypothetical inference, which for the present reconstruction can be read as the General Logic side making explicit a regressive condition-seeking structure already used in Transcendental Logic.
- k7. The division of inferences of reason by inherence, ground, and logical division can be read, on the present reconstruction, as a principle of rationality: reason must mediate through mark, ground, or whole.
- k8. Mark, ground, and excluded middle function here as distinct truth-protocols internal to the Concept's mediation, so syllogism is the point at which Concept(), raised from Object(), becomes explicitly self-mediating as rational form.
- k9. The formal spine of the present Subjectivity pass runs from copula, to consequentia, to disjunctio; this is the clearest matter/form ladder to preserve in LG.
- k10. For LG, the fuller doctrine of immediate inferences, figures, and reductions remains in the component syllogism workbook unless one of those moments becomes a necessary architectonic hinge.
- k11. For implementation, Concept() should preserve internal protocols for mark-binding, ground-tracing, and whole-division rather than acting as a passive container for Object()-side content.

Claims: (Claim)

- c1. id: kant-subjpath-syllogism-c1
  - subject: syllogism
  - predicate: completes
  - object: derivational_region_of_subjective_logic
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - `syllogism.md:1-18`
    - `SYLLOGISM-DISTILLATION.md:7-19`

- c2. id: kant-subjpath-syllogism-c2
  - subject: syllogistic_mediation
  - predicate: completes
  - object: derivational_mediation_of_the_present_kantian_pass
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - `syllogism.md:109-136`
    - `syllogism.md:269-323`
    - `SYLLOGISM-DISTILLATION.md:167-205`

- c3. id: kant-subjpath-syllogism-c3
  - subject: syllogistic_matter_and_form_lifecycle
  - predicate: advances_as
  - object: subject_predicate_to_ground_consequence_to_membra_and_copula_to_consequentia_to_disjunctio
  - modality: interpreted
  - confidence: 0.9
  - evidence:
    - `judgment.md:24-78`
    - `syllogism.md:1-18`
    - `syllogism.md:109-136`
    - `SYLLOGISM-DISTILLATION.md:52-110`

- c4. id: kant-subjpath-syllogism-c4
  - subject: hypothetical_inference_in_general_logic
  - predicate: explicitly_states
  - object: principle_of_ground_as_formal_mediation_of_condition_and_conditioned
  - modality: interpreted
  - confidence: 0.92
  - evidence:
    - `syllogism.md:146-188`
    - `syllogism.md:336-373`

Relations: (Relation)

- r1. type: explicates
  - targetEntryId: kant-subjpath
  - targetWorkbook: `LOGOGENESIS-WORKBOOK.md`
  - note: syllogism gives the final subjective stage of the path.
  - sourceClaimIds: [`kant-subjpath-syllogism-c1`, `kant-subjpath-syllogism-c2`, `kant-subjpath-syllogism-c3`, `kant-subjpath-syllogism-c4`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`, `k5`, `k6`]
  - targetClaimIds: [`kant-subjpath-c1`]

- r2. type: transitions_to
  - targetEntryId: kant-subj
  - targetWorkbook: `SUBJECTIVITY-WORKBOOK.md`
  - note: this entry closes the present Kantian container without forcing a separate Hegelian continuation into the same graph.
  - sourceClaimIds: [`kant-subjpath-syllogism-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`kant-subj-c2`]

Review outcome:

- review_pending
- notes: this entry now includes a compact summary of the syllogistic matter/form lifecycle and the explicit principle-of-ground note, while keeping the stronger genetic progression marked as a present reconstruction.
- notes: this entry has been tightened to preserve LG at spine-level; fuller local syllogism doctrine remains in the component workbook family unless it becomes architectonically necessary here.
