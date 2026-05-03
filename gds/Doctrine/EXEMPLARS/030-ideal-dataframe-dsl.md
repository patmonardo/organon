# Exemplar 030 — The Ideal DataFrame DSL (Design Fiction)

**Source file**: *none — this is design fiction*
**Arc position**: What the Shell requires of Beginning — the ideal first register
**Prior exemplar**: [029 — GDS Shell as Compute Protocol](029-shell-compute-protocol.md)
**Next exemplar**: TBD

---

> **Staged Design Fiction**: Part of this exemplar is now real. The direct transition
> from a named DataFrame namespace into `GdsShell` exists as
> `into_shell_with_program_features(...)`, and Shell register identity is readable through
> `register_kind()`. The Shell can also return `ShellDataFrameKnowledge`, a compact
> account of the DataFrame as executable Form-body. It can also return
> `ShellModelFeaturePlanKnowledge`, the compact account of Dataset as Model:Feature::Plan.
> The PureForm return is now named as `ShellPureFormReturn` with kind
> `ShellPipelineKind::DataPipeline`.
> The remaining sections still describe the next-generation DataFrame DSL we want: column
> roles, schema declaration before materialization, and named transform nodes.

---

## Principle

The Shell speaks. The DataFrame listens.

When the shell assembles its descriptor, it seeds from the immediate register: column names,
dtypes, row count, schema. It does not need more. But in practice, the DataFrame offers much
more than the shell can directly address — and much of that excess is the wrong shape for
a data-driven framework to consume.

An ideal DataFrame DSL would be designed not as a general-purpose table library but as
the ideal immediate register for the shell. It would know it is a register. It would
speak in the shell's language natively. Every operation would have a name the framework
understands. Every column would be a Series with a Doctrine-aware identity. Every
transformation would be an Expr that knows where it fits in the arc.

This document imagines that DataFrame.

---

## The Shape of the Ideal Beginning

The shell asks three things of the immediate register:

1. **Body**: A tabular body — rows and columns, typed, addressable by name.
2. **Schema Seed**: A lightweight shape descriptor that can be projected to `ShellSchema`
   without materializing the full table.
3. **Register Transition**: A clean `into_shell()` path that does not require the
   framework to unwrap internals.

Everything else the current DataFrame does — lazy evaluation, IO plugins, string namespaces,
array operations, window functions — is valuable but secondary. The shell wants exactly
these three things. An ideal DSL would make them the primary surface.

---

## What the Ideal DSL Would Look Like

### 1. Column Identity (Series with Role)

In the current system, a Series is a named, typed column. Full stop. In the ideal DSL,
every Series knows its role in the arc:

```rust
// Ideal: Series declares its role at construction
let term_series = Series::new("term")
    .typed(ColumnKind::Nominal)
    .annotated(FeatureRole::Target)
    .values(["being", "essence", "concept"]);
```

`ColumnKind` is not a Polars dtype. It is a semantic category:
`Nominal`, `Ordinal`, `Continuous`, `Categorical`, `Temporal`, `Identifier`.
The dtype and the kind are both present. The kind speaks to the framework; the dtype
speaks to the engine.

`FeatureRole` is the Dataset vocabulary (`Target`, `Feature`, `Grouping`, `Weight`,
`Metadata`, `Index`) attached directly to the column — not declared later as a separate
`FeatureStruct`. The column carries its own role from the first moment.

### 2. Frame as Named Register Body

In the current system, a DataFrame is assembled from Series and given a name externally
through `DatasetDataFrameNameSpace`. In the ideal DSL, a Frame is already a named
register body:

```rust
// Ideal: Frame is constructed with its arc position
let frame = Frame::named("shell-concept-demo")
    .at(RegisterKind::Immediate)
    .columns([term_id_series, term_series, moment_series]);
```

`Frame::named(...)` fixes the semantic envelope at construction. `.at(RegisterKind::Immediate)`
declares that this frame is the raw body of an address, not an intermediate computation.
There is no step where an anonymous DataFrame later receives a name from a wrapper.

The framework does not need to discover the name from a `DatasetDataFrameNameSpace`.
The body announces itself.

### 3. Expr as First-Class Pipeline Node

In the current system, an Expr is a Polars lazy computation — powerful, composable, but
opaque to the framework. It has no arc position. The ideal DSL makes Expr a named,
self-describing pipeline node:

```rust
// Ideal: Expr is a named pipeline declaration
let filter_node = Expr::named("concept-filter")
    .at(ArcMoment::Reflection)
    .filter(col("moment").eq(lit("mediated")));

let select_node = Expr::named("term-select")
    .at(ArcMoment::Principle)
    .select([col("term_id"), col("term")]);
```

`ArcMoment` is `Source | Observation | Reflection | Principle | Concept | Judgment |
Syllogism | Procedure`. The Expr knows where it is. When the shell assembles the
descriptor, it can ask: which pipeline nodes are at Reflection? Which are at Principle?
The answer comes from the nodes, not from external metadata.

### 4. Schema Seed Without Materialization

The current `ShellSchema::from_dataframe(...)` requires the full Polars DataFrame to be
present. In the ideal DSL, a Frame can produce a schema seed from its column declarations
before any data is loaded:

```rust
// Ideal: schema seed from declaration alone
let seed = frame.schema_seed();
// -> ShellSchema { columns: ["term_id", "term", "moment"], ... }
// No rows needed. No IO. Just the declared shape.
```

This enables the shell to assemble its address and project to PureFormPrinciple at
declaration time, before any compute begins. The framework knows the shape of every
compute graph before it runs.

### 5. Native Register Transition

The current path from DataFrame to Shell requires:

```rust
// Current:
let frame = ds_frame(table)
    .named("x")
    .artifact_kind(...)
    .facet("...")
    .source_io(...);
let dataset = frame.into_dataset();
let shell = GdsShell::from_dataset(dataset).with_program_features(program);
```

In the ideal DSL, the Frame already speaks shell:

```rust
// Ideal, partially real today through DatasetDataFrameNameSpace:
let shell = frame
    .with_program(program)
    .into_shell();
```

The current concrete version is:

```rust
let shell = ds_frame(table)
    .named("shell-concept-demo")
    .artifact_kind(DatasetArtifactKind::ProgramImage)
    .facet("dialectical-cube")
    .source_io(io_path("memory://shell-concept-demo"))
    .into_shell_with_program_features(program);
```

The namespace knows how to produce a shell because it carries the DataFrame body plus
the semantic envelope. The shell constructor becomes a projection, not an assembly.

The current shell can also answer what it knows about that frame:

```rust
let knowledge = shell.dataframe_knowledge();
```

That knowledge object is deliberately smaller than a DataFrame. It does not expose the
full Polars surface. It names only what the Shell needs: address, columns, dtypes, shape,
schema, semantic envelope, and program identity.

The Shell also knows when the same body has entered the Dataset middle:

```rust
let middle = shell.model_feature_plan_knowledge();
```

This is the first real integration of `Model:Feature::Plan` into the Shell: Dataset gives
the Model envelope, ProgramFeatures give the Feature commitments, and the Shell names the
Plan as the staged return back to Concept.

The return is also explicit:

```rust
let pureform_return = shell.descriptor().to_pure_form_return();
```

For now this return is a `DataPipeline`, not a future multipipeline. That distinction
matters: the Shell is the single pipeline by which DataFrame/Dataset collections clients
return to `PureFormPrinciple`. Later work can introduce additional pipeline kinds, but
the current truth is deliberately simple.

### 6. Fluent ChainExpr Notation

The most visible surface of the ideal DSL is the way computations are chained. Not as
a builder pattern, but as a named sequence of arc transitions:

```rust
// Ideal: ChainExpr as a named sequence
let pipeline = frame
    .chain("observation")
        .filter(col("moment").neq(lit("mediated")))
    .chain("reflection")
        .select([col("term"), col("term_id")])
    .chain("principle")
        .annotate("dialect", "dialectical-cube")
    .compile();
```

Each `.chain(name)` names an arc moment. The `compile()` at the end produces a
`DatasetCompilation` artifact — the same artifact that `005-compile-ir.md` shows
from the other direction. The ideal DSL and the compiler IR are the same thing seen
from the beginning and the end respectively.

### 7. Native Annotation on Columns and Groups

In the current system, semantic annotations (facets, roles, corpus tags) are attached
to Dataset or program declarations externally. In the ideal DSL, annotation is a first-
class column and group operation:

```rust
// Ideal: column-level semantic annotation
let annotated = frame
    .annotate_column("term", AnnotationKind::FeatureTarget)
    .annotate_column("moment", AnnotationKind::CorpusTag("dialectical-arc"))
    .annotate_group("term_id", GroupRole::Identifier);
```

The DataFrame body carries the annotations. The shell reads them directly from the
column metadata when seeding. The `FeatureStruct` in Essence is assembled from what
the columns already declared, not from a separate declaration.

### 8. `select`, `filter`, `group`, `join` as Higher-Kind Operations

In the current system, `select`, `filter`, `group`, `join` are polars operations that
return a new DataFrame. In the ideal DSL, they return a `TransformNode` — a named,
arc-aware operation that is also an Expr:

```rust
// Ideal: operations are named nodes, not just transformations
let transform = frame
    .select_expr("arc-select", [col("term"), col("term_id")])
    .filter_expr("concept-filter", col("moment").eq(lit("immediate")))
    .group_expr("term-group", [col("moment")], [col("term_id").count()]);

// The pipeline is a sequence of named TransformNodes
println!("{:?}", transform.nodes());
// -> ["arc-select", "concept-filter", "term-group"]
```

The shell can ask: what transformations does this pipeline declare? The answer is the
node list. A procedure can inspect, reorder, or replace nodes. The framework governs
execution; the frame governs declaration.

---

## The Minimal DataFrame Contract

Assembled from the above, the minimal ideal contract between DataFrame and Shell:

```
trait ImmediateRegister {
    fn name(&self) -> &str;
    fn register_kind(&self) -> RegisterKind;
    fn dataframe_knowledge(&self) -> ShellDataFrameKnowledge;
    fn schema_seed(&self) -> ShellSchema;
    fn column_names(&self) -> &[String];
    fn dtypes(&self) -> &[DataType];
    fn row_count(&self) -> usize;
    fn column_count(&self) -> usize;
    fn into_shell(self, program: ShellProgram) -> GdsShell;
}
```

The current `GDSDataFrame` already satisfies the read-side of this contract through
`ShellSeed`. The direct transition is now real at the namespace boundary through
`DatasetDataFrameNameSpace::into_shell_with_program_features(...)`, and the current Shell
exposes its non-Polars account through `ShellDataFrameKnowledge`. The remaining gap is not
the bridge itself but the deeper declaration surface: column roles, schema seed before
materialization, and named transformation inventory.

---

## Why This Is the Right Fiction

The fictional DataFrame described here is not a wish list. It is the negative space
left by the Shell's architecture:

- The Shell seeds from column names, dtypes, row count → the DataFrame should make
  these primary, not derived.
- The Shell expects a semantic envelope (name, kind, facet) → the DataFrame should
  carry this natively, not receive it from a wrapper.
- The Shell projects to PureFormPrinciple → the DataFrame's Expr nodes should be
  arc-aware so the projection is exact, not approximated.
- The Shell assembles a descriptor → the DataFrame should be self-describing enough
  that the descriptor is a read, not a computation.

The fiction is faithful because it was written by reading the Shell, not by imagining
features. Every element of the ideal DSL is implied by what the Shell is already asking
for from the immediate register.

---

## What This Means for the Current GDSDataFrame

The current `GDSDataFrame` is not wrong. It is a correctly constrained Polars wrapper
that does what the shell needs today. The ideal DSL does not replace it; it is the
target that future DataFrame iterations should move toward as the shell's address language
becomes more articulated.

The path is incremental:
1. Done: `GdsShell::register_kind()` exposes the shell's inferred register identity.
2. Done: `DatasetDataFrameNameSpace::into_shell_with_program_features(...)` makes the bridge executable.
3. Done: `GdsShell::dataframe_knowledge()` names what Shell knows about the DataFrame as Form-body.
4. Next: named `TransformNode` wrappers over Polars lazy operations.
5. Next: column-level annotation as a first-class DataFrame/Series declaration.
6. Later: schema seed from declaration before data materialization.

Each step is a real code change of modest size. The fiction is already half-built.

---

## The Spectacle

A Data Driven Framework earns the name when the framework, not the programmer, knows
where every piece of data is in the arc. The Shell already knows this at the address
level. The ideal DataFrame would know it at the column level. When both registers speak
the same arc language natively, the Doctrine stops being a description of what the code
does and becomes the code itself: the framework reads like the philosophy because they
are the same structure.

That is the spectacle. Not a clever API. A system whose architecture is readable as
Doctrine and whose Doctrine is executable as architecture.
