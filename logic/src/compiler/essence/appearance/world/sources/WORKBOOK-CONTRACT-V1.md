# Workbook Contract (V1)

Status: active
Applies to: all World/Appearance workbooks in this folder
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

## Appearance/World span policy (binding)

- Part A (`law.txt`) has explicit numbered movements `1/2/3`; entry spans must obey these boundaries.
- Part B (`world.txt`) and Part C (`disappearance.txt`) use conceptual sentence-group spans unless explicit numbering is present.
- Transition handoffs between Thingness and World should be modeled as explicit `transitions_to` relations.

## Session discipline

- Each workbook must keep a short `Decision:` block describing current pass constraints.
- Constraints are binding for that session unless explicitly revised in the workbook.
