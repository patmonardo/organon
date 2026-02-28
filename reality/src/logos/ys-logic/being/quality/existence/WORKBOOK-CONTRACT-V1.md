# Workbook Contract (V1)

Status: active
Applies to: all Existence workbooks in this folder
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

## Human-scale extraction rule (Existence)

- This folder uses a strict anti-explosion policy.
- First pass target: 1 marker entry + 1-3 numbered entries per logical part.
- Maximum 3 claims per entry.
- Prefer larger coherent spans over micro-fragmentation.

## KG notation discipline

- Use explicit node-label notation in section headers:
  - `Key points: (KeyPoint)`
  - `Claims: (Claim)`
  - `Relations: (Relation)`
- Keep key-point ids stable (`k1`, `k2`, ... within entry scope).
- Keep claim ids stable (`<entry-id>-c1`, `<entry-id>-c2`, ...).
- Use direct visible claim bullets (`- c1. id: ...`) and relation bullets (`- r1. type: ...`).

## Session discipline

- Each workbook must keep a short `Decision:` block describing current pass constraints.
- Constraints are binding for that session unless explicitly revised.

## Source-restriction and cycle protocol

- Keep `Key points` and `Claims` source-restricted to named source text and line-anchored evidence.
- Expand `Relations` in iterative cycles after base entries are stable.
- Distinguish first-order claims from second-order overlays in notes when needed.
