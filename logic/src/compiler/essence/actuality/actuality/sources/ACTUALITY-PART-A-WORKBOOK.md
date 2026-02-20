# Actuality Part A (TopicMap) Workbook (V1)

Part: `A. CONTINGENCY OR FORMAL ACTUALITY, POSSIBILITY, AND NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `contingency.txt` as authority for Part A.
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

- file: `contingency.txt`
- active range: lines `1-end` (`A. CONTINGENCY OR FORMAL ACTUALITY, POSSIBILITY, AND NECESSITY`)

Decision:

- Complete Part A in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `act-part-a-001` for lines `3-14`
- completed: `act-part-a-002` for lines `15-125`
- completed: `act-part-a-003` for lines `126-242`

### Entry act-part-a-001 — Formal actuality as immediate being containing possibility

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/contingency.txt`
- lineStart: 3
- lineEnd: 14

Summary:

The chapter opens with first actuality as immediate unreflected being, yet essentially as form-unity it already contains possibility.

Key points: (KeyPoint)

- k1. First actuality is formal and immediate, not yet totality of form.
- k2. As such it appears as mere being or concrete existence.
- k3. By essence actuality is unity of inwardness and externality.
- k4. Therefore possibility is immediately contained in actuality.

Claims: (Claim)

- c1. id: act-part-a-001-c1
  - subject: first_actuality
  - predicate: is
  - object: immediate_unreflected_formal_actuality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [3-6] "Actuality is formal... immediate, unreflected actuality... not as the totality of form."

- c2. id: act-part-a-001-c2
  - subject: formal_actuality
  - predicate: is
  - object: mere_being_or_concrete_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [7-8] "it is nothing more than a being, or concrete existence in general."

- c3. id: act-part-a-001-c3
  - subject: actuality
  - predicate: contains
  - object: immediate_possibility
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [9-13] "it is the form-unity of... inwardness and externality... immediately contains... possibility. What is actual is possible."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: act-part-a-002
  - note: from immediate formal actuality to the developed structure and contradiction of formal possibility.
  - sourceClaimIds: [act-part-a-001-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [act-part-a-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry act-part-a-002 — Formal possibility as contradiction and self-sublating reflection

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/contingency.txt`
- lineStart: 15
- lineEnd: 125

Summary:

Possibility as reflected in-itself is initially formal, but because it includes the opposite as possible it becomes contradiction and self-sublates into actuality.

Key points: (KeyPoint)

- k1. Possibility is actuality reflected into itself but at first only formal self-identity.
- k2. Possibility is both positive in-itself and negative deficiency pointing to actuality.
- k3. Purely formal possibility and identity assertions are empty and superficial.
- k4. Possibility includes opposition and thus impossibility/contradiction.
- k5. Reflective contradiction of possibility sublates itself and becomes actuality.

Claims: (Claim)

- c1. id: act-part-a-002-c1
  - subject: possibility
  - predicate: is
  - object: actuality_reflected_into_itself_as_formal_self_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [15-18] "This possibility is actuality reflected into itself... only the determination of self-identity..."

- c2. id: act-part-a-002-c2
  - subject: possibility
  - predicate: has_dual_moment
  - object: positive_reflection_and_negative_deficiency_toward_actuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [24-32] "Possibility entails... positive moment... [and]... deficient... points to an other, to actuality..."

- c3. id: act-part-a-002-c3
  - subject: formal_possibility_assertion
  - predicate: is
  - object: empty_identity_like_A_equals_A
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [55-61] "it is possible... superficial and empty... 'A is possible' says no more than 'A is A.'"

- c4. id: act-part-a-002-c4
  - subject: possibility
  - predicate: is_internally
  - object: contradiction_or_impossibility
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [91-92] "Internally, therefore, possibility is contradiction, or it is impossibility."
    - [109-116] "the opposite also is possible... entailed in possible A... possible not-A..."

- c5. id: act-part-a-002-c5
  - subject: self_sublating_possibility_connection
  - predicate: becomes
  - object: actuality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [118-125] "connection... contradiction that sublates itself... consequently becomes actuality."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: act-part-a-001
  - note: unfolds the immediate possibility contained in formal actuality into explicit contradiction structure.
  - sourceClaimIds: [act-part-a-002-c1, act-part-a-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [act-part-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: act-part-a-003
  - note: self-sublating possibility yields reflected actuality and contingency/necessity articulation.
  - sourceClaimIds: [act-part-a-002-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-a-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: contradiction/impossibility terminology preserved for downstream necessity mapping.

### Entry act-part-a-003 — Contingency as restless unity of possibility/actuality and emergence of necessity

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/contingency.txt`
- lineStart: 126
- lineEnd: 242

Summary:

Reflected actuality is unity with possibility and therefore contingency; this groundless/grounded oscillation is absolute restlessness that resolves as necessity.

Key points: (KeyPoint)

- k1. Reflected actuality is unity of actuality and possibility where each is only a moment.
- k2. This unity is contingency: the actual is only possible and the possible is itself contingent actuality.
- k3. Contingent is both groundless immediacy and grounded positedness.
- k4. Immediate conversion of inner/outer drives absolute restlessness.
- k5. The identity of each determination in its opposite is necessity.

Claims: (Claim)

- c1. id: act-part-a-003-c1
  - subject: reflected_actuality
  - predicate: is
  - object: unity_of_actuality_and_possibility_with_each_as_sublated_moment
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [126-138] "This actuality is... reflected actuality, posited as unity of itself and possibility... only a moment..."

- c2. id: act-part-a-003-c2
  - subject: unity_of_possibility_and_actuality
  - predicate: is
  - object: contingency
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [151-157] "This unity of possibility and actuality is contingency..."
    - [158-163] "possibility... what is possible... itself something contingent."

- c3. id: act-part-a-003-c3
  - subject: contingent
  - predicate: is
  - object: both_groundless_and_grounded
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [164-172] "it is... immediate actuality... has no ground... likewise groundless."
    - [174-183] "the contingent... have their immanent reflection in an other... The contingent thus has no ground... and... has a ground..."

- c4. id: act-part-a-003-c4
  - subject: contingency
  - predicate: is
  - object: absolute_restlessness_of_becoming_of_possibility_and_actuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [209-210] "This absolute restlessness of the becoming of these two determinations is contingency."

- c5. id: act-part-a-003-c5
  - subject: necessity
  - predicate: emerges_as
  - object: identity_of_each_determination_in_its_opposite
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [211-216] "because each determination immediately turns into the opposite... identity of the two... is necessity."
    - [218-242] "The necessary is an actual... As this identity, it is necessity."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: act-part-a-002
  - note: resolves possibility's contradiction by demonstrating reflected identity as necessity.
  - sourceClaimIds: [act-part-a-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-a-002-c4, act-part-a-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: act-part-a-001
  - note: determines initial formal actuality as contingent actuality within possibility/actuality conversion.
  - sourceClaimIds: [act-part-a-003-c1, act-part-a-003-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [act-part-a-001-c1, act-part-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: act-part-b-001
  - note: transition to relative necessity where real actuality, possibility, and necessity are determined.
  - sourceClaimIds: [act-part-a-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-b-001-c1, act-part-b-001-c2]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part B handoff claim targets are now resolved.
