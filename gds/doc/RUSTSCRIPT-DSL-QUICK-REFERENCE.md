# RustScript DSL Quick Reference

This is a short index for the “Client DSL” module pattern used by Collections-first surfaces.

## The invariant (2×2 matrix)

`Expr : LazyFrame :: Series : DataFrame`

- **Expr/LazyFrame**: lazy world (build plans; no execution)
- **Series/DataFrame**: eager world (materialized; execution allowed)

## Prescribed files for a DSL module `X`

Required entrypoints:

- `X/expr.rs` — `Expr → Expr` namespaces + `ExprXExt`
- `X/lazy.rs` — `LazyFrame → LazyFrame` namespaces + `LazyFrameXExt`
- `X/series.rs` — `Series → Series` namespaces + `SeriesXExt`
- `X/frame.rs` — `DataFrame → DataFrame` namespaces + `DataFrameXExt`

Recommended support:

- `X/functions/` — pure constructors/builders
- `X/macro.rs` — RustScript syntax sugar (lowers into functions/entrypoints)
- `X/expressions/` — low-level `Expr` builders
- `X/namespaces/` — grouped namespaces by domain
- `X/prelude.rs` — stable user import surface
- `X/mod.rs` — curated exports

## Current implementations

- Dataset DSL:
  - `gds/src/collections/dataset/expr.rs`
  - `gds/src/collections/dataset/lazy.rs`
  - `gds/src/collections/dataset/series.rs`
  - `gds/src/collections/dataset/frame.rs`

- DataFrame DSL:
  - `gds/src/collections/dataframe/expr.rs`
  - `gds/src/collections/dataframe/lazy.rs`
  - `gds/src/collections/dataframe/series.rs`
  - `gds/src/collections/dataframe/frame.rs`

## ADR

- `gds/doc/ADR-2026-02-10-RUSTSCRIPT-DSL-MATRIX.md`
