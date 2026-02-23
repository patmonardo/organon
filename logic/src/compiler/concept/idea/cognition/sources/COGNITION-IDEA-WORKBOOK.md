# Cognition Idea Workbook

Part: `IDEA. CHAPTER 2 — COGNITION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Cognition source files in this folder.
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

## Session: 2026-02-23 (initial Cognition chapter scaffold)

Scope:

- files:
  - `cognition-idea.txt`
  - `idea-of-the-true.txt`
  - `idea-of-the-good.txt`
- focus: two-part Cognition chapter (`true`, `good`) and transition to absolute idea.

Decision:

- This chapter is modeled with two parts only (`A`, `B`), intentionally excluding a Part C workbook.
- Keep chapter-level synthesis compact and transition-focused.
- Preserve explicit endpoint marker to absolute idea.

### Entry con-idea-cog-001 — Chapter opening: life judged into cognition

Span:

- sourceFile: `src/compiler/concept/idea/cognition/sources/cognition-idea.txt`
- lineStart: 6
- lineEnd: 41

Summary:

Chapter 2 opens by deriving cognition from life through the idea's judgment, where concept confronts itself as subject matter.

Key points: (KeyPoint)

- k1. Life is immediate idea and cognition arises as the idea's judgment.
- k2. The concept's object is the concept itself in liberated universality.
- k3. The idea is doubled into subjective concept and objective concept-as-life.

Claims: (Claim)

- c1. id: con-idea-cog-001-c1
  - subject: cognition_chapter_opening
  - predicate: is_determined_as
  - object: judgmental_doubling_of_idea_from_life
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [cognition-idea.txt:6-9] life as immediate idea, idea in judgment as cognition.
    - [cognition-idea.txt:33-37] doubling into subjective and objective concept.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-cog-002
  - note: chapter opening passes into theoretical form as the idea of the true.
  - sourceClaimIds: [`con-idea-cog-001-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-cog-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: opening chapter move fixed as life-to-cognition gate.

### Entry con-idea-cog-002 — First chapter moment: idea of the true

Span:

- sourceFile: `src/compiler/concept/idea/cognition/sources/idea-of-the-true.txt`
- lineStart: 4
- lineEnd: 2197

Summary:

The idea of the true develops cognition from finite impulse through analytic and synthetic forms toward higher truth movement.

Key points: (KeyPoint)

- k1. Subjective idea begins as impulse of truth.
- k2. Analytic cognition is necessary but limited.
- k3. Synthetic cognition carries the concrete progression toward higher transition.

Claims: (Claim)

- c1. id: con-idea-cog-002-c1
  - subject: idea_of_the_true
  - predicate: is_determined_as
  - object: theoretical_cognition_progression
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [idea-of-the-true.txt:46-50] truth impulse as theoretical idea.
    - [idea-of-the-true.txt:223-225] analytic cognition marker.
    - [idea-of-the-true.txt:678-678] synthetic cognition marker.

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-cog-001
  - note: specifies how chapter opening's cognition-form first appears as truth-cognition.
  - sourceClaimIds: [`con-idea-cog-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-cog-001-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-cog-003
  - note: theoretical completion requires practical idea of the good.
  - sourceClaimIds: [`con-idea-cog-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-cog-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first chapter moment consolidated over full Part A source.

### Entry con-idea-cog-003 — Second chapter moment: idea of the good and self-transition

Span:

- sourceFile: `src/compiler/concept/idea/cognition/sources/idea-of-the-good.txt`
- lineStart: 4
- lineEnd: 364

Summary:

The idea of the good develops practical actuality and its finite contradiction, then transitions through mediated realization to the threshold of the absolute idea.

Key points: (KeyPoint)

- k1. Good is practical, self-determining objective validity.
- k2. Realization of good remains finite as ought under opposition.
- k3. Through mediation, the good passes into absolute idea.

Claims: (Claim)

- c1. id: con-idea-cog-003-c1
  - subject: idea_of_the_good
  - predicate: is_determined_as
  - object: practical_idea_that_self_transitions_to_absolute_idea
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [idea-of-the-good.txt:43-50] determination of the good and practical superiority.
    - [idea-of-the-good.txt:154-162] unresolved ought and subjective encumbrance.
    - [idea-of-the-good.txt:364-364] explicit endpoint in absolute idea.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-cog-002
  - note: practical contradiction confirms limits of purely theoretical completion.
  - sourceClaimIds: [`con-idea-cog-003-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`con-idea-cog-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-cog-004
  - note: practical consummation opens explicit absolute-idea gate.
  - sourceClaimIds: [`con-idea-cog-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-cog-004-c1`, `con-idea-cog-b-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second chapter moment fixed as practical transition arc.

### Entry con-idea-cog-004 — Transition gate: completion into absolute idea

Span:

- sourceFile: `src/compiler/concept/idea/cognition/sources/idea-of-the-good.txt`
- lineStart: 329
- lineEnd: 364

Summary:

The concluding movement of the good sublates remaining finitude and states the explicit completion in the absolute idea.

Key points: (KeyPoint)

- k1. Final mediation resolves opposition of subjective purpose and objective actuality.
- k2. Completion requires sublation of finitude in the good.
- k3. The explicit endpoint is the absolute idea.

Claims: (Claim)

- c1. id: con-idea-cog-004-c1
  - subject: cognition_to_absolute_idea_transition
  - predicate: is_determined_as
  - object: explicit_completion_of_practical_theoretical_unity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [idea-of-the-good.txt:329-363] closing transition argument.
    - [idea-of-the-good.txt:364-364] "This is the absolute idea."

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-cog-003
  - note: closing gate confirms practical idea's own consummation claim.
  - sourceClaimIds: [`con-idea-cog-004-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-cog-003-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: boundary line fixed as chapter-level handoff to absolute idea.
