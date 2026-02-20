# Absolute Part C (TopicMap) Workbook (V1)

Part: `C. THE MODE OF THE ABSOLUTE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `mode.txt` as authority for Part C.
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

- file: `mode.txt`
- active range: lines `1-end` (`C. THE MODE OF THE ABSOLUTE`)

Decision:

- Complete Part C in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `abs-part-c-001` for lines `3-35`
- completed: `abs-part-c-002` for lines `37-74`
- completed: `abs-part-c-003` for lines `76-114`

### Entry abs-part-c-001 — Mode as externality and self-identity of the absolute

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/mode.txt`
- lineStart: 3
- lineEnd: 35

Summary:

Mode first appears as the attribute’s negative externality, yet this same reflective shine posits absolute identity only through mode.

Key points: (KeyPoint)

- k1. Attribute has two extremes: simple self-identity and negation as formal immanent reflection.
- k2. The negative extreme as mode is self-externality, contingency, and non-total manifoldness.
- k3. Mode is also externality posited as externality: reflective shine that turns form into self-identity.
- k4. Absolute identity is first posited only in mode as self-referring negativity.

Claims: (Claim)

- c1. id: abs-part-c-001-c1
  - subject: attribute
  - predicate: has_structure
  - object: extremes_of_self_identity_and_negation_with_attribute_as_middle
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [3-11] "The attribute is first... self-identity... Second, it is negation... two extremes... middle term..."

- c2. id: abs-part-c-001-c2
  - subject: negative_as_mode
  - predicate: is
  - object: self_externality_and_contingent_non_total_manifoldness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [12-23] "negative as negative... reflection external... self-externality... changeability and contingency... lacks totality."

- c3. id: abs-part-c-001-c3
  - subject: mode
  - predicate: is
  - object: externality_posited_as_externality_and_reflective_shine
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [25-30] "mode... not just this... externality posited as externality... reflective shine... self-identity..."

- c4. id: abs-part-c-001-c4
  - subject: absolute_identity
  - predicate: is_first_posited_in
  - object: mode_as_self_referring_negativity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [31-35] "the absolute is first posited as absolute identity only in the mode... only as self-referring negativity..."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: abs-part-c-002
  - note: from initial determination of mode to explicit triadic account of exposition's movement.
  - sourceClaimIds: [abs-part-c-001-c4]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [abs-part-c-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: abs-part-b-003
  - note: clarifies Part B's null form as positively re-read through mode's immanent reflective function.
  - sourceClaimIds: [abs-part-c-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [abs-part-b-003-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry abs-part-c-002 — Exposition’s three determinations and immanent reflective movement

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/mode.txt`
- lineStart: 37
- lineEnd: 74

Summary:

The exposition through identity, attribute, and mode is complete only as reflective movement; mode is immanent return, and the beginning determinateness is itself found in the absolute.

Key points: (KeyPoint)

- k1. The exposition through identity -> attribute -> mode exhaustively runs the moments.
- k2. The act is not merely negative treatment but reflective movement through which the absolute is identity.
- k3. Mode is not mere outermost externality but self-dissolving immanent reflection.
- k4. What appears as externally taken determinateness is found within the absolute itself.
- k5. Absolute form and self-positing arise only through this reflective movement.

Claims: (Claim)

- c1. id: abs-part-c-002-c1
  - subject: exposition_path
  - predicate: exhausts
  - object: moments_of_absolute_identity_attribute_mode
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [37-41] "begins from... identity... passes over to attribute... from there to mode... exhaustively run through its moments."

- c2. id: abs-part-c-002-c2
  - subject: expository_act
  - predicate: is
  - object: reflective_movement_required_for_true_absolute_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [43-47] "does not just behave negatively... its act is rather the reflective movement itself..."

- c3. id: abs-part-c-002-c3
  - subject: mode
  - predicate: is
  - object: immanent_turning_back_and_self_dissolving_reflection
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [49-54] "mode is not only... externality... reflective shine as shine... immanent turning back, self-dissolving reflection..."

- c4. id: abs-part-c-002-c4
  - subject: initial_determinateness_of_exposition
  - predicate: is_found_in
  - object: absolute_itself_as_determinate_unreflected_attribute
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [56-67] "seems to begin... from something external... But it has in fact found the determinateness... in the absolute itself..."

- c5. id: abs-part-c-002-c5
  - subject: reflective_movement
  - predicate: alone_confers
  - object: absolute_form_and_self_positing_of_absolute
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [68-74] "belongs to the reflective movement... through this movement alone... has absolute form... posits itself as self-equal."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: abs-part-c-001
  - note: explicates mode's role by embedding it in the complete triadic movement of exposition.
  - sourceClaimIds: [abs-part-c-002-c1, abs-part-c-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [abs-part-c-001-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: abs-part-a-003
  - note: confirms Part A's claim that exposition is absolute self-doing, not external imposition.
  - sourceClaimIds: [abs-part-c-002-c4, abs-part-c-002-c5]
  - sourceKeyPointIds: [k4, k5]
  - targetClaimIds: [abs-part-a-003-c1]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: abs-part-c-003
  - note: movement-account transitions to explicit determination of content, expression, and actuality.
  - sourceClaimIds: [abs-part-c-002-c3, abs-part-c-002-c5]
  - sourceKeyPointIds: [k3, k5]
  - targetClaimIds: [abs-part-c-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keep c4/c5 distinction for second-order inference cycle.

### Entry abs-part-c-003 — Mode as self-manifestation: content as exposition and actuality

Span:

- sourceFile: `src/compiler/essence/actuality/absolute/sources/mode.txt`
- lineStart: 76
- lineEnd: 114

Summary:

Mode is the absolute's own reflective movement whose transparent externality is inwardness; the content is this exposition itself, culminating in absolute manifestation as actuality.

Key points: (KeyPoint)

- k1. True meaning of mode is absolute self-determining movement that becomes what it already is.
- k2. Externality is transparent self-reference where outwardness is equally inwardness.
- k3. In the absolute, form/content distinction is dissolved; manifestation itself is content.
- k4. Absolute form is negative that rejoins itself, yielding self-identity and absolute content.
- k5. As self-bearing expression of itself for itself, the absolute is actuality.

Claims: (Claim)

- c1. id: abs-part-c-003-c1
  - subject: mode
  - predicate: is
  - object: absolutes_own_reflective_movement
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [76-80] "true meaning of mode... absolute's own reflective movement... absolute would become... what it already is."

- c2. id: abs-part-c-003-c2
  - subject: mode_externality
  - predicate: is
  - object: transparent_self_pointing_where_outwardness_equals_inwardness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [81-88] "transparent externality... pointing to itself... being outwardly is just as much inwardness... absolute being."

- c3. id: abs-part-c-003-c3
  - subject: content_of_absolute_exposition
  - predicate: is
  - object: dissolution_of_form_content_distinction_as_self_manifestation
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [90-95] "distinction of form and content... dissolved... this is the content... that it manifests itself."

- c4. id: abs-part-c-003-c4
  - subject: absolute_form
  - predicate: is
  - object: negative_that_rejoins_itself_as_absolute_self_identity_and_content
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [96-104] "absolute form... negative as negative... rejoins itself... absolute self-identity... absolute content..."

- c5. id: abs-part-c-003-c5
  - subject: absolute
  - predicate: is
  - object: actuality_as_absolute_manifestation_of_itself_for_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [106-114] "as this self-bearing movement... absolute is expression... absolute manifestation of itself for itself. Thus it is actuality."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: abs-part-c-001
  - note: resolves initial externality by showing mode as transparent self-manifesting inwardness.
  - sourceClaimIds: [abs-part-c-003-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [abs-part-c-001-c2, abs-part-c-001-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: refines
  - targetEntryId: abs-part-c-002
  - note: determines reflective movement's outcome as form/content identity and actuality.
  - sourceClaimIds: [abs-part-c-003-c3, abs-part-c-003-c5]
  - sourceKeyPointIds: [k3, k5]
  - targetClaimIds: [abs-part-c-002-c2, abs-part-c-002-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r3. type: transitions_to
  - targetEntryId: abs-idea-001
  - note: culmination in actuality now targets the Absolute idea-level handoff anchor.
  - sourceClaimIds: [abs-part-c-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [abs-idea-001-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: forward transition is now anchored to `ABSOLUTE-IDEA-WORKBOOK.md`.
