# Ratio Part B Workbook

Part: `B. THE INVERSE RATIO`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/ratio/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `inverse.txt` as authority.
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

- file: `inverse.txt`
- fixed range: lines `1-268`
- pass policy: numeric ordinary entries only

Decision:

- Use IDs: `ratio-inv-001`, `ratio-inv-002`, `ratio-inv-003`.
- Ratio has no subspecies in this workbook pass.
- Keep handoff explicit to Part C (`ratio-pow-001`).

### Entry ratio-inv-001 — Inverse ratio I: exponent as product and internalized alteration

Span:

- sourceFile: `src/compiler/being/quantity/ratio/inverse.txt`
- lineStart: 4
- lineEnd: 71

Summary:

Inverse ratio sublates direct ratio by determining exponent as product and internalizing alteration within the ratio as qualitative limit.

Key points: (KeyPoint)

- k1. Exponent now has value of product.
- k2. Side relation becomes alterable with unit change.
- k3. Alteration is limited within ratio by exponent’s negative determinateness.

Claims: (Claim)

- c1. id: ratio-inv-001-c1
  - subject: inverse_ratio
  - predicate: is_determined_as
  - object: product_exponent_with_internalized_alteration
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-8] exponent value as product.
    - [50-70] alteration is internal to ratio and limited by exponent.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ratio-inv-002
  - note: internalized alteration is analyzed as unity of affirmative and negative moments.
  - sourceClaimIds: [`ratio-inv-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ratio-inv-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry ratio-inv-002 — Inverse ratio II: reciprocal limiting, bad infinity, and affirmative exponent

Span:

- sourceFile: `src/compiler/being/quantity/ratio/inverse.txt`
- lineStart: 72
- lineEnd: 193

Summary:

Inverse ratio realizes reciprocal limiting where sides contain each other through negation, generating bad infinite approximation yet affirming exponent as their in-itself unity.

Key points: (KeyPoint)

- k1. Sides reciprocally limit and determine each other.
- k2. Infinite approximation to beyond is bad infinity.
- k3. Exponent is affirmatively present as unity of both sides.

Claims: (Claim)

- c1. id: ratio-inv-002-c1
  - subject: exponent_in_inverse_ratio
  - predicate: is
  - object: negative_limit_and_affirmative_unity_of_sides
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [98-100] exponent as negative unity and limit.
    - [163-171] beyond as bad infinite approximation.
    - [181-192] affirmative infinity present as exponent.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ratio-inv-003
  - note: unity of limit and mediation yields transition to a new ratio determination.
  - sourceClaimIds: [`ratio-inv-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ratio-inv-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry ratio-inv-003 — Inverse ratio III: mediated self-rejoining exponent and transition to powers

Span:

- sourceFile: `src/compiler/being/quantity/ratio/inverse.txt`
- lineStart: 194
- lineEnd: 268

Summary:

Inverse ratio culminates when exponent preserves itself through negation of indifferent moments and becomes determining factor of self-surpassing, yielding ratio of powers.

Key points: (KeyPoint)

- k1. Exponent develops as mediation with itself in finite moments.
- k2. Infinite progression negates indifferent value-fixity.
- k3. Exponent rejoins itself as determiner of movement.

Claims: (Claim)

- c1. id: ratio-inv-003-c1
  - subject: inverse_ratio
  - predicate: transitions_into
  - object: ratio_of_powers
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [265-268] explicit determination as ratio of powers.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ratio-pow-001
  - note: inverse ratio passes into ratio of powers.
  - sourceClaimIds: [`ratio-inv-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`ratio-pow-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.
