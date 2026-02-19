# Foundation Part B (TopicMap) Workbook (V2)

Part: `B. DIFFERENCE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `difference.txt` as authority.
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

## Session: 2026-02-18 (fresh Part B pass)

Scope:

- file: `difference.txt`
- section range: lines `2-589`

Decision:

- Fresh TopicMap analysis from source text only.
- Keep claim count minimal and non-redundant.
- Adopt pseudo-Cypher KG labels in entry headers: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.
- Relations expansion beyond local movement is deferred until Ground synthesis; keep only minimal transition/support links in this pass.
- Apply complex-section exception for Difference: use richer multi-entry decomposition to match textual movement without forcing uniform section size.
- Migration: `relation_schema_v1 -> relation_schema_v1_1_overlay` (non-breaking).
- Apply relation claim/keypoint anchor overlay for this workbook pass; keep claims unchanged.

### Entry fnd-dif-b-001 — Absolute difference is self-referring negativity

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 2
- lineEnd: 48

Summary:

Difference first appears as reflection's own negativity and is defined as difference in and for itself, not as external otherness.

Key points: (KeyPoint)

- k1. Difference is an essential moment internal to reflection and identity.
- k2. Absolute difference is simple, self-referring difference.
- k3. Difference of reflection is not the external otherness of existence.
- k4. Reflection-posited difference is explicit as what it is in itself.

Claims: (Claim)

- c1. id: fnd-dif-b-001-c1
  - subject: difference
  - predicate: is
  - object: reflection_internal_negativity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [6-12] "Difference is the negativity that reflection possesses in itself..."

- c2. id: fnd-dif-b-001-c2
  - subject: absolute_difference
  - predicate: is_determined_as
  - object: in_and_for_itself_self_referring_difference
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [14-18] "This difference is difference in and for itself... self-referring, hence simple, difference."

- c3. id: fnd-dif-b-001-c3
  - subject: difference_of_reflection
  - predicate: is_not
  - object: external_otherness_of_existence
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [25-27] "It is the difference of reflection, not the otherness of existence."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: fnd-dif-b-002
  - note: move from absolute difference to difference as unity of itself and identity.
  - sourceClaimIds: [`fnd-dif-b-001-c2`, `fnd-dif-b-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`fnd-dif-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: boundary locked to first absolute-difference movement.

### Entry fnd-dif-b-002 — Difference in itself is self-reference containing identity

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 49
- lineEnd: 83

Summary:

Difference in itself is self-differentiation that contains identity as its own moment and grounds reflection's self-movement.

Key points: (KeyPoint)

- k1. Difference is self-differentiation, not external comparison.
- k2. Difference and identity are reciprocal moments of one whole.
- k3. Reflectional self-reference is the primordial origin of activity and self-movement.

Claims: (Claim)

- c1. id: fnd-dif-b-002-c1
  - subject: difference_in_itself
  - predicate: refers_to
  - object: itself_as_its_own_other
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [49-54] "Difference in itself is the difference that refers itself to itself... of itself from itself."

- c2. id: fnd-dif-b-002-c2
  - subject: difference
  - predicate: includes
  - object: identity_as_internal_moment
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [55-60] "What is different from difference... Difference is, therefore, itself and identity..."

- c3. id: fnd-dif-b-002-c3
  - subject: reflectional_self_reference
  - predicate: grounds
  - object: activity_and_self_movement
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [73-79] "This is to be regarded as the essential nature of reflection... primordial origin of all activity and self-movement."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-001
  - note: deepens absolute difference into explicit internal two-moment structure.
  - sourceClaimIds: [`fnd-dif-b-002-c1`, `fnd-dif-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-dif-b-001-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-003
  - note: hands off to the third numbered movement of Absolute difference.
  - sourceClaimIds: [`fnd-dif-b-002-c2`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`fnd-dif-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: entry span aligned to source numbering (Absolute difference §2).

### Entry fnd-dif-b-003 — Difference's reflected two-moment form is diversity

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 84
- lineEnd: 95

Summary:

Difference, insofar as it contains two self-reflected moments (identity and difference), is determined as diversity.

Key points: (KeyPoint)

- k1. Difference includes identity and difference as two moments.
- k2. Each moment is a self-referring positedness.
- k3. Their reflected duality determines diversity.

Claims: (Claim)

- c1. id: fnd-dif-b-003-c1
  - subject: difference
  - predicate: has_moments
  - object: identity_and_difference_as_positedness
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [84-90] "Difference has both these moments, identity and difference; thus the two are both a positedness, determinateness..."

- c2. id: fnd-dif-b-003-c2
  - subject: each_moment
  - predicate: refers_to
  - object: itself
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [87-92] "But in this positedness each refers to itself..."

- c3. id: fnd-dif-b-003-c3
  - subject: reflected_difference
  - predicate: is_determined_as
  - object: diversity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [93-95] "Difference... is diversity."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-002
  - note: completes Absolute difference by making the two-moment form explicit.
  - sourceClaimIds: [`fnd-dif-b-003-c1`, `fnd-dif-b-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-dif-b-002-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-004
  - note: transition from Absolute difference (§3) to Diversity (§1).
  - sourceClaimIds: [`fnd-dif-b-003-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`fnd-dif-b-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: small entry retained as its own numbered movement with explicit claim.

### Entry fnd-dif-b-004 — Diversity arises from identity's self-differentiation

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 96
- lineEnd: 141

Summary:

Diversity first appears as identity's internal self-differentiation, producing indifferent diverse terms grounded in reflected self-identity.

Key points: (KeyPoint)

- k1. Diversity is generated from identity's self-differentiation.
- k2. Diverse terms subsist as indifferent while grounded in identity.
- k3. Difference here is external to the moments as moments.

Claims: (Claim)

- c1. id: fnd-dif-b-004-c1
  - subject: diversity
  - predicate: is_generated_from
  - object: identity_self_differentiation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [98-112] "Identity internally breaks apart into diversity..."

- c2. id: fnd-dif-b-004-c2
  - subject: diverse
  - predicate: subsists_as
  - object: indifferent_terms_grounded_in_identity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [109-114] "The different subsists as diverse... because identity constitutes its base and element..."

- c3. id: fnd-dif-b-004-c3
  - subject: moments_of_difference
  - predicate: are_related_as
  - object: externally_differentiated_indifferent_moments
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [125-141] "...the two are not differentiated within... the difference is external to them..."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-003
  - note: first expansion of two-moment reflected difference into Diversity.
  - sourceClaimIds: [`fnd-dif-b-004-c1`, `fnd-dif-b-004-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-dif-b-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-005
  - note: move from initial Diversity to external reflection structure.
  - sourceClaimIds: [`fnd-dif-b-004-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`fnd-dif-b-005-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: aligned to source numbering (Diversity §1).

### Entry fnd-dif-b-005 — Diversity develops external reflection as likeness/unlikeness

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 142
- lineEnd: 209

Summary:

In diversity, reflection externalizes itself; identity/difference appear as externally posited moments, and this yields likeness/unlikeness as comparative determinations.

Key points: (KeyPoint)

- k1. Diversity makes reflection externally determinate.
- k2. Identity and difference are treated as externally posited moments.
- k3. Likeness/unlikeness depend on external comparative standpoint.

Claims: (Claim)

- c1. id: fnd-dif-b-005-c1
  - subject: diversity
  - predicate: renders
  - object: reflection_external
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [142-151] "In diversity... reflection has in general become external..."

- c2. id: fnd-dif-b-005-c2
  - subject: reflection_moments_identity_difference
  - predicate: are_posited_as
  - object: externally_determined_moments
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [167-195] "...its two moments... are thus externally posited..."

- c3. id: fnd-dif-b-005-c3
  - subject: likeness_and_unlikeness
  - predicate: are
  - object: external_reflection_determinations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [196-209] "Now this external identity is likeness, and external difference is unlikeness..."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-004
  - note: specifies the mode of externalization latent in initial Diversity.
  - sourceClaimIds: [`fnd-dif-b-005-c1`, `fnd-dif-b-005-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-dif-b-004-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-006
  - note: move into oscillation and self-sublation of likeness/unlikeness.
  - sourceClaimIds: [`fnd-dif-b-005-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`fnd-dif-b-006-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: keep comparison language strictly source-anchored.

### Entry fnd-dif-b-006 — External comparison self-sublates into opposition

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 210
- lineEnd: 348

Summary:

External comparison between likeness and unlikeness undermines itself, producing a negative unity in which diversity passes over into opposition.

Key points: (KeyPoint)

- k1. External comparison oscillates between likeness and unlikeness.
- k2. The attempted separation of likeness/unlikeness destroys itself.
- k3. Negative unity of this movement yields opposition.

Claims: (Claim)

- c1. id: fnd-dif-b-006-c1
  - subject: external_reflection
  - predicate: oscillates_between
  - object: likeness_and_unlikeness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [210-231] "External reflection connects diversity... moves back and forth from likeness to unlikeness..."

- c2. id: fnd-dif-b-006-c2
  - subject: separated_likeness_unlikeness
  - predicate: self_sublates
  - object: through_internal_contradiction
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [249-283] "Because of this separation... is their destruction..."

- c3. id: fnd-dif-b-006-c3
  - subject: diversity
  - predicate: passes_over_into
  - object: opposition_through_negative_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [332-348] "The merely diverse thus passes over... Diversity... is opposition."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-005
  - note: develops external reflection into explicit self-sublating movement.
  - sourceClaimIds: [`fnd-dif-b-006-c1`, `fnd-dif-b-006-c2`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`fnd-dif-b-005-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-dif-b-007
  - note: negative unity of Diversity yields Opposition proper.
  - sourceClaimIds: [`fnd-dif-b-006-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`fnd-dif-b-007-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: relation expansion remains constrained until Ground pass.

### Entry fnd-dif-b-007 — Opposition completes difference as positive/negative in-and-for-itself

Span:

- sourceFile: `src/relative/essence/reflection/foundation/sources/difference.txt`
- lineStart: 349
- lineEnd: 589

Summary:

Opposition completes determinate reflection by constituting positive and negative as internally mediated, self-subsisting moments that are in and for themselves only through their exclusive relation.

Key points: (KeyPoint)

- k1. Opposition is the completed unity of identity and diversity.
- k2. Positive/negative are self-subsisting sides containing their other within.
- k3. Their determination is not merely external comparison but internal reflective opposition.
- k4. In-and-for-itself status includes, rather than cancels, opposed reference.

Claims: (Claim)

- c1. id: fnd-dif-b-007-c1
  - subject: opposition
  - predicate: is
  - object: unity_of_identity_and_diversity
  - modality: asserted
  - confidence: 0.98
  - evidence:
    - [349-354] "Opposition is the unity of identity and diversity..."

- c2. id: fnd-dif-b-007-c2
  - subject: positive_and_negative
  - predicate: are
  - object: self_subsisting_sides_of_opposition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [422-431] "The positive and the negative are thus the sides of opposition that have become self-subsisting..."

- c3. id: fnd-dif-b-007-c3
  - subject: positive_and_negative
  - predicate: are_in_and_for_themselves_as
  - object: exclusively_related_opposites
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [558-589] "not just in themselves, but in and for themselves... this reference, precisely as exclusive, constitutes their determination..."

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3
- c3 -> k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: fnd-dif-b-006
  - note: opposition is the explicit completion of diversity's negative unity.
  - sourceClaimIds: [`fnd-dif-b-007-c1`, `fnd-dif-b-007-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`fnd-dif-b-006-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: fnd-ctr-c-001
  - note: prepares direct transition into Contradiction where opposition becomes explicit contradiction.
  - sourceClaimIds: [`fnd-dif-b-007-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: pending_cross_workbook
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: contradiction handoff is provisional until Part C workbook is created.
