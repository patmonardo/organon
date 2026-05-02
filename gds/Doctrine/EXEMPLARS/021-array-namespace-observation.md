# Exemplar 021 — Array Namespace as Fixed-Arity Observation

**Source file**: `gds/examples/collections_array_namespace_rustscript.rs`
**Arc position**: Observation (fixed-arity mark derivation on structured appearances)
**Prior exemplar**: [020 — Framing and Chunking as PureForm Shape](020-framing-chunking-pureform.md)
**Next exemplar**: [022 — List Namespace as Variable-Arity Observation](022-list-namespace-observation.md)

---

## Principle

An Array is an Observation surface with fixed arity.
Every entity has the same number of elements. The shape is part of the schema.
Operations on Arrays are mark derivations that preserve the fixed-arity contract.

---

## What This Example Does

It builds a DataFrame with a `vals` column (lists of 3 i64) and a `tags` column (lists of 2 strings),
casts them to fixed-arity Arrays, then derives marks:

```rust
arr_ns(gds::col!(vals)).len().alias("len")        // always 3
arr_ns(gds::col!(vals)).sum().alias("sum")        // sum of 3 elements
arr_ns(gds::col!(vals)).sort(false, false).alias("sorted")
arr_ns(gds::col!(vals)).get_expr(gds::col!(idx), true).alias("picked")
arr_ns(gds::col!(tags)).join("-", true).alias("joined")
```

Then demonstrates `GDSSeries + arr_ns` without a DataFrame, for direct Series access.

---

## The Arc Reading

```
Source: a column of fixed-length sequences (vals: [i64; 3], tags: [str; 2])
  → cast to DataType::Array(...)              [Schema commitment: arity is fixed and declared]
  → arr_ns(...).len()                         [Mark: length — trivially fixed, but explicitly confirmed]
  → arr_ns(...).sum()                         [Mark: aggregate across the fixed array]
  → arr_ns(...).sort(...)                     [Mark: ordered form of the same array]
  → arr_ns(...).get_expr(col!(idx), ...)      [Mark: indexed access — positional element as observation]
  → arr_ns(...).join("-", ...)                [Mark: collapsed string form — array→string identity]
```

Each `arr_ns` operation is an Observation derivation: a mark extracted from the fixed structure
of the appearance. The Array arity is the schema commitment made at Source time — it cannot change.

---

## Fixed Arity as Doctrinal Commitment

When a column is cast to `DataType::Array(Box::new(DataType::Int64), 3)`, the number `3`
is a schema commitment. Every entity in that column *must* have exactly 3 elements.
The kernel enforces this. An entity with 4 elements cannot inhabit this column.

This commitment matters for the arc:
- At Source, the format declares the arity (e.g. a corpus of 3-gram token windows)
- At Observation, every derived mark is computed over exactly 3 elements — the derivation is structurally guaranteed
- At Principle, the fixed-arity mark is a reliable condition: `sum >= threshold` is well-defined for all entities

In contrast, a variable-length List (Exemplar 022) carries no such guarantee.
The choice between Array and List is an epistemological decision at Source time:
do we commit to a fixed structure, or do we accept variable structure?

---

## `get_expr(col!(idx), true)` as Positional Observation

```rust
arr_ns(gds::col!(vals)).get_expr(gds::col!(idx), true).alias("picked")
```

`idx` is a column of `[1i64, -1]` — one per row. This indexes into `vals` per entity:
entity 0 picks element at index 1; entity 1 picks element at index -1 (last).
The result is a positional observation: not the whole array, but a specific element.

In the arc, this is the mark `derive <mark> = <expr>` applied to a structural position:
```gdsl
appearance tokens from corpus {
  derive anchor = vals[idx];
}
```
The anchor element is the observation derived from the array's positional structure.

---

## `GDSSeries + arr_ns`: Concept-Level Array Access

```rust
let array_series = GDSSeries::new(vals.cast(&DataType::Array(...))?);
let as_array = array_series.arr().slice(0, 2, true)?;
```

`GDSSeries.arr()` gives direct access to Array namespace operations on a single Series —
the Concept level, without a DataFrame wrapper. This is how the kernel accesses array-typed
Concepts during Principle evaluation: directly, with no intermediate table construction.

---

## Key Vocabulary

**Array (fixed-arity)** — A column where every entity has exactly the same number of elements. Arity is a schema commitment.

**`arr_ns`** — The Array namespace: operations that derive marks from fixed-arity array columns.

**Positional observation** — A mark derived from a specific index position: `get_expr(col!(idx))`.

**Schema commitment** — The declaration at Source time that fixes the structure of all entities in a column.

**`GDSSeries.arr()`** — Direct Concept-level access to Array namespace operations, without a DataFrame.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

---

## Student Notes

- Try removing the `.cast(DataType::Array(...))` calls and re-running. The `arr_ns` operations will fail: they require Array type, not List. This is the kernel enforcing schema commitment. The cast is not optional.
- The `tags` column uses `arr_ns(...).join("-", true)` to produce a string mark from a string array. This is an Observation derivation that collapses a structured array into a single identity token — a common operation at the boundary between structured and text-form appearances.
- `GDSSeries.arr().slice(0, 2, true)` takes the first 2 elements of each array in the Series. Notice it returns a new Series, not a scalar. Array operations preserve the row structure: one result per entity.

---

## What This Example Does Not Show

- Variable-arity list operations (Exemplar 022)
- String namespace operations for text-form marks (Exemplar 023)
- How array columns appear in GDSL `appearance` blocks (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
