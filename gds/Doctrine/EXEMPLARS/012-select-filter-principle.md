# Exemplar 012 — Select/Filter as Principle Evaluation

**Source file**: `gds/examples/collections_select_filter.rs`
**Arc position**: Principle (Absolute Reflection → Principle gate)
**Prior exemplar**: [011 — DataFrame Macros as Reflection Engine](011-dataframe-macros-reflection.md)
**Next exemplar**: [013 — Series as Concept Surface](013-series-concept-surface.md)

---

## Principle

Principle is the nomological gate between Reflection and Concept.
An entity that cannot satisfy a Principle condition cannot become a Concept.
`filter!` is the architectural expression of Principle evaluation in the DataFrame world.

---

## What This Example Does

It builds a score/weight/group table and demonstrates five operations:

1. **`select_columns!`** — projects down to only the columns relevant to the Principle check
2. **`filter!`** — retains only entities where `score > 20.0` (the Principle condition)
3. **`select!`** (exprs) — projects with derivation (`score * 2.0`) into a new identity surface
4. **`order_by!`** — orders survivors by the key measure
5. **`group_by!` + `agg!`** — aggregates survivors into named categories

Each operation is a form of Principle application: does this entity belong to the scientific population, and under what identity does it appear if it does?

---

## The Arc Reading

```
Absolute Reflection (entities have been worked through six moments)
  → select_columns!   [Narrow the inspection surface to what Principle needs]
  → filter!           [Principle evaluation: does score > 20.0 hold?]
  → select! (exprs)   [Post-Principle identity projection: new surface for survivors]
  → order_by!         [Ordering survivors by the key mark]
  → group_by! + agg!  [Concept clustering: named groups as proto-Concepts]
```

Entities that do not satisfy `filter!` do not proceed. They are not destroyed.
They remain as a separate population — entities that did not pass Principle,
available for diagnostic review but excluded from the scientific arc.

---

## Principle Is Not Filtering in the Ordinary Sense

This is the doctrinal correction that this exemplar exists to make.

In ordinary data processing, a `filter` is a convenience: drop some rows for efficiency.

In this system, `filter!` at the Principle stage is a nomological gate:

> An entity that fails Principle evaluation is not a less-interesting row.
> It is an entity that cannot become a scientific object in this program world.

The condition `score > 20.0` is not arbitrary. In a fully-specified GDSL program,
it would be derived from a `principle` block that names the required conditions
and unifies concept, judgment, and syllogism targets. The DataFrame `filter!`
is the runtime expression of that named Principle condition.

---

## Group-By as Proto-Concept Clustering

After Principle evaluation, the survivors are grouped:

```rust
let grouped = group_by!(
    table,
    [group],
    [agg!(score.mean => "avg_score"), agg!(id.count => "rows")]
)?;
```

This is proto-Concept formation: entities that share a `group` identity are clustered.
The cluster is named. The cluster has aggregate measures.
The named cluster is almost a Concept — it has identity, it has measure, it has a population.
It becomes a Concept when the GDSL `concept` block fixes its identity field, marks, and bounds.

---

## Key Vocabulary

**Principle** — The nomological gate that determines whether an entity can become a Concept. Evaluated at the Absolute Reflection threshold.
*See*: [Principle Evaluation](../REFERENCES/form-processor/principle-evaluation.md)

**Absolute Reflection threshold** — The boundary where the kernel has finished six Reflection moments and the agent must now evaluate Principle conditions.
*See*: [Absolute Reflection](../REFERENCES/form-processor/absolute-reflection.md)

**`filter!`** — The DataFrame macro expressing a Principle condition. Entities that fail are excluded from the scientific arc.
*See*: [Frame DSL](../REFERENCES/collections-dataset/frame-dsl.md)

**`group_by!` + `agg!`** — Proto-Concept formation. Named clusters of survivors with aggregate measures. Not yet Concept until identity is fixed.
*See*: [Core Concepts — Concept](../REFERENCES/collections-dataset/core-concepts.md)

---

## Student Notes

- The example says: "These operations show the DataFrame as the analytic surface through which a smart Dataset can be inspected and transformed." The word "smart" is doing doctrinal work. The Dataset is not a passive store. It knows the Principle conditions because it was built from a GDSL specification.
- The student should practice distinguishing *Observation filtering* from *Principle filtering*. In Observation, a `where_!` narrows the appearance to what is relevant to study. In Principle, a `filter!` is a yes/no gate on scientific eligibility. The difference is semantic, not syntactic.
- `group_by!` is the last step before Concept. Write it out, look at the result, and ask: what name would I give this cluster? That name is the identity field of the Concept block.

---

## What This Example Does Not Show

- The full GDSL `principle` block syntax (see [GDSL Grammar](../REFERENCES/gdsl/grammar.md))
- Concept emergence once Principle is satisfied (that is Exemplar 013)
- ZeroCopy evaluation at the Principle boundary (see [ZeroCopy Boundary](../REFERENCES/gds-kernel/zeroCopy-boundary.md))
