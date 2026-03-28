# Life Idea Workbook

Part: `IDEA. CHAPTER 1 — LIFE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Life source files in this folder.
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

### Entry (Topic) <id> — <title>

- span: `<lineStart-lineEnd>`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-23 (initial Life chapter scaffold)

Scope:

- files:
  - `living-individual.txt`
  - `life-process.txt`
  - `genus.txt`
- focus: Chapter 1 (`Life`) as triadic development and explicit transition to cognition.

Decision:

- Keep this workbook chapter-level and transition-focused.
- Keep first-order claims anchored only to the three Life source files.
- Preserve boundary sentence to cognition as explicit transition gate.

### Entry con-idea-life-001 — Chapter opening: immediate life as concept-objectivity unity

Span:

- sourceFile: `src/compiler/concept/idea/life/living-individual.txt`
- lineStart: 4
- lineEnd: 86

Summary:

Life begins as immediate idea where concept and objectivity correspond only through negative self-relation that posits externality as its own moment.

Key points: (KeyPoint)

- k1. Life is immediate idea with concept-objectivity correspondence.
- k2. Correspondence holds only through negativity and self-determining diremption.
- k3. The living individual appears as singularized center of this unity.

Claims: (Claim)

- c1. id: con-idea-life-001-c1
  - subject: life
  - predicate: is_determined_as
  - object: immediate_idea_of_concept_objectivity_unity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [living-individual.txt:4-26] immediate idea and self-determining negativity.
    - [living-individual.txt:77-86] living individual as negative self-identity and centrality.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-life-002
  - note: immediate unity transitions into mediated life-process against externality.
  - sourceClaimIds: [`con-idea-life-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-life-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: opening determination anchored to Part A's first movement.

### Entry con-idea-life-002 — Mediation: contradiction, pain, and purposive assimilation

Span:

- sourceFile: `src/compiler/concept/idea/life/life-process.txt`
- lineStart: 30
- lineEnd: 158

Summary:

Life-process mediates immediate unity through contradiction and pain, then sublates externality by assimilation into reproduction.

Key points: (KeyPoint)

- k1. Need introduces contradiction as constitutive form of living self-determination.
- k2. Pain is the concrete existence of this contradiction.
- k3. External purposiveness is sublated into immanent concept, yielding reproduction.

Claims: (Claim)

- c1. id: con-idea-life-002-c1
  - subject: life_process
  - predicate: is_determined_as
  - object: mediation_of_contradiction_into_reproduction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [life-process.txt:30-68] contradiction and pain.
    - [life-process.txt:121-158] sublation of external purposiveness and reproduction.

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-life-001
  - note: mediated process clarifies how immediate life unity maintains itself in otherness.
  - sourceClaimIds: [`con-idea-life-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-life-001-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-003
  - note: reproduction and universality handoff into genus determination.
  - sourceClaimIds: [`con-idea-life-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: mediation determination consolidated around contradiction-to-reproduction arc.

### Entry con-idea-life-003 — Concrete universality: genus and self-relation through otherness

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 21
- lineEnd: 143

Summary:

In genus, life reaches concrete universality as identity with otherness, yet only through contradiction, longing, and reciprocal sublation of singularities.

Key points: (KeyPoint)

- k1. Genus is identity of individual with indifferent otherness.
- k2. Genus relation is contradictory in immediacy and therefore impulsive.
- k3. Realized genus emerges through sublation of singularity and reflective return.

Claims: (Claim)

- c1. id: con-idea-life-003-c1
  - subject: genus
  - predicate: is_determined_as
  - object: concrete_universality_via_contradictory_reciprocal_life_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [genus.txt:21-37] genus as identity with otherness and duplication.
    - [genus.txt:52-86] contradiction, impulse, and reflected negative unity.
    - [genus.txt:127-143] genus rejoining itself as explicit universality.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-002
  - note: genus universalization confirms life-process elevation from particularity.
  - sourceClaimIds: [`con-idea-life-003-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-idea-life-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-004
  - note: explicit universality of genus passes into cognition.
  - sourceClaimIds: [`con-idea-life-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: concrete universality fixed as chapter-level third determination.

### Entry con-idea-life-004 — Transition gate: death of immediate life and the idea of cognition

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 144
- lineEnd: 157

Summary:

The perishing of immediate living individuality becomes spirit's coming-to-be and marks the explicit transition from Life to Cognition.

Key points: (KeyPoint)

- k1. Immediate individuality perishes in copulation.
- k2. This death is the coming-to-be of spirit.
- k3. The explicit endpoint is the idea of cognition.

Claims: (Claim)

- c1. id: con-idea-life-004-c1
  - subject: life_to_cognition_transition
  - predicate: is_determined_as
  - object: death_of_immediate_life_as_becoming_of_spirit
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [genus.txt:144-147] death of immediate living individuality and becoming of spirit.
    - [genus.txt:148-157] explicit statement of the idea of cognition.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-003
  - note: transition gate is the realized consequence of genus universality becoming explicit.
  - sourceClaimIds: [`con-idea-life-004-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-life-003-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: boundary sentence preserved as chapter handoff to Cognition.
