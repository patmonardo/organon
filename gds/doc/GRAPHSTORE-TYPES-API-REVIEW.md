# GraphStore Types API Review

## Status

Active platform-upgrade plan. This document reviews and upgrades the existing
GraphStore Types surface before Collections integration or a `CoreGraphStore`
implementation begins.

- **Layer:** Kernel
- **Target:** conceptual validity, constrained by empirical adequacy
- **Scope:** `GraphStore`, `Graph`, IDs, topology, schema, properties, default
  implementers, and representative direct consumers
- **Out of scope:** broad algorithm migration, persistence, transactions, global
  node-ID migration, and implementation of `CoreGraphStore`

The governing architectural question is whether the current Types surface is a
genuine contract that both `DefaultGraphStore` and a future `CoreGraphStore` can
implement, or whether it still encodes the shape of the Bootstrap implementation.

## Review Posture

Java GDS compatibility remains an external behavioral reference, especially for
the GraphStore and Graph capabilities consumed by algorithms. It is not a mandate
to reproduce Java ownership, factories, or class structure in Rust.

`DefaultGraphStore` is a disposable conformance probe. Its standalone RAM model,
`Arc` snapshots, nested vectors, and direct property maps are useful because they
make API contradictions cheap to expose and test. They are not design constraints
for `CoreGraphStore`. Code should be retained only where it clarifies the shared
contract or remains useful for small deterministic algorithm tests.

The Applications layer exposes two first-class public worlds:

- Dataset is the primary data and statistical-learning API.
- GraphStore is the graph-native API for projection, topology, typed graph
  properties, graph views, algorithms, and catalog operations.

GraphStore may interoperate with Dataset and Collections, but it must not disappear
behind them. `CoreGraphStore` therefore needs a stable Applications-facing contract,
not merely an internal Dataset adapter.

## Invariants

- **Empirical adequacy:** existing graph and property behavior remains executable
  and testable.
- **Reflexive consistency:** `DefaultGraphStore` remains explicitly RAM-only
  Bootstrap infrastructure rather than being redescribed as production storage.
- **Dialectical mediation:** the target API must preserve current GML and algorithm
  utility while opening a real implementation boundary for Collections and Core.

## Verified Baseline

Verified on 2026-08-01 with the installed stable toolchain selected explicitly:

| Check                                        | Result    |
| -------------------------------------------- | --------- |
| `cargo check -p gds --lib`                   | Pass      |
| `cargo test -p gds --lib types::graph_store` | 60 passed |
| `cargo test -p gds --lib types::graph`       | 80 passed |
| `cargo test -p gds --lib types::properties`  | 59 passed |

The repository has no pinned or default rustup toolchain in the current
environment, so the executable commands used `cargo +stable`. No global rustup
configuration was changed.

## Current Architectural Shape

The current system has two distinct but partially collapsed levels:

1. `GraphStore` presents metadata, schema, IDs, properties, topology-derived
   counts, mutation, and graph-view construction through one trait.
2. `Graph` is the runtime algorithm view. It composes `IdMap`, node-property,
   degree, relationship-iteration, and relationship-property capabilities into
   an object-safe `Arc<dyn Graph>` surface.

`DefaultGraphStore` is both the principal implementation of `GraphStore` and a
large operation host. It now composes `DefaultGraphPropertyStore` and
`DefaultNodePropertyStore`, while relationship properties remain grouped by
relationship type in `DefaultRelationshipPropertyStore` instances. `DefaultGraph`
still materializes graph-view property maps directly, so store composition and
runtime-view representation remain intentionally distinct Bootstrap concerns.

## Contract Matrix

| Surface                           | Present role                                             | Implementation dependency                                                 | Initial classification                                       |
| --------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------- | ------------------------------------------------------------ |
| `GraphStore`                      | Store metadata, lookup, mutation, and graph-view factory | Implemented principally by `DefaultGraphStore`; generic consumers exist   | Intended stable API, currently implementation-shaped         |
| `GraphStoreAdapter<G>`            | Read delegation and adapter helpers                      | Mutation always returns `InvalidOperation`                                | Bootstrap adapter, not a transparent `GraphStore` adapter    |
| `Graph`                           | Runtime graph view for algorithms                        | Returned as `Arc<dyn Graph>`                                              | Strongest current implementation boundary                    |
| `IdMap`                           | Original/mapped ID mediation and labels                  | `DefaultGraph` and `DefaultGraphStore` use `SimpleIdMap` internally       | Stable concept; concrete return/storage choices need review  |
| `RelationshipTopology`            | Outgoing and optional incoming adjacency                 | `Vec<Vec<MappedNodeId>>` representation                                   | Bootstrap representation behind a valid concept              |
| `GraphSchema`                     | Immutable structural description                         | Stored beside runtime maps and topology                                   | Stable concept; synchronization invariant is implicit        |
| `MutableGraphSchema`              | Schema derivation/building                               | Used inside Bootstrap mutation and derivation                             | Construction mechanism, not runtime store API                |
| `PropertyValues` families         | Typed value access                                       | Often backed by Collections implementations                               | Necessary boundary between graph semantics and storage       |
| `PropertyStore`                   | Common keyed property map behavior                       | Exposes concrete `HashMap` by reference                                   | Translation-shaped contract with backend leakage             |
| Three specialized property stores | Graph/node/relationship property grouping                | All are composed by `DefaultGraphStore`; relationship stores are per type | Live Bootstrap composition; Core authority still unspecified |
| Default property stores           | In-memory typed-column ownership and mutation            | Used by `DefaultGraphStore`; views receive selected value maps            | Live Bootstrap components, not Core storage constraints      |

## Findings

### F1. High: `GraphStore` is not yet a substitutable Core boundary

The trait combines four responsibilities:

- immutable store description and lookup;
- mutable property and label operations;
- destructive relationship operations;
- construction of filtered runtime graph views.

Its mutation methods accept `impl Into<String>`, which makes the trait unsuitable
for trait objects. Generic consumers can use `G: GraphStore`, but catalogs,
loaders, factories, exporters, procedure results, and mutation paths frequently
name `DefaultGraphStore` directly. A second implementation can satisfy the trait
syntactically without becoming substitutable across the actual system.

**Consequence:** implementing `CoreGraphStore: GraphStore` alone would not provide
a usable Core replacement boundary.

**Review direction:** define the smallest implementation-neutral read/view
contract first, then decide whether mutation and derivation belong in separate
capability traits or associated operation services.

### F2. High: the declared three-PropertyStore architecture is not the effective architecture

**Partial resolution:** `DefaultGraphStore` now composes all three specialized
property-store families. Graph and node mutations pass through typed column
objects and update schema alongside the live stores. `DefaultGraph` still receives
selected property-value maps when a runtime view is constructed, which is a view
materialization choice rather than store ownership.

**Remaining consequence:** the common `PropertyStore` trait still exposes a
concrete `HashMap` representation, and no implementation-neutral contract states
whether schema or materialized columns authorize existence and type. Collections-
backed property storage therefore has composition points but not yet a valid Core
boundary.

**Remaining direction:** specify authority and validation rules for each property
family, then narrow or replace representation-leaking `PropertyStore` methods.
Do not require Core storage to reproduce the Bootstrap maps used by runtime views.

### F3. High: schema and runtime state have an implicit dual-source-of-truth contract

`DefaultGraphStore` holds `GraphSchema` beside ID labels, topologies, property
maps, label/property indexes, and relationship metadata. Some queries consult
runtime stores first and fall back to schema; other queries merge runtime and
schema state. Mutations update selected runtime structures but do not express one
central invariant that reconstructs or validates the complete state.

**Consequence:** a method may report that a property or relationship type exists
through one representation while values or topology are absent in another.
Future persistent storage would magnify this ambiguity.

**Review direction:** specify which component is authoritative for existence,
type, and availability, and define validation rules for materialized values versus
declared schema.

#### Authority audit (2026-08-01)

`DefaultGraphStore` is a Bootstrap coordinator over several authorities; it is
not itself the authority model that Core storage must reproduce.

| Dimension                | Existence and availability                                  | Declared type or shape                             | Coordinated mutation                                                       |
| ------------------------ | ----------------------------------------------------------- | -------------------------------------------------- | -------------------------------------------------------------------------- |
| Node identity and labels | `SimpleIdMap`                                               | Node entries in `GraphSchema`                      | `DefaultGraphStore`; label addition now updates both representations       |
| Relationship types       | Materialized topology map                                   | Relationship entries in `GraphSchema`              | `DefaultGraphStore`; deletion now removes topology, properties, and schema |
| Graph properties         | `DefaultGraphPropertyStore` columns                         | Column-embedded schema mirrored into `GraphSchema` | `DefaultGraphStore`                                                        |
| Node properties          | `DefaultNodePropertyStore`; separate per-label key index    | Column-embedded schema and node schema entries     | `DefaultGraphStore`                                                        |
| Relationship properties  | Per-type `DefaultRelationshipPropertyStore` columns         | Column-embedded relationship schema                | `DefaultGraphStore`                                                        |
| Inverse availability     | Incoming adjacency on each topology                         | Not declared by schema                             | Topology construction and metadata rebuild                                 |
| Counts                   | ID map and topology; aggregate relationship count is cached | Not applicable                                     | Structural operations and metadata rebuild                                 |

Six direct contradictions were repaired during the audit:

- `add_node_label` now updates `SimpleIdMap` and `GraphSchema` together and is
  covered by `validate_graph_store_schema`.
- `delete_relationships` now removes the complete relationship dimension from
  topology, property storage, and schema.
- node-property replacement now replaces its label domain in both schema and
  `node_properties_by_label`.
- `relationship_property_type` now requires a relationship type and resolves
  only within that type's materialized property store.
- topology rebuilds now reject retained relationship columns whose materialized
  cardinality no longer matches the rebuilt per-type topology.
- structural topology mutations rebuild aggregate relationship count,
  relationship ordering, parallel-edge state, and inverse-index type caches
  through one metadata boundary, with lifecycle regression coverage.

The Core contract should make materialized topology and typed columns
authoritative for availability, use schema as the validated declaration of shape,
and require construction or mutation boundaries to reconcile both atomically.

### F4. High: graph-view constructors do not implement one coherent contract

**Resolution:** selector-aware and plain type filtering now share
one internal `DefaultGraph::filtered_by_relationship_types` implementation owned
by `DefaultGraphStore` view construction. Empty type filters, filtered schema
construction, topology metadata, and relationship-property filtering no longer
diverge between entrypoints. `Graph` no longer exposes view construction;
HashGNN and CSV export now request views from `GraphStore`.

Reverse and undirected views now materialize oriented Bootstrap adjacency while
sharing relationship-property values through an edge-index remap. Regression tests
verify direction, relationship counts, schema/characteristics, and distinct edge
weights. This is a conformance implementation, not the required Core storage
strategy.

`GraphViewSpec` is now the canonical Rust request implemented by GraphStore. The
Java-compatible `get_graph_with_*` methods remain public compatibility conveniences
and delegate to it. A direct conformance test and overload tests exercise the same
path.

The canonical path validates explicit relationship types and property selectors
against materialized topology and relationship-property state before allocating
an oriented view. Missing materialization and selectors for unselected types are
reported through `GraphViewError`.

The graph-level concurrency operation is now named `concurrent_view`, distinct
from `RelationshipIterator::concurrent_copy` and its cursor-iterator semantics.

### F5. Medium: relationship property key aggregation contradicts its contract

**Resolution:** corrected on 2026-07-31. `DefaultGraphStore` now computes the
intersection for selected relationship types and treats an empty selection as all
relationship property keys, matching the node-property contract. A regression
test covers shared, type-specific, and empty-selection behavior.

### F6. Medium: Bootstrap operations are embedded in the substrate implementation

`DefaultGraphStore` includes backend conversion, scaling, path collapse, inverse
index construction, induced-subgraph projection, random construction support,
and several structural derivations. These operations were useful for rapidly
building algorithms, but they are not all intrinsic store capabilities.

**Consequence:** the apparent GraphStore API is narrower than the effective API
consumers receive from `DefaultGraphStore`, encouraging concrete coupling and
making Core requirements difficult to distinguish from Bootstrap conveniences.

**Review direction:** classify each inherent method as constructor, invariant-
preserving store primitive, derived graph operation, algorithm, or compatibility
helper. Only primitives required by multiple implementations belong in the Core
contract.

### F7. Medium: mutation semantics differ across otherwise similar surfaces

`GraphStore` exposes `&mut self` mutation. Catalog mutation instead shallow-clones
a `DefaultGraphStore`, mutates the successor, and replaces the catalog entry.
`GraphStoreAdapter` delegates reads but rejects every mutation even when the
adapter itself is borrowed mutably.

**Consequence:** callers cannot infer from the trait whether mutation means
in-place change, copy-on-write snapshot derivation, unsupported capability, or a
persistent transaction.

**Review direction:** make mutation capability and result semantics explicit.
The Bootstrap snapshot policy may remain an implementation, but it should not be
silently encoded as the universal Core model.

### F8. Medium: error contracts are split and weakly typed at the view boundary

**Partial resolution:** store operations use `GraphStoreError`; graph-view
construction now uses typed `GraphViewError` categories for unmaterialized types,
unselected selector types, and unmaterialized relationship properties. Several
application boundaries still convert failures to `String`.

**Consequence:** implementations and consumers lose stable error categories at
precisely the boundary a second store implementation must share.

**Remaining direction:** preserve `GraphViewError` categories through the Kernel.
Application serialization may translate them at the outer boundary.

## Target API Criteria

The final target specification will be accepted only if it demonstrates:

1. A minimal read/view contract implementable by both a small in-memory store and
   a Collections/Core-backed store.
2. A stable GraphStore surface that Applications can expose alongside Dataset
   without coupling callers to `DefaultGraphStore`.
3. Explicit authority rules for schema, materialized topology, and materialized
   properties.
4. A coherent composition model for graph, node, and relationship property
   storage.
5. Explicit mutation or successor-store capabilities rather than universal
   mutable methods with implementation-dependent behavior.
6. Typed errors across store-to-view construction.
7. One canonical graph-view request whose type filters, property selectors,
   orientation, schema, and empty-filter behavior cannot diverge by entrypoint.
8. Preservation of `Graph` as the algorithm-facing runtime abstraction unless a
   concrete conformance failure requires changing it.
9. A compile-time conformance fixture using a second minimal implementation, so
   the contract is not validated solely by `DefaultGraphStore`.

## Platform Upgrade Gate

CoreGraphStore implementation must not begin until these phases complete in
order:

1. **Refresh the executable baseline (complete 2026-08-01).** The library-scoped
   GraphStore, Graph, and property checks pass after the Types and algorithm
   upgrades.
2. **Specify authority (in progress).** The initial authority matrix and two
   direct synchronization repairs are complete. Finish the remaining property
   index, topology rebuild, and aggregate-cache decisions.
3. **Split capabilities.** Derive the smallest read/view contract from that
   authority model; separate mutation, successor-store derivation, and Bootstrap
   operations where their semantics differ.
4. **Prove substitution.** Compile a second minimal implementation and pass it
   through one representative catalog, factory, graph-view, and algorithm path.
5. **Reconcile decisions.** Amend draft ADRs 0001-0003 so they target the proven
   contract rather than requiring the pre-upgrade `GraphStore` surface unchanged.

The immediate implementation slice is phase 1 followed by the schema/runtime
authority audit in phase 2. Resolved graph-view behavior remains a regression
gate; it is no longer an open audit item.
