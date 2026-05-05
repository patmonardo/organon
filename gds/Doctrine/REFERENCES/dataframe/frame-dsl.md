# Frame DSL

The Frame DSL is the ergonomic surface where Dataset authoring stays readable while DataFrame execution remains Polars-backed.

---

## Doctrine

Dataset names intent. DataFrame executes. The Frame DSL keeps those roles together without collapsing them.

---

## Use

Use Frame DSL operations when a semantic Dataset needs ordinary tabular work: columns, expressions, filtering, projection, aggregation, and compilation into a `DatasetCompilation` graph.

---

## Review Rule

Frame DSL examples should show the Dataset boundary clearly. If an example reads like raw Polars with no semantic name, it has fallen below Doctrine level.
