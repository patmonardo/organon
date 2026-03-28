# Object Part A Workbook

Part: `A. THE MECHANICAL OBJECT`
Status: active
Authority: original source text only

## Authority + format lock (must persist)

- Contract reference: `src/compiler/essence/reflection/foundation/WORKBOOK-CONTRACT-V1.md`
- This workbook markdown is the authoritative Knowledge Graph artifact for this part.
- Any generated Cypher/DB projection is derivative and non-authoritative.
- Future edits must preserve this exact heading order and entry schema unless a migration is explicitly recorded in `Decision:` and applied repo-wide to all affected workbooks.

## Clean-room rules

- Use only `object.txt` as authority.
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

## Session: 2026-02-22 (first Object pass)

Scope:

- file: `object.txt`
- fixed range: lines `4-157`
- segmentation basis: numbered sections `1`, `2`, `3`

Decision:

- Keep claims minimal and line-anchored.
- Keep relation semantics conservative pending cross-part review.
- Use pseudo-Cypher labels: `Key points: (KeyPoint)`, `Claims: (Claim)`, `Relations: (Relation)`.

### Entry mecha-obj-001 — Section 1: object as immediate totality without intrinsic opposition

Span:

- sourceFile: `src/compiler/concept/object/mechanism/object.txt`
- lineStart: 12
- lineEnd: 73

Summary:

Section 1 determines the object as informed totality that excludes abstract matter/form division, yet appears as an aggregate plurality whose determinacy does not self-ground.

Key points: (KeyPoint)

- k1. The object is not an external compound of matter and form; singularity and universality are already conceptually integrated.
- k2. Thing/property, substance/accident, and part/whole reflection forms are treated as surpassed by the object-concept.
- k3. The object is initially indeterminate internally and relationless in its manifold.
- k4. This indeterminacy yields a plurality of objects that are totalities but externally indifferent.
- k5. Monad language approximates object-totality better than atomism, yet still fails self-determination because its ground lies outside.

Claims: (Claim)

- c1. id: mecha-obj-001-c1
  - subject: mechanical_object
  - predicate: is_determined_as
  - object: informed_totality_without_matter_form_split
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [12-20] explicit rejection of abstract matter/form split and requirement that matter be understood as informed.
    - [17-18] abstract separation of singularity and universality has no place in the object.

- c2. id: mecha-obj-001-c2
  - subject: mechanical_object
  - predicate: negates_as_explanatory_forms
  - object: thing_property_substance_accident_and_external_part_whole_models
  - modality: asserted
  - confidence: 0.94
  - evidence:
    - [21-29] inherited reflection-relations are treated as concluded in the concept; no separable properties/accidents.
    - [30-34] differences are themselves object-totalities, not merely parts against a whole.

- c3. id: mecha-obj-001-c3
  - subject: object_plurality
  - predicate: is_determined_as
  - object: aggregate_of_totalities_without_self_grounding_determinacy
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [36-56] object is internally indeterminate/relationless and therefore appears as plurality/composite aggregate.
    - [57-73] atom rejected; monad considered but remains non-self-determined because grounding lies outside.

Claim ↔ key point map:

- c1 -> k1
- c2 -> k2
- c3 -> k3, k4, k5

Relations: (Relation)

- r1. type: transitions_to
  - targetEntryId: mecha-obj-002
  - note: shifts from internal indeterminacy/plurality to explicit externalization of determinacy and deterministic regress.
  - sourceClaimIds: [`mecha-obj-001-c3`]
  - sourceKeyPointIds: [`k3`, `k4`]
  - targetClaimIds: [`mecha-obj-002-c1`, `mecha-obj-002-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section-1 segmentation locked to numbered boundary.

### Entry mecha-obj-002 — Section 2: externalized determinacy and deterministic infinite progression

Span:

- sourceFile: `src/compiler/concept/object/mechanism/object.txt`
- lineStart: 75
- lineEnd: 130

Summary:

Section 2 presents the object as determinate yet indifferent to its determinations, so determinacy is displaced into other objects and determinism collapses into an empty infinite regress.

Key points: (KeyPoint)

- k1. The object is a totality of determinateness but lacks negative self-unity.
- k2. Internal determinacies and their unifying form are mutually indifferent.
- k3. The object's determinateness is located outside itself in other objects.
- k4. Determinism therefore proceeds infinitely without principled self-determination.
- k5. Explanatory extension to another object becomes formally empty.

Claims: (Claim)

- c1. id: mecha-obj-002-c1
  - subject: mechanical_object
  - predicate: is_determined_as
  - object: determinate_totality_without_immanent_negative_unity
  - modality: asserted
  - confidence: 0.95
  - evidence:
    - [75-85] totality of determinateness is stated, but singular determinations are indifferent and not intrinsically comprehended.
    - [86-92] unifying form is external and indifferent to the determinacies it combines.

- c2. id: mecha-obj-002-c2
  - subject: object_determinateness
  - predicate: is_displaced_into
  - object: other_objects_in_indefinite_regress
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [94-103] determinateness of totality lies outside in other objects ad infinitum.
    - [105-113] object points beyond itself and no principle of self-determination is found.

- c3. id: mecha-obj-002-c3
  - subject: determinist_explanation
  - predicate: collapses_into
  - object: empty_formal_progression_without_grounding
  - modality: asserted
  - confidence: 0.93
  - evidence:
    - [114-124] deterministic assignment to another indifferent object is infinitely extendable and arbitrary in stopping.
    - [125-130] explanation beyond the object is called empty because no self-determination is found in the other.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3
- c3 -> k4, k5

Relations: (Relation)

- r1. type: supports
  - targetEntryId: mecha-obj-001
  - note: confirms section-1 diagnosis that plurality lacks intrinsic self-grounding determination.
  - sourceClaimIds: [`mecha-obj-002-c2`, `mecha-obj-002-c3`]
  - sourceKeyPointIds: [`k3`, `k4`, `k5`]
  - targetClaimIds: [`mecha-obj-001-c3`]
  - logicalOperator: implicative_support
  - analysisMode: first_order_claim_projection

- r2. type: transitions_to
  - targetEntryId: mecha-obj-003
  - note: deterministic externalization transitions into explicit contradiction and mechanical process.
  - sourceClaimIds: [`mecha-obj-002-c3`]
  - sourceKeyPointIds: [`k4`, `k5`]
  - targetClaimIds: [`mecha-obj-003-c1`, `mecha-obj-003-c2`]
  - logicalOperator: sequential_transition
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section-2 span preserved exactly at numbered boundary.

### Entry mecha-obj-003 — Section 3: tautological doubling and contradiction as mechanical process

Span:

- sourceFile: `src/compiler/concept/object/mechanism/object.txt`
- lineStart: 132
- lineEnd: 157

Summary:

Section 3 argues that when determinateness lies in another object, explanation becomes tautological doubling, yielding the contradiction of identity-in-difference that constitutes the mechanical process.

Key points: (KeyPoint)

- k1. Determinateness replicated across objects yields no qualitative differentiation.
- k2. Explanation becomes tautological external back-and-forth.
- k3. Objects remain externally self-subsistent despite identical determinateness.
- k4. This identity/externality contradiction forms a negative unity of reciprocal repulsion.

Claims: (Claim)

- c1. id: mecha-obj-003-c1
  - subject: duplicated_determinateness_between_objects
  - predicate: entails
  - object: tautological_explanation
  - modality: asserted
  - confidence: 0.97
  - evidence:
    - [132-138] determinateness doubled across two objects is identical; comprehension is tautology.
    - [139-145] doubling expresses only externality and vacuity of difference.

- c2. id: mecha-obj-003-c2
  - subject: object_relation_under_mechanical_condition
  - predicate: culminates_in
  - object: contradiction_of_external_indifference_and_identical_determinateness
  - modality: asserted
  - confidence: 0.96
  - evidence:
    - [146-153] objects remain external while determinateness is identical.
    - [154-157] contradiction becomes negative unity of reciprocal repulsion: the mechanical process.

Claim ↔ key point map:

- c1 -> k1, k2
- c2 -> k3, k4

Relations: (Relation)

- r1. type: refines
  - targetEntryId: mecha-obj-002
  - note: makes explicit the logical contradiction implicit in deterministic infinite progression.
  - sourceClaimIds: [`mecha-obj-003-c1`, `mecha-obj-003-c2`]
  - sourceKeyPointIds: [`k2`, `k4`]
  - targetClaimIds: [`mecha-obj-002-c2`, `mecha-obj-002-c3`]
  - logicalOperator: contradiction_explication
  - analysisMode: first_order_claim_projection

Review outcome:

- review_pending
- notes: section-3 endpoint fixed at the explicit closing sentence of mechanical process.
