# Chemism Part C Workbook

Part: `C. TRANSITION OF CHEMISM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `chemism.txt` as authority.
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

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-22 (paragraph-segmentation pass)

Scope:

- file: `chemism.txt`
- fixed range: lines `4-119`
- segmentation basis: paragraph blocks `001|002|003`

Decision:

- Segment strictly by paragraph boundaries.
- Keep IDs in normalized form: `obj-chem-c-001`, `obj-chem-c-002`, `obj-chem-c-003`.

### Entry obj-chem-c-001 — Paragraph 1: self-mediating chemical connection

Span:

- sourceFile: `src/compiler/concept/object/chemism/chemism.txt`
- lineStart: 4
- lineEnd: 23

Summary:

The opening paragraph models chemical alteration as self-mediated connection in which the object posits its own presupposition and middle term, uniting concept and reality in objectified return.

Key points: (KeyPoint)

- k1. Chemical connection is not immediate one-sided relation.
- k2. Object posits the presupposition required for real combination.
- k3. Self-given middle term concludes concept and reality.
- k4. Negative principle disjoins extremes and reunites them in objectified return.

Claims: (Claim)

- c1. id: obj-chem-c-001-c1
  - subject: chemical_object
  - predicate: posits_for_itself
  - object: presupposition_and_middle_term_for_real_connection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [12-17] object posits needed presupposition and middle term to unite concept with reality.

- c2. id: obj-chem-c-001-c2
  - subject: concrete_concept_as_negative_principle
  - predicate: both_disjoins_and_reunites
  - object: extremes_in_objectified_return
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [18-23] singularity/concrete concept as disjunctive-reunifying negative principle returning objectified.

Claim ↔ key point map:

- c1 -> k1, k2, k3
- c2 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-chem-c-002
  - note: self-mediated model transitions to diagnosis of chemism's still-external three-syllogism structure.
  - sourceClaimIds: [`obj-chem-c-001-c1`, `obj-chem-c-001-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`obj-chem-c-002-c1`, `obj-chem-c-002-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph boundary preserved.

### Entry obj-chem-c-002 — Paragraph 2: chemism totality and external falling-apart

Span:

- sourceFile: `src/compiler/concept/object/chemism/chemism.txt`
- lineStart: 25
- lineEnd: 78

Summary:

The second paragraph presents chemism as first negation with three-syllogism totality, yet still burdened by externality so processes fall apart into condition-bound, externally reactivated cycles.

Key points: (KeyPoint)

- k1. Chemism negates indifferent objectivity but remains externally burdened.
- k2. Totality appears as three syllogisms with formal neutrality, real neutrality, and self-realizing concept.
- k3. Due to immediacy/externality, the syllogisms and process moments fall apart.
- k4. Neutralization and dissolution remain bound as externally split sides.

Claims: (Claim)

- c1. id: obj-chem-c-002-c1
  - subject: chemism
  - predicate: is_determined_as
  - object: first_negation_not_yet_self_determining_totality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [25-34] chemism as first negation still burdened by externality.

- c2. id: obj-chem-c-002-c2
  - subject: chemism_totality
  - predicate: is_articulated_as
  - object: three_syllogisms_that_fall_apart_under_externality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [35-51] three-syllogism articulation and falling apart.

- c3. id: obj-chem-c-002-c3
  - subject: chemical_process_moments
  - predicate: remain
  - object: externally_conditioned_and_split_into_diverse_sides
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [52-78] reactivation from outside; neutralization and dissolution bound yet externally bifurcated.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-chem-c-003
  - note: external conditionality is progressively sublated into unconditioned conceptual totality.
  - sourceClaimIds: [`obj-chem-c-002-c2`, `obj-chem-c-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`obj-chem-c-003-c1`, `obj-chem-c-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph boundary preserved.

### Entry obj-chem-c-003 — Paragraph 3: staged sublation to objective free concept

Span:

- sourceFile: `src/compiler/concept/object/chemism/chemism.txt`
- lineStart: 80
- lineEnd: 119

Summary:

The final paragraph presents necessary staged sublations of externality and conditionality across processes, culminating in the concept's fully liberated objective freedom as purpose.

Key points: (KeyPoint)

- k1. Multiple processes are necessary stages sublating externality/conditionality.
- k2. First and second process sublations internalize determinateness and mediation.
- k3. Third syllogism removes remaining abstract immediacy.
- k4. Fully liberated objective free concept is purpose.

Claims: (Claim)

- c1. id: obj-chem-c-003-c1
  - subject: staged_chemical_processes
  - predicate: sublate
  - object: externality_and_conditionality_toward_unconditioned_totality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [80-93] processes as necessary stages sublating externality; first/second sublations specified.

- c2. id: obj-chem-c-003-c2
  - subject: third_syllogism_and_concept
  - predicate: culminate_in
  - object: objective_free_concept_as_purpose
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [106-119] third syllogism sublates final immediacy; concept liberated as purpose.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: obj-chem-c-001
  - note: final liberation confirms opening thesis that chemism's own mediation is concept-driven.
  - sourceClaimIds: [`obj-chem-c-003-c1`, `obj-chem-c-003-c2`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`obj-chem-c-001-c1`, `obj-chem-c-001-c2`]
  - logicalOperator: retrospective_grounding
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: pending_teleology_part_a_entry
  - note: purpose marks boundary transition beyond chemism into teleological determination.
  - sourceClaimIds: [`obj-chem-c-003-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`pending`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph boundary preserved; final transition explicitly captured.
