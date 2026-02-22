# Being-for-self Part C Workbook

Part: `C. REPULSION AND ATTRACTION`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quality/being-for-self/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `attraction.txt` as authority.
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

### Entry (Topic) `id` — `title`

- span: `lineStart-lineEnd`
- summary: one sentence
- keyPoints: (KeyPoint) 3-8 non-redundant points
- claims: (Claim) 1-3 minimum, with evidence
- relations: (Relation) typed only (`supports|contrasts|negates|sublates|presupposes|refines|transitions_to`), using labeled bullets (`r1`, `r2`, ...)

## Session: 2026-02-22 (deep pass)

Scope:

- file: `attraction.txt`
- fixed range: full file
- pass policy: lock species (`a`/`b`/`c`) with analytic subentries

Decision:

- Species-first IDs for Part C:
  - Marker: `bfs-c`
  - Species groups from source labels: `c-a`, `c-b`, `c-c`
  - Numbered subentries within each species: `<species>-001..003`
- Numbered subentries are analytic segmentations, not additional source-labeled species.
- Keep transition closure available toward Quantity stage in later passes.

### Entry bfs-c — Marker `C`: Repulsion and attraction

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/attraction.txt`
- lineStart: 2
- lineEnd: 392

Summary:

Part C presents repulsion and attraction as inseparable moments: reciprocal exclusion among ones dissolves into attraction, and their mediated unity prepares the transition beyond quality.

Key points: (KeyPoint)

- k1. Exclusion among many ones reveals contradiction in repulsion and drives attraction.
- k2. Attraction is not external addition but immanent in repulsion's own movement.
- k3. Repulsion and attraction are mutually mediated and tend toward conceptual unity.

Claims: (Claim)

- c1. id: bfs-c-c1
  - subject: repulsion_of_many_ones
  - predicate: is_determined_as
  - object: mutual_exclusion_that_dissolves_itself
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [29-40] mutual repulsion posits many ones while undoing ideal unity.
    - [53-64] self-preservation through repulsion shows itself as dissolution.

- c2. id: bfs-c-c2
  - subject: attraction
  - predicate: emerges_as
  - object: self_positing_in_a_one_of_repulsion_process
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [137] attraction as self-positing-in-a-one.
    - [147-156] repulsion passes into attraction and presupposes it.
    - [198-212] one-one of attraction contains mediation of repulsion.

- c3. id: bfs-c-c3
  - subject: repulsion_and_attraction
  - predicate: are
  - object: inseparable_mutual_mediation_of_one_process
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [278-287] inseparability and mutual positing through mediation.
    - [293-301] each is repulsion/attraction only through the other and through self-mediation.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-b
  - note: Part C develops repulsion announced in Part B into explicit dual movement with attraction.
  - sourceClaimIds: [`bfs-c-c1`, `bfs-c-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-b-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: qty-a
  - note: mediated unity of repulsion/attraction prepares transition from quality to quantity.
  - sourceClaimIds: [`bfs-c-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry c-a-001 — `a. Exclusion of the one`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/attraction.txt`
- lineStart: 4
- lineEnd: 138

Summary:

Exclusion makes mutual repulsion explicit as the unstable self-preservation of many ones, and this very instability yields attraction as their self-positing-into-one.

Key points: (KeyPoint)

- k1. Exclusion is mutual, relative repelling among already-opposed ones.
- k2. Reciprocal repulsion both preserves and dissolves onehood.
- k3. The collapse of externality in repulsion yields attraction.

Claims: (Claim)

- c1. id: c-a-001-c1
  - subject: exclusion
  - predicate: is_determined_as
  - object: mutual_relative_repulsion_of_many_ones
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [14-20] repulsion appears as mutual excluding limited by being of ones.
    - [29-40] mutual negation posits being-only-for-one and repels this ideality.

- c2. id: c-a-001-c2
  - subject: self_preservation_of_ones
  - predicate: turns_into
  - object: dissolution
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [53-64] self-preservation via repulsion is rather dissolution.
    - [104-121] negation of reciprocal negating negates their being.

- c3. id: c-a-001-c3
  - subject: attraction
  - predicate: emerges_as
  - object: self_positing_of_many_ones_into_one
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [127-132] negative relation of ones is coming-together-with-self.
    - [137] self-positing-in-a-one of many ones is attraction.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-c
  - note: unpacks the opening exclusion movement summarized by marker claim c1.
  - sourceClaimIds: [`c-a-001-c1`, `c-a-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`bfs-c-c1`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-b-001
  - note: attraction announced at end of exclusion section becomes explicit mediated one-one.
  - sourceClaimIds: [`c-a-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry c-b-001 — `b. The one one of attraction`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/attraction.txt`
- lineStart: 139
- lineEnd: 212

Summary:

Attraction realizes ideality presupposing repulsion: the one-one is a mediated one that contains repulsion and thus preserves multiplicity within unity.

Key points: (KeyPoint)

- k1. Repulsion passes into attraction; attraction presupposes repulsion's produced many.
- k2. Attraction and repulsion are inseparable.
- k3. The one-one is mediated and includes repulsion as its own determination.

Claims: (Claim)

- c1. id: c-b-001-c1
  - subject: attraction
  - predicate: presupposes
  - object: repulsion_and_many_ones
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [147-156] repulsion passes into attraction; attraction has repulsion as presupposition.
    - [160-168] continuing attraction presupposes continuing generation of ones.

- c2. id: c-b-001-c2
  - subject: attraction
  - predicate: is
  - object: inseparable_from_repulsion
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [169-173] attraction contains negation of itself and is inseparable from repulsion.

- c3. id: c-b-001-c3
  - subject: one_one
  - predicate: is_determined_as
  - object: mediated_unity_of_repulsion_and_attraction
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [191-202] one of attraction is mediated one containing mediation by repulsion.
    - [206-212] repulsion preserves many within attraction; unity of both is present.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-c
  - note: details the marker-level claim that attraction emerges immanently.
  - sourceClaimIds: [`c-b-001-c1`, `c-b-001-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`bfs-c-c2`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: c-c-001
  - note: mediated one-one leads into explicit conceptual connection of repulsion and attraction.
  - sourceClaimIds: [`c-b-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`c-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

### Entry c-c-001 — `c. The connection of repulsion and attraction`

Span:

- sourceFile: `src/compiler/being/quality/being-for-self/sources/attraction.txt`
- lineStart: 213
- lineEnd: 392

Summary:

Repulsion and attraction are shown as inseparable presupposing moments whose reciprocal mediation is simultaneously each one's self-mediation, restoring conceptual unity and preparing transition to quantity.

Key points: (KeyPoint)

- k1. Repulsion and attraction first appear externally separated but essentially joined.
- k2. Each presupposes the other, so each is only through mediation by the other.
- k3. Their reciprocal mediation turns into self-mediation and conceptual unity.

Claims: (Claim)

- c1. id: c-c-001-c1
  - subject: repulsion_and_attraction
  - predicate: are
  - object: externally_distinct_but_essentially_joined
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [215-224] difference appears as two references yet essential joining is asserted.
    - [226-242] repulsion appears immediate while attraction seems external.

- c2. id: c-c-001-c2
  - subject: repulsion_and_attraction
  - predicate: mutually_presuppose
  - object: one_another_as_conditions
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [258-276] attraction is within repulsion; reverse presupposition also present.
    - [293-296] each is itself only through mediation of the other.

- c3. id: c-c-001-c3
  - subject: mediation_of_repulsion_and_attraction
  - predicate: culminates_in
  - object: self_mediation_and_restored_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [297-301] mediation through the other is negated into each determination's self-mediation.
    - [350-369] each contains the other as moment and transitions into the other.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: bfs-c
  - note: specifies the marker claim that repulsion and attraction are inseparable mutual mediation.
  - sourceClaimIds: [`c-c-001-c2`, `c-c-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`bfs-c-c3`]
  - logicalOperator: decomposition
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: qty-a
  - note: resolved unity of repulsion/attraction provides the final quality-to-quantity handoff.
  - sourceClaimIds: [`c-c-001-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`pending_cross_workbook`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: marker plus lettered subtopic entries completed in bounded first pass.
