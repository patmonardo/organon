# Life Part A Workbook

Part: `A. THE LIVING INDIVIDUAL`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `living-individual.txt` as authority.
- Every accepted claim must have line-anchored evidence.
- If uncertain, mark `review_pending` and capture an open question.
- Span boundaries follow complete sentence groups and numeric section boundaries.

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

## Session: 2026-02-23 (first Life Part A pass)

Scope:

- file: `living-individual.txt`
- fixed range: lines `4-333`

Decision:

- Part A is segmented strictly by numeric labels `1`, `2`, `3` in the source.
- Claims remain minimal, line-anchored, and non-redundant for first pass.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

### Entry con-idea-life-a-001 — Section 1: immediate life as self-determining individual organism

Span:

- sourceFile: `src/compiler/concept/idea/life/living-individual.txt`
- lineStart: 4
- lineEnd: 162

Summary:

Section 1 determines universal life as immediate idea that particularizes itself into subject-object opposition and organizes objective corporeity as inward purposive organism.

Key points: (KeyPoint)

- k1. Universal life is immediate idea: concept-objectivity unity through negative self-reference.
- k2. Self-determination particularizes life into judgment and syllogistic structure.
- k3. Objectivity is only a posited moment of concept, not self-standing substance against subject.
- k4. Living individuality is soul-corporeity unity with inward purposiveness.
- k5. Organism is a membered whole whose moments mutually sublate and mediate one another.

Claims: (Claim)

- c1. id: con-idea-life-a-001-c1
  - subject: universal_life
  - predicate: is_determined_as
  - object: immediate_idea_as_negative_self_reference
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-26] immediate idea, negativity, self-determining diremption, and judgment-to-syllogism articulation.
    - [28-33] opposition determinations arise from concept itself.

- c2. id: con-idea-life-a-001-c2
  - subject: living_individual
  - predicate: has
  - object: inwardly_purposive_organismic_objectivity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [106-136] mechanism/chemism are inadequate; immanent concept defines inner purposiveness.
    - [137-162] organism as membered manifold with reciprocal self-sublating impulses.

Claim ↔ key point map:

- c1 -> k1, k2, k3
- c2 -> k4, k5

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: con-idea-life-a-002
  - note: from immediate organismic constitution to immanent self-process of individuality.
  - sourceClaimIds: [`con-idea-life-a-001-c1`, `con-idea-life-a-001-c2`]
  - sourceKeyPointIds: [`k2`, `k5`]
  - targetClaimIds: [`con-idea-life-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: first numeric block stabilized as one entry.

### Entry con-idea-life-a-002 — Section 2: self-process as productive negativity

Span:

- sourceFile: `src/compiler/concept/idea/life/living-individual.txt`
- lineStart: 163
- lineEnd: 206

Summary:

Section 2 presents the living process as a premise that is simultaneously conclusion, where externality is driven into self-sublation by concept.

Key points: (KeyPoint)

- k1. The living process with itself unites means-relation and achieved purposive unity.
- k2. Externality is not inert for life; it is the site where concept manifests as unrest and self-sublation.
- k3. Production in life is reproduction: product is producing factor.

Claims: (Claim)

- c1. id: con-idea-life-a-002-c1
  - subject: living_process
  - predicate: is_determined_as
  - object: premise_that_is_also_conclusion
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [163-185] process with itself repeats purposive premise while already realizing purpose.
    - [186-190] externality returns into negative unity of purpose.

- c2. id: con-idea-life-a-002-c2
  - subject: conceptual_productivity
  - predicate: culminates_in
  - object: reproduction_as_self_positing_production
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [191-199] mutability manifests concept and objectivity's self-sublation.
    - [200-205] product is producing factor through negative positing.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2, k3

Relations: (Relation)

- r1. type: refines
  - targetEntryId: con-idea-life-a-001
  - note: clarifies organismic purposiveness as active self-process.
  - sourceClaimIds: [`con-idea-life-a-002-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`con-idea-life-a-001-c2`]
  - logicalOperator: presuppositional_link
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: con-idea-life-a-003
  - note: passes from process-form determination to explicit conceptual moments of living reality.
  - sourceClaimIds: [`con-idea-life-a-002-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`con-idea-life-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 2 isolated as process-only hinge entry.

### Entry con-idea-life-a-003 — Section 3: conceptual moments of living corporeity and reproduction

Span:

- sourceFile: `src/compiler/concept/idea/life/living-individual.txt`
- lineStart: 207
- lineEnd: 333

Summary:

Section 3 articulates sensibility, irritability, and reproduction as differentiated yet total moments through which living individuality becomes explicit self-related totality oriented to external process.

Key points: (KeyPoint)

- k1. Living corporeity contains concept-determinations (universality, particularity, singularity) as essential differences.
- k2. Sensibility is inward universality that receives and reduces external determinacy to self-feeling.
- k3. Irritability is particularized negativity and outward-directed self-referential differentiation.
- k4. Reproduction is concrete unity of prior abstract moments and locus of feeling/resistance.
- k5. Reproduction actualizes individuality and opens transition to life-process against presupposed objectivity.

Claims: (Claim)

- c1. id: con-idea-life-a-003-c1
  - subject: living_objectivity
  - predicate: includes
  - object: conceptual_moments_as_essential_differences
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [223-231] corporeity includes universality-particularity-singularity differentiation.
    - [233-253] sensibility as inwardly reflected universality.

- c2. id: con-idea-life-a-003-c2
  - subject: reproduction
  - predicate: is_determined_as
  - object: concrete_totality_of_life_moments_and_actual_individuality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [290-311] immanent reflection yields reproduction as concrete totality beyond abstract moments.
    - [319-333] reproduction posits actual individuality and transition to objective life-process.

Claim ↔ key point map:

- c1 -> k1, k2, k3
- c2 -> k4, k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: con-idea-life-a-002
  - note: concreteness of reproduction confirms section 2's claim that production is self-reproductive negativity.
  - sourceClaimIds: [`con-idea-life-a-003-c2`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`con-idea-life-a-002-c2`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 3 spans sensibility-irritability-reproduction through transition marker.
