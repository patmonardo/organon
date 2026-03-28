# Workbook Contract V1

Domain: `Being -> Quantity -> Quantum`
Status: active
Authority: source text only

## Authoritative artifact

- Workbook markdown files are the authoritative Knowledge Graph artifacts for this domain.
- Any generated graph/DB/Cypher output is derivative and non-authoritative.
- Schema order in each workbook must remain stable unless a migration is explicitly recorded.

## Clean-room constraints

- Use only local source `.txt` files in this folder as authority.
- Every accepted claim must include line-anchored evidence.
- If uncertain, mark `review_pending` and include an open question.
- Span boundaries must follow complete sentence groups.

## Entry schema (required order)

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- claims-to-keyPoint map
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`)
- review outcome

## Seed policy for Quantum

- Quantum is a denser moment inside Being/Quantity language and may include explicit subspecies (`a/b/c`) and numeric moments (`1/2/3`).
- Preserve source-labeled subspecies and numeric labels when present.
- Where a subspecies lacks numeric triads, introduce analytic ordinary entries to create triadic seed coverage.
- One seed claim per ordinary entry is valid.
- Do not type subspecies as genera in this pass; typing refinement is deferred to IDEA semantics pass.
