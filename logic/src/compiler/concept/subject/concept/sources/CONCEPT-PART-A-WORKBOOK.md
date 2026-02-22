# Concept Part A Workbook

Part: `A. UNIVERSAL`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `universal.txt` as authority.
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

## Session: 2026-02-20 (first Universal pass)

Scope:

- file: `universal.txt`
- fixed range: lines `4-307`

Decision:

- Start with minimal non-redundant claims.
- Keep relation semantics conservative until cross-part links are stable.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

### Entry con-sub-a-001 — Immediate determination: universality as absolute negativity

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/universal.txt`
- lineStart: 4
- lineEnd: 92

Summary:

The opening determination presents the concept as self-originating universality: negation of negation, self-reference, and non-external mediation.

Key points: (KeyPoint)

- k1. The pure concept is introduced as unconditioned, free, and generated through the return movement from being/essence.
- k2. Universality is self-reference produced through negativity.
- k3. The universal is simple yet internally determinate and richer than immediate being.
- k4. Even abstract universality already contains negation of negation.
- k5. The universal here is not externally determined but remains absolute negativity in itself.

Claims: (Claim)

- c1. id: con-sub-a-001-c1
  - subject: pure_concept
  - predicate: is_determined_as
  - object: unconditioned_free_self_originating_universal
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-15] pure concept as absolutely infinite/unconditioned/free and as outcome of genesis from being/essence.
    - [23-29] concept as mutual penetration and immanent turning-back into self-referring determinateness.

- c2. id: con-sub-a-001-c2
  - subject: universality
  - predicate: is
  - object: self_reference_through_negation_of_negation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [31-36] concept as negation of negation; universality as pure self-reference through negativity.
    - [77-92] negation of negation in abstraction, yet universal remains absolute negativity in itself.

- c3. id: con-sub-a-001-c3
  - subject: universal
  - predicate: contains
  - object: internal_determinateness_without_externality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [42-46] universal simplicity contains difference/determinateness in highest degree.
    - [60-64] universal as simple self-reference and absolute mediation (not externally mediated).

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k4, k5
- c3 -> k3, k5

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-sub-a-002
  - note: from immediate universality to universality as immanent substance of determinations.
  - sourceClaimIds: [`con-sub-a-001-c2`, `con-sub-a-001-c3`]
  - sourceKeyPointIds: [`k2`, `k3`, `k5`]
  - targetClaimIds: [`con-sub-a-002-c1`, `con-sub-a-002-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: opening block consolidated as immediate determination for three-stage analysis.

### Entry con-sub-a-002 — Mediated determination: universal as immanent self-preserving substance

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/universal.txt`
- lineStart: 94
- lineEnd: 156

Summary:

The universal is shown as positively self-identical in determination, with concept-internal positedness and reflective shine as appearance of the identical.

Key points: (KeyPoint)

- k1. Determination is not an external restriction of the universal.
- k2. The universal remains itself in manifold determinations and persists through becoming.
- k3. Determinateness in concept is positedness/negative-of-negative, not external shine.
- k4. Mediation is concept-internal and creative, not an external constraining necessity.

Claims: (Claim)

- c1. id: con-sub-a-002-c1
  - subject: universal_in_determination
  - predicate: remains
  - object: positively_self_identical
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [94-98] first negative is not restriction; universal maintains itself positively.
    - [108-117] universal remains what it is, inhabits concrete manifoldness, persists undisturbed.

- c2. id: con-sub-a-002-c2
  - subject: concept_determinateness
  - predicate: is_determined_as
  - object: immanent_positedness_negative_of_negative
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [129-137] determination is concept-internal positedness and negative of the negative.
    - [151-156] determination as sublated limitation; reflective shine as appearance of identical.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-sub-a-001
  - note: mediating development confirms that initial universality is intrinsically self-referential, not externally mediated.
  - sourceClaimIds: [`con-sub-a-002-c1`, `con-sub-a-002-c2`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`con-sub-a-001-c2`, `con-sub-a-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-a-003
  - note: from immanent self-preserving universality to explicit articulation of free power and concrete totality.
  - sourceClaimIds: [`con-sub-a-002-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`con-sub-a-003-c1`, `con-sub-a-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: this entry now ends at line 156 to isolate the pre-free-power mediation block.

### Entry con-sub-a-003 — Middle development: free power, concrete totality, and higher inward universality

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/universal.txt`
- lineStart: 158
- lineEnd: 278

Summary:

The universal is developed as free power and concrete totality, then elevated through doubly reflective mediation into higher inward universality.

Key points: (KeyPoint)

- k1. The universal is free power in its relation-to-other and remains at rest in the other as in itself.
- k2. The universal contains determinateness as particularity and singularity and is concrete totality.
- k3. Determinateness is doubly reflective (outward and inward), preserving universality in differentiation.
- k4. Lower genus resolves in higher inward universality.
- k5. True higher universality is concrete (life, I, spirit, absolute concept), not merely abstract genus.

Claims: (Claim)

- c1. id: con-sub-a-003-c1
  - subject: universal
  - predicate: is_determined_as
  - object: free_power_and_self_returning_relation_to_other
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [158-169] universal as free power/free love, relating to distinct other as to itself.
    - [174-191] universal contains determinateness as particularity/singularity in and for itself.

- c2. id: con-sub-a-003-c2
  - subject: universal
  - predicate: is_determined_as
  - object: concrete_totality_and_doubly_reflective_self_mediation
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [192-205] universal as concrete totality; abstract universal as isolated, truth-deficient moment.
    - [209-247] doubly reflective determinateness and inward shining preserve universality.

- c3. id: con-sub-a-003-c3
  - subject: true_higher_universal
  - predicate: is_determined_as
  - object: inwardly_redirected_concrete_universality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [249-263] lower genus resolves in higher universal through inward redirection.
    - [264-278] life/I/spirit/absolute concept as concrete universals with self-contained determinacies.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2, k3
- c3 -> k4, k5

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-sub-a-002
  - note: develops the mediated block into explicit free-power and reflective-totality determinations.
  - sourceClaimIds: [`con-sub-a-003-c1`, `con-sub-a-003-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-sub-a-002-c1`, `con-sub-a-002-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-a-004
  - note: higher inward universality transitions to explicit infinite-universal determination and handoff sentence.
  - sourceClaimIds: [`con-sub-a-003-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`con-sub-a-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: middle block now isolates lines 158-278 per revised span strategy.

### Entry con-sub-a-004 — Final handoff: true infinite universal proceeds to particularity

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/universal.txt`
- lineStart: 280
- lineEnd: 307

Summary:

The final lines explicitly hand off from universal to particularity through free self-determination as creative self-referring negativity.

Key points: (KeyPoint)

- k1. The true infinite universal is immediately particularity and singularity.
- k2. Its becoming finite is creative self-referring negativity, not mere being-transition.
- k3. It posits universal, self-referring differences as fixed, isolated determinations.

Claims: (Claim)

- c1. id: con-sub-a-004-c1
  - subject: infinite_universal
  - predicate: freely_determines_itself_as
  - object: particularity_through_creative_self_differentiation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [280-283] true infinite universal immediately includes particularity and singularity; proceeds to particularity.
    - [284-290] becoming finite as creative self-referring negativity, not being-transition.

- c2. id: con-sub-a-004-c2
  - subject: universal_creativity
  - predicate: posits
  - object: self_referring_universal_differences_as_isolated_determinations
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [291-297] internal differentiation as determining; positing of self-referring universal differences.
    - [298-307] finite isolation resolved as universality and creativity of concept.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k2, k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-sub-a-003
  - note: makes explicit the final transition sentence implied by the higher-universal development.
  - sourceClaimIds: [`con-sub-a-004-c1`, `con-sub-a-004-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-sub-a-003-c3`]
  - logicalOperator: constrained_refinement
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-b-001
  - note: explicit handoff from Universal (Part A) to Particular (Part B).
  - sourceClaimIds: [`con-sub-a-004-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [pending]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: final handoff block isolated as 280-307 per revised span policy.
