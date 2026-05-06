# Fourfold Alignment Gaps

Purpose: execute only what is missing across form -> shell -> dataframe -> dataset.

## Rule

Do not upgrade already covered paths unless they block a missing path.

## Current Reality (2026-05-06)

Covered runnable examples:
- form_framing_chunking.rs
- shell_compute_protocol.rs
- dataframe_frame_surface.rs
- dataframe_lazy_valuation.rs
- dataframe_intuition.rs
- dataframe_series_concept.rs
- dataframe_expr_basic.rs
- dataframe_expr_pipeline.rs
- dataframe_select_filter.rs
- dataframe_macros_reflection.rs
- dataset_model_feature_plan.rs
- dataset_sem_meta_pipeline.rs
- dataset_frame_dsl.rs
- dataset_source_corpus.rs
- dataset_compile_ir.rs
- dataset_compile_external_gdsl.rs
- dataset_tree_structures.rs
- dataset_feature_structures.rs
- dataset_io_catalog_extensible.rs
- dataset_streaming_procedure.rs
- dataset_io_json.rs
- dataframe_order_group.rs
- dataframe_join_operations.rs
- dataset_namespace_list.rs
- dataset_namespace_string.rs
- dataset_namespace_array.rs
- dataset_io_xml_html.rs
- dataset_source_stdlib.rs
- dataset_streaming_lazy.rs
- form_applications_expository.rs
- dataset_io_catalog_graphframe.rs
- dataframe_namespace_string.rs (supplemental)

Missing runnable examples from mapped curriculum:
- none

## Fourfold Missingness

Form:
- `form::framing_chunking` is covered by `form_framing_chunking.rs`.
- Additional form namespaces are still pending.

Shell:
- `shell::compute` is covered by `shell_compute_protocol.rs`.
- Additional shell namespaces are still pending.

DataFrame:
- `frame` and `lazy` are covered.
- `series` is covered by `dataframe_series_concept.rs`.
- `expr` is covered by `dataframe_expr_basic.rs` and `dataframe_expr_pipeline.rs`.
- `selectors` is covered by `dataframe_select_filter.rs`.
- `macros` is covered by `dataframe_macros_reflection.rs`.
- `namespaces::structure` and higher-order joins/groups remain planned.

Dataset:
- Middle and semantic pipeline are covered.
- Intake and compile lanes are covered by `dataset_frame_dsl.rs`, `dataset_source_corpus.rs`, `dataset_compile_ir.rs`, and `dataset_compile_external_gdsl.rs`.
- Structure/feature/registry/streaming lanes are now covered by `dataset_tree_structures.rs`, `dataset_feature_structures.rs`, `dataset_io_catalog_extensible.rs`, and `dataset_streaming_procedure.rs`.
- IO xml/html lane is covered by `dataset_io_xml_html.rs`.
- Namespace array lane is covered by `dataset_namespace_array.rs`.

## Sequencing (Lack-First)

1. Low-priority deferred:
- none

## Done Criteria

For each missing example:
- Example file exists under gds/examples.
- Fixture root exists under gds/fixtures/collections/<root>/<example-stem>.
- Fixture manifest uses rooted relative paths.
- Exemplar source line points to the runnable file.
- Namespace coverage row moves from planned to covered.
- cargo run -p gds --example <name> succeeds.

## Guardrails

- Keep top-level root order fixed: form -> shell -> dataframe -> dataset.
- Keep DataFrame and Dataset references separate (no recombined authority root).
- Treat external `.gdsl` as shell artifact format, not governing runtime authority.
