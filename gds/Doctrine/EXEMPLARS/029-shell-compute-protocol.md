# Exemplar 029 — GDS Shell as Compute Protocol

**Source file**: `gds/examples/collections_gds_shell.rs`
**Arc position**: Protocol layer — Shell unifies Beginning (DataFrame) and Essence (Dataset) into a single addressable compute language
**Prior exemplar**: [028 — DataFrame as Intuition](028-dataframe-intuition.md)
**Next exemplar**: [030 — Ideal DataFrame DSL (Design Fiction)](030-ideal-dataframe-dsl.md)

---

## Principle

The Shell is the internal language in which the GDS kernel coordinates pipeline execution.
It does not replace DataFrame or Dataset. It holds them both as distinct registers at a
shared address and speaks on their behalf toward PureForm.

This is the principle of a proper shell: the Shell is the Living Doctrine Runtime.
Projection validity is not asserted by claim, but by Shell-traceability through one
canonical line:

```text
Frame -> Model:Feature::Plan -> SemDataset -> PureForm return
```

The example architecture now has an explicit runtime check for this law:
`GdsShell::validate_projection_trace()` returns a structured report of required steps,
observed steps, and missing steps; `GdsShell::is_projection_trace_valid()` provides the
boolean gate.

It also has a personal progression surface: `GdsShell::learning_report()` returns a
`ShellLearningReport` naming what the Dataset has concretely learned into SemDataset
(gained principles/concepts/procedures), what remains unresolved, and whether
fundamental NLP graph and mathematical logic readiness has been reached.

Before the Shell, DataFrame and Dataset were two separate objects with overlapping concerns.
After the Shell, they are two moments of one address: the immediate register carries the
raw Polars body; the mediated register carries the semantic expansion through ProgramFeatures.
The Shell descriptor computes the bridge between them and projects the result to
`PureFormPrinciple` without collapsing either side.

This is the architectural move that makes GDS a Data Driven Framework rather than a
pipeline library: the framework speaks in addresses, not objects.

This also establishes a scale law: even if downstream GML/GNN execution is massive,
the MetaPipeline authority remains with the containing Dataset (as read by Shell),
not with graph-store size or compute complexity.

---

## What This Example Does

1. **Builds a DataFrame** using `tbl_def!` — three rows encoding Being/Essence/Concept
   as immediate tabular content. This is the raw body: no semantic intent yet.

2. **Wraps the DataFrame in a `DatasetDataFrameNameSpace`** (`ds_frame(...)`) and assigns
   it a name, artifact kind, facet, and source IO path. The frame now has a semantic
   envelope; it is no longer anonymous.

3. **Keeps the namespace as the transition surface**. The namespace can still materialize
   a `Dataset`, but the exemplar now enters the Shell directly through
   `into_shell_with_program_features(...)`.

4. **Declares a `ShellProgram`** via `program_features(...)` with five moments:
   `program_source`, `program_reflection`, `program_principle`, `program_concept`,
   `program_procedure`. These are not operations. They are declarations of what the
   compute graph is doing — the program knows its own arc position.

5. **Enters `GdsShell` directly from the named DataFrame namespace**. The shell now holds
   both registers, the schema seed extracted from the DataFrame body, and the program
   declaration.

6. **Queries `ShellDataFrameKnowledge`** — the shell's compact account of what the
   DataFrame contributes to executable Form: address, columns, dtypes, shape, semantic
   envelope, and program identity.

7. **Queries `ShellModelFeaturePlanKnowledge`** — the shell's account of the Dataset
   middle. It reads Model from the Dataset envelope, Feature from `ProgramFeatures`,
   and Plan from the concept-return order.

8. **Queries `ShellSemanticPipelineKnowledge`** — the shell's account of SemDataset
   evolution. It reports readiness checkpoints (`frame`, `middle`, `semdataset`,
   `pureform return`) and capability classes that define semantic intelligence for SDSL.

9. **Queries the `ShellPipelineDescriptor`** — the shell's self-description. It answers:
   - What is my address in the 3³ cube?
   - Do I have an immediate body? A mediated body? A metapipeline?
   - What does my PureFormPrinciple look like?

10. **Returns as `ShellPureFormReturn`** (`descriptor.to_pure_form_return()`), naming the
   current Shell pipeline as a `DataPipeline` and carrying the `PureFormPrinciple` that
   PureFormProcessors can consume without knowing whether they are operating on a
   DataFrame or Dataset.

11. **Adds pipeline operations** through `dataset_pipeline()` — text input, encode,
   transform, output — showing that the shell governs pipeline construction after the
   address is fixed.

---

## The Arc Reading

```text
tbl_def! -> DataFrame                       [Immediate register: raw body]
         -> DatasetDataFrameNameSpace        [Named semantic envelope]
         -> program_features(...)            [Program declaration: arc self-knowledge]
         -> into_shell_with_program_features  [Direct register transition]
         -> GdsShell                         [Unified address: both registers present]
         -> ShellDataFrameKnowledge           [What Shell knows about the DataFrame]
         -> ShellModelFeaturePlanKnowledge    [What Shell knows about the Dataset middle]
         -> ShellSemanticPipelineKnowledge    [What Shell knows about SemDataset evolution]
         -> ShellPipelineDescriptor          [Shell's self-description]
         -> ShellPureFormReturn              [DataPipeline returning to PureForm]
         -> PureFormPrinciple                [Shape:Context::Morph]
         -> DatasetPipeline ops              [Execution after address is fixed]
```

The shell does not execute. It assembles, names, and describes. Execution comes from
procedures that read the descriptor.

---

## The 3³ Shell Address

Every shell sits at a coordinate in the language cube:

| Axis | Variants | What it names |
|---|---|---|
| `ShellRegister` | `ImmediateDataFrame`, `MediatedDataset`, `Unified` | Which body is present |
| `ShellPipeline` | `Frame`, `ModelFeaturePlan`, `TableImage` | Which pipeline moment is active |
| `ShellAlgebra` | `Schema`, `FeatStruct`, `ProgramFeature` | Which algebra governs this moment |

A fully assembled shell sits at `(Unified, ModelFeaturePlan, ProgramFeature)` — both
registers present, in the Essence middle, with program declarations active.

The descriptor makes this readable without exposing the internal Polars or Dataset objects:
a procedure sees an address and a principle, not a DataFrame.

---

## Shell Moments

`ShellMoment` is the simplified triad that maps the three moments of the Dataset arc
into the shell language:

| ShellMoment | Maps to | Meaning |
|---|---|---|
| `Frame` | DataFrame / Beginning | Immediate tabular body is present |
| `ModelFeaturePlan` | Model:Feature::Plan / Essence | Schema, typing, and feature intent are assembled |
| `Table` | Table / Image / Concept | Realized exchange form is available |

These are not pipeline steps. They are recognition events: the shell knows which moment
has been reached because the right fields are populated.

At `ModelFeaturePlan`, the Shell comes into its own. It does not merely carry a Dataset.
It can name the middle explicitly:

| Dataset middle | Shell account |
|---|---|
| `Model` | Dataset name and artifact profile |
| `Feature` | ProgramFeature declarations grouped by role |
| `Plan` | Concept-return order from DataFrame seed through Dataset middle to Shell program |

This is now exposed as `GdsShell::model_feature_plan_knowledge()`.

---

## DataFrame and Dataset as Registers, Not Rivals

The crucial insight this exemplar teaches:

**DataFrame is the immediate register** — it supplies the raw body, the column names,
the dtypes, the row count. It is the body that the shell is a body of.

**Dataset is the mediated register** — it wraps the same body with name, artifact kind,
facet, IO path, program features, and pipeline. It is the body in the framework.

Before the Shell, it looked like Dataset was a replacement for DataFrame, or a
heavyweight alternative. The shell dissolves that confusion: both registers are present
at the same address. The shell seeds from the DataFrame, expands through the Dataset, and
projects both to a descriptor that neither register could produce alone.

---

## What the Shell Reveals About DataFrame

The shell is architecturally more fundamental than the DataFrame. When the shell
asks for what it needs from the immediate register, it specifies the minimum:

- Column names (as strings)
- Data types (as Polars `DataType`)
- Row count and column count
- A `ShellSchema` seeded from the frame body

That is all the immediate register must supply. A DataFrame that does exactly this —
and exposes a clean `into_shell()` transition — is all the framework needs at the
Beginning moment. The shell does not need the full Polars API. It needs a body,
a shape, and a path to the mediated register.

The first real API for that path is now the namespace transition:
`DatasetDataFrameNameSpace::into_shell_with_program_features(...)`. The DataFrame body
does not become Dataset by identity. It becomes readable by Shell as the immediate
register of a form that also has a mediated Dataset register.

The next real API is `GdsShell::dataframe_knowledge()`. This gives procedures and
examples a single place to ask what the shell knows about the DataFrame: its address,
columns, dtypes, shape, schema, Dataset envelope, and Program identity. This is the
minimal non-Polars account of a DataFrame as Form-body.

The paired middle API is `GdsShell::model_feature_plan_knowledge()`. This is where the
Shell integrates the Dataset ideas of Model, Feature, and Plan: Model from the mediated
Dataset object, Feature from typed ProgramFeature commitments, and Plan from the return
sequence that leads back to Shell Concept.

The semantic evolution API is `GdsShell::semantic_pipeline_knowledge()`. This is the
control surface for Dataset -> SemDataset progression: it exposes readiness checkpoints
and capability classes so semantic growth is guided, inspectable, and aligned with one
DataPipeline return.

This also explains Dataset evolution toward SemDataset. The Shell shows that Dataset is not
static metadata around a table. It is a staged semantic controller that becomes richer as
Model, Feature, Plan, and ProgramFeature commitments accumulate. By the time Corpus and
LanguageModel are paired, the same mediated line has matured into SemDataset.

In short: Dataset becomes SemDataset by getting smarter through the middle, and the Shell
is the protocol that keeps that intelligence tied to one DataPipeline.

The return API is `ShellPipelineDescriptor::to_pure_form_return()`. This is the exact
meaning of “return to PureForm” in the current system: one Shell `DataPipeline` has
gathered DataFrame body, Dataset middle, and Program declaration, then returns them as a
`PureFormPrinciple` with `Shape:Context::Morph`. It is not yet a multipipeline system.
The Shell is currently one kind of pipeline: the DataPipeline.

This is the negative space where the ideal DataFrame DSL lives. See
[Exemplar 030](030-ideal-dataframe-dsl.md) for the design fiction.

---

## Namespace Discipline

| Symbol | Canonical path |
|---|---|
| `GdsShell` | `gds::shell::GdsShell` |
| `ShellDataFrameKnowledge` | `gds::shell::ShellDataFrameKnowledge` |
| `ShellModelFeaturePlanKnowledge` | `gds::shell::ShellModelFeaturePlanKnowledge` |
| `ShellSemanticPipelineKnowledge` | `gds::shell::ShellSemanticPipelineKnowledge` |
| `ShellSemanticCapability` | `gds::shell::ShellSemanticCapability` |
| `ShellPureFormReturn` | `gds::shell::ShellPureFormReturn` |
| `ShellPipelineKind::DataPipeline` | `gds::shell::ShellPipelineKind` |
| `ShellSeed` | `gds::shell::ShellSeed` |
| `ShellPipelineDescriptor` | `gds::shell::ShellPipelineDescriptor` |
| `ShellProgram` | `gds::shell::ShellProgram` |
| `ShellSchema` | `gds::shell::ShellSchema` |
| `ShellAddress` | `gds::shell::ShellAddress` |
| `ShellRegister` / `ShellPipeline` / `ShellAlgebra` | `gds::shell::moments` |
| `PureFormPrinciple` | `gds::form::PureFormPrinciple` |
| `program_features` / `program_source` etc. | `gds::dsl::*` (re-exported via `gds::shell::*`) |
