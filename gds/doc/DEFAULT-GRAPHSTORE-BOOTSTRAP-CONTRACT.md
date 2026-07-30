# DefaultGraphStore Bootstrap Contract

## Status

`DefaultGraphStore` is RAM-only Bootstrap infrastructure for developing and testing
graph and ML algorithms. It is not a persistent GraphStore, CoreGraphStore, or the
planned Rust counterpart of Java HugeGraphStore.

Layer: Kernel.

Target: empirical adequacy during algorithm development.

## Supported Contract

- Algorithms may share immutable snapshots through `Arc<DefaultGraphStore>`.
- Catalog property updates shallow-clone the current snapshot, apply the update,
  and atomically replace the catalog entry.
- Existing readers continue to observe their prior snapshot. New readers observe
  the replacement snapshot.
- Snapshot forks share `Arc`-backed ID maps, relationship topologies, and property
  values. Metadata maps, sets, and vectors are copied.
- Structural derivations may allocate new ID maps, topology, and projected property
  values. Their cost must remain explicit in the operation that requests them.
- Mutation metadata must report effects visible in the resulting catalog snapshot.

## Non-Goals

- Persistence, transactions, crash recovery, and durable concurrent writes.
- Production memory accounting or HugeGraphStore-scale storage.
- A final CoreGraphStore ownership or locking model.
- Global NodeId type migration.

Operational code must not rely on `Arc::get_mut` for catalog mutation. Unique `Arc`
ownership is an incidental condition that normal readers invalidate.

## Node Identifier Discipline

Until the ID model is revised, new graph APIs should use `MappedNodeId` and
`OriginalNodeId` to express meaning. `usize` is reserved for allocation and indexing.
Conversions at storage, serialization, and dataframe boundaries should be checked.
Existing mixed `u32`, `u64`, `i64`, and `usize` surfaces are Bootstrap debt, not a
contract to reproduce.

## Persistence Boundary

Persistent GraphStore work belongs to the Polars/Collections GraphFrame direction
described in [POLARS-COLLECTIONS-API.md](POLARS-COLLECTIONS-API.md). CoreGraphStore
and HugeGraphStore work may replace this snapshot model rather than preserve its
implementation details.

## Invariants

- **Empirical adequacy:** algorithm effects are directly testable in the latest
  catalog snapshot.
- **Reflexive consistency:** RAM-only behavior is described as Bootstrap behavior,
  not presented as production persistence.
- **Dialectical mediation:** the temporary store supports present algorithm work
  while remaining explicitly subordinate to the persistent Polars/CoreGraphStore
  architecture.
