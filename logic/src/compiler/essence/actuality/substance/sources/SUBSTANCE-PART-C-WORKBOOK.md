# Substance Part C (TopicMap) Workbook (V1)

Part: `C. RECIPROCITY OF ACTION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `../../absolute/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `reciprocity-action.txt` as authority for Part C.
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

- file: `reciprocity-action.txt`
- active range: lines `1-end` (`C. RECIPROCITY OF ACTION`)

Decision:

- Complete Part C in one first-order claim projection pass.
- Enforce minimum three claims per entry with line-anchored evidence.
- Keep relation schema compatible with V1.1 overlay (`sourceClaimIds`, `sourceKeyPointIds`, `targetClaimIds`, `logicalOperator`, `analysisMode`).

## Decomposition status

- completed: `sub-part-c-001` for lines `3-61`
- completed: `sub-part-c-002` for lines `62-120`
- completed: `sub-part-c-003` for lines `121-163`

### Entry sub-part-c-001 — Reciprocity as sublation of mechanism and reciprocal conditioning

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/reciprocity-action.txt`
- lineStart: 3
- lineEnd: 61

Summary:

Reciprocity sublates mechanical externality by showing each substance as simultaneously active and passive, so causality is conditioned and conditioning in one movement.

Key points: (KeyPoint)

- k1. Mechanistic external causality is sublated in reciprocity of action.
- k2. Reciprocal causality initially appears as transparent reflective shine where active/passive difference is sublated.
- k3. Presupposed immediacy is removed; conditioning factor is own mediated passivity.
- k4. Causality is both conditioned and conditioning through self-negation.
- k5. In effect, cause refers back to itself.

Claims: (Claim)

- c1. id: sub-part-c-001-c1
  - subject: reciprocity_of_action
  - predicate: sublates
  - object: mechanism_as_externality_of_causality
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [3-17] "Mechanism consists in... externality... In reciprocity of action this mechanism is now sublated..."

- c2. id: sub-part-c-001-c2
  - subject: reciprocal_substances
  - predicate: are
  - object: both_active_and_passive_with_difference_sublated
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [19-28] "each... both active and passive... their difference... sublated..."

- c3. id: sub-part-c-001-c3
  - subject: external_conditioning_factor
  - predicate: is
  - object: mediated_passivity_posited_by_causalitys_own_activity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [45-50] "external factor... mediated through the causality itself... passivity posited by its own activity."

- c4. id: sub-part-c-001-c4
  - subject: causality
  - predicate: is
  - object: conditioned_and_conditioning
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [51-57] "Causality is conditioned and conditioning..."

- c5. id: sub-part-c-001-c5
  - subject: cause
  - predicate: refers_back_to_itself
  - object: in_effect
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [58-60] "the cause does not just have an effect but, in the effect, refers as cause back to itself."

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: sub-part-c-002
  - note: from reciprocal conditioning to explicit determination of freedom as manifested necessity.
  - sourceClaimIds: [sub-part-c-001-c4, sub-part-c-001-c5]
  - sourceKeyPointIds: [k4, k5]
  - targetClaimIds: [sub-part-c-002-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: sub-part-b-003
  - note: realizes Part B's infinite reciprocal action as explicit chapter determination.
  - sourceClaimIds: [sub-part-c-001-c1, sub-part-c-001-c5]
  - sourceKeyPointIds: [k1, k5]
  - targetClaimIds: [sub-part-b-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: line anchors verified against numbered source.

### Entry sub-part-c-002 — Reciprocity as manifestation of freedom through identity of necessity and contingency

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/reciprocity-action.txt`
- lineStart: 62
- lineEnd: 120

Summary:

Causality returns to its absolute concept in reciprocity, where necessity's inward identity is manifested as freedom and contingency is equally transformed into freedom.

Key points: (KeyPoint)

- k1. In reciprocity, causality returns to its absolute concept.
- k2. Necessity's substantial otherness is sublated and elevated to freedom.
- k3. Originative causality appears as becoming from and into passivity, yet as reflection-into-itself.
- k4. Necessity and causality disappear as merely inner relations; necessity unveils itself in manifested identity.
- k5. Contingent independent actualities are posited as one identity, becoming freedom.

Claims: (Claim)

- c1. id: sub-part-c-002-c1
  - subject: causality
  - predicate: returns_to
  - object: absolute_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [62-63] "Causality has thereby returned to its absolute concept..."

- c2. id: sub-part-c-002-c2
  - subject: reciprocity
  - predicate: elevates
  - object: necessity_to_freedom
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [68-73] "necessity... substantial otherness... sublated... necessity is elevated to freedom."

- c3. id: sub-part-c-002-c3
  - subject: originative_causality
  - predicate: is
  - object: becoming_from_negation_as_positive_rejoining
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [74-81] "originative causality... arising from its negation... transition into otherness is reflection-into-itself..."

- c4. id: sub-part-c-002-c4
  - subject: necessity_and_causality
  - predicate: are_sublated_as_merely_inner_relation
  - object: manifested_identity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [83-90] "necessity and causality have disappeared... absolute contradiction."
    - [101-109] "inwardness... sublates movement... necessity unveils itself... manifestation is identical movement..."

- c5. id: sub-part-c-002-c5
  - subject: contingency
  - predicate: becomes
  - object: freedom_in_identity_of_independent_actualities
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [110-119] "contingency... comes to be freedom... independent, free actualities... posited as an identity..."

Relations: (Relation)

- r1. type: refines
  - targetEntryId: sub-part-c-001
  - note: clarifies reciprocal conditioning as manifestation of freedom, not merely formal reciprocity.
  - sourceClaimIds: [sub-part-c-002-c2, sub-part-c-002-c5]
  - sourceKeyPointIds: [k2, k5]
  - targetClaimIds: [sub-part-c-001-c4, sub-part-c-001-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-part-c-003
  - note: from manifested freedom to explicit determination of universal/singular/particular and concept.
  - sourceClaimIds: [sub-part-c-002-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-part-c-003-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: preserve c4/c5 distinction for concept transition mapping.

### Entry sub-part-c-003 — Differentiated totalities and emergence of the concept

Span:

- sourceFile: `src/compiler/essence/actuality/substance/sources/reciprocity-action.txt`
- lineStart: 121
- lineEnd: 163

Summary:

Absolute substance no longer disperses into indifferent externality but differentiates into universal and singular totalities whose immediate unity as particularity yields the concept and the realm of freedom.

Key points: (KeyPoint)

- k1. Absolute substance no longer repels itself as necessity into indifferent external substances.
- k2. Substance differentiates into universal totality and singular totality.
- k3. Universal and singular are immediately identical through negativity.
- k4. Their simple identity is particularity as immediate unity.
- k5. The one reflection differentiating transparently into these totalities is the concept.

Claims: (Claim)

- c1. id: sub-part-c-003-c1
  - subject: absolute_substance
  - predicate: differentiates_into
  - object: universal_and_singular_totalities
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [121-127] "No longer... falls apart... but... differentiates itself..."
    - [133-142] "on the one hand... universal... on the other hand... singular..."

- c2. id: sub-part-c-003-c2
  - subject: universality_and_singularity
  - predicate: are
  - object: immediately_identical_through_negative_self_identity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [143-149] "the universal... immediately is the same negativity... singularity... same identity..."

- c3. id: sub-part-c-003-c3
  - subject: simple_identity_of_universal_and_singular
  - predicate: is
  - object: particularity
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [150-153] "This, their simple identity, is the particularity..."

- c4. id: sub-part-c-003-c4
  - subject: three_totalities
  - predicate: are
  - object: one_reflection_differentiating_itself_transparently
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [154-160] "These three totalities... one and the same reflection... transparent difference..."

- c5. id: sub-part-c-003-c5
  - subject: this_reflection
  - predicate: is
  - object: concept_realm_of_subjectivity_or_freedom
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [161-163] "This is the concept, the realm of subjectivity or of freedom."

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: sub-part-c-002
  - note: concretizes manifested freedom as conceptually articulated universal/singular/particular unity.
  - sourceClaimIds: [sub-part-c-003-c3, sub-part-c-003-c5]
  - sourceKeyPointIds: [k3, k5]
  - targetClaimIds: [sub-part-c-002-c2, sub-part-c-002-c5]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: sub-idea-001
  - note: chapter culmination hands off to the Substance idea-level closure anchor.
  - sourceClaimIds: [sub-part-c-003-c5]
  - sourceKeyPointIds: [k5]
  - targetClaimIds: [sub-idea-001-c1]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: refines
  - targetEntryId: sub-part-b-003
  - note: develops reciprocal action into explicit logical articulation of freedom and concept.
  - sourceClaimIds: [sub-part-c-003-c1, sub-part-c-003-c5]
  - sourceKeyPointIds: [k1, k5]
  - targetClaimIds: [sub-part-b-003-c5]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: concept handoff is anchored to `SUBSTANCE-IDEA-WORKBOOK.md`.
