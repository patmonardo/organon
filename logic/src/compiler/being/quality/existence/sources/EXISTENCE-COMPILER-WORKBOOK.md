# Existence Compiler Workbook

Part: `COMPILER. EXISTENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/existence/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for compiler-facing Existence modeling.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Existence source files in this folder.
- Claims must be line-anchored.
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

## Session: 2026-02-22 (initial scaffold)

Scope:

- files:
  - `existence.txt`
  - `something-and-other.txt`
  - `constitution.txt`
  - `finitude.txt`
  - `infinity.txt`
  - `alternating-infinity.txt`
  - `affirmative-infinity.txt`
- focus: compiler-oriented relation normalization for Existence (`Transition` and `TruthOf` alignment)

Decision:

- Maintain claim-level compatibility with `EXISTENCE-PART-A/B/C-WORKBOOK.md`.
- Use this file for compiler-facing relation normalization once part claims stabilize.
- Prioritize stable labeled-subtopic mappings before numbered subtopic expansion.
- Avoid algorithmic/generated content without source-anchored claims.

Review outcome:

- review_pending
- notes: initialized; ready for relation-schema pass after first-order review.

### Entry exi-compiler-001 — Compiler coordinator: normalize `Transition` and `TruthOf` over Part A/B/C

Span:

- sourceFile: `src/compiler/being/quality/existence/sources/existence.txt`
- lineStart: 4
- lineEnd: 13

Summary:

This coordinator establishes a compiler-facing scaffold where Existence part progression is represented by `Transition` links and stabilized determinations are represented by `TruthOf` anchors, with full inferential expansion deferred until whole-sphere review.

Key points: (KeyPoint)

- k1. The source itself presents Existence as an ordered articulation (`a/b/c`) suitable for transition modeling.
- k2. Finitude is explicitly developed as contradiction and transition toward infinity, supporting directed transition edges.
- k3. True infinity is presented as resolved unity, supporting truth-anchoring semantics beyond bad infinite alternation.

Claims: (Claim)

- c1. id: exi-compiler-001-c1
  - subject: existence_part_structure
  - predicate: supports
  - object: transition_normalization_over_a_b_c_movements
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-13] explicit `a/b/c` schema in `existence.txt`.

- c2. id: exi-compiler-001-c2
  - subject: finite_contradiction_development
  - predicate: supports
  - object: directed_transition_from_finitude_to_infinity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [268-310] finite contradiction and transition into infinite in `finitude.txt`.

- c3. id: exi-compiler-001-c3
  - subject: affirmative_infinite_resolution
  - predicate: supports
  - object: truthof_anchor_for_resolved_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [306-314] negation of joint finitude yields true infinite in `affirmative-infinity.txt`.
    - [372-382] circle image as self-return of true infinite in `affirmative-infinity.txt`.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: exi-a
  - note: transition normalization starts from stabilized Part A marker structure.
  - sourceClaimIds: [`exi-compiler-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`exi-a-c1`, `exi-a-c2`, `exi-a-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: exi-b
  - note: directed finite contradiction movement supplies core transition contract.
  - sourceClaimIds: [`exi-compiler-001-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`exi-b-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: supports
  - targetEntryId: exi-c
  - note: true infinite resolution provides primary TruthOf anchor for Part C closure.
  - sourceClaimIds: [`exi-compiler-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`exi-c-c3`, `exi-c-003-c2`, `exi-c-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: scaffold-level compiler coordination only; full relation analysis deferred until all part workbooks are generated and reviewed as a whole.
