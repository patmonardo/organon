# Workbook Contract (V1)

Status: active
Applies to: all Thingness workbooks in this folder
Priority: workbook markdown over generated projections

## Canonical authority

- Workbook markdown files are the authoritative Knowledge Graph artifacts.
- Any Cypher/DB/graph projection is derivative and non-authoritative.
- Source authority for claims is the named source text in each workbook.

## Stability rules

- Keep workbook format stable across sessions.
- Do not change section order, field names, or claim structure unless a structural migration is approved.
- If migration is approved, apply the same migration to every affected workbook in one pass.

## Structural migration protocol

- Record migration decision in each workbook `Decision:` section before edits.
- Set a migration label (example: `schema_v2 -> schema_v3`).
- Update all workbooks in scope in the same session.
- Add a brief compatibility note if old entries remain.

## Minimum claim discipline

- Every accepted claim must include line-anchored evidence.
- Keep claim count minimal and non-redundant.
- If uncertain, mark `review_pending`.

## KG notation discipline (pseudo-Cypher labels)

- Use explicit node-label notation in section headers:
  - `Key points: (KeyPoint)`
  - `Claims: (Claim)`
  - `Relations: (Relation)`
- Keep key-point ids stable (`k1`, `k2`, ... within entry scope).
- Keep claim ids stable (`<entry-id>-c1`, `<entry-id>-c2`, ...).
- Use direct visible claim bullets in each claim block (`- c1. id: <claim-id>`, `- c2. id: <claim-id>`, ...), with no extra claim-header field.
- Use direct visible relation bullets in each relation block (`- r1. type: <relation_type>`, `- r2. type: <relation_type>`, ...).
- Do not mix alternate label spellings once adopted.

## Exclusion discipline

- If a range/topic is excluded for a pass, write the excluded range explicitly in `Scope` or `Decision`.
- Excluded blocks must not be silently reintroduced.

## Thingness span policy (binding)

- Part A (`thing.txt`) has a mixed structure:
  - Subpart `a. The thing in itself and concrete existence` has explicit numbered movements `1/2/3`; span boundaries must obey those numbered boundaries.
  - Subparts `b. Property` and `c. The reciprocal action of things` do not have required numeric decomposition; use complete sentence-group spans without forcing artificial `1/2/3` splits.
- Part B (`matter.txt`) and Part C (`dissolution.txt`) are non-numbered in this source set; do not impose synthetic numeric chunking.

## Relation analysis overlay (V1.1, non-breaking)

- Relation blocks may be enriched without changing top-level entry schema.
- Keep existing relation fields (`type`, `targetEntryId`, `note`) intact.
- Optional relation-analysis fields:
  - `sourceClaimIds`: list of source claim ids that ground the relation.
  - `sourceKeyPointIds`: list of source key point ids that ground the relation.
  - `targetClaimIds`: list of target claim ids when known (`pending_cross_workbook` allowed).
  - `logicalOperator`: concise operator tag for inferential intent (`implicative_support|sequential_transition|contrastive_negation|sublative_transition|presuppositional_link`, etc.).
  - `analysisMode`: `first_order_claim_projection|second_order_inference`.
- First-order passes default to `analysisMode: first_order_claim_projection`.
- If relation semantics are provisional, keep `review_pending` and state uncertainty in `note`.

## Session discipline

- Each workbook must keep a short `Decision:` block describing current pass constraints.
- Constraints are considered binding for that session unless explicitly revised in the workbook.

## Source-restriction and cycle protocol

- Keep `Key points` and `Claims` source-restricted to the named source text and line-anchored evidence.
- Expand `Relations` in iterative cycles after base source-restricted entries are stable.
- Relations may include broader inferential structure (including hypothetical/prolog-like forms) only as second-order modeling artifacts, not as first-order source claims.
- Distinguish first-order and second-order status in notes when needed, and keep `review_pending` until relation semantics are accepted.
- Current sequencing default for this track: `Thing and its Properties -> Constitution out of Matters -> Dissolution of the Thing -> relation-expansion cycles`.
