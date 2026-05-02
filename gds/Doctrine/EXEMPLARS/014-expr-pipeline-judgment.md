# Exemplar 014 — Expr Pipeline as Judgment

**Source file**: `gds/examples/collections_series_expr_pipeline.rs`
**Arc position**: Judgment (Concept → Judgment)
**Prior exemplar**: [013 — Series as Concept Surface](013-series-concept-surface.md)
**Next exemplar**: [015 — Order/Group Exprs as Syllogism](015-order-group-syllogism.md)

---

## Principle

Judgment is the determination of a Concept's members as universal, particular, or singular.
The Expr pipeline is how the DataFrame world applies that determination:
each `series_expr` application is a Judgment over a named Series population.

---

## What This Example Does

It opens with a doctrinal statement: "Series and Expr together provide the smallest compositional
units of the Collections analytic grammar." The example then demonstrates:

1. **`series_expr(s).str().apply(|e| e.ends_with("x"))`** — boolean judgment over each string member
2. **`.apply(|e| e.into_expr().sum())`** — universal aggregate (one value for the whole Series)
3. **`.apply(|e| e.into_expr().mean())`** — universal aggregate (the mean of all members)
4. **`col("a") + col("b")` / `col("a") * col("b")`** — cross-Concept derivation into new identities

Each of these is a different form of Judgment:
- The boolean mask (`ends_with("x")`) is a **Disjunctive Judgment**: each member is either X or not-X
- The aggregate (`.sum()`, `.mean()`) is a **Universal Judgment**: the whole population, one measure
- The cross-Concept derivation (`a + b`) is an **Apodeictic Judgment**: the relationship is necessary given the identity of both Concepts

---

## The Arc Reading

```
Concept surface (Series: named, typed, populated)
  → series_expr(...).apply(|e| e.ends_with("x"))   [Disjunctive Judgment: each member is or isn't]
  → series_expr(...).apply(|e| e.sum())             [Universal Judgment: whole population, one verdict]
  → series_expr(...).apply(|e| e.mean())            [Universal Judgment: measure of the whole]
  → col("a") + col("b") → "sum_ab"                 [Apodeictic Judgment: a and b necessitate sum_ab]
  → col("a") * col("b") → "prod_ab"                [Apodeictic Judgment: a and b necessitate prod_ab]
```

After Judgment, each entity in the Concept population has a determined status.
It is not just a member of a named type. It is determined — marked as X or not-X,
measured against the universal, or derived into a new identity through necessary combination.

---

## Three Forms of Judgment in This Example

### 1. Disjunctive Judgment (boolean mask)
```rust
let contains_x = series_expr(s).str().apply(|e| e.ends_with("x"))?;
```
Each member is judged: does it contain X? The result is `true`/`false` — pure disjunction.
In GDSL, this corresponds to a `judgment` block's `infer <label> when <condition>` clause
where the condition is a boolean test on each member.

### 2. Universal Judgment (aggregate)
```rust
let total = series_expr(nums.clone()).apply(|e| e.into_expr().sum())?;
```
The whole Series is collapsed to one value. This is a Judgment about the population as a whole:
*this Concept has total X*. Not a per-member verdict but a statement about the Concept's
universal extent.

### 3. Apodeictic Judgment (necessary derivation)
```rust
let result = df.with_columns(&[
    (col("a") + col("b")).alias("sum_ab"),
])?;
```
Given the identity of Concepts `a` and `b`, `sum_ab` is necessarily determined.
This is not a contingent observation. It follows from the nature of the Concepts.
That necessity is what makes it Apodeictic (not just true, but necessarily true).

---

## Key Vocabulary

**Judgment** — The determination of a Concept's members as universal, particular, or singular.
*See*: [Concept Emergence](../REFERENCES/form-processor/concept-emergence.md)

**Disjunctive Judgment** — A yes/no determination for each member: belongs or does not belong.
Expressed as a boolean `series_expr` application.

**Universal Judgment** — A statement about the whole Concept population: `.sum()`, `.mean()`.

**Apodeictic Judgment** — A necessary derivation: the new identity is determined by the nature
of the input Concepts. `col("a") + col("b")` is apodeictic given the types of `a` and `b`.

**Expr** — The compositional unit of Judgment. An Expr operates on one or more Series and
produces a determined result. The Expr is the judgment form at the DataFrame level.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

---

## Student Notes

- The example is brief but the doctrinal content is dense. Spend time with each `apply` call.
  Ask: is this a per-member judgment (Disjunctive), a whole-population judgment (Universal),
  or a necessary derivation (Apodeictic)?
- The `series_expr` adapter is the interface between the Concept (Series) and the Judgment (Expr).
  It converts a named identity into an expression-capable context. This matters: the Series
  is not directly operated on. It is first made expressible.
- Cross-Concept derivation (`col("a") + col("b")`) is the germ of Syllogism. The middle term
  is not yet explicit here — that is Exemplar 015. But the structure is the same:
  two Concepts in relation, a third identity derived necessarily from both.

---

## What This Example Does Not Show

- The GDSL `judgment` block syntax (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
- Syllogism (explicit middle-term derivation — Exemplar 015)
- Judgment artifact materialization (see [Artifact Materialization](../REFERENCES/collections-dataset/artifact-materialization.md))
