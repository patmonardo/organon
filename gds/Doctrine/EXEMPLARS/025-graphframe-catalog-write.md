# Exemplar 025 — GraphFrame Catalog Write as Graph Artifact

**Source file**: `gds/examples/collections_graphframe_catalog_write.rs`
**Arc position**: GraphFrame (graph-structured Procedure artifacts entering the Dataset catalog)
**Prior exemplar**: [024 — Catalog Extension as Artifact Persistence](024-catalog-extension-persistence.md)
**Next exemplar**: [026 — Catalog Polars IO as Typed Artifact Load](026-catalog-polars-io.md)

---

## Principle

A graph is not a special case. It is a Dataset with a relational structure.
GraphFrame writes a graph as three named artifacts: nodes, edges, graph metadata.
Each artifact is a DataFrame-backed Dataset — retrievable, schema-aware, queryable
through the same Judgment and Syllogism operations as any other Dataset.

---

## What This Example Does

The doctrinal statement: "This example shows how graph-structured data can already enter the
Dataset catalog world as GraphFrame-style artifacts."

It:
1. **Builds a random `DefaultGraphStore`** — 8 nodes (Person/Movie labels), KNOWS relationships
2. **Opens a `DatasetCatalog`** — the higher-level semantic catalog
3. **Cleans prior entries** — idempotent re-run support
4. **`write_graph_store_polars32_to_catalog`** — the single write call that produces three artifacts
5. **Loads each artifact** by name and prints the head

The three artifacts: `graphframe-demo.nodes`, `graphframe-demo.edges`, `graphframe-demo.graph`.

---

## The Arc Reading

```
Graph source (DefaultGraphStore, random, 8 nodes)
  → DatasetCatalog::load(&root)          [Open the semantic catalog]
  → write_graph_store_polars32_to_catalog [Procedure: emit nodes, edges, graph metadata]
     → nodes artifact: id, label, properties per node
     → edges artifact: src, dst, type, properties per edge
     → graph artifact: graph-level metadata
  → catalog.load_table(nodes_name)       [Retrieve: nodes as Dataset]
  → catalog.load_table(edges_name)       [Retrieve: edges as Dataset]
  → catalog.load_table(graph_table_name) [Retrieve: graph metadata as Dataset]
  → .head(5)                             [Inspect: first 5 rows of each artifact]
```

The graph traversal in GDS produces edges and node states as Datasets.
GraphFrame is the form that names the graph structure explicitly in the catalog:
not one anonymous table, but three named, structured artifacts with known schemas.

---

## Three Artifacts = One Graph Process

The three-artifact structure is doctrinal. A graph Procedure emits:

| Artifact | Content | Arc role |
|---|---|---|
| `*.nodes` | One row per node: id, label, properties | Concept table — named identities |
| `*.edges` | One row per edge: src, dst, type, properties | Judgment table — relational determinations |
| `*.graph` | Graph-level metadata | Syllogism table — the whole as concluded structure |

Reading this against the arc:
- **Nodes** are Concepts: named, typed identities with fixed properties
- **Edges** are Judgments: determinations of relationship between two Concepts
- **Graph metadata** is the Syllogism conclusion: the whole graph as a concluded structure

When a graph algorithm runs (BFS, PageRank, centrality), its output artifacts follow the same structure.
The nodes table receives the algorithm's per-node conclusions; the edges table records traversal relationships.

---

## `DatasetCatalog` vs. `CatalogExtension`

Exemplar 024 used `CatalogExtension` — the general-purpose catalog facade.
This exemplar uses `DatasetCatalog` — the Dataset-semantic catalog.

| | `CatalogExtension` | `DatasetCatalog` |
|---|---|---|
| Layer | Collections extension | Dataset semantic layer |
| Schema | Inferred or explicit | Dataset-aware |
| Use case | General artifact persistence | GDSL Dataset output |
| Load result | `GDSDataFrame` | `Dataset` |

`DatasetCatalog.load_table(name)` returns a `Dataset`, not a raw DataFrame.
The Dataset carries the semantic layer: it knows its schema, its provenance markers,
and its role in the GDSL arc (nodes / edges / graph metadata).

---

## Idempotent Write Pattern

```rust
let _ = catalog.catalog_mut().remove(&nodes_name);
let _ = catalog.catalog_mut().remove(&edges_name);
let _ = catalog.catalog_mut().remove(&graph_table_name);
catalog.save()?;
```

Before writing, the example cleans prior entries. This is the idempotent pattern:
running the example multiple times produces the same catalog state.
In a GDSL `procedure` block, this corresponds to the `emit` clause replacing prior artifacts
with the current Procedure's outputs — the catalog entry for a given name always reflects
the most recent Procedure run.

---

## Key Vocabulary

**GraphFrame** — A graph-as-Dataset structure: nodes, edges, and metadata as three named, DataFrame-backed artifacts.

**`write_graph_store_polars32_to_catalog`** — The Procedure that emits graph artifacts to the Dataset catalog. Returns paths to the three written files.

**`DatasetCatalog`** — The semantic catalog: loads and stores `Dataset` objects, not raw DataFrames.

**`Dataset.head(n)`** — Inspect the first n rows of a Dataset artifact.

**Idempotent write** — Remove prior entries before writing so multiple runs produce the same catalog state.
*See*: [Artifact Materialization](../REFERENCES/collections-dataset/artifact-materialization.md)

---

## Student Notes

- After running this example, open the `target/collections_graphframe_catalog/` directory. You will find three Parquet files: `*.nodes.parquet`, `*.edges.parquet`, `*.graph.parquet`. These are the raw artifacts. Each can be loaded independently by any Parquet-aware tool — they are not opaque binary formats. This openness is a doctrinal requirement.
- The `RandomGraphConfig` uses `seed: Some(21)` — deterministic randomness. Every run of this example produces the same graph. This matters for testing: a Procedure that reads a graph artifact should always see the same nodes and edges from this example. Determinism is a provenance requirement at Source.
- Notice that `write_graph_store_polars32_to_catalog` takes a `CollectionsIoFormat::Parquet` argument. In a future GDSL `procedure` block: `emit graph dataset format=parquet;` — the format is declared at the procedure level, not hardcoded in the runtime.

---

## What This Example Does Not Show

- Running graph algorithms (BFS, centrality) on the written artifacts (see ML exemplars)
- The `GraphStore` absorption into Collections that is architecturally planned (see [Applications Facade](../REFERENCES/applications/applications-facade.md))
- Full provenance chain from graph algorithm output to named catalog artifact (see [Provenance](../REFERENCES/collections-dataset/provenance.md))
- GDSL `procedure` syntax for graph emission (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
