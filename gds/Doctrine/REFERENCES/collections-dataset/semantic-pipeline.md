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

### MetaPipeline Sovereignty

Even when GLM/GNN delivery becomes enormous (large graph stores, long projection chains,
heavy optimization), its MetaPipeline is still defined by the containing `Dataset`.

Scale does not override governance: compute magnitude can grow without limit, but semantic
control remains bound to Dataset identity, ProgramFeature commitments, and Shell traceability.

Compactly:

`HugeGraphStore != MetaPipelineAuthority`

`ContainingDataset + ShellTrace = MetaPipelineAuthority`

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

## Evolution of Dataset -> SemDataset

The Semantic Pipeline should be read as an evolution, not a replacement.

`Dataset` is the first mediated intelligence over DataFrame. As features are added, Dataset
becomes increasingly semantically capable until it reaches `SemDataset` as end-view.

```text
DataFrame body
    -> Dataset identity/profile
    -> Model:Feature::Plan middle
    -> ProgramFeature commitments
    -> Corpus:LanguageModel pairing
    -> SemDataset
```

This is the meaning of “Dataset gets smarter”: more of the semantic contract is explicit,
typed, and inspectable. The Shell is the master controller of this evolution because it
holds immediate body, mediated middle, and PureForm return in one addressable protocol.

In current implementation terms:

- `ShellDataFrameKnowledge` names what the DataFrame body contributes.
- `ShellModelFeaturePlanKnowledge` names what Dataset contributes in the middle.
- `ShellSemanticPipelineKnowledge` names evolution checkpoints and semantic capabilities:
    `frame_ready`, `middle_ready`, `semdataset_ready`, `pureform_return_ready`.
- `ShellPureFormReturn` names the DataPipeline return into `PureFormPrinciple`.
- `ShellLearningReport` names what the Dataset has learned on the way to SemDataset:
    gained principles/concepts/procedures, unresolved forms, KG/NLP/logic readiness,
    and a readiness score.

Together these make the evolution auditable rather than implicit.

`ShellSemanticPipelineKnowledge` also exposes semantic capability classes that define the
intelligence envelope users target in SDSL:

- `form-ingestion`
- `logical-form-parsing`
- `corpus-language-pairing`
- `language-model-fitting`
- `principle-gated-concept-return`

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

---

## Forward-Looking: World Projection Principle

At the Shell/Form boundary, a larger pattern appears at the Project/Factory layer:

- `Base`: stable kernel contracts (address, schema, feature kinds, return principle)
- `Meta`: projection machinery that composes those contracts into agent-world runtime

This is the Meta/Base pattern at the World Projection moment.

### Proper Shell Principle

A proper Shell is the Living Doctrine Runtime.

Projection validity requires Shell-traceability through:

```text
Frame -> Model:Feature::Plan -> SemDataset -> PureForm return
```

This principle is machine-checkable in runtime through:

- `GdsShell::validate_projection_trace() -> ShellProjectionTraceValidation`
- `GdsShell::is_projection_trace_valid() -> bool`
- `GdsShell::learning_report() -> ShellLearningReport`

### Doctrine Rule

Every RootAgent world projection must be grounded by definition in a semantic root:

1. The projected world has a corresponding Principle in some `SemDataset`.
2. The RootAgent projection declares its root Dataset identity (name/profile/source lineage).
3. The projection is valid only if Shell can trace:
    `Frame -> Model:Feature::Plan -> SemDataset -> PureForm return`.

So world projection is never free-floating simulation. It is a controlled projection of a
root semantic commitment, carried by Shell and anchored in Dataset evidence.

### Compact Law

`RootAgentProjection(World) => exists RootSemDataset(Principle)`
