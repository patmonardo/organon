# FormDB Three-Store Contract

Status: draft runtime contract for TS-first SDSL compilation.

Quick reference: this is the fast mental map for returning to the architecture.

## Purpose

Make the architecture executable as compiler artifacts:

1. FormCatalog (finite SDSL list)
2. FactStore (unbounded facticity as grounds and conditions)
3. KnowledgeStore (compiled unification target)

## Contract Objects

- FormCatalog
  - backend: neo4j
  - finiteCatalog: true
  - listRef: `neo4j://formdb/specifications/{specId}`
  - specificationIds: finite list of governed SDSL ids
  - purity: **pure form root table** (no dataset store required)

- FactStore
  - graphBackend: neo4j
  - graphRef: `neo4j://factstore/{graphName}`
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

- KnowledgeStore
  - targetRef: `sdsl-spec-{specId}-knowledge-store` (or caller-provided output)
  - compileMode: `transcendental-logic`
  - sourceRefs: links to FormCatalog and FactStore graph/grounds refs

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

- FormCatalog carries principle governance (finite list of forms) as pure root table.
- FactStore carries appearance/facticity growth (potentially infinite) across DataFrame + living Dataset aspects.
- FactStore carries the Grounds/Conditions of facticity while facts-in-themselves are maintained in Neo4j.
- KnowledgeStore is the transcendental unification target compiled from specification-driven TS artifacts.

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
