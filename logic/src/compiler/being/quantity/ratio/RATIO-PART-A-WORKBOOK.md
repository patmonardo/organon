# Ratio Part A Workbook

Part: `A. THE DIRECT RATIO`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/ratio/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `ratio.txt` as authority.
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

- file: `ratio.txt`
- fixed range: lines `1-110`
- pass policy: numeric ordinary entries only

Decision:

- Use IDs: `rat-a-001`, `rat-a-002`, `rat-a-003`.
- Ratio has no subspecies in this workbook pass.
- Keep handoff explicit to Part B (`rat-b-001`).

### Entry rat-a-001 — Direct ratio I: reciprocal determination under one exponent

Span:

- sourceFile: `src/compiler/being/quantity/ratio/ratio.txt`
- lineStart: 4
- lineEnd: 9

Summary:

Direct ratio is immediate reciprocal determinateness of two quanta under one common exponent.

Key points: (KeyPoint)

- k1. Each side is determined through the other.
- k2. Both share one limit.
- k3. This common limit is exponent.

Claims: (Claim)

- c1. id: rat-a-001-c1
  - subject: direct_ratio
  - predicate: is_determined_by
  - object: common_exponent_of_both_sides
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-9] direct ratio has one reciprocal determinateness, namely exponent.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rat-a-002
  - note: exponent is next developed as unity of unit and amount.
  - sourceClaimIds: [`rat-a-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`rat-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry rat-a-002 — Direct ratio II: exponent as qualitative unity of unit and amount

Span:

- sourceFile: `src/compiler/being/quantity/ratio/ratio.txt`
- lineStart: 11
- lineEnd: 52

Summary:

The exponent is a qualitatively determined quantum that immediately unifies unit and amount while fixing ratio determinateness.

Key points: (KeyPoint)

- k1. Exponent includes difference of unit and amount.
- k2. Exponent determines both sides indifferently to side magnitude.
- k3. Ratio determinateness rests solely on exponent.

Claims: (Claim)

- c1. id: rat-a-002-c1
  - subject: exponent_in_direct_ratio
  - predicate: is
  - object: qualitative_unity_of_unit_and_amount
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [27-37] exponent as simple determinateness with both moments.
    - [38-52] side magnitudes vary while ratio determinateness remains in exponent.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rat-a-003
  - note: incomplete side-self-subsistence drives move to more real ratio.
  - sourceClaimIds: [`rat-a-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`rat-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry rat-a-003 — Direct ratio III: incompleteness and transition to inverse ratio

Span:

- sourceFile: `src/compiler/being/quantity/ratio/ratio.txt`
- lineStart: 53
- lineEnd: 110

Summary:

Because sides count only as incomplete moments and exponent is not fully posited as qualitative unity, direct ratio passes into inverse ratio.

Key points: (KeyPoint)

- k1. Sides are incomplete moments (unit/amount split).
- k2. Exponent is not fully posited as determinant unity.
- k3. More real ratio requires exponent as product.

Claims: (Claim)

- c1. id: rat-a-003-c1
  - subject: direct_ratio
  - predicate: transitions_into
  - object: inverse_ratio
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [105-110] explicit emergence of inverse ratio as more real form.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: rat-b-001
  - note: direct ratio is sublated into inverse ratio.
  - sourceClaimIds: [`rat-a-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`rat-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.
