# Exemplar 032 — Pathfinding Procedure Facade as Routing Doctrine

File: `gds/examples/proc_pathfinding_procedure.rs`
Fixture root: `gds/fixtures/collections/proc/proc_pathfinding_procedure/`

## Principle

This exemplar teaches Procedure as the first operational form of Objectivity. A Pathfinding procedure is not the algorithm itself. It is the public route that receives a graph request, validates configuration, selects the graph projection, wires progress and termination, and then descends into the algorithm controller.

The method is facade first. The application sees a stable procedure. The storage runtime sees the graph. The computation runtime sees only ephemeral state.

## What It Does

1. Builds a small directed graph as an in-memory `DefaultGraphStore`
2. Wraps that store with `GraphFacade`
3. Calls `GraphFacade::random_walk()` through procedure modes only
4. Emits stream rows, stats, and memory estimate artifacts
5. Persists a fixture manifest that records the procedure route

The example deliberately does not call `::algo::random_walk` directly. It proves the routing law: application code enters through the procedure facade.

## The Arc

This exemplar belongs to the Procedure stage of the arc:

```text
Source -> Observation -> Reflection -> Principle -> Concept -> Judgment -> Syllogism -> Procedure
```

At this stage, the system no longer asks only what the graph means. It asks what operation is emitted. The procedure is the emitted operation as a controlled route.

Pathfinding is special because the route immediately reveals the two-level descent:

```text
Application
  -> Procedure Facade        [public route, validation, modes]
  -> Storage Runtime         [controller, graph access, progress]
  -> Computation Runtime     [ephemeral algorithm state]
```

The exemplar teaches the first level only. The deeper storage/computation doctrine belongs to the algorithm exemplars.

## Namespace Discipline

Procedure examples use the `proc_` prefix and fixture root:

```text
gds/examples/proc_pathfinding_procedure.rs
gds/fixtures/collections/proc/proc_pathfinding_procedure/
```

Algorithm-internal examples use the `algo_` prefix. They are internal architecture material, not application guidance.

The procedure law is strict: application-facing examples call `GraphFacade` and procedure builders. They do not call `::algo::` modules directly.

## Key Vocabulary

- **Procedure Facade** — The public route from application code into a graph algorithm. See [Pathfinding Procedure Facade](../../REFERENCES/procedures/pathfinding-procedure-facade.md).
- **GraphFacade** — The application-facing graph handle that exposes procedure builders.
- **Mode** — A procedure emission form such as `stream`, `stats`, `mutate`, `write`, or `estimate_memory`.
- **Storage Runtime** — The controller that owns graph access, progress, termination, and algorithm loop orchestration.
- **Computation Runtime** — The pure algorithm state holder, without graph access.

## Next Exemplar

Next: `algo_pathfinding_storage_computation.rs` when it exists.

That exemplar should descend one level. It should show how a procedure route hands control to storage, and how storage delegates only state operations to computation. Dijkstra is the recommended first algorithm because its priority-search split is compact and canonical.

## Notes for Students

Watch for the shape of the call, not only the result. The output rows and stats are ordinary Rust values, but the important doctrine is the route that produced them.

Try this: change the exemplar from Random Walk to Dijkstra or Spanning Tree while preserving the same rule. If you need to import `gds::algo::*` in the application example, the exemplar has left Procedure Doctrine and entered Algorithm Doctrine.

Remember: a procedure facade is a man page in code. It tells the caller what can be done; it does not expose the machinery by which the algorithm does it.
