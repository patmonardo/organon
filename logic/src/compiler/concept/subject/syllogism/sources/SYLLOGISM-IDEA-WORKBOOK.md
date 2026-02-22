# Syllogism Idea Workbook

Part: `IDEA. SYLLOGISM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Syllogism source files in this folder.
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

## Session: 2026-02-21 (initial scaffold)

Scope:

- files:
  - `existence.txt`
  - `reflection.txt`
  - `necessity.txt`
- focus: initialize Syllogism as final qualitative-logic transition toward Reason/Idea and seed precise SubTopic/Entry extraction.

Decision:

- Keep this workbook as the sphere-level Syllogism-IDEA coordinator.
- Keep Part A/B/C workbooks as first-order extraction artifacts.
- Represent all three syllogisms as both genera and species via marker SubTopics with numbered Entries.
- Track the U-U-U mathematical syllogism as an explicit fourth subspecies inside Part A (`d`).
- Keep Kant/Hegel comparative placement as a method note until all Part A claims are stabilized.

### Entry syl-idea-001 — Syllogism as closure of qualitative logic and transition to Reason

Span:

- sourceFile: `src/compiler/concept/subject/syllogism/sources/existence.txt`
- lineStart: 2
- lineEnd: 895

Summary:

Syllogism completes the qualitative-formal sequence while internally transitioning concept-determinations toward reflection, necessity, and Idea-level mediation.

Key points: (KeyPoint)

- k1. Syllogism is treated as end of qualitative/formal logic while preserving contentful determinations.
- k2. The three primary syllogisms are organized for extraction as both genera and species.
- k3. The U-U-U mathematical syllogism is retained as an immanent figure in the transition.

Claims: (Claim)

- c1. id: syl-idea-001-c1
  - subject: syllogism_of_existence
  - predicate: initiates
  - object: transition_from_immediate_formal_mediation_to_reflective_mediation
  - modality: asserted
  - confidence: 0.86
  - evidence:
    - [2-37] opening determination of immediate/formal syllogism and dialectical movement of mediation.
    - [874-895] transition from formal figures to syllogism of reflection.

- c2. id: syl-idea-001-c2
  - subject: fourth_figure_mathematical_syllogism
  - predicate: is_posited_as
  - object: integral_moment_of_syllogism_of_existence_development
  - modality: asserted
  - confidence: 0.86
  - evidence:
    - [785-873] explicit fourth figure U-U-U and its role.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: syl-exi-a
  - note: idea-level framing hands off to Part A first figure seed extraction.
  - sourceClaimIds: [`syl-idea-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`syl-exi-a-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: initialized with one special IDEA entry; refine confidence and comparative claims after Part A deep pass.
