# Cognition Part B Workbook

Part: `B. THE IDEA OF THE GOOD`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `idea-of-the-good.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow coherent determination windows of complete sentence groups.

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

## Session: 2026-02-23 (initial Cognition Part B pass)

Scope:

- file: `idea-of-the-good.txt`
- fixed range: lines `4-364`

Decision:

- Part B has no internal source numbering/lettering; sectioning is analysis-driven only.
- Part B is sectioned into three analysis windows for initial extraction.
- Keep first pass conservative with minimal, line-anchored claims.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

Extraction map:

- b-001 (analysis): lines `4-108`
- b-002 (analysis): lines `109-220`
- b-003 (analysis): lines `221-364`

IDEA notebook intake (working map):

- intake-1: `4-50` (good as absolute practical determination)
- intake-2: `154-220` (ought/duality and completion pressure)
- intake-3: `221-364` (syllogism of action to absolute idea)

### Entry con-idea-cog-b-001 — b-001 (analysis): good as practical objective validity

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-good.txt`
- lineStart: 4
- lineEnd: 108

Summary:

The practical idea appears as singular self-determining purpose whose determinateness is the good, objectively valid yet still in subjective form.

Key points: (KeyPoint)

- k1. Practical idea stands as actual certainty of itself over against world-nullity.
- k2. The good is concept-determinateness demanding external actuality.
- k3. The good is intrinsically true in itself before external realization.

Claims: (Claim)

- c1. id: con-idea-cog-b-001-c1
  - subject: practical_idea
  - predicate: is_determined_as
  - object: singular_self_determining_purpose
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-27] singular subjective determination and certainty over non-actual world.
    - [40-50] good as absolute determination with objective dignity.

- c2. id: con-idea-cog-b-001-c2
  - subject: good
  - predicate: is_determined_as
  - object: intrinsically_true_particular_purpose
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [63-68] own determination posited in actuality.
    - [99-107] good valid in and for itself as true.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-cog-b-002
  - note: intrinsic validity moves into contradiction of realization and ought.
  - sourceClaimIds: [`con-idea-cog-b-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-cog-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: immediate determination established from opening practical idea block.

### Entry con-idea-cog-b-002 — b-002 (analysis): finitude of realization and unresolved ought

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-good.txt`
- lineStart: 109
- lineEnd: 220

Summary:

Realized good remains fragile in finite actuality, preserving the unresolved contradiction of ought and dual-world opposition.

Key points: (KeyPoint)

- k1. Realization of finite purpose remains means-like and externally exposed.
- k2. The good remains an ought under conditions of conflict, contingency, and resistance.
- k3. The practical idea lacks theoretical completion and points back to the true.

Claims: (Claim)

- c1. id: con-idea-cog-b-002-c1
  - subject: realized_good
  - predicate: remains
  - object: finite_fragile_and_conflictual
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [120-147] realized purpose remains means-like and vulnerable.
    - [154-168] good remains an ought across opposed worlds.

- c2. id: con-idea-cog-b-002-c2
  - subject: practical_idea
  - predicate: requires_completion_in
  - object: idea_of_the_true
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [185-193] practical idea lacks moment of theoretical idea.
    - [217-220] explicit completion in the idea of the true and self-transition.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-cog-b-001
  - note: shows limits internal to the immediate practical validity of the good.
  - sourceClaimIds: [`con-idea-cog-b-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-cog-b-001-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-cog-b-003
  - note: self-transition of the good opens the synthesis of mediated consummation.
  - sourceClaimIds: [`con-idea-cog-b-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-cog-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: mediation determination fixed around the ought contradiction.

### Entry con-idea-cog-b-003 — b-003 (analysis): mediated consummation and absolute idea threshold

Span:

- sourceFile: `src/compiler/concept/idea/cognition/idea-of-the-good.txt`
- lineStart: 221
- lineEnd: 364

Summary:

The syllogism of action is re-read so mediation consummates what immediacy already implies, sublating finitude and leading to the absolute idea.

Key points: (KeyPoint)

- k1. Practical syllogism unifies immediate communication and mediated realization.
- k2. The opposition of good and external being is progressively sublated.
- k3. The endpoint is explicit transition to the absolute idea.

Claims: (Claim)

- c1. id: con-idea-cog-b-003-c1
  - subject: syllogism_of_good
  - predicate: is_determined_as
  - object: mediation_of_immediate_identity_into_consummation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [221-260] two premises brought together for consummation of the good.
    - [267-298] actuality of the good through mediated realization.

- c2. id: con-idea-cog-b-003-c2
  - subject: practical_idea
  - predicate: culminates_in
  - object: absolute_idea
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [329-364] final transition and explicit endpoint statement.
    - [364-364] "This is the absolute idea."

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-cog-b-002
  - note: mediated consummation resolves the unresolved ought structure.
  - sourceClaimIds: [`con-idea-cog-b-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-cog-b-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: synthesis determination closes at absolute idea marker.
