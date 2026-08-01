# GraphStore Types API Review

## Status

In progress. This document reviews the existing GraphStore Types surface before
Collections integration or a `CoreGraphStore` implementation begins.

- **Layer:** Kernel
- **Target:** conceptual validity, constrained by empirical adequacy
- **Scope:** `GraphStore`, `Graph`, IDs, topology, schema, properties, default
  implementers, and representative direct consumers
- **Out of scope:** broad algorithm migration, persistence, transactions, global
  node-ID migration, and implementation of `CoreGraphStore`

The governing architectural question is whether the current Types surface is a
genuine contract that both `DefaultGraphStore` and a future `CoreGraphStore` can
implement, or whether it still encodes the shape of the Bootstrap implementation.

## Invariants

- **Empirical adequacy:** existing graph and property behavior remains executable
  and testable.
- **Reflexive consistency:** `DefaultGraphStore` remains explicitly RAM-only
  Bootstrap infrastructure rather than being redescribed as production storage.
- **Dialectical mediation:** the target API must preserve current GML and algorithm
  utility while opening a real implementation boundary for Collections and Core.

## Verified Baseline

Verified on 2026-07-31:

| Check                                        | Result    |
| -------------------------------------------- | --------- |
| `cargo check -p gds --lib`                   | Pass      |
| `cargo test -p gds --lib types::graph_store` | 38 passed |
| `cargo test -p gds --lib types::graph`       | 48 passed |
| `cargo test -p gds --lib types::properties`  | 51 passed |

The unqualified `cargo test -p gds types::graph_store` does not reach the focused
tests because unrelated integration test compilation currently fails in
`gds/tests/config_validations.rs`: it imports the absent `gds::concurrency` path.
This is baseline debt, not a finding against the GraphStore Types slice.

## Current Architectural Shape

The current system has two distinct but partially collapsed levels:

1. `GraphStore` presents metadata, schema, IDs, properties, topology-derived
   counts, mutation, and graph-view construction through one trait.
2. `Graph` is the runtime algorithm view. It composes `IdMap`, node-property,
   degree, relationship-iteration, and relationship-property capabilities into
   an object-safe `Arc<dyn Graph>` surface.

`DefaultGraphStore` is both the principal implementation of `GraphStore` and a
large operation host. It directly owns graph and node property maps, but owns
relationship properties through `DefaultRelationshipPropertyStore`. Separate
`DefaultGraphPropertyStore` and `DefaultNodePropertyStore` implementations exist
and are tested, yet they are not the components used by `DefaultGraphStore`.

## Contract Matrix

| Surface                           | Present role                                             | Implementation dependency                                               | Initial classification                                      |
| --------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------- | ----------------------------------------------------------- |
| `GraphStore`                      | Store metadata, lookup, mutation, and graph-view factory | Implemented principally by `DefaultGraphStore`; generic consumers exist | Intended stable API, currently implementation-shaped        |
| `GraphStoreAdapter<G>`            | Read delegation and adapter helpers                      | Mutation always returns `InvalidOperation`                              | Bootstrap adapter, not a transparent `GraphStore` adapter   |
| `Graph`                           | Runtime graph view for algorithms                        | Returned as `Arc<dyn Graph>`                                            | Strongest current implementation boundary                   |
| `IdMap`                           | Original/mapped ID mediation and labels                  | `DefaultGraph` and `DefaultGraphStore` use `SimpleIdMap` internally     | Stable concept; concrete return/storage choices need review |
| `RelationshipTopology`            | Outgoing and optional incoming adjacency                 | `Vec<Vec<MappedNodeId>>` representation                                 | Bootstrap representation behind a valid concept             |
| `GraphSchema`                     | Immutable structural description                         | Stored beside runtime maps and topology                                 | Stable concept; synchronization invariant is implicit       |
| `MutableGraphSchema`              | Schema derivation/building                               | Used inside Bootstrap mutation and derivation                           | Construction mechanism, not runtime store API               |
| `PropertyValues` families         | Typed value access                                       | Often backed by Collections implementations                             | Necessary boundary between graph semantics and storage      |
| `PropertyStore`                   | Common keyed property map behavior                       | Exposes concrete `HashMap` by reference                                 | Translation-shaped contract with backend leakage            |
| Three specialized property stores | Graph/node/relationship property grouping                | Only relationship store is composed into the graph/store runtime        | Intended symmetry not realized by effective architecture    |
| Default property stores           | Tested in-memory implementations and builders            | Graph/node defaults are not used by `DefaultGraphStore`                 | Mixed: live utilities and parallel scaffolding              |

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

The three specialized store traits and defaults are structurally parallel, but
`DefaultGraphStore` stores:

- graph properties as `HashMap<String, Arc<dyn GraphPropertyValues>>`;
- node properties as `HashMap<String, Arc<dyn NodePropertyValues>>` plus a second
  label-to-key index;
- relationship properties as
  `HashMap<RelationshipType, DefaultRelationshipPropertyStore>`.

`DefaultGraph` repeats the same asymmetry. Therefore the dedicated graph and node
property stores do not currently define the semantics of the main GraphStore
implementation, while the relationship store does.

**Consequence:** Collections-backed property storage has no single composition
point, and tests of `DefaultGraphPropertyStore` or `DefaultNodePropertyStore` do
not prove `DefaultGraphStore` behavior.

**Review direction:** determine whether all three stores become explicit
GraphStore components, or whether the specialized store traits are removed in
favor of a different typed-column boundary. Do not preserve nominal symmetry
unless it corresponds to live ownership and invariants.

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

### F4. High: graph-view constructors do not implement one coherent contract

The five `get_graph*` methods look like overloads of one view-construction
operation, but their behavior diverges:

- `get_graph_with_types` delegates to `DefaultGraph::relationship_type_filtered_graph`,
  which treats an empty relationship-type set as no filter and preserves all
  relationships;
- `get_graph_with_types_and_selectors` constructs its own view and treats an empty
  relationship-type set as an empty graph;
- the first path filters `GraphSchema`, while the selector path passes the full
  store schema into the filtered graph;
- both orientation-taking methods ignore the supplied `Orientation`;
- the selector path duplicates topology, metadata, characteristics, property,
  and schema assembly logic instead of using one canonical constructor.

Orientation is not a dormant parameter. Dozens of algorithm and procedure call
sites request `Orientation::Undirected` or a configured orientation through these
methods. The current implementation returns a type-filtered graph with its
original traversal orientation.

**Consequence:** algorithm behavior can depend on which apparent overload was
called, and callers requesting undirected traversal do not receive the advertised
view.

**Review direction:** replace the overload family internally with one validated
`GraphViewSpec`-like request and one construction path. Define empty-set semantics,
schema filtering, selector validation, and orientation transformation exactly
once. Compatibility methods may delegate to that path.

### F5. Medium: relationship property key aggregation contradicts its contract

`relationship_property_keys_for_types` is documented as returning keys common to
all selected relationship types. `DefaultGraphStore` implements it with
`flat_map(...).collect()`, producing the union. The corresponding node method
computes an intersection.

**Consequence:** validation or projection code may accept a relationship property
that is absent from some selected relationship types.

**Review direction:** make graph/node/relationship aggregation semantics explicit
and symmetric, then add a behavioral test with overlapping but non-identical key
sets before changing the implementation.

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

Store operations use `GraphStoreError`; graph-view construction returns
`GraphResult<T> = Result<T, Box<dyn Error + Send + Sync>>`. Several application
boundaries convert failures to `String`.

**Consequence:** implementations and consumers lose stable error categories at
precisely the boundary a second store implementation must share.

**Review direction:** define typed construction/view errors and preserve their
categories through the Kernel. Application serialization may translate them at
the outer boundary.

## Target API Criteria

The final target specification will be accepted only if it demonstrates:

1. A minimal read/view contract implementable by both a small in-memory store and
   a Collections/Core-backed store.
2. Explicit authority rules for schema, materialized topology, and materialized
   properties.
3. A coherent composition model for graph, node, and relationship property
   storage.
4. Explicit mutation or successor-store capabilities rather than universal
   mutable methods with implementation-dependent behavior.
5. Typed errors across store-to-view construction.
6. One canonical graph-view request whose type filters, property selectors,
   orientation, schema, and empty-filter behavior cannot diverge by entrypoint.
7. Preservation of `Graph` as the algorithm-facing runtime abstraction unless a
   concrete conformance failure requires changing it.
8. A compile-time conformance fixture using a second minimal implementation, so
   the contract is not validated solely by `DefaultGraphStore`.

## Next Review Slice

The next implementation step is a symbol-level audit of:

1. schema/runtime authority in every `GraphStore` query and mutation;
2. the three property-store families and their actual call sites;
3. `DefaultGraphStore::graph` and `get_graph*` construction of `DefaultGraph`;
4. one representative generic algorithm consumer and one concrete catalog,
   factory, result, and Collections consumer.

That evidence will refine these initial findings into the target trait split and
a sequenced, compatibility-preserving refactor backlog.
