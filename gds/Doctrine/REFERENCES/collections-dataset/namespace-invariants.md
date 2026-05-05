# Namespace Invariants (Nine-Moment Dataset SDK)

This reference defines the namespace contract used by Doctrine exemplars and generator inputs.

---

## Canonical Pipeline

```text
Frame:Series::Expr -> Model:Feature::Plan -> Corpus:Language::Semantics
```

The nine moments are not only conceptual. They are canonical module homes and should be treated as normative in examples and generated text.

---

## Namespace Doctrine

A namespace is a doctrinal address. It is not merely a Rust module name. Each
namespace says where a kind of work belongs, what it may depend on, and what it
is allowed to explain.

The new examples must therefore document namespaces directly. Every example
should name the namespace it is exercising and persist fixtures under the same
short stem.

Rules:

1. A namespace has a doctrinal role before it has example code.
2. A namespace example must produce or read a durable fixture.
3. A namespace should be named with the same short prefix in Doctrine, examples,
  and fixtures.
4. External DSL examples are exceptions and must name the external surface
  explicitly.

---

## Platform Namespace Roots

| Root | Role | Example Prefix | Fixture Prefix |
|---|---|---|---|
| `form` | PureForm, ProgramFeature, principle-gated return | `form_*` | `fixtures/collections/form_*` |
| `shell` | unified protocol over immediate and mediated registers | `shell_*` | `fixtures/collections/shell_*` |
| `dataframe` | executable Frame:Series::Expr body | `dataframe_*` | `fixtures/collections/dataframe_*` |
| `dataset` | mediated Dataset grammar and artifact production | `dataset_*` | `fixtures/collections/dataset_*` |

The roots are ordered by use, not by ownership. DataFrame supplies immediate
runtime body; Dataset mediates it; Shell governs the path; Form receives the
principle-gated return.

---

## DataFrame Namespace Registry

DataFrame namespaces expose the immediate runtime body. They are allowed to
compute, select, filter, group, join, construct expressions, and materialize
tables. They must not silently become Dataset semantic commitments.

| Namespace | Doctrinal Role | Current Rust Surface | Example Stem |
|---|---|---|---|
| `dataframe::frame` | whole tabular enclosure | `collections::dataframe::frame` / `GDSDataFrame` | `dataframe_intuition` |
| `dataframe::series` | named typed line of appearance | `collections::dataframe::series` | `dataframe_series_concept` |
| `dataframe::expr` | executable operation and judgment grammar | `collections::dataframe::expr`, `expressions::*`, `functions::expr` | `dataframe_expr_basic`, `dataframe_expr_pipeline` |
| `dataframe::lazy` | deferred DataFrame valuation | `collections::dataframe::lazy` | `dataframe_lazy_valuation` |
| `dataframe::selectors` | column selection and address grammar | `collections::dataframe::selectors` | `dataframe_select_filter` |
| `dataframe::macros` | RustScript ergonomic construction surface | `collections::dataframe::macros` | `dataframe_macros_reflection` |
| `dataframe::namespaces::array` | fixed-arity list/array observation | `collections::dataframe::namespaces::array` | `dataset_namespace_array` |
| `dataframe::namespaces::list` | variable-arity list observation | `collections::dataframe::namespaces::list` | `dataset_namespace_list` |
| `dataframe::namespaces::string` | text-form observation | `collections::dataframe::namespaces::string` | `dataset_namespace_string` |
| `dataframe::namespaces::structure` | struct and record projection | `collections::dataframe::namespaces::structure` | `dataframe_structure_projection` |

Future DataFrame namespace examples should feel like RustScript: short,
Polars/Python-inspired, and client-facing. If the example reads like low-level
Rust plumbing, the namespace has not yet found its proper surface.

---

## Canonical Module Homes

### Beginning

- `dataset::frame` (root module at `gds/src/collections/dataset/frame/mod.rs`)
- `dataset::series` (root module at `gds/src/collections/dataset/series/mod.rs`)
- `dataset::expr` (root module at `gds/src/collections/dataset/expr/mod.rs`)

### Essence

- `dataset::model`
  - `dataset::model::prep`
  - `dataset::model::exec`
  - `dataset::model::image`
- `dataset::feature`
  - `dataset::feature::featstruct`
  - `dataset::feature::role`
- `dataset::plan`

### Concept

- `dataset::corpus`
- `dataset::lm`
- `dataset::sem`

---

## Dataset Support Namespace Registry

The nine moments are the canonical pipeline. Dataset also has support
namespaces that feed or expose that pipeline. These must be documented because
they are where most examples and fixtures enter the system.

| Namespace | Doctrinal Role | Current Rust Surface | Example Stem |
|---|---|---|---|
| `dataset::source` | source identity before or beside Corpus | `dataset::source`, `dataset::io`, `DatasetSource` | `dataset_source_corpus` |
| `dataset::stdlib` | curated resource commitments | `dataset::stdlib::*` | `dataset_source_stdlib` |
| `dataset::io` | durable artifact IO and structured source loading | `dataset::io`, `expressions::io` | `dataset_io_json`, `dataset_io_xml_html` |
| `dataset::tree` | observed syntactic and semantic tree structure | `dataset::tree`, `expressions::tree` | `dataset_tree_structures` |
| `dataset::grammar` | grammar-level source structure | `dataset::grammar` | `dataset_tree_structures` |
| `dataset::feature::featstruct` | symbolic feature algebra | `dataset::feature::featstruct` | `dataset_feature_structures` |
| `dataset::compile` | compiler graph and artifact materialization | `dataset::compile`, `DatasetCompilation` | `dataset_compile_ir` |
| `dataset::functions::program` | internal Rust DSL ProgramFeature authoring | `dataset::functions::program` | `dataset_model_feature_plan` |
| `dataset::functions::shell` | DataFrame/Dataset entry helpers | `dataset::functions::shell` | `dataset_frame_dsl` |
| `dataset::macro` | RustScript macro surface over typed constructors | `dataset::macro` | `dataset_frame_dsl` |
| `dataset::namespaces::*` | typed Dataset namespace adapters | `dataset::namespaces::{dataset,feature,text,tree,dataop}` | `dataset_namespace_*` |
| `dataset::registry` | artifact catalog and fixture registration | `dataset::registry` | `dataset_io_catalog_extensible` |
| `dataset::streaming` | procedure and deferred stream collection | `dataset::streaming` | `dataset_streaming_procedure`, `dataset_streaming_lazy` |
| `dataset::toolchain` | internal/external DSL lowering boundary | `dataset::toolchain` | `dataset_compile_ir`, `dataset_compile_external_gdsl` |

Support namespaces must name which canonical moment they serve. For example,
`dataset::functions::program` serves the Essence middle because it authors
`ProgramFeatures` that lower into Dataset compilation artifacts; it is not a
free-standing language authority.

---

## Example Coverage Rule

Every normative namespace needs one of three coverage states:

| State | Meaning |
|---|---|
| `covered` | runnable example exists and writes/reads fixtures |
| `planned` | Doctrine names the namespace and assigns an example stem |
| `external` | textual DSL or external adapter example, explicitly marked |

The initial spine covers the three folds:

| Fold | Example | Fixture Root | State |
|---|---|---|---|
| `Frame:Series::Expr` | `dataframe_intuition.rs` | `fixtures/collections/dataframe_intuition` | `covered` |
| `Model:Feature::Plan` | `dataset_model_feature_plan.rs` | `fixtures/collections/dataset_model_feature_plan` | `covered` |
| `Corpus:LM::SemDataset` | `dataset_sem_meta_pipeline.rs` | `fixtures/collections/dataset_sem_meta_pipeline` | `covered` |

Remaining namespace examples should be added in small batches, with Doctrine
and fixtures updated together.

---

## Compatibility Policy

Compatibility shims remain for legacy callers where canonical and legacy names differ.

Examples:

- `dataset::model_prep` -> `dataset::model::prep`
- `dataset::model_exec` -> `dataset::model::exec`
- `dataset::model_image` -> `dataset::model::image`
- `dataset::featstruct` -> `dataset::feature::featstruct`
- `dataset::feature_role` -> `dataset::feature::role`
- `dataset::semantic` -> `dataset::sem`

Important exception:

- For same-name root modules (`frame`, `series`, `expr`), public paths already remain stable after folderization, so no separate same-name shim files are used.

---

## Generator Rule

Doctrine generators must emit canonical paths by default and should emit shim paths only when targeting explicit backward-compatibility examples.

Default references should prefer:

- `dataset::model::prep` over `dataset::model_prep`
- `dataset::feature::role` over `dataset::feature_role`
- `dataset::sem` over `dataset::semantic`

---

## Review Rule

When adding a new module or exemplar:

1. Place it in its canonical moment first.
2. Add a shim only if canonical and existing public path differ.
3. Keep prelude exports stable while migration is in progress.
4. Update Doctrine references when canonical homes change.
