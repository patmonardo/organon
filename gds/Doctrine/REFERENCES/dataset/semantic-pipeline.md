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

Internally, `LM` may be read as **Learning Module**. Technically it remains the
LanguageModel parameter of `SemDataset<L>`, but doctrinally its role is the Model
in training: the learning organ working underneath SemDataset, fitted over Corpus
evidence and then governed by Shell traceability on the return to PureForm.

GDS may use published linguistic model forms such as tokenizers, taggers,
parsers, and language models, but the system is not governed by linguistics as
an external discipline. These tools become conformant only when adapted into the
Dataset grammar:

`Model:Feature::Plan -> Corpus:LM::SemDataset`

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

From the Dataset perspective, step 1 is EssentialBeing: immediate determinacy
(`Frame:Series::Expr`) that mediation must receive. The DataFrame does not
become Dataset-aware; Dataset mediation reads and organizes the DataFrame body.

This is why Plan belongs in Dataset, not DataFrame. DataFrame executes. Dataset governs.

---

## Analytical Path Stages (Term Locator)

This section fixes abstract terms to software locations. The point is not to
invent new vocabulary, but to force every major term to have a precise runtime
home.

| Stage | Doctrinal Name | Abstract Function | Software Surface | Shell Check Surface |
|---|---|---|---|---|
| 0 | Immediate Beginning | EssentialBeing | `Frame:Series::Expr` over `GDSDataFrame` | `ShellDataFrameKnowledge` |
| 1 | First Mediation | Named body | `Dataset { name, frame, artifact_profile }` | `ShellModelFeaturePlanKnowledge` (model identity) |
| 2 | Essence Middle | Determinate mediation | `Model:Feature::Plan` and three boxes (`prepare_model`, `execute_essence`, `realize_image`) | `ShellModelFeaturePlanKnowledge` |
| 3 | Program Commitment | Logical self-declaration | `ProgramFeature` / `ShellProgram` commitments | `ShellSemanticPipelineKnowledge` + `ShellLearningReport` |
| 4 | Concept Formation | Evidence-meaning pairing | `Corpus + LanguageModel` | `ShellCorpusReport` |
| 5 | Concept Return | Semantic object | `SemDataset<L>` with `SemForm` parse state | `ShellSemanticPipelineKnowledge` |
| 6 | Principle-Gated Return | Unified protocol return | `ShellPipelineDescriptor -> ShellPureFormReturn -> PureFormPrinciple` | `validate_projection_trace`, `is_projection_trace_valid` |

### Reading Rule

If a term cannot be pointed to one stage and one software surface, it is still
speculative. Keep it out of normative doctrine until it acquires a location.

### Asymmetry Rule

At stages 0-2 the asymmetry must remain explicit:

- DataFrame can exist and execute without Dataset-awareness.
- Dataset cannot mediate without DataFrame immediacy.

This preserves the architecture: mediation is a higher-order reading of the
same body, not a second body.

---

## Path Axioms

These axioms are normative rules for the Dataset Meta Pipeline. They are meant
to be short, checkable, and reusable in reviews.

1. **Immediacy Axiom**
    The first object of the path is always a DataFrame body (`Frame:Series::Expr`).
    No doctrine may insert a prior pipeline object before this beginning.

2. **EssentialBeing Axiom**
    From the Dataset perspective, the immediate DataFrame is EssentialBeing:
    determinate enough to mediate, not yet mediated enough to count as
    `Model:Feature::Plan`.

3. **Asymmetry Axiom**
    DataFrame can execute without Dataset-awareness. Dataset cannot mediate
    without DataFrame immediacy.

4. **Mediation Axiom**
    Dataset mediation must be locatable as `Model:Feature::Plan` and remain
    distinguishable from raw DataFrame execution.

5. **Commitment Axiom**
    Program commitments (`ProgramFeature`/`ShellProgram`) must be explicit and
    inspectable. Implicit semantic commitments are invalid for normative doctrine.

6. **Concept Axiom**
    Semantic end-state must be materialized as `Corpus + LanguageModel -> SemDataset`
    with visible `SemForm` success/failure state.

7. **Return Axiom**
    A projection is doctrinally valid only when Shell can trace and report a
    principle-gated return:

    `Frame -> Model:Feature::Plan -> SemDataset -> PureForm return`.

### Audit Use

For any new feature, answer these seven axioms in order. A missing answer marks
the feature as exploratory rather than doctrinally integrated.

---

## Plan As Reflective Control (Not Total Sensation Capture)

Your reading is correct: EssentialBeing does not disappear by magic, but it does
empty itself into mediated form through Plan. In that passage, Plan becomes the
controlling notion of Essence.

That control has a strict boundary:

- Plan is a reflective selector and organizer.
- Plan is not the totality of sensation carried by live DataFrame body.

So the passage should be read as a change of sphere:

- Being sphere: rows/columns/expressions behave as immediate, extensional,
    runtime variation.
- Essence sphere: that variation is held up for reflection, typed, ordered, and
    constrained by `Model:Feature::Plan` commitments.

In compact form:

`Plan governs what is retained for mediation; it does not claim to exhaust all immediate variation.`

### Practical Doctrine Rule

When introducing or reviewing a Plan, verify all three conditions:

1. The Plan states what it retains from the immediate body (selection/projection).
2. The Plan states what it excludes or defers (unmediated residual variation).
3. The Shell trace can still point back to DataFrame immediacy as source body.

If (1) and (2) are not explicit, Plan has become vague. If (3) fails, mediation
has become detached from its own beginning.

---

## LM As Compilation Point (Feature Algebra Into Application)

LanguageModel is not merely a type parameter in `SemDataset<L>`. It is the
Concept-fold moment where the Essence algebra of `Feature` and `FeatStruct` is
compiled into an actual deployable application.

The distinction matters:

- `Model:Feature::Plan` is the Essence algebra. It names, binds, and orders.
  It produces well-known Feature forms and FeatStruct structures. But it does not
  yet execute as an application a caller can deploy.
- `Corpus:LM::SemDataset` is the Concept return. `LM` is the point where those
  Feature/FeatStruct commitments are compiled and fitted into something that
  runs — a real Learning Module, a tokenizer pipeline, a semantic parser, or
  any other deployable model form.

Compactly:

`LM = compilation of Feature/FeatStruct algebra into a deployable application`

This changes how we read the Concept fold:

| Moment | Role |
|---|---|
| `Corpus` | extensional evidence: documents, sources, annotations — what LM is fitted over |
| `LM` | intensional application: the compiled model, fitted and deployable |
| `SemDataset` | the end-view that holds both: evidence + application + logical form state |

### Why This Is The Right Reading

Feature and FeatStruct define the well-known semantic roles the system
recognizes: tokenization contracts, parsing roles, annotation schemes, feature
vectors, modality declarations. These are the algebra of what can be committed
to semantically.

But algebra is not execution. A deployed tokenizer is not just a FeatStruct. A
deployed NLP parser is not just a Feature declaration. They are those
commitments compiled into a running form over actual evidence.

LM is where the algebra becomes an application. Corpus is what the application
runs over. SemDataset is the result of that running — the Concept that carries
evidence, application, and parsed SemForm state together.

### Practical Consequence

This means building out `LM` is not downstream detail work. It is the act that
closes the Concept fold. Without a real compiled LM, the path stops at Essence.

For the current phase the canonical way to seed LM is:

```text
ProgramFeature subfeature declarations (external `.gdsl` artifact)
  -> lowered to ProgramFeatureKind::Subfeature
  -> materialized in SemDataset runtime as the LM body
```

New LM applications — deployed taggers, parsers, retrieval models,
embedding engines — should be introduced as ProgramFeature subfeature
declarations or as explicit FeatStruct-backed model wrappers, not as raw
third-party dependencies that bypass the Essence middle.

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
- `GdsShell::materialize_semdataset_from_texts()` explicitly connects Shell to real
    Corpus production by turning caller-provided texts into `Corpus -> MLE -> SemDataset`.
- `ShellCorpusReport` names the materialized corpus state: document count, LM order,
    vocabulary size, SemForm count, and parsed logical-form count.
- `ShellCapabilityMap` names platform capability states across immediate, mediation,
    and recursive bands, including DataPipeline, progress tracking, memory estimation,
    concurrency runtime, Pregel runtime, DefaultGraphStore, Corpus materialization,
    SemDataset learning, and mathematical logic readiness.
- `GdsShell::pipeline_progress_tracker()` and `GdsShell::estimate_pipeline_memory()`
    expose the kernel progress-tracker and memory-estimation infrastructure as
    common, reusable Shell pipeline tooling.

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
- `GdsShell::materialize_semdataset_from_texts() -> Result<GdsShell, ShellCorpusError>`
- `GdsShell::corpus_report() -> Option<ShellCorpusReport>`
- `GdsShell::capability_map() -> ShellCapabilityMap`

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
