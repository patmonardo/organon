# GDS Examples

This directory is the new Doctrine-aligned runnable curriculum. The archived
pre-Doctrine examples live in `gds/examples.old` and are preserved as historical
source material.

## Naming Contract

New examples use short Doctrine names. Do not use the old `collections_` prefix.

Allowed top-level prefixes:

- `form`
- `shell`
- `dataframe`
- `dataset`

These are also the top-level fixture and Doctrine roots, in order:

```text
form -> shell -> dataframe -> dataset
```

Allowed Dataset subdomain prefixes:

- `dataset_source`
- `dataset_io`
- `dataset_tree`
- `dataset_feature`
- `dataset_model`
- `dataset_plan`
- `dataset_compile`
- `dataset_sem`
- `dataset_streaming`
- `dataset_namespace`

Internal DSL examples are the default. In this repo, internal DSL means the
Rust-hosted function, macro, and typed-constructor surface.

External DSL examples must say `external` in the filename. A `.gdsl` file or
string is external DSL even when it is loaded from Rust.

## Fixture Rule

Every new example must use a durable fixture root:

`gds/fixtures/collections/<root>/<example-stem>/`

The `<root>` must match the example prefix: `form`, `shell`, `dataframe`, or
`dataset`.

Generated manifests should use fixture-relative paths, not local absolute paths.
Temporary scratch output belongs in `target`, but canonical example artifacts do
not.

## Doctrine Mapping

| Exemplar | New example | Old archive source |
|---|---|---|
| 001-frame-dsl | `dataset_frame_dsl.rs` | `examples.old/collections_dataset_frame_dsl.rs` |
| 002-corpus-readers | `dataset_source_corpus.rs` | `examples.old/collections_dataset_corpus_readers.rs` |
| 003-tree-structures | `dataset_tree_structures.rs` | `examples.old/collections_dataset_tree.rs` |
| 004-featstruct-model | `dataset_feature_structures.rs` | `examples.old/collections_dataset_featstruct_model.rs` |
| 005-compile-ir | `dataset_compile_ir.rs` | `examples.old/collections_dataset_compile_ir.rs` |
| 006-external-shell-program-artifact | `dataset_compile_external_gdsl.rs` | `examples.old/collections_dataset_gdsl_absolute_concept.rs` |
| 007-applications-expository | `form_applications_expository.rs` | `examples.old/collections_dataset_applications_expository.rs` |
| 008-stdlib-resources | `dataset_source_stdlib.rs` | `examples.old/collections_dataset_stdlib.rs` |
| 009-json-semantic-form | `dataset_io_json.rs` | `examples.old/collections_dataset_json.rs` |
| 010-xml-html-semantic-form | `dataset_io_xml_html.rs` | `examples.old/collections_dataset_xml_html.rs` |
| 011-dataframe-macros-reflection | `dataframe_macros_reflection.rs` | `examples.old/collections_dataframe_macros_expository.rs` |
| 012-select-filter-principle | `dataframe_select_filter.rs` | `examples.old/collections_select_filter.rs` |
| 013-series-concept-surface | `dataframe_series_concept.rs` | `examples.old/collections_series_basic.rs` |
| 014-expr-pipeline-judgment | `dataframe_expr_pipeline.rs` | `examples.old/collections_series_expr_pipeline.rs` |
| 015-order-group-syllogism | `dataframe_order_group.rs` | `examples.old/collections_order_group_exprs.rs` |
| 016-join-operations-inference | `dataframe_join_operations.rs` | `examples.old/collections_join_operations.rs` |
| 017-streaming-dataset-procedure | `dataset_streaming_procedure.rs` | `examples.old/collections_streaming_dataset.rs` |
| 018-streaming-lazy-deferred | `dataset_streaming_lazy.rs` | `examples.old/collections_streaming_lazy.rs` |
| 019-expr-basic-judgment-grammar | `dataframe_expr_basic.rs` | `examples.old/collections_expr_basic.rs` |
| 020-framing-chunking-pureform | `form_framing_chunking.rs` | `examples.old/collections_extensions_framing_chunking.rs` |
| 021-array-namespace-observation | `dataset_namespace_array.rs` | `examples.old/collections_array_namespace_rustscript.rs` |
| 022-list-namespace-observation | `dataset_namespace_list.rs` | `examples.old/collections_list_namespace_rustscript.rs` |
| 023-string-namespace-observation | `dataset_namespace_string.rs` | `examples.old/collections_string_namespace_rustscript.rs` |
| 024-catalog-extension-persistence | `dataset_io_catalog_extensible.rs` | `examples.old/collections_catalog_extensible.rs` |
| 025-graphframe-catalog-write | `dataset_io_catalog_graphframe.rs` | `examples.old/collections_graphframe_catalog_write.rs` |
| 026-semantic-meta-pipeline | `dataset_sem_meta_pipeline.rs` | `examples.old/collections_semantic_meta_pipeline.rs` |
| 027-model-feature-plan-middle | `dataset_model_feature_plan.rs` | `examples.old/collections_model_feature_plan.rs` |
| 028-dataframe-intuition | `dataframe_intuition.rs` | `examples.old/collections_dataframe_intuition.rs` |
| 029-shell-compute-protocol | `shell_compute_protocol.rs` | `examples.old/collections_gds_shell.rs` |
| 030-ideal-dataframe-dsl | design-only for now | none |

## Batch 1 Spine

Start with a small executable spine before rebuilding the whole curriculum:

1. `dataframe_intuition.rs`
2. `dataset_frame_dsl.rs`
3. `dataset_compile_ir.rs`
4. `dataset_model_feature_plan.rs`
5. `dataset_sem_meta_pipeline.rs`
6. `shell_compute_protocol.rs`

## Namespace Coverage

The examples are not only demos. They are executable namespace documentation.
Each normative namespace should eventually have one fixture-backed example.

Coverage states:

- `covered`: runnable example exists and writes/reads fixtures
- `planned`: Doctrine names the namespace and assigns an example stem
- `external`: textual DSL or external adapter example, explicitly marked

Initial covered spine:

| Namespace Fold | Example | Fixture Root | State |
|---|---|---|---|
| `Frame:Series::Expr` | `dataframe_intuition.rs` | `fixtures/collections/dataframe/dataframe_intuition` | `covered` |
| `Model:Feature::Plan` | `dataset_model_feature_plan.rs` | `fixtures/collections/dataset/dataset_model_feature_plan` | `covered` |
| `Corpus:LM::SemDataset` | `dataset_sem_meta_pipeline.rs` | `fixtures/collections/dataset/dataset_sem_meta_pipeline` | `covered` |

Form namespace coverage:

| Namespace | Example | Fixture Root | State |
|---|---|---|---|
| `form::framing_chunking` | `form_framing_chunking.rs` | `fixtures/collections/form/form_framing_chunking` | `covered` |

Shell namespace coverage:

| Namespace | Example | Fixture Root | State |
|---|---|---|---|
| `shell::compute` | `shell_compute_protocol.rs` | `fixtures/collections/shell/shell_compute_protocol` | `covered` |

Planned DataFrame namespace examples:

| Namespace | Example | State |
|---|---|---|
| `dataframe::series` | `dataframe_series_concept.rs` | `covered` |
| `dataframe::expr` | `dataframe_expr_basic.rs`, `dataframe_expr_pipeline.rs` | `covered` |
| `dataframe::frame` | `dataframe_frame_surface.rs` | `covered` |
| `dataframe::lazy` | `dataframe_lazy_valuation.rs` | `covered` |
| `dataframe::selectors` | `dataframe_select_filter.rs` | `covered` |
| `dataframe::macros` | `dataframe_macros_reflection.rs` | `covered` |
| `dataframe::namespaces::structure` | `dataframe_structure_projection.rs` | `planned` |

Planned Dataset support namespace examples:

| Namespace | Example | State |
|---|---|---|
| `dataset::source` | `dataset_source_corpus.rs` | `covered` |
| `dataset::stdlib` | `dataset_source_stdlib.rs` | `planned` |
| `dataset::io` | `dataset_io_json.rs`, `dataset_io_xml_html.rs` | `planned` |
| `dataset::tree` | `dataset_tree_structures.rs` | `planned` |
| `dataset::feature::featstruct` | `dataset_feature_structures.rs` | `planned` |
| `dataset::compile` | `dataset_compile_ir.rs` | `covered` |
| `dataset::functions::program` | `dataset_model_feature_plan.rs` | `covered` |
| `dataset::functions::shell` | `dataset_frame_dsl.rs` | `covered` |
| `dataset::macro` | `dataset_frame_dsl.rs` | `covered` |
| `dataset::registry` | `dataset_io_catalog_extensible.rs` | `planned` |
| `dataset::streaming` | `dataset_streaming_procedure.rs`, `dataset_streaming_lazy.rs` | `planned` |
| `dataset::toolchain` | `dataset_compile_ir.rs`, `dataset_compile_external_gdsl.rs` | `covered` / `external` |
