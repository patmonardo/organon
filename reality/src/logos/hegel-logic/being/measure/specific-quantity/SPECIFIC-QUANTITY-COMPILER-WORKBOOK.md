# Specific Quantity Compiler Workbook

Part: `COMPILER. SPECIFIC QUANTITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/specific-quantity/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for compiler-facing Specific Quantity modeling.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Specific Quantity source files in this folder.
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

## Session: 2026-02-23 (initial scaffold)

Scope:

- files:
  - `specific-quantum.txt`
  - `specifying-measure.txt`
  - `being-for-itself.txt`
- focus: compiler-oriented relation normalization for Specific Quantity (`Measure`, `Ratio`, `Exponent`, and transition edges toward Becoming-Essence).

Decision:

- Maintain claim-level compatibility with `SPECIFIC-QUANTITY-PART-A/B/C-WORKBOOK.md`.
- Use this file for compiler-facing relation normalization once part claims stabilize.
- Avoid algorithmic/generated content without source-anchored claims.

Review outcome:

- review_pending
- notes: initialized; ready for relation-schema pass after first-order review.
