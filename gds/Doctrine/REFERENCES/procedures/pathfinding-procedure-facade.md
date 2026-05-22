# Pathfinding Procedure Facade

Status: Reference doctrine for Pathfinding procedure routing.

## Definition

A Pathfinding procedure facade is the public route from application code into a graph algorithm. It validates the request, selects the graph projection, constructs storage and computation runtimes, wires task progress and termination, executes the requested mode, and returns a stable public result.

It is intentionally shallower than algorithm doctrine. The facade explains the operation and its modes. The algorithm doctrine explains storage loops, computation state, concurrency strategy, and termination checkpoints.

## Routing Law

Application-facing code follows this route:

```text
GraphFacade
  -> pathfinding procedure builder/facade
  -> storage runtime compute method
  -> computation runtime state methods
```

Application-facing examples and procedures must not call `::algo::` modules directly. Direct storage/computation examples are allowed only as `algo_` internal architecture exemplars.

## Procedure Entry Contract

Each Pathfinding procedure entry should name:

- Purpose: the graph problem solved by the procedure
- Graph contract: orientation, source/target behavior, weighted or unweighted assumptions, and unsupported cases
- Config surface: builder methods, spec config fields, Java-style aliases, defaults, and validation rules
- Modes: `stream`, `stats`, `mutate`, `write`, and `estimate_memory`, with unsupported or deferred modes named explicitly
- Result shape: stream row, stats object, mutation/write summaries, and property write semantics
- Task behavior: task name, progress unit, phase structure, and whether task factories are consumed
- Concurrency and termination: request concurrency, true parallel work, batch parallelism, and cancellation boundary
- Memory estimate: node-scaled, relationship-scaled, walk/path-scaled, and concurrency-scaled structures
- Boundary covenant: procedure validates and orchestrates; storage owns graph access and the controlling loop; computation owns ephemeral state
- Exemplar: a `GraphFacade` usage or runnable `proc_` example

## Completeness Checklist

- The public type is a facade, with a builder alias when existing call sites expect builder naming.
- `new`, `from_spec_config`, `from_spec_json`, and `with_spec_config` validate through the spec config model when a spec exists.
- Java-style aliases are accepted where this crate tracks Java GDS config names.
- `stream` and `stats` are implemented when the algorithm naturally has rows and aggregate stats.
- `mutate` and `write` are implemented only when their semantics are real, or explicitly deferred in the entry.
- `estimate_memory` uses actual graph size and the runtime structures allocated by the procedure.
- Progress is storage-owned and logged in meaningful incremental units.
- Termination is request-scoped and passed into storage when the storage loop can observe it.
- Concurrency is either real algorithmic parallelism, batch parallelism, or documented as request/task configuration.
- Tests cover config aliases, memory scaling, and at least one mode-level smoke path.

## Initial Pathfinding Entries

### Dijkstra

Purpose: weighted single-source shortest paths.

Procedure surface: `GraphFacade::dijkstra()` returns a facade/builder that validates source and direction, selects the requested projection, creates Dijkstra storage and computation runtimes, and exposes public modes according to the current facade implementation.

Boundary: procedure owns request parsing and projection selection. Storage owns the priority-search loop and graph relationship streaming. Computation owns distances, predecessors, visited state, and path reconstruction state.

Doctrine role: first deep exemplar for Algorithm Doctrine because the storage/computation split is compact and canonical.

### Random Walk

Purpose: generate seeded or unseeded node walks, including node2vec-style return and in-out bias factors.

Procedure surface: `GraphFacade::random_walk()` accepts `walks_per_node`, `walk_length`, `return_factor`, `in_out_factor`, optional `source_nodes`, optional `random_seed`, and `concurrency`. Spec JSON accepts Java-style aliases such as `walksPerNode`, `walkLength`, `returnFactor`, `inOutFactor`, `sourceNodes`, and `randomSeed`.

Modes: `stream` yields walk paths; `stats` reports walk count and execution time; `mutate` and `write` materialize walk paths through the current path relationship-store adapter; `estimate_memory` accounts for generated walk storage and actual relationship-count graph overhead.

Boundary: procedure validates config, selects natural orientation, sets up task progress and termination, and invokes storage. Storage materializes adjacency and controls concurrent walk generation. Computation owns walk sampling state and seed-driven generation.

Doctrine role: first facade upgrade target because it combines config aliases, task setup, termination, concurrency, memory estimation, and procedure-level modes in one compact file.

### Spanning Tree

Purpose: compute a minimum or maximum spanning tree from a start node.

Procedure surface: `GraphFacade::spanning_tree()` accepts start node, min/max selection, relationship type filters, direction, weight property, and concurrency configuration. It exposes Java-style aliases for common config names.

Boundary: procedure validates and projects; storage owns Prim traversal, termination checks, and relationship-work progress; computation owns priority queue, parent state, costs, and visited state.

Doctrine role: tree-family exemplar for progress parity and termination-aware storage loops.

## Descent Boundary

The procedure facade stops at routing. The companion Algorithm Doctrine begins with storage and computation:

- Traversal: BFS and DFS
- Priority search: Dijkstra and A*
- Relaxation: Bellman-Ford and Delta-Stepping
- Meta-algorithms: Yen's and All Shortest Paths
- Tree algorithms: Spanning Tree, K-Spanning Tree, Steiner Tree, and Prize-Collecting Steiner Tree
- DAG algorithms: Topological Sort and DAG Longest Path
- Stochastic walks: Random Walk

Each algorithm entry should describe storage responsibility, computation state, graph access pattern, progress unit, concurrency model, termination checkpoints, memory structures, and intentional differences from Java GDS.
