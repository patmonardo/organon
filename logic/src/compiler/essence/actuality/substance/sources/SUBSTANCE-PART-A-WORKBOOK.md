# Substance Part A (TopicMap) Workbook (V1)

Part: `A. THE RELATION OF SUBSTANTIALITY`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `relation-substantiality.txt` as authority for Part A.
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

- file: `relation-substantiality.txt`
- active range: lines `1-end` (`A. THE RELATION OF SUBSTANTIALITY`)

Decision:

- Complete Part A in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `sub-part-a-001` for lines `3-58`
- completed: `sub-part-a-002` for lines `59-131`
- completed: `sub-part-a-003` for lines `132-179`

### Entry sub-part-a-001 — Substance as absolute relation and unity of being/reflection

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-substantiality.txt`
- lineStart: 3
- lineEnd: 58

Summary:

Absolute necessity as self-mediating being is substance: immediate actuality reflected into itself, whose accidentality movement unites being categories and reflective determinations.

Key points: (KeyPoint)

- k1. Absolute necessity as absolute mediation/self-relation is substance.
- k2. Substance is final unity of essence and being: immediate actuality absolutely reflected into itself.
- k3. Substance includes both self-identical being and shining accidentality.
- k4. Accidentality is the becoming/contingency movement of possibility and actuality.
- k5. This movement displays mutual reflective shining of being and essence determinations.

Claims: (Claim)

- c1. id: sub-part-a-001-c1
  - subject: absolute_necessity
  - predicate: is
  - object: absolute_relation_and_self_mediating_being_called_substance
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [3-10] "Absolute necessity is absolute relation... absolute mediation... This being is substance..."

- c2. id: sub-part-a-001-c2
  - subject: substance
  - predicate: is
  - object: immediate_actuality_reflected_into_self_as_in_and_for_itself_subsisting
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [11-17] "neither unreflected immediate... but immediate actuality itself... absolutely reflected into itself..."

- c3. id: sub-part-a-001-c3
  - subject: substance_being
  - predicate: includes
  - object: self_identical_positedness_as_shining_totality_accidentality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [18-23] "Substance... is shining and positedness... this being is substance... as such it is... accidentality."

- c4. id: sub-part-a-001-c4
  - subject: accidentality_movement
  - predicate: is
  - object: becoming_contingency_and_conversion_of_possibility_actuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [25-34] "unity of possibility and actuality... contingency... immediate conversion..."

- c5. id: sub-part-a-001-c5
  - subject: movement_of_accidentality
  - predicate: exhibits
  - object: mutual_reflective_shine_of_being_and_essence_categories
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [38-41] "movement... exhibits... reflective shine of categories of being and... essence."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: sub-part-a-002
  - note: from substance/accidentality identity to substantial actuosity as creative-destructive power.
  - sourceClaimIds: [sub-part-a-001-c3, sub-part-a-001-c5]
  - sourceKeyPointIds: [k3, k5]
  - targetClaimIds: [sub-part-a-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: act-part-c-003
  - note: carries forward absolute necessity into substance as explicit actuality-level relation.
  - sourceClaimIds: [sub-part-a-001-c1]
  - sourceKeyPointIds: [k1]
  - targetClaimIds: [act-part-c-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry sub-part-a-002 — Actuosity of substance and the power of creative/destructive conversion

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-substantiality.txt`
- lineStart: 59
- lineEnd: 131

Summary:

The movement of accidentality is substance's own actuosity; as absolute power, substance translates possible into actual and actual into possible, uniting creation and destruction.

Key points: (KeyPoint)

- k1. Accidentality movement is substance's tranquil coming-forth, active only against itself.
- k2. Sublating presupposition is disappearing shine where immediacy is posited through sublation.
- k3. Substance is totality embracing accidentality; accidentality is the whole substance itself.
- k4. Flux of accidents is substance as absolute power.
- k5. Substance manifests as creative/destructive power since possibility/actuality are absolutely united.

Claims: (Claim)

- c1. id: sub-part-a-002-c1
  - subject: movement_of_accidentality
  - predicate: is
  - object: actuosity_of_substance_against_itself
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [59-63] "movement... actuosity of substance... not active against something, but only against itself..."

- c2. id: sub-part-a-002-c2
  - subject: sublating_of_presupposition
  - predicate: is
  - object: disappearing_shine_that_posits_immediacy
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [63-67] "sublating of a presupposition is the disappearing shine... beginning... positing of this itself..."

- c3. id: sub-part-a-002-c3
  - subject: substance
  - predicate: is
  - object: totality_embracing_accidentality_where_accidentality_is_whole_substance
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [69-71] "substance... totality... embraces accidentality... accidentality is the whole substance itself."

- c4. id: sub-part-a-002-c4
  - subject: flux_of_accidents
  - predicate: is
  - object: substance_as_absolute_power
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [82-84] "flux of accidents... is... substance as absolute power."

- c5. id: sub-part-a-002-c5
  - subject: substance
  - predicate: manifests_as
  - object: creative_and_destructive_power_in_absolute_union_of_possibility_negativity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [94-101] "substance manifests... as creative power... as destructive power... creating is destructive... absolutely united."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: sub-part-a-001
  - note: determines how substance/accidentality identity operates as self-activity.
  - sourceClaimIds: [sub-part-a-002-c1, sub-part-a-002-c5]
  - sourceKeyPointIds: [k1, k5]
  - targetClaimIds: [sub-part-a-001-c3, sub-part-a-001-c4]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-part-a-003
  - note: from substantial power to explicit mediation difference and passage toward causality.
  - sourceClaimIds: [sub-part-a-002-c4, sub-part-a-002-c5]
  - sourceKeyPointIds: [k4, k5]
  - targetClaimIds: [sub-part-a-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: preserve c5 for causal power handoff in Part B.

### Entry sub-part-a-003 — Substantiality as vanishing relation and transition to causality

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/relation-substantiality.txt`
- lineStart: 132
- lineEnd: 179

Summary:

Because substance is immediately present in accidents, real difference is not yet fully posited; substantiality is a vanishing relation that passes into causality when accidentality is posited as self-referring negativity.

Key points: (KeyPoint)

- k1. Immediate identity of substance in accidents means no real difference yet present.
- k2. Substance as power mediates difference; necessity is middle uniting substantiality and accidentality.
- k3. Substantiality is only immediately vanishing relation, with negativity not yet fully retained.
- k4. Initially substance appears as inner of accidents and formal power.
- k5. Posited accidentality as self-referring negativity yields transition into causality.

Claims: (Claim)

- c1. id: sub-part-a-003-c1
  - subject: immediate_presence_of_substance_in_accidents
  - predicate: entails
  - object: absence_of_real_difference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [132-136] "Because of this immediate identity... there is still no real difference present..."

- c2. id: sub-part-a-003-c2
  - subject: substance_as_power
  - predicate: mediates
  - object: difference_as_necessity_middle_of_substantiality_and_accidentality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [137-145] "substance... as power... mediates the difference... necessity... middle... unity of substantiality and accidentality..."

- c3. id: sub-part-a-003-c3
  - subject: substantiality
  - predicate: is
  - object: immediately_vanishing_relation_with_unretained_negativity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [146-155] "Substantiality is... relation as immediately vanishing... does not [retain] negative essence..."

- c4. id: sub-part-a-003-c4
  - subject: first_relation_of_substantiality
  - predicate: is
  - object: formal_power_where_substance_is_inner_of_accidents
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [165-170] "relation... formal power... substance only is as the inner of the accidents..."

- c5. id: sub-part-a-003-c5
  - subject: relation_of_substantiality
  - predicate: passes_over_into
  - object: relation_of_causality
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [171-179] "accidentality... posited as... self-referring negativity... relation of substantiality passes over into the relation of causality."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: sub-part-a-002
  - note: transforms sheer substantial power into explicit mediated difference and relational transition.
  - sourceClaimIds: [sub-part-a-003-c2, sub-part-a-003-c5]
  - sourceKeyPointIds: [k2, k5]
  - targetClaimIds: [sub-part-a-002-c4, sub-part-a-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-part-b-001
  - note: explicit handoff from substantiality relation to causality relation.
  - sourceClaimIds: [sub-part-a-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-b-001-c1, sub-part-b-001-c3]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: refines
  - targetEntryId: sub-part-a-001
  - note: clarifies that initial substance identity remains incomplete until mediated negativity is posited.
  - sourceClaimIds: [sub-part-a-003-c1, sub-part-a-003-c3]
  - sourceKeyPointIds: [k1, k3]
  - targetClaimIds: [sub-part-a-001-c1, sub-part-a-001-c3]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: Part B handoff claim targets are now resolved.
