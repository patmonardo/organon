# Community Algorithms - Architectural Review

## Overview

This document mirrors the Procedure-First Controller Pattern checklist used for centrality algorithms. Every community algorithm should present:

- **Procedure Layer**: the facade that validates inputs, creates runtimes, wires progress trackers, and invokes the algorithm stack.
- **Storage Runtime**: the controller that owns graph access, enforces progress/termination, and feeds a computation runtime with only the data it needs.
- **Computation Runtime**: a pure-state, graph-free runtime that receives callbacks or pre-materialized data and produces the final result.

## Current Status Summary

- **Total Community Algorithms**: 12
- ✅ **Fully Architecturally Sound** (12 algorithms): approx_max_kcut, conductance, k1coloring, kcore, kmeans, label_propagation, leiden, louvain, modularity, scc, triangle, wcc
- ⚠️ **Partially Compliant** (0 algorithms)
- ❌ **Non-Compliant** (0 algorithms)

**Reviewed**: 12/12 procedures
**Notes**: Triangle has been collapsed into a single facade aligned with the storage/controller pattern; Local Clustering Coefficient was removed pending a future reintroduction.

---

## approx_max_kcut - ✅ Controller-Backed, Java-Shaped Parity In Progress

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/approx_max_kcut.rs](gds/src/procedures/community/approx_max_kcut.rs)): validates config, creates the Java-shaped task progress tracker, spawns `TerminationFlag`, instantiates storage + computation runtimes, exposes stream/stats/run/mutate/write/estimate modes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/approx_max_kcut/storage.rs](gds/src/algo/approx_max_kcut/storage.rs)): controller obtains the natural view, validates node-count-dependent min community sizes, materializes weighted adjacency once (weight fallback respects the `has_relationship_weight_property` flag), drives Java-named progress phases, honors `TerminationFlag`, and calls computation with a neighbor provider.
- **3. Computation Runtime** ([gds/src/algo/approx_max_kcut/computation.rs](gds/src/algo/approx_max_kcut/computation.rs)): pure state; builds adjacency/reverse caches, enforces min community sizes, and runs GRASP-style randomized restarts plus a contained serial VNS pass when `vns_max_neighborhood_order > 0`.

### Pattern Compliance
- Fully aligned: procedure wires runtimes, storage owns graph access + progress/termination, computation remains graph-free.

### Key Notes
- Config now includes Java-shaped `min_batch_size` and `vns_max_neighborhood_order`; the application layer passes both through the procedure facade.
- Memory estimation is component-shaped after Java's candidate/workspace/improvement-cache/swap-status/VNS candidate model, with Rust-specific eager adjacency materialization included.
- Full Java parallel local search remains deferred: Rust does not yet implement degree partitions, atomic node-to-community weight caches, swap-status conflict control, or executor-backed per-partition tasks.

---

## conductance - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/conductance.rs](gds/src/procedures/community/conductance.rs)): builds `ConductanceConfig`, creates the Java-shaped `Conductance` progress task, exposes stream/stats/mutate/write, and instantiates the storage + computation runtimes with a termination flag.
- **2. Storage Runtime** ([gds/src/algo/conductance/storage.rs](gds/src/algo/conductance/storage.rs)): obtains the natural-orientation graph view, validates the community property, drives three well-defined phases (count relationships, accumulate counts, compute conductances) while logging node/community-capacity progress, and passes per-phase results into the computation runtime. It enforces the `TerminationFlag` and reports phase-level progress.
- **3. Computation Runtime** ([gds/src/algo/conductance/computation.rs](gds/src/algo/conductance/computation.rs)): purely stateful; it partitions nodes by degree, counts internal/external weights per community with Java-aligned default weighting, accumulates totals, and finally computes conductance scores without touching the graph.

### Pattern Compliance
- Storage is the controller (graph access, progress, termination) and delegates all state management to computation. Procedure only wires the two runtimes together and reports a component-shaped memory estimate for local counts, accumulated counts, conductances, and result materialization.

---

## k1coloring - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/k1coloring.rs](gds/src/procedures/community/k1coloring.rs)): validates config, builds a task progress tracker, spawns a `TerminationFlag`, instantiates storage + computation runtimes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/k1coloring/storage.rs](gds/src/algo/k1coloring/storage.rs)): owns the undirected graph view, supplies neighbor iteration, enforces termination, and drives per-iteration progress (color and validate phases) before invoking computation.
- **3. Computation Runtime** ([gds/src/algo/k1coloring/computation.rs](gds/src/algo/k1coloring/computation.rs)): remains graph-free; it manages atomic colors/bitsets, runs greedy coloring and validation loops with configurable concurrency, and surfaces per-iteration progress callbacks.

### Pattern Compliance
- Fully aligned: storage owns graph access + progress/termination, computation stays pure state, and the procedure merely wires the runtimes together.

### Key Notes
- Storage now drives neighbor access, per-iteration progress, and termination; computation remains purely stateful.

---

## kcore - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/kcore.rs](gds/src/procedures/community/kcore.rs)): validates config, builds the task progress tracker, spawns a `TerminationFlag`, instantiates storage + computation runtimes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/kcore/storage.rs](gds/src/algo/kcore/storage.rs)): owns the undirected graph view, supplies neighbor iteration, enforces termination, and drives progress before invoking computation.
- **3. Computation Runtime** ([gds/src/algo/kcore/computation.rs](gds/src/algo/kcore/computation.rs)): remains graph-free; manages degrees/cores, rebuilds node providers when sparse, and performs scan/act phases with configurable concurrency.

### Pattern Compliance
- Fully aligned: storage owns graph access + progress/termination, computation keeps state, and the procedure only wires the runtimes together.

---

## kmeans - ✅ Controller-Backed, Java-Shaped Parity In Progress

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/kmeans.rs](gds/src/procedures/community/kmeans.rs)): validates numeric config, builds the KMeans progress task, wires the termination flag, exposes stream/stats/run/mutate/write/estimate modes, instantiates storage + computation runtimes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/kmeans/storage.rs](gds/src/algo/kmeans/storage.rs)): owns the undirected graph view, validates presence/dimensions/NaN-free feature values (plus seeded centroid shapes), materializes per-node feature vectors with progress + termination, then calls computation.
- **3. Computation Runtime** ([gds/src/algo/kmeans/computation.rs](gds/src/algo/kmeans/computation.rs)): remains pure; handles sampling (uniform/KMeans++), swaps-based iteration with delta threshold, seeded centroid fast-path, optional silhouettes, and multi-restart selection without graph access.

### Pattern Compliance
- Storage now acts as the controller (graph/property access, progress, termination) and feeds the pure computation runtime; the procedure only wires runtimes together.

### Key Notes
- Application dispatch now passes `deltaThreshold` and `numberOfRestarts` through the procedure facade and preserves optional `randomSeed` semantics.
- Memory estimation is component-shaped after Java's best-community, centroid, distance, cluster-manager, per-task, silhouette, and seeded-centroid components, with Rust's materialized feature vectors included.
- Current progress is phase-level (`read feature properties`, `KMeans iteration`, optional `Silhouette`) because the pure computation runtime does not expose Java's per-partition task callbacks.

---

## label_propagation - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/label_propagation.rs](gds/src/procedures/community/label_propagation.rs)): validates config, builds a concurrent progress tracker over seed-materialization + iteration volume, spawns `TerminationFlag`, instantiates storage + computation runtimes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/label_propagation/storage.rs](gds/src/algo/label_propagation/storage.rs)): owns the undirected graph view, validates optional weight/seed properties, materializes weights and Java-parity seeds (fallback `maxLabelId + originalNodeId + 1`), reports progress, and calls computation with a weighted neighbor provider.
- **3. Computation Runtime** ([gds/src/algo/label_propagation/computation.rs](gds/src/algo/label_propagation/computation.rs)): unchanged pure state; executes deterministic Gauss–Seidel voting with weighted ties, optional seeds, and convergence detection without graph access.

### Pattern Compliance
- Storage now acts as the controller (graph/property access, progress, termination) and feeds the pure computation runtime; the procedure only wires runtimes together.

---

## leiden - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/leiden.rs](gds/src/procedures/community/leiden.rs)): wires `LeidenConfig`, prepares a progress tracker over node count, creates `TerminationFlag`, and calls `LeidenStorageRuntime::compute_leiden`.
- **2. Storage Runtime** ([gds/src/algo/leiden/storage.rs](gds/src/algo/leiden/storage.rs)): builds weighted adjacency lists via the undirected view, logs progress per node, honors the termination flag, and then hands the `AdjacencyGraph` plus config to the computation runtime.
- **3. Computation Runtime** ([gds/src/algo/leiden/computation.rs](gds/src/algo/leiden/computation.rs)): implements the Leiden pipeline (local move, refinement, aggregation), maintains modularity/tolerance tracking, and returns communities/modularity/level data without touching the graph store.

### Pattern Compliance
- Fully aligned: storage controls graph access and iteration, computation keeps state, and the procedure only wires them together with progress/termination.

---

## modularity - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/modularity.rs](gds/src/procedures/community/modularity.rs)): validates the community property and concurrency, builds a task progress tracker, exposes stream/stats/run/mutate/write, spawns a `TerminationFlag`, instantiates storage + computation runtimes, and delegates to storage.
- **2. Storage Runtime** ([gds/src/algo/modularity/storage.rs](gds/src/algo/modularity/storage.rs)): owns the undirected graph view, validates the community property, materializes per-node community labels with termination checks, streams weighted neighbors with Java-aligned fallback weight `1.0`, logs progress, and calls computation.
- **3. Computation Runtime** ([gds/src/algo/modularity/computation.rs](gds/src/algo/modularity/computation.rs)): pure state; densifies community ids, aggregates inside weight and degree totals, and computes per-community modularity plus the overall score without graph access.

### Pattern Compliance
- Fully aligned: storage controls graph/property access and progress/termination; computation remains graph-free; procedure wires the runtimes together and reports a Java-shaped memory estimate for community mapping, relationship accumulators, result objects, and per-thread collectors.

---

## modularity_optimization - ✅ Controller-Backed, Parity In Progress

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/modularity_optimization.rs](gds/src/procedures/community/modularity_optimization.rs)): exposes the assignment-producing stream/stats/run/mutate/write modes, validates local-moving config, builds the Java-shaped progress task, and writes community IDs as node properties for mutate/write.
- **2. Storage Runtime** ([gds/src/algo/modularity_optimization/storage.rs](gds/src/algo/modularity_optimization/storage.rs)): owns the undirected graph view, optional relationship-weight selector, adjacency materialization, progress, and termination checks before invoking pure computation.
- **3. Computation Runtime** ([gds/src/algo/modularity_optimization/computation.rs](gds/src/algo/modularity_optimization/computation.rs)): remains graph-free and performs the current deterministic local-moving modularity optimization over `ModularityOptimizationInput`.

### Pattern Compliance
- The module now has the same Procedure -> Storage -> Computation boundary as the rest of the Community family while preserving Louvain's existing dependency on the pure computation runtime.
- Full Java K1-colored parallel optimization, seed-property reversal, and exact per-color progress remain follow-up parity work rather than being folded into this controller-first slice.

---

## louvain - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/louvain.rs](gds/src/procedures/community/louvain.rs)): obtains an undirected view, creates a progress tracker sized by relationships, and calls into `LouvainStorageRuntime` and `LouvainComputationRuntime`.
- **2. Storage Runtime** ([gds/src/algo/louvain/storage.rs](gds/src/algo/louvain/storage.rs)): materializes adjacency lists with weight fallbacks, reports progress per node, constructs a `ModularityOptimizationInput`, and forwards it to the computation runtime.
- **3. Computation Runtime** ([gds/src/algo/louvain/computation.rs](gds/src/algo/louvain/computation.rs)): iterates levels of modularity optimization, leverages `ModularityOptimizationComputationRuntime`, handles aggregation, and finally maps original nodes into community ids.

### Pattern Compliance
- Storage acts as controller, computation keeps state, and procedure only wires runtimes. The modularity optimizer already lets computation focus on state transitions without touching the graph view.

---

## scc - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/scc.rs](gds/src/procedures/community/scc.rs)): validates concurrency, wires progress/termination together, and calls `SccStorageRuntime::compute_scc`.
- **2. Storage Runtime** ([gds/src/algo/scc/storage.rs](gds/src/algo/scc/storage.rs)): obtains a natural orientation view, begins progress tracking, enforces the termination flag, and hands the graph view to the computation runtime.
- **3. Computation Runtime** ([gds/src/algo/scc/computation.rs](gds/src/algo/scc/computation.rs)): stream-based Tarjan-style exploration using stacks, HugeLongArray indexes, and frequent progress logging; it never touches the graph store directly beyond the supplied view.

### Pattern Compliance
- Fully compliant; storage orchestrates the directed traversal while computation tracks indexes/stack membership.

---

## triangle - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/triangle.rs](gds/src/procedures/community/triangle.rs)): builds progress tracker, instantiates termination flag, and calls `TriangleStorageRuntime::compute` with config.
- **2. Storage Runtime** ([gds/src/algo/triangle/storage.rs](gds/src/algo/triangle/storage.rs)): gathers sorted adjacency lists, logs per-node progress, enforces `TerminationFlag`, and serves `get_neighbors` closures to computation.
- **3. Computation Runtime** ([gds/src/algo/triangle/computation.rs](gds/src/algo/triangle/computation.rs)): caches neighbor sets, intersects adjacency lists, and returns local/global triangle counts without graph access.

### Pattern Compliance
- Storage acts as controller and computation handles pure logic; procedure merely wires them with progress/termination.

---

## wcc - ✅ Architecturally Sound

### Layered Architecture
- **1. Procedure Layer** ([gds/src/procedures/community/wcc.rs](gds/src/procedures/community/wcc.rs)): validates concurrency, wires progress/termination, and invokes `WccStorageRuntime::compute_wcc`.
- **2. Storage Runtime** ([gds/src/algo/wcc/storage.rs](gds/src/algo/wcc/storage.rs)): creates a natural orientation view (to ensure weak connectivity), logs progress, and passes the view to the computation runtime.
- **3. Computation Runtime** ([gds/src/algo/wcc/computation.rs](gds/src/algo/wcc/computation.rs)): orchestrates sampled/unsampled union-find passes, uses `HugeAtomicDisjointSetStruct`, respects optional thresholds, and emits community assignments.

### Pattern Compliance
- Fully aligned; storage handles graph access + progress, computation keeps union-find state, and procedure only wires termination + progress.

### Parity/Correctness Notes
- Mirrors Java `Wcc` sampled/unsampled strategies: sampled path uses inverse relationships when available; unsampled path unions outgoing edges only, so weak connectivity on directed graphs depends on inverse topology or undirected projections, matching Java behavior.

---

## Scope Conclusion

- Fully compliant procedures already follow the Procedure→Storage→Computation pattern.
- Partially compliant procedures still own graph access; migrating their adjacency/neighbor streaming into the storage runtime (or helping the facade call storage before reaching computation) is the next step.
- No applications layer content is referenced in this review, keeping the focus on the core controller pattern defined in the repository guidelines.
