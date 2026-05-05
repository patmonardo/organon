# Exemplar 013 — Series as Concept Surface

**Source file**: `gds/examples/dataframe_series_concept.rs`
**Arc position**: Concept (Principle → Concept emergence)
**Prior exemplar**: [012 — Select/Filter as Principle Evaluation](../dataframe/012-select-filter-principle.md)
**Next exemplar**: [014 — Expr Pipeline as Judgment](../dataframe/014-expr-pipeline-judgment.md)

---

## Principle

The Series is the Concept's material substrate.
A Concept is a named, typed, homogeneous identity with determined operations.
The Series enacts that: one name, one type, one population, fully determined.

---

## What This Example Does

It builds three Series by name and type — `values` (i64), `weights` (f64), `mask` (bool) —
assembles them into a DataFrame, and then demonstrates deterministic operations:

1. **`gds::col!(values).abs()`** — absolute value of the identity surface
2. **`.sum()`, `.mean()`** — aggregate measures of the Concept population
3. **`where_!(df, mask)`** — filter by a boolean identity gate
4. **`arrange!` + `.head(2)`** — order and take the leading members

Each operation is not a transformation of raw data. It is a determined operation on a named identity.
The Series already knows its name and type. Every operation is a consequence of that knowledge.

---

## The Arc Reading

```
Principle satisfied (entity has passed the nomological gate)
  → Series construction     [Concept emergence: named, typed identity fixed]
  → col!(values).abs()      [Concept operation: determined by the identity's type]
  → .sum(), .mean()         [Concept measure: what this population is in aggregate]
  → where_!(df, mask)       [Concept boundary: which members belong under the mask condition]
  → arrange! + .head(2)     [Concept ordering: the leading members of the identity surface]
```

The Series is what Concept looks like in the DataFrame world.
It does not describe the Concept. It *is* the Concept, materialized as a named column.

---

## What Makes This a Concept and Not Just a Column

A column in a generic data table is an anonymous sequence of values.
A Series in this system is:

1. **Named** — it has a fixed identity label (`values`, `weights`, `mask`)
2. **Typed** — its type determines which operations are valid
3. **Populated** — it has a definite membership (the rows that passed Principle)
4. **Operable** — its operations are determined by its identity, not by external instruction

This four-part structure is exactly the structure of a Concept in Hegel's Logic:
- Identity (the name and type)
- Universality (the operations valid for all members)
- Particularity (the actual population — which entities are in it)
- Singularity (individual access: `sc!(df, values)` retrieves one member)

The external `.gdsl` `concept` block makes this explicit:
```gdsl
concept Score from students {
  identity score;
  mark honors;
}
```

The Series `score` in this example is that Concept, waiting to be named.

---

## The Boolean Mask as Concept Boundary

```rust
let filtered = gds::where_!(df, mask)?.select_columns(&gds::selector![values])?;
```

The `mask` Series is itself a Concept — a boolean identity that determines membership.
`where_!` applied with `mask` is Concept applying to Concept: the boundary Series
determines which members of the `values` Series are within the Concept's scope.

This is the Judgment at work inside the Concept stage — even before Exemplar 014,
the Concept already contains the germ of Judgment: the determination of which
singular members belong under the universal identity.

---

## Key Vocabulary

**Concept** — The named, typed identity that emerges after Principle is satisfied. Not a description of an entity — the entity's scientific form.
*See*: [Concept Emergence](../../REFERENCES/form/concept-emergence.md)

**Series** — The DataFrame-world materialization of a Concept: one name, one type, one population.
*See*: [Core Concepts — Concept](../../REFERENCES/dataset/core-concepts.md)

**Identity field** — The name and type that fix the Concept. In an external `.gdsl` `concept` block: `identity score;`.
*See*: [Shell External Program Grammar](../../REFERENCES/shell/grammar.md)

**Concept operation** — An operation determined by the Concept's type: `.abs()` is valid for i64, `.mean()` for f64. Type determines what is permitted.
*See*: [Frame DSL](../../REFERENCES/dataframe/frame-dsl.md)

**Boolean mask** — A Concept boundary: a boolean Series that determines membership. The germ of Judgment within the Concept stage.
*See*: [Core Concepts — Mark](../../REFERENCES/dataset/core-concepts.md)

---

## Student Notes

- The example is short and clean by design. Concept is simple. What makes it doctrinal is not complexity but the fixedness: once a Series has a name and type, it is determined. All operations follow from that determination.
- Practice this: build three Series with different types. Note which operations are available to each. Note that you cannot take `.mean()` of a boolean Series. That restriction is Concept logic, not a runtime error. The type is the Concept's universal boundary.
- The `sc!` macro (single column extraction) is the singular access operator: it retrieves one named Concept from the DataFrame. This is the Singularity moment of the Concept — one identity pulled from the universal surface.

---

## What This Example Does Not Show

- Judgment (the determination of which members are universal/particular/singular — that is Exemplar 014)
- Syllogism (the middle-term derivation across Concepts — Exemplar 015)
- The full external `.gdsl` `concept` block syntax (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
- Artifact materialization of the Concept table (see [Artifact Materialization](../../REFERENCES/dataset/artifact-materialization.md))
