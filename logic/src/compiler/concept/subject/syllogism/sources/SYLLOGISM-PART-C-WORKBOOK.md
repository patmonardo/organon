# Syllogism Part C Workbook

Part: `C. THE SYLLOGISM OF NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
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

- Keep two-level ID system aligned with Parts A/B:
  - Level 1 SubTopic markers: `syl-nes-a`, `syl-nes-b`, `syl-nes-c`
  - Level 2 numbered Entries: `syl-nes-<letter>-<nnn>`
- Use explicit source numerals where present; for `c` (no numeric labels), use paragraph-cluster decomposition.
- Keep transition continuity from Part B (`syl-ref-c-003`) and forward endpoint toward objectivity.

### Entry syl-nes-001 — Necessity setup: objective universal middle and contentful identity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 4
- lineEnd: 39

Summary:

The opening defines necessity-syllogism as contentful identity: middle term is objective universality/genus, and realization requires form and content to coincide.

Key points: (KeyPoint)

- k1. Middle develops from determinate universality to objective universality as genus.
- k2. Inner identity of extremes is carried by middle’s content-determinations.
- k3. Initial necessity is still immediate/formal and must become posited-form connection.

Claims: (Claim)

- c1. id: syl-nes-001-c1
  - subject: mediating_middle
  - predicate: is_determined_as
  - object: objective_universality_genus
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-12] objective universality and genus statement.

- c2. id: syl-nes-001-c2
  - subject: syllogism_of_necessity
  - predicate: is
  - object: immanent_reflection_of_extreme_determinateness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [17-24] inner identity in middle.

- c3. id: syl-nes-001-c3
  - subject: realization_of_necessity_syllogism
  - predicate: requires
  - object: posited_form_connection_of_totality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [31-39] realization requirement.

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: syl-ref-c-003
  - note: necessity opening presupposes higher universality result from reflection.
  - sourceClaimIds: [`syl-nes-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syl-ref-c-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-nes-a
  - note: setup passes into categorical syllogism.
  - sourceClaimIds: [`syl-nes-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part C opening stabilized.

### Entry syl-nes-a — Marker `a`: the categorical syllogism

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 40
- lineEnd: 200

Summary:

Categorical syllogism introduces necessity through substantial identity of terms but still contains subjective immediacy/contingency, requiring transition to hypothetical form.

Key points: (KeyPoint)

- k1. Categorical form is first syllogism of necessity.
- k2. It overcomes formal-existence contingency through essential middle content.
- k3. Residual immediacy of singular and extremes makes identity only inner/formal.
- k4. This determines transition to hypothetical syllogism.

Claims: (Claim)

- c1. id: syl-nes-a-c1
  - subject: categorical_syllogism
  - predicate: is
  - object: first_syllogism_of_necessity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [42-57] first necessity-form via substance.

- c2. id: syl-nes-a-c2
  - subject: categorical_middle
  - predicate: is_determined_as
  - object: objective_universal_genus_specific_difference
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [70-90] universal/specific-difference structure.

- c3. id: syl-nes-a-c3
  - subject: categorical_syllogism
  - predicate: transitions_to
  - object: hypothetical_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [189-200] explicit transition sentence.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-a-001
  - note: marker to subsection `1`.
  - sourceClaimIds: [`syl-nes-a-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syl-nes-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-nes-b
  - note: marker handoff to hypothetical syllogism.
  - sourceClaimIds: [`syl-nes-a-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syl-nes-b-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for `a` stabilized.

### Entry syl-nes-a-001 — Categorical subsection 1: necessity through substance/concept

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 42
- lineEnd: 90

Summary:

Subsection 1 establishes categorical necessity by concept-elevated substance, with universality/singularity extremes mediated via specific difference.

Key points: (KeyPoint)

- k1. Categorical premises derive from categorical judgment and objective universality.
- k2. Substance elevated into concept supplies necessity-form.
- k3. Extremes are universality and singularity, tied through specific-difference middle.

Claims: (Claim)

- c1. id: syl-nes-a-001-c1
  - subject: categorical_syllogism
  - predicate: takes_from
  - object: categorical_judgment_with_objective_universal_middle
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [42-49] initial setup.

- c2. id: syl-nes-a-001-c2
  - subject: concept_elevated_substance
  - predicate: determines
  - object: necessity_structure_of_terms
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [56-69] substance-as-universal under concept.

- c3. id: syl-nes-a-001-c3
  - subject: universality_singularity_relation
  - predicate: is_structured_by
  - object: specific_difference
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [70-90] specific-difference articulation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-a-002
  - note: subsection `1` moves to formal-schema clarification without infinite regress.
  - sourceClaimIds: [`syl-nes-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syl-nes-a-002 — Categorical subsection 2: no contingency-regress, objective identity begins

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
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

- c1. id: syl-nes-a-002-c1
  - subject: categorical_necessity_form
  - predicate: appears_as
  - object: S_P_U
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [91-95] schema placement.

- c2. id: syl-nes-a-002-c2
  - subject: essential_middle_content
  - predicate: removes
  - object: contingency_and_infinite_progression_demand
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [96-113] no arbitrary connection/proof regress.

- c3. id: syl-nes-a-002-c3
  - subject: terms_in_categorical_syllogism
  - predicate: share
  - object: one_substantial_essence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [114-139] identity-through-terms and objectivity beginning.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-a-003
  - note: objective beginning transitions to residual-subjectivity diagnosis.
  - sourceClaimIds: [`syl-nes-a-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syl-nes-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syl-nes-a-003 — Categorical subsection 3: residual immediacy and transition to hypothetical

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 140
- lineEnd: 200

Summary:

Subsection 3 diagnoses residual subjectivity: substantial identity is not yet form-identity, singular immediacy remains contingent, and categorical form transitions to hypothetical.

Key points: (KeyPoint)

- k1. Identity is still inner necessity, not fully posited form-identity.
- k2. Singular immediacy leaves contingency in subsumption and concrete content.
- k3. Therefore necessity determines itself as hypothetical syllogism.

Claims: (Claim)

- c1. id: syl-nes-a-003-c1
  - subject: categorical_identity
  - predicate: remains
  - object: substantial_inner_bond_not_form_identity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [140-149] identity still inner/necessary.

- c2. id: syl-nes-a-003-c2
  - subject: singular_immediacy
  - predicate: posits
  - object: contingency_of_extreme_relations
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [150-187] contingency and indifferent concrete existence.

- c3. id: syl-nes-a-003-c3
  - subject: syllogism_of_necessity
  - predicate: determines_itself_as
  - object: hypothetical_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [189-200] explicit concluding transition.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-nes-a
  - note: subsection completion grounds marker-level transition claim.
  - sourceClaimIds: [`syl-nes-a-003-c1`, `syl-nes-a-003-c2`, `syl-nes-a-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-nes-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-nes-b-001
  - note: direct handoff to hypothetical subsection `1`.
  - sourceClaimIds: [`syl-nes-a-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syl-nes-b — Marker `b`: the hypothetical syllogism

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 201
- lineEnd: 364

Summary:

Hypothetical syllogism explicitly combines conditional necessity with immediate being, then shows mediator/mediated identity that culminates in disjunctive form.

Key points: (KeyPoint)

- k1. Hypothetical judgment gives necessary connection without immediate term-being.
- k2. Syllogism adds immediate being while preserving mediated unity.
- k3. Conditioned relation resolves into identity of mediator and necessary.
- k4. This determination is disjunctive syllogism.

Claims: (Claim)

- c1. id: syl-nes-b-c1
  - subject: hypothetical_syllogism
  - predicate: adds
  - object: immediacy_to_hypothetical_connection
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [203-220] if-then plus immediate minor and conclusion.

- c2. id: syl-nes-b-c2
  - subject: hypothetical_mediation
  - predicate: is
  - object: negative_unity_translating_condition_to_conditioned
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [227-329] condition relation and mediating negativity.

- c3. id: syl-nes-b-c3
  - subject: hypothetical_syllogism
  - predicate: transitions_to
  - object: disjunctive_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [330-364] explicit transition sentence.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-b-001
  - note: marker to subsection `1`.
  - sourceClaimIds: [`syl-nes-b-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syl-nes-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-nes-c
  - note: marker handoff to disjunctive syllogism.
  - sourceClaimIds: [`syl-nes-b-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syl-nes-c-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for `b` stabilized.

### Entry syl-nes-b-001 — Hypothetical subsection 1: conditional bond plus immediate minor

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 203
- lineEnd: 226

Summary:

Subsection 1 states hypothetical judgment as necessary bond without term-being and then adds immediate minor-premise being in syllogistic form.

Key points: (KeyPoint)

- k1. Hypothetical judgment posits connection, not term existence.
- k2. Syllogism adds immediate being of A.
- k3. Conclusion is mediated unity, not abstract copula only.

Claims: (Claim)

- c1. id: syl-nes-b-001-c1
  - subject: hypothetical_judgment
  - predicate: contains
  - object: necessary_connection_without_immediate_being
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [203-210] conditional form.

- c2. id: syl-nes-b-001-c2
  - subject: hypothetical_syllogism
  - predicate: adds
  - object: immediate_minor_A_is
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [211-218] explicit syllogistic addition.

- c3. id: syl-nes-b-001-c3
  - subject: conclusion_B_is
  - predicate: is
  - object: accomplished_mediating_unity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [219-226] conclusion characterization.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-b-002
  - note: subsection `1` transitions to condition/conditioned mediation analysis.
  - sourceClaimIds: [`syl-nes-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `1` stabilized.

### Entry syl-nes-b-002 — Hypothetical subsection 2: condition relation and mediating negativity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
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

- c1. id: syl-nes-b-002-c1
  - subject: condition_relation
  - predicate: corresponds_to
  - object: hypothetical_syllogism_structure
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [227-268] relation analysis.

- c2. id: syl-nes-b-002-c2
  - subject: mediating_term
  - predicate: is
  - object: self_referring_negative_unity_as_activity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [279-309] mediating means and activity.

- c3. id: syl-nes-b-002-c3
  - subject: conclusion
  - predicate: posits
  - object: identity_of_mediator_and_mediated
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [322-329] identity statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-b-003
  - note: subsection `2` transitions to explicit disjunctive determination.
  - sourceClaimIds: [`syl-nes-b-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`syl-nes-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `2` stabilized.

### Entry syl-nes-b-003 — Hypothetical subsection 3: form-necessity unity and disjunctive result

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 330
- lineEnd: 364

Summary:

Subsection 3 shows the unity of form-connectedness and necessary content, where difference becomes empty and mediation determines itself as disjunctive form.

Key points: (KeyPoint)

- k1. Necessary connection appears as negative form-unity.
- k2. Necessity and necessary merge into reflected identity.
- k3. This determines the syllogism as disjunctive.

Claims: (Claim)

- c1. id: syl-nes-b-003-c1
  - subject: hypothetical_syllogism
  - predicate: displays
  - object: form_connectedness_of_necessity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [330-337] first display statement.

- c2. id: syl-nes-b-003-c2
  - subject: unity_in_hypothetical
  - predicate: is
  - object: identity_of_content_and_form_activity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [338-354] empty name difference and reflected unity.

- c3. id: syl-nes-b-003-c3
  - subject: mediated_determination
  - predicate: is
  - object: disjunctive_syllogism
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [355-364] explicit concluding sentence.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-nes-b
  - note: subsection completion grounds marker-level transition.
  - sourceClaimIds: [`syl-nes-b-003-c1`, `syl-nes-b-003-c2`, `syl-nes-b-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-nes-b-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-nes-c-001
  - note: direct handoff to disjunctive opening.
  - sourceClaimIds: [`syl-nes-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3` stabilized.

### Entry syl-nes-c — Marker `c`: the disjunctive syllogism

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 365
- lineEnd: 537

Summary:

The disjunctive syllogism realizes total concept-form in the middle, sublates formal mediation-distinction, and concludes in objectivity as concept restored in its otherness.

Key points: (KeyPoint)

- k1. Middle is universality replete with universality/particularity/singularity.
- k2. Disjunctive structure unites positive genus-identity with negative exclusion.
- k3. Mediating and mediated distinction collapses as middle contains complete determination.
- k4. Concept realizes itself as objectivity.

Claims: (Claim)

- c1. id: syl-nes-c-c1
  - subject: disjunctive_middle
  - predicate: is
  - object: developed_objective_universality_totality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [365-385] total middle-term characterization.

- c2. id: syl-nes-c-c2
  - subject: disjunctive_inference
  - predicate: posits
  - object: truth_of_hypothetical_as_unity_of_mediator_and_mediated
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [411-431] truth of hypothetical and non-syllogistic completion claim.

- c3. id: syl-nes-c-c3
  - subject: realized_concept
  - predicate: is
  - object: objectivity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [536-537] final objectivity conclusion.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-c-001
  - note: marker to first disjunctive block.
  - sourceClaimIds: [`syl-nes-c-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`syl-nes-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-obj-001
  - note: Part C endpoint transitions from necessity-syllogism into objectivity.
  - sourceClaimIds: [`syl-nes-c-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker-level entry for `c` stabilized.

### Entry syl-nes-c-001 — Disjunctive block 1: totalized middle and explicit either-or form

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 365
- lineEnd: 431

Summary:

The first disjunctive block details total middle-form and shows conclusion as truth of hypothetical through explicit unity of mediator and mediated.

Key points: (KeyPoint)

- k1. Middle includes genus identity and differentiated species-totality.
- k2. Either-or carries both exclusion and self-referring singular determinacy.
- k3. A remains subject across premises and conclusion as universal/determinate/excluding.

Claims: (Claim)

- c1. id: syl-nes-c-001-c1
  - subject: disjunctive_middle
  - predicate: includes
  - object: universal_sphere_and_negative_exclusion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [365-396] middle and either-or articulation.

- c2. id: syl-nes-c-001-c2
  - subject: A_in_disjunctive_structure
  - predicate: is
  - object: universal_then_determinate_then_excluding_singular
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [405-421] A’s formal positions.

- c3. id: syl-nes-c-001-c3
  - subject: disjunctive_syllogism
  - predicate: posits
  - object: unity_of_mediator_and_mediated
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [422-431] explicit truth-of-hypothetical statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-c-002
  - note: first block transitions to formalism-sublation analysis.
  - sourceClaimIds: [`syl-nes-c-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `1` stabilized.

### Entry syl-nes-c-002 — Disjunctive block 2: sublation of formalism and completed middle

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 432
- lineEnd: 499

Summary:

The second disjunctive block shows formalism/subjectivity sublated because complete form-determination is posited in middle-term identity.

Key points: (KeyPoint)

- k1. Positive content-identity and form-identity coincide.
- k2. Distinction of middle versus extremes loses independent determinacy.
- k3. Formalism of syllogistic subjectivity is sublated.

Claims: (Claim)

- c1. id: syl-nes-c-002-c1
  - subject: disjunctive_unity
  - predicate: is
  - object: identity_of_content_and_form
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [432-447] content/form identity statements.

- c2. id: syl-nes-c-002-c2
  - subject: middle_extreme_difference
  - predicate: falls_away_in
  - object: totality_of_concept_determination
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [448-457] mediating/mediated distinction collapse.

- c3. id: syl-nes-c-002-c3
  - subject: syllogistic_formalism
  - predicate: is_sublated_as
  - object: subjectivity_of_concept
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [458-499] explicit formalism/subjectivity sublation.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-nes-c-003
  - note: sublation analysis transitions to final objectivity conclusion.
  - sourceClaimIds: [`syl-nes-c-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`syl-nes-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `2` stabilized.

### Entry syl-nes-c-003 — Disjunctive block 3: realized concept and emergence of objectivity

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/necessity.txt`
- lineStart: 500
- lineEnd: 537

Summary:

The final block concludes that mediation-sublation yields immediate-being identical with mediation, i.e., concept restored in otherness as objectivity.

Key points: (KeyPoint)

- k1. Syllogistic movement sublates mediation where each term is only through another.
- k2. Resulting immediacy is mediation-restored identity.
- k3. This being is objectivity.

Claims: (Claim)

- c1. id: syl-nes-c-003-c1
  - subject: syllogistic_result
  - predicate: is
  - object: immediacy_emerging_through_sublated_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [519-529] immediacy-through-sublation statement.

- c2. id: syl-nes-c-003-c2
  - subject: restored_concept
  - predicate: is
  - object: identity_of_being_and_mediation_in_otherness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [529-536] restored concept formulation.

- c3. id: syl-nes-c-003-c3
  - subject: resulting_being
  - predicate: is
  - object: objectivity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [536-537] explicit final sentence.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: syl-nes-c
  - note: final block grounds marker-level endpoint claim.
  - sourceClaimIds: [`syl-nes-c-003-c1`, `syl-nes-c-003-c2`, `syl-nes-c-003-c3`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`syl-nes-c-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: syl-obj-001
  - note: explicit handoff from necessity-syllogism to objectivity placeholder.
  - sourceClaimIds: [`syl-nes-c-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: []
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: disjunctive block `3` stabilized; Part C first-pass decomposition complete.
