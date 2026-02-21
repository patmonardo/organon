# Foundation Part C (TopicMap) Workbook (V2)

Part: `C. CONTRADICTION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `contradiction.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries must follow complete sentence groups (no mid-sentence start/end).

## TopicMap terminology contract

- Workbook = serialized artifact of one TopicMap.
- TopicMap = graph container (topics + typed relations) within the broader Knowledge Graph.
- Entry (Topic) = one topic node with id, title, key points, claims, and relations.
- Scope / section / span = textual referents for source inclusion boundaries.
- Chunk = informal analysis term only; do not use as a formal schema field.

## Chunk Analysis notation (simple)

- Symbols: `I` (Identity), `D` (Difference), `X` (Contradiction), `S` (Self-sublation / resolution), `G` (Ground).
- Entry-level `Move` records the dominant transition for that Entry (Topic): `A -> B`.
- Claim-level tagging is optional in this pass; use Entry-level `Move` as the canonical chunk-analysis marker.
- Use `I` when self-reference/self-identity is explicit; use `D` when exclusion/otherness is explicit.
- Use `X` only when identity and difference are asserted in one and the same respect.
- Use `S` for contradiction's self-resolution dynamics; use `G` when the result is explicitly ground.

## Working template

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-18 (first Part C pass)

Scope:

- file: `contradiction.txt`
- section range: lines `4-293`

Decision:

- Fresh TopicMap analysis from source text only.
- Enforce source numbering directly: section `1`, section `2`, section `3` each becomes one `Entry (Topic)`.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Relations expansion beyond local movement is deferred until Ground synthesis; keep only minimal transition/support links in this pass.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay for this workbook pass; keep claims unchanged.

### Entry fnd-ctr-c-001 — Opposition's self-subsistence is contradiction

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/contradiction.txt`
- lineStart: 4
- lineEnd: 131

Summary:

Section 1 shows that positive and negative, as self-subsisting opposites that both contain and exclude each other, are explicit contradiction.

Chunk Analysis:

- move: `D -> X`
- dominant: `X`

Key points: (KeyPoint)

- k1. Difference develops from moments into self-subsisting opposites in opposition.
- k2. Each side is whole opposition by containing the other within and excluding it.
- k3. This structure is contradiction, first implicit in difference and then posited in positive/negative.
- k4. The negative is presented as the whole opposition in explicitly posited form.

Claims: (Claim)

- c1. id: fnd-ctr-c-001-c1
  - subject: self_subsisting_determination_of_reflection
  - predicate: is
  - object: contradiction
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [25-39] self-subsistence both contains and excludes its other; "And so it is contradiction."

- c2. id: fnd-ctr-c-001-c2
  - subject: difference_and_opposition
  - predicate: culminate_as
  - object: posited_contradiction_in_positive_and_negative
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [41-48] "Difference as such is already implicitly contradiction... The positive and the negative, however, are the posited contradiction..."

- c3. id: fnd-ctr-c-001-c3
  - subject: negative
  - predicate: is_determined_as
  - object: whole_opposition_and_posited_contradiction
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [109-131] negative as posited contradiction and "The negative is therefore the whole opposition..."

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-007
  - note: makes explicit that opposition's self-subsistence is contradiction.
  - sourceClaimIds: [`fnd-ctr-c-001-c1`, `fnd-ctr-c-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-ctr-c-002
  - note: move from contradiction-posited to contradiction's self-resolution.
  - sourceClaimIds: [`fnd-ctr-c-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`fnd-ctr-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section boundary locked to source section `1`.

### Entry fnd-ctr-c-002 — Contradiction resolves through self-sublation

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/contradiction.txt`
- lineStart: 132
- lineEnd: 211

Summary:

Section 2 develops contradiction as ceaseless self-translation of opposites whose result is not mere nullity but self-subsistent unity through sublated positedness.

Chunk Analysis:

- move: `X -> S`
- dominant: `S`

Key points: (KeyPoint)

- k1. Each opposite self-sublates by passing over into its opposite.
- k2. Contradiction yields nullity but also positive result beyond mere null.
- k3. Self-subsistence is positedness that founders and is sublated as sublated positedness.
- k4. The achieved unity is self-identity through negation of itself.

Claims: (Claim)

- c1. id: fnd-ctr-c-002-c1
  - subject: positive_and_negative_in_contradiction
  - predicate: self_translate_into
  - object: their_opposite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [136-143] each sublates itself as "self-translating of itself into its opposite" and this vanishing first appears as null.

- c2. id: fnd-ctr-c-002-c2
  - subject: result_of_contradiction
  - predicate: is_not_only
  - object: null_but_also_positive_reflective_result
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [145-154] contradiction contains positive as well; "the result of contradiction is not only the null... founders to the ground in contradiction."

- c3. id: fnd-ctr-c-002-c3
  - subject: self_subsistence
  - predicate: becomes
  - object: self_unity_through_negation_of_its_own_positedness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [194-210] sublation of positedness as sublated positedness; unity through negation "not of an other, but of itself."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-ctr-c-001
  - note: shows contradiction's internal dynamic as self-sublation, not static antinomy.
  - sourceClaimIds: [`fnd-ctr-c-002-c1`, `fnd-ctr-c-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-ctr-c-001-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-ctr-c-003
  - note: move from resolved contradiction to explicit determination as ground.
  - sourceClaimIds: [`fnd-ctr-c-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`fnd-ctr-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section boundary locked to source section `2`.

### Entry fnd-ctr-c-003 — Resolved contradiction is ground

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/contradiction.txt`
- lineStart: 212
- lineEnd: 293

Summary:

Section 3 states that contradiction's resolution is essence as ground, where opposition is both removed and preserved in self-identical negativity.

Chunk Analysis:

- move: `S -> G`
- dominant: `G`

Key points: (KeyPoint)

- k1. Foundering opposition returns to foundation/ground.
- k2. As ground, essence is positedness that excludes and posits itself.
- k3. Ground is unity of positive and negative with contradiction preserved-and-removed.
- k4. Opposition was already ground implicitly, completed by explicit self-unity.

Claims: (Claim)

- c1. id: fnd-ctr-c-003-c1
  - subject: opposition_in_foundering
  - predicate: returns_to
  - object: foundation_ground
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [212-219] opposition founders and "has gone back to its foundation, to its ground."

- c2. id: fnd-ctr-c-003-c2
  - subject: essence_as_ground
  - predicate: is
  - object: excluding_reflection_and_positedness_that_posits_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [244-257] essence as ground is positedness and excluding reflection that "excludes itself from itself, it posits itself."

- c3. id: fnd-ctr-c-003-c3
  - subject: resolved_contradiction
  - predicate: is
  - object: ground_as_unity_of_positive_and_negative
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [267-276] "The resolved contradiction is therefore ground... opposition and its contradiction are just as much removed as preserved... Ground is essence as positive self-identity..."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-ctr-c-002
  - note: determines the positive result of contradiction explicitly as ground.
  - sourceClaimIds: [`fnd-ctr-c-003-c1`, `fnd-ctr-c-003-c3`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`fnd-ctr-c-002-c2`, `fnd-ctr-c-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-grd-d-001
  - note: handoff to Ground workbook where essence as ground is unfolded directly.
  - sourceClaimIds: [`fnd-ctr-c-003-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section boundary locked to source section `3`; ground handoff id provisional until Part D workbook exists.
