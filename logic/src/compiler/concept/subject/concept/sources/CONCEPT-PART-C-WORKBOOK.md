# Concept Part C Workbook

Part: `C. SINGULAR`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `singular.txt` as authority.
- Every accepted claim must have line-anchored evidence.
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

## Session: 2026-02-20 (span-locked Singular pass)

Scope:

- file: `singular.txt`
- fixed range: lines `10-282`

Decision:

- Keep claims source-first and line-anchored.
- Reserve higher-order synthesis for `CONCEPT-IDEA-WORKBOOK.md` and notebook.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Span lock for this pass:
  - entry-1: `10-177`
  - entry-2: `179-227`
  - entry-3: `229-266`
  - transition-only: `268-282`

### Entry con-sub-c-001 — Section 1: singularity as self-mediating return

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 10
- lineEnd: 177

Summary:

Section 1 develops singularity as the concept's self-mediation and totalizing return against abstraction.

Key points: (KeyPoint)

- k1. Singularity appears as reflection of the concept out of determinateness into itself.
- k2. Abstraction's ascent to higher genus loses conceptual depth and truth.
- k3. Singularity restores inseparability of universal, particular, and singular as totality.

Claims: (Claim)

- c1. id: con-sub-c-001-c1
  - subject: singularity
  - predicate: is_determined_as
  - object: self_mediation_and_totalizing_return_of_concept
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [10-41] singularity as reflective return and critique of abstract ascent.
    - [43-177] inseparability and totality of concept determinations in singularity.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-sub-b-003
  - note: confirms Part B culmination in singularity.
  - sourceClaimIds: [`con-sub-c-001-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-sub-b-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-c-002
  - note: moves from reflective return to immediate actualization in section 2.
  - sourceClaimIds: [`con-sub-c-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`con-sub-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: entry-1 span locked at 10-177.

### Entry con-sub-c-002 — Section 2a: immediate loss into actuality

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 179
- lineEnd: 227

Summary:

The opening of section 2 determines singularity as immediate loss of the concept into actuality and as exclusive qualitative this.

Key points: (KeyPoint)

- k1. Singularity is immediate loss of concept into actuality while remaining self-referential negativity.
- k2. The singular appears as qualitative one/this and exclusion.
- k3. Universal-as-commonality is the lowest external relation to singularity.

Claims: (Claim)

- c1. id: con-sub-c-002-c1
  - subject: singularity
  - predicate: is_determined_as
  - object: immediate_actualizing_this_and_exclusive_negativity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [179-198] immediate loss into actuality; posited abstraction as determinate determinateness.
    - [200-227] singular as one/this, exclusion, and external-commonality interpretation.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-sub-c-001
  - note: concretizes section 1 return as actualized singular determinateness.
  - sourceClaimIds: [`con-sub-c-002-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-sub-c-001-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-c-003
  - note: advances to reflective synthesis of exclusion with essential universal relation.
  - sourceClaimIds: [`con-sub-c-002-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-sub-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: entry-2 span locked at 179-227.

### Entry con-sub-c-003 — Section 2b synthesis + judgment transition marker

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 229
- lineEnd: 282

Summary:

This entry captures reflective synthesis of singular exclusion with essential universality relation and preserves the final paragraph as transition-only to judgment.

Key points: (KeyPoint)

- k1. Entry span (`229-266`) synthesizes mediated immediacy and essential relation to universality.
- k2. Transition-only span (`268-282`) marks originative partition and positing as judgment.

Claims: (Claim)

- c1. id: con-sub-c-003-c1
  - subject: singular_abstraction
  - predicate: synthesizes
  - object: self_subsisting_difference_with_essential_universal_relation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [229-248] singular as mediated immediacy with internal separation and positive connection.
    - [250-266] abstraction posits concept-difference; exclusion refers essentially to universal.

Claim ↔ key point map:

- c1 -> k1

Transition (non-claim):

- t1. span: `268-282`
  - function: transition_to_judgment
  - note: this span is transition-only and is not encoded as a claim.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-sub-c-002
  - note: synthesis block grounds section 2 by uniting exclusion with essential relation.
  - sourceClaimIds: [`con-sub-c-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`con-sub-c-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-001
  - note: transition-only span (`268-282`) opens next movement to judgment.
  - sourceClaimIds: [`con-sub-c-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [pending]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: entry-3 span locked at 229-266; final paragraph retained as transition-only marker.
