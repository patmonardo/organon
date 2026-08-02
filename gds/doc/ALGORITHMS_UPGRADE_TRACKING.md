# Algorithms Upgrade Tracking

## Status

Active Kernel upgrade ledger as of 2026-08-01.

- **Target:** empirical adequacy, constrained by the shared Graph/GraphStore contract.
- **Registered application bindings:** 47 of 47.
- **Interpretation:** binding coverage proves reachability, not algorithm correctness.
- **Completed tranche:** pathfinding request-context propagation.
- **Next tranche:** shared focused-spec macro debt.

An algorithm is certified only when all applicable gates pass:

1. configuration parsing and validation use live request values;
2. execution uses live Graph/GraphStore data rather than placeholders;
3. deterministic small-graph tests assert exact behavior;
4. application execution propagates request-scoped progress and termination;
5. progress reports incremental work while `AlgorithmMachinery` owns the
   application lifecycle;
6. supported modes and structured errors are tested;
7. focused library tests and `cargo check -p gds --lib` pass.

## Active Tranche

| Algorithm          | Request progress                                    | Request termination                                                             | Focused correctness   | Status              |
| ------------------ | --------------------------------------------------- | ------------------------------------------------------------------------------- | --------------------- | ------------------- |
| Yen's              | Propagated through application, facade, and storage | Checked before work, per solution, parallel spur execution, and Dijkstra        | 54 focused tests pass | Complete 2026-08-01 |
| Dijkstra           | Propagated through application, facade, and storage | Checked before work and during queue processing                                 | 73 focused tests pass | Complete 2026-08-01 |
| A\*                | Propagated through application, facade, and storage | Checked before coordinate/search work and during delegated Dijkstra             | 60 focused tests pass | Complete 2026-08-01 |
| BFS                | Propagated through application, facade, and storage | Checked before work, per queue iteration, frontier, and parallel chunk          | 60 focused tests pass | Complete 2026-08-01 |
| DFS                | Propagated through application, facade, and storage | Checked before work and per stack iteration                                     | 60 focused tests pass | Complete 2026-08-01 |
| Bellman-Ford       | Propagated through application, facade, and storage | Checked before work, per frontier, parallel chunk, node, and relationship       | 39 focused tests pass | Complete 2026-08-01 |
| Delta-Stepping     | Propagated through application, facade, and storage | Checked before work, per bin/frontier, parallel chunk, node, and relationship   | 45 focused tests pass | Complete 2026-08-01 |
| All Shortest Paths | Propagated through application, facade, and storage | Checked before work, per MSBFS batch/callback, worker, queue, and relationship  | 22 focused tests pass | Complete 2026-08-01 |
| Topological Sort   | Propagated through application, facade, and storage | Checked during graph scan, in-degree setup, ready queues, workers, and results  | 17 focused tests pass | Complete 2026-08-01 |
| DAG Longest Path   | Propagated through application, facade, and storage | Checked during graph scan, queues, edge updates, results, and path backtracking | 15 focused tests pass | Complete 2026-08-01 |
| Spanning Tree      | Propagated through application, facade, and storage | Checked before work, during graph scans, Prim queues, relationships, and result | 46 focused tests pass | Complete 2026-08-01 |
| K-Spanning Tree    | Propagated through application, facade, and storage | Checked through delegated Prim, k-limiting queues, neighbors, and cleanup       | 14 focused tests pass | Complete 2026-08-01 |
| Steiner Tree       | Propagated through application, facade, and storage | Checked through terminal scans, delta bins, path merging, pruning, and results  | 29 focused tests pass | Complete 2026-08-01 |
| Random Walk        | Propagated through application, facade, and storage | Checked during adjacency scans, workers, walks, steps, and biased sampling      | 23 focused tests pass | Complete 2026-08-01 |

Direct compatibility methods create a local tracker and running flag.
Application stream/stats use caller-owned context, and storage emits only work
deltas. Pre-terminated storage and facade tests prove cancellation before useful
work. Composite Yen's and A\* searches propagate the same request termination
flag into delegated Dijkstra execution. BFS and DFS propagate the same flag
through their shared traversal engines; parallel BFS checks each frontier and
Rayon expansion chunk. Bellman-Ford and Delta-Stepping propagate the same flag
through sequential and parallel relaxation loops, including relationship scans.
All Shortest Paths propagates the request flag through unweighted MSBFS batches
and weighted parallel Dijkstra workers without replacing it in worker scope.
Topological Sort and DAG Longest Path preserve the same flag through directed
adjacency materialization, concurrent ready-queue traversal, and result assembly.
Spanning Tree checks the request flag through graph cursor collection and Prim
queue/relationship processing. K-Spanning Tree preserves that exact flag through
its delegated Prim phase and fallible k-limiting phase without nested progress
lifecycle. Steiner Tree checks terminal preparation, delta-stepping frontiers and
bins, graph relationships, predecessor-chain merging, pruning, and aggregation.
Random Walk preserves the same request flag through adjacency materialization,
parallel source assignment, each generated walk, each step, and biased neighbor
sampling.

## Focused Spec Macro Contract

`define_algorithm_spec!` now supports an optional `config_type` clause. Opted-in
specs deserialize, apply defaults and aliases, call the typed `validate()` method,
and return structured `ConfigError` values before execution. Legacy invocations
remain source-compatible and retain pass-through parsing until migrated.

BFS is the first certified adoption. Its generated spec accepts Java aliases and
defaults while rejecting invalid concurrency through the shared macro contract.
Further adoptions are per-spec correctness work rather than shared macro work.

## Historical Inventory

### Procedure modules present

Directory: `gds/src/procedures/`

Observed modules:

- Centrality/community-ish: `pagerank/`, `hits/`, `louvain/`, `betweenness/`, `closeness/`, `harmonic/`, `degree_centrality/`
- Path/traversal: `bfs/`, `dfs/`, `dijkstra/`, `bellman_ford/`, `delta_stepping/`, `astar/`, `yens/`, `all_shortest_paths/`, `traversal/`
- Components/structure: `scc/`, `wcc/`, `kcore/`, `k1coloring/`
- Triangles/cluster: `triangle_count/`, `local_clustering_coefficient/`
- Misc placeholders / grouping: `algorithms/`, `facades/`, `core/`

### Original `define_algorithm_spec!` inventory

These have an explicit “spec” entrypoint wired via the focused macros:

- `all_shortest_paths/spec.rs`
- `astar/spec.rs`
- `bellman_ford/spec.rs`
- `bfs/spec.rs`
- `degree_centrality/spec.rs`
- `delta_stepping/spec.rs`
- `dfs/spec.rs`
- `dijkstra/spec.rs`
- `hits/spec.rs`
- `local_clustering_coefficient/spec.rs`
- `scc/spec.rs`
- `spanning_tree/spec.rs`
- `sum/spec_focused.rs`
- `wcc/spec.rs`
- `yens/spec.rs`

### Existing tests

There are many unit + integration tests scattered across procedures (notably: `*_integration_tests.rs`, plus module-local tests in `computation.rs`, `storage.rs`, and `spec.rs`).

This is good news: we can upgrade algorithm correctness while keeping the harness stable.

## Known upgrade hotspots

These are patterns seen in the codebase that tend to indicate “translated but not yet correct”:

- Hard-coded placeholders like `let node_count = 100; // Note: Replace with actual graph store`.
- TODOs for missing graph-store accessors (edge weights, node counts, property access).
- Specs validating config with `projection::codegen::config::validation::ConfigError` while other config flows use `config::validation::ConfigError`.

## Recommended upgrade workflow (repeat per algorithm)

1. **Spec sanity**: ensure `parse_config` + `validate()` match expected parameters and defaults.
2. **GraphStore contract**: replace placeholder graph values with real `GraphStore` queries.
3. **Correctness tests**:
   - Add 1–3 minimal graph fixtures per algorithm.
   - Assert exact outputs for small graphs.
4. **Performance knobs**: wire concurrency / chunking fields (don’t micro-optimize yet).
5. **Agent UX**: ensure errors are structured and actionable (field + message).

## Campaign Order

1. Migrate focused specs with stable typed validation to `config_type` in small
   algorithm-family tranches.
2. Select the next family from concrete failures; community facades are the
   current reference for context-aware execution.

## Notes

- Keep changes surgical: prioritize correctness and stable contracts over refactors.
- Collections/Arrow work should be pulled in only when an algorithm is blocked by missing property backends or materialization.
