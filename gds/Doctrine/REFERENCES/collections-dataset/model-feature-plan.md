# Model:Feature::Plan

`Model:Feature::Plan` is the Middle Layer of the Dataset Meta Pipeline. It is the engine of Essence: the place where DataFrame real body is mediated into Dataset ideal form.

---

## Definition

The Middle Layer has three terms:

| Term | Role |
|---|---|
| `Model` | The schema or classificatory commitment that reads rows as meaningful objects |
| `Feature` | The named typed address through which a model receives determination |
| `Plan` | The deferred DataFrame recipe that gives a feature its runtime body |

This is not a new runtime beside Polars. It is the Dataset client SDK over Polars.

DataFrame executes. `Model:Feature::Plan` mediates.

---

## Doctrine

Beginning and End are simple.

- Beginning: DataFrame, immediate real body
- End: `SemDataset`, Corpus:LM gathered as Concept
- Middle: `Model:Feature::Plan`, Essence as mediation

The Middle is difficult because it must do three things at once:

1. preserve the executable DataFrame body
2. name the feature as semantic address
3. prepare the model under principle, mark, and modality

This is why Plan belongs in Dataset, not DataFrame. A DataFrame plan is physical or lazy execution. A Dataset Plan is a Meta Plan: a semantic wrapper over the runtime plan.

---

## The Three Boxes

The Middle Layer currently appears as three boxes:

```text
Box 1: prepare_model(...)              -> ModelEssence
Box 2: execute_essence(essence, lf)    -> Execution
Box 3: realize_image(essence, exec)    -> OntologyDataFrameImage
```

Reading:

1. **Model preparation** resolves feature marks against accumulated essence.
2. **Feature execution** lowers executable features into a Polars `LazyFrame`.
3. **Image realization** records the model, features, constraints, and provenance.

The image is not merely output. It is the Middle Layer becoming readable.

---

## Feature As Plan Wrapper

A `Feature` currently wraps a `Plan` in its Projection role.

```rust
pub struct Feature {
    plan: Plan,
}
```

This means a Feature is not just a column. It is a named semantic address whose body is a deferred DataFrame recipe.

The canonical projection artifact is an `item` struct column. `Feature::requiring_item(plan)` enforces that the Plan ends by producing that `item`.

---

## Modality

Model preparation stamps every Feature with modality:

| Modality | Meaning |
|---|---|
| Necessary | the model requires or entails the mark |
| Contingent | the mark may be observed at execution |
| Possible | reserved for deferred preparation |
| Impossible | the mark contradicts accumulated essence |
| Unknown | preparation could not determine executability |

Modality is the bridge from principle to execution. Box 2 reads modality to decide what can be lowered.

---

## Review Rule

Do not treat Dataset Plan as a generic Polars convenience wrapper.

A Dataset Plan is the Middle Layer of the semantic system. It must answer:

1. Which Dataset body does this plan read?
2. Which Feature does it give body to?
3. Which Model prepares that feature?
4. Which modality stamped the feature?
5. Which provenance row records the decision?

If these questions are invisible, the change belongs in DataFrame, not in Dataset.
