# DataOps/DataExprs Note (Polars-style Client DSL)

## Goal

Define a minimal, Polars/SQL-aligned DataOps surface that is faithful to the existing client DSL in [gds/src/collections/dataframe](gds/src/collections/dataframe). This is the foundation for future StatFrame and GraphFrame-style extensions without coupling to statistical modeling.

## Current client DSL anchors

- DataFrame/LazyFrame shape and dataflow are the top-level entrypoints.
- Series/Expr composition is already modeled on the Python Polars client DSL.
- Namespaces exist for Series and Expr (string/list/datetime/etc.).

See:

- [gds/src/collections/dataframe/series.rs](gds/src/collections/dataframe/series.rs)
- [gds/src/collections/dataframe/expr.rs](gds/src/collections/dataframe/expr.rs)
- [gds/src/collections/dataframe/functions](gds/src/collections/dataframe/functions)
- [gds/src/collections/dataframe/expressions](gds/src/collections/dataframe/expressions)
- [gds/src/collections/dataframe/namespaces](gds/src/collections/dataframe/namespaces)

## Naming stance (proposed)

- DataOps = the chain or pipeline surface across DataFrame/LazyFrame.
- DataExprs = the expression DSL (functions are chained into Exprs).
- Functions = constructors and helpers that yield Exprs or Series (e.g., `col`, `lit`, `when/then`).

## Core DataOps (minimal surface)

DataOps should reflect Polars/SQL mechanics with key/index features for hierarchical text (book/section/chapter/verse/line/word). The following are the minimum semantics to keep coherent with the client DSL:

- Selection: `select`, `with_columns`, `drop`, `rename`.
- Filtering: `filter`, `where`.
- Aggregation: `group_by`, `agg`.
- Join: `join`, `left`, `inner`, `outer` with explicit key/index support.
- Sort: `sort`, `sort_by`.
- Projection: `project` from Text/PropertyGraph -> indexed DataFrame.
- IO (basic dataset operations): `scan`, `read`, `write`, `download`, `unzip` (keep separate from StatFrame).

## DataExprs (Expr-first)

DataExprs are built from functions and chained into expressions. The split stays aligned with Polars:

- Column constructors: `col`, `lit`, `len`, `repeat`, `range`.
- Branching: `when/then/otherwise`.
- Namespaces: string/list/datetime/etc. as Expr namespace adaptors.
- Selectors: structured column selection for bulk expressions.

This keeps the chain shape:

- `DataOps -> Expr` via `with_columns`, `select`, `filter`.
- `Series -> Expr` via namespaces in [gds/src/collections/dataframe/expr.rs](gds/src/collections/dataframe/expr.rs).

## API design 101 (Expr vs Series)

We should treat the skrub-style toolset as two layers:

- Exprs are lazy, parallelizable DataOps and belong to LazyFrame pipelines.
- Series operations are eager and belong to DataFrame (materialized) workflows.

This implies a structured split:

- Functions are small builders that produce Exprs (or Expr namespaces).
- Container Exprs are composed into LazyFrame plans.
- Series helpers are only for eager, column-level logic that cannot be expressed lazily.

In practice, every skrub utility should be classified as:

- Expr builder (lazy, composable), or
- Series helper (eager, column-level), or
- Dataset utility (IO/metadata/reporting above the DataFrame DSL).

## Key/index system (hierarchical text)

We want Polars/SQL-aligned keys, but with hierarchical paths. Proposed concept:

- Key fields are standard columns (`book_id`, `section_id`, `chapter_id`, `verse_id`, `line_id`, `word_id`).
- Keys are defined by declarative metadata (schema or table config) and enforced by join rules.
- Index lists are derived tables with ordering semantics (e.g., `word_id` has a natural order within `line_id`).

DataOps should express:

- `keyed_by([cols...])` or `with_key([cols...])` to declare primary keys.
- `indexed_by([cols...])` to declare ordered path segments.
- `join_on_keys` (syntactic sugar over `join` + key metadata) for stability.

## PropertyGraph extension points (non-RDF)

We can extend DataOps/Exprs without adopting RDF:

- PropertyGraph tables are just DataFrames with schema conventions:
  - `nodes`: `node_id`, `type`, `props` (struct/map).
  - `edges`: `edge_id`, `src_id`, `dst_id`, `type`, `props`.
- The same join/filter/sort mechanics apply.
- Projection is simply `nodes` + `edges` into DataOps, then used for text/graph analytics.

## Boundaries

- DataOps/DataExprs stay model-agnostic (no StatFrame, no model fit/eval).
- StatFrame is a separate layer that consumes DataOps results.
- GraphFrame can be a higher-order library that builds on the same key/index + join semantics.

## Dataset utility layer (above DataFrame DSL)

Dataset utilities sit above the DataFrame client DSL and should stay light:

- Download/unzip, dataset registry, and local cache management.
- Reporting hooks (table summaries, lightweight ETL diagnostics).
- Data cleaning utilities that are deterministic and do not require model fitting.

The dataset plan macros already exist as a higher-level façade. The skrub-inspired alias `dop!` is defined in [gds/src/collections/dataset/macro.rs](gds/src/collections/dataset/macro.rs).

## Seed pass scope (1/3 coverage)

This pass targets the most fundamental third of DataOps that any Polars/SQL client DSL expects:

- Frame and LazyFrame basics: select, with_columns, filter, group_by/agg, join, sort.
- Expr builders: col/lit/when-then/alias, string/list/datetime namespaces.
- Selector and schema helpers: column selection and dtype-driven selection.
- Deterministic ETL helpers: rename, cast, drop, dedupe, null cleanup.

Advanced DataOps are intentionally stubbed for now (window ops, complex joins, streaming, optimizers, and specialized compute). The intent is to reserve the API surface and iterate later as the DataFrame DSL matures.

## NLP-first dataset note

We can keep the NTLK-inspired NLP-first dataset ideas as an overlay in the Dataset layer, but the base remains the DataFrame client DSL. Text datasets should be projectable into indexed DataFrames, with a path to richer GraphFrame semantics later.

## Seed extraction map (from skrub)

This is a rough, first-pass split to help define a Dataset-oriented DataOps layer. It is intentionally conservative and does not need to be perfect yet.

Dataset/DataOps candidates (keep, or adapt):

- DataFrame adapters + selectors: [gds/src/skrub/\_dataframe](gds/src/skrub/_dataframe), [gds/src/skrub/selectors](gds/src/skrub/selectors)
- Dataset IO and local assets: [gds/src/skrub/datasets](gds/src/skrub/datasets)
- Join utilities and standard joins: [gds/src/skrub/\_join_utils.py](gds/src/skrub/_join_utils.py), [gds/src/skrub/\_agg_joiner.py](gds/src/skrub/_agg_joiner.py), [gds/src/skrub/\_multi_agg_joiner.py](gds/src/skrub/_multi_agg_joiner.py)
- Column/shape hygiene: [gds/src/skrub/\_check_input.py](gds/src/skrub/_check_input.py)
- Lightweight column transforms (DataOps-adjacent): [gds/src/skrub/\_select_cols.py](gds/src/skrub/_select_cols.py), [gds/src/skrub/\_to_str.py](gds/src/skrub/_to_str.py), [gds/src/skrub/\_to_float.py](gds/src/skrub/_to_float.py), [gds/src/skrub/\_to_datetime.py](gds/src/skrub/_to_datetime.py), [gds/src/skrub/\_to_categorical.py](gds/src/skrub/_to_categorical.py)
- Basic table transforms (non-ML semantics): [gds/src/skrub/\_deduplicate.py](gds/src/skrub/_deduplicate.py), [gds/src/skrub/\_clean_null_strings.py](gds/src/skrub/_clean_null_strings.py), [gds/src/skrub/\_clean_categories.py](gds/src/skrub/_clean_categories.py)

StatFrame/ML candidates (do not include in core DataOps):

- DataOps graph + estimator interface: [gds/src/skrub/\_data_ops](gds/src/skrub/_data_ops)
- Hyperparam search/reporting: [gds/src/skrub/\_optuna.py](gds/src/skrub/_optuna.py), [gds/src/skrub/\_parallel_coord.py](gds/src/skrub/_parallel_coord.py)
- Encoders/learned transforms: [gds/src/skrub/\_gap_encoder.py](gds/src/skrub/_gap_encoder.py), [gds/src/skrub/\_minhash_encoder.py](gds/src/skrub/_minhash_encoder.py), [gds/src/skrub/\_string_encoder.py](gds/src/skrub/_string_encoder.py), [gds/src/skrub/\_text_encoder.py](gds/src/skrub/_text_encoder.py), [gds/src/skrub/\_similarity_encoder.py](gds/src/skrub/_similarity_encoder.py)
- Modeling pipelines: [gds/src/skrub/\_table_vectorizer.py](gds/src/skrub/_table_vectorizer.py), [gds/src/skrub/\_tabular_pipeline.py](gds/src/skrub/_tabular_pipeline.py), [gds/src/skrub/\_interpolation_joiner.py](gds/src/skrub/_interpolation_joiner.py)

Needs review (depends on where we draw the boundary):

- Fuzzy joins and matchers: [gds/src/skrub/\_fuzzy_join.py](gds/src/skrub/_fuzzy_join.py), [gds/src/skrub/\_joiner.py](gds/src/skrub/_joiner.py), [gds/src/skrub/\_matching.py](gds/src/skrub/_matching.py)
- Association diagnostics: [gds/src/skrub/\_column_associations.py](gds/src/skrub/_column_associations.py)
- Utility and config helpers: [gds/src/skrub/\_utils.py](gds/src/skrub/_utils.py), [gds/src/skrub/\_config.py](gds/src/skrub/_config.py)

## Skrub module mapping (seed pass)

This is a first-pass classification to guide the Expr/Series/utility split. It is not final and should be revised as we implement namespaces.

SKLearn-generic (non-statistical) bucket: utilities that are safe for the Dataset/DataFrame DSL (no model training or statistical optimization). These can land in Series helpers or Dataset utilities.

| Module                                                                                      | Primary role         | Target layer    |
| ------------------------------------------------------------------------------------------- | -------------------- | --------------- |
| [gds/src/skrub/\_dataframe](gds/src/skrub/_dataframe)                                       | DataFrame adapter    | Dataset utility |
| [gds/src/skrub/selectors](gds/src/skrub/selectors)                                          | Column selection     | Expr builder    |
| [gds/src/skrub/datasets](gds/src/skrub/datasets)                                            | Fetch/cache/registry | Dataset utility |
| [gds/src/skrub/\_join_utils.py](gds/src/skrub/_join_utils.py)                               | Join helpers         | Expr builder    |
| [gds/src/skrub/\_agg_joiner.py](gds/src/skrub/_agg_joiner.py)                               | Grouped join         | Expr builder    |
| [gds/src/skrub/\_multi_agg_joiner.py](gds/src/skrub/_multi_agg_joiner.py)                   | Multi-join           | Expr builder    |
| [gds/src/skrub/\_select_cols.py](gds/src/skrub/_select_cols.py)                             | Column selection     | Expr builder    |
| [gds/src/skrub/\_apply_to_cols.py](gds/src/skrub/_apply_to_cols.py)                         | Columnwise apply     | Series helper   |
| [gds/src/skrub/\_apply_to_frame.py](gds/src/skrub/_apply_to_frame.py)                       | Frame apply          | Series helper   |
| [gds/src/skrub/\_to_str.py](gds/src/skrub/_to_str.py)                                       | Cast to string       | Series helper   |
| [gds/src/skrub/\_to_float.py](gds/src/skrub/_to_float.py)                                   | Cast to float        | Series helper   |
| [gds/src/skrub/\_to_datetime.py](gds/src/skrub/_to_datetime.py)                             | Cast to datetime     | Series helper   |
| [gds/src/skrub/\_to_categorical.py](gds/src/skrub/_to_categorical.py)                       | Cast to categorical  | Series helper   |
| [gds/src/skrub/\_deduplicate.py](gds/src/skrub/_deduplicate.py)                             | Deduping             | Series helper   |
| [gds/src/skrub/\_clean_null_strings.py](gds/src/skrub/_clean_null_strings.py)               | Null cleanup         | Series helper   |
| [gds/src/skrub/\_clean_categories.py](gds/src/skrub/_clean_categories.py)                   | Category cleanup     | Series helper   |
| [gds/src/skrub/\_drop_uninformative.py](gds/src/skrub/_drop_uninformative.py)               | Drop sparse info     | Series helper   |
| [gds/src/skrub/\_single_column_transformer.py](gds/src/skrub/_single_column_transformer.py) | Single-col base      | Series helper   |
| [gds/src/skrub/\_wrap_transformer.py](gds/src/skrub/_wrap_transformer.py)                   | Transformer wrapper  | Series helper   |
| [gds/src/skrub/\_check_input.py](gds/src/skrub/_check_input.py)                             | Input hygiene        | Dataset utility |
| [gds/src/skrub/\_column_associations.py](gds/src/skrub/_column_associations.py)             | Diagnostics          | Dataset utility |
| [gds/src/skrub/\_reporting](gds/src/skrub/_reporting)                                       | Reporting UI         | Dataset utility |
| [gds/src/skrub/\_dispatch.py](gds/src/skrub/_dispatch.py)                                   | Dispatch helpers     | Dataset utility |
| [gds/src/skrub/\_data_ops](gds/src/skrub/_data_ops)                                         | DataOps graph        | StatFrame/ML    |
| [gds/src/skrub/\_table_vectorizer.py](gds/src/skrub/_table_vectorizer.py)                   | Feature pipeline     | StatFrame/ML    |
| [gds/src/skrub/\_tabular_pipeline.py](gds/src/skrub/_tabular_pipeline.py)                   | Pipeline             | StatFrame/ML    |
| [gds/src/skrub/\_datetime_encoder.py](gds/src/skrub/_datetime_encoder.py)                   | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_gap_encoder.py](gds/src/skrub/_gap_encoder.py)                             | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_minhash_encoder.py](gds/src/skrub/_minhash_encoder.py)                     | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_string_encoder.py](gds/src/skrub/_string_encoder.py)                       | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_text_encoder.py](gds/src/skrub/_text_encoder.py)                           | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_similarity_encoder.py](gds/src/skrub/_similarity_encoder.py)               | Encoder              | StatFrame/ML    |
| [gds/src/skrub/\_scaling_factor.py](gds/src/skrub/_scaling_factor.py)                       | Scaling helper       | StatFrame/ML    |
| [gds/src/skrub/\_squashing_scaler.py](gds/src/skrub/_squashing_scaler.py)                   | Scaler               | StatFrame/ML    |
| [gds/src/skrub/\_fast_hash.py](gds/src/skrub/_fast_hash.py)                                 | Hash helper          | StatFrame/ML    |
| [gds/src/skrub/\_string_distances.py](gds/src/skrub/_string_distances.py)                   | String distances     | StatFrame/ML    |
| [gds/src/skrub/\_sklearn_compat.py](gds/src/skrub/_sklearn_compat.py)                       | SKLearn compat       | StatFrame/ML    |
| [gds/src/skrub/\_fuzzy_join.py](gds/src/skrub/_fuzzy_join.py)                               | Fuzzy join           | Needs review    |
| [gds/src/skrub/\_joiner.py](gds/src/skrub/_joiner.py)                                       | Fuzzy joiner         | Needs review    |
| [gds/src/skrub/\_matching.py](gds/src/skrub/_matching.py)                                   | Matchers             | Needs review    |
| [gds/src/skrub/\_utils.py](gds/src/skrub/_utils.py)                                         | Utilities            | Needs review    |
| [gds/src/skrub/\_config.py](gds/src/skrub/_config.py)                                       | Config               | Needs review    |

## Immediate next tasks

1. Align DataFrame/LazyFrame with namespaces like Series/Expr have today.
2. Define explicit key/index metadata and join rules at the DataOps layer.
3. Add a minimal IO surface (download/unzip/read/scan/write) as DataOps utilities.
4. Write a short DSL spec listing DataExpr functions and namespaces.

## Dataset DSL checklist (seed pass)

This is the concrete, first-pass checklist of Dataset DSL functionality to cover next.

- Dataset registry: named datasets, local cache, version pins.
- Download/unzip: http(s) fetch, checksum, unpack (zip/tar), idempotent caching.
- Scan/read: csv/parquet/jsonl with schema hints; lazy vs eager read.
- Projection: text/corpus -> indexed DataFrame; metadata retained.
- Key/index metadata: declare keys and ordered indices for joins and slicing.
- Dataset splits: train/val/test/custom, stable split metadata.
- ETL helpers: rename, cast, drop, dedupe, null cleanup (deterministic only).
- Reporting hooks: row/column summaries, simple profiling, exportable reports.

## Dataset utilities outline (modules)

Proposed module clusters for the Dataset layer (thin, DataFrame-centric). These can live under `dataset/expressions` so they align with the existing namespace style:

- `dataset/expressions/registry`: dataset catalog, version pins, path resolution.
- `dataset/expressions/io`: download, unzip, cache, scan/read/write.
- `dataset/expressions/metadata`: schema, keys, indices, splits.
- `dataset/expressions/projection`: text/corpus -> DataFrame, graph -> DataFrame.
- `dataset/expressions/etl`: deterministic cleaning helpers.
- `dataset/expressions/reporting`: summaries and diagnostics.

Note: `download.rs` currently lives at the dataset root; consider moving or re-exporting it through `dataset/expressions/io` once the namespace boundary is decided.

## Minimal Dataset API surface (composable with DataFrame DSL)

- `Dataset::from_path(path)` and `Dataset::from_url(url, checksum)`.
- `Dataset::scan_*` and `Dataset::read_*` for common formats.
- `Dataset::with_key([...])`, `Dataset::with_index([...])`.
- `Dataset::split(split)` and `Dataset::splits()`.
- `Dataset::to_frame()` / `Dataset::to_lazy()`.
- `Dataset::profile()` / `Dataset::summary()` (lightweight reporting).
