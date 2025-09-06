# ADR-0001: Keep DSL as an Embedded HLO Engine

Status
- Accepted (2025-09-03)

Context
- Need a stable core to serve @logic, @model, @task and integrate with @core GraphStore.
- Avoid vendor lock-in; keep logic/graph semantics portable.

Options
- A) Fold DSL into @logic
- B) DSL as standalone engine with adapters (Memory, @core, Neo4j)

Decision
- Choose B. DSL exports a small, stable API (validate, analyze, project, export).
- @logic/@model/@task consume the engine; @core implements GraphStore.

Consequences
- Clear module boundaries; easier testing and reuse.
- One projection to multiple stores.
- Small vocabulary/grammar needs to be curated and documented.
