# Thing Part C (TopicMap) Workbook (V2)

Part: `C. DISSOLUTION OF THE THING`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `dissolution.txt` as authority for Part C.
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

## Session: 2026-02-19 (initial scaffold)

Scope:

- file: `dissolution.txt`
- active range: full Part C source in this file

Decision:

- Initialize Part C workbook structure before full entry extraction.
- Part C is non-numbered in this source; do not impose synthetic `1/2/3` decomposition.
- Use conceptual sentence-group spans only.
- Decomposition policy update: treat Part C as three core paragraphs plus a dedicated transition paragraph.
- Special handling rule: the final transition paragraph is not merged into the third core analysis block; it is modeled as a distinct transition entry for cross-chapter handoff.

## Decomposition status

- completed: `thg-dis-c-001` for paragraph 1 (line `4` to `19`)
- completed: `thg-dis-c-002` for paragraph 2 (line `20` to `55`)
- completed: `thg-dis-c-003` for paragraph 3 (line `56` to `102`)
- completed: `thg-dis-c-004` for special transition paragraph (line `103` to `110`) -> handoff into Appearance/World architecture

### Entry thg-dis-c-001 — Quantitative thinghood as absolute alterability and porous dissolution

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/dissolution.txt`
- lineStart: 4
- lineEnd: 19

Summary:

In Part C paragraph 1, the thing determined as a merely quantitative combination of free matters is shown as absolutely alterable, with coming-to-be and passing-away as external dissolution and with thinghood reduced to porous circulation without intrinsic form.

Key points: (KeyPoint)

- k1. The thing as quantitative combination of free matters is absolutely alterable.
- k2. Alteration consists in additive/subtractive/rearranged quantitative composition of matters.
- k3. The thing's becoming and perishing are external dissolution, and the thing is porous without proper measure or form.

Claims: (Claim)

- c1. id: thg-dis-c-001-c1
  - subject: thing_as_quantitative_combination_of_free_matters
  - predicate: is
  - object: absolutely_alterable
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-6] the thing, as merely quantitative combination of free matters, is absolutely alterable.

- c2. id: thg-dis-c-001-c2
  - subject: alteration_of_the_thing
  - predicate: consists_in
  - object: quantitative_addition_subtraction_and_rearrangement_of_matters
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [7-11] alteration defined as matters dropped/added/rearranged in quantitative ratio.

- c3. id: thg-dis-c-001-c3
  - subject: coming_to_be_and_passing_away_of_the_thing
  - predicate: are
  - object: external_dissolution_of_indifferent_bonding_and_absolute_porosity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [12-16] coming-to-be/passing-away are external dissolution of indifferent bonds.
    - [17-19] stuffs circulate unchecked; the thing is absolute porosity without measure/form.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: thg-mat-b-003
  - sourceClaimIds: [thg-dis-c-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-mat-b-003-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection
  - note: dissolution paragraph 1 operationalizes Part B's result that thinghood is a quantitative collection of indifferent matters.

- r2. type: transitions_to
  - targetEntryId: thg-dis-c-002
  - sourceClaimIds: [thg-dis-c-001-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: move from external porosity/alterability to explicit determination of the thing as absolutely dissoluble.

Review outcome:

- review_pending
- notes: `c-001` complete under the three-core-plus-transition Part C decomposition.

### Entry thg-dis-c-002 — The thing as absolutely dissoluble self-dissolution and negative unity

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/dissolution.txt`
- lineStart: 20
- lineEnd: 55

Summary:

Part C paragraph 2 determines the thing as absolutely dissoluble such that its essential being is externality, yet this same thing is the dissolution of itself because the matters' determinateness makes their relation a negative reflection that constitutes the thing's puncticity.

Key points: (KeyPoint)

- k1. As a determinate “this,” the thing is absolutely dissoluble, and dissolution/externality are essential to its being.
- k2. The thing is not merely an abstract “this” or indifferent “also”; the whole thing is the dissolution of itself.
- k3. Because matter-content is determinate and referred-to-other, matters are negatively related, and this negative reflection is the puncticity of the thing.

Claims: (Claim)

- c1. id: thg-dis-c-002-c1
  - subject: thing_as_absolute_this
  - predicate: is
  - object: absolutely_dissoluble_with_externality_as_essential_being
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [20-27] thing as absolute determinateness of “this” is absolutely dissoluble; dissolution/externality are essential.

- c2. id: thg-dis-c-002-c2
  - subject: this_thing_whole
  - predicate: is
  - object: dissolution_of_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [28-31] thing consists not only in abstract “this” but as whole is the dissolution of itself.
    - [32-39] it is an external collection of self-subsisting matters while thinghood belongs to reflected properties.

- c3. id: thg-dis-c-002-c3
  - subject: relation_of_matters_in_the_thing
  - predicate: is
  - object: negative_reflection_as_puncticity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [40-48] content determinateness refers to an other; matters are not merely indifferent “also.”
    - [49-55] matters are themselves negative reflection, the puncticity of the thing, via differential determinateness.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: thg-dis-c-001
  - sourceClaimIds: [thg-dis-c-002-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [thg-dis-c-001-c3]
  - logicalOperator: determinative_refinement
  - analysisMode: first_order_claim_projection
  - note: c-002 makes explicit that porous externality from c-001 is the essential being of the thing as absolutely dissoluble.

- r2. type: transitions_to
  - targetEntryId: thg-dis-c-003
  - sourceClaimIds: [thg-dis-c-002-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: the established punctic negative unity opens into paragraph 3's full contradictory mediation of interpenetrating porous matters.

Review outcome:

- review_pending
- notes: `c-002` complete; relation depth from self-dissolution to contradictory mediation continues in `c-003`.

### Entry thg-dis-c-003 — Contradictory mediation of interpenetrating porous matters

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/dissolution.txt`
- lineStart: 56
- lineEnd: 102

Summary:

Part C paragraph 3 articulates the thing as a self-contradictory mediation in which matters both subsist and do not subsist in one another, interpenetrate without contact through essential porosity, and thus realize negative unity through reciprocal sublation.

Key points: (KeyPoint)

- k1. The thing is the connecting reference of matters where each subsists only through the sublation of the other.
- k2. This reciprocal subsistence/non-subsistence is the puncticity or negative unity in which matters interpenetrate absolutely without touching.
- k3. Essential porosity makes each matter's subsistence simultaneously its sublatedness, yielding the thing as self-contradictory mediation of self-subsistence through negation.

Claims: (Claim)

- c1. id: thg-dis-c-003-c1
  - subject: thing
  - predicate: is
  - object: connecting_reference_of_matters_via_reciprocal_sublation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [56-67] one matter subsists in the thing only insofar as the other is sublated, while the thing remains the “also” of the other.
    - [68-72] with diverse matters, each both subsists and does not subsist in relation to the others.

- c2. id: thg-dis-c-003-c2
  - subject: one_subsistence_of_matters
  - predicate: is
  - object: puncticity_or_negative_unity_with_absolute_interpenetration
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [73-78] shared subsistence is puncticity/negative unity; the two interpenetrate absolutely.
    - [79-83] as indifferent matters in interpenetration, they do not touch.

- c3. id: thg-dis-c-003-c3
  - subject: thing
  - predicate: is
  - object: self_contradictory_mediation_of_self_subsistence_through_negation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [84-98] matters are essentially porous; each subsistence is simultaneously sublatedness and the subsistence of others.
    - [99-102] the thing is self-contradictory mediation of independent subsistence through its opposite/negation.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: thg-dis-c-002
  - sourceClaimIds: [thg-dis-c-003-c1, thg-dis-c-003-c2]
  - sourceKeyPointIds: [k1, k2]
  - targetClaimIds: [thg-dis-c-002-c3]
  - logicalOperator: dialectical_expansion
  - analysisMode: first_order_claim_projection
  - note: c-003 unfolds the prior claim of negative reflection/puncticity into explicit reciprocal subsistence, interpenetration, and non-contact.

- r2. type: transitions_to
  - targetEntryId: thg-dis-c-004
  - sourceClaimIds: [thg-dis-c-003-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [pending_local]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: contradictory mediation now sets the direct transition to appearance in the final dedicated handoff paragraph.

Review outcome:

- review_pending
- notes: `c-003` complete; next entry is the reserved transition paragraph into Appearance/World architecture.

### Entry thg-dis-c-004 — Completion of concrete existence as appearance through absolute otherness

Span:

- sourceFile: `src/relative/essence/appearance/thing/sources/dissolution.txt`
- lineStart: 103
- lineEnd: 110

Summary:

In the concluding transition paragraph, concrete existence reaches completion as the unity of independent subsistence and unessentiality, whose truth is subsistence in absolute otherness (its own nothingness as substrate), and this is explicitly named appearance.

Key points: (KeyPoint)

- k1. In this thing, concrete existence is complete as both in-itself being (independent subsistence) and unessential concrete existence.
- k2. The truth of concrete existence is to have its in-itself in unessentiality, subsisting in an absolute other.
- k3. Concrete existence has its own nothingness as substrate and is therefore appearance.

Claims: (Claim)

- c1. id: thg-dis-c-004-c1
  - subject: concrete_existence_in_this_thing
  - predicate: has_attained
  - object: completion_as_unity_of_independent_subsistence_and_unessential_concrete_existence
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [103-105] concrete existence is at once independent subsistence and unessential concrete existence.

- c2. id: thg-dis-c-004-c2
  - subject: truth_of_concrete_existence
  - predicate: is
  - object: subsistence_in_absolute_otherness_and_own_nothingness_as_substrate
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [106-109] truth is in-itself in unessentiality, subsisting in an other, indeed the absolute other, with own nothingness as substrate.

- c3. id: thg-dis-c-004-c3
  - subject: concrete_existence_so_determined
  - predicate: is
  - object: appearance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [110] "It is, therefore, appearance."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: thg-dis-c-003
  - sourceClaimIds: [thg-dis-c-004-c2]
  - sourceKeyPointIds: [k2]
  - targetClaimIds: [thg-dis-c-003-c3]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection
  - note: c-004 states the explicit truth-form of the contradictory mediation developed in c-003.

- r2. type: transitions_to
  - targetEntryId: wld-law-a-001
  - sourceClaimIds: [thg-dis-c-004-c3]
  - sourceKeyPointIds: [k3]
  - targetClaimIds: [wld-law-a-001-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection
  - note: explicit handoff from thing-dissolution's conclusion (appearance) into `A. THE LAW OF APPEARANCE`.

Review outcome:

- review_pending
- notes: transition paragraph complete; Thing Part C now fully complete and linked to World Part A.
