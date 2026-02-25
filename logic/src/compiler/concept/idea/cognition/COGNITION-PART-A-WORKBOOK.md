# Cognition Part A Workbook

Part: `A. THE IDEA OF THE TRUE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `idea-of-the-true.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow coherent determination windows of complete sentence groups.

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Working template

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-23 (subspecies numeric pass)

Scope:

- file: `idea-of-the-true.txt`
- fixed range: lines `223-2197`

Decision:

- Subspecies rule enforced: lines `4-221` are treated as IDEA-level material, not Part A extraction.
- Part A starts at line `223` with `a. Analytic cognition`.
- `a-a` is sectioned by analysis.
- `a-b` follows numbered section boundaries.
- `a-c-*` is fixed to `1894-2194`; final transition to action is preserved separately at `2195-2197`.

Extraction map:

- `a-a-001`: `223-275`
- `a-a-002`: `276-418`
- `a-a-003`: `419-676`
- `a-b-001`: `759-1117`
- `a-b-002`: `1119-1483`
- `a-b-003`: `1486-1892`
- `a-c-001`: `1894-2030`
- `a-c-002`: `2031-2118`
- `a-c-003`: `2119-2194`
- transition gate: `2195-2197`

### Entry cogni-tru-a-001 — a-a-001 (analysis): formal difference and immediate principle of analytic cognition

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 223
- lineEnd: 275

Summary:

Analytic cognition is first defined against synthetic cognition and determined as immediate conceptual communication grounded in abstract identity.

Key points: (KeyPoint)

- k1. Popular known/unknown distinction of analytic vs synthetic is rejected as conceptually insufficient.
- k2. Analytic cognition is the first premise: immediate communication of the concept.
- k3. Its principle is abstract universality/identity that excludes transition to otherness.

Claims: (Claim)

- c1. id: cogni-tru-a-001-c1
  - subject: analytic_cognition
  - predicate: is_determined_as
  - object: immediate_communication_of_concept_under_abstract_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [225-246] critique of known/unknown difference.
    - [248-275] first-premise immediacy and identity principle.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-a-002
  - note: formal principle moves to detailed analysis of presupposition and positing.
  - sourceClaimIds: [`cogni-tru-a-001-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`cogni-tru-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first analysis window of analytic cognition fixed.

### Entry cogni-tru-a-002 — a-a-002 (analysis): presupposition, positing/presupposing unity, and limit of formal analysis

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 276
- lineEnd: 418

Summary:

Analytic cognition is shown as starting from concrete presupposition, transforming content into logical determination while remaining formally repetitive and non-dialectical.

Key points: (KeyPoint)

- k1. Analysis presupposes concrete material while claiming conceptual transformation.
- k2. Positing and presupposing are inseparable moments in analytic cognition.
- k3. Formal progression repeats abstractions and cannot generate genuine immanent transition.

Claims: (Claim)

- c1. id: cogni-tru-a-002-c1
  - subject: analytic_cognition
  - predicate: is_determined_as
  - object: positing_presupposing_unity_with_formal_limit
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [276-337] presupposition, conceptual products, and positing/presupposing unity.
    - [338-418] formal repetition and critique of undemonstrated synthetic progression.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-a-003
  - note: formal-limit critique advances into arithmetic/algebra exemplar and final transition marker.
  - sourceClaimIds: [`cogni-tru-a-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second analysis window of analytic cognition fixed.

### Entry cogni-tru-a-003 — a-a-003 (analysis): arithmetic exemplar, analytic method limit, and handoff to synthetic cognition

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 419
- lineEnd: 676

Summary:

Arithmetic/algebra are used to display analytic identity in operation, revealing both its power and limit, culminating in the explicit transition from analytic to synthetic cognition.

Key points: (KeyPoint)

- k1. Analytical science privileges external operations over immanent conceptual genesis.
- k2. Even advanced analysis presupposes non-analytic transitions (e.g., qualitative determinations).
- k3. The transition from analytic to synthetic is stated as necessary movement from immediacy to mediation.

Claims: (Claim)

- c1. id: cogni-tru-a-003-c1
  - subject: analytic_cognition
  - predicate: is_determined_as
  - object: identity_based_method_that_necessitates_transition_to_synthetic
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [419-649] arithmetic/algebra exemplar and differential-calculus limit discussion.
    - [651-676] explicit general transition to synthetic cognition.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-b-001
  - note: transition opens numbered synthetic movement beginning with definition.
  - sourceClaimIds: [`cogni-tru-a-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: third analysis window ends at the synthetic-cognition handoff.

### Entry cogni-tru-b-001 — a-b-001 (numbered): 1. Definition

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 759
- lineEnd: 1117

Summary:

The first numbered movement determines definition as the opening synthetic fixation of concept-content unity.

Key points: (KeyPoint)

- k1. Definition initiates the numbered synthetic progression.
- k2. Determination is fixed but still finite in form.
- k3. Definition prepares the need for division.

Claims: (Claim)

- c1. id: cogni-tru-b-001-c1
  - subject: synthetic_definition
  - predicate: is_determined_as
  - object: initial_numbered_fixation_of_conceptual_content
  - modality: asserted
  - confidence: 0.91
  - evidence:
    - [759-760] marker: `1. Definition`.
    - [760-1117] development of definitional determination.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-b-002
  - note: definition passes into division.
  - sourceClaimIds: [`cogni-tru-b-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numbered section 1 fixed by user boundary.

### Entry cogni-tru-b-002 — a-b-002 (numbered): 2. Division

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 1119
- lineEnd: 1483

Summary:

The second numbered movement develops division as mediated organization of determinations prior to theorem deployment.

Key points: (KeyPoint)

- k1. Division structures the manifold in ordered conceptual relations.
- k2. Mediation thickens but remains within finite synthetic method.
- k3. Division transitions into theorem movement.

Claims: (Claim)

- c1. id: cogni-tru-b-002-c1
  - subject: synthetic_division
  - predicate: is_determined_as
  - object: mediated_ordering_of_determinations
  - modality: asserted
  - confidence: 0.9
  - evidence:
    - [1119-1119] marker: `2. Division`.
    - [1120-1483] divisional development up to theorem threshold.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-b-003
  - note: division opens theorem movement.
  - sourceClaimIds: [`cogni-tru-b-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numbered section 2 fixed by user boundary.

### Entry cogni-tru-b-003 — a-b-003 (numbered): 3. The theorem

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 1486
- lineEnd: 1892

Summary:

The theorem movement develops mediation, construction, and proof, culminating in the exposure of subjective proof-ground.

Key points: (KeyPoint)

- k1. Theorem progression intensifies mediated connectedness.
- k2. Construction and proof articulate necessity for cognition.
- k3. Ground appears as subjective with respect to the fact itself.

Claims: (Claim)

- c1. id: cogni-tru-b-003-c1
  - subject: theorem_movement
  - predicate: is_determined_as
  - object: mediated_proof_progression_with_subjective_ground
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [1486-1488] marker and theorem movement opening.
    - [1854-1892] proof-ground as subjective for cognition.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-c-001
  - note: theorem limit opens final `a-c` critical movement.
  - sourceClaimIds: [`cogni-tru-b-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numbered section 3 fixed by user boundary.

### Entry cogni-tru-c-001 — a-c-001 (analysis): explicit method limit and geometry restriction

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 1894
- lineEnd: 2030

Summary:

The first `a-c` analysis window states the limit of synthetic method and restricts geometric formalism's transposition.

Key points: (KeyPoint)

- k1. Necessary limit of this cognition is explicitly named.
- k2. Geometry is exemplary but methodologically restricted.
- k3. Intuition-prestige arguments are criticized.

Claims: (Claim)

- c1. id: cogni-tru-c-001-c1
  - subject: synthetic_method_limit
  - predicate: is_determined_as
  - object: explicit_boundary_of_geometric_formalism
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [1894-1907] limit and geometry framing.
    - [1928-1935] critique of intuition-based supremacy.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-c-002
  - note: first limit critique advances to deeper truth-relation correction.
  - sourceClaimIds: [`cogni-tru-c-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`cogni-tru-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: `a-c-*` range begins at user-fixed line 1894.

### Entry cogni-tru-c-002 — a-c-002 (analysis): correction of one-sided truth relation

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 2031
- lineEnd: 2118

Summary:

The second `a-c` window critiques one-sided demonstration and reorients cognition toward the absolutely true.

Key points: (KeyPoint)

- k1. External demonstration is exposed as inadequate.
- k2. One-sided methodological abstraction is shown as conceptual deficit.
- k3. Orientation to absolute truth is made explicit.

Claims: (Claim)

- c1. id: cogni-tru-c-002-c1
  - subject: truth_relation_correction
  - predicate: is_determined_as
  - object: critique_of_one_sided_external_demonstration
  - modality: asserted
  - confidence: 0.9
  - evidence:
    - [2031-2118] critical mid-section argument.
    - [2117-2118] orientation toward what is absolutely true.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-c-003
  - note: corrected truth relation moves into final synthesis close of `a-c`.
  - sourceClaimIds: [`cogni-tru-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second `a-c` window fixed.

### Entry cogni-tru-c-003 — a-c-003 (analysis): closure of true-cognition movement

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 2119
- lineEnd: 2194

Summary:

The third `a-c` window closes the true-cognition movement and sets the transition structure whose result is for cognition.

Key points: (KeyPoint)

- k1. Closing argument consolidates synthetic movement at its limit.
- k2. Transition is first inner necessity and only result is for cognition.
- k3. The move prepares practical determination.

Claims: (Claim)

- c1. id: cogni-tru-c-003-c1
  - subject: closure_of_true_cognition
  - predicate: is_determined_as
  - object: final_inner_necessity_before_practical_transition
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [2119-2190] closing movement of synthetic cognition.
    - [2191-2194] transition as inner necessity; result for cognition.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-tru-tr-001
  - note: closes `a-c-*` and hands off to explicit practical idea marker.
  - sourceClaimIds: [`cogni-tru-c-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`cogni-tru-tr-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: `a-c-*` range ends at user-fixed line 2194.

### Entry cogni-tru-tr-001 — transition gate: practical idea (action)

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-true.txt`
- lineStart: 2195
- lineEnd: 2197

Summary:

The explicit transition sentence determines the practical idea as action.

Key points: (KeyPoint)

- k1. Concept is now determined for itself in and for itself.
- k2. This determination is the practical idea.
- k3. Practical idea is action.

Claims: (Claim)

- c1. id: cogni-tru-tr-001-c1
  - subject: practical_idea
  - predicate: is_determined_as
  - object: action
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [2195-2197] explicit transition statement.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: cogni-god-001
  - note: direct handoff from end of Part A into Part B movement.
  - sourceClaimIds: [`cogni-tru-tr-001-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`cogni-god-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: final transition into action preserved outside `a-c-*` range.
