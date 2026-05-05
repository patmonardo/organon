# Exemplar 024 — Catalog Extension as Artifact Persistence

**Source file**: `gds/examples/dataset_io_catalog_extensible.rs`
**Arc position**: Artifact persistence (Procedure → named, retrievable Dataset artifacts)
**Prior exemplar**: [023 — String Namespace as Text-Form Observation](../dataset/023-string-namespace-observation.md)
**Next exemplar**: [025 — GraphFrame Catalog Write as Graph Artifact](../dataset/025-graphframe-catalog-write.md)

---

## Principle

A Catalog is where Procedure outputs become retrievable scientific artifacts.
Not a file system — a named, schema-aware store where each entry carries identity,
format, and schema metadata.
Write once; read with structural verification.

---

## What This Example Does

It creates a `CatalogExtension` rooted at `target/collections_catalog_extensible`,
configures it with Parquet format, schema inference, and auto-save, then runs a pipeline:

1. **`catalog.write_table("sample_table", &table, None)`** — persist the raw artifact
2. **`catalog.read_table("sample_table")`** — retrieve it by name
3. **`scale_f64_column(&mut table, "score", 2.0)`** — transform (a post-Procedure derivation)
4. **`catalog.write_table("sample_table_processed", ...)`** — persist the derived artifact
5. **Select, filter, group** — further analytic operations on the retrieved artifact
6. **`catalog.list()`** — inspect the catalog: names, formats, schema metadata

---

## The Arc Reading

```
Procedure emitted Process artifacts (tables with provenance)
  → CatalogExtension::new(&root)        [Open the named artifact store]
  → catalog.write_table(name, table)    [Persist: artifact enters the catalog under its name]
  → catalog.read_table(name)            [Retrieve: artifact is loaded by name with schema check]
  → scale, select, filter, group        [Post-retrieval analytic operations]
  → catalog.write_table(new_name, ...)  [Persist derived artifact under a new name]
  → catalog.list()                      [Inspect: names, formats, schemas]
```

The Catalog is the persistence layer of the arc. It is not part of the core pipeline —
it is the store that makes Procedure outputs durable and named across program runs.

---

## `CatalogExtensionConfig` as Artifact Policy

```rust
CatalogExtensionConfig {
    default_format: CollectionsIoFormat::Parquet,
    infer_schema_on_write: true,
    infer_schema_on_read: true,
    validate_on_read: false,
    eager: true,
    auto_save: true,
    ..Default::default()
}
```

Each field is a policy decision about how artifacts are treated:

| Field | Policy decision |
|---|---|
| `default_format: Parquet` | All artifacts stored in columnar, compressed format |
| `infer_schema_on_write` | The schema is captured at write time from the table |
| `infer_schema_on_read` | The schema is re-read at load time for structural awareness |
| `validate_on_read: false` | Schema violations are not errors on read (loose retrieval) |
| `eager: true` | Retrieval materializes immediately (no lazy deferred loading) |
| `auto_save: true` | Catalog metadata is persisted automatically after each write |

In an external `.gdsl` artifact, the `procedure` block's `emit <stage> dataset` clause implies a specific
Catalog policy: which stage artifacts are written, in what format, with what schema policy.

---

## Schema as Artifact Identity

```rust
fn print_catalog(catalog: &CollectionsCatalogDisk) {
    for entry in catalog.list() {
        println!("- {} ({:?})", entry.name, entry.io_policy.format);
        if let Some(schema) = &entry.schema {
            let columns = schema.fields.iter()
                .map(|f| format!("{}:{:?}", f.name, f.value_type))
                .collect::<Vec<_>>();
            println!("  schema columns: {}", columns.join(", "));
            println!("  note: this is schema metadata, not a row-bearing table.");
        }
    }
}
```

The Catalog does not just store tables. It stores schema metadata separately from row data.
The comment — "this is schema metadata, not a row-bearing table" — is doctrinal:
the schema is the artifact's identity; the rows are the artifact's content.
You can inspect an artifact's identity (schema) without loading its content (rows).

This separation is a provenance requirement: a reader can understand what an artifact is
before loading it. Format, column names, types — all are available in the catalog entry.

---

## Two Named Artifacts

After the pipeline, the catalog contains two entries:
- `sample_table` — the raw artifact (unscaled scores)
- `sample_table_processed` — the derived artifact (scores × 2)

Both are named, both have schemas, both are retrievable independently.
The derivation relationship between them is implicit in their names — in a full provenance system,
it would be explicit: `sample_table_processed` would carry a provenance record pointing to `sample_table`
and the `scale_f64_column` operation that produced it.
*See*: [Provenance](../../REFERENCES/dataset/provenance.md)

---

## Key Vocabulary

**Catalog** — The named, schema-aware artifact store. Persists Procedure outputs across program runs.

**`CatalogExtension`** — The GDS Collections catalog facade. Wraps the `CollectionsCatalogDisk` with configuration and convenience methods.

**`write_table(name, table)`** — Persist an artifact under a name. The name is the artifact's identity in the catalog.

**`read_table(name)`** — Retrieve an artifact by name. Schema metadata is available before rows are loaded.

**Schema metadata** — The artifact's structural identity: column names and types. Stored separately from row data.

**`CollectionsIoFormat::Parquet`** — The default artifact format: columnar, compressed, schema-embedded.
*See*: [IO and Readers](../../REFERENCES/dataset/io-and-readers.md)

---

## Student Notes

- Run the example twice. On the second run, the catalog already contains `sample_table` from the first run. Notice that `auto_save: true` means the catalog metadata is updated on each write. The raw table and processed table accumulate across runs — the catalog is persistent state.
- The `print_catalog` function shows the catalog entries with schema but without loading rows. Practice reading the schema output and asking: what Concept does each column represent? What Judgment produced the schema structure? The schema is the Concept surface made persistent.
- `validate_on_read: false` means a changed schema on disk will not cause a load error. This is a deliberate loose policy for exploratory work. In a production Shell pipeline, set `validate_on_read: true` to enforce that retrieved artifacts match the declared schema. Schema drift is an integrity violation.

---

## What This Example Does Not Show

- The `DatasetCatalog` (the higher-level semantic catalog used in Exemplar 025)
- Provenance links between derived artifacts (see [Provenance](../../REFERENCES/dataset/provenance.md))
- External `.gdsl` `procedure` block's `emit <stage> dataset` syntax (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
- Graph artifact persistence (Exemplar 025)
