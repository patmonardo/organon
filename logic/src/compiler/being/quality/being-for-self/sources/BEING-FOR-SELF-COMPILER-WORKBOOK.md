# Being-for-self Compiler Workbook

Part: `COMPILER. BEING-FOR-SELF`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being-for-self/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for compiler-facing Being-for-self modeling.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Being-for-self source files in this folder.
- Claims must be line-anchored.
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

## Session: 2026-02-22 (initial scaffold)

Scope:

- files:
  - `being-for-self.txt`
  - `one-many.txt`
  - `attraction.txt`
- focus: compiler-oriented relation normalization (`Transition` and `TruthOf` alignment)

Decision:

- Maintain claim-level compatibility with `BEING-FOR-SELF-PART-A/B/C-WORKBOOK.md`.
- Use this file for compiler-facing relation normalization once part claims stabilize.
- Avoid algorithmic/generated content without source-anchored claims.

Review outcome:

- review_pending
- notes: initialized; ready for relation-schema pass after first-order review.

### Entry bfs-compiler-001 — Compiler coordinator: `Transition` and `TruthOf` scaffold for Being-for-self

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 35
- lineEnd: 47

Summary:

This compiler coordinator seeds relation normalization where the A/B/C movement is encoded as `Transition` structure and stabilization points are encoded as provisional `TruthOf` anchors, pending whole-sphere review.

Key points: (KeyPoint)

- k1. The source gives a direct ordered progression suitable for transition normalization.
- k2. The repulsion/attraction complex provides a mediated middle suitable for truth-anchoring of process-unity.
- k3. Quantity handoff is explicit and should be preserved as boundary transition.

Claims: (Claim)

- c1. id: bfs-compiler-001-c1
  - subject: being_for_self_progression
  - predicate: supports
  - object: transition_chain_a_to_b_to_c
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [35-47] explicit three-step progression in `being-for-self.txt`.

- c2. id: bfs-compiler-001-c2
  - subject: repulsion_attraction_mediation
  - predicate: supports
  - object: truthof_anchor_for_unity_of_process
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [293-301] repulsion and attraction each through mediation of the other and self-mediation in `attraction.txt`.

- c3. id: bfs-compiler-001-c3
  - subject: part_c_closure
  - predicate: supports
  - object: transition_to_quantity_boundary
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [43-47] quality passes over into quantity in `being-for-self.txt`.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: transition chain starts from stabilized Part A marker.
  - sourceClaimIds: [`bfs-compiler-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`bfs-a-001-c1`, `bfs-a-003-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: bfs-c
  - note: repulsion/attraction mediation provides key TruthOf scaffold for Part C.
  - sourceClaimIds: [`bfs-compiler-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`bfs-c-002-c3`, `bfs-c-003-c2`, `bfs-c-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: qty-a
  - note: preserve explicit quality->quantity boundary transition.
  - sourceClaimIds: [`bfs-compiler-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: compiler scaffold only; full relation inference deferred to whole-view pass.
