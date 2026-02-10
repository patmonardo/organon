# ADR: RustScript DSL Matrix (Frame/Lazy × Series/Expr)

- Date: 2026-02-10
- Status: Accepted
- Scope: `gds::collections::{dataframe,dataset}` and any future “Client DSL” modules

## Context

We are adopting a Polars-inspired “Client DSL” architecture (py-polars shaped), but implemented as Rust-first, workspace-native APIs.

The core difficulty we hit during initial implementation was a _confusion matrix_ between:

- eager vs lazy evaluation, and
- column vs table operations.

This confusion produces misplacements like “LazyFrame namespace glue living in series.rs” (conceptually wrong) and “Expr helper logic mixed with LazyFrame APIs” (hard to reason about).

We want a stable convention that:

- scales across modules (`dataframe`, `dataset`, and future DSL clients),
- makes code generation predictable,
- keeps call sites “RustScript”-like (Scheme/Pythonic), and
- minimizes direct Polars imports at call sites.

## Decision

Adopt the **Frame/Lazy × Series/Expr** 2×2 matrix as the invariant structure for all Client DSL modules.

### Matrix invariant

> `Expr` is to `LazyFrame` as `Series` is to `DataFrame`.

Interpretation:

- **Expr/LazyFrame**: _lazy world_ (plan building)
- **Series/DataFrame**: _eager world_ (materialized computation)

### Entrypoint files (required pattern)

Each Client DSL module MUST provide the following top-level entrypoints:

- `expr.rs`
  - Owns Expr namespaces and `Expr*Ext` traits (e.g. `ExprDatasetExt`).
  - Input/Output: `Expr → Expr`.
  - No execution.

- `lazy.rs`
  - Owns LazyFrame namespaces and `LazyFrame*Ext` traits (e.g. `LazyFrameDatasetExt`).
  - Input/Output: `LazyFrame → LazyFrame`.
  - No execution.

- `series.rs`
  - Owns Series namespaces and `Series*Ext` traits (e.g. `SeriesDatasetExt`).
  - Input/Output: `Series → Series`.
  - Execution allowed.
  - Bridges (optional): evaluate an `Expr` against a `Series` for tests/parity.

- `frame.rs`
  - Owns DataFrame namespaces and `DataFrame*Ext` traits (e.g. `DataFrameDatasetExt`).
  - Input/Output: `DataFrame → DataFrame`.
  - Execution allowed.
  - Bridges (optional): explicit eager→lazy conversion (e.g. `df.lazy().ds()` style).

### Supporting surfaces

- `functions/` (pure constructors and small helpers)
  - Functions should return core types (`Expr`, `Series`, `DataFrame`, `LazyFrame`) or their facades.
  - Keep them stable and composable.

- `macro.rs` (syntax / “RustScript” ergonomics)
  - Macros should lower into `functions/` or into the four entrypoints.
  - Macros must not introduce hidden execution; respect eager vs lazy worlds.

## Prescribed module template (for every Client DSL)

For any module `X` (e.g. `dataframe`, `dataset`, future `graphframe`, `statframe`), the following files form the **prescribed skeleton**.

### Required entrypoints (the 2×2 matrix)

- `X/expr.rs`
  - Defines: `XExprNameSpace` + `ExprXExt` (`fn x(self) -> XExprNameSpace`).
  - Returns: `Expr`.
  - Scope: column semantics as expressions.

- `X/lazy.rs`
  - Defines: `XLazyFrameNameSpace` + `LazyFrameXExt`.
  - Returns: `LazyFrame` (or the project wrapper, e.g. `GDSLazyFrame`).
  - Scope: plan-level orchestration and composition.

- `X/series.rs`
  - Defines: `XSeriesNameSpace` + `SeriesXExt`.
  - Returns: `Series` (or wrapper).
  - Scope: eager column operations.
  - Optional: small bridges to evaluate `Expr` on `Series` for tests/parity.

- `X/frame.rs`
  - Defines: `XDataFrameNameSpace` + `DataFrameXExt`.
  - Returns: `DataFrame` (or wrapper).
  - Scope: eager table operations.
  - Optional: explicit eager→lazy conversion entry (e.g. `df.lazy().x()`).

### Recommended supporting files

- `X/functions/`
  - Pure constructors/builders; stable, small, composable.
  - Rule: prefer `functions/` for leaf operations that are easy to test and reuse.

- `X/macro.rs`
  - RustScript-style syntax sugar.
  - Rule: macros lower into `functions/` and/or the entrypoints; do not hide execution.

- `X/namespaces/`
  - Sub-namespaces grouped by domain (e.g. `token`, `parse`, `text`).
  - Rule: expression namespaces belong logically under `expr.rs` (even if implemented in files under `namespaces/`).

- `X/expressions/`
  - Low-level `Expr` builders (useful for keeping `expr.rs` readable).
  - Rule: keep these “dumb”: `Expr in → Expr out`.

- `X/prelude.rs`
  - The intended user import surface.
  - Rule: re-export the extension traits and the minimal stable types.

- `X/mod.rs`
  - Curated public exports.
  - Rule: re-export the four entrypoints and extension traits explicitly.

### Naming conventions (to prevent the confusion matrix)

- If a thing wraps/extends `Expr`, it lives in `expr.rs` and is named `*Expr*NameSpace`.
- If a thing wraps/extends `LazyFrame`, it lives in `lazy.rs` and is named `*LazyFrame*NameSpace`.
- If a thing wraps/extends `Series`, it lives in `series.rs` and is named `*Series*NameSpace`.
- If a thing wraps/extends `DataFrame`, it lives in `frame.rs` and is named `*DataFrame*NameSpace`.

These are _intentionally redundant_ to make the quadrant obvious at the call site and in search results.

### Export policy

- `mod.rs` re-exports the curated public entrypoints (`expr`, `lazy`, `series`, `frame`) and extension traits.
- `prelude.rs` re-exports the stable surface intended for users.
- Downstream call sites should prefer `use gds::collections::<module>::prelude::*;`.

## Consequences

### Positive

- Removes ambiguity: every operation has a “home quadrant”.
- Makes codegen predictable: new namespaces go into `expr.rs` or `lazy.rs` depending on evaluation world.
- Makes DSL composition obvious:
  - dataset DSL compiles down to dataframe DSL, which compiles down to Polars.
- Enables “no Polars imports” at call sites by concentrating Polars types behind facades/preludes.

### Negative / Trade-offs

- Some duplication is intentional: small `frame.rs` and `series.rs` may exist even when thin.
- Bridges (e.g. evaluating `Expr` against a `Series`) add a little conceptual overhead, but are contained.

## Implementation notes (current state)

Dataset module currently follows this matrix:

- `gds/src/collections/dataset/expr.rs`: dataset-flavored Expr namespaces (token/parse/tag/stem/text).
- `gds/src/collections/dataset/lazy.rs`: dataset-flavored LazyFrame namespaces (`.ds().feature().apply(...)`).
- `gds/src/collections/dataset/series.rs`: eager bridge for evaluating dataset Expr pipelines against a `GDSSeries`.
- `gds/src/collections/dataset/frame.rs`: minimal eager DataFrame facade and explicit bridge into dataset lazy namespace.

The `dataframe` module is the base Client DSL; the `dataset` module is a higher-level Client DSL built on top of it.

## Guidance for adding new DSL features

1. If it returns `Expr`, it belongs in `expr.rs`.
2. If it returns `LazyFrame`, it belongs in `lazy.rs`.
3. If it executes on `Series`, it belongs in `series.rs`.
4. If it executes on `DataFrame`, it belongs in `frame.rs`.
5. If it’s a reusable constructor, it belongs in `functions/`.
6. If it’s syntax sugar, it belongs in `macro.rs` but should lower into (1)-(5).

## References

- Polars-first control direction: gds/doc/POLARS-MASTER-CONTROL-THREAD.md
- Quick reference index: gds/doc/RUSTSCRIPT-DSL-QUICK-REFERENCE.md
- Dataset module entrypoints:
  - gds/src/collections/dataset/expr.rs
  - gds/src/collections/dataset/lazy.rs
  - gds/src/collections/dataset/series.rs
  - gds/src/collections/dataset/frame.rs
- DataFrame module entrypoints:
  - gds/src/collections/dataframe/expr.rs
  - gds/src/collections/dataframe/lazy.rs
  - gds/src/collections/dataframe/series.rs
  - gds/src/collections/dataframe/frame.rs
