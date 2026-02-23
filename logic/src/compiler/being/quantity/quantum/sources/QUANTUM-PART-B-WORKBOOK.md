# Quantum Part B Workbook

Part: `B. EXTENSIVE AND INTENSIVE QUANTUM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/quantity/quantum/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `quantum.txt` as authority.
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

## Session: 2026-02-22 (seed pass)

Scope:

- file: `quantum.txt`
- fixed range: lines `1-304`
- pass policy: preserve source `a/b/c`, preserve numeric `1/2/3` under `a`, and build analytic ordinary-entry triads where numeric subdivision is absent

Decision:

- ID grammar:
  - `qtm-b-a-001..003` (source numeric moments under `a`)
  - `qtm-b-b-001..003` (analytic triad for subspecies `b`)
  - `qtm-b-c-001..003` (analytic triad for subspecies `c`)
- One claim per ordinary entry in this seed pass.

### Entry qtm-b-a-001 — Difference (a1): extensive magnitude as limit-in-amount

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 6
- lineEnd: 54

Summary:

Quantum with a limit as plurality is extensive magnitude, distinguished from but related to continuity/discreteness.

Key points: (KeyPoint)

- k1. Quantum’s limit in amount gives extensive magnitude.
- k2. Extensive/intensive differ from continuous/discrete determination.
- k3. Extensive is amount of one and same unit.

Claims: (Claim)

- c1. id: qtm-b-a-001-c1
  - subject: extensive_quantum
  - predicate: is
  - object: limit_as_plurality_amount_of_same_unit
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [6-13] quantum with limit as plurality is extensive magnitude.
    - [49-54] extensive quantum as amount of one and same unit.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-a-002
  - note: extensive limit passes to simple limit as degree.
  - sourceClaimIds: [`qtm-b-a-001-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`qtm-b-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: source numeric subsection 1.

### Entry qtm-b-a-002 — Difference (a2): degree as intensive magnitude

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 56
- lineEnd: 117

Summary:

The self-external number-limit collapses into simple determinateness, yielding intensive magnitude as degree.

Key points: (KeyPoint)

- k1. External plurality collapses into simple unity.
- k2. Intensive magnitude is degree.
- k3. Degree carries number as sublated amount.

Claims: (Claim)

- c1. id: qtm-b-a-002-c1
  - subject: intensive_quantum
  - predicate: is_determined_as
  - object: degree_simple_limit_with_sublated_amount
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [88-96] limit passes into simple determinateness as degree.
    - [103-117] number in degree is not aggregate amount but sublated amount.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-a-003
  - note: degree’s simplicity externalizes determinateness and continuous scale reference.
  - sourceClaimIds: [`qtm-b-a-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`qtm-b-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: source numeric subsection 2.

### Entry qtm-b-a-003 — Difference (a3): degree as self-reference through externality

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 119
- lineEnd: 170

Summary:

Degree is simple self-reference that nevertheless has its determinateness in external continuity with other degrees.

Key points: (KeyPoint)

- k1. Degree excludes external amount within itself.
- k2. Degree has determinateness via external plurality.
- k3. Scale alteration is continuous and indivisible.

Claims: (Claim)

- c1. id: qtm-b-a-003-c1
  - subject: degree
  - predicate: has_quality_as
  - object: self_reference_through_external_continuity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [133-145] degree refers to external plurality as its determinateness.
    - [156-170] continuous scale progression constitutes degree’s qualitative reference.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-b-001
  - note: relation of degree to amount opens identity of extensive and intensive magnitude.
  - sourceClaimIds: [`qtm-b-a-003-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`qtm-b-b-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: source numeric subsection 3.

### Entry qtm-b-b-001 — Identity (b1): degree implicitly contains amount

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 172
- lineEnd: 197

Summary:

Although intensive magnitude is simple, its determinateness includes amount as continuous plurality.

Key points: (KeyPoint)

- k1. Degree is simple one of many more/less.
- k2. Determinateness lies in connection with many.
- k3. Degree thus contains amount implicitly.

Claims: (Claim)

- c1. id: qtm-b-b-001-c1
  - subject: degree
  - predicate: contains
  - object: amount_as_continuous_plurality
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [187-197] degree’s determinateness is via many; it contains amount through that mediation.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-b-002
  - note: containment of amount becomes explicit dual-sided determination.
  - sourceClaimIds: [`qtm-b-b-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-b-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `b`.

### Entry qtm-b-b-002 — Identity (b2): intensive determinateness from exclusion and internal amount

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 199
- lineEnd: 220

Summary:

Intensive magnitude is determined both by exclusion of other degrees and by internal amount as its own determinateness.

Key points: (KeyPoint)

- k1. Degree excludes other degrees.
- k2. Degree is determined within by its amount.
- k3. Twentieth degree is determined by twenty within it.

Claims: (Claim)

- c1. id: qtm-b-b-002-c1
  - subject: intensive_magnitude
  - predicate: is_determined_by
  - object: exclusion_and_internal_amount
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [206-217] exclusion and internal amount jointly determine degree.
    - [218-220] degree is essentially extensive through amount.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-b-003
  - note: two-sided determination resolves as identity of extensive and intensive magnitude.
  - sourceClaimIds: [`qtm-b-b-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`qtm-b-b-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `b`.

### Entry qtm-b-b-003 — Identity (b3): extensive and intensive as one determinateness

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 222
- lineEnd: 254

Summary:

Extensive and intensive magnitude are one determinateness of quantum, distinguished only by amount being internal or external.

Key points: (KeyPoint)

- k1. Extensive and intensive are one determination.
- k2. Distinction is internal/external amount.
- k3. Their unity yields qualitative something as indifferent substrate.

Claims: (Claim)

- c1. id: qtm-b-b-003-c1
  - subject: extensive_and_intensive_magnitude
  - predicate: are_identical_as
  - object: one_quantitative_determinateness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [222-234] explicit identity claim.
    - [236-254] identity returns into unity as qualitative something.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-c-001
  - note: identity now unfolds as alteration of quantum.
  - sourceClaimIds: [`qtm-b-b-003-c1`]
  - sourceKeyPointIds: [`k1`]
  - targetClaimIds: [`qtm-b-c-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `b`.

### Entry qtm-b-c-001 — Alteration (c1): contradiction of quantum’s self-reference and external determinateness

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 256
- lineEnd: 272

Summary:

Quantum’s determinateness is contradictory: simple self-reference whose determinateness lies in another quantum.

Key points: (KeyPoint)

- k1. Quantum is sublated determinateness.
- k2. Its limit negates itself.
- k3. Determinateness lies in another quantum.

Claims: (Claim)

- c1. id: qtm-b-c-001-c1
  - subject: quantum
  - predicate: is_contradictory_as
  - object: self_reference_with_external_determinateness
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [258-272] explicit contradiction of self-reference and external determination.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-c-002
  - note: contradiction is expressed as necessary alteration.
  - sourceClaimIds: [`qtm-b-c-001-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-b-c-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `c`.

### Entry qtm-b-c-002 — Alteration (c2): quantum must alter by continuity into otherness

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 273
- lineEnd: 281

Summary:

Quantum not only can alter but must alter, since its being lies in continuity with an other.

Key points: (KeyPoint)

- k1. Every quantitative determinateness is transcendable.
- k2. Alteration is necessary, not optional.
- k3. Limit is becoming.

Claims: (Claim)

- c1. id: qtm-b-c-002-c1
  - subject: quantum
  - predicate: necessarily_alters_into
  - object: otherness_by_continuity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [275-281] necessity of alteration and becoming-limit.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-b-c-003
  - note: necessary alteration appears as increase/decrease and endless beyond.
  - sourceClaimIds: [`qtm-b-c-002-c1`]
  - sourceKeyPointIds: [`k2`]
  - targetClaimIds: [`qtm-b-c-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `c`.

### Entry qtm-b-c-003 — Alteration (c3): increase/decrease and self-sublating limit to infinity

Span:

- sourceFile: `src/compiler/being/quantity/quantum/sources/quantum.txt`
- lineStart: 283
- lineEnd: 304

Summary:

Quantum sends itself beyond itself as increase/decrease, producing limits that repeatedly sublate into infinity.

Key points: (KeyPoint)

- k1. Quantum repels itself from itself as determinate one.
- k2. Increase/decrease is internal externality of determinateness.
- k3. New limit continually sublates into further limit.

Claims: (Claim)

- c1. id: qtm-b-c-003-c1
  - subject: quantum_limit
  - predicate: unfolds_as
  - object: endless_self_sublating_beyond
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [293-304] quantum impels itself beyond each new limit to infinity.

Claim ↔ key point map:

- c1 -> k1, k2, k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: qtm-c-a-001
  - note: alteration of quantum passes into quantitative infinity.
  - sourceClaimIds: [`qtm-b-c-003-c1`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`qtm-c-a-001-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: analytic ordinary entry under subspecies `c`.
