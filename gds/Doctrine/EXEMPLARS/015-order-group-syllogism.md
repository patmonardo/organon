# Exemplar 015 — Order/Group Exprs as Syllogism

**Source file**: `gds/examples/collections_order_group_exprs.rs`
**Arc position**: Syllogism (Judgment → Syllogism)
**Prior exemplar**: [014 — Expr Pipeline as Judgment](014-expr-pipeline-judgment.md)
**Next exemplar**: [016 — Join Operations as Inference](016-join-operations-inference.md)

---

## Principle

Syllogism is the mediation of a conclusion through a middle term.
In the DataFrame world, `group_by!` expresses this exactly:
the middle term is the grouping key; the conclusion is the aggregate over that group.

---

## What This Example Does

It builds a score/weight/region table, then:

1. **`mutate!`** — derives `weighted_score` and `is_high` as marks (post-Judgment preparation)
2. **`order_by_columns`** — orders by region (asc) then weighted_score (desc) — establishing the order of the middle
3. **`group_by!` + `agg!`** — the syllogistic operation: entities sharing `region` conclude to a group with definite measures

The comment: "This is the kind of entity-analysis body the Dataset can drive into view for downstream inspection."
That phrase — "drive into view" — is the Dataset acting as the Syllogism's driver.

---

## The Arc Reading

```
Judgment results (entities have been determined — weighted_score, is_high)
  → order_by_columns([region, weighted_score])   [Establish the middle-term order]
  → group_by!(ordered, [region], [...])           [Syllogism: region is the middle term]
  → agg!(weighted_score.mean => "avg_weighted")   [Conclusion: every region-entity concludes to avg]
  → agg!(score.max => "max_score")                [Conclusion: every region-entity concludes to max]
  → agg!(id.count => "rows")                      [Conclusion: population size of each group]
```

The Syllogism: All entities in `region = NE` (Minor) → share the universal `region` (Middle) →
conclude to a group with definite aggregate measures (Major conclusion).

---

## The Middle Term

In Hegel's Syllogism, the middle term mediates between singular and universal.

Here, `region` is the middle term:
- **Singular**: each individual entity (id = 1, 2, 3...)
- **Middle term**: `region` — the shared identity that allows mediation
- **Universal conclusion**: `avg_weighted`, `max_score`, `rows` — the determined measures of the group

The middle term is not a filter. It does not exclude entities. It mediates them.
Every entity participates in the Syllogism through its `region` value.
Entities with different `region` values participate in different Syllogisms, simultaneously.

---

## Ordering as Syllogistic Preparation

```rust
let ordered = weighted.order_by_columns(
    &gds::selector![region, weighted_score],
    PolarsSortMultipleOptions::new().with_order_descending_multi([false, true]),
)?;
```

This ordering is not cosmetic. It establishes the priority structure within each group:
within `region = NE`, which entity has the highest `weighted_score`?
That priority will be preserved in the `max_score` aggregate.

The ordering step is the Syllogism's internal ordering of the minors before the conclusion is drawn.

---

## Multiple Simultaneous Syllogisms

After `group_by!`, the result has two rows: one for `NE`, one for `SW`.
These are two Syllogisms operating simultaneously:

1. All NE entities (via `region = NE` as middle) → conclude to `{avg_weighted: X, max_score: Y, rows: Z}`
2. All SW entities (via `region = SW` as middle) → conclude to `{avg_weighted: A, max_score: B, rows: C}`

This is not two separate operations run sequentially. It is one `group_by!` that
simultaneously drives both Syllogisms to their conclusions. The Dataset's semantic knowledge
of `region` as the middle term makes this possible.

---

## Key Vocabulary

**Syllogism** — Mediation of a conclusion through a middle term. The form:
*Singular is under Middle; Middle concludes to Universal; therefore Singular concludes to Universal.*
*See*: [GDSL Grammar — syllogism block](../REFERENCES/gdsl/grammar.md)

**Middle term** — The grouping key. The identity shared across the singular entities that allows
a conclusion to be drawn about the group. Here: `region`.

**`group_by!`** — The DataFrame expression of Syllogism. The grouping key is the middle term.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

**`agg!`** — The conclusion form. Each aggregation produces one determined measure per group.

**Syllogism conclusion** — The aggregate result: `avg_weighted`, `max_score`, `rows`.
Not a summary of the data — the necessary conclusion given the middle term.

---

## Student Notes

- The example opens with a subtle doctrinal hint: "the Dataset can drive into view for downstream inspection." This is the Dataset acting as the Syllogism's semantic controller. The Dataset knows `region` is a grouping key because the GDSL specification named it as such. The DataFrame is driven by that knowledge.
- Practice: take any `group_by!` call in your own work and ask: what is my middle term? What is the universal conclusion I am drawing? State the full Syllogism in words. This will clarify whether the grouping key is genuinely mediating or just being used as a partitioning convenience.
- The two-row result (NE, SW) is the system's Syllogism surface. Each row is a concluded group. From here, Judgment can be re-applied: which group has the higher `avg_weighted`? That re-application is Inference — the next stage.

---

## What This Example Does Not Show

- Cross-table Syllogism (multiple DataFrames joined through a shared middle — Exemplar 016)
- Inference chains across multiple Syllogisms (Exemplar 016)
- The GDSL `syllogism` block syntax (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
- Artifact materialization of Syllogism outputs (see [Artifact Materialization](../REFERENCES/collections-dataset/artifact-materialization.md))
