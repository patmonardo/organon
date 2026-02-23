# Speculation Part C Workbook

Part: `C. METHOD ADVANCE (ANALYSIS EXTRACTION)`
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
- Span boundaries follow analysis windows, not explicit source subspecies.

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

## Session: 2026-02-23 (analysis pass over method-advance)

Scope:

- file: `method-advance.txt`
- fixed range: lines `1-735`

Decision:

- Part C is a second extraction of `method-advance.txt`, sectioned strictly by analysis windows (`c-001..003`).
- This pass emphasizes method-logic pivots over source-structural seams.
- Keep claims minimal and relation-rich for downstream operation mapping.

Extraction map:

- c-001: `1-199`
- c-002: `201-444`
- c-003: `445-735`

### Entry con-idea-spec-c-001 — c-001: universal beginning and analytic/synthetic immanence

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 1
- lineEnd: 199

Summary:

The opening analysis window frames concrete universality as beginning that already contains analytic/synthetic immanence and objective method.

Key points: (KeyPoint)

- k1. Beginning is concrete totality with intrinsic development.
- k2. Absolute method draws determinations immanently from subject matter.
- k3. Analytic and synthetic are internally unified at method-level.

Claims: (Claim)

- c1. id: con-idea-spec-c-001-c1
  - subject: method_beginning
  - predicate: is_determined_as
  - object: concrete_universality_with_immanent_advance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [1-33] concrete beginning and method setup.
    - [183-199] analytic/synthetic unity in concept-objectivity.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-spec-c-002
  - note: immanent setup opens full dialectical critique of fixed determinations.
  - sourceClaimIds: [`con-idea-spec-c-001-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-spec-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first analysis window fixed.

### Entry con-idea-spec-c-002 — c-002: dialectical critique and preservation-through-negativity

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 201
- lineEnd: 444

Summary:

This window analyzes dialectic as objective movement where contradiction and negativity sublate fixed oppositions while preserving positive content.

Key points: (KeyPoint)

- k1. Dialectic is objective determination, not subjective illusion.
- k2. Fixed oppositions are transitions in and for themselves.
- k3. Negative mediation preserves the first within the second.

Claims: (Claim)

- c1. id: con-idea-spec-c-002-c1
  - subject: dialectical_negativity
  - predicate: is_determined_as
  - object: preserving_sublation_of_fixed_determinations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [201-320] objective dialectic and critique of external conjunction.
    - [377-391] positive preserved in the negative.
    - [425-444] mediated negative determination and posited contradiction.

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-spec-c-001
  - note: concretizes the opening claim by showing the immanent engine of method advance.
  - sourceClaimIds: [`con-idea-spec-c-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-spec-c-001-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-spec-c-003
  - note: negativity unfolds into result, circularity, and systematic closure.
  - sourceClaimIds: [`con-idea-spec-c-002-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-spec-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second analysis window fixed.

### Entry con-idea-spec-c-003 — c-003: self-returning method, circle form, and absolute idea closure

Span:

- sourceFile: `src/compiler/concept/idea/speculation/sources/method-advance.txt`
- lineStart: 445
- lineEnd: 735

Summary:

The final analysis window develops the return from negativity to restored immediacy, the system-form of method, and logic's circular closure in absolute idea.

Key points: (KeyPoint)

- k1. Negative self-reference turns method back into itself as restored immediacy.
- k2. Method expands as system with deduced content.
- k3. Science is circular self-mediation culminating in absolute idea.

Claims: (Claim)

- c1. id: con-idea-spec-c-003-c1
  - subject: method_closure
  - predicate: is_determined_as
  - object: circular_systematic_return_of_absolute_idea
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [516-573] restoration of immediacy through sublating mediation.
    - [707-714] circle of circles form.
    - [721-735] logical return in absolute idea.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-spec-c-002
  - note: closure demonstrates the positive truth-content of dialectical negativity.
  - sourceClaimIds: [`con-idea-spec-c-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-spec-c-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: final analysis window fixed.
