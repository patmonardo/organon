# Quantum Part A Workbook

Part: `A. NUMBER`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/quantum/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `number.txt` as authority.
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

- file: `number.txt`
- fixed range: lines `1-136`
- pass policy: preserve inline subspecies `(a)(b)(c)` and seed one ordinary entry for each

Decision:

- Use IDs: `qtm-a-001`, `qtm-a-002`, `qtm-a-003`.
- Inline `(a)(b)(c)` is analyzed as ordered moments, not subspecies IDs, in this pass.
- Keep transition handoff explicit to Part B (`qtm-b-a-001`).

### Entry qtm-a-001 — Self-referring one as quantitative principle

Span:

- sourceFile: `src/compiler/being/quantity/quantum/number.txt`
- lineStart: 21
- lineEnd: 29

Summary:

The one, as one of quantity, is continuous unity and discrete plurality of equal ones.

Key points: (KeyPoint)

- k1. One is principle of quantum.
- k2. One is continuous unity.
- k3. One is discrete plurality of equal ones.

Claims: (Claim)

- c1. id: qtm-a-001-c1
  - subject: one_of_quantity
  - predicate: is
  - object: unity_and_equal_plurality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [21-29] one as principle is both continuous unity and discrete plurality.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-a-002
  - note: principle one is next determined as enclosing and excluding limit.
  - sourceClaimIds: [`qtm-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`qtm-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry qtm-a-002 — Enclosing limit

Span:

- sourceFile: `src/compiler/being/quantity/quantum/number.txt`
- lineStart: 30
- lineEnd: 37

Summary:

The one is a simple limit that encloses itself and excludes other quanta.

Key points: (KeyPoint)

- k1. One negates many ones as simple limit.
- k2. One encloses itself.
- k3. One excludes otherness.

Claims: (Claim)

- c1. id: qtm-a-002-c1
  - subject: one_of_quantity
  - predicate: functions_as
  - object: self_enclosing_other_excluding_limit
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [30-37] one is negating, enclosing, and other-excluding limit.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-a-003
  - note: enclosing limit is fully posited in number as unity/amount structure.
  - sourceClaimIds: [`qtm-a-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.

### Entry qtm-a-003 — Amount/unit contradiction and quantitative quality

Span:

- sourceFile: `src/compiler/being/quantity/quantum/number.txt`
- lineStart: 39
- lineEnd: 136

Summary:

Number posits quantum completely as determinate plurality (amount and unit), while preserving indifference that develops into quantum’s intrinsic contradiction.

Key points: (KeyPoint)

- k1. Number is complete positedness of quantum.
- k2. Number contains moments of amount and unit.
- k3. Number’s indifference to others is also its exteriority and contradiction.

Claims: (Claim)

- c1. id: qtm-a-003-c1
  - subject: number
  - predicate: contains
  - object: unity_of_amount_unit_and_intrinsic_contradiction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [55-65] amount and unit are moments of number.
    - [124-136] number is self-related yet externally referred; this contradiction develops further.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-a-001
  - note: contradiction of number unfolds in extensive/intensive quantum.
  - sourceClaimIds: [`qtm-a-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-b-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim numeric ordinary entry.
