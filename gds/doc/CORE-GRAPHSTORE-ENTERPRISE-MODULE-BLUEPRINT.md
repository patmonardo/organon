# CoreGraphStore Enterprise Module Blueprint

Date: 2026-08-05
Status: Forward implementation contract (modular, non-monolithic)

## Task Framing

Layer:

- Kernel: GraphStore execution/persistence substrate
- Agent: workflow mediation over graph/model/feature/plan artifacts
- Logic: rationale for why persisted productions count as knowledge

Target:

- empirical adequacy
- conceptual validity
- rationale coherence

Invariant checks:

- empirical adequacy
- reflexive consistency
- dialectical mediation

## Current Baseline

The repository already has a working `DefaultGraphStore` bootstrap implementation for deterministic
algorithm development and tests, with explicit note that it is not the production CoreGraphStore.

Reference surface:

- `gds/src/types/graph_store/default_graph_store.rs`
- `gds/src/types/graph_store/graph_store.rs`
- `gds/src/config/graph_store_config.rs`

The backend layer already supports adaptive selection and huge-array pathways via
collections backend config.

## Architectural Decision

CoreGraphStore should be composed from enterprise feature modules, not implemented as one monolithic
module.

Principle:

- One stable GraphStore API surface
- Multiple specialized implementation modules behind that surface
- Feature modules can evolve independently as enterprise requirements grow

## Proposed Module Topology

### 1) Core API and Snapshot Kernel

Module:

- `gds/src/types/graph_store/core/`

Responsibilities:

- stable `GraphStore` contract and immutable runtime views
- snapshot/version identity, schema consistency guards
- canonical graph view request and validation pipeline

Primary traits:

- `CoreGraphStore`
- `CoreGraphStoreSnapshot`
- `CoreGraphStoreVersion`

### 2) PropertyStore Backplane

Module:

- `gds/src/types/properties/backplane/`

Responsibilities:

- graph/node/relationship property persistence abstraction
- typed property materialization and cardinality checks
- storage adapters for vec, huge, arrow, and persisted engines

Primary traits:

- `PropertyStoreBackplane`
- `PropertyColumnCodec`
- `PropertyColumnCursor`

### 3) Huge Array and Partition Runtime

Module:

- `gds/src/collections/backends/partitioned/`

Responsibilities:

- partitioned arrays for huge node/edge domains
- adaptive page sizing and boundary-safe cursor traversal
- high-cardinality operations without monolithic allocations

Primary traits:

- `PartitionedArray<T>`
- `PartitionPlanner`
- `PartitionCursor`

### 4) Relationship Topology Partition Index

Module:

- `gds/src/types/graph/topology_partition/`

Responsibilities:

- partitioned CSR/adjacency management
- inverse index options per relationship type
- topology rebuild operations that preserve property cardinality invariants

Primary traits:

- `PartitionedTopologyStore`
- `RelationshipPartitionIndex`

### 5) Cursor and Stream Execution

Module:

- `gds/src/types/graph_store/cursor_runtime/`

Responsibilities:

- zero-copy, partition-aware node/edge/property cursors
- batch and stream traversal interfaces for algorithm and pipeline runtimes
- explicit fallback behavior for sparse or missing property regions

Primary traits:

- `NodeCursorRuntime`
- `RelationshipCursorRuntime`
- `PropertyCursorRuntime`

### 6) Dataset/Polars Integration Bridge

Module:

- `gds/src/collections/dataset/graphstore_bridge/`

Responsibilities:

- map GraphStore projections to Dataset/DataFrame SDK structures
- persist and replay model/feature/plan artifact links
- support graph-to-tabular and tabular-to-graph mediation without surface breakage

Primary traits:

- `GraphDatasetBridge`
- `GraphProjectionSerializer`
- `ArtifactLinkIndexer`

### 7) Enterprise Persistence and Recovery

Module:

- `gds/src/types/graph_store/persistence/`

Responsibilities:

- durable metadata/state persistence
- checkpointing, replay, and recovery contracts
- compatibility/version migration gates

Primary traits:

- `GraphStorePersistence`
- `GraphStoreCheckpointStore`
- `GraphStoreRecoveryPlanner`

## Why This Fits Existing Direction

- Keeps `DefaultGraphStore` as fast bootstrap/test substrate.
- Preserves the GraphStore API while adding enterprise capability behind it.
- Matches existing huge-array backend strategy already visible in config and backend factories.
- Supports your Enterprise GDSL/GML workflow direction by making model/feature/plan artifacts
  first-class linked outputs rather than side effects.

## Minimal Contract Additions (Non-Breaking)

Additive (do not remove current API):

- `graph_store_kind`: `default | core_enterprise`
- `store_version_id`: stable snapshot/version identity
- `partition_profile`: opaque descriptor of partition strategy
- `cursor_capabilities`: declared cursor/stream guarantees

These fields can be exposed through capability metadata first, then tightened into typed contracts.

## Implementation Sequencing

Phase 1:

- Extract partition and cursor traits from current backends
- Introduce `core` module with adapter wrapping `DefaultGraphStore`

Phase 2:

- Introduce `PropertyStoreBackplane` and partitioned topology module
- Add conformance tests for schema/property/topology invariants

Phase 3:

- Add persistence/checkpoint modules
- Add Dataset bridge contracts for artifact-linked projections

Phase 4:

- Promote enterprise implementations by feature lane (huge arrays, partitioned arrays, cursors)
- Keep `DefaultGraphStore` as deterministic fallback and test fixture substrate

## Non-Goals

- No immediate removal of existing `DefaultGraphStore`.
- No monolithic rewrite that couples topology, properties, persistence, and cursors in one module.
- No premature external API breakage while enterprise modules are stabilizing.

## Closing Rule

CoreGraphStore is the integration center, not a monolith.

GraphStore API remains the control surface; PropertyStore and partitioned backends provide
enterprise persistence/scale semantics; Dataset/Polars bridge provides modeler-grade mediation.
