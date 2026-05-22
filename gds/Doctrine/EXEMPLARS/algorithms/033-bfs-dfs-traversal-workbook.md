# Exemplar 033 — BFS and DFS as Traversal Workbook

File: `gds/examples/proc_pathfinding_procedure.rs`
Fixture root: `gds/fixtures/procedures/032-pathfinding-procedure-facade/`

## Principle

This exemplar treats Doctrine as a Knowledge Base: a Doctrinal Dataset whose entries are not loose notes, but reusable learning artifacts. BFS and DFS are the first Pathfinding workbook because they reveal the most basic algorithmic difference: breadth as level-order expansion, depth as branch-first descent.

The procedure view tells us how the operation is routed. The algorithm view tells us what kind of movement the graph undergoes.

## What It Does

1. Starts from the same `GraphFacade` procedure exemplar as Exemplar 032
2. Reads the BFS result as level-order traversal from a source node
3. Reads the DFS result as branch-first traversal from the same source node
4. Compares their shared facade contract: source, targets, max depth, path tracking, stream, stats, memory
5. Names the descent into storage and computation without making application code call internals

## The Arc

BFS and DFS sit at the first algorithmic threshold inside Procedure. Procedure emits the operation, but traversal teaches what it means for the graph to become process.

```text
Procedure Route
  -> BFS: queue, frontier, level
  -> DFS: stack, branch, backtrack
```

This is the first workbook form for Pathfinding: one graph, one source, two movements.

## Workbook Reading

### Shared Surface

Both procedures expose the same basic user grammar:

- `source` names the starting node
- `targets` optionally names termination goals
- `max_depth` bounds traversal
- `track_paths` decides whether path reconstruction evidence is retained
- `stream` emits row-level path results
- `stats` emits aggregate evidence
- `estimate_memory` describes queue or stack state, visited state, optional path state, and actual relationship-scaled graph overhead

### BFS Reading

BFS is the traversal of equality of distance. It exhausts the current depth before moving to the next. In unweighted pathfinding, this gives shortest paths by edge count.

Storage owns the queue/frontier control and graph relationship streaming. Computation owns visited state, depth checks, and optional path state.

### DFS Reading

DFS is the traversal of commitment to a branch. It goes as deep as allowed, then returns to alternatives. It is the natural seed of cycle detection, topological reasoning, and structural search.

Storage owns the stack-like traversal control and graph relationship streaming. Computation owns visited state, depth checks, and optional path state.

## Namespace Discipline

This is an `algo_` Doctrine exemplar in meaning, even though its current executable evidence is the `proc_` example. That is intentional for this first workbook: application code must still enter through procedures.

Future internal architecture examples may use `algo_` filenames when they explicitly teach storage/computation internals. Those examples must be marked as internal and must not be presented as application guidance.

## Key Vocabulary

- **BFS** — Breadth-first traversal, level-order movement. See [BFS/DFS Traversal](../../REFERENCES/algorithms/pathfinding-bfs-dfs-traversal.md).
- **DFS** — Depth-first traversal, branch-first movement. See [BFS/DFS Traversal](../../REFERENCES/algorithms/pathfinding-bfs-dfs-traversal.md).
- **Frontier** — The current boundary of traversal expansion.
- **Visited State** — Computation-owned memory that prevents repeated traversal of the same node.
- **Depth** — The aggregate traversal weight used by both BFS and DFS in this unweighted setting.

## Next Exemplar

Next: Dijkstra and A* as priority-search workbook.

After BFS and DFS teach movement without weight, Dijkstra and A* introduce order by cost and heuristic judgment. That is the next real descent into Pathfinding.

## Notes for Students

Watch for the shape of memory. BFS carries queue/frontier pressure. DFS carries stack/branch pressure. Both carry visited state. When path tracking is enabled, both retain predecessor/path evidence.

Try this: run the procedure exemplar and compare `01-bfs-stream.txt` with `02-dfs-stream.txt`. The same graph has two movements. That is the beginning of Pathfinding as a Knowledge Base topic, not only an API surface.
