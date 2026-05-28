# Frame Compatibility Matrix

Purpose: run Job One as a concrete, testable program.

Scope for Step One:

- `expr`
- `lazy`
- `frame`
- `series`
- `namespaces`
- `functions`

Principle:

- DataFrame is the UR DSL and remains isomorphic to Polars Python semantics.
- Dataset may extend semantics, but must label extensions explicitly.

## Status Legend

- `PARITY`: same semantic contract as DataFrame/Polars surface.
- `EXTENSION`: compatible base + explicit Dataset semantic layer.
- `SPECULATIVE`: unclear/unstable semantics relative to lazy/dataframe execution.

## Matrix (Initial Slice)

| Surface | Entry | DataFrame/Polars meaning | Dataset meaning today | Status | Notes |
|---|---|---|---|---|---|
| Expr | `ExprDatasetExt::ds` | Namespace adapter over `Expr` | Thin adapter to `DatasetExprNameSpace` | PARITY | No hidden execution.
| Expr | `DatasetExprNameSpace::text/token/parse/tag/stem` | Expression composition | Expression composition via `expressions::*` builders | PARITY | Good substrate for lazy plans.
| Expr | `DatasetExprNameSpace::bin` | Binary expression namespace (`contains`, `starts_with`, `ends_with`, `encode`, `decode`, `size`) | Thin binary facade delegating to Dataset `expressions::binary` helpers | PARITY | Added in current sprint as parity slice.
| Lazy | `LazyFrameDatasetExt::ds` (`GDSLazyFrame`, `polars::LazyFrame`) | Namespace adapter over lazy frame | Thin adapter to `DatasetLazyFrameNameSpace` | PARITY | Entry semantics are clean.
| Lazy | `FeatureLazyFrameNameSpace::apply` | Apply lazy transform | Applies `Feature` plan to lazy frame | EXTENSION | Explicit semantic extension.
| Lazy | `DatasetLazyFrameNameSpace::apply_plan` / `with_plan` | Apply deferred logical plan to lazy query graph | Direct Plan-to-LazyFrame bridge via tabular steps | PARITY | Centers Plan as lazy contract surface.
| Frame | `DataFrameDatasetExt::ds` | Namespace adapter over eager frame | Adapter + optional metadata/profile/source context | EXTENSION | Keep extension explicit.
| Frame | `DatasetDataFrameNameSpace::lazy` | DataFrame -> LazyFrame bridge | Converts into lazy namespace | PARITY | Core bridge for execution parity.
| Series | `SeriesDatasetExt::ds` | Namespace adapter over `Series` | Thin adapter to `DatasetSeriesNameSpace` | PARITY | Good for eager one-off eval.
| Series | `DatasetSeriesNameSpace::eval_expr` | Evaluate expr on series | Uses DataFrame helper `series_expr(...).eval_expr` | PARITY | Confirm with parity tests.
| Namespaces | `DatasetNs::{io,metadata,project,report,registry}` | Builder surface | Orchestration builders over expression structs | PARITY | Avoid hidden evaluation.
| Functions | `pipeline`, `dataop_*`, `text_*`, `io_*`, `registry*`, `metadata`, `project_*`, `report_*` | Factory helpers | Thin factories over typed expr/pipeline objects | PARITY | Function/object parity currently good.
| Functions | `text_lifecycle` | N/A in pure Polars | Canonical semantic scaffold | EXTENSION | Accept as explicit Dataset extension.
| Functions | `scan_text_dir` | N/A in pure Polars | File-system corpus ingestion helper | EXTENSION | Keep out of core lazy semantics.

## Function Audit (Initial)

### Parity-safe now

- `pipeline`
- `dataop_input`, `dataop_encode`, `dataop_transform`, `dataop_decode`, `dataop_output`
- `text_input`, `text_encode`, `text_transform`, `text_decode`, `text_output`
- `io_path`, `io_url`
- `registry`, `registry_versioned`
- `metadata`
- `project_text`, `project_corpus`, `project_graph`
- `report_summary`, `report_profile`

### Explicit extension (keep)

- `text_lifecycle`
- `scan_text_dir`

### Speculative candidates (none confirmed yet)

- No immediate removals required from the current `functions` module.
- Next check should verify each helper is either pure factory or clearly extension-scoped.

## Immediate Test Backlog

1. `Expr` parity tests: Dataset expr builders vs direct expression builders produce same result columns.
2. `Lazy` parity tests: `.ds()` path and non-`.ds()` path produce same collected frame for equivalent transformations.
3. `Frame` bridge tests: `DatasetDataFrameNameSpace::lazy` round-trip preserves schema and row count.
4. `Series` eval parity tests: `DatasetSeriesNameSpace::eval_expr` equals direct `series_expr(...).eval_expr`.
5. `Functions` factory tests: every constructor is deterministic and side-effect free (except explicit IO extensions).

## Exit Criteria for Step One

- Matrix entries for core operations are all either `PARITY` or explicit `EXTENSION`.
- No unidentified speculative functions in core frame/lazy/expr/series/namespaces/functions paths.
- Compatibility tests exist for each matrix row family.

## Implemented Protocol Alignment (Current Sprint)

- Added Dataset Expr binary namespace parity slice:
	- `expressions/binary.rs` added as the protocol-level expression layer.
	- `expr/mod.rs` now exposes `.bin()` with binary expression methods.
	- Binary methods delegate through `expressions::binary`.
- Rewired Dataset `expr/mod.rs` to consume `expressions::*` directly for
	text/token/parse/tag/stem/bin methods.
- Kept `functions/expr.rs` as a downstream compatibility helper surface.
- Added fluent Plan builders (`from_var`, `from_dataset`, `filter`, `select`,
	`with_columns`, `split`, `batch`, `dataop`) and explicit lazy bridge methods
	(`DatasetLazyFrameNameSpace::apply_plan` / `with_plan`).
- Refactored namespace facades to delegate through Dataset functions:
	- `namespaces/dataset.rs` now routes builders via `functions::*`.
	- `namespaces/dataop.rs` now routes data-op constructors via `functions::*`.

Result: Dataset layering now mirrors the DataFrame protocol more closely:

- Expr + Expressions define the beginning protocol surface.
- Namespaces stay thin orchestration façades.
- Functions remain an end-stage compatibility/factory layer.
