# Exemplar 027 — Model:Feature::Plan as Essence Middle

**Source file**: `gds/examples/collections_model_feature_plan.rs`
**Arc position**: Middle Layer: Essence mediation between DataFrame Beginning and SemDataset End
**Prior exemplar**: [026 — Semantic Meta Pipeline as End-Stage Concept](026-semantic-meta-pipeline.md)
**Next exemplar**: [028 — DataFrame as Intuition](028-dataframe-intuition.md)

---

## Principle

`Model:Feature::Plan` is the hard Middle of the Dataset Meta Pipeline. DataFrame gives the real executable body. `SemDataset` gives the returned End. The Middle prepares, stamps, executes, and records the transformation by which real DataFrame body becomes ideal Dataset form.

---

## What This Example Does

It constructs a compact Middle Layer pipeline:

1. Creates a DataFrame-backed `Dataset`
2. Builds a `Plan` over that Dataset and inspects its attention report
3. Wraps the Plan as a `Feature` with the canonical `item` projection
4. Prepares a `Model` with required, contingent, and impossible feature marks
5. Executes the prepared essence over the DataFrame lazy body
6. Realizes an ontology image recording model, features, constraints, and provenance

This exemplar is not about the End. It is about Essence: the mediated engine.

---

## The Arc Reading

```text
DataFrame body
  -> Dataset                         [semantic client over body]
  -> Plan                            [deferred Meta Plan]
  -> Feature                         [named address with a Plan body]
  -> Model preparation               [essence and modality]
  -> Feature execution               [lowering into LazyFrame]
  -> OntologyDataFrameImage          [readable record of mediation]
```

This is the point where real being becomes ideal being. The DataFrame remains executable, but it is no longer merely raw data. It has been read by Model, named by Feature, and mediated by Plan.

---

## The Three Boxes

The example uses the three-box substrate:

| Box | Function | Output |
|---|---|---|
| Box 1 | `prepare_model` | `ModelEssence` |
| Box 2 | `execute_essence` | `Execution` |
| Box 3 | `realize_image` | `OntologyDataFrameImage` |

The boxes are the current code form of the Middle Layer. They make Essence inspectable.

---

## Why This Follows Exemplar 026

Exemplar 026 showed the End: `SemDataset` as Corpus:LM plus logical forms.

After seeing the End, the system returns to the Middle. The question becomes: how did DataFrame become readable enough for the End to know itself?

The answer is `Model:Feature::Plan`.

---

## Key Vocabulary

**`Model:Feature::Plan`** — The Middle Layer engine. See [Model:Feature::Plan](../REFERENCES/collections-dataset/model-feature-plan.md).

**`Plan`** — A deferred Dataset recipe over a DataFrame body. See [Core Concepts](../REFERENCES/collections-dataset/core-concepts.md).

**`Feature`** — A named semantic address whose current Projection role wraps a `Plan`. See [Feature Structures](../REFERENCES/collections-dataset/feature-structures.md).

**`Modality`** — The preparation stamp that controls execution. See [Modality](../REFERENCES/collections-dataset/modality.md).

**`OntologyDataFrameImage`** — The readable image of model-feature execution. See [Ontology Images](../REFERENCES/collections-dataset/ontology-images.md).

---

## Student Notes

- The impossible feature is intentional. Contradiction is not erased; it becomes an image constraint and provenance fact.
- `PlanAttentionReport` shows why Dataset Plan is a Meta Plan. It reports intent and observed shape before the End-stage semantic object receives the result.
- The `item` column is the canonical idealization point: a feature-map artifact in a DataFrame body.

---

## What This Example Does Not Show

- The full seven moments of Reflection
- Agent-side condition evaluation
- DirectCompute GLM/GNN dispatch
- End-stage logical-form parsing through `SemDataset`
