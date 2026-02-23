# Workbook Contract V1

Domain: `Being -> Quantity -> Quantity`
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

## Seed policy for Quantity

- Quantity is treated here as language of quantity (conceptual articulation), not kernel computation.
- For this seed pass, each Part must contain exactly 3 Entries.
- One seed claim per entry is valid and preferred.
- Only Part B uses explicit source-native numeric subsectioning.
- Part A and Part C triads are analytic segmentations, not additional source-labeled species.
