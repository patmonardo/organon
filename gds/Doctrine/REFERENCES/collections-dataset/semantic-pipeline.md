# Semantic Pipeline

The Semantic Pipeline is the Dataset Meta Pipeline. It is the agential access path where Corpus, LanguageModel, and Program Features meet.

---

## Definition

`SemDataset` is the End-stage Dataset object:

```rust
pub struct SemDataset<L> {
    corpus: Corpus,
    lm: L,
    forms: Vec<SemForm>,
}
```

It pairs:

- `Corpus`: extensional evidence, documents, sources, annotations
- `LM`: intensional distribution fitted over that evidence
- `SemForm`: Form Program Features parsed into logical expressions

This is not a replacement for DataFrame. It is the Dataset-level controller above DataFrame.

---

## Doctrine

The Semantic Pipeline is the **Semantic Graph platform**.

The GLM/GNN path is the **Compute Graph platform**.

They coexist as two aspects of the same system:

| Platform | Role | Access path |
|---|---|---|
| Semantic Graph | meaning, forms, plans, provenance, agential interpretation | `SemDataset`, `ProgramFeature`, Dataset Plan |
| Compute Graph | scale, optimization, projection, training | DirectCompute, GLM/GNN, Root Projection |

The Semantic Pipeline may invoke Compute Graph work. It must not collapse into it.

---

## The Meta Pipeline

The line is:

```text
DataFrame -> Model:Feature Plan -> Corpus:LM -> SemDataset
```

Reading:

1. `DataFrame` is the real runtime body.
2. `Dataset` names and stages intent.
3. `Plan` is the middle layer: a Meta Plan over Polars plans.
4. `Corpus:LM` is the end-view: evidence and meaning paired.
5. `SemDataset` receives Form Program Features and parses them into logical form.

This is why Plan belongs in Dataset, not DataFrame. DataFrame executes. Dataset governs.

---

## SemForm

A `SemForm` is a Program Feature viewed inside the Semantic Pipeline.

It carries:

- `kind`: the `ProgramFeatureKind`
- `text`: the logical or doctrinal phrase
- `source`: provenance-bearing origin
- `expr`: parsed logical expression, if parsing succeeds
- `error`: parse error text, if parsing fails

`SemForm` is the first bridge from `form/program` into the NLP semantic layer.

---

## Current Boundary

Current implementation:

- fit a LanguageModel from Corpus text
- ingest Program Features
- parse forms through `ml::nlp::sem::logic::LogicParser`
- retain parse success and failure as inspectable semantic state

Deferred implementation:

- model-theoretic evaluation through `ml::nlp::sem::evaluate`
- proof dispatch through `ml::nlp::inference`
- DirectCompute handoff for GLM/GNN scale
- code generation from stable semantic plans

---

## Review Rule

Do not route ordinary agential semantic access directly to GLM/GNN compute.

Use `SemDataset` when the task is about meaning, provenance, forms, judgments, or inference over source evidence.
Use DirectCompute when the task requires large-scale graph projection or model training.

The Semantic Pipeline governs. The Compute Graph executes when summoned.
