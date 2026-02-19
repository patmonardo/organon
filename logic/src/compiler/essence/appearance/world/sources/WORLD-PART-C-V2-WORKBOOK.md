# World Part C (TopicMap) Workbook (V2)

Part: `C. THE DISSOLUTION OF APPEARANCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `disappearance.txt` as authority for Part C.
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

- file: `disappearance.txt`
- active range: lines `2-123` (`C. THE DISSOLUTION OF APPEARANCE` block)

Decision:

- Initialize Part C workbook before entry extraction.
- User-guided decomposition rule: first three paragraphs are core analysis entries (`c-001..c-003`).
- User-guided transition rule: the last two paragraphs are modeled as dedicated transition entries.

## Decomposition status

- completed: `wld-dis-c-001` for paragraph 1 (line `4` to `21`)
- completed: `wld-dis-c-002` for paragraph 2 (line `22` to `55`)
- completed: `wld-dis-c-003` for paragraph 3 (line `56` to `76`)
- completed: `wld-dis-c-004` for transition paragraph 1 (line `77` to `100`)
- completed: `wld-dis-c-005` for transition paragraph 2 (line `101` to `123`)

### Entry wld-dis-c-001 — Oppositional ground-relation of the two worlds

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/disappearance.txt`
- lineStart: 4
- lineEnd: 21

Summary:

Paragraph 1 determines the world in-and-for-itself as determinate ground of appearance only as negative moment and totality of corresponding determinations, such that the two worlds stand in strict oppositional inversion of positive and negative.

Key points: (KeyPoint)

- k1. The in-and-for-itself world is determinate ground of the world of appearance only insofar as it is negative moment and totality of corresponding determinations/alterations.
- k2. This same ground is at once the completely opposed side of the world of appearance.
- k3. The relation between worlds is systematic polarity inversion (positive↔negative), illustrated by north/south, electricity, and evil/good.

Claims: (Claim)

- c1. id: wld-dis-c-001-c1
  - subject: world_in_and_for_itself
  - predicate: is
  - object: determinate_ground_as_negative_moment_and_totality_of_corresponding_determinations
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-10] world in-and-for-itself is determinate ground only as negative moment and totality of corresponding determinations/alterations.

- c2. id: wld-dis-c-001-c2
  - subject: relation_of_world_in_and_for_itself_to_world_of_appearance
  - predicate: is
  - object: simultaneous_ground_and_completely_opposed_side
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [10-12] it is at once determinate ground and completely opposed side.

- c3. id: wld-dis-c-001-c3
  - subject: two_world_relation
  - predicate: has_form
  - object: polarity_inversion_of_positive_and_negative
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [11-15] positive in one world is negative in the other, and conversely.
    - [16-21] examples: north/south pole inversion, positive/negative electricity, evil/good luck inversion.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: wld-wld-b-003
  - sourceClaimIds: [wld-dis-c-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [wld-wld-b-003-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: Part C.1 makes explicit the inversion structure that Part B.3 formulates.

- r2. type: transitions_to
  - targetEntryId: wld-dis-c-002
  - sourceClaimIds: [wld-dis-c-001-c2, wld-dis-c-001-c3]
  - sourceKeyPointIds: [k2, k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: proceeds to paragraph 2 where oppositional difference is shown to dissolve into reciprocal self-reference.

Review outcome:

- review_pending
- notes: `c-001` complete under user-defined first-three-plus-two-transition decomposition.

### Entry wld-dis-c-002 — Opposition dissolves into reciprocal self-reference and self-inverting content

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/disappearance.txt`
- lineStart: 22
- lineEnd: 55

Summary:

Paragraph 2 shows that in the very opposition of the two worlds their difference disappears: each world passes into the other, appearance returns to lawful self-reference, and the in-and-for-itself world becomes self-opposed, self-inverting content that is both sublated ground and immediate concrete existence.

Key points: (KeyPoint)

- k1. The oppositional difference of world of appearance and in-itself world collapses, each becoming the other.
- k2. Appearance's reflection into otherness sublates the other-as-other, yielding self-reference and law equal to itself.
- k3. The in-and-for-itself world, as reflection of appearance, contains negativity and reference-to-otherness, becoming self-opposed and self-inverting.
- k4. Because opposition is internal, the in-and-for-itself world is both ground and sublated ground, retaining immediate concrete existence.

Claims: (Claim)

- c1. id: wld-dis-c-002-c1
  - subject: opposition_of_the_two_worlds
  - predicate: results_in
  - object: disappearance_of_their_difference_and_reciprocal_identity
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [22-28] in their opposition, difference disappears; world-in-itself is itself world of appearance and conversely essential within.

- c2. id: wld-dis-c-002-c2
  - subject: world_of_appearance
  - predicate: becomes
  - object: lawful_self_reference_through_sublation_of_otherness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [29-38] reflection into otherness leads to other that sublates itself as other; both refer to themselves.
    - [38-39] world of appearance is within it law equal to itself.

- c3. id: wld-dis-c-002-c3
  - subject: world_in_and_for_itself
  - predicate: is
  - object: self_inverting_content_that_is_sublated_ground_and_immediate_concrete_existence
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [40-49] self-identical content as complete reflection of appearance contains negativity and self-reference as reference to otherness, becoming self-opposed/self-inverting.
    - [50-55] because opposition is in it, it is equally sublated ground and immediate concrete existence.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-dis-c-001
  - sourceClaimIds: [wld-dis-c-002-c1, wld-dis-c-002-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [wld-dis-c-001-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-002` internalizes the inversion stated in `c-001` by showing reciprocal self-reference rather than fixed external polarity.

- r2. type: transitions_to
  - targetEntryId: wld-dis-c-003
  - sourceClaimIds: [wld-dis-c-002-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: moves to paragraph 3 where both worlds are explicitly totalities containing both moments in unity.

Review outcome:

- review_pending
- notes: `c-002` complete; ready for core paragraph `c-003`.

### Entry wld-dis-c-003 — Two totalities unified by reciprocal containment

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/disappearance.txt`
- lineStart: 56
- lineEnd: 76

Summary:

Paragraph 3 determines both world of appearance and essential world as totalities each containing self-identical reflection and reflection-into-other, so their distinct self-subsistence stands only as reciprocal reference and unity.

Key points: (KeyPoint)

- k1. Both worlds are self-subsisting wholes that each contain the two moments of reflection and being-in-and-for-itself.
- k2. The distinction between reflected and immediate concrete existence is preserved only within a higher reciprocal identity.
- k3. Totality splits into reflected and immediate totality, but each is self-subsistent only as totality containing the other.
- k4. Distinct self-subsistence is thus posited as essential reference to the other and unity of both.

Claims: (Claim)

- c1. id: wld-dis-c-003-c1
  - subject: world_of_appearance_and_essential_world
  - predicate: are
  - object: totalities_of_self_identical_reflection_and_reflection_into_other
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [56-60] both worlds are each the totality of self-identical reflection and reflection-into-other.
    - [61-64] both are self-subsisting wholes of concrete existence.

- c2. id: wld-dis-c-003-c2
  - subject: reflected_and_immediate_concrete_existence
  - predicate: are
  - object: mutually_continuing_moments_within_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [62-64] one is reflected and the other immediate concrete existence.
    - [64-65] each continues into the other and is identity of both moments.

- c3. id: wld-dis-c-003-c3
  - subject: distinct_self_subsistence_of_each_totality
  - predicate: is
  - object: posited_reference_to_other_and_unity_of_the_two
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [66-72] totality splits into reflected and immediate totalities, each self-subsistent only as totality containing the other moment.
    - [73-76] distinct self-subsistence is essentially reference to the other and subsistence in their unity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-dis-c-002
  - sourceClaimIds: [wld-dis-c-003-c2, wld-dis-c-003-c3]
  - sourceKeyPointIds: [k2, k4]
  - targetClaimIds: [wld-dis-c-002-c1, wld-dis-c-002-c3]
  - logicalOperator: dialectical_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-003` stabilizes the reciprocal inversion of `c-002` as explicit two-totality structure with unity-in-reference.

- r2. type: transitions_to
  - targetEntryId: wld-dis-c-004
  - sourceClaimIds: [wld-dis-c-003-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: hands off to the first transition paragraph where law is revisited as realized identity.

Review outcome:

- review_pending
- notes: `c-003` complete; core three-paragraph analysis for Part C is complete.

### Entry wld-dis-c-004 — Transition I: from merely internal law-identity to realized law

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/disappearance.txt`
- lineStart: 77
- lineEnd: 100

Summary:

This first transition paragraph returns to the law of appearance and shows the shift from merely internal, unrealized identity of differentiated content to realized law in which each side contains the other and is truly identical with itself and with the other.

Key points: (KeyPoint)

- k1. Law begins as identity of different contents where the positedness of one is the positedness of the other.
- k2. Initially this identity is only internal and unrealized; content remains indifferent and diversified.
- k3. Law is realized when inner identity exists in the content itself through sublation and reflection-into-self.
- k4. Realized law entails that each side contains the other and is truly self-identical in that relation.

Claims: (Claim)

- c1. id: wld-dis-c-004-c1
  - subject: law_of_appearance
  - predicate: is_initially
  - object: identity_of_different_contents_via_reciprocal_positedness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [77-81] law starts as identity of one content with another different content through reciprocal positedness.

- c2. id: wld-dis-c-004-c2
  - subject: initial_identity_in_law
  - predicate: is
  - object: internal_and_not_yet_realized_with_indifferent_content
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [82-89] identity of sides is at first only internal and not yet in them.
    - [86-92] content is not identical but indifferent/diversified; determination not yet present in it.

- c3. id: wld-dis-c-004-c3
  - subject: realized_law
  - predicate: consists_in
  - object: existent_inner_identity_and_reflected_ideality_where_each_side_contains_the_other
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [93-96] law is now realized; inner identity is existent and content raised to ideality.
    - [97-100] each side has the other in it and is truly identical with it and with itself.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-dis-c-003
  - sourceClaimIds: [wld-dis-c-004-c3]
  - sourceKeyPointIds: [k3, k4]
  - targetClaimIds: [wld-dis-c-003-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection
  - note: `c-004` restates the unity-of-two-totalities result as realized law at the level of explicit conceptual determination.

- r2. type: transitions_to
  - targetEntryId: wld-dis-c-005
  - sourceClaimIds: [wld-dis-c-004-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: moves to final transition paragraph where realized law is named essential relation and totality is reconstituted as relation.

Review outcome:

- review_pending
- notes: `c-004` complete as transition paragraph 1.

### Entry wld-dis-c-005 — Transition II: law as essential relation and consummated unity of worlds

Span:

- sourceFile: `src/relative/essence/appearance/world/sources/disappearance.txt`
- lineStart: 101
- lineEnd: 123

Summary:

The concluding transition paragraph states that law is essential relation, recasts both essential and appearance worlds as one relational totality, and culminates in essential relation as the consummated unity of form between the two totalities.

Key points: (KeyPoint)

- k1. Law is explicitly determined as essential relation.
- k2. The truth of the unessential world is a world in-and-for-itself that, as totality, includes both immediate concrete existence and reflected otherness.
- k3. The world as formless manifold totality has foundered as separate essential/appearance worlds and persists as relational totality.
- k4. Two content-totalities initially indifferent become connected through form, and essential relation consummates their unity of form.

Claims: (Claim)

- c1. id: wld-dis-c-005-c1
  - subject: law
  - predicate: is
  - object: essential_relation
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [101] explicit statement: law is essential relation.

- c2. id: wld-dis-c-005-c2
  - subject: truth_of_unessential_world
  - predicate: is
  - object: totality_of_immediate_and_reflected_concrete_existence
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [102-108] truth is world in-and-for-itself and other-to-it, as totality of immediate existence and reflection in otherness, equally reflected into itself.
    - [109-114] world as formless manifold totality has foundered as separate worlds but remains totality as essential relation.

- c3. id: wld-dis-c-005-c3
  - subject: essential_relation
  - predicate: consummates
  - object: unity_of_form_between_two_content_totalities
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [115-121] two content-totalities arise as initially indifferent self-subsistents with form not yet for-the-other.
    - [121-123] form proves to be connecting reference; essential relation is consummation of unity of form.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: wld-dis-c-004
  - sourceClaimIds: [wld-dis-c-005-c1, wld-dis-c-005-c3]
  - sourceKeyPointIds: [k1, k4]
  - targetClaimIds: [wld-dis-c-004-c3]
  - logicalOperator: consummative_refinement
  - analysisMode: first_order_claim_projection
  - note: `c-005` completes `c-004` by naming the realized law explicitly as essential relation and showing its consummated formal unity.

- r2. type: transitions_to
  - targetEntryId: pending_next_chapter
  - sourceClaimIds: [wld-dis-c-005-c3]
  - sourceKeyPointIds: [k4]
  - targetClaimIds: [pending_cross_workbook]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: closes World Part C and prepares handoff to the next chapter-level workbook in the Appearance sequence.

Review outcome:

- review_pending
- notes: `c-005` complete; World Part C is fully complete (`c-001..c-005`).
