# Exemplar 020 — Framing and Chunking as PureForm Shape

**Source file**: `gds/examples/collections_extensions_framing_chunking.rs`
**Arc position**: PureForm / Kernel Shape (the structural substrate beneath Dataset and DataFrame)
**Prior exemplar**: [019 — Expr as Reusable Judgment Grammar](019-expr-basic-judgment-grammar.md)
**Next exemplar**: [021 — Array Namespace as Tensor Surface](021-array-namespace-tensor.md)

---

## Principle

Before there is a Dataset, before there is a DataFrame, there is Shape.
Framing and Chunking are the two most primitive structural operations:
they fix the geometric and partitional form of a collection before any semantic content is assigned.
This is PureForm at the Collections level.

---

## What This Example Does

Two entirely independent demonstrations, no semantic content:

### Framing
```rust
let framing_values: Vec<i32> = (1..=9).collect();
let framing_base = VecInt::from(framing_values);
let mut framed = FrameCollection::new(framing_base);
framed.enable_framing(FramingConfig {
    shape: FrameShape { rows: 3, cols: 3, order: FrameOrder::RowMajor },
    strict_bounds: true,
})?;
let cell = framed.get_cell(1, 1)?;   // → 5
let row  = framed.row_values(1)?;    // → [4, 5, 6]
let col  = framed.col_values(2)?;    // → [3, 6, 9]
```

Nine integers are given a 3×3 shape. No semantics. Just form.

### Chunking
```rust
let chunking_values: Vec<i32> = (1..=12).collect();
let chunked = ChunkedCollection::new(chunking_base);
chunked.enable_chunking(ChunkingConfig {
    chunk_size: 4,
    max_chunks: None,
    prefer_power_of_two: false,
    allow_tail: true,
})?;
chunked.for_each_chunk(|range, values| {
    println!("  chunk {:?} => {values:?}", range);
})?;
```

Twelve integers are partitioned into chunks of 4. No semantics. Just partition.

---

## The Arc Reading

```
Below Observation (before the appearance has semantic content)
  → VecInt::from(values)        [Raw material: a flat sequence]
  → FrameCollection::new(...)   [PureForm wrapper: the flat sequence becomes a form-capable body]
  → enable_framing(config)      [Shape assignment: rows × cols × order]
  → get_cell / row_values / col_values  [Morph: access the formed material through its shape]

  → ChunkedCollection::new(...) [PureForm wrapper: the flat sequence becomes a chunk-capable body]
  → enable_chunking(config)     [Partition assignment: chunk_size, allow_tail]
  → for_each_chunk(...)         [Morph: traverse the chunked body]
```

Both `FrameCollection` and `ChunkedCollection` are Form/Context/Morph triads:
- **Form**: the raw values (what is given)
- **Context**: the configuration (rows/cols/order, chunk_size/policy)
- **Morph**: the shaped access (get_cell, row_values, for_each_chunk)

---

## PureForm: Form Before Semantics

The GDS kernel's PureForm principle is that shape is prior to content.
A 3×3 grid of integers is not yet semantic — it has no meaning assigned to any cell.
But it has form: a cell at (1,1) is determinately positioned; a row is determinately bounded.

This formedness is what the kernel provides before the Dataset layer adds meaning.

When a GDSL program declares `source corpus : parquet("path/to/data.parquet")`,
the kernel loads the file as shaped material — rows and columns with types but no GDSL semantics yet.
Framing and chunking are the primitive operations that shape that material before semantic assignment begins.

---

## `FramingConfig` as Shape Specification

```rust
FramingConfig {
    shape: FrameShape { rows: 3, cols: 3, order: FrameOrder::RowMajor },
    strict_bounds: true,
}
```

`FrameShape` is a minimal kernel-level schema: rows, columns, traversal order.
`strict_bounds: true` means the kernel will reject any access outside the declared shape.
This is the kernel enforcing form integrity before any semantic layer is activated.

`FrameOrder::RowMajor` specifies traversal order — this determines how cell indices
map to flat memory positions. Row-major: row 0 occupies positions 0..cols-1.
This is the zero-copy contract: the kernel accesses the flat Vec directly at computed offsets,
with no copy and no intermediate representation.

---

## `ChunkingConfig` as Partition Specification

```rust
ChunkingConfig {
    chunk_size: 4,
    max_chunks: None,
    prefer_power_of_two: false,
    allow_tail: true,
}
```

Chunking partitions the flat sequence into fixed-size ranges.
`allow_tail: true` means the last chunk may be smaller than `chunk_size`.
This tail policy is the kernel's acknowledgment that real data does not always divide evenly.

The `for_each_chunk` traversal visits each range with its values — a zero-copy slice view.
This is the primitive form of the streaming batch traversal in Exemplar 017,
brought down to its simplest possible expression.

---

## The Form/Context/Morph Triad

| Component | Framing | Chunking |
|---|---|---|
| **Form** | `VecInt` (flat values) | `VecInt` (flat values) |
| **Context** | `FramingConfig` (rows, cols, order, bounds) | `ChunkingConfig` (chunk_size, policy) |
| **Morph** | `get_cell`, `row_values`, `col_values` | `for_each_chunk` |

Every shaped collection in the GDS kernel follows this triad.
DataFrame, GraphFrame, StreamingDataset — all of them are Form/Context/Morph triads
with richer semantic content layered on top of these primitive operations.
*See*: [PureForm](../REFERENCES/gds-kernel/pureform.md)

---

## Key Vocabulary

**PureForm** — The kernel principle that shape is prior to content. Form is assigned before semantics.
*See*: [PureForm](../REFERENCES/gds-kernel/pureform.md)

**`FrameCollection`** — A shaped collection: flat material given rows × cols form.

**`ChunkedCollection`** — A partitioned collection: flat material divided into fixed-size ranges.

**`FrameShape`** — The geometric specification: rows, columns, traversal order.

**Form/Context/Morph triad** — The three-component structure of every shaped collection in the kernel.
*See*: [GDS Kernel — ZeroCopy Boundary](../REFERENCES/gds-kernel/zeroCopy-boundary.md)

**Zero-copy access** — Direct index computation over flat memory with no intermediate allocation.
The kernel computes `cell(r, c) = flat[r * cols + c]` without copying.

---

## Student Notes

- Run this example and change `FrameOrder::RowMajor` to `FrameOrder::ColMajor` (if available). Observe that `cell(1,1)` now maps to a different flat offset. The shape changed; the values did not. This is what PureForm means: the form is separate from the content.
- The `strict_bounds: true` option will cause a panic or error if you request `get_cell(5, 0)` on a 3×3 frame. Try it. That error is the kernel enforcing form integrity. The kernel does not allow shape violations.
- Notice that neither framing nor chunking imports anything from the Dataset or DataFrame modules. They are pure kernel extensions. This layering is intentional: PureForm is below all semantic layers. A student who understands framing and chunking can understand any shaped access in the system.

---

## What This Example Does Not Show

- How `FrameShape` connects to the ZeroCopy boundary at Absolute Reflection (see [ZeroCopy Boundary](../REFERENCES/gds-kernel/zeroCopy-boundary.md))
- How chunking maps to the batch streaming in Exemplar 017
- The full PureForm triad in the GDS kernel's graph algorithm layer (see [PureForm](../REFERENCES/gds-kernel/pureform.md))
