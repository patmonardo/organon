# Essence Part B (TopicMap) Workbook (V2)

Part: `B. SHINE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.
- Do not introduce alternate entry styles, headings, or compressed formats.

## Clean-room rules

- Source authority is `shine.txt` only.
- Claims must be line-anchored.
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

## Session: 2026-02-17 (prepared)

Scope:

- file: `shine.txt`
- focus: how shine is non-self-subsistent immediacy and essence's own determinateness
- fixed range (active in this pass): lines `6-36`, `95-215`

Decision:

- Use Part A workbook format unchanged.
- Exclude comparative skepticism/idealism discussion (`38-93`) from active base entries.
- Working segmentation (provisional): entries use spans (`6-36`, `95-131`, `133-175`, `177-190`) followed by a concluding transition (`192-215`).
- Section labels are not encoded in entry headings in this pass.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Align this pass with `WORKBOOK-CONTRACT-V1.md` Path-forward principle (`LogoLogia`) and KG-first direction.
- Keep `Key points` and `Claims` first-order, source-restricted, and line-anchored to `shine.txt`.
- Treat `Relations` as second-order modeling expanded across iterative cycles (`Shine -> Reflection -> relation-expansion cycles`).
- Keep `review_pending` where relation semantics remain provisional.
- Use comparative block (`38-93`) as second-order relation reservoir (not base claims), especially for skepticism/idealism and realism/idealism dialectical polarity.
- When relation-cycling this block, model philosophical references as interpretive scaffolding in relation notes, not as first-order source claims.
- Treat relation-building as a proof-trace of the passage from Essence/Appearance into Concept, to inform the Meta Compiler Generation system.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay for this workbook pass; keep claims unchanged.

### Entry ess-ref-b-001 — Shine as non-self-subsistent immediacy

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/shine.txt`
- lineStart: 6
- lineEnd: 36

Summary:

Part B opens by determining shine as the residue of sublated being: immediate non-being that has existence only in negation and mediation.

Key points: (KeyPoint)

- k1. Shine's being consists solely in being's sublatedness/nothingness.
- k2. As the leftover of the being-sphere, shine still appears as an immediate side seemingly independent of essence.
- k3. Otherness here contains the two moments of existence and non-existence, but for the unessential only non-existence remains.
- k4. Shine is immediate non-existence in the determinateness of being, existing only in reference-to-another and in negation.
- k5. Shine's immediacy is reflected/mediated, an empty determinateness rather than simple immediacy.

Claims: (Claim)

- c1. id: ess-ref-b-001-c1
  - subject: shine
  - predicate: is
  - object: residue_of_being_as_sublated_nothingness
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [6-12] shine as being's sublatedness/nothingness and negative posited as negative.
    - [14-17] shine as what remains from being while seeming immediate and other than essence.

- c2. id: ess-ref-b-001-c2
  - subject: shine
  - predicate: exists_only_in
  - object: nonexistence_relation_and_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [18-23] otherness has existence/non-existence moments; for the unessential only non-existence remains as immediate non-existence.
    - [24-28] shine exists only with reference to another, in its negation, as non-self-subsistent.

- c3. id: ess-ref-b-001-c3
  - subject: shine_immediacy
  - predicate: is
  - object: reflected_mediated_empty_immediacy
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [29-36] pure determinateness of immediacy as reflected immediacy through mediation of negation and empty immediacy of non-existence.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4
- c3 -> k5

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: ess-ref-b-002
  - note: move from first determination of shine to explicit statement of the task.
  - sourceClaimIds: [`ess-ref-b-001-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`ess-ref-b-002-c3`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: comparative block [38-93] excluded from base claims in this pass and reserved for second-order relation expansion cycles.

### Entry ess-ref-b-002 — Task shift from return-proof to immanent demonstration

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/shine.txt`
- lineStart: 95
- lineEnd: 131

Summary:

The entry states the methodological task and immediately grounds it in what constitutively defines shine as immediacy of non-being.

Key points: (KeyPoint)

- k1. Shine appears as immediate presupposition against essence.
- k2. Being's return into essence is already complete.
- k3. The method is demonstration of internal identity of determinations.
- k4. What constitutes shine is immediacy of non-being, i.e., essence's own negativity and reflective immediacy.

Claims: (Claim)

- c1. id: ess-ref-b-002-c1
  - subject: shine
  - predicate: appears_as
  - object: immediate_presupposition_against_essence
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [95-97] shine contains an immediate presupposition, an independent side vis-à-vis essence.

- c2. id: ess-ref-b-002-c2
  - subject: return_of_being_to_essence
  - predicate: is
  - object: already_accomplished
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [98-101] not to show return; being has returned in totality; shine is the null as such.

- c3. id: ess-ref-b-002-c3
  - subject: task
  - predicate: is_to_demonstrate_and_ground
  - object: shine_as_essence_determinations_through_constitutive_immediacy
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [102-105] determinations distinguishing shine from essence are determinations of essence itself and sublated in it.
    - [107-131] shine is constituted as immediacy of non-being and reflective immediacy (being as moment).

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-b-001
  - note: methodological precision on what exactly must be shown.
  - sourceClaimIds: [`ess-ref-b-002-c2`, `ess-ref-b-002-c3`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`ess-ref-b-001-c2`, `ess-ref-b-001-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-b-003
  - note: after constitutive grounding, the exposition moves to explicit two-moment identity and self-sublation.
  - sourceClaimIds: [`ess-ref-b-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`ess-ref-b-003-c1`, `ess-ref-b-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: entry now includes the constitutive/immediacy paragraph through line 131.

### Entry ess-ref-b-003 — Middle movement (identity and self-sublation)

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/shine.txt`
- lineStart: 133
- lineEnd: 175

Summary:

The middle movement begins from the explicit two-moment identity and develops it into determinateness and self-sublation.

Key points: (KeyPoint)

- k1. The two moments of shine are explicitly stated as moments of essence itself.
- k2. Shine in essence is not that of an other but essence's own shining.
- k3. Shine is explicitly essence in determinateness.
- k4. Essence is presented as unity of negativity and immediacy.
- k5. Determinateness self-sublates as the negative returns to itself.

Claims: (Claim)

- c1. id: ess-ref-b-003-c1
  - subject: shine_moments
  - predicate: are
  - object: moments_of_essence_itself
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [133-141] the two moments of shine are moments of essence; shine in essence is essence's own shining.

- c2. id: ess-ref-b-003-c2
  - subject: shine_in_determinateness
  - predicate: is_identical_with
  - object: essence_as_unity_of_negativity_and_immediacy
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [143-147] shine is essence itself in determinateness.
    - [148-159] essence as identical unity of negativity and immediacy.

- c3. id: ess-ref-b-003-c3
  - subject: shine_determinateness
  - predicate: self_sublates_as
  - object: return_to_self
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [161-175] non-self-subsistent negative in another/negation and absolute sublation of determinateness.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4
- c3 -> k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: ess-ref-b-002
  - note: executes the task by deriving identity of determinations.
  - sourceClaimIds: [`ess-ref-b-003-c1`, `ess-ref-b-003-c2`, `ess-ref-b-003-c3`]
  - sourceKeyPointIds: [`k2`, `k4`, `k5`]
  - targetClaimIds: [`ess-ref-b-002-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-b-004
  - note: from middle movement to the concluding paragraph on determinateness-as-moment.
  - sourceClaimIds: [`ess-ref-b-003-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`ess-ref-b-004-c1`, `ess-ref-b-004-c3`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second entry now starts at line 133.

### Entry ess-ref-b-004 — Infinite determinateness as moment

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/shine.txt`
- lineStart: 177
- lineEnd: 190

Summary:

This paragraph states infinite determinateness and concludes with shine as essence itself where determinateness is only a moment.

Key points: (KeyPoint)

- k1. Determinateness in essence is infinite determinateness.
- k2. The negative that coincides with itself is self-subsistent as determinateness.
- k3. Self-subsistence as immediacy is equally determinateness and moment.
- k4. Identity of negativity and immediacy is essence.
- k5. Shine is essence itself in determinateness, where determinateness is only a moment.

Claims: (Claim)

- c1. id: ess-ref-b-004-c1
  - subject: determinateness_in_essence
  - predicate: is
  - object: infinite_determinateness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [177-181] determinateness in essence is infinite; negative coincides with itself.

- c2. id: ess-ref-b-004-c2
  - subject: self_subsistence_and_determinateness
  - predicate: are
  - object: reciprocal_moments_in_identity_of_negativity_and_immediacy
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [182-186] self-subsistence and determinateness are reciprocal moments; negativity/immediacy identity is essence.

- c3. id: ess-ref-b-004-c3
  - subject: shine
  - predicate: is
  - object: essence_itself_with_determinateness_as_moment
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [187-190] shine is essence itself; determinateness is only a moment; essence shines within itself.

Claim ↔ key point map:

- c1 -> k1, k5
- c2 -> k2, k3, k4
- c3 -> k5

Relations: (Relation)

- r1. type: refines
  - targetEntryId: ess-ref-b-003
  - note: states explicitly the identity result toward which the middle movement was directed.
  - sourceClaimIds: [`ess-ref-b-004-c1`, `ess-ref-b-004-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`, `k4`]
  - targetClaimIds: [`ess-ref-b-003-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: ess-ref-b-005
  - note: transition from determinateness-as-moment into reflection movement.
  - sourceClaimIds: [`ess-ref-b-004-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`ess-ref-b-005-c3`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: this entry uses the single paragraph 177-190.

### Entry ess-ref-b-005 — Transition: into reflection

Span:

- sourceFile: `src/relative/essence/reflection/essence/sources/shine.txt`
- lineStart: 192
- lineEnd: 215

Summary:

This transition re-situates shine within the being/essence contrast and shows that sublating first immediacy yields essence's self-movement as reflection.

Key points: (KeyPoint)

- k1. Being/non-being in the being-sphere has becoming as truth.
- k2. Non-essential and shine are leftovers of being in the essence-sphere.
- k3. The apparent split comes from taking essence first as immediate.
- k4. Sublation shows unessential as shine contained in essence.
- k5. Essence determines immediacy as negativity and negativity as immediacy.

Claims: (Claim)

- c1. id: ess-ref-b-005-c1
  - subject: being_sphere_pair
  - predicate: has_truth
  - object: becoming
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [192-193] being/non-being truth is becoming.

- c2. id: ess-ref-b-005-c2
  - subject: nonessential_and_shine
  - predicate: are
  - object: leftovers_of_being_within_essence_contrast
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [194-198] contrast of essence with non-essential/shine; both leftovers of being.

- c3. id: ess-ref-b-005-c3
  - subject: essence
  - predicate: is
  - object: self_movement_reflection
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [207-215] sublation of first immediacy and essence as infinite self-contained movement; essence is reflection.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3, k4
- c3 -> k5

Relations: (Relation)

- r1. type: sublates
  - targetEntryId: ess-ref-b-001
  - note: transition retroactively resolves first immediacy into reflection.
  - sourceClaimIds: [`ess-ref-b-005-c2`, `ess-ref-b-005-c3`]
  - sourceKeyPointIds: [`k2`, `k4`, `k5`]
  - targetClaimIds: [`ess-ref-b-001-c1`, `ess-ref-b-001-c3`]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

- r2. type: supports
  - targetEntryId: ess-ref-b-004
  - note: confirms section-2 determinateness-as-moment within the movement to reflection.
  - sourceClaimIds: [`ess-ref-b-005-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`ess-ref-b-004-c2`, `ess-ref-b-004-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition boundary fixed at final line of Part B.
