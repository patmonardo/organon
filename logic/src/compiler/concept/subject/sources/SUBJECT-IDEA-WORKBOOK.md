# Subject Idea Workbook

Part: `IDEA. SUBJECT (member sphere of ROOT CONCEPT SPHERE)`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Source authority is limited to Concept source files in this folder.
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

## Session: 2026-02-20 (initial scaffold)

Scope:

- files:
  - `universal.txt`
  - `particular.txt`
  - `singular.txt`
- focus: Subject-IDEA framing as a member-sphere within the Root Concept Sphere (`Subject | Object | Idea`), with internal topic domains: Concept, Judgment, Syllogism.

Decision:

- Keep this workbook second-order and member-sphere-level (not root sphere).
- Keep first-order claims anchored only to Concept source lines available in this folder authority set.
- Treat this workbook as the parent map for the Subject member-sphere only.
- Treat the Root Concept Sphere as the superordinate sphere with three members: Subject, Object, Idea.
- Current status: only Chapter 1-focused `CONCEPT-IDEA-WORKBOOK.md` is active; dedicated Judgment/Syllogism workbooks are pending creation.

### Entry con-idea-001 — Concept genesis baseline for IDEA transitions

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/universal.txt`
- lineStart: 4
- lineEnd: 36

Summary:

The concept is established as self-originating universality through the return from being/essence into negation-of-negation.

Key points: (KeyPoint)

- k1. Concept genesis is presented as the return-movement out of being and essence.
- k2. Universality is self-reference through negativity, not an external presupposition.
- k3. This baseline is required for all subsequent transition modeling.

Claims: (Claim)

- c1. id: con-idea-001-c1
  - subject: concept
  - predicate: emerges_as
  - object: self_originating_universal_from_being_essence_genesis
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [universal.txt:4-15] concept introduced as absolutely infinite/unconditioned/free with genesis through being/essence.
    - [universal.txt:23-36] concept as negation-of-negation and universality as pure self-reference.

- c2. id: con-idea-001-c2
  - subject: universality
  - predicate: is_not
  - object: externally_given_axiomatic_start
  - modality: interpretive
  - confidence: 0.87
  - evidence:
    - [universal.txt:16-31] movement is self-mediated mutual penetration and immanent turning-back.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-002
  - note: genesis baseline transitions into true logical division as concept's own self-differentiation.
  - sourceClaimIds: [`con-idea-001-c1`, `con-idea-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: promoted from placeholder to active baseline entry.

### Entry con-idea-002 — Division node: true logical division of the concept

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/particular.txt`
- lineStart: 73
- lineEnd: 95

Summary:

Division is explicitly determined as the concept's own self-division into coordinated particulars grounded in simple negativity.

Key points: (KeyPoint)

- k1. There is one true logical division: concept self-division.
- k2. The two sides are coordinated particulars, not externally assembled classes.
- k3. Their opposition is one determinateness: negativity in the universal.

Claims: (Claim)

- c1. id: con-idea-002-c1
  - subject: true_logical_division
  - predicate: consists_in
  - object: concept_self_division_into_coordinated_particulars
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [particular.txt:73-84] no other true logical division than concept setting universality and particular as coordinated particulars.
    - [particular.txt:89-95] opposition is one determinateness: simple negativity.

- c2. id: con-idea-002-c2
  - subject: division_relation
  - predicate: is_determined_as
  - object: immanent_not_external_partition
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [particular.txt:40-47] particularity as universality is immanent connection and intrinsic totality.
    - [particular.txt:52-71] universal determines itself as particular and remains with itself.

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-001
  - note: specifies the operative form of genesis as division rather than static classification.
  - sourceClaimIds: [`con-idea-002-c1`, `con-idea-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-001-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-003
  - note: division node requires singular-depth recovery against abstract ascent.
  - sourceClaimIds: [`con-idea-002-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-idea-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marked as primary high-value compiler node.

### Entry con-idea-003 — Logos-genesis guard: singular depth against abstract ascent

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 10
- lineEnd: 41

Summary:

The text blocks abstraction from bypassing conceptual depth: true return occurs through singularity, not genus-ascent.

Key points: (KeyPoint)

- k1. Singularity is the concept's self-mediation and return.
- k2. Abstract ascent to higher genus is diagnosed as a false start.
- k3. Conceptual depth is preserved only through singular determination.

Claims: (Claim)

- c1. id: con-idea-003-c1
  - subject: singularity
  - predicate: functions_as
  - object: depth_principle_of_conceptual_return
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [singular.txt:10-24] singularity as concept's self-mediation and restored self-equality in absolute negativity.
    - [singular.txt:31-41] abstraction's ascent loses truth; singularity is depth where concept grasps itself.

- c2. id: con-idea-003-c2
  - subject: logos_genesis_method
  - predicate: forbids
  - object: skipping_singularity_in_transition_design
  - modality: interpretive
  - confidence: 0.89
  - evidence:
    - [singular.txt:31-41] explicit critique of abstraction that strays from the way of the concept.

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-002
  - note: protects division from collapsing into static taxonomy by enforcing singular depth.
  - sourceClaimIds: [`con-idea-003-c1`, `con-idea-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-idea-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-004
  - note: singular depth advances into explicit judgment transition marker.
  - sourceClaimIds: [`con-idea-003-c1`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-idea-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: designated non-skippable logos-genesis guard.

### Entry con-idea-004 — Transition gate: originative division into judgment

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 268
- lineEnd: 282

Summary:

The end of singularity is the originative partition in which concept is posited as judgment, opening the next movement of the Science.

Key points: (KeyPoint)

- k1. Concept loses immediate unity of moments as independent self-subsistents.
- k2. Turning-back into itself is simultaneously originative partition.
- k3. This partition is explicitly posited as judgment.

Claims: (Claim)

- c1. id: con-idea-004-c1
  - subject: concept_turning_back
  - predicate: is_posited_as
  - object: originative_partition_into_judgment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [singular.txt:268-282] concept no longer posited unity of moments; turning-back is originative partition, posited as judgment.

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-005
  - note: establishes judgment as required next-topic extraction target for IDEA pass.
  - sourceClaimIds: [`con-idea-004-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: gate claim fixed as hard transition marker for compiler flow.

### Entry con-idea-005 — Division roadmap for Concept / Judgment / Syllogism extraction

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/singular.txt`
- lineStart: 268
- lineEnd: 282

Summary:

The current source set secures the transition into judgment and sets the roadmap for full Division-topic extraction of Concept, Judgment, and Syllogism.

Key points: (KeyPoint)

- k1. Current in-folder authority fully anchors Concept triad and explicit transition to judgment.
- k2. Full chapter-level extraction for `Judgment` and `Syllogism` requires loading their source spans into this folder authority set.
- k3. IDEA workbook must preserve Division as a first-class topic node, not a skipped bridge.

Claims: (Claim)

- c1. id: con-idea-005-c1
  - subject: current_concept_sources
  - predicate: provide
  - object: authoritative_transition_anchor_to_judgment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [singular.txt:268-282] explicit positing of judgment as transition result.

- c2. id: con-idea-005-c2
  - subject: idea_workbook_method
  - predicate: requires
  - object: non_skipped_division_topic_and_follow_on_judgment_syllogism_passes
  - modality: interpretive
  - confidence: 0.88
  - evidence:
    - [particular.txt:73-95] true logical division is concept-internal and therefore method-critical.
    - [singular.txt:268-282] transition marker forces explicit next extraction movement.

Relations: (Relation)

- r1. type: presupposes
  - targetEntryId: con-idea-002
  - note: roadmap presupposes division-node determination.
  - sourceClaimIds: [`con-idea-005-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-002-c1`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: con-idea-004
  - note: turns transition marker into staged extraction directive.
  - sourceClaimIds: [`con-idea-005-c1`, `con-idea-005-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-004-c1`]
  - logicalOperator: constrained_refinement
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: full Judgment/Syllogism topicization queued for next pass once source spans are added to folder authority set.
