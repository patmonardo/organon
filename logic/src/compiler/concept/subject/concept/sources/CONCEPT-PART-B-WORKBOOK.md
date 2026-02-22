# Concept Part B Workbook

Part: `B. PARTICULAR`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `particular.txt` as authority.
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

## Session: 2026-02-20 (first Particular pass)

Scope:

- file: `particular.txt`
- fixed range: lines `4-452`

Decision:

- Keep claim count minimal and non-redundant.
- Preserve cross-links to Part A/Part C only when claim anchors are stable.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

### Entry con-sub-b-001 — Immediate determination: particularity as immanent universality

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/particular.txt`
- lineStart: 4
- lineEnd: 95

Summary:

Particularity is first determined as the universal's own immanent moment: internally self-differentiating, self-related, and the sole true logical division of the concept.

Key points: (KeyPoint)

- k1. Conceptual determinateness is particularity, not an external limit.
- k2. The particular contains and exhibits universality as its substance and totality.
- k3. Diversity is not contingent multiplicity in truth but immanent opposition/connection.
- k4. The universal determines itself as particular and remains with itself in this difference.
- k5. True logical division yields coordinated opposing particulars grounded in one simple negativity.

Claims: (Claim)

- c1. id: con-sub-b-001-c1
  - subject: particularity
  - predicate: is_determined_as
  - object: universal_immanent_moment_nonexternal_limit
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [4-9] determinateness of concept is particularity; universal remains solely with itself.
    - [40-47] particularity as universality is immanent connection, intrinsic totality, essential principle.

- c2. id: con-sub-b-001-c2
  - subject: particular
  - predicate: contains_and_exhibits
  - object: universality_as_substance_totality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [11-26] particular contains universality and exhibits it through determinateness.
    - [52-71] universal determines itself as particular; principle of diversity is self-determined.

- c3. id: con-sub-b-001-c3
  - subject: true_logical_division
  - predicate: consists_in
  - object: concept_self_division_into_coordinated_particular_opposites
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [73-84] concept sets indeterminate universality and particular as coordinated particulars.
    - [89-95] opposing sides are one determinateness: simple negativity in the universal.

Claim ↔ key point map:

- c1 -> k1, k3
- c2 -> k2, k4
- c3 -> k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-sub-a-003
  - note: confirms the Universal conclusion that transition to particularity is concept-immanent self-differentiation.
  - sourceClaimIds: [`con-sub-b-001-c1`, `con-sub-b-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k4`]
  - targetClaimIds: [`con-sub-a-003-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-b-002
  - note: from immediate determination into mediated analysis of difference, abstraction, and understanding.
  - sourceClaimIds: [`con-sub-b-001-c3`]
  - sourceKeyPointIds: [`k5`]
  - targetClaimIds: [`con-sub-b-002-c1`, `con-sub-b-002-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

- r3. type: supports
  - targetEntryId: con-sub-a-004
  - note: reciprocal cross-part support: Particular's immediate determination confirms Universal's explicit handoff into particularity.
  - sourceClaimIds: [`con-sub-b-001-c1`, `con-sub-b-001-c2`]
  - sourceKeyPointIds: [`k1`, `k2`, `k4`]
  - targetClaimIds: [`con-sub-a-004-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: initial division logic stabilized for triadic pass.

### Entry con-sub-b-002 — Mediated determination: abstract universality and unconceptualized concept

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/particular.txt`
- lineStart: 97
- lineEnd: 291

Summary:

Particularity is mediated through conceptual difference and abstraction up to the limit where the determinate concept is exposed as one-sided unless grasped in conceptual totality.

Key points: (KeyPoint)

- k1. Difference reaches truth when grasped as conceptual unity/opposition.
- k2. Nature's manifold species do not reliably exhibit conceptual rigor.
- k3. Particular determinateness appears as abstract universality (form/content split).
- k4. Abstract universal still contains concept-moments but only in immediate, unposited unity.
- k5. The determinate concept remains one-sided and "empty" when its determinateness is not the principle of differentiation.

Claims: (Claim)

- c1. id: con-sub-b-002-c1
  - subject: conceptual_difference
  - predicate: is_truth_of
  - object: prior_relational_differences
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [97-110] difference in concept is truth; prior determinations attain truth by reaching concept.
    - [119-131] relations like whole/part and cause/effect are one determinate concept, not separate concepts.

- c2. id: con-sub-b-002-c2
  - subject: abstract_universal
  - predicate: is_determined_as
  - object: particularity_as_unposited_mediated_totality
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [171-199] particular determinateness as abstract universality; universality as form, determinateness as content.
    - [201-236] abstract-universal contains concept-moments but mediation is unposited; thus unconceptualized concept.

- c3. id: con-sub-b-002-c3
  - subject: natural_species_manifold
  - predicate: is
  - object: self_externality_not_trustworthy_copy_of_concept
  - modality: asserted
  - confidence: 0.9
  - evidence:
    - [138-145] multiplicity of natural species shows impotence to exhibit conceptual rigor.
    - [158-169] manifold forms are free self-externality and abstract side of nothingness.

- c4. id: con-sub-b-002-c4
  - subject: determinate_abstract_concept
  - predicate: is_determined_as
  - object: one_sided_if_not_principled_by_conceptual_totality
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [278-287] determinate concept is empty when it does not contain totality and principled differentiation.
    - [288-291] reproach of emptiness misses absolute determinateness as concept-difference content.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k3, k4
- c3 -> k2
- c4 -> k5

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-sub-b-001
  - note: explains how immediate particularity is mediated in abstraction without yet positing full conceptual totality.
  - sourceClaimIds: [`con-sub-b-002-c1`, `con-sub-b-002-c2`]
  - sourceKeyPointIds: [`k1`, `k3`, `k4`]
  - targetClaimIds: [`con-sub-b-001-c1`, `con-sub-b-001-c3`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-b-003
  - note: from one-sided abstract determinateness to explicit reason/understanding dialectic.
  - sourceClaimIds: [`con-sub-b-002-c2`, `con-sub-b-002-c4`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`con-sub-b-003-c1`, `con-sub-b-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: mediated block now runs through line 291 to include the one-sidedness/emptiness boundary.

### Entry con-sub-b-003 — High-level dialectic: understanding, reason, and the force of transition

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/particular.txt`
- lineStart: 293
- lineEnd: 426

Summary:

This block develops the high-level dialectic in which understanding and reason are non-separable: fixity and universality become the very medium of transition.

Key points: (KeyPoint)

- k1. Common usage of concept remains tied to understanding and abstraction.
- k2. Understanding fixes determinacies but also sharpens them to dialectical transition.
- k3. Separation of understanding and reason is rejected.
- k4. The determinate abstract concept is an essential moment of reason.
- k5. Reason appears through the internal kindling of finite determinacies in universal self-reference.

Claims: (Claim)

- c1. id: con-sub-b-003-c1
  - subject: understanding_fixity
  - predicate: both_limits_and_enables
  - object: dialectical_transition_of_determinacies
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [293-310] understanding grants fixed universality to finite determinacies.
    - [381-390] that same sharpening enables dissolution and transition into opposite.

- c2. id: con-sub-b-003-c2
  - subject: understanding_reason_relation
  - predicate: is
  - object: nonseparable_with_abstract_concept_as_moment_of_reason
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [414-420] separation of understanding and reason is rejected.
    - [421-426] determinate abstract concept is essential moment and beginning appearance of reason.

- c3. id: con-sub-b-003-c3
  - subject: reason
  - predicate: emerges_through
  - object: dialectical_kindling_of_finite_in_universal_form
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [324-330] infinite force of understanding as power causing transition of determinacies.
    - [397-413] self-referring universality exposes finite disproportionality and posits unity as concept.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4
- c3 -> k2, k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-sub-b-002
  - note: explains why abstract universality cannot remain final: its own form drives dialectical overcoming.
  - sourceClaimIds: [`con-sub-b-003-c1`, `con-sub-b-003-c2`]
  - sourceKeyPointIds: [`k2`, `k3`, `k4`]
  - targetClaimIds: [`con-sub-b-002-c2`, `con-sub-b-002-c4`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-b-004
  - note: high-level reason/understanding dialectic transitions to explicit singularity determination.
  - sourceClaimIds: [`con-sub-b-003-c2`, `con-sub-b-003-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`con-sub-b-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: high-level abstract discussion isolated as 293-426 before explicit transition block.

### Entry con-sub-b-004 — Transition: singularity as third moment and conceptual return

Span:

- sourceFile: `src/compiler/concept/subject/concept/sources/particular.txt`
- lineStart: 428
- lineEnd: 452

Summary:

The transition explicitly posits singularity as the third moment and as the concept's turning back into itself.

Key points: (KeyPoint)

- k1. Difference in determinate concept is brought to explicit due as essential moment.
- k2. Self-referring determinateness is singularity.
- k3. Particularity immediately is singularity, and singularity is the concept's absolute return.

Claims: (Claim)

- c1. id: con-sub-b-004-c1
  - subject: singularity
  - predicate: is_determined_as
  - object: third_moment_and_absolute_turning_back_of_concept
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [428-440] determinate universality as self-referring determinateness; self-reference is singularity.
    - [441-452] particularity immediately singularity; singularity as third moment and turning back of concept.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-sub-b-003
  - note: makes explicit the singularity determination implied by the reason/understanding dialectical block.
  - sourceClaimIds: [`con-sub-b-004-c1`]
  - sourceKeyPointIds: [`k1`, `k2`, `k3`]
  - targetClaimIds: [`con-sub-b-003-c2`]
  - logicalOperator: constrained_refinement
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-sub-c-001
  - note: explicit handoff from Particular (Part B) to Singularity (Part C).
  - sourceClaimIds: [`con-sub-b-004-c1`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-sub-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: transition block isolated as 428-452 per revised span strategy.
