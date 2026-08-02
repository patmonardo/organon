Shell Control Protocol Fixture

Namespace: shell::compute

Doctrinal Method Topic Map
This directory is authored to read as a top-level software design map:
DataFrame -> TaskFrame through shell/proc/eval mediation under one root authority.

00 Frame
artifact: fixtures/collections/shell/shell_compute_protocol/00-frame.csv
meaning: DataFrame seed for the graph relation surface.

00 Graph
artifact: fixtures/collections/shell/shell_compute_protocol/00-graph.txt
meaning: in-memory graph store with explicit adjacency and edge listings.

01 Plan
artifact: fixtures/collections/shell/shell_compute_protocol/01-shell-plan.txt
meaning: Shell internal DSL plan for a BFS traversal call.

01 Program Features
artifact: fixtures/collections/shell/shell_compute_protocol/01-program-features.txt
meaning: mediation mapping from shell_ to proc_ to eval_ under one TaskFrame root.

02 Result
artifact: fixtures/collections/shell/shell_compute_protocol/02-shell-result.txt
meaning: procedure runtime output with full BFS row exposition.

02 Return
artifact: fixtures/collections/shell/shell_compute_protocol/02-shell-return.txt
meaning: shell return register and readiness state for reflective reuse.

03 Pureform Principle
artifact: fixtures/collections/shell/shell_compute_protocol/03-pureform-principle.txt
meaning: doctrinal account of why this path is a valid mediation of algorithmic knowledge.

Summary Map
Kernel   -> 00-frame.csv, 00-graph.txt, 02-shell-result.txt
Agent    -> 01-shell-plan.txt, 02-shell-return.txt
Logic    -> 01-program-features.txt, 03-pureform-principle.txt
