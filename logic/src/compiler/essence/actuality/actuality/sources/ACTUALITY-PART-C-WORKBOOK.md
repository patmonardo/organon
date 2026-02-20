# Actuality Part C (TopicMap) Workbook (V1)

Part: `C. ABSOLUTE NECESSITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `absolute-necessity.txt` as authority for Part C.
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

## Session: 2026-02-20 (initial full Part C pass)

Scope:

- file: `absolute-necessity.txt`
- active range: lines `1-end` (`C. ABSOLUTE NECESSITY`)

Decision:

- Complete Part C in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `act-part-c-001` for lines `3-104`
- completed: `act-part-c-002` for lines `105-164`
- completed: `act-part-c-003` for lines `165-232`

### Entry act-part-c-001 — Real necessity's self-determination as contingency and absolute necessity

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/absolute-necessity.txt`
- lineStart: 3
- lineEnd: 104

Summary:

Real necessity, containing contingency, performs its own presupposition/positing movement and thereby determines itself as contingency while form/content difference is penetrated toward absolute necessity.

Key points: (KeyPoint)

- k1. Real necessity is determinate because contingency is its own negation within.
- k2. Immediate actual necessity as absolute actuality flips into empty contingency/possibility.
- k3. Necessity's becoming is its own positing via conversion of actuality and possibility.
- k4. Necessity determines itself as contingency through positing/sublating positedness.
- k5. Form pervades distinctions; resolution of content/form difference is absolute necessity.

Claims: (Claim)

- c1. id: act-part-c-001-c1
  - subject: real_necessity
  - predicate: is
  - object: determinate_necessity_with_contingency_within_it
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [3-8] "Real necessity is determinate necessity... having its negation, contingency, within it."

- c2. id: act-part-c-001-c2
  - subject: absolute_actuality
  - predicate: becomes
  - object: empty_contingency_and_absolute_possibility
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [18-31] "because this actuality is posited to be absolute... it is contingency... possibility is itself absolute possibility..."

- c3. id: act-part-c-001-c3
  - subject: necessity
  - predicate: is
  - object: own_becoming_and_own_posited_presupposition
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [33-46] "contingency... becomes in it... necessity's own becoming... own positing... positive unity..."

- c4. id: act-part-c-001-c4
  - subject: necessity
  - predicate: determines_itself_as
  - object: contingency
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [67-77] "It is necessity itself... that determines itself as contingency..."

- c5. id: act-part-c-001-c5
  - subject: absolute_necessity
  - predicate: resolves
  - object: form_content_difference_by_self_penetrating_difference
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [100-104] "The resolution of this difference is... absolute necessity whose content is this difference which... penetrates itself."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: act-part-c-002
  - note: from genesis of absolute necessity to its explicit truth-definition as unity of being and essence.
  - sourceClaimIds: [act-part-c-001-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-c-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: act-part-b-003
  - note: confirms Part B claim that real necessity contains contingency and tends toward absolute actuality.
  - sourceClaimIds: [act-part-c-001-c1, act-part-c-001-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [act-part-b-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry act-part-c-002 — Absolute necessity as identity of being and essence

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/absolute-necessity.txt`
- lineStart: 105
- lineEnd: 164

Summary:

Absolute necessity is the truth-return of actuality/possibility and formal/real necessity as self-grounding identity of pure being and pure essence, yet with conversion into free actualities and blind necessity.

Key points: (KeyPoint)

- k1. Absolute necessity is the truth-return of actuality and possibility, formal and real necessity.
- k2. It is unity of pure being and pure essence, without external condition/ground.
- k3. As reflection, it has only itself as ground and condition.
- k4. It is reflection/form of the absolute: unity of being and essence as absolute negativity.
- k5. This includes absolute conversion of actuality and possibility and yields blind necessity.

Claims: (Claim)

- c1. id: act-part-c-002-c1
  - subject: absolute_necessity
  - predicate: is
  - object: truth_of_actuality_possibility_and_formal_real_necessity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [105-107] "Absolute necessity is therefore the truth in which actuality and possibility... formal and real necessity return."

- c2. id: act-part-c-002-c2
  - subject: absolutely_necessary
  - predicate: is
  - object: self_grounding_identity_of_being_and_essence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [108-113] "being... and essence... one and the same... The absolutely necessary only is because it is..."
    - [118-121] "has a ground and a condition but has only itself for this ground and condition..."

- c3. id: act-part-c-002-c3
  - subject: absolute_necessity
  - predicate: is
  - object: reflection_or_form_of_absolute_as_absolute_negativity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [129-133] "Absolute necessity is... reflection or form of the absolute... simple immediacy which is absolute negativity."

- c4. id: act-part-c-002-c4
  - subject: absolute_necessity
  - predicate: performs
  - object: absolute_conversion_of_actuality_and_possibility
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [138-143] "absolute conversion of its actuality into its possibility and its possibility into its actuality."

- c5. id: act-part-c-002-c5
  - subject: absolute_necessity
  - predicate: is
  - object: blind
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [143-157] "Absolute necessity is therefore blind... reciprocal contact... appears as empty externality... contingency."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: act-part-c-001
  - note: makes explicit the truth-structure toward which C.1's self-determining necessity was moving.
  - sourceClaimIds: [act-part-c-002-c1, act-part-c-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [act-part-c-001-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: act-part-c-003
  - note: blind necessity is unfolded into collapse/rejoining movement that culminates as substance.
  - sourceClaimIds: [act-part-c-002-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-part-c-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keep c4/c5 distinction for second-order analysis of blindness vs identity.

### Entry act-part-c-003 — Blind collapse, self-rejoining contingency, and emergence of substance

Span:

- sourceFile: `src/compiler/essence/actuality/actuality/sources/absolute-necessity.txt`
- lineStart: 165
- lineEnd: 232

Summary:

What appears as blind contingent externality is absolute necessity's own negative movement: collapse into otherness and rejoining identity, whose result is substance.

Key points: (KeyPoint)

- k1. Contingency is the essence of free inherently necessary actualities.
- k2. Their simple self-grounded being is absolute negativity and self-contradiction.
- k3. The negative breaks forth as nothing/otherness and as perishability of determinate actualities.
- k4. Transition of actual to possible (being to nothing) is self-rejoining absolute identity.
- k5. Identity of being-with-negation is substance; blind transition is absolute self-exposition.

Claims: (Claim)

- c1. id: act-part-c-003-c1
  - subject: contingency
  - predicate: is
  - object: absolute_necessity_as_essence_of_free_actualities
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [165-166] "this contingency is rather absolute necessity; it is the essence of those free... actualities."

- c2. id: act-part-c-003-c2
  - subject: self_grounded_being_of_actualities
  - predicate: is
  - object: absolute_negativity_and_self_contradiction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [176-183] "simplicity of their being... is absolute negativity... being... is self-contradiction..."

- c3. id: act-part-c-003-c3
  - subject: negative_breakthrough
  - predicate: appears_as
  - object: otherness_nothing_and_perishing_of_actualities
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [184-189] "break forth... as the negation... as their nothing, as an otherness..."
    - [201-205] "necessity appeals... the actualities now perish..."

- c4. id: act-part-c-003-c4
  - subject: transition_of_actual_into_possible
  - predicate: is
  - object: self_rejoining_absolute_identity_where_contingency_is_absolute_necessity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [217-221] "transition... is a self-rejoining; contingency is absolute necessity..."

- c5. id: act-part-c-003-c5
  - subject: identity_of_being_with_itself_in_negation
  - predicate: is
  - object: substance_and_absolutes_own_exposition
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [223-231] "This identity... is now substance... The blind transition... absolute's own exposition..."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: act-part-c-002
  - note: resolves blind externality by showing it as absolute self-exposition and rejoining identity.
  - sourceClaimIds: [act-part-c-003-c4, act-part-c-003-c5]
  - sourceKeyPointIds: [k4, k5]
  - targetClaimIds: [act-part-c-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: act-part-c-001
  - note: concretizes C.1's thesis that necessity determines itself as contingency.
  - sourceClaimIds: [act-part-c-003-c1, act-part-c-003-c4]
  - sourceKeyPointIds: [k1, k4]
  - targetClaimIds: [act-part-c-001-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: act-idea-001
  - note: culmination in substance transitions to the chapter-level idea anchor.
  - sourceClaimIds: [act-part-c-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [act-idea-001-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: forward transition target is now anchored to `ACTUALITY-IDEA-WORKBOOK.md`.
