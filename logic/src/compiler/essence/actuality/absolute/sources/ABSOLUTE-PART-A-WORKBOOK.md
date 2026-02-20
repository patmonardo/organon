# Absolute Part A (TopicMap) Workbook (V1)

Part: `A. THE EXPOSITION OF THE ABSOLUTE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `exposition.txt` as authority for Part A.
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
- claims: (Claim) minimum 3, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-20 (initial full Part A pass)

Scope:

- file: `exposition.txt`
- active range: lines `1-end` (`A. THE EXPOSITION OF THE ABSOLUTE`)

Decision:

- Complete Part A in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `abs-part-a-001` for lines `3-75`
- completed: `abs-part-a-002` for lines `77-144`
- completed: `abs-part-a-003` for lines `146-181`

### Entry abs-part-a-001 — Absolute as unity of being and essence

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/exposition.txt`
- lineStart: 3
- lineEnd: 75

Summary:

The opening establishes the absolute as the unity and ground of being/essence, form/content, and inner/outer.

Key points: (KeyPoint)

- k1. The absolute is neither immediate being nor reflected essence taken separately.
- k2. The absolute is the unity that grounds essential relation and brings being/essence together.
- k3. Absolute identity means each determinateness is totality, i.e., transparent reflective shine.
- k4. The absolute is both absolute form and absolute content in one substantial identity.
- k5. Reflection's movement is sublated into absolute identity as its inner/outer moment.

Claims: (Claim)

- c1. id: abs-part-a-001-c1
  - subject: absolute
  - predicate: is_not
  - object: mere_being_or_mere_essence
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [3-7] "The absolute is not just being, nor even essence... each is explicitly a totality, but a determinate totality."

- c2. id: abs-part-a-001-c2
  - subject: absolute
  - predicate: is
  - object: unity_of_being_and_essence_as_ground_of_relation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [17-22] "The absolute itself is the absolute unity of the two... the ground of the essential relation..."

- c3. id: abs-part-a-001-c3
  - subject: absolute_identity
  - predicate: entails
  - object: each_determinateness_is_totality_and_reflective_shine
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [39-45] "each of its parts is itself the whole... determinateness has become... transparent reflective shine..."

- c4. id: abs-part-a-001-c4
  - subject: reflection_movement
  - predicate: is_sublated_as
  - object: inner_outer_moment_of_absolute_identity
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [71-75] "The movement is sublated in this identity and is thus only its inner; but consequently its outer."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: abs-part-a-002
  - note: from absolute identity to the explicit negative/positive exposition movement.
  - sourceClaimIds: [abs-part-a-001-c3, abs-part-a-001-c4]
  - sourceKeyPointIds: [k3, k5]
  - targetClaimIds: [abs-part-a-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry abs-part-a-002 — Negative and positive exposition as finite foundering and shine

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/exposition.txt`
- lineStart: 77
- lineEnd: 144

Summary:

Reflection first presents a negative exposition of the absolute and then a positive one, but the positive remains reflective shine.

Key points: (KeyPoint)

- k1. Initial movement is the sublation of reflective activity into the absolute.
- k2. Negative exposition recapitulates the prior logical movement of being and essence as internally necessary.
- k3. Finite determinations are shown to have the absolute as abyss and ground.
- k4. Positive exposition treats finite as expression/copy of the absolute.
- k5. This positive exposition collapses into reflective shine and external starting form.

Claims: (Claim)

- c1. id: abs-part-a-002-c1
  - subject: initial_reflective_movement
  - predicate: is
  - object: negative_exposition_of_absolute
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [77-83] "At first... movement consists only in sublating its act in the absolute... negative exposition of the absolute."

- c2. id: abs-part-a-002-c2
  - subject: exposition
  - predicate: is
  - object: immanent_return_of_being_and_essence_into_absolute_ground
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [84-96] "the preceding whole... has determined itself within it... has returned into the absolute as into its ground."

- c3. id: abs-part-a-002-c3
  - subject: finite_determinations
  - predicate: have
  - object: absolute_as_abyss_and_ground
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [98-108] "the finite demonstrates... referred to the absolute... absolute for their abyss, but also for their ground..."

- c4. id: abs-part-a-002-c4
  - subject: positive_exposition
  - predicate: is_reduced_to
  - object: reflective_shine_with_external_starting_point
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [126-135] "This positive exposition... is only a reflective shine... form... a nullity... starting point for its activity."
    - [142-144] "starting point... a determination external to the absolute."

Relations: (Relation)

- r1. type: supports
  - targetEntryId: abs-part-a-001
  - note: finite determinations are grounded only in the absolute, reinforcing absolute identity.
  - sourceClaimIds: [abs-part-a-002-c2, abs-part-a-002-c3]
  - sourceKeyPointIds: [k2, k3]
  - targetClaimIds: [abs-part-a-001-c2, abs-part-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: abs-part-a-003
  - note: critique of external positivity transitions to absolute self-exposition.
  - sourceClaimIds: [abs-part-a-002-c4]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [abs-part-a-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keep `c4` wording stable for second-order relation cycles.

### Entry abs-part-a-003 — Absolute self-exposition beyond abstract identity

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/exposition.txt`
- lineStart: 146
- lineEnd: 181

Summary:

The exposition is re-grounded as the absolute's own act, and mere identity is corrected into identity-of-inner-and-outer as absolute form.

Key points: (KeyPoint)

- k1. In truth, exposition is the absolute's own doing from itself to itself.
- k2. Absolute as mere identity is only a determined absolute (attribute), not the absolutely absolute.
- k3. External reflection is immediately internal to the absolute.
- k4. The absolute is not abstract identity but identity of being/essence and inner/outer.

Claims: (Claim)

- c1. id: abs-part-a-003-c1
  - subject: exposition_of_absolute
  - predicate: is
  - object: absolutes_own_self_movement
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [146-150] "the exposition of the absolute is the absolute's own doing, an act that begins from itself and arrives at itself."

- c2. id: abs-part-a-003-c2
  - subject: absolute_as_mere_identity
  - predicate: is
  - object: determined_absolute_called_attribute
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [151-167] "only as absolute identity... not the absolutely absolute but the absolute in a determination, or it is attribute."

- c3. id: abs-part-a-003-c3
  - subject: reflection_externality
  - predicate: is
  - object: immediately_internal_to_absolute
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [169-174] "reflection is not only external... it is immediately internal to it."

- c4. id: abs-part-a-003-c4
  - subject: absolute
  - predicate: is_determined_as
  - object: identity_of_being_and_essence_or_inner_and_outer_as_absolute_form
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [175-181] "not abstract identity but... identity of being and essence... inner and outer... absolute form..."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: abs-part-a-002
  - note: removes the residual external starting point by grounding exposition in self-movement.
  - sourceClaimIds: [abs-part-a-003-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [abs-part-a-002-c4]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: abs-part-a-001
  - note: clarifies that absolute identity must include immanent reflection, not abstract sameness.
  - sourceClaimIds: [abs-part-a-003-c2, abs-part-a-003-c4]
  - sourceKeyPointIds: [k2, k4]
  - targetClaimIds: [abs-part-a-001-c2, abs-part-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: abs-part-b-001
  - note: hands off to Part B where attribute becomes explicit topic.
  - sourceClaimIds: [abs-part-a-003-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [abs-part-b-001-c1, abs-part-b-001-c2]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part B handoff claim targets are now resolved.
