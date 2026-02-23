# Teleology Part A Workbook

Part: `A. THE SUBJECTIVE PURPOSE`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `subjective.txt` as authority.
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

## Session: 2026-02-22 (first Subjective Purpose pass)

Scope:

- file: `subjective.txt`
- fixed range: lines `4-164`
- segmentation basis: analytical blocks (`a-001..003`)

Decision:

- No fixed subspecies in text; use conceptual blocks for first pass.
- Keep IDs in normalized form: `obj-tel-a-001`, `obj-tel-a-002`, `obj-tel-a-003`.

### Entry obj-tel-a-001 — Block 1: subjective purpose as self-repelling rational concept

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/subjective.txt`
- lineStart: 4
- lineEnd: 74

Summary:

The opening block defines purpose as subjective concept that self-repels into externalization while remaining self-identical, and distinguishes it from force/cause/substance by free self-mediation.

Key points: (KeyPoint)

- k1. Purpose arises from concept-determinacy bearing externality within unity.
- k2. Purpose is striving to posit itself externally without losing itself in transition.
- k3. Force/cause/substance analogies are inadequate unless transformed into self-causing self-expression.
- k4. Purpose as such is rational concrete existence, internally syllogistic.

Claims: (Claim)

- c1. id: obj-tel-a-001-c1
  - subject: purpose
  - predicate: is_determined_as
  - object: self_repelling_subjective_concept_striving_for_externalization
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [12-20] purpose defined as subjective concept and striving impulse to posit itself externally.

- c2. id: obj-tel-a-001-c2
  - subject: purpose
  - predicate: differs_from
  - object: ordinary_force_cause_substance_by_free_self_mediation
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [21-41] force/cause/substance transition-bound unless sublated as self-causing expression.

- c3. id: obj-tel-a-001-c3
  - subject: purpose_as_such
  - predicate: manifests
  - object: rationality_as_concrete_conceptual_unity_of_difference
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [46-51] purpose as rational concrete existence and essential syllogism.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-a-002
  - note: rational subjective structure transitions into finitude through objective indifference.
  - sourceClaimIds: [`obj-tel-a-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`obj-tel-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: block segmentation used due to lack of explicit subspecies.

### Entry obj-tel-a-002 — Block 2: finite purpose and objective presupposition

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/subjective.txt`
- lineStart: 75
- lineEnd: 120

Summary:

Purpose is finite because its determinateness appears as objective indifference and presupposed external world (mechanical/chemical whole) standing over against it.

Key points: (KeyPoint)

- k1. Purpose includes objective indifference as moment of its own difference.
- k2. Finitude appears as determinate content under infinite form.
- k3. Finitude also appears as presupposed external world opposed to purpose.

Claims: (Claim)

- c1. id: obj-tel-a-002-c1
  - subject: finite_purpose
  - predicate: is_determined_as
  - object: concept_with_externalized_determinateness_and_presupposition
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [75-90] purpose in objectivity: objective indifference enclosed in concept-unity.
    - [103-110] determinateness has form of presupposed objective world.

- c2. id: obj-tel-a-002-c2
  - subject: purposive_activity
  - predicate: stands_as
  - object: reflection_into_itself_and_reflection_outwards
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [111-119] self-determining activity is immediately external to itself and opposed by objective whole.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-a-003
  - note: finite opposition drives movement toward sublating presupposition and positing means.
  - sourceClaimIds: [`obj-tel-a-002-c1`, `obj-tel-a-002-c2`]
  - sourceKeyPointIds: [`k2`, `k3`]
  - targetClaimIds: [`obj-tel-a-003-c1`, `obj-tel-a-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: second analytical block locked.

### Entry obj-tel-a-003 — Block 3: realization movement and first positing of means

Span:

- sourceFile: `src/compiler/concept/object/teleology/souces/subjective.txt`
- lineStart: 121
- lineEnd: 164

Summary:

The final block defines purposive movement as dual sublation of object and subjectivity, where self-determination immediately presupposes external objectivity and first posits means.

Key points: (KeyPoint)

- k1. Purpose moves to sublate presupposed object-immediacy and determine it conceptually.
- k2. This negative relation to object is also sublation of subjective one-sidedness.
- k3. Resolution both determines content internally and presupposes external objectivity.
- k4. First practical determination is object posited as means.

Claims: (Claim)

- c1. id: obj-tel-a-003-c1
  - subject: movement_of_purpose
  - predicate: aims_at
  - object: sublation_of_presupposition_and_realization_as_unification
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [121-134] movement sublates object-immediacy and unifies objective being with concept.

- c2. id: obj-tel-a-003-c2
  - subject: purposive_resolution
  - predicate: posits
  - object: simultaneous_inner_determination_and_external_presupposition
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [150-163] determination as content and simultaneous presupposing of indifferent objectivity.
    - [164-164] first explicit practical outcome: object posited as means.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: obj-tel-b-001
  - note: first positing of means opens Part B treatment of the middle term.
  - sourceClaimIds: [`obj-tel-a-003-c2`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`obj-tel-b-001-c1`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: boundary to means explicitly captured.
