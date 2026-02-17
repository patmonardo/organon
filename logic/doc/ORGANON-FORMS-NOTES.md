# Organon Forms Notes

Status: supporting notes for `ORGANON-FORMS.md`.

## Scope

This file collects optional and downstream material that is intentionally kept
out of the core forms language document.

## Projection to External Standards (Optional)

Projection is interoperability-oriented and not the source of truth.

| Organon Form | RDFS                | OWL                                          | SHACL                     | SPARQL/SPIN              | SKOS                     |
| ------------ | ------------------- | -------------------------------------------- | ------------------------- | ------------------------ | ------------------------ |
| Entity       | Class, subClassOf   | equivalentClass, disjointWith                | targetClass               | class-scoped views/rules | ConceptScheme (optional) |
| Property     | domain/range        | ObjectProperty/DatatypeProperty, cardinality | property path constraints | derive/normalize updates | lexical term links       |
| Aspect       | relation predicates | property characteristics                     | node/property constraints | relation infer rules     | concept relation hints   |
| Constraint   | basic range checks  | logical consistency checks                   | primary execution target  | rule guard conditions    | vocabulary policy checks |
| Rule         | n/a                 | SWRL-adjacent semantics                      | SHACL rules (profile)     | primary execution target | n/a                      |
| Vocabulary   | labels              | annotations                                  | label constraints         | term extraction queries  | primary execution target |
| Provenance   | metadata predicates | annotation properties                        | trace shape checks        | audit query surfaces     | editorial lineage        |

## Notes

- Core authoring order remains in `ORGANON-FORMS.md`:
  1. `Form/Context/Morph`
  2. `Entity/Property/Aspect` instantiation
- Projection can evolve independently without changing core language rules.

## Implementation Notes (Working)

These notes are implementation-oriented and intentionally kept out of the core
forms language document.

- `Neo4j/Cypher`: primary graph query and execution substrate.
- `FormDB`: semantic control layer orchestrating form/context/morph execution.
- `Postgres`: durable grounding and supporting memory.
- `Polars`: high-throughput analytics over large graph-derived datasets.

Boundary rule:

- Treat compiler semantics, form engine internals, and storage/runtime topology
  as separate tracks; evolve one track at a time.

## Likely Way Forward (Current Direction)

- `@logic`: Cypher-first execution engine.
- `@model`: Prisma-first application/data engine.
- `FormDB`: manages the three-store model (`FactStore` / `RelationStore` / `KnowledgeStore`).

Operational split:

- Graph semantics, traversal, and state transitions remain Cypher-first.
- `@model` apps use Postgres/Prisma for memory-augmented support data.
- Dashboard-facing application flows are primarily `Entity/Property/Aspect` focused.
