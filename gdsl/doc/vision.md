# DSL Vision — Anchor of the Logic–Model–Task Cube

Goal
- DSL is a standalone HLO engine and schema-first PropertyGraph API.
- @logic, @model, @task are connectors/hosts; @core provides a general GraphStore.

Principles
- Engine-first: parse → validate → analyze → project (graph) → export.
- Vendor-neutral: TS PropertyGraph API; adapters for Memory, @core, Neo4j/Cypher; GQL later.
- TopicModel alignment: Chunk (Document) → summary/digest → HLOs (tokens) → Concepts (Topics/datasets).
- Dialectical processing: EssentialRelations + Reflective Marks drive operators and datasets.

Key Constructs
- Chunk(id, title, text, summary, digest)
- HLO(id, chunkId, clauses[predicates/tags])
- Concept = named graph dataset
- GraphStore: nodes/edges with schema, keys, props
- Operators: HLO predicates → typed graph relations
- Features/Index: predicate/tag vectors, KV indexes

Outcomes
- Readable outline (titles + summaries)
- Validated HLO vocabulary
- Graph projection for analytics/ML
- Neo4j/GQL export path without lock-in
