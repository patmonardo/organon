# Exemplar 016 — Join Operations as Inference

**Source file**: `gds/examples/dataframe_join_operations.rs`
**Arc position**: Inference (Syllogism → Inference across Concept populations)
**Prior exemplar**: [015 — Order/Group Exprs as Syllogism](../dataframe/015-order-group-syllogism.md)
**Next exemplar**: [017 — Streaming Dataset as Process](../dataset/017-streaming-dataset-procedure.md)

---

## Principle

Inference is the conclusion drawn across two independently-structured Concept populations
through a shared middle term.
A join operation is Inference: the shared key is the middle term; the result is the conclusion
that follows from both populations being in relation through that term.

---

## What This Example Does

It builds two tables — `left` (id, left) and `right` (id, right) — with partial overlap in `id`,
then demonstrates five join types:

1. **inner join** — only entities where both populations confirm the middle term
2. **left join** — all left-side entities; right-side filled as null where the middle term is absent
3. **right join** — all right-side entities; left-side filled as null where absent
4. **full join** — all entities from both populations; both sides filled where present
5. **semi join** — left-side entities whose `id` appears in the right population (membership test)

Each join type is a different inference strength.

---

## The Arc Reading

```
Two Concept populations (left, right) each internally structured
  → inner join    [Strong inference: both sides confirm the middle term]
  → left join     [Asymmetric inference: left asserts, right corroborates where possible]
  → right join    [Asymmetric inference: right asserts, left corroborates where possible]
  → full join     [Maximal inference: all entities included, null where corroboration fails]
  → semi join     [Membership inference: left entities confirmed by the right population]
```

The Inference surface is the combined result. Entities present in both populations
under the shared `id` are fully determined — their inference is complete.
Entities present in only one population are partially determined — their inference has a gap.

---

## The Middle Term Across Populations

In a single `group_by!` Syllogism (Exemplar 015), the middle term mediates within one table.
In a join, the middle term mediates *across* two independently-structured tables.

```rust
let inner = gds::join!(left, right, on = [id], how = inner)?;
```

Here `id` is the shared middle term. It appears in both `left` and `right`.
The conclusion: entities with the same `id` in both populations can be brought into
a single determined row — their left-side properties and right-side properties are both
ascribed to the same identity.

This is a cross-Concept Inference: not just *this population grouped by a key*,
but *these two populations brought into necessary relation through a shared key*.

---

## Join Types as Inference Strengths

| Join type | Inference form | Description |
|---|---|---|
| `inner` | Bilateral confirmation | Both sides must confirm the middle; unconfirmed entities are excluded |
| `left` | Left-assertive | Left population asserts; right corroborates where possible |
| `right` | Right-assertive | Right population asserts; left corroborates where possible |
| `full` | Open inference | Both populations included; nulls mark incomplete corroboration |
| `semi` | Membership test | Left entities confirmed by presence in the right population |

The strongest inference is `inner`: both populations confirm every concluded entity.
The most open inference is `full`: every entity is included regardless of corroboration.

---

## Null as Inference Gap

In a left join, when a left-side entity has no matching `id` in the right population,
the right-side columns are `null`. This null is not a data quality issue.
It is the logical form of an incomplete inference:

> The left-side entity exists and has a determined identity.
> The right-side population does not corroborate it through the middle term.
> The inference is partial — the entity participates, but not fully.

This has immediate doctrinal consequences: when building an external `.gdsl` `procedure` that emits
inference artifact tables, nulls in a join must be recorded and preserved as evidence
of partial inference, not silently dropped or filled.

---

## Semi Join as Membership Inference

```rust
let semi = left.join_on(&right, &["id"], JoinType::Semi, None)?;
```

The semi join returns only left-side rows whose `id` appears in the right population.
This is a Membership Inference: the right population acts as a corroborating set.
The question is not "what is on the right side?" but "which left-side entities are confirmed
as belonging to a population that the right side knows about?"

In external artifact terms, a semi join corresponds to a `syllogism` block that uses a reference
population (the right table) as the corroborating source for a conclusion about the left.

---

## Key Vocabulary

**Inference** — A conclusion drawn across two Concept populations through a shared middle term.
Distinguished from Syllogism (which mediates within one population) by operating across
independently-structured tables.
*See*: [Shell External Program Grammar — query/syllogism](../../REFERENCES/shell/grammar.md)

**Middle term (cross-population)** — The shared key (`id`) that allows two independently-structured
Concept populations to be brought into necessary relation.

**`join!`** — The DataFrame expression of Inference. The `on` parameter names the middle term.
*See*: [Frame DSL](../../REFERENCES/dataframe/frame-dsl.md)

**Null (inference gap)** — A null in a join result is not missing data — it is a partial inference:
the entity participates but is not fully corroborated by both populations.

**Semi join** — Membership inference: which left entities are confirmed by the right population?

---

## Student Notes

- The example says joins show "how multiple analytic bodies can be brought into relation inside the Collections layer." That is the Inference doctrine in one sentence. The Collections layer is the logical space where Inference happens.
- Study the difference between `inner` and `full` join results carefully. The inner join has 2 rows (ids 2, 3 — confirmed by both). The full join has 4 rows (ids 1, 2, 3, 4 — all entities, nulls where corroboration fails). The logical difference is significant: inner is a strong claim; full is an open investigation.
- When reviewing an external `.gdsl` artifact's `query` block, notice whether it uses filtering that corresponds to inner-join semantics (only confirmed entities) or left-join semantics (all entities of a given population, with corroboration where available). That choice encodes an epistemological stance.

---

## What This Example Does Not Show

- The streaming/lazy execution of Inference at scale (Exemplar 017)
- Procedure emission of Inference artifact tables (see [Hegel Objectivity](../../REFERENCES/philosophy/hegel-objectivity.md))
- External `.gdsl` `query` block syntax for cross-population inference (see [Shell External Program Grammar](../../REFERENCES/shell/grammar.md))
