# Quantity Part C Workbook

Part: `C. THE LIMITING OF QUANTITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `limiting-quantity.txt` as authority.
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

- file: `limiting-quantity.txt`
- fixed range: lines `1-47`
- pass policy: exactly three seed entries with one claim each

Decision:

- Use IDs: `qty-c-001`, `qty-c-002`, `qty-c-003`.
- This part has no explicit numeric subsectioning in the source; triadic segmentation here is analytic.
- Keep closure handoff explicit toward Quantum seeding.

### Entry qty-c-001 — Limiting I: one as determinateness and limit of discrete quantity

Span:

- sourceFile: `src/compiler/being/quantity/quantity/limiting-quantity.txt`
- lineStart: 4
- lineEnd: 23

Summary:

Discrete magnitude is posited as one magnitude whose determinateness is a one that functions as limit to unity.

Key points: (KeyPoint)

- k1. Discrete magnitude is plurality and continuity together.
- k2. It is posited as one magnitude.
- k3. The one is determinateness as limit.

Claims: (Claim)

- c1. id: qty-c-001-c1
  - subject: discrete_magnitude
  - predicate: is_determined_by
  - object: one_as_limiting_determinateness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [12-16] one magnitude is posited and one is determinateness that excludes as limit.
    - [20-23] first negation and limit in this something is the one.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qty-c-002
  - note: limit is further determined as self-referred enclosing limit against continuity.
  - sourceClaimIds: [`qty-c-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qty-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim opening seed entry.

### Entry qty-c-002 — Limiting II: enclosing limit and continuity’s transcendence

Span:

- sourceFile: `src/compiler/being/quantity/quantity/limiting-quantity.txt`
- lineStart: 24
- lineEnd: 36

Summary:

The one as limit is self-referred and enclosing, yet continuity transcends this point and becomes indifferent to it, yielding quantum.

Key points: (KeyPoint)

- k1. Limit is self-referred as one.
- k2. Limit is enclosing negative point.
- k3. Continuity transcends and is indifferent to this limit.

Claims: (Claim)

- c1. id: qty-c-002-c1
  - subject: limiting_one
  - predicate: is_sublated_by
  - object: continuity_that_transcends_indifferent_limit
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [27-33] one is enclosing negative point while continuity transcends and is indifferent to it.
    - [34-36] real discrete quantity is thereby one quantity, quantum.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qty-c-003
  - note: the limiting relation is generalized to both continuous and discrete magnitude becoming quanta.
  - sourceClaimIds: [`qty-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qty-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim middle seed entry.

### Entry qty-c-003 — Limiting III: indifference of the distinction and passage into quantum

Span:

- sourceFile: `src/compiler/being/quantity/quantity/limiting-quantity.txt`
- lineStart: 37
- lineEnd: 47

Summary:

Because the limiting one equally limits both moments, the distinction between continuous and discrete magnitude becomes indifferent and both pass into quanta.

Key points: (KeyPoint)

- k1. One limit encompasses many ones.
- k2. It limits continuity of each moment equally.
- k3. Both moments pass over into quanta.

Claims: (Claim)

- c1. id: qty-c-003-c1
  - subject: distinction_of_continuous_and_discrete_magnitude
  - predicate: passes_over_into
  - object: indifferent_quanta
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [42-47] distinction is indifferent and both pass over into being quanta.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-a-001
  - note: closure of Quantity section passes to Quantum seed domain (target to be confirmed in quantum workbook pass).
  - sourceClaimIds: [`qty-c-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim closure seed entry.
