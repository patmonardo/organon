# Being Part A Workbook

Part: `A. BEING`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `being.txt` as authority.
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

- file: `being.txt`
- fixed range: lines `1-47`
- pass policy: lock spans first, then first-order claim extraction

Decision:

- Adopt two-level IDs for this part:
  - Level 1 marker: `bei-a`
  - Level 2 numbered entries: `bei-a-<nnn>`
- Apply short-section exception if needed; avoid artificial subdivision.
- Keep transition handoff explicit to Part B (`bei-b-001`) when evidence supports it.

### Entry bei-a-001 — Pure being as indeterminate immediacy and self-vanishing into nothing

Span:

- sourceFile: `src/compiler/being/quality/being/sources/being.txt`
- lineStart: 32
- lineEnd: 47

Summary:

Pure being is defined as indeterminate immediacy devoid of internal or external difference, and this very emptiness determines its identity with nothing.

Key points: (KeyPoint)

- k1. Pure being is immediate and without further determination.
- k2. Any posited distinction would negate the purity of being.
- k3. Pure indeterminacy and emptiness culminate in being's identity with nothing.

Claims: (Claim)

- c1. id: bei-a-001-c1
  - subject: pure_being
  - predicate: is_determined_as
  - object: indeterminate_immediacy_without_difference
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [32-35] pure being is without further determination and without difference.

- c2. id: bei-a-001-c2
  - subject: pure_being
  - predicate: excludes
  - object: posited_determination_or_content
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [36-39] posited content/distinction would prevent being from holding fast to purity.

- c3. id: bei-a-001-c3
  - subject: indeterminate_immediate_being
  - predicate: is_identical_with
  - object: nothing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [40-47] pure indeterminateness/emptiness leads to explicit statement that being is nothing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: bei-b-001
  - note: Part A closes by explicitly stating being's identity with nothing, opening Part B.
  - sourceClaimIds: [`bei-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bei-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: short-section exception applied with one seed entry for first pass.
