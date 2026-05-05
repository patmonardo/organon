# Exemplar 017 — Streaming Dataset as Procedure

**Source file**: `gds/examples/dataset_streaming_procedure.rs`
**Arc position**: Procedure (Inference → Procedure → Process emission)
**Prior exemplar**: [016 — Join Operations as Inference](../dataframe/016-join-operations-inference.md)
**Next exemplar**: [018 — Streaming Lazy as Deferred Collection](../dataset/018-streaming-lazy-deferred.md)

---

## Principle

Procedure is the entry into Objectivity. It emits Process — not dead output.
The Streaming Dataset is Procedure in motion: the Dataset is the semantic source,
and streaming exposes its analytic body as a living, batch-traversable Process.

---

## What This Example Does

The doctrinal statement arrives in the first `println!`:
> "A Dataset is the semantic source; streaming exposes its analytic body in motion."

The example then:

1. **Builds a Dataset** from a raw table — fixing the semantic source
2. **Wraps it in `StreamingDataset::with_config`** with a `LazyFrame` transform — the Procedure body
3. **Applies the transform**: derives `weighted_score`, marks `is_high`, filters, groups, aggregates
4. **Iterates**: calls `.iter().next()` to collect the first (semantic summary) batch
5. **Streams raw batches**: iterates without transform, printing each batch as an "analytic body slice"

The two streaming modes — transformed summary and raw batch — are two emission registers of the same Procedure:
- The transformed summary is the **Process conclusion**: the final artifact for downstream use
- The raw batch stream is the **Process trace**: the material record of how the Dataset was traversed

---

## The Arc Reading

```
Inference results (cross-population conclusions available)
  → Dataset::new(table)                    [Fix the semantic source]
  → StreamingDataset::with_config(...)     [Wrap as Procedure body]
  → .with_transform(|lazy| {              [Define the Process stages:]
       weighted_score derivation,           → mark derivation
       is_high classification,              → Judgment application
       filter!(score > 12.0),              → Principle re-application
       group_by!(is_high) + agg!(...)       → Syllogism (final grouping)
    })
  → .iter().next()                         [Emit: first batch = Process conclusion]
  → raw batches: for (index, batch)        [Emit: trace = Process record]
```

The Procedure is not a loop over data. It is the traversal of a semantic body in stages.
Each stage in the transform corresponds to an arc moment re-applied at the emission level.

---

## Dataset as Semantic Source

```rust
let dataset = Dataset::new(table);
println!("Dataset rows: {}", dataset.row_count());
println!("Dataset columns: {:?}", dataset.column_names());
```

The Dataset knows its own shape before any streaming begins.
It is not an anonymous table. It has a structure, a row count, named columns.
This is the semantic knowledge the Procedure inherits from the Dataset specification.
In a fully-specified external `.gdsl` artifact, the Dataset was built by the compiler from a `procedure` block —
the `emit <stage> dataset` clauses determine exactly what the Dataset contains before streaming starts.

---

## Two Emission Registers

### Transformed Summary (Process Conclusion)
```rust
let streaming_summary = StreamingDataset::with_config(dataset.clone(), 3, config)
    .with_transform(|lazy| { ... });
let summary = streaming_summary.iter().next()...;
```
This is the final artifact: the concluded Process output. One batch. Fully derived.
A reader can inspect this and understand what the program concluded.

### Raw Batch Stream (Process Trace)
```rust
let streaming_batches = StreamingDataset::new(dataset, 3);
for (index, batch) in streaming_batches.iter().enumerate() {
    println!("Batch @{} (analytic body slice):\n{}", offset, ...);
}
```
This is the trace: how the Dataset was traversed, batch by batch, at offset 0, 3, 6.
A reader can inspect this and understand what material was processed at each step.
The offset is the provenance marker — it locates each batch in the original semantic body.

Together, these two registers are the Process: conclusion plus trace, inseparable.

---

## `StreamingConfig` as Process Policy

```rust
StreamingConfig { enable_new_streaming: false }
```

The `StreamingConfig` is the Process policy: it governs how the kernel executes the streaming.
This is the analog of the external `.gdsl` `procedure` block's configuration clauses.
In a future fully-specified system, the policy will also include:
- Whether to preserve batch provenance in artifact tables
- Whether to allow parallel batch execution
- Whether to emit intermediate stage artifacts or only the final conclusion

---

## Key Vocabulary

**Procedure** — The arc stage that emits Objectivity as Process. Not a function call — a traversal of a semantic body that produces a complete Process record.
*See*: [Hegel Objectivity — Procedure as Process](../../REFERENCES/philosophy/hegel-objectivity.md)

**Dataset** — The semantic source for a Procedure. Its structure is fixed before the Procedure runs.
*See*: [Core Concepts — Dataset](../../REFERENCES/dataset/core-concepts.md)

**`StreamingDataset`** — The Procedure body: a Dataset wrapped in a traversal mechanism with optional transformation.
*See*: [Core Concepts — Pipeline](../../REFERENCES/dataset/core-concepts.md)

**Process conclusion** — The final transformed batch: the artifact that records what the Procedure concluded.

**Process trace** — The raw batch stream: the material record of how the semantic body was traversed.

**`StreamingConfig`** — The process policy governing kernel execution strategy.

---

## Student Notes

- The example is gated by `#![cfg(feature = "dataset")]`. This is doctrinal: not every program needs the full Dataset streaming facility. The feature gate marks the boundary between the lightweight DataFrame world and the full Dataset-as-semantic-source world.
- Study the `with_transform` closure. It is the internal RustScript equivalent of an external `.gdsl` artifact: derivation → classification → filter → group → aggregate. Every step is an arc moment. The fact that this compiles to a lazy plan and executes in one pass is the ZeroCopy principle at the Procedure level.
- Ask: what is the difference between the transformed summary and the raw batch stream? The summary is what you report. The batch stream is your provenance. Both are required for a defensible scientific artifact.

---

## What This Example Does Not Show

- External `.gdsl` `procedure` block syntax with `emit <stage> dataset` (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
- Artifact table materialization and provenance recording (see [Artifact Materialization](../../REFERENCES/dataset/artifact-materialization.md))
- Deferred lazy collection for large-scale Procedures (Exemplar 018)
