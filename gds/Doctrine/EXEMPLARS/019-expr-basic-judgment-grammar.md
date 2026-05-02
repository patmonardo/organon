# Exemplar 019 — Expr as Reusable Judgment Grammar

**Source file**: `gds/examples/collections_expr_basic.rs`
**Arc position**: Judgment grammar (reusable predicate units, first-class analytic components)
**Prior exemplar**: [018 — Streaming Lazy as Deferred Collection](018-streaming-lazy-deferred.md)
**Next exemplar**: [020 — Framing and Chunking as PureForm Shape](020-framing-chunking-pureform.md)

---

## Principle

An Expr is not a filter condition.
An Expr is a first-class Judgment: a named, reusable determination
that can be applied to any compatible Concept surface.

---

## What This Example Does

The doctrinal statement: "Exprs are the reusable analytic grammar of the DataFrame layer."

It demonstrates:

1. **Build a table** with `id` and `score`
2. **`let high_score = gds::expr!(score > 20.0)`** — construct the Judgment as a named object
3. **`table.filter_expr(high_score.clone())`** — apply as filter (retain members satisfying the Judgment)
4. **`table.with_columns(&[high_score.alias("is_high")])`** — apply as derivation (mark each member)
5. **`group_by!(tagged, [is_high], [agg!(score.mean => "mean_score")])`** — Syllogism driven by the Judgment mark

The same `high_score` Expr is used twice: once as a filter gate, once as a derived column.
This reuse is not incidental — it is the doctrinal point.

---

## The Arc Reading

```
Concept surface (score Series, typed as f64)
  → let high_score = expr!(score > 20.0)            [Judgment constructed as first-class object]
  → table.filter_expr(high_score.clone())            [Judgment as gate: retain satisfying members]
  → table.with_columns([high_score.alias("is_high")]) [Judgment as mark: stamp each member]
  → group_by!(tagged, [is_high], [...])              [Syllogism: is_high as middle term]
```

The Judgment precedes both its applications. It is defined once and applied in two modes:
as a gate (exclude/include) and as a stamp (mark the member with the verdict).
The ability to do both is what makes it a first-class object, not a syntactic shorthand.

---

## Judgment as First-Class Object

In most DataFrame libraries, a filter condition is an expression that disappears after use.
Here, `high_score` is a named Rust binding. It persists. It can be passed to functions,
stored in collections, logged for provenance, or composed with other Exprs.

This is the grammatical implication of treating Judgment as a first-class construct.
In a GDSL program, a `judgment` block names the inference rule:

```gdsl
judgment HighScore for Score {
  infer high when score > 20.0;
}
```

The GDSL compiler will produce a named Expr for this judgment — exactly `expr!(score > 20.0)` —
and apply it in both filter and mark roles wherever `HighScore` is referenced.

---

## `.clone()` as Provenance Preservation

```rust
let filtered = table.filter_expr(high_score.clone())?;
let tagged = table.with_columns(&[high_score.alias("is_high")])?;
```

The `.clone()` is not boilerplate. It is the signal that `high_score` is being used twice
from the same source definition. In a provenance-aware system, both artifacts —
the filtered table and the tagged table — trace back to the same Judgment definition.
A reader can inspect either artifact and find the same `high_score` rule as their origin.

---

## `alias("is_high")` as Concept Naming After Judgment

```rust
table.with_columns(&[high_score.alias("is_high")])?
```

When a Judgment is applied as a column derivation, `alias` names the result.
`is_high` is not just a column name — it is the Concept that the Judgment produces.
Entities in this column are now marked: they belong to the `is_high` Concept or they do not.

This is the exact transition from Judgment to Concept materialization: the Judgment fires,
the verdict is named, the name becomes a Concept boundary for subsequent Syllogism.

---

## Group-By Driven by a Judgment Mark

```rust
let grouped = gds::group_by!(tagged, [is_high], [gds::agg!(score.mean => "mean_score")])?;
```

After `is_high` is stamped as a column, it becomes the middle term for a Syllogism.
All entities with `is_high = true` conclude to one group. All with `is_high = false` conclude to another.
The Syllogism is directly grounded in the Judgment.

This is the full arc in miniature: Expr (Judgment) → alias (Concept name) → group_by (Syllogism).

---

## Key Vocabulary

**Expr** — A first-class Judgment: a named determination that can be applied as a filter gate or a derivation mark.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

**`gds::expr!`** — The macro that constructs a typed Expr from a predicate expression. Returns a reusable object.

**`filter_expr`** — Apply a Judgment as a gate: retain only members where the Judgment holds.

**`with_columns` + `alias`** — Apply a Judgment as a mark: stamp each member with the Judgment verdict under a named Concept.

**Reusable Judgment** — A Judgment defined once and applied in multiple roles. The `clone()` call marks each reuse.

---

## Student Notes

- Build a single `gds::expr!` and apply it three different ways: as a filter, as a column derivation, and as a group-by key (via alias). Observe that the same logical definition produces three different structural results. That is the proof that Expr is a first-class object, not a syntactic placeholder.
- Compare this to the boolean mask in Exemplar 013. There, the mask was a Series (a Concept). Here, the `high_score` is an Expr (a Judgment definition). The Series is the verdict already applied; the Expr is the rule before application. The distinction matters for provenance.
- Notice that `group_by!(tagged, [is_high], ...)` requires the Judgment mark to already be stamped as a column. You cannot group by an unapplied Expr. This is the correct doctrinal sequence: Judgment fires, mark is named, Syllogism uses the mark.

---

## What This Example Does Not Show

- Compound Expr composition (multiple Judgments combined into one — see Exemplar 020 for structural composition)
- Lazy Expr evaluation (Exemplar 018)
- Named Judgment blocks in GDSL (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
- Expr artifact logging for provenance (see [Provenance](../REFERENCES/collections-dataset/provenance.md))
