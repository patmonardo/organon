# Becoming-Essence Part A Workbook

Part: `A. ABSOLUTE INDIFFERENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/becoming-essence/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `indifference-absolute.txt` as authority.
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

## Session: 2026-02-23 (initial scaffold)

Scope:

- file: `indifference-absolute.txt`
- pass policy: concept-part extraction for Part A using paragraph-based decomposition.

Decision:

- Use ID prefix `be-a` for Part A entries.
- Concept-part-first workflow is active.
- Transition-boundary choices are treated as review-sensitive decisions to minimize disruption of abstraction products.
- Part A has no subentries.
- Sectioning is paragraph-based.
- Final-entry/transition boundary remains review-sensitive and may be revised in a non-disruptive pass if needed.
- Defer IDEA-special pass and compiler-normalization pass until Part A/B/C first-order claims are stabilized.

## Decomposition lock

- Entry model: three-entry paragraph grouping.
- Planned entries:
  - `be-a-001` -> paragraph 1 (`4-21`)
  - `be-a-002` -> paragraph 2 (`23-39`)
  - `be-a-003` -> paragraph 3 (`40-44`)

### Entry be-a-001 — Abstract indifference and mediated absolute indifference

Span:

- sourceFile: `src/compiler/being/measure/becoming-essence/indifference-absolute.txt`
- lineStart: 4
- lineEnd: 21

Summary:

The opening movement distinguishes abstract indifferentness and pure quantity from absolute indifference, which is self-mediated through negation of determinate being and measure.

Key points: (KeyPoint)

- k1. Abstract indifferentness names being thought without determinate content.
- k2. Pure quantity is indifferent openness to externally given determinations.
- k3. Absolute indifference is mediated unity through negation of quality, quantity, and immediate measure.
- k4. Determinateness remains only as external state on the substrate.

Claims: (Claim)

- c1. id: be-a-001-c1
  - subject: pure_quantity
  - predicate: is
  - object: externally_determined_indifference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [9-13] pure quantity is indifferent and open only to external determinations.

- c2. id: be-a-001-c2
  - subject: absolute_indifference
  - predicate: is
  - object: self_mediated_unity_via_negation_of_being_determinations
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [14-18] absolute indifference mediates itself through negation of quality, quantity, and measure.

- c3. id: be-a-001-c3
  - subject: determinateness_in_absolute_indifference
  - predicate: remains_as
  - object: external_state_on_substrate
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [19-21] determinateness is only qualitative/external state with indifference as substrate.

Claim ↔ key point map:

- be-a-001-c1 -> k1, k2
- be-a-001-c2 -> k3
- be-a-001-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: be-a-002
  - note: external state is tested as vanishing differentiation and internalized mediation.
  - sourceClaimIds: [`be-a-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`be-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph 1 stabilized.

### Entry be-a-002 — Vanishing externality and concrete indifference as result

Span:

- sourceFile: `src/compiler/being/measure/becoming-essence/indifference-absolute.txt`
- lineStart: 23
- lineEnd: 39

Summary:

What was external qualitative determinateness vanishes, and through this vanishing indifference appears as concrete, self-mediated differentiation.

Key points: (KeyPoint)

- k1. External qualitative determinateness is self-opposed vanishing.
- k2. Determinateness is first only empty differentiation in substrate.
- k3. This empty differentiation is the result-form of indifference.
- k4. Concrete indifference includes immanent, self-referring differentiation.

Claims: (Claim)

- c1. id: be-a-002-c1
  - subject: external_qualitative_determinateness
  - predicate: is
  - object: vanishing_self_sublating_opposition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [23-28] qualitative external sphere is opposite of itself and sublates itself.

- c2. id: be-a-002-c2
  - subject: empty_differentiation_in_substrate
  - predicate: is
  - object: result_form_of_indifference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [29-34] empty differentiation is precisely indifference as result.

- c3. id: be-a-002-c3
  - subject: concrete_indifference
  - predicate: contains
  - object: immanent_self_referring_differentiation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [34-39] indifference as mediation contains negation/relation and immanent differentiation.

Claim ↔ key point map:

- be-a-002-c1 -> k1
- be-a-002-c2 -> k2, k3
- be-a-002-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: be-a-003
  - note: concrete indifference is closed by explicitly internalizing externality and vanishing.
  - sourceClaimIds: [`be-a-002-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`be-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph 2 stabilized.

### Entry be-a-003 — Internalized externality and de-substratification of indifference

Span:

- sourceFile: `src/compiler/being/measure/becoming-essence/indifference-absolute.txt`
- lineStart: 40
- lineEnd: 44

Summary:

The closing paragraph internalizes externality and vanishing into indifference itself, so indifference ceases to be merely substrate and abstract.

Key points: (KeyPoint)

- k1. Externality and vanishing are constitutive of indifference-unity.
- k2. They are internal moments, not external attachments.
- k3. Indifference ceases to be merely substrate and merely abstract.

Claims: (Claim)

- c1. id: be-a-003-c1
  - subject: indifference_unity
  - predicate: internalizes
  - object: externality_and_vanishing
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [40-42] externality and vanishing make unity of being into indifference and are inside it.

- c2. id: be-a-003-c2
  - subject: indifference
  - predicate: ceases_to_be
  - object: merely_substrate_and_merely_abstract
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [43-44] indifference no longer only substrate or only abstract.

Claim ↔ key point map:

- be-a-003-c1 -> k1, k2
- be-a-003-c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: be-b-001
  - note: Part A closes by opening Part B, where indifference is explicitly posited as inverse ratio of factors.
  - sourceClaimIds: [`be-a-003-c1`, `be-a-003-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`be-b-001-c1`]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: paragraph 3 stabilized; Part A first-order pass complete.
