# Common Language Core v0

Status: design seed for the Organon language of science.

## The Word (Logos)

The Word is the shared semantic principle that both Kernel and Agent speak.
It is one language with multiple moments, not multiple unrelated dialectics.

- Kernel moment: Object-in-general stream (lawful, algorithmic, structured input).
- Essence moment: mediated grounding plus relation coherence.
- Concept moment: free synthesis under evidence and contradiction constraints.

This gives one science-language across Being, Essence, and Concept.

## Scope of v0

This v0 defines:

1. The core semantic primitives.
2. The judgment pipeline.
3. Three-store persistence semantics.
4. Projection boundaries to external standards.

## Layer Contract

- Sphere of Being
  - ingress and processing only
  - no store
  - JSON stream from the GDS Kernel

- Sphere of Essence
  - persistence domain
  - reflective judgment through FactStore and RelationStore

- Sphere of Concept
  - synthesis and freedom under mediated constraints
  - KnowledgeStore as special persistence

## Core Primitives (v0)

1. Assertion
   - propositional unit presented for possible persistence.
2. Ground
   - condition-bearing support for an assertion.
3. Condition
   - explicit circumstance under which a ground is valid.
4. Fact
   - grounded assertion admitted into FactStore.
5. Relation
   - coherence link among facts admitted into RelationStore.
6. Constraint
   - rule of validity/correctness (shape, invariant, policy).
7. Contradiction
   - explicit incompatibility requiring revision, deferral, or sublation.
8. Mediation
   - process that resolves opposition into determinate transition.
9. Modality
   - possibility/actuality/necessity determination.
10. Causality
    - ordered dependence and reciprocal conditioning.
11. Synthesis
    - concept-level unification of facts and relations.
12. Provenance
    - traceability of every persistent unit to source and transition.

## Morphological Framing (v0.1)

Morph is the operative power of Essence where grounding becomes effective
determination.

- Form and Context are presupposed moments.
- Morph sublates Form/Context into concrete persistence operations.
- The active morphology is: Ground -> Condition -> Fact.

Operational sequence:

- Form -> Context -> Morph(Ground, Condition, Fact) -> Relation -> Knowledge.

Interpretive note:

- This is Aristotelian-priority in implementation (determinate actuality first),
  while remaining compatible with later formal/categorical expression.

## Assertion-Ground Laws (v0.1)

1. Ground Priority
   - no assertion may persist without at least one explicit ground.
2. Determinacy
   - each ground must include explicit determination metadata
     (identity/difference/contradiction path).
3. Non-Circularity
   - an assertion cannot be sufficient ground for itself.
4. Mediation Requirement
   - unresolved contradiction blocks persistence and forces defer/revise/sublate.
5. Provenance Requirement
   - every ground must carry source trace (`stateId`, `irId`, transition edge).
6. Revisability
   - grounded assertions remain corrigible under stronger or conflicting grounds.
7. Coherence Lift
   - only grounded assertions are eligible for RelationStore coherence checks.

## Judgment Pipeline

The pipeline is one movement with staged determinations:

1. Judgment of Existence
   - claim enters as candidate assertion.
2. Judgment of Reflection
   - claim is checked against grounds/conditions and relation coherence.
3. Judgment of Necessity
   - claim is stabilized through modality and causal/absolute relation.
4. Judgment of Concept
   - claim is synthesized as knowledge for theoretical/practical use.

In current platform terms:

- active now: Existence + Reflection
- next major expansion: Necessity

## Three-Store Semantics

### Cypher Conjunction Principle

- FactStore and RelationStore are both Cypher-defined persisted graphs.
- RelationStore presupposes persisted FactStore outputs (facts are prior in
  persistence order, though co-evolved in runtime).
- The Modeling DataStore is a downstream artifact of this Cypher conjunction,
  not the semantic ground of persistence.
- Therefore the conjunction `FactStore + RelationStore` is the permanent
  specification substrate.

### Modeling Overlay Clarification

- Data Modeling may be treated as an optional fourth overlay for engineering
  convenience.
- This overlay is non-discursive and kernel-oriented as essential slices over
  the Kernel GraphFrame (DataFrame/Dataset representation of object streams).
- It is a ToolChain artifact for Agent and Human-in-the-loop activities.
- It is primarily object-oriented and functions as a perceptible-substance
  monitor rather than part of the semantic core.
- It is subsumed under the Three-Store model and does not alter the canonical
  semantic core: `FactStore -> RelationStore -> KnowledgeStore`.

### FactStore

- role: grounded facticity persistence.
- persistence criterion: mediated grounding.
- includes FormDB internals as schema/Cypher substrate.
- not a generic datastore and not the Modeling export surface.

### RelationStore

- role: essential-relation persistence.
- persistence criterion: relation coherence with FactStore.
- only store allowed to project schemas into Modeling layer.

### KnowledgeStore

- role: synthesized concept persistence.
- persistence criterion: compositional use of FactStore and RelationStore.
- special persistence form at the Concept boundary.

## Normative Flow Rules

- required chain: FactStore -> RelationStore -> KnowledgeStore.
- required modeling handoff: RelationStore -> Modeling layer.
- forbidden handoff: FactStore -> Modeling layer.
- required traceability: every persisted unit carries provenance.

## Law-to-Layer Mapping (v0.1)

| Law / Rule             | Primary Layer                                | Enforcement Surface                |
| ---------------------- | -------------------------------------------- | ---------------------------------- |
| Ground Priority        | FactStore                                    | Morph grounding checks             |
| Determinacy            | FactStore                                    | reflection metadata + invariants   |
| Non-Circularity        | FactStore                                    | grounding graph validation         |
| Mediation Requirement  | FactStore                                    | contradiction handling policy      |
| Provenance Requirement | FactStore + RelationStore + KnowledgeStore   | persisted provenance fields        |
| Revisability           | FactStore                                    | re-grounding / correction workflow |
| Coherence Lift         | RelationStore                                | relation admission gate            |
| Required Chain         | FactStore -> RelationStore -> KnowledgeStore | pipeline orchestration             |
| Modeling Handoff Rule  | RelationStore                                | schema projection boundary         |

## SDSL and GDSL Roles

### SDSL (specification language)

- human-authored scientific intent.
- declares domain entities, constraints, goals, and persistence policy.
- modeling-language layer: many domain/runtime models are allowed.
- legacy React-form style schemas are SDSL/UI artifacts and are not the GDSL
  semantic root.

### Many Models, One GDSL (normative)

- SDSL is plural by design (multiple domain model dialects and adapters).
- GDSL is singular by contract (one canonical normalized execution ontology).
- no model may bypass normalization: every SDSL variant must compile into the
  same GDSL semantic primitives before persistence.

### Isomorphic Claim and Shape Sharing (normative)

- an SDSL model is valid only as an isomorphic claim over canonical GDSL root
  shapes.
- this claim is a morphic restriction: SDSL payload must satisfy required
  correspondences and may not violate forbidden shape constraints.
- shape sharing is therefore controlled and directional:
  `SDSL -> normalized mapping -> GDSL RootShape`.

### Root Schema Contract (FormShape vs EntityShape)

- `FormShape` is the container schema (GDSL): it carries
  `Form + Context + Morph` as admissibility and transformation law.
- `EntityShape` is the contained schema (GDSL): it carries grounded content,
  contradiction state, and store-direction (`FactStore` / `RelationStore` /
  `KnowledgeStore`).
- dependency law: `EntityShape` presupposes a valid `FormShape`; never the
  reverse.
- persistence law: persisted `EntityShape` requires explicit `Ground` and
  `targetStore`.

### GDSL (graph executable language)

- normalized semantic IR for execution.
- carries explicit primitives (Assertion, Ground, Relation, Contradiction, etc.).
- drives Kernel/Agent interoperability.
- uses `GraphFrame` as the canonical term for the kernel graph substrate in
  specification language (instead of `GraphStore`).

SDSL compiles to GDSL; GDSL executes and persists under store rules.

## External Standards Mapping (Interop, Not Ground Language)

- RDF/RDFS
  - graph data substrate and basic vocabulary.
- OWL
  - entailment and ontological commitments.
- SHACL
  - structural and policy constraints.
- SPARQL
  - querying and transformation.

Policy for v0:

- GDSL/SDSL is canonical internal language.
- RDF/OWL/SHACL/SPARQL are projection and interchange surfaces.

## Minimal Conformance Checklist

A language implementation is Common Language Core v0 compliant only if:

1. It preserves one judgment pipeline across layers.
2. It enforces mediated grounding before fact persistence.
3. It enforces relation coherence before relation persistence.
4. It blocks direct FactStore to Modeling schema export.
5. It records provenance for every persisted artifact.
6. It supports modality/causality extension path to necessity.

## Why this is the Seed

This core is the seed because it gives one shared Word for:

- Kernel object generation,
- Essence persistence discipline,
- Concept-level scientific synthesis.

That is the design principle for the Organon platform as a language of science.
