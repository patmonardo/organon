# Quantity Part B Workbook

Part: `B. CONTINUOUS AND DISCRETE MAGNITUDE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `magnitude.txt` as authority.
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

- file: `magnitude.txt`
- fixed range: lines `1-59`
- pass policy: exactly three seed entries with one claim each

Decision:

- Use IDs: `quant-mag-001`, `quant-mag-002`, `quant-mag-003`.
- This part contains source-native numeric subsectioning; seed segmentation respects that structure.
- Keep handoff explicit to Part C (`quant-lim-001`).

### Entry quant-mag-001 — Magnitude I: immediate unity as continuous magnitude

Span:

- sourceFile: `src/compiler/being/quantity/quantity/magnitude.txt`
- lineStart: 4
- lineEnd: 24

Summary:

Quantity is first posited as immediate unity of continuity and discreteness under the determination of continuity, i.e. continuous magnitude.

Key points: (KeyPoint)

- k1. Quantity contains continuity and discreteness.
- k2. It is first posited in continuity.
- k3. Continuous magnitude is whole quantity.

Claims: (Claim)

- c1. id: quant-mag-001-c1
  - subject: immediate_quantity
  - predicate: is_posited_as
  - object: continuous_magnitude
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-9] quantity is initially posited in continuity as continuous magnitude.
    - [21-24] continuity is whole quantity as continuous magnitude.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: quant-mag-002
  - note: immediate continuity is negated as sufficient, requiring discrete magnitude.
  - sourceClaimIds: [`quant-mag-001-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`quant-mag-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim seed entry aligned to source subsection 1.

### Entry quant-mag-002 — Magnitude II: discrete magnitude as immanent determinateness

Span:

- sourceFile: `src/compiler/being/quantity/quantity/magnitude.txt`
- lineStart: 26
- lineEnd: 34

Summary:

Quantity is not merely immediate; in its immanent determinateness of the one it is discrete magnitude.

Key points: (KeyPoint)

- k1. Immediate quantity is insufficient.
- k2. Immanent determinateness is the one.
- k3. Quantity is therefore discrete magnitude.

Claims: (Claim)

- c1. id: quant-mag-002-c1
  - subject: quantity
  - predicate: is_determined_as
  - object: discrete_magnitude
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [26-34] immediate continuity is sublated into determinateness of the one, yielding discrete magnitude.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: quant-mag-003
  - note: discrete magnitude is further shown to retain continuity as unity of many ones.
  - sourceClaimIds: [`quant-mag-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`quant-mag-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim seed entry aligned to source subsection 2 opening.

### Entry quant-mag-003 — Magnitude III: discrete magnitude as continuity of many ones

Span:

- sourceFile: `src/compiler/being/quantity/quantity/magnitude.txt`
- lineStart: 35
- lineEnd: 58

Summary:

Discrete magnitude remains the whole quantity because its many ones are continuous in the same unity.

Key points: (KeyPoint)

- k1. Discreteness remains within unity with continuity.
- k2. Discrete outsideness is not atomistic void-repulsion.
- k3. Many ones are posited as many of one unity.

Claims: (Claim)

- c1. id: quant-mag-003-c1
  - subject: discrete_magnitude
  - predicate: contains
  - object: continuity_of_many_ones_in_same_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [49-58] discreteness is itself continuous; many ones are many of a unity.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: quant-lim-001
  - note: unity of many ones now appears as determinately limited quantity.
  - sourceClaimIds: [`quant-mag-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`quant-lim-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: one-claim seed entry aligned to source subsection 2 development.
