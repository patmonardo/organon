# Being-for-self Part B Workbook

Part: `B. THE ONE AND THE MANY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being-for-self/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `one-many.txt` as authority.
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

## Session: 2026-02-22 (seed pass)

Scope:

- file: `one-many.txt`
- fixed range: full file
- pass policy: lock 1 marker + up to 3 numbered entries only

Decision:

- Two-level IDs:
  - Level 1 marker: `bfs-b`
  - Level 2 numbered entries: `bfs-b-<nnn>`
- Keep first pass at conceptual readability level; no micro-fragmentation.
- Preserve transition target toward Part C marker `bfs-c`.

### Entry bfs-b — Marker `B`: The One and the Many

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/one-many.txt`
- lineStart: 2
- lineEnd: 247

Summary:

Part B develops the one into many ones through repulsion, while showing one/void and many-ones as internally mediated determinations of being-for-itself.

Key points: (KeyPoint)

- k1. The one is immediate self-reference of being-for-itself, but this immediacy externalizes moments as existents.
- k2. The one/void articulation shows the negative terrain of onehood.
- k3. Repulsion posits many ones and prepares transition to attraction.

Claims: (Claim)

- c1. id: bfs-b-c1
  - subject: the_one
  - predicate: is_determined_as
  - object: immediate_self_reference_of_being_for_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-13] one as simple self-reference and infinite self-determining.
    - [22-31] unity of being/existence with internal opposition.

- c2. id: bfs-b-c2
  - subject: one_void_relation
  - predicate: expresses
  - object: negative_terrain_of_onehood
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [71-83] void as posited nothing and quality of one in immediacy.
    - [86-99] one and void as differentiated within one reference.

- c3. id: bfs-b-c3
  - subject: repulsion
  - predicate: posits
  - object: many_ones_as_externalized_infinity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [145-160] one repels itself from itself; repulsion as self-relation.
    - [223-247] plurality as contradiction of one producing itself.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-a
  - note: Part B unfolds the one that Part A establishes.
  - sourceClaimIds: [`bfs-b-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`bfs-a-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-c
  - note: repulsion of many ones transitions to explicit repulsion/attraction dynamics.
  - sourceClaimIds: [`bfs-b-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-c-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry bfs-b-001 — `a. The one within`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/one-many.txt`
- lineStart: 43
- lineEnd: 83

Summary:

The one within is unalterable self-reference that negates external determination and bears the void as its immediate qualitative determination.

Key points: (KeyPoint)

- k1. The one is not constituted by relation-to-other but by negation of that relational circle.
- k2. Its determinateness is self-reference, not indeterminate being.
- k3. The void is posited as the one's immediate quality.

Claims: (Claim)

- c1. id: bfs-b-001-c1
  - subject: one_within
  - predicate: is_determined_as
  - object: unalterable_self_reference
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [45-53] one negates relational determination and is unalterable.

- c2. id: bfs-b-001-c2
  - subject: determinateness_of_one
  - predicate: is
  - object: self_referring_absolute_being
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [54-59] indeterminateness of one is determinateness of self-reference.
    - [60-67] directedness to other immediately reverts to self.

- c3. id: bfs-b-001-c3
  - subject: void
  - predicate: is_posited_as
  - object: quality_of_one_in_immediacy
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [71-83] posited nothing in one is void; void is quality of one in immediacy.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-b
  - note: isolates the inward constitution of onehood that grounds Part B.
  - sourceClaimIds: [`bfs-b-001-c1`, `bfs-b-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-b-c1`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-b-002
  - note: posited void within the one becomes explicit one/void relation.
  - sourceClaimIds: [`bfs-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry bfs-b-002 — `b. The one and the void`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/one-many.txt`
- lineStart: 84
- lineEnd: 110

Summary:

The one and the void are absolutely diverse yet co-determined within one reference, reintroducing existence as an externalized split of being-for-itself.

Key points: (KeyPoint)

- k1. Void differs absolutely from the immediate affirmative being of the one.
- k2. Their difference is posited in one relation and appears as void outside the one.
- k3. Being-for-itself regains existence through this split.

Claims: (Claim)

- c1. id: bfs-b-002-c1
  - subject: one_and_void
  - predicate: are
  - object: absolutely_diverse_within_one_reference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [86-93] void is absolutely diverse from one yet difference is posited in one relation.

- c2. id: bfs-b-002-c2
  - subject: being_for_itself_as_one_and_void
  - predicate: reacquires
  - object: existence
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [94-99] determination as one and void acquires existence again.

- c3. id: bfs-b-002-c3
  - subject: unity_of_moments
  - predicate: is_lowered_to
  - object: existence_confronted_by_void
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [100-110] moments externalize; unity withdraws and confronts negation as void.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k3
- c3 -> k2

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-b
  - note: details the one/void articulation named at marker level.
  - sourceClaimIds: [`bfs-b-002-c1`, `bfs-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`bfs-b-c2`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-b-003
  - note: regained externality drives the emergence of many ones and repulsion.
  - sourceClaimIds: [`bfs-b-002-c3`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`bfs-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry bfs-b-003 — `c. Many ones`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/one-many.txt`
- lineStart: 111
- lineEnd: 247

Summary:

Many ones arise as the one's self-repulsion: plurality is both produced by and external to the one, disclosing an externalized infinity that transitions to attraction.

Key points: (KeyPoint)

- k1. The one becomes many ones through its own negative self-reference.
- k2. Repulsion is the one's immanent movement, not external becoming into other.
- k3. Plurality is the contradiction of onehood as externalized infinity.

Claims: (Claim)

- c1. id: bfs-b-003-c1
  - subject: one
  - predicate: becomes
  - object: many_ones_through_self_repulsion
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [145-160] one is becoming of many ones; negative self-reference is repulsion.

- c2. id: bfs-b-003-c2
  - subject: repulsion
  - predicate: is_determined_as
  - object: immanent_positing_of_ones
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [162-176] first repulsion posits many ones through one itself and determines external repulsion.
    - [223-233] repulsion makes explicit what one is implicitly.

- c3. id: bfs-b-003-c3
  - subject: plurality_of_ones
  - predicate: is
  - object: contradiction_of_one_as_externalized_infinity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [236-247] plurality both posited by one and external to one; contradiction self-produces.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-b
  - note: explicates repulsion as the core mechanism of the marker section.
  - sourceClaimIds: [`bfs-b-003-c1`, `bfs-b-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-b-c3`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: bfs-c-001
  - note: contradictory plurality advances into explicit exclusion and mutual repulsion.
  - sourceClaimIds: [`bfs-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bfs-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker plus lettered subtopic entries completed in bounded first pass.
