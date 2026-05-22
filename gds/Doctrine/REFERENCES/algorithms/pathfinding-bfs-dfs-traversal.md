# Pathfinding BFS/DFS Traversal

Status: Reference doctrine for the first Pathfinding workbook pair.

## Definition

BFS and DFS are the elementary traversal pair. They share a procedure surface and differ in movement form.

- BFS is breadth-first: it expands by frontier level.
- DFS is depth-first: it commits to a branch before backtracking.

Together they teach the first distinction in Pathfinding Algorithm Doctrine: the graph can be traversed by equality of distance or by depth of commitment.

## Shared Procedure Contract

Both facades expose:

- `source` / `source_node`
- `target` / `targets`
- `max_depth`
- `track_paths`
- `concurrency`
- `stream`
- `stats`
- `mutate`
- `write`
- `estimate_memory`

Both spec configs accept Java-style aliases:

- `sourceNode`
- `targetNodes`
- `maxDepth`
- `trackPaths`

Omitted JSON fields fall back to Rust defaults.

## Storage/Computation Boundary

Traversal storage owns:

- graph access
- traversal control
- progress volume based on relationships
- target exit predicates
- depth aggregation hooks

Traversal computation owns:

- visited state
- source and depth initialization
- max-depth checks
- optional path state

The computation runtime does not stream graph relationships. That belongs to storage.

## BFS Doctrine

BFS storage controls a queue/frontier. Each visited node belongs to a level. In unweighted traversal, this makes BFS the natural shortest-path-by-hop-count procedure.

Memory basis:

- queue entries
- visited state
- optional path/predecessor state
- actual relationship-count graph overhead

Progress basis: relationships examined.

Concurrency: the storage path can use the parallel BFS traversal helper when configured with concurrency greater than one.

## DFS Doctrine

DFS storage controls branch-first traversal through the sequential DFS helper. Each expansion deepens the current branch until a limit, target, or dead end forces return.

Memory basis:

- stack-like traversal entries
- visited state
- optional path/predecessor state
- actual relationship-count graph overhead

Progress basis: relationships examined.

Concurrency: the facade carries concurrency as part of the common procedure surface, but the current DFS traversal helper is sequential.

## Workbook Use

Use BFS/DFS together when teaching Pathfinding from first principles:

1. Use one graph and one source.
2. Run BFS stream and DFS stream.
3. Compare row order and depth.
4. Inspect stats.
5. Inspect memory estimates.
6. Descend into storage/computation only after the procedure route is clear.

This pair sets the tone for later Pathfinding workbooks: procedure first, then algorithm movement, then storage/computation mechanism.
