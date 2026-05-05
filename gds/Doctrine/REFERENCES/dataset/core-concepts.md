# Core Concepts: Dataset, DataFrame, and Marks

This reference defines the three fundamental layers of the Collections system.

---

## Dataset

**Definition**: The semantic model builder layer above DataFrame. Dataset is where names are
fixed and pipeline intent is authored before runtime.

**Structure**:
```rust
pub struct Dataset {
    pub name: Option<String>,
    pub frame: GDSDataFrame,
    pub artifact_profile: DatasetArtifactProfile,
}
```

**Purpose**: Dataset is not a runtime object. It is a _semantic commitment_. When you create
a Dataset, you are saying:
- "This collection has a name"
- "This collection serves a specific artifact kind" (e.g., Corpus, FeatureMap, ModelView)
- "This collection can be compiled into executable operations"

**Use when**: You want to author semantic intent. Whenever you create a named, staged collection
that will be compiled into artifact records.

**Do not use for**: Direct Polars operations. For raw frame manipulation, use DataFrame directly.

### Dataset View Of Frame (EssentialBeing)

From the Dataset point of view, `Frame` is not merely tabular storage. It is
the immediate beginning that makes mediation possible.

Compactly:

`Frame as EssentialBeing = immediate determinacy available for Dataset mediation`

The asymmetry is required:
- Dataset depends on Frame as its runtime body.
- Frame does not depend on Dataset to exist or execute.

So Dataset reads Frame as the first present body of evidence and then mediates
that body into `Model:Feature::Plan`.

---

## DataFrame

**Definition**: The executable tabular body. DataFrame is Polars-backed and uses lazy and eager
execution modes.

**Key types**:
- `GDSDataFrame`: wrapper over Polars `DataFrame` and `LazyFrame`
- `Expr`: Polars expressions for lazy plan construction
- `Series`: single-column typed data

**Purpose**: DataFrame is where computation happens. It is the _runtime image_ of semantic intent.
When Dataset says "I want this marked," DataFrame executes the mark derivation.

**Use when**: You need to execute operations, filter rows, compute columns, join tables, materialize
results.

**Do not use for**: Semantic naming. Dataset names; DataFrame executes.

From a Dataset perspective, DataFrame behavior is therefore double:
- in itself: executable frame/series/expr substrate
- for Dataset: immediate field of determination to be reflected into mediation

For the fuller ordinary and doctrinal account, see
[Doctrine of the DataFrame](dataframe.md).

---

## Mark

**Definition**: A named, typed semantic token attached to an entity.

**Structure**:
```rust
pub struct Mark {
    pub name: String,
    pub role: MarkRole,     // Quality, Measure, Ground, Determinacy
    pub value_expr: Expr,   // DataFrame expression that computes the mark
}
```

**Roles**:
- **Quality**: the kind or type (e.g., "red", "metal", "acidic")
- **Measure**: the quantity (e.g., "3", "5.7kg", "high intensity")
- **Ground**: the reason or cause (e.g., "grounded_in_middle", "supported_by_principle")
- **Determinacy**: the degree of fixedness (e.g., "determinate", "possible", "contradictory")

**Purpose**: Marks are not columns. A mark is a semantic address that carries:
- A role (what kind of property is this?)
- A value expression (how is it computed?)
- Provenance (where did this mark come from?)

**Use when**: You want to name a derived quantity with semantic significance, not just a computed
column.

**Example**:
- Mark: "quality::color" = determine_color(modality, intensity)
- Mark: "measure::wavelength" = extract_wavelength(spectrum_data)
- Mark: "ground::sensibility" = synthesize(quality, measure)

---

## Feature

**Definition**: A named typed address with projection, binder, reentrancy, and annotation roles.

This is different from "mark." While a mark is a derived value, a feature is the _semantic address_
of that value in the model.

**Use case**: Features appear in model specifications, feature structures, and annotations. They
are the vocabulary of what a model cares about.

**Example**:
- Feature: "color" (a semantic address in the model)
- Mark: "quality::color" (the specific stamp on an entity)
- Value: "red" (the materialized result)

---

## DatasetPipeline

**Definition**: A named, typed envelope for a sequence of Dataset operations.

**Structure**:
```rust
pub struct DatasetPipeline {
    pub specification: Option<SdslSpecification>,
    pub registry: Option<DatasetRegistryExpr>,
    pub io: Option<DatasetIoExpr>,
    pub metadata: Vec<DatasetMetadataExpr>,
    pub ops: Vec<DatasetDataOpExpr>,
    pub projection: Option<DatasetProjectionExpr>,
    pub report: Option<DatasetReportExpr>,
}
```

**Purpose**: A pipeline is not execution. It is _declaration_. You declare:
- Where data comes from (io)
- What metadata is attached
- What operations are applied
- What projections are extracted
- What reports are generated

The pipeline becomes a compiled artifact (DatasetCompilation graph) that the GDS kernel
can execute.

**Use when**: You want to author a complete, named semantic program that will be compiled
and executed.

---

## DatasetCompilation

**Definition**: The compiled graph of all Dataset operations and their dependencies.

**Nodes** (DatasetNodeKind):
- `Image`: the top-level program image
- `Model`: a model specification
- `Feature`: a feature specification
- `Frame`: a DataFrame
- `Series`: a column
- `Expr`: a lazy expression
- `Function`: a reusable operation
- `Macro`: a code generator

**Purpose**: The compilation graph is the visible compiler-IR. Every node has:
- A unique ID
- A name
- A kind
- Dependencies (edges to other nodes)
- Metadata (annotations)

This graph is what the GDS kernel traverses to execute the program.

**Use when**: You need to inspect the structure of a program, trace dependencies, or generate
code.

---

## Artifact

**Definition**: A named semantic product emitted by a Dataset operation.

**Kinds**:
- `Table`: generic tabular data
- `Corpus`: source with lineage
- `FeatureMap`: feature schema
- `ModelView`: model snapshot
- `ModelState`: model runtime state
- `SemanticSubgraph`: graph projection
- `ProgramPlan`: Shell-readable program plan
- `ProgramImage`: compiled program

**Purpose**: Every Dataset operation emits an artifact. Artifacts are:
- Queryable (they are tables)
- Composable (they can be inputs to other operations)
- Traceable (they carry provenance)
- Archivable (they can be stored and replayed)

---

## Provenance

**Definition**: The lineage metadata attached to every artifact.

**Fields**:
- `source_id`: where the data came from
- `specification_id`: which external program specified this
- `program_name`: which program created it
- `generated_at_unix_ms`: when it was created

**Purpose**: Provenance is not decoration. It is the basis of absolute interpretability.
Every artifact answers the question: "How did you come to be?"

**Critical**: Provenance must be mandatory, never optional. Without provenance, an artifact
is untraced and untrustworthy.

---

## Summary

The three layers work together:

1. **Dataset** names and declares semantic intent
2. **DataFrame** executes that intent as tabular operations
3. **Marks** and **Features** carry the semantic tokens
4. **Pipeline** sequences them into programs
5. **Compilation** makes the program structure visible
6. **Artifacts** record what was produced
7. **Provenance** traces the lineage

For stage-by-stage doctrinal placement of abstract terms in concrete software
surfaces, see the Analytical Path Stages section in
`REFERENCES/dataset/semantic-pipeline.md`.

Learn this order. Use Dataset for intent. Use DataFrame for execution. Use marks for semantic tokens.
Use provenance for traceability. Use artifacts for results.
