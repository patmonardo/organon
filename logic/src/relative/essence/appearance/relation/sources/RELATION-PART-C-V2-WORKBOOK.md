# Relation Part C (TopicMap) Workbook (V2)

Part: `C. RELATION OF OUTER AND INNER`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `outer-inner.txt` as authority for Part C.
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

## Session: 2026-02-19 (initial Part C scaffold)

Scope:

- file: `outer-inner.txt`
- active range: lines `2-231` (`C. RELATION OF OUTER AND INNER` block)

Decision:

- Part C decomposition follows explicit numeric markers `1/2/3`.
- Translation rule: the final paragraph is modeled as a dedicated transition entry.
- Part-entry extraction is therefore `c-001..c-003` for numbered movements plus `c-004` for the final transition paragraph.

## Decomposition status

- completed: `rel-oin-c-001` for movement `1.` (line `4` to `67`)
- completed: `rel-oin-c-002` for movement `2.` (line `68` to `156`)
- completed: `rel-oin-c-003` for movement `3.` main block (line `157` to `228`)
- completed: `rel-oin-c-004` for final transition paragraph (line `229` to `231`) ["...has determined itself as actuality."]

### Entry rel-oin-c-001 — Immediate unity of inner and outer through mediated reflective shine

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/outer-inner.txt`
- lineStart: 4
- lineEnd: 67

Summary:

In movement `1`, the expression of force determines immediate and reflected self-subsistence as mediated negative unity, thereby introducing inner and outer as one contentful identity whose distinction persists only as indifferent external form-determinations.

Key points: (KeyPoint)

- k1. The immediate relation (whole/parts) is sublated in force-expression into mediated negative unity where transition is self-return.
- k2. The reflective shine difference is not mere nullity but mediation that is independent subsistence.
- k3. Inner and outer are one identity as contentful substrate (the fact), not two separate contents.
- k4. Their difference remains as external form-determinations indifferent to their common identity.

Claims: (Claim)

- c1. id: rel-oin-c-001-c1
  - subject: relation_of_inner_and_outer_in_its_first_determination
  - predicate: emerges_from
  - object: mediated_negative_unity_of_force_expression
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-16] self-subsisting immediacies are now posited as negative unity in expression of force.
    - [17-30] transition is self-positing turning-back; each determination is already unity with its other.

- c2. id: rel-oin-c-001-c2
  - subject: inner_and_outer
  - predicate: are
  - object: one_identity_as_contentful_substrate_or_fact
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [31-44] inner and outer are one identity, sustaining unity as substrate replete of content.
    - [45-51] outer is equal to inner in content; both are one fact.

- c3. id: rel-oin-c-001-c3
  - subject: difference_of_inner_and_outer
  - predicate: persists_as
  - object: external_indifferent_form_determinations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [52-63] fact differs from its form determinations; determinations are external to it.
    - [64-67] as forms, inner and outer are indifferent to identity and to each other.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: rel-frc-b-003
  - sourceClaimIds: [rel-oin-c-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [rel-frc-b-003-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection
  - note: movement `1` unfolds the Part B infinity claim into the explicit inner/outer identity-form relation.

- r2. type: transitions_to
  - targetEntryId: rel-oin-c-002
  - sourceClaimIds: [rel-oin-c-001-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: from indifferent external form distinction to explicit formal conversion dynamics in movement `2`.

Review outcome:

- review_pending
- notes: `c-001` complete at explicit `1.` boundary.

### Entry rel-oin-c-002 — Formal opposition and immediate conversion of inner and outer

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/outer-inner.txt`
- lineStart: 68
- lineEnd: 156

Summary:

In movement `2`, inner and outer are treated as opposed form-determinations on an identical substrate, such that each is immediately its opposite and their unity appears only as formal conversion and a negative point still empty of content.

Key points: (KeyPoint)

- k1. Inner and outer are distinct reflection-forms (essentiality/unessentiality) sharing one substrate beyond them.
- k2. As pure form-opposites, each is immediately the other (inner only outer, outer only inner).
- k3. This identity remains formal and abstract, not yet the contentful totality.
- k4. Each determination points beyond itself to totality, but mediation still lacks the identical substrate in the relation itself, yielding immediate conversion and empty negative unity.

Claims: (Claim)

- c1. id: rel-oin-c-002-c1
  - subject: inner_and_outer_as_form_determinations
  - predicate: are
  - object: opposed_reflection_forms_on_identical_substrate
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [68-76] inner and outer are different forms on an identical substrate not in them but in another.
    - [77-80] relation's nature shows these determinations constitute one identity.

- c2. id: rel-oin-c-002-c2
  - subject: identity_of_inner_and_outer_in_form
  - predicate: has_mode
  - object: immediate_conversion_of_each_into_its_opposite
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [81-96] in pure form, unity is abstract determination in which one is immediately the other.
    - [97-103] inner is only outer and outer only inner.

- c3. id: rel-oin-c-002-c3
  - subject: mediation_of_inner_outer_in_movement_two
  - predicate: is
  - object: formal_conversion_with_negative_unity_still_empty_of_content
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [104-132] determination remains on side of form; substrate is said in each opposite determination only formally.
    - [133-156] each passes into other as truth and points to totality, but mediation misses identical substrate; relation is immediate conversion with simple empty negative unity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-oin-c-001
  - sourceClaimIds: [rel-oin-c-002-c2, rel-oin-c-002-c3]
  - sourceKeyPointIds: [k2, k4]
  - targetClaimIds: [rel-oin-c-001-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-002` specifies the mode of distinction from `c-001` as formal inversion rather than contentful difference.

- r2. type: transitions_to
  - targetEntryId: rel-oin-c-003
  - sourceClaimIds: [rel-oin-c-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: moves to movement `3` where content-substrate and pure-form identities are unified as one totality.

Review outcome:

- review_pending
- notes: `c-002` complete at explicit `2.` boundary.

### Entry rel-oin-c-003 — Totality of inner and outer as self-mediating expression

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/outer-inner.txt`
- lineStart: 157
- lineEnd: 228

Summary:

In movement `3`, the content-identity and form-identity of inner/outer are unified as one totality in which each side is the total relation through the other, culminating in the thesis that something is wholly in its externality as self-revealing essence.

Key points: (KeyPoint)

- k1. The two prior identities (substrate-content identity and immediate form-conversion identity) are only sides of one totality.
- k2. In this totality, content and form mutually determine each other through reflective mediation.
- k3. Inner and outer are each totalities containing themselves and their other, with each side mediated through the other.
- k4. Therefore externality is not alien to essence but the very expression of it: essence is self-revealing.

Claims: (Claim)

- c1. id: rel-oin-c-003-c1
  - subject: inner_outer_relation_in_movement_three
  - predicate: is
  - object: unity_of_content_identity_and_form_identity_as_one_totality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [157-168] first identity (content/substrate) and second identity (pure form conversion) are two sides of one totality.
    - [169-179] totality is reflected immediacy through presupposing reflection that sublates difference.

- c2. id: rel-oin-c-003-c2
  - subject: inner_and_outer
  - predicate: are
  - object: each_totality_of_relation_mediated_through_its_other
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [180-201] each form-difference is posited as totality of itself and its other; inner includes being/externality and outer returns to ground as inner.
    - [202-211] each side is what it is in itself precisely through the other; totality mediates itself through determinateness and simple identity.

- c3. id: rel-oin-c-003-c3
  - subject: something_in_its_truth
  - predicate: is
  - object: entirely_in_externality_as_self_revealing_essence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [213-221] what something is is entirely in its externality; appearance is immanent reflection and expression of in-itself.
    - [222-228] content and form are absolutely identical; essence consists in self-revealing.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-oin-c-002
  - sourceClaimIds: [rel-oin-c-003-c1, rel-oin-c-003-c2]
  - sourceKeyPointIds: [k1, k3]
  - targetClaimIds: [rel-oin-c-002-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-003` overcomes the empty formal conversion of `c-002` by restoring mediated totality and contentful identity.

- r2. type: transitions_to
  - targetEntryId: rel-oin-c-004
  - sourceClaimIds: [rel-oin-c-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: the self-revealing identity of essence and appearance prepares the final transition to actuality.

Review outcome:

- review_pending
- notes: `c-003` complete at explicit `3.` boundary.

### Entry rel-oin-c-004 — Transition: essential relation determines itself as actuality

Span:

- sourceFile: `src/relative/essence/appearance/relation/sources/outer-inner.txt`
- lineStart: 229
- lineEnd: 231

Summary:

The concluding transition paragraph states that the essential relation, as identity of appearance with inner essence, determines itself as actuality.

Key points: (KeyPoint)

- k1. Essential relation is now explicitly identified with the identity of appearance and essence.
- k2. This identity is determinative, not merely descriptive: it determines itself.
- k3. The resultant determination is actuality.

Claims: (Claim)

- c1. id: rel-oin-c-004-c1
  - subject: essential_relation
  - predicate: is
  - object: identity_of_appearance_with_inner_or_essence
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [229-230] essential relation is stated in identity of appearance with inner/essence.

- c2. id: rel-oin-c-004-c2
  - subject: essential_relation_in_this_identity
  - predicate: determines_itself_as
  - object: actuality
  - modality: asserted
  - confidence: 0.99
  - evidence:
    - [231] explicit determination: it has determined itself as actuality.

- c3. id: rel-oin-c-004-c3
  - subject: transition_status_of_part_c
  - predicate: is
  - object: completed_handoff_to_actuality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [229-231] final sentence functions as chapter-terminal transition from essential relation to actuality.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: rel-oin-c-003
  - sourceClaimIds: [rel-oin-c-004-c1, rel-oin-c-004-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [rel-oin-c-003-c3]
  - logicalOperator: consummative_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-004` consummates movement `3` by explicitly naming its result as actuality.

- r2. type: transitions_to
  - targetEntryId: pending_actuality_chapter
  - sourceClaimIds: [rel-oin-c-004-c2]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: closes Essential Relation and hands off to the actuality sequence.

Review outcome:

- review_pending
- notes: `c-004` complete; Relation Part C is fully complete (`c-001..c-004`).
