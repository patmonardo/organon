# Being Part C Workbook

Part: `C. BECOMING`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `becoming.txt` as authority.
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

- file: `becoming.txt`
- fixed range: lines `1-118`
- pass policy: lock spans first, then first-order claim extraction

Decision:

- Adopt two-level IDs for this part:
  - Level 1 markers: `bei-c-a`, `bei-c-b`
  - Level 2 numbered entries: `bei-c-<letter>-<nnn>`
- Preserve explicit internal movement: unity of being/nothing -> moments of becoming.
- Keep cross-part transition closure available for export to Existence stage in later passes.

### Entry bei-c-a-001 — Becoming I: truth of being and nothing as movement

Span:

- sourceFile: `src/compiler/being/quality/being/sources/becoming.txt`
- lineStart: 12
- lineEnd: 30

Summary:

The first movement of Becoming states that being and nothing are the same yet distinct, and their truth is the vanishing movement of each into the other.

Key points: (KeyPoint)

- k1. Being and nothing are asserted as the same.
- k2. Their sameness includes absolute distinction and inseparability.
- k3. Their truth is movement: immediate vanishing into one another.

Claims: (Claim)

- c1. id: bei-c-a-001-c1
  - subject: truth_of_being_and_nothing
  - predicate: is
  - object: movement_of_passing_over_between_them
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [14-18] truth is not either side alone, but passed-over relation between being and nothing.
    - [27-30] truth is becoming as immediate vanishing of one into the other.

- c2. id: bei-c-a-001-c2
  - subject: being_and_nothing
  - predicate: are
  - object: absolutely_distinct_yet_unseparated
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [20-25] not the same as undifferentiated identity; distinct yet inseparable and mutually vanishing.

Claim ↔ key point map:

- c1 -> k1, k3
- c2 -> k2

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: bei-c-b-001
  - note: movement-truth is developed into explicit moment structure of becoming.
  - sourceClaimIds: [`bei-c-a-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`bei-c-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numbered entry for subsection `1. Unity of being and nothing`.

### Entry bei-c-b-001 — Becoming II: doubled moments as coming-to-be and ceasing-to-be

Span:

- sourceFile: `src/compiler/being/quality/being/sources/becoming.txt`
- lineStart: 32
- lineEnd: 81

Summary:

The second movement articulates becoming as unity of being and nothing in sublated moments, doubly determined as coming-to-be and ceasing-to-be that mutually interpenetrate.

Key points: (KeyPoint)

- k1. Becoming contains being and nothing as sublated yet distinguished moments.
- k2. Becoming is doubly determined as two directional passings-over.
- k3. Coming-to-be and ceasing-to-be interpenetrate and each sublates itself.

Claims: (Claim)

- c1. id: bei-c-b-001-c1
  - subject: becoming
  - predicate: contains
  - object: sublated_moments_of_being_and_nothing
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [34-45] becoming as determinate unity where moments are distinguished yet sublated.

- c2. id: bei-c-b-001-c2
  - subject: becoming
  - predicate: is_determined_as
  - object: coming_to_be_and_ceasing_to_be
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [56-64] doubled determination into transitions from nothing->being and being->nothing.

- c3. id: bei-c-b-001-c3
  - subject: coming_to_be_and_ceasing_to_be
  - predicate: interpenetrate_as
  - object: self_sublating_opposed_directions
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [66-81] opposite directions paralyze/interpenetrate; each sublates itself in itself.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: bei-c-b-002
  - note: doubled unrest proceeds to sublation into quiescent unity.
  - sourceClaimIds: [`bei-c-b-001-c2`, `bei-c-b-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`bei-c-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numbered entry for subsection `2. The moments of becoming`.

### Entry bei-c-b-002 — Becoming III: sublation of becoming into existence

Span:

- sourceFile: `src/compiler/being/quality/being/sources/becoming.txt`
- lineStart: 83
- lineEnd: 117

Summary:

The third movement shows becoming collapsing its unrest into quiescent simplicity, whose result is existence as the immediate unity of being and nothing.

Key points: (KeyPoint)

- k1. Becoming's unrest vanishes into quiescent unity.
- k2. The result is not nothing but quiescent unity of being and nothing.
- k3. This quiescent unity is explicitly named existence.

Claims: (Claim)

- c1. id: bei-c-b-002-c1
  - subject: becoming
  - predicate: sublates_into
  - object: quiescent_unity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [85-92] equilibrium and collapse of ceaseless unrest into quiescent result.

- c2. id: bei-c-b-002-c2
  - subject: result_of_becoming
  - predicate: is
  - object: quiescent_unity_of_being_and_nothing_not_nothing
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [103-110] result is not nothing but the unity become quiescent simplicity.

- c3. id: bei-c-b-002-c3
  - subject: becoming_transition_result
  - predicate: is_named
  - object: existence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [112-117] explicit conclusion: becoming as transition into this unity is existence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: bei-c-a-001
  - note: final sublation confirms that becoming is the truth of being/nothing relation.
  - sourceClaimIds: [`bei-c-b-002-c2`, `bei-c-b-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`bei-c-a-001-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: exi-001
  - note: becoming's result opens the next stage (`Existence`) in the quality sequence.
  - sourceClaimIds: [`bei-c-b-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: subsection `3. Sublation of becoming` stabilized for seed pass.
