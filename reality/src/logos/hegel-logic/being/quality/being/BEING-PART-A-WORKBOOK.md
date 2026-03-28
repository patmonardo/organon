# Being Part A Workbook

Part: `A. BEING`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being/WORKBOOK-CONTRACT-V1.md`
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

## Session: 2026-02-22 (deep pass)

Scope:

- file: `being.txt`
- fixed range: lines `1-47`
- pass policy: stabilize analytic subentries with one claim per subentry

Decision:

- Expanded IDs for Part A:
  - Analytic entries: `being-bei-<nnn>`
- Numbered subentries are analytic segmentations, not additional source-labeled species.
- Prefer one-claim subentries over claim-packed entries.
- Keep transition handoff explicit to Part B (`being-not-001`) when evidence supports it.

### Entry being-bei-001 — Pure being I: indeterminate immediacy

Span:

- sourceFile: `src/compiler/being/quality/being/being.txt`
- lineStart: 32
- lineEnd: 35

Summary:

Pure being is immediate and without further determination.

Key points: (KeyPoint)

- k1. Being is immediate.
- k2. Being is undetermined.
- k3. Being has no difference.

Claims: (Claim)

- c1. id: being-bei-001-c1
  - subject: pure_being
  - predicate: is_determined_as
  - object: indeterminate_immediacy_without_difference
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [32-35] pure being is without further determination and without difference.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: being-bei-002
  - note: immediacy is tested against any posited content.
  - sourceClaimIds: [`being-bei-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`being-bei-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim moment for opening determination.

### Entry being-bei-002 — Pure being II: exclusion of posited content

Span:

- sourceFile: `src/compiler/being/quality/being/being.txt`
- lineStart: 36
- lineEnd: 39

Summary:

Any posited distinction or content would negate pure being.

Key points: (KeyPoint)

- k1. Distinction introduces determination.
- k2. Determination breaks purity.
- k3. Pure being excludes such content.

Claims: (Claim)

- c1. id: being-bei-002-c1
  - subject: pure_being
  - predicate: excludes
  - object: posited_determination_or_content
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [36-39] posited content/distinction would prevent being from holding fast to purity.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: being-bei-003
  - note: exclusion of content leads to explicit emptiness and identity with nothing.
  - sourceClaimIds: [`being-bei-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`being-bei-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim moment for purity constraint.

### Entry being-bei-003 — Pure being III: identity with nothing

Span:

- sourceFile: `src/compiler/being/quality/being/being.txt`
- lineStart: 40
- lineEnd: 47

Summary:

Pure indeterminacy and emptiness culminate in explicit identity of being and nothing.

Key points: (KeyPoint)

- k1. Pure being is empty.
- k2. Emptiness removes determinacy.
- k3. Being is explicitly stated as nothing.

Claims: (Claim)

- c1. id: being-bei-003-c1
  - subject: indeterminate_immediate_being
  - predicate: is_identical_with
  - object: nothing
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [40-47] pure indeterminateness/emptiness leads to explicit statement that being is nothing.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: being-not-001
  - note: Part A closes with being=nothing, opening Part B from the side of nothing.
  - sourceClaimIds: [`being-bei-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`being-not-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim closure moment for Part A.
