# ADR-0003: PropertyGraph + Schema API (GraphStore)

Status
- Proposed (2025-09-03)

Context
- DSL must project to a vendor-neutral graph for @logic/@model/@task, with adapters for Memory, @core, Neo4j (Cypher), and later GQL/PGQ.

Decision
- Adopt a minimal TS-first API:
  - Nodes (labels, props, key), Edges (type, from, to, props)
  - Reads: getNode, findNodes, neighbors
  - Writes: upsertNode(s), upsertEdge(s), deleteNode/Edges
  - Optional transactions (beginTx/commit/rollback)
- Separate SchemaStore for node/edge types, keys, constraints.

Consequences
- Stable backbone for algorithms/ML and exports.
- Clear adapter points for @core and Neo4j.
- Schema enforcement lives in adapters or in-memory checks.

Open Questions
- Index API now or defer to adapter capabilities?
- Error semantics on constraint violations?
