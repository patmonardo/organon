# Teleology Part B Workbook

Part: `B. THE MEANS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `means.txt` as authority.
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

## Session: 2026-02-22 (first Means pass)

Scope:

- file: `means.txt`
- fixed range: lines `4-180`
- segmentation basis: analytical blocks (`b-001..003`)

Decision:

- No fixed subspecies in text; use conceptual blocks for first pass.
- Keep IDs in normalized form: `obj-tel-b-001`, `obj-tel-b-002`, `obj-tel-b-003`.

### Entry obj-tel-b-001 — Block 1: means as first realization-beginning and finite mediation

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/means.txt`
- lineStart: 4
- lineEnd: 55

Summary:

The first block introduces means as the object determined in the first, still external negation of purpose: beginning of realization under finite split between supposing and presupposing.

Key points: (KeyPoint)

- k1. First positing of purpose includes presupposed objective world.
- k2. Means is not realized purpose but initial realization-step.
- k3. Finitude of purpose requires external middle term.
- k4. Purpose’s negation is immanent only one-sidedly and still outwardly reflected.

Claims: (Claim)

- c1. id: obj-tel-b-001-c1
  - subject: means
  - predicate: is_determined_as
  - object: first_objective_positedness_of_purpose_not_yet_realized_end
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [22-25] means identified as beginning of realization, not realized purpose.

- c2. id: obj-tel-b-001-c2
  - subject: finite_purpose
  - predicate: requires
  - object: external_middle_term_due_to_split_supposing_presupposing
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [30-35] finite purpose needs means/middle term with external existence.
    - [46-54] finitude as external determining split; negation only partly immanent.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-b-002
  - note: initial finitude of means transitions to formal syllogistic middle-term structure.
  - sourceClaimIds: [`obj-tel-b-001-c2`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`obj-tel-b-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: block segmentation used due to no explicit subspecies.

### Entry obj-tel-b-002 — Block 2: formal middle term and external linkage

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/means.txt`
- lineStart: 56
- lineEnd: 124

Summary:

The second block treats means as formal medius terminus: concept and objectivity are externally linked, so means is mechanical object carrying purpose as initially external determinateness.

Key points: (KeyPoint)

- k1. Means functions as replaceable formal middle term between extremes.
- k2. Means mediates only via external reference-form indifferent to itself.
- k3. Concept-objectivity linkage in means is external; means is mechanical object.
- k4. Purpose in means appears initially as external determinateness.

Claims: (Claim)

- c1. id: obj-tel-b-002-c1
  - subject: means
  - predicate: functions_as
  - object: formal_medius_terminus_of_formal_syllogism
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [56-77] means as formal, externally related middle term.

- c2. id: obj-tel-b-002-c2
  - subject: means
  - predicate: is_determined_as
  - object: mechanical_object_with_externally_linked_purpose
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [79-83] concept and objectivity only externally linked; means merely mechanical.
    - [97-103] purpose initially external determinateness in means.

- c3. id: obj-tel-b-002-c3
  - subject: middle_term
  - predicate: tends_toward
  - object: totality_of_purpose_through_immanent_reflection_activity
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [103-114] unifying middle must be totality; reflection implies activity.
    - [119-124] objectification splits moments into externalized apartness.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-b-003
  - note: formal-external means transitions to concept-penetrable object serving purposive activity.
  - sourceClaimIds: [`obj-tel-b-002-c2`, `obj-tel-b-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`obj-tel-b-003-c1`, `obj-tel-b-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second analytical block locked.

### Entry obj-tel-b-003 — Block 3: means as penetrable concept-totality and active premise

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/means.txt`
- lineStart: 125
- lineEnd: 180

Summary:

The final block presents means as concept-totality receptive to purpose, powerless before it, yet still retaining presuppositional externality that sustains purposive activity against what remains.

Key points: (KeyPoint)

- k1. Means as concept-totality is penetrable and receptive to purposive communication.
- k2. Through centrality/chemism background, means is non-self-subsistent and serves purpose.
- k3. Connection of purpose and immediately subjected object forms a premise.
- k4. Residual externality/presupposition keeps activity directed against what remains external.

Claims: (Claim)

- c1. id: obj-tel-b-003-c1
  - subject: means_object
  - predicate: is_determined_as
  - object: concept_identical_penetrable_and_serviceable_to_purpose
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [136-146] means as concept-totality penetrable/receptive to purpose.
    - [149-161] non-self-subsistence and service-character toward purpose.

- c2. id: obj-tel-b-003-c2
  - subject: purposive_activity_through_means
  - predicate: remains_directed_against
  - object: persisting_presupposition_of_external_objectivity
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [167-172] means still has side of self-subsistence and persisting presupposition.
    - [173-180] purpose as activity against this presupposition.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-c-001
  - note: persistence of presupposition transitions to realized-purpose section where externality must self-sublate.
  - sourceClaimIds: [`obj-tel-b-003-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`obj-tel-c-001-c1`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: boundary to realized purpose captured.
