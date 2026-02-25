# Chemism Part B Workbook

Part: `B. THE PROCESS`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `process.txt` as authority.
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

## Session: 2026-02-22 (numeric-subentry pass)

Scope:

- file: `process.txt`
- fixed range: lines `4-179`
- segmentation basis: numeric subentries `1|2|3`

Decision:

- Use numeric section boundaries as primary entry boundaries.
- Keep IDs in normalized form: `chemi-pro-001`, `chemi-pro-002`, `chemi-pro-003`.

### Entry chemi-pro-001 — Section 1: affinity, communication, and formal neutralization

Span:

- sourceFile: `src/compiler/concept/object/chemism/process.txt`
- lineStart: 4
- lineEnd: 87

Summary:

The first section presents chemically tensed objects as reciprocal affinity that seeks concept-conformable unity through communication, yielding a neutral but still formal product.

Key points: (KeyPoint)

- k1. Tension is reciprocal and grounded in each object's contradiction with one-sided existence.
- k2. Communication medium serves as formal neutrality enabling external community.
- k3. Union blunts opposition into tranquil neutrality.
- k4. Resulting neutrality is formal and retains latent capacity for renewed tension.

Claims: (Claim)

- c1. id: chemi-pro-001-c1
  - subject: chemically_tensed_objects
  - predicate: strive_toward
  - object: reciprocal_complementation_and_concept_conformable_reality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-17] reciprocal striving to sublate one-sidedness and combine into concept-conformable reality.

- c2. id: chemi-pro-001-c2
  - subject: communication_medium
  - predicate: functions_as
  - object: formal_neutral_middle_term
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [24-39] middle term as abstract neutrality/theoretical element.
    - [40-45] examples: water (bodies), sign/language (spirit analogy).

- c3. id: chemi-pro-001-c3
  - subject: neutral_product
  - predicate: is_determined_as
  - object: formal_unity_with_retained_tension_capacity
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [69-77] product neutralizes ingredients while retaining capacity for prior tension.
    - [81-87] non-indifference only immediately sublated; product remains formal unity.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: chemi-pro-002
  - note: formal neutrality externalizes negativity and requires reactivation.
  - sourceClaimIds: [`chemi-pro-001-c3`]
  - sourceKeyPointIds: [`k4`]
  - targetClaimIds: [`chemi-pro-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subentry boundary preserved.

### Entry chemi-pro-002 — Section 2: externalized negativity and disjunctive totality

Span:

- sourceFile: `src/compiler/concept/object/chemism/process.txt`
- lineStart: 88
- lineEnd: 155

Summary:

The second section shows negativity stepping outside neutral product, disrupting it into abstract moments and exhibiting chemism as a disjunctive totality of unity, activity, and resolved moments.

Key points: (KeyPoint)

- k1. Process does not auto-restart because non-indifference was presupposed, not posited.
- k2. External singular negativity disrupts neutral object and re-determines mediation.
- k3. Real neutrality breaks into indifferent base and activating principle.
- k4. Disjunctive syllogism displays chemism's total structure.

Claims: (Claim)

- c1. id: chemi-pro-002-c1
  - subject: externalized_negativity
  - predicate: reactivates
  - object: chemical_process_by_disrupting_neutrality
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [88-109] negativity remains outside neutral product and reconnects as restless consuming activity.
    - [110-123] immediate connection differentiates/disrupts middle term.

- c2. id: chemi-pro-002-c2
  - subject: disruption_of_real_neutrality
  - predicate: yields
  - object: abstract_indifferent_base_and_activating_principle
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [129-143] opposite extreme and breakup into neutral moments/base + activating principle.

- c3. id: chemi-pro-002-c3
  - subject: disjunctive_syllogism
  - predicate: is_determined_as
  - object: totality_of_chemism
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [144-155] whole shown as negative unity, real unity, and resolved abstract moments.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: chemi-pro-003
  - note: resolved abstract moments become elemental presupposition for renewed cycle and sublation of chemism.
  - sourceClaimIds: [`chemi-pro-002-c2`, `chemi-pro-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`chemi-pro-003-c1`, `chemi-pro-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subentry boundary preserved.

### Entry chemi-pro-003 — Section 3: elemental return and transition beyond chemism

Span:

- sourceFile: `src/compiler/concept/object/chemism/process.txt`
- lineStart: 156
- lineEnd: 179

Summary:

The third section re-posits elemental objects as basis of renewed reciprocal tension while simultaneously showing chemism returning to concept and passing into a higher sphere.

Key points: (KeyPoint)

- k1. Elemental objects are liberated from current tension and posited as beginning-presupposition.
- k2. Inner contradiction of elemental determinateness re-generates outward tension and neutralization.
- k3. Chemism both cycles back and sublates itself into a higher domain.

Claims: (Claim)

- c1. id: chemi-pro-003-c1
  - subject: elemental_objects
  - predicate: are_determined_as
  - object: liberated_basis_for_renewed_chemical_tension
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [156-170] elemental objects posited and driven to relate/non-indifferent neutralization.

- c2. id: chemi-pro-003-c2
  - subject: chemism
  - predicate: both_restores_and_sublates
  - object: its_beginning_and_itself_into_higher_sphere
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [171-179] return to beginning process and explicit transition to higher sphere.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: chemi-chm-001
  - note: explicit beyond-chemism transition is developed in Part C transition treatment.
  - sourceClaimIds: [`chemi-pro-003-c2`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`chemi-chm-001-c1`]
  - logicalOperator: boundary_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: numeric subentry boundary preserved.
