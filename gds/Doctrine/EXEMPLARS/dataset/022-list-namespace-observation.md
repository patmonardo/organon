# Exemplar 022 — List Namespace as Variable-Arity Observation

**Source file**: `gds/examples/dataset_namespace_list.rs`
**Arc position**: Observation (variable-arity mark derivation on open-structure appearances)
**Prior exemplar**: [021 — Array Namespace as Fixed-Arity Observation](../dataset/021-array-namespace-observation.md)
**Next exemplar**: [023 — String Namespace as Text-Form Observation](../dataset/023-string-namespace-observation.md)

---

## Principle

A List is an Observation surface with variable arity.
Entities may have zero, one, or many elements. No structural commitment is made at Source time.
Operations on Lists must handle the variability — including the empty list.

---

## What This Example Does

It builds a DataFrame with a `values` column containing lists of varying length —
`[1,2,3]`, `[3,3,2]`, and `[]` — and derives marks:

```rust
list_ns(gds::col!(values)).len().alias("len")           // 3, 3, 0
list_ns(gds::col!(values)).sum().alias("sum")           // 6, 8, null
list_ns(gds::col!(values)).unique(true).alias("unique") // deduped lists
list_ns(gds::col!(values)).get(0, true).alias("first")  // first element or null
list_ns(gds::col!(values)).contains(2_i64, true).alias("has_2")  // membership test
```

Then demonstrates `GDSSeries + list_ns` directly, without a DataFrame.

---

## The Arc Reading

```
Source: a column of variable-length sequences (values: Vec<i64>)
  → list_ns(...).len()           [Mark: how many elements — may be 0]
  → list_ns(...).sum()           [Mark: aggregate — null if empty]
  → list_ns(...).unique(true)    [Mark: deduplicated form — structural reduction]
  → list_ns(...).get(0, true)    [Mark: first element — null if empty]
  → list_ns(...).contains(2, ..) [Mark: membership — boolean, always defined]
```

The `null` results for the empty list are not errors. They are correct observations:
an empty list has no sum, no first element, no deduplication to perform.
Null is the doctrinal answer for an entity that structurally cannot participate in a mark derivation.

---

## The Empty List as Doctrinal Case

```
values = []  →  len=0, sum=null, first=null, has_2=false, unique=[]
```

The empty list entity exists — it passed Source and entered Observation.
But it contributes `null` to most marks. This matters at Principle:

> An entity with `sum = null` may fail a Principle condition like `require sum >= 5`.
> That is correct: the empty-list entity cannot become a Concept under a sum-based Principle.
> It is not destroyed — it is a valid entity that did not pass the nomological gate.

This is the doctrinal meaning of null in the arc: not missing data, but a structurally
defined absence that has consequences at Principle.

---

## Array vs. List: The Epistemological Choice

| | Array (Exemplar 021) | List (Exemplar 022) |
|---|---|---|
| Arity | Fixed (declared at Source) | Variable (open at Source) |
| Schema | Committed | Open |
| Null marks | Not possible (arity enforced) | Possible (empty list → null) |
| Best for | Token windows, fixed-feature vectors | Sentences, tag sequences, corpora |
| Principle risk | None (all marks well-defined) | Null marks may fail Principle |

Choose Array when the structural contract is known and should be enforced.
Choose List when the data is inherently variable and nulls must be handled in Principle.

---

## `GDSSeries.list()`: Concept-Level List Access

```rust
let series = GDSSeries::from_list_i64("values", &[vec![1,2,3], vec![3,3,2], vec![]]);
let lengths = series.list().len()?;
let uniques = series.list().unique(true)?;
```

As with Array, the List namespace is accessible directly on a `GDSSeries` without a DataFrame.
This is how the kernel inspects list-typed Concepts during Principle evaluation:
`series.list().len()` gives the length Series — one count per entity — for condition checking.

---

## Key Vocabulary

**List (variable-arity)** — A column where entities may have zero or more elements. No arity commitment at Source.

**`list_ns`** — The List namespace: operations that derive marks from variable-arity list columns.

**Null (structural absence)** — A null in a List mark is not missing data. It is the correct observation for an entity that structurally cannot participate in the derivation (e.g., `sum` of an empty list).

**Membership test** — `list_ns(...).contains(value)` — a boolean mark: does this entity's list contain the target? Always defined, even for empty lists (returns `false`).

**`GDSSeries.list()`** — Direct Concept-level access to List namespace operations.
*See*: [Frame DSL](../../REFERENCES/dataframe/frame-dsl.md)

---

## Student Notes

- The three-row example with `[]` (empty list) is the doctrinal test case. Always include an empty-list entity when testing List operations. It will reveal which marks produce null and which produce valid results. Plan your Principle conditions around these nulls.
- `contains(2_i64, true)` returns `false` for the empty list — not `null`. This is the one List operation that is always defined: asking whether something is in an empty set has a clear answer (no). Keep this distinction in mind when choosing which marks to build Principle conditions on.
- `unique(true)` on `[3,3,2]` produces `[3,2]` — order-preserving deduplication. The result is still a List, not a scalar. List operations on List columns produce List results, not reductions, unless the operation explicitly aggregates (like `.sum()`).

---

## What This Example Does Not Show

- String-form observations (Exemplar 023) — the text analog of List with Unicode considerations
- How null List marks propagate through Principle evaluation (see [Principle Evaluation](../../REFERENCES/form/principle-evaluation.md))
- External `.gdsl` `appearance` block with `derive` over list columns (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
