# Speculation Part B Workbook

Part: `B. METHOD ADVANCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `method-advance.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow complete sentence groups and movement transitions.

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

## Session: 2026-02-23 (final chapter initial pass)

Scope:

- file: `method-advance.txt`
- fixed range: lines `1-735`

Decision:

- Part B is extracted from `method-advance.txt` as three entries (`b-001..003`).
- Sectioning follows movement seams: setup → dialectical deepening → systematic return.
- Keep first pass conservative with minimal, line-anchored claims.

Extraction map:

- b-001: `1-230`
- b-002: `231-515`
- b-003: `516-735`

### Entry con-idea-spec-b-001 — b-001: concrete beginning and analytic/synthetic method setup

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 1
- lineEnd: 230

Summary:

The concrete beginning is shown as differentiated universality whose immanent method is at once analytic and synthetic.

Key points: (KeyPoint)

- k1. Concrete totality contains its own advance from the start.
- k2. Absolute method finds determinateness immanently in subject matter.
- k3. Dialectical moment is introduced as universal self-othering.

Claims: (Claim)

- c1. id: con-idea-spec-b-001-c1
  - subject: absolute_method
  - predicate: is_determined_as
  - object: immanent_analytic_and_synthetic_movement
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [1-33] beginning and analytic/synthetic setup.
    - [183-204] dialectical moment as self-determining otherness.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-spec-b-002
  - note: setup advances into full dialectical critique and negative determination.
  - sourceClaimIds: [`con-idea-spec-b-001-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-spec-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first movement of method-advance fixed.

### Entry con-idea-spec-b-002 — b-002: dialectic, contradiction, and negative mediation

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 231
- lineEnd: 515

Summary:

This movement defends dialectic as objective concept-activity and develops negativity as the turning-point of mediation and subjectivity.

Key points: (KeyPoint)

- k1. Dialectic is objective and necessary, not subjective artifice.
- k2. Fixed oppositions are intrinsically self-transitioning.
- k3. Negative mediation is the soul of living/spiritual movement.

Claims: (Claim)

- c1. id: con-idea-spec-b-002-c1
  - subject: dialectic
  - predicate: is_determined_as
  - object: objective_necessity_of_conceptual_movement
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [231-320] critique of subjective and skeptical reductions.
    - [333-366] concept moves fixed oppositions from within.

- c2. id: con-idea-spec-b-002-c2
  - subject: negativity
  - predicate: is_determined_as
  - object: turning_point_of_absolute_mediation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [425-444] mediated/negative determination and contradiction.
    - [468-515] negative as dialectical soul and mediating factor.

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-spec-b-001
  - note: demonstrates concretely how the initial analytic/synthetic claim becomes dialectical necessity.
  - sourceClaimIds: [`con-idea-spec-b-002-c1`, `con-idea-spec-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-idea-spec-b-001-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-spec-b-003
  - note: turning-point negativity unfolds into result, system, and circular closure.
  - sourceClaimIds: [`con-idea-spec-b-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-spec-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: central dialectical deepening fixed.

### Entry con-idea-spec-b-003 — b-003: result restoration, expansion, and circular closure

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 516
- lineEnd: 735

Summary:

The method returns into itself as restored immediacy through mediation, expands into a system, and culminates in the circular return of absolute idea.

Key points: (KeyPoint)

- k1. Negativity restores first immediacy as mediated truth.
- k2. Method expands into systematic totality.
- k3. Logic returns to beginning in absolute idea as circle of circles.

Claims: (Claim)

- c1. id: con-idea-spec-b-003-c1
  - subject: method_result
  - predicate: is_determined_as
  - object: restored_immediacy_and_systematic_totality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [516-573] return, restoration, and mediated beginning.
    - [578-614] onward enrichment and intensification.

- c2. id: con-idea-spec-b-003-c2
  - subject: logical_science
  - predicate: culminates_in
  - object: circle_of_circles_absolute_idea_return
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [707-714] science as circle of circles.
    - [721-735] return of logic in absolute idea and fulfilled concept.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-spec-b-002
  - note: circular closure confirms that negative mediation is productive truth-motion.
  - sourceClaimIds: [`con-idea-spec-b-003-c1`, `con-idea-spec-b-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-spec-b-002-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: final movement of method-advance fixed.
