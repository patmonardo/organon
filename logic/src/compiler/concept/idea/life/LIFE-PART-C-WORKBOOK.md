# Life Part C Workbook

Part: `C. THE GENUS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `genus.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow coherent analysis windows of complete sentence groups.

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

## Session: 2026-02-23 (first Life Part C pass)

Scope:

- file: `genus.txt`
- fixed range: lines `4-157`

Decision:

- Part C is sectioned by analysis windows, not by numeric/letter subentries.
- Keep first pass conservative with minimal, line-anchored claims.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

### Entry con-idea-life-c-001 — Analysis window 1: from presupposed individual to produced actuality

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 4
- lineEnd: 37

Summary:

The individual that was merely presupposed becomes self-grounding actuality and, through sublated opposition, is determined as genus via self-particularization.

Key points: (KeyPoint)

- k1. The individual initially lacks self-proof and stands as presupposition.
- k2. Through process with world, it posits itself as foundation and actuality of idea.
- k3. Genus emerges as identity with otherness and duplication into living self-relation.

Claims: (Claim)

- c1. id: con-idea-life-c-001-c1
  - subject: living_individual
  - predicate: transitions_from_to
  - object: presupposed_unproven_to_self_produced_actuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [4-19] presupposition becomes production through process.

- c2. id: con-idea-life-c-001-c2
  - subject: genus
  - predicate: is_determined_as
  - object: identity_with_otherness_and_duplicated_self_relation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [21-29] genus as identity of individual with indifferent otherness.
    - [30-37] particularization as duplication and relation to another living being.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-life-c-002
  - note: from produced genus identity to explicit contradiction of reciprocal individuality.
  - sourceClaimIds: [`con-idea-life-c-001-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: opening window stabilizes production-to-genus transition.

### Entry con-idea-life-c-002 — Analysis window 2: contradiction of genus in immediate reciprocity

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 38
- lineEnd: 71

Summary:

Genus is truth of enclosed life yet remains immediate and singularized, so genus relation appears as contradiction and renewed impulse.

Key points: (KeyPoint)

- k1. Genus is third stage where externality is immanent moment of self-reference.
- k2. Identity with another self-subsistent individual is contradictory for immediate life.
- k3. The individual is genus only in-itself; for-itself it confronts another living individual.

Claims: (Claim)

- c1. id: con-idea-life-c-002-c1
  - subject: genus_relation
  - predicate: is_determined_as
  - object: contradictory_identity_of_reciprocal_individuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [38-50] third stage of self-referential process with subsisting externality.
    - [52-57] contradiction and renewed impulse.

- c2. id: con-idea-life-c-002-c2
  - subject: immediate_genus
  - predicate: remains
  - object: singularized_objective_reciprocity_not_yet_concept_for_itself
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [58-64] universality still immediate in singular shape.
    - [65-71] object is another living concept with reciprocal externality.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-life-c-001
  - note: clarifies limits of first achieved genus identity.
  - sourceClaimIds: [`con-idea-life-c-002-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-c-001-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-c-003
  - note: contradiction advances into longing, sublation of singularities, and germ determination.
  - sourceClaimIds: [`con-idea-life-c-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`con-idea-life-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: this window isolates immediacy/contradiction of genus relation.

### Entry con-idea-life-c-003 — Analysis window 3: longing, sublation, and germ as concept-actuality

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 72
- lineEnd: 126

Summary:

Subjective universality longs for realization through sublation of singular individuals, yielding the germ where concept and external actuality are visibly united.

Key points: (KeyPoint)

- k1. Inner universality seeks explicit realization by sublating isolated singularities.
- k2. Realized genus identity is negative unity reflecting into itself.
- k3. The germ is actual concept: undeveloped yet complete inner concretion of individuality.

Claims: (Claim)

- c1. id: con-idea-life-c-003-c1
  - subject: impulse_of_genus
  - predicate: realizes_itself_through
  - object: sublation_of_singular_individualities
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [72-86] longing to posit universality and realized identity as negative unity.
    - [87-93] individuality of life appears as concept that objectifies itself.

- c2. id: con-idea-life-c-003-c2
  - subject: germ
  - predicate: is_determined_as
  - object: complete_inner_concretion_of_living_individuality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [94-105] germ displays subjective concept with external actuality.
    - [97-103] full determinateness contained in simple non-sensuous inwardness.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-c-002
  - note: resolves immediate contradiction by specifying the generative mechanism of genus realization.
  - sourceClaimIds: [`con-idea-life-c-003-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-life-c-002-c1`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-c-004
  - note: from germ determination to realized genus universality and cognition handoff.
  - sourceClaimIds: [`con-idea-life-c-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`con-idea-life-c-004-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: this window treats germ as explicit concept-actuality unity.

### Entry con-idea-life-c-004 — Analysis window 4: propagation, finite repetition, and transition to cognition

Span:

- sourceFile: `src/compiler/concept/idea/life/genus.txt`
- lineStart: 127
- lineEnd: 157

Summary:

Genus realizes itself through propagation and reciprocal perishing of singularity, sublates immediacy, and becomes the explicit universal self-relation that transitions to cognition.

Key points: (KeyPoint)

- k1. Propagation posits individuality within genus through reflection into self.
- k2. The genus-process both repeats finite immediacy and sublates it into a higher form.
- k3. Death of immediate individuality is spirit's coming-to-be and the rise of the idea of cognition.

Claims: (Claim)

- c1. id: con-idea-life-c-004-c1
  - subject: process_of_genus
  - predicate: culminates_in
  - object: explicit_universality_of_idea
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [127-143] reciprocal sublation of singulars and genus rejoining itself.
    - [148-156] idea becomes explicit universal self-relation.

- c2. id: con-idea-life-c-004-c2
  - subject: death_of_immediate_life
  - predicate: is_determined_as
  - object: transition_to_spirit_and_cognition
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [144-147] perishing of living individuality is coming-to-be of spirit.
    - [157-157] explicit handoff: idea of cognition.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-c-003
  - note: actual propagation and universality fulfills the germ's conceptual potential.
  - sourceClaimIds: [`con-idea-life-c-004-c1`]
  - sourceKeyPointIds: [`k1`, `k2`]
  - targetClaimIds: [`con-idea-life-c-003-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: closing window preserves explicit transition marker to cognition.
