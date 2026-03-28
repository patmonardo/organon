# Workbook Contract (V1)

Status: active
Applies to: all Essential Relation workbooks in this folder
Priority: workbook markdown over generated projections

## Canonical authority

- Workbook markdown files are the authoritative Knowledge Graph artifacts.
- Any Cypher/DB/graph projection is derivative and non-authoritative.
- Source authority for claims is the named source text in each workbook.

## Stability rules

- Keep workbook format stable across sessions.
- Do not change section order, field names, or claim structure unless a structural migration is approved.
- If migration is approved, apply the same migration to every affected workbook in one pass.

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
- Use direct visible claim bullets in each claim block (`- c1. id: <claim-id>`, `- c2. id: <claim-id>`, ...).
- Use direct visible relation bullets in each relation block (`- r1. type: <relation_type>`, `- r2. type: <relation_type>`, ...).

## Essential Relation span policy (binding)

- Chapter-level ideas and section-level ideas are maintained in the dedicated Idea workbook and are not merged into part-analysis entries.
- Part A (`whole-parts.txt`) decomposition follows explicit textual markers where available.
- Part B (`force-expression.txt`) and Part C (`outer-inner.txt`) use explicit sub-markers where present, else conceptual sentence-group boundaries.

## Session discipline

- Each workbook keeps a short `Decision:` block describing active pass constraints.
- Constraints are binding for that session unless explicitly revised in the workbook.
