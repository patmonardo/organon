# Three-Store Contract (FactStore, RelationStore, KnowledgeStore)

Status: draft runtime contract for TS-first SDSL compilation.

Quick reference: this is the fast mental map for returning to the architecture.

## Purpose

Make the architecture executable as compiler artifacts:

- Sphere of Being: ingress and processing only (no store).
- Sphere of Essence: persistence domain.

1. FactStore (grounded facticity + FormDB internals)
2. RelationStore (essential-relation surface validated against FactStore)
3. KnowledgeStore (compiled conceptual unification target; special persistence)

## Lockdown Rules v1

These are normative and must hold for all implementations.

1. Being ingress accepts raw JSON from the Mathematic Pure Reason layer.
2. Being acts as input channel to Essence by compiling JSON into logical form
   for perception/intuitive structure.
3. Sphere of Being has no store.
4. Essence is persistence, and persistence here is discursive.
5. KnowledgeStore is a special type of persistence.
6. Essence persistence is discursive: store payloads are lexical/propositional
   (words, predicates, relations), not raw modeling tables.
7. FactStore is grounding-only (facticity, grounds, conditions, contradiction
   handling); it is not the Modeling export surface.
8. RelationStore is the only store allowed to export schema material into the
   Modeling layer for relational data.
9. KnowledgeStore is synthesis-only: applies Concept to Appearance by composing
   FactStore and RelationStore outputs.
10. Forbidden edge: `FactStore -> Modeling layer`.
11. Required edge chain: `FactStore -> RelationStore -> KnowledgeStore` and
    `RelationStore -> Modeling layer`.

## Kernel and Schema Identity

- Mathematic Pure Reason is the GDS Kernel.
- Being ingress receives raw JSON from this kernel-side layer.
- Being itself remains pre-persistence (no store in Sphere of Being).
- FormDB internals are not a fourth store; they are the schema internals of
  FactStore (raw Cypher/form-definition substrate).

## Contract Objects

- FactStore
  - graphBackend: neo4j
  - graphRef: `neo4j://factstore/{graphName}`
  - includesFormDBInternals: true
  - formDbRole: `internal-cypher-and-form-definitions` (not a top-level store)
  - groundsBackend: postgres
  - groundsRef: `postgres://fact_grounds/{specId}`
  - groundsPurpose: `fact-grounds-support` (single-purpose, not general SQL modeling)
  - groundsDomain: `grounds-and-conditions`
  - unboundedFacticity: true
  - prismaTemplate:
    - templateRef: `@organon/factstore-prisma-template`
    - metamodelRef: `prisma.metamodel.factstore.v1`
    - scope: `single-factstore-metamodel`
  - postgresRole: `grounds-and-conditions-maintenance`
  - filesystemGrounds:
    - backend: `filesystem`
    - ref: `file://fact_grounds/{specId}`
    - domains: `image | audio | video | binary | document`
  - kernelCache:
    - mode: `polyglot-dataset-cache`
    - backends: `polars | duckdb | postgres | arrow` (extensible)
  - dataAccess:
    - polars: `postgres=partial-direct`, `filesystem=partial-direct`
    - kernel: `neo4j=direct`, `postgres=direct`, `filesystem=direct`
  - agentMonitoring:
    - enabled: true
    - scope: `factstore-structure-observability`
    - channels: `agent-runtime | dataset-health | cache-health`
  - datasetAugmentation:
    - dragonSeedSdkRef: `@organon/dragonseed-sdsl-sdk`
    - profile: `sdsl-dataset-analyses`
  - components:
    - DataFrame aspect (tabular/compilation-oriented)
    - living Dataset aspect (presumed actual evolving facticity)
  - substrate role: DragonBook/DragonSeed data substrate for transcendental execution

- RelationStore
  - source: `FactStore`-validated assertions and essential relations
  - relationBackend: neo4j
  - relationRef: `neo4j://relationstore/{graphName}`
  - schemaExportRole: `only-store-allowed-to-feed-modeling`
  - requiredValidation: `must-agree-with-factstore`
  - outputs:
    - relation assertions
    - schema-ready relation projections for Modeling layer

- KnowledgeStore
  - targetRef: `sdsl-spec-{specId}-knowledge-store` (or caller-provided output)
  - compileMode: `transcendental-logic`
  - sourceRefs: links to FactStore and RelationStore refs

## Compiler Placement

The contract is emitted by `compileSdslSpecification` as `storeContract` in
`SdslCompilerArtifacts`.

Additional emitted artifacts for fast reference:

- `substrateMapping.rootDataFrame`: GDSL root DataFrame descriptor
  - produced by specification compiler
  - `datasetSdkExecuted: false`
- `substrateMapping.mappings`: Model/Feature + FeatureStruct mappings into
  both DataFrame definitions and Dataset definitions.
- `datasetSdkGeneration`: SDSL SDK generator contract (NLP processor + DragonSeed augmentation)
  - includes direct-access capability matrix and agent monitoring profile

This keeps the TS-first workflow while preserving a stable kernel-aligned
compilation boundary.

## Architectural Meaning

- FactStore carries appearance/facticity growth (potentially infinite) across DataFrame + living Dataset aspects.
- FactStore contains FormDB internals as the raw Cypher/form-definition substrate.
- FactStore is a refined facticity engine, not a generic datastore.
- FactStore behaves as an associated memory-ground that supports the
  appearances processed in RelationStore.
- FactStore carries the Grounds/Conditions of facticity while facts-in-themselves are maintained in Neo4j.
- RelationStore carries essential relations validated against FactStore and is the Modeling export boundary.
- KnowledgeStore is the transcendental unification target compiled from specification-driven TS artifacts.

## Essence/Form Engine: Two Sides

- Side A (Grounding): FactStore as conditioned facticity and contradiction
  resolution.
- Side B (Appearance-Relation): RelationStore as essential relation surface
  validated against FactStore.
- Their mediated unity yields KnowledgeStore at the Concept transition boundary.

## Inner Intuition Persistence Spec (Draft)

This section defines the three stores as layered persistence for inner intuition.

### Axioms

1. **Ingress Axiom**: Sphere of Being is pre-persistence and holds no store.
   It transforms kernel JSON into logical form for Essence.
2. **Grounding Axiom**: A claim persists in FactStore only if it is grounded
   under explicit conditions.
3. **Relation Axiom**: A relation persists in RelationStore only if it agrees
   with FactStore-supported facticity.
4. **Synthesis Axiom**: Knowledge persists in KnowledgeStore only as mediated
   synthesis over FactStore and RelationStore references.
5. **Direction Axiom**: Persistence flows `FactStore -> RelationStore -> KnowledgeStore`.
6. **Boundary Axiom**: `FactStore -> Modeling layer` is forbidden; only
   RelationStore may project schemas into Modeling.
7. **Discursive Axiom**: Essence persistence payloads are lexical/propositional
   (words, predicates, relations), not raw tabular modeling artifacts.
8. **Internality Axiom**: FormDB internals are schema internals of FactStore,
   not an independent persistence axis.

### Operational Checks

- Every RelationStore assertion includes proof of FactStore validation.
- Every KnowledgeStore assertion includes `usesFactIds` and `usesRelationIds`.
- Every persistence transition carries provenance (`stateId`, `irId`).
- Any attempt to emit modeling schema directly from FactStore is rejected.

## Facticity Doctrine

- Facts-in-themselves remain in Neo4j (FactStore proper, pure Cypher).
- Grounds and Conditions of Facts are maintained in Postgres through the TS-Zod-Prisma grounds model.
- The Reflective Form Processor uses this grounds layer when reflecting on essential relations.
- Historical wording of "overflow" is retained only as a migration intuition; canonical wording is
  **grounding transition** from fact graph representation to grounds/conditions representation.
- Without this reflective grounds aspect, the relative form process lacks a stable foundation.

## Hegelian Mapping in Relative Form Processor

- Architecture is grounded in Hegelian logic of Essence and Appearance, not generic storage design.
- `Entity-Property-Aspect` is the operational triad used by the Relative Form Processor.
- This triad is interpreted through the Essence structures:
  - `Thing`
  - `World`
  - `Law`
  - `Essential Relations`
- FactStore and grounds contracts are therefore reflective-logical commitments, not only persistence choices.

## Runtime Positioning

- TS (GDSL/SDSL, Zod, Cypher) is authoritative for specification and reflective design.
- Kernel compilation/runtime is cache-oriented realization of those specifications.
- Operationally: epistemic reflection in TS + transcendental realization in kernel.

## Clarified Generation Rule

- The Root DataFrame of GDSL is not produced by executing the Dataset SDK.
- Dataset SDK generation produces SDSL-level dataset structures from specifications.
- The compiler must persist explicit mapping links:
  - Model/Feature + FeatureStruct → DataFrame definition
  - Model/Feature + FeatureStruct → Dataset definition
- Non-tabular heavy payloads (image/audio/video/binary/document) route to filesystem grounds
  as filesystem grounds rather than burdening Postgres.
