# Exemplar 018 — Streaming Lazy as Deferred Collection

**Source file**: `gds/examples/collections_streaming_lazy.rs`
**Arc position**: Procedure (deferred execution / lazy Process)
**Prior exemplar**: [017 — Streaming Dataset as Procedure](017-streaming-dataset-procedure.md)
**Next exemplar**: [019 — Expr as Reusable Judgment Grammar](019-expr-basic-judgment-grammar.md)

---

## Principle

A lazy pipeline is a Procedure whose Process is deferred until collection.
Nothing is materialized until `.collect_streaming_lazy()` is called.
This deferral is not laziness — it is the architectural expression of the kernel's
right to delay materialization until the full plan is known.

---

## What This Example Does

It is the minimal form of a streaming Procedure:

1. **Build a table** — the raw material
2. **`GDSStreamingFrame::from(table)`** — wrap as a kernel streaming context
3. **`streaming.enable_streaming(config)`** — activate the streaming policy
4. **`GDSLazyFrame::new(...).select_exprs(...).filter(...)`** — build the plan without executing it
5. **`streaming.collect_streaming_lazy(lazy)`** — collect: execute the full plan in one kernel pass

The result is a fully determined DataFrame — but nothing was evaluated until step 5.

---

## The Arc Reading

```
Procedure plan (all arc stages defined)
  → GDSStreamingFrame::from(table)         [Kernel streaming context: semantic body wrapped]
  → streaming.enable_streaming(config)     [Process policy activated]
  → GDSLazyFrame::new(...)                 [Plan begins: no execution yet]
       .select_exprs([id, score])          [Projection plan]
       .filter(score > 20.0)               [Principle plan]
  → streaming.collect_streaming_lazy(...)  [Collect: execute full plan, emit result]
```

Between plan construction and collection, the kernel holds the complete Process specification.
It can optimize across all stages before emitting a single byte.
This is the architectural benefit of lazy evaluation at the Procedure level.

---

## Why Lazy Matters for the Doctrine

In the arc, a Procedure emits a Process. The Process is a complete, inspectable record.
But the kernel cannot know the optimal execution plan for the Process
until the full arc — reflection, principle, concept, judgment, syllogism, inference — is declared.

A lazy pipeline is how the kernel receives the full plan before executing any of it.

Concretely:

```rust
let lazy = GDSLazyFrame::new(streaming.stream_lazy())
    .select_exprs(vec![gds::col!(id), gds::col!(score)])
    .filter(gds::expr!(score > 20.0));

let df = streaming.collect_streaming_lazy(lazy.into_lazyframe())?;
```

`lazy` is a plan object. It is not a result. `.into_lazyframe()` converts it to the kernel's
internal representation. `collect_streaming_lazy` executes the plan and materializes the result.

The doctrinal point: **the plan is the Procedure; the collected result is the Process artifact**.

---

## `GDSStreamingFrame` as Kernel Context

`GDSStreamingFrame` is the kernel-side streaming context. It wraps the raw DataFrame
and activates the streaming execution engine. It knows:

- The underlying data shape (via `stream_lazy()`)
- The streaming policy (`StreamingConfig`)
- How to collect a lazy plan into a materialized result

This is the Rust-side kernel counterpart to the TS-side Relative Form Processor.
The Form Processor builds plans (lazy); the kernel collects them.

---

## Minimal Form as Doctrinal Purity

This example is deliberately minimal — two columns, one filter, one collect.
That minimality is the point: a Procedure does not need to be complex to be a Procedure.
Even the simplest lazy-collect cycle is a complete doctrinal arc:

> Semantic body (streaming frame) → plan declared (lazy) → plan executed (collect) → artifact emitted.

Every larger Procedure in the system is this pattern, extended.

---

## Key Vocabulary

**Lazy pipeline** — A Procedure plan: the full sequence of arc stages declared without execution. Not evaluated until collected.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

**`GDSStreamingFrame`** — The kernel streaming context. Wraps a DataFrame and activates streaming execution.

**`GDSLazyFrame`** — The plan builder. Receives select, filter, and other arc-stage declarations without executing them.

**`.collect_streaming_lazy()`** — The collection point: where the full plan is handed to the kernel for optimized execution, and the Process artifact is emitted.

**Deferred materialization** — The kernel's right to withhold execution until the full plan is known. Required for ZeroCopy optimization across stages.
*See*: [ZeroCopy Boundary](../REFERENCES/gds-kernel/zeroCopy-boundary.md)

---

## Student Notes

- Compare this example to Exemplar 017. In 017, the transform is embedded in `StreamingDataset::with_config`. In 018, the plan is built explicitly as a `GDSLazyFrame`. Both are Procedures. The difference is the level of plan visibility: 018 makes the plan a first-class object you can inspect, pass around, and compose.
- The `into_lazyframe()` call is the transition from the GDS lazy abstraction to the kernel's internal Polars `LazyFrame`. This is the boundary where the GDS semantic layer hands off to the raw Polars execution engine. It is the architectural seam between the semantic world (GDS) and the execution world (Polars).
- A future GDSL compiler will emit `GDSLazyFrame` plans automatically from `procedure` blocks. The handwritten plans in this example are what the compiler will generate.

---

## What This Example Does Not Show

- Multi-stage transforms with derivation and grouping (Exemplar 017)
- Provenance recording during lazy collection (see [Artifact Materialization](../REFERENCES/collections-dataset/artifact-materialization.md))
- The full GDSL `procedure` block that generates this plan (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
