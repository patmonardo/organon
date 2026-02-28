# Syllogism Part B Workbook

Part: `B. THE SYLLOGISM OF REFLECTION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `reflection.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries must follow complete sentence groups (no mid-sentence start/end).

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-21 (first Reflection pass)

Scope:

- file: `reflection.txt`
- fixed range: lines `2-542`
- pass policy: full first decomposition by source-given marker (`a/b/c`) and numbered subsections (`1/2/3`).

Decision:

- Keep marker-free numbered IDs aligned with Part A: `syllo-ref-<letter>-<nnn>`.
- Treat source numeric subdivisions as direct Entry boundaries.
- Keep transition continuity from Part A (`syllo-exi-d-003`) and forward into Part C (`syllo-nec-001`).

### Entry syllo-ref-001 — Reflection setup: middle as totality and first concrete form-determinacy

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 4
- lineEnd: 60

Summary:

The opening defines the syllogism of reflection as first concrete form-determinacy, with middle term expanded from abstract particularity into totality.

Key points: (KeyPoint)

- k1. Qualitative syllogism sublates abstract term-isolation into reflective connectedness.
- k2. Middle term is now totality: singularity, allness, and genus-ground.
- k3. This first reflection form is named the syllogism of allness.

Claims: (Claim)

- c1. id: syllo-ref-001-c1
  - subject: syllogism_of_reflection
  - predicate: posits
  - object: terms_as_concrete_determinateness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-16] mediated necessary connection of terms.

- c2. id: syllo-ref-001-c2
  - subject: middle_term
  - predicate: is_determined_as
  - object: totality_of_singularity_allness_and_genus
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [44-52] explicit triadic content of middle.

- c3. id: syllo-ref-001-c3
  - subject: first_reflection_syllogism
  - predicate: is_named
  - object: syllogism_of_allness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [59-60] explicit naming.

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: syllo-exi-d-003
  - note: reflection setup presupposes Part A transition from formal existence syllogism.
  - sourceClaimIds: [`syllo-ref-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syllo-exi-d-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syllo-ref-a-001
  - note: setup passes into allness subsection `1`.
  - sourceClaimIds: [`syllo-ref-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-ref-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part B opening stabilized as foundation entry.

### Entry syllo-ref-a-001 — Allness subsection 1: external universality despite concrete middle

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 63
- lineEnd: 117

Summary:

Subsection 1 shows allness upgrades middle concretion but retains external universality and reflective contingency.

Key points: (KeyPoint)

- k1. Middle is concrete totality, satisfying a conceptual requirement.
- k2. Universality remains external reflection, not negation-of-negation.
- k3. Concrete totality restricts arbitrary predicates versus qualitative syllogism.

Claims: (Claim)

- c1. id: syllo-ref-a-001-c1
  - subject: allness_middle
  - predicate: is
  - object: concrete_not_abstract_particularity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [65-71] middle developed into moments.

- c2. id: syllo-ref-a-001-c2
  - subject: allness_universality
  - predicate: remains
  - object: external_reflective_universality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [72-83] not yet concept universality.

- c3. id: syllo-ref-a-001-c3
  - subject: concreted_middle_term
  - predicate: permits_only
  - object: predicates_commensurate_with_concrete_totality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [108-117] concrete-allness constraint.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-a-002
  - note: from reflective perfection to illusion/tautological presupposition.
  - sourceClaimIds: [`syllo-ref-a-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`syllo-ref-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syllo-ref-a-002 — Allness subsection 2: reflective illusion and presupposed conclusion

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 118
- lineEnd: 153

Summary:

Subsection 2 argues allness is illusory because the major already contains and presupposes the conclusion.

Key points: (KeyPoint)

- k1. “All singulars” already includes predicate attributed in conclusion.
- k2. Major premise validity depends on prior conclusion validity.
- k3. Counter-instance possibility shows circular dependence.

Claims: (Claim)

- c1. id: syllo-ref-a-002-c1
  - subject: reflective_perfection_of_allness
  - predicate: is
  - object: mere_semblance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [118-126] explicit illusion claim.

- c2. id: syllo-ref-a-002-c2
  - subject: major_premise_in_allness
  - predicate: presupposes
  - object: conclusion
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [127-136] major already contains conclusion.

- c3. id: syllo-ref-a-002-c3
  - subject: all_humans_major_premise_example
  - predicate: depends_on
  - object: prior_correctness_of_singular_conclusion
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [138-153] Gaius example and counter-instance logic.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-a-003
  - note: illusion diagnosis transitions to explicit induction move.
  - sourceClaimIds: [`syllo-ref-a-002-c2`, `syllo-ref-a-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`syllo-ref-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syllo-ref-a-003 — Allness subsection 3: subjective semblance and transition to induction

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 154
- lineEnd: 186

Summary:

Subsection 3 internalizes the earlier regress result and concludes that singularity as such must be posited as middle, i.e., induction.

Key points: (KeyPoint)

- k1. Reflection syllogism makes premise-conclusion presupposition explicit.
- k2. Allness-inference is an external, empty reflective semblance.
- k3. Required explicit middle is singularity-as-such, yielding induction.

Claims: (Claim)

- c1. id: syllo-ref-a-003-c1
  - subject: syllogism_of_reflection
  - predicate: posits
  - object: presupposition_of_conclusion_in_premise
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [154-163] explicit presupposition structure.

- c2. id: syllo-ref-a-003-c2
  - subject: essence_of_allness_inference
  - predicate: rests_on
  - object: subjective_singularity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [164-176] subjective singularity as real base.

- c3. id: syllo-ref-a-003-c3
  - subject: required_middle_term
  - predicate: is
  - object: singularity_as_such_leading_to_induction
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [177-186] explicit induction conclusion.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-b-001
  - note: direct handoff to induction subsection `1`.
  - sourceClaimIds: [`syllo-ref-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-ref-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syllo-ref-b-001 — Induction subsection 1: completed singularity in U-S-P configuration

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 189
- lineEnd: 216

Summary:

Subsection 1 defines induction as second-figure form where middle is completed singularity bearing universality externally.

Key points: (KeyPoint)

- k1. Induction maps onto U-S-P with completed singularity.
- k2. One extreme is universal predicate common to singulars.
- k3. Other extreme is genus exhausted in collection of singulars/species.

Claims: (Claim)

- c1. id: syllo-ref-b-001-c1
  - subject: induction_middle
  - predicate: is
  - object: completed_singularity_with_external_universality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [189-196] schema and middle specification.

- c2. id: syllo-ref-b-001-c2
  - subject: induction_extreme_one
  - predicate: is
  - object: predicate_common_to_all_singulars
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [197-203] common predicate side.

- c3. id: syllo-ref-b-001-c3
  - subject: induction_extreme_two
  - predicate: is
  - object: immediate_genus_exhausted_in_collection
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [204-216] genus/collection description and configuration.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-b-002
  - note: formal deficiency correction and experiential status follow in subsection `2`.
  - sourceClaimIds: [`syllo-ref-b-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syllo-ref-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syllo-ref-b-002 — Induction subsection 2: corrected formal deficiency and experiential meaning

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 217
- lineEnd: 253

Summary:

Subsection 2 removes second-figure formal deficiency via extension-equality and determines induction as syllogism of experience.

Key points: (KeyPoint)

- k1. U-S-P deficiency is removed in induction by “all singulars” middle.
- k2. Equality of extension posits reflective-form indifference.
- k3. Induction is subjective gathering into genus plus universal mark.

Claims: (Claim)

- c1. id: syllo-ref-b-002-c1
  - subject: induction
  - predicate: eliminates
  - object: second_figure_deficiency
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [217-226] elimination claim and identity of extensions.

- c2. id: syllo-ref-b-002-c2
  - subject: formal_indifference_result
  - predicate: is_posited_through
  - object: equality_of_extension
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [227-239] indifferent form-difference statement.

- c3. id: syllo-ref-b-002-c3
  - subject: induction
  - predicate: has_meaning
  - object: experience_as_subjective_collection_and_objective_mark_relation
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [240-253] experiential and objective significance statements.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-b-003
  - note: experiential meaning transitions to critique of subjective incompleteness.
  - sourceClaimIds: [`syllo-ref-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-ref-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syllo-ref-b-003 — Induction subsection 3: problematic infinity and truth in analogy

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 254
- lineEnd: 321

Summary:

Subsection 3 shows induction remains subjective and problematic through infinite-task completeness, while presupposing in-itself universality that yields analogy.

Key points: (KeyPoint)

- k1. Collecting all singulars is external reflection and endless task.
- k2. Inductive conclusion remains problematic under bad infinity.
- k3. Truth lies in singularity immediately universal, i.e., analogy.

Claims: (Claim)

- c1. id: syllo-ref-b-003-c1
  - subject: induction
  - predicate: remains
  - object: subjective_syllogism_of_external_collection
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [254-262] subjective status and external allness.

- c2. id: syllo-ref-b-003-c2
  - subject: inductive_unity_of_singularity_and_universality
  - predicate: is
  - object: problematic_ought_under_bad_infinity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [263-277] bad infinity and problematic conclusion.

- c3. id: syllo-ref-b-003-c3
  - subject: truth_of_induction
  - predicate: is
  - object: syllogism_of_analogy
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [302-321] objective universal as true middle and explicit analogy transition.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-c-001
  - note: direct handoff to analogy subsection `1`.
  - sourceClaimIds: [`syllo-ref-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-ref-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syllo-ref-c-001 — Analogy subsection 1: universal nature as singular middle

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 324
- lineEnd: 345

Summary:

Subsection 1 defines analogy’s middle as singular taken in universal nature and exemplifies inferential transfer between singulars sharing that nature.

Key points: (KeyPoint)

- k1. Middle is concreted singular with immanent universal nature.
- k2. Other extreme shares that universal nature.
- k3. Example expresses transfer via shared nature.

Claims: (Claim)

- c1. id: syllo-ref-c-001-c1
  - subject: analogy_middle
  - predicate: is
  - object: singular_in_its_universal_nature
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [324-333] middle-term determination.

- c2. id: syllo-ref-c-001-c2
  - subject: analogy_extreme_relation
  - predicate: requires
  - object: shared_universal_nature_between_singulars
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [334-337] same universal nature condition.

- c3. id: syllo-ref-c-001-c3
  - subject: analogy_example
  - predicate: instantiates
  - object: inference_from_earth_to_moon_inhabitance
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [338-345] explicit example.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-c-002
  - note: structural definition transitions to critique of superficial similarity-form.
  - sourceClaimIds: [`syllo-ref-c-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syllo-ref-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syllo-ref-c-002 — Analogy subsection 2: critique of superficial analogy and quaternio discussion

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 346
- lineEnd: 474

Summary:

Subsection 2 rejects similarity-mark analogy as logical method, clarifies form-content relation, and diagnoses remaining immediacy despite universalized middle.

Key points: (KeyPoint)

- k1. Mere quality-similarity makes analogy superficial.
- k2. Treating syllogistic form as empirical major-content is logically vacuous.
- k3. Quaternio concern is secondary; real defect is immediate unity of singular/universal in middle.

Claims: (Claim)

- c1. id: syllo-ref-c-002-c1
  - subject: similarity_based_analogy
  - predicate: is
  - object: superficial_representation_logic
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [346-357] superficiality diagnosis.

- c2. id: syllo-ref-c-002-c2
  - subject: major_premise_form_as_content_strategy
  - predicate: is
  - object: vacuous_for_logical_assessment
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [358-406] critique of converting form into content.

- c3. id: syllo-ref-c-002-c3
  - subject: defect_of_analogy
  - predicate: lies_in
  - object: immediate_unity_of_singularity_and_universality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [438-474] quaternio discussion and immediacy defect account.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-ref-c-003
  - note: diagnosed immediacy defect transitions to explicit demand for sublation.
  - sourceClaimIds: [`syllo-ref-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-ref-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syllo-ref-c-003 — Analogy subsection 3: self-demand for mediation and rise to higher universality

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/reflection.txt`
- lineStart: 475
- lineEnd: 542

Summary:

Subsection 3 states analogy presupposes its conclusion and therefore demands sublation of singular immediacy, producing objective universality as higher universality.

Key points: (KeyPoint)

- k1. Analogy repeats premise-conclusion identity and presupposition.
- k2. Its own demand is sublation of singular immediacy in middle.
- k3. Second negation yields in-and-for-itself universality beyond external reflection.

Claims: (Claim)

- c1. id: syllo-ref-c-003-c1
  - subject: analogy_structure
  - predicate: presupposes
  - object: its_conclusion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [475-489] conclusion/premise identity and presupposition.

- c2. id: syllo-ref-c-003-c2
  - subject: demand_of_analogy
  - predicate: is
  - object: sublation_of_singularity_as_immediate
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [490-506] explicit demand and purification to objective universal.

- c3. id: syllo-ref-c-003-c3
  - subject: universality_of_reflection
  - predicate: becomes
  - object: higher_universality_through_identity_with_mediation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [507-542] second negation, identity of mediation and presupposition, rise to higher universality.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-001
  - note: Part B handoff to Part C opening placeholder.
  - sourceClaimIds: [`syllo-ref-c-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized; Part B first-pass decomposition complete.
