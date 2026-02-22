# Being-for-self Idea Workbook

Part: `IDEA. BEING-FOR-SELF`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being-for-self/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
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
- focus: sphere-level map across Part A/B/C and transition toward Quantity

Decision:

- Keep this workbook as the Being-for-self IDEA coordinator.
- Keep Part A/B/C workbooks as first-order extraction artifacts.
- Defer higher-order synthesis claims until part-level spans are stabilized.

Review outcome:

- review_pending
- notes: initialized; ready for sphere-level coordinator entry.

### Entry bfs-idea-001 — Being-for-self sphere coordinator: A/B/C progression

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/being-for-self.txt`
- lineStart: 35
- lineEnd: 47

Summary:

Being-for-self is coordinated as a three-part movement: one, one-many via repulsion/attraction, and their equilibrium transition toward quantity.

Key points: (KeyPoint)

- k1. Part A establishes existent-for-itself as the one.
- k2. Part B develops the one into plurality through repulsion and ideality.
- k3. Part C develops repulsion/attraction mediation and prepares quantity transition.

Claims: (Claim)

- c1. id: bfs-idea-001-c1
  - subject: being_for_self_sphere
  - predicate: begins_with
  - object: one_as_immediate_existent_for_itself
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [35-37] first movement: being-for-itself as existent-for-itself, the one.

- c2. id: bfs-idea-001-c2
  - subject: being_for_self_sphere
  - predicate: develops_into
  - object: multiplicity_of_ones_via_repulsion_and_attraction
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [38-41] one passes over into many ones, repulsion and attraction.

- c3. id: bfs-idea-001-c3
  - subject: being_for_self_sphere
  - predicate: culminates_in
  - object: equilibrium_of_repulsion_and_attraction_transitioning_to_quantity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [43-47] alternating repulsion/attraction sinks to equilibrium and quality passes into quantity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bfs-a
  - note: coordinator grounds Part A marker as first movement.
  - sourceClaimIds: [`bfs-idea-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`bfs-a-001-c1`, `bfs-a-002-c3`, `bfs-a-003-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: bfs-b
  - note: coordinator grounds Part B marker as second movement.
  - sourceClaimIds: [`bfs-idea-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`bfs-b-001-c1`, `bfs-b-002-c1`, `bfs-b-003-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: supports
  - targetEntryId: bfs-c
  - note: coordinator grounds Part C marker and quantity handoff.
  - sourceClaimIds: [`bfs-idea-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-c-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first sphere-level coordinator added; whole-view synthesis deferred.
