# Essence Part A (TopicMap) Workbook (V2)

Part: `A. THE ESSENTIAL AND THE UNESSENTIAL`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `essence.txt` as authority.
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

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-17 (fresh Part A pass)

Scope:

- file: `essence.txt`
- fixed range: lines `232-302` (end of Part A in this pass)

Decision:

- This is a fresh TopicMap analysis from source text only.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay for this workbook pass; keep claims unchanged.

### Entry ess-ref-a-001 — Essence is sublated being and determined negation

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/essence.txt`
- lineStart: 234
- lineEnd: 251

Summary:

Part A opens by defining essence as sublated being and determined negation, with being first set against it as what lacks in-and-for-itself status.

Key points: (KeyPoint)

- k1. Essence is immediately declared as sublated being.
- k2. Essence is self-equality as negation of the domain of immediate being-determinations.
- k3. Being appears over against essence as negative and not in-and-for-itself.
- k4. The first relation still presents both in an immediate comparative frame of equal value.

Claims: (Claim)

- c1. id: ess-ref-a-001-c1
  - subject: essence
  - predicate: is
  - object: sublated_being
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [234-234] "Essence is sublated being."

- c2. id: ess-ref-a-001-c2
  - subject: essence
  - predicate: is_determined_as
  - object: determined_negation_of_being
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [234-247] essence as negation of being-sphere and determined negation.

- c3. id: ess-ref-a-001-c3
  - subject: being_and_essence_in_first_comparison
  - predicate: stand_as
  - object: mutually_indifferent_equals_in_immediacy
  - modality: asserted
  - confidence: 0.9
  - evidence:
    - [248-251] being and essence stand in equal value according to their immediate being.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ess-ref-a-002
  - note: the comparative frame shifts into explicit essential/unessential split.
  - sourceClaimIds: [`ess-ref-a-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`ess-ref-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: sentence-complete boundary locked at line 251.
- interpretive note (outside formal claims): possible cognitive transition marker from outer intuition to inner intuition.

### Entry ess-ref-a-002 — Essential/unessential split relapses into external standpoint

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/essence.txt`
- lineStart: 253
- lineEnd: 281

Summary:

The essential/unessential distinction, taken as a partition within existence, is exposed as external positing dependent on viewpoint rather than intrinsic determination.

Key points: (KeyPoint)

- k1. Being is called unessential over against essence.
- k2. This contrast also makes "the essential" appear as another existence.
- k3. The distinction is described as an external taking-apart.
- k4. The same content may be judged essential or unessential from different standpoints.

Claims: (Claim)

- c1. id: ess-ref-a-002-c1
  - subject: essential_unessential_distinction
  - predicate: is
  - object: external_positing
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [271-281] the distinction is an external positing that leaves existence untouched.

- c2. id: ess-ref-a-002-c2
  - subject: essential_unessential_distinction
  - predicate: depends_on
  - object: external_standpoint
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [279-281] same content can be considered essential or unessential depending on standpoint.

- c3. id: ess-ref-a-002-c3
  - subject: essence_in_this_split
  - predicate: is_reduced_to
  - object: another_existence_called_essential
  - modality: asserted
  - confidence: 0.83
  - evidence:
    - [258-259] "essence itself is not essence proper but is just another existence, the essential."

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-a-001
  - note: explains why first determination is insufficient if held externally.
  - sourceClaimIds: [`ess-ref-a-002-c1`, `ess-ref-a-002-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-a-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-a-003
  - note: move to deeper determination of essence as absolute negativity.
  - sourceClaimIds: [`ess-ref-a-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`ess-ref-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: decide whether c3 should be modality `interpretive` or `asserted`.

### Entry ess-ref-a-003 — Absolute negativity resolves the split into non-essence (shine)

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/essence.txt`
- lineStart: 283
- lineEnd: 302

Summary:

On closer consideration, essence is absolute negativity of being; therefore what still appears immediate over against essence has no self-subsistence and is reduced to non-essence, shine.

Key points: (KeyPoint)

- k1. Essence, if merely contrasted, is only first negation.
- k2. The text deepens essence to absolute negativity of being.
- k3. Being does not persist except as what essence is.
- k4. The immediate opposed to essence is null in-and-for-itself.
- k5. This remainder is named non-essence (shine).

Claims: (Claim)

- c1. id: ess-ref-a-003-c1
  - subject: essence
  - predicate: is
  - object: absolute_negativity_of_being
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [295-300] essence as being that has sublated immediate being and immediate negation.

- c2. id: ess-ref-a-003-c2
  - subject: being_or_existence
  - predicate: does_not_persist_except_as
  - object: what_essence_is
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [301-301] being or existence does not persist except as what essence is.

- c3. id: ess-ref-a-003-c3
  - subject: immediate_opposed_to_essence
  - predicate: is_determined_as
  - object: non_essence_shine
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [302-302] immediate opposed to essence is null in-and-for-itself, non-essence, shine.

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: ess-ref-a-002
  - note: deeper determination of essence overcomes external essential/unessential partition.
  - sourceClaimIds: [`ess-ref-a-003-c1`, `ess-ref-a-003-c3`]
  - sourceKeyPointIds: [`k2`, `k4`, `k5`]
  - targetClaimIds: [`ess-ref-a-002-c1`]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-b-001
  - note: culminates in the immanent synthesis of shine itself, which Part B then unfolds explicitly.
  - sourceClaimIds: [`ess-ref-a-003-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: sentence-complete boundary locked at 283-302.
