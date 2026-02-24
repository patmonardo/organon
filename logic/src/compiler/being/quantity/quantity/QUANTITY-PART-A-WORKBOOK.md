# Quantity Part A Workbook

Part: `A. PURE QUANTITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `pure-quantity.txt` as authority.
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

- file: `pure-quantity.txt`
- fixed range: lines `1-67`
- pass policy: exactly three seed entries with one claim each

Decision:

- Use IDs: `qty-a-001`, `qty-a-002`, `qty-a-003`.
- This part has no source-native numeric subsections; triadic segmentation here is analytic.
- Keep handoff explicit to Part B (`qty-b-001`).

### Entry qty-a-001 — Pure quantity I: sublated being-for-itself as attraction-continuity

Span:

- sourceFile: `src/compiler/being/quantity/quantity/pure-quantity.txt`
- lineStart: 4
- lineEnd: 15

Summary:

Pure quantity begins as sublated being-for-itself in which repelling one passes into attraction and continuity.

Key points: (KeyPoint)

- k1. Repelling one loses exclusive determinacy.
- k2. Being-for-itself passes into attraction.
- k3. Attraction is continuity in quantity.

Claims: (Claim)

- c1. id: qty-a-001-c1
  - subject: being_for_itself
  - predicate: sublates_into
  - object: attraction_as_moment_of_continuity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-15] repelling one loses determination and passes into attraction, stated as continuity moment.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qty-a-002
  - note: continuity is next determined as unbroken self-same unity of ones.
  - sourceClaimIds: [`qty-a-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qty-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim opening seed entry.

### Entry qty-a-002 — Pure quantity II: continuity as unbroken equality of ones

Span:

- sourceFile: `src/compiler/being/quantity/quantity/pure-quantity.txt`
- lineStart: 17
- lineEnd: 31

Summary:

Continuity is self-same reference to itself as unbroken unity of many ones in undifferentiated equality.

Key points: (KeyPoint)

- k1. Continuity is unbroken self-reference.
- k2. Continuity contains plurality of ones.
- k3. Plurality is posited as simple equality.

Claims: (Claim)

- c1. id: qty-a-002-c1
  - subject: continuity
  - predicate: is_determined_as
  - object: unbroken_self_same_unity_of_plural_ones
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [17-31] continuity is unbroken self-reference of ones, with plurality as undifferentiated equality.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qty-a-003
  - note: continuity now unfolds as unity with discreteness in quantity.
  - sourceClaimIds: [`qty-a-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`qty-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim middle seed entry.

### Entry qty-a-003 — Pure quantity III: unity of continuity and discreteness

Span:

- sourceFile: `src/compiler/being/quantity/quantity/pure-quantity.txt`
- lineStart: 33
- lineEnd: 67

Summary:

Quantity is the unity of continuity and discreteness as self-equal plurality in which being-for-itself is posited in its truth.

Key points: (KeyPoint)

- k1. Quantity unites continuity and discreteness.
- k2. Discreteness persists without breaking equality.
- k3. Being-for-itself is now true as self-equal plurality.

Claims: (Claim)

- c1. id: qty-a-003-c1
  - subject: quantity
  - predicate: is
  - object: unity_of_continuity_and_discreteness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [43-44] quantity is explicitly the unity of continuity and discreteness.
    - [53-67] moments are preserved as self-equal plurality in truth.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qty-b-001
  - note: Part A unity is developed as continuous magnitude in Part B.
  - sourceClaimIds: [`qty-a-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`qty-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim closure seed entry.
