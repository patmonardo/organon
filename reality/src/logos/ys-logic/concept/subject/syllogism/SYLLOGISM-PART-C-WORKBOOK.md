# Syllogism Part C Workbook

Part: `C. THE SYLLOGISM OF NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `necessity.txt` as authority.
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

## Session: 2026-02-21 (first Necessity pass)

Scope:

- file: `necessity.txt`
- fixed range: lines `2-537`
- pass policy: full first decomposition from source sectioning (`a/b` with `1/2/3`) plus `c` paragraph-cluster entries.

Decision:

- Keep marker-free numbered IDs aligned with Parts A/B: `syllo-nec-<letter>-<nnn>`.
- Use explicit source numerals where present; for `c` (no numeric labels), use paragraph-cluster decomposition.
- Keep transition continuity from Part B (`syllo-ref-c-003`) and forward endpoint toward objectivity.

### Entry syllo-nec-001 — Necessity setup: objective universal middle and contentful identity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 4
- lineEnd: 39

Summary:

The opening defines necessity-syllogism as contentful identity: middle term is objective universality/genus, and realization requires form and content to coincide.

Key points: (KeyPoint)

- k1. Middle develops from determinate universality to objective universality as genus.
- k2. Inner identity of extremes is carried by middle’s content-determinations.
- k3. Initial necessity is still immediate/formal and must become posited-form connection.

Claims: (Claim)

- c1. id: syllo-nec-001-c1
  - subject: mediating_middle
  - predicate: is_determined_as
  - object: objective_universality_genus
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-12] objective universality and genus statement.

- c2. id: syllo-nec-001-c2
  - subject: syllogism_of_necessity
  - predicate: is
  - object: immanent_reflection_of_extreme_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [17-24] inner identity in middle.

- c3. id: syllo-nec-001-c3
  - subject: realization_of_necessity_syllogism
  - predicate: requires
  - object: posited_form_connection_of_totality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [31-39] realization requirement.

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: syllo-ref-c-003
  - note: necessity opening presupposes higher universality result from reflection.
  - sourceClaimIds: [`syllo-nec-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syllo-ref-c-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syllo-nec-a-001
  - note: setup passes into categorical subsection `1`.
  - sourceClaimIds: [`syllo-nec-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part C opening stabilized.

### Entry syllo-nec-a-001 — Categorical subsection 1: necessity through substance/concept

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 42
- lineEnd: 90

Summary:

Subsection 1 establishes categorical necessity by concept-elevated substance, with universality/singularity extremes mediated via specific difference.

Key points: (KeyPoint)

- k1. Categorical premises derive from categorical judgment and objective universality.
- k2. Substance elevated into concept supplies necessity-form.
- k3. Extremes are universality and singularity, tied through specific-difference middle.

Claims: (Claim)

- c1. id: syllo-nec-a-001-c1
  - subject: categorical_syllogism
  - predicate: takes_from
  - object: categorical_judgment_with_objective_universal_middle
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [42-49] initial setup.

- c2. id: syllo-nec-a-001-c2
  - subject: concept_elevated_substance
  - predicate: determines
  - object: necessity_structure_of_terms
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [56-69] substance-as-universal under concept.

- c3. id: syllo-nec-a-001-c3
  - subject: universality_singularity_relation
  - predicate: is_structured_by
  - object: specific_difference
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [70-90] specific-difference articulation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-a-002
  - note: subsection `1` moves to formal-schema clarification without infinite regress.
  - sourceClaimIds: [`syllo-nec-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syllo-nec-a-002 — Categorical subsection 2: no contingency-regress, objective identity begins

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 91
- lineEnd: 139

Summary:

Subsection 2 places categorical necessity under S-P-U while removing arbitrary-middle contingency and infinite proof regress, initiating objectivity.

Key points: (KeyPoint)

- k1. Form still appears under S-P-U.
- k2. Essential middle blocks arbitrary quality-mediation.
- k3. Conclusion is no longer presupposed as in reflection-allness.
- k4. One substantial essence runs through all three terms.

Claims: (Claim)

- c1. id: syllo-nec-a-002-c1
  - subject: categorical_necessity_form
  - predicate: appears_as
  - object: S_P_U
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [91-95] schema placement.

- c2. id: syllo-nec-a-002-c2
  - subject: essential_middle_content
  - predicate: removes
  - object: contingency_and_infinite_progression_demand
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [96-113] no arbitrary connection/proof regress.

- c3. id: syllo-nec-a-002-c3
  - subject: terms_in_categorical_syllogism
  - predicate: share
  - object: one_substantial_essence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [114-139] identity-through-terms and objectivity beginning.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-a-003
  - note: objective beginning transitions to residual-subjectivity diagnosis.
  - sourceClaimIds: [`syllo-nec-a-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syllo-nec-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syllo-nec-a-003 — Categorical subsection 3: residual immediacy and transition to hypothetical

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 140
- lineEnd: 200

Summary:

Subsection 3 diagnoses residual subjectivity: substantial identity is not yet form-identity, singular immediacy remains contingent, and categorical form transitions to hypothetical.

Key points: (KeyPoint)

- k1. Identity is still inner necessity, not fully posited form-identity.
- k2. Singular immediacy leaves contingency in subsumption and concrete content.
- k3. Therefore necessity determines itself as hypothetical syllogism.

Claims: (Claim)

- c1. id: syllo-nec-a-003-c1
  - subject: categorical_identity
  - predicate: remains
  - object: substantial_inner_bond_not_form_identity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [140-149] identity still inner/necessary.

- c2. id: syllo-nec-a-003-c2
  - subject: singular_immediacy
  - predicate: posits
  - object: contingency_of_extreme_relations
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [150-187] contingency and indifferent concrete existence.

- c3. id: syllo-nec-a-003-c3
  - subject: syllogism_of_necessity
  - predicate: determines_itself_as
  - object: hypothetical_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [189-200] explicit concluding transition.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-b-001
  - note: direct handoff to hypothetical subsection `1`.
  - sourceClaimIds: [`syllo-nec-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syllo-nec-b-001 — Hypothetical subsection 1: conditional bond plus immediate minor

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 203
- lineEnd: 226

Summary:

Subsection 1 states hypothetical judgment as necessary bond without term-being and then adds immediate minor-premise being in syllogistic form.

Key points: (KeyPoint)

- k1. Hypothetical judgment posits connection, not term existence.
- k2. Syllogism adds immediate being of A.
- k3. Conclusion is mediated unity, not abstract copula only.

Claims: (Claim)

- c1. id: syllo-nec-b-001-c1
  - subject: hypothetical_judgment
  - predicate: contains
  - object: necessary_connection_without_immediate_being
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [203-210] conditional form.

- c2. id: syllo-nec-b-001-c2
  - subject: hypothetical_syllogism
  - predicate: adds
  - object: immediate_minor_A_is
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [211-218] explicit syllogistic addition.

- c3. id: syllo-nec-b-001-c3
  - subject: conclusion_B_is
  - predicate: is
  - object: accomplished_mediating_unity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [219-226] conclusion characterization.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-b-002
  - note: subsection `1` transitions to condition/conditioned mediation analysis.
  - sourceClaimIds: [`syllo-nec-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syllo-nec-b-002 — Hypothetical subsection 2: condition relation and mediating negativity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 227
- lineEnd: 329

Summary:

Subsection 2 analyzes condition/conditioned relation as indifferent-yet-necessary content and identifies mediating singular negativity that actualizes conditions.

Key points: (KeyPoint)

- k1. Condition relation best captures this hypothetical form.
- k2. Conditions are dispersed material requiring application.
- k3. Mediating unity is active negative conceptivity.
- k4. Conclusion exhibits identity of mediating term and mediated necessary.

Claims: (Claim)

- c1. id: syllo-nec-b-002-c1
  - subject: condition_relation
  - predicate: corresponds_to
  - object: hypothetical_syllogism_structure
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [227-268] relation analysis.

- c2. id: syllo-nec-b-002-c2
  - subject: mediating_term
  - predicate: is
  - object: self_referring_negative_unity_as_activity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [279-309] mediating means and activity.

- c3. id: syllo-nec-b-002-c3
  - subject: conclusion
  - predicate: posits
  - object: identity_of_mediator_and_mediated
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [322-329] identity statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-b-003
  - note: subsection `2` transitions to explicit disjunctive determination.
  - sourceClaimIds: [`syllo-nec-b-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syllo-nec-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syllo-nec-b-003 — Hypothetical subsection 3: form-necessity unity and disjunctive result

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 330
- lineEnd: 364

Summary:

Subsection 3 shows the unity of form-connectedness and necessary content, where difference becomes empty and mediation determines itself as disjunctive form.

Key points: (KeyPoint)

- k1. Necessary connection appears as negative form-unity.
- k2. Necessity and necessary merge into reflected identity.
- k3. This determines the syllogism as disjunctive.

Claims: (Claim)

- c1. id: syllo-nec-b-003-c1
  - subject: hypothetical_syllogism
  - predicate: displays
  - object: form_connectedness_of_necessity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [330-337] first display statement.

- c2. id: syllo-nec-b-003-c2
  - subject: unity_in_hypothetical
  - predicate: is
  - object: identity_of_content_and_form_activity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [338-354] empty name difference and reflected unity.

- c3. id: syllo-nec-b-003-c3
  - subject: mediated_determination
  - predicate: is
  - object: disjunctive_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [355-364] explicit concluding sentence.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-c-001
  - note: direct handoff to disjunctive opening.
  - sourceClaimIds: [`syllo-nec-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syllo-nec-c-001 — Disjunctive block 1: totalized middle and explicit either-or form

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 365
- lineEnd: 431

Summary:

The first disjunctive block details total middle-form and shows conclusion as truth of hypothetical through explicit unity of mediator and mediated.

Key points: (KeyPoint)

- k1. Middle includes genus identity and differentiated species-totality.
- k2. Either-or carries both exclusion and self-referring singular determinacy.
- k3. A remains subject across premises and conclusion as universal/determinate/excluding.

Claims: (Claim)

- c1. id: syllo-nec-c-001-c1
  - subject: disjunctive_middle
  - predicate: includes
  - object: universal_sphere_and_negative_exclusion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [365-396] middle and either-or articulation.

- c2. id: syllo-nec-c-001-c2
  - subject: A_in_disjunctive_structure
  - predicate: is
  - object: universal_then_determinate_then_excluding_singular
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [405-421] A’s formal positions.

- c3. id: syllo-nec-c-001-c3
  - subject: disjunctive_syllogism
  - predicate: posits
  - object: unity_of_mediator_and_mediated
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [422-431] explicit truth-of-hypothetical statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-c-002
  - note: first block transitions to formalism-sublation analysis.
  - sourceClaimIds: [`syllo-nec-c-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `1` stabilized.

### Entry syllo-nec-c-002 — Disjunctive block 2: sublation of formalism and completed middle

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 432
- lineEnd: 499

Summary:

The second disjunctive block shows formalism/subjectivity sublated because complete form-determination is posited in middle-term identity.

Key points: (KeyPoint)

- k1. Positive content-identity and form-identity coincide.
- k2. Distinction of middle versus extremes loses independent determinacy.
- k3. Formalism of syllogistic subjectivity is sublated.

Claims: (Claim)

- c1. id: syllo-nec-c-002-c1
  - subject: disjunctive_unity
  - predicate: is
  - object: identity_of_content_and_form
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [432-447] content/form identity statements.

- c2. id: syllo-nec-c-002-c2
  - subject: middle_extreme_difference
  - predicate: falls_away_in
  - object: totality_of_concept_determination
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [448-457] mediating/mediated distinction collapse.

- c3. id: syllo-nec-c-002-c3
  - subject: syllogistic_formalism
  - predicate: is_sublated_as
  - object: subjectivity_of_concept
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [458-499] explicit formalism/subjectivity sublation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syllo-nec-c-003
  - note: sublation analysis transitions to final objectivity conclusion.
  - sourceClaimIds: [`syllo-nec-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syllo-nec-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `2` stabilized.

### Entry syllo-nec-c-003 — Disjunctive block 3: realized concept and emergence of objectivity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/necessity.txt`
- lineStart: 500
- lineEnd: 537

Summary:

The final block concludes that mediation-sublation yields immediate-being identical with mediation, i.e., concept restored in otherness as objectivity.

Key points: (KeyPoint)

- k1. Syllogistic movement sublates mediation where each term is only through another.
- k2. Resulting immediacy is mediation-restored identity.
- k3. This being is objectivity.

Claims: (Claim)

- c1. id: syllo-nec-c-003-c1
  - subject: syllogistic_result
  - predicate: is
  - object: immediacy_emerging_through_sublated_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [519-529] immediacy-through-sublation statement.

- c2. id: syllo-nec-c-003-c2
  - subject: restored_concept
  - predicate: is
  - object: identity_of_being_and_mediation_in_otherness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [529-536] restored concept formulation.

- c3. id: syllo-nec-c-003-c3
  - subject: resulting_being
  - predicate: is
  - object: objectivity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [536-537] explicit final sentence.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-obj-001
  - note: explicit handoff from necessity-syllogism to objectivity placeholder.
  - sourceClaimIds: [`syllo-nec-c-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `3` stabilized; Part C first-pass decomposition complete.
