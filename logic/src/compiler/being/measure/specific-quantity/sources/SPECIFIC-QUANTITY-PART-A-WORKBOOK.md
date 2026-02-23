# Specific Quantity Part A Workbook

Part: `A. THE SPECIFIC QUANTUM`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/being/measure/specific-quantity/sources/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `specific-quantum.txt` as authority.
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

## Session: 2026-02-23 (initial scaffold)

Scope:

- file: `specific-quantum.txt`
- fixed range: lines `1-190`
- pass policy: establish first-order entry decomposition for sections `1-3` with stable IDs and minimal non-redundant claims.

Decision:

- Use ID prefix `spq-a` for Part A entries.
- Part A has no subentries.
- Section boundaries are the numerical labels `1`, `2`, `3` in `specific-quantum.txt`.
- Keep first pass source-restricted and line-anchored.
- Defer cross-part inferential expansions until Part B and Part C first-order entries are stabilized.

## Decomposition lock

- Entry model: one entry per numbered section.
- Planned entries:
  - `spq-a-001` -> section `1`
  - `spq-a-002` -> section `2`
  - `spq-a-003` -> section `3`

### Entry spq-a-001 — Specific quantum I: measure as qualitative quantum

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/sources/specific-quantum.txt`
- lineStart: 4
- lineEnd: 110

Summary:

Measure is first determined as the self-referential qualitative determination of quantum, where magnitude is constitutive of what something is.

Key points: (KeyPoint)

- k1. Quantum as measure is no longer an indifferent limit but a qualitative determinateness.
- k2. Every existent has a measure; alteration of constitutive magnitude alters quality.
- k3. External standards are conventional comparators and not intrinsic natural measures of things.
- k4. Mere gradual quantitative change does not explain qualitative transition, which includes passage into non-existence.

Claims: (Claim)

- c1. id: spq-a-001-c1
  - subject: measure
  - predicate: determines
  - object: quantum_as_qualitative
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [4-18] measure is self-reference of quantum and quantum is qualitative, not indifferent.

- c2. id: spq-a-001-c2
  - subject: something_with_measure
  - predicate: has_quality_dependent_on
  - object: constitutive_magnitude
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [19-33] the magnitude belongs to the nature of the something; changing it changes quality.

- c3. id: spq-a-001-c3
  - subject: external_standard_and_gradualness_category
  - predicate: are_insufficient_as
  - object: intrinsic_measure_or_exhaustive_explanation_of_qualitative_transition
  - modality: asserted
  - confidence: 0.92
  - evidence:
    - [34-72] universal standards are externally selected and conventional, not in-themselves natural measures.
    - [73-110] gradualness alone does not explain transition to another quality or non-existence.

Claim ↔ key point map:

- spq-a-001-c1 -> k1
- spq-a-001-c2 -> k2
- spq-a-001-c3 -> k3, k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: spq-a-002
  - note: section 1 grounds section 2 by moving from constitutive measure to the elenchi of qualitative threshold.
  - sourceClaimIds: [`spq-a-001-c2`, `spq-a-001-c3`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`spq-a-002-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 1 stabilized as one entry with line-anchored first-order claims.

### Entry spq-a-002 — Specific quantum II: elenchi and the threshold to qualitative change

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/sources/specific-quantum.txt`
- lineStart: 111
- lineEnd: 169

Summary:

The classical examples of the bald and the heap show that seemingly insignificant quantitative subtractions culminate in qualitative disappearance, exposing quantum as a moment of measure.

Key points: (KeyPoint)

- k1. Repeated negligible quantitative removals culminate in a qualitative break.
- k2. The contradiction comes from assuming quantity is merely indifferent.
- k3. Quantity is linked to quality as a moment of measure.
- k4. Concept operates through the indifferent side to effect collapse unexpectedly.

Claims: (Claim)

- c1. id: spq-a-002-c1
  - subject: repeated_insignificant_quantitative_changes
  - predicate: culminate_in
  - object: qualitative_transformation_or_loss
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [111-143] one-hair/one-grain removals accumulate into baldness and disappearance of the heap.

- c2. id: spq-a-002-c2
  - subject: ordinary_consciousness
  - predicate: errs_by_assuming
  - object: quantity_as_merely_indifferent_limit
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [144-157] the mistake is taking quantity in a one-sided abstract sense.

- c3. id: spq-a-002-c3
  - subject: quantum
  - predicate: is
  - object: moment_of_measure_linked_to_quality
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [154-169] quantity is tied to quality; abstract adherence to indifferent quantum is refuted.

Claim ↔ key point map:

- spq-a-002-c1 -> k1
- spq-a-002-c2 -> k2
- spq-a-002-c3 -> k3, k4

Relations: (Relation)

- r1. type: supports
  - targetEntryId: spq-a-001
  - note: section 2 confirms section 1's thesis that constitutive magnitude is not indifferent to qualitative being.
  - sourceClaimIds: [`spq-a-002-c1`, `spq-a-002-c3`]
  - sourceKeyPointIds: [`k1`, `k3`]
  - targetClaimIds: [`spq-a-001-c2`, `spq-a-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: spq-a-003
  - note: from demonstrated threshold effects to explicit dual-sided immediacy of measure and specifying behavior.
  - sourceClaimIds: [`spq-a-002-c3`]
  - sourceKeyPointIds: [`k3`]
  - targetClaimIds: [`spq-a-003-c1`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 2 stabilized as one entry centered on elenchi and measure-threshold logic.

### Entry spq-a-003 — Specific quantum III: immediate duality and the rise of specifying measure

Span:

- sourceFile: `src/compiler/being/measure/specific-quantity/sources/specific-quantum.txt`
- lineStart: 170
- lineEnd: 190

Summary:

Immediate measure contains a dual aspect—indifferent fluctuation and qualitative specificity—whose tension drives the sublation of indifference into specifying measure.

Key points: (KeyPoint)

- k1. Measure is immediate quality of a specific magnitude.
- k2. The same thing contains indifferent fluctuation and qualitative specificity.
- k3. The concrete measure side sublates the indifference of external alterability.
- k4. This sublating movement is the transition to specifying measure.

Claims: (Claim)

- c1. id: spq-a-003-c1
  - subject: immediate_measure
  - predicate: contains
  - object: dual_magnitude_determination_indifferent_and_specific
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [170-181] one and the same thing has both indifferent-limit and qualitative-specific sides.

- c2. id: spq-a-003-c2
  - subject: determinate_measure_side
  - predicate: sublates
  - object: indifference_of_external_alterable_side
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [182-190] concrete existence of measure negates indifference and specifies it.

- c3. id: spq-a-003-c3
  - subject: sublation_of_indifference
  - predicate: is
  - object: specifying_measure
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [188-190] text concludes this movement is a specifying of measure.

Claim ↔ key point map:

- spq-a-003-c1 -> k1, k2
- spq-a-003-c2 -> k3
- spq-a-003-c3 -> k4

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: spq-b-a-001
  - note: Part A culminates in the explicit transition to Part B (`a. The rule`) as specifying measure.
  - sourceClaimIds: [`spq-a-003-c2`, `spq-a-003-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`spq-b-a-001-c1`]
  - logicalOperator: sublative_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section 3 stabilized; Part A now handshakes to Part B decomposition.
