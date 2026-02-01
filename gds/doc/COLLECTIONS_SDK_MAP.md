# Collections SDK (Python‑shaped)

This repo treats **Collections** as the SDK layer and a **py‑polars‑style facade** over Polars Rust.
The goal is a stable, minimal, Python‑shaped surface that maps cleanly into Polars crates.

## 1) Roles

- **Client**: application code imports Collections SDK only.
- **SDK**: thin adapters that translate into Polars types + functions.
- **Datasets layer**: the Collections SDK itself (dataset translations live here).

## 2) Facade Objects (Python‑shaped)

- `SeriesModel`
- `FrameModel`
- `LazyFrameModel`
- `Expr`
- `Selectors`

## 3) Namespaces (Python‑style)

- `series.*` → series construction + dtype helpers
- `expr.*` → expression constructors + namespaces (`str`, `dt`, `list`, `arr`, `struct`)
- `selectors.*` → column selection DSL
- `io.*` → parquet/csv/ipc/json + metadata
- `catalog.*` → disk/unity metadata + registry

## 4) Mapping Table (Python → SDK → Polars)

### `SeriesModel`

- Python: `pl.Series(...)`, `.cast()`, `.alias()`, `.name`, `.dtype`, `.is_null()`
- SDK: `SeriesModel::new`, `SeriesModel::cast`, `SeriesModel::alias`, `SeriesModel::name`, `SeriesModel::dtype`, `SeriesModel::is_null`
- Polars: `polars::prelude::Series`

### `Expr`

- Python: `pl.col`, `pl.lit`, `pl.when`, `pl.sum`, `pl.concat_list`, namespaces like `pl.col("x").str.*`
- SDK: `expr::col`, `expr::lit`, `expr::when`, `expr::sum`, `expr::concat_list`, `expr::str::*`
- Polars: `polars::lazy::dsl::*`, `Expr` namespaces

### `FrameModel` / `LazyFrameModel`

- Python: `pl.DataFrame`, `pl.LazyFrame`, `.select`, `.filter`, `.with_columns`, `.group_by`
- SDK: `FrameModel::select`, `FrameModel::filter`, `FrameModel::with_columns`, `FrameModel::group_by`
- Polars: `polars::prelude::DataFrame`, `polars::prelude::LazyFrame`

### `Selectors`

- Python: `cs.*` with set ops
- SDK: `selectors::*` DSL
- Polars: selector → `Vec<Expr>` (select/projection input)

### `IO`

- Python: `pl.read_parquet`, `scan_parquet`, `read_csv`, `read_ipc`, metadata
- SDK: `io::parquet::read_table`, `io::parquet::scan_table`, `io::parquet::read_metadata_*`
- Polars: `LazyFrame::scan_parquet`, `ParquetWriter`, `polars_parquet`

### `Catalog`

- Python: Unity + dataset catalog style
- SDK: `catalog::unity::*`, `catalog::disk::*`
- Polars: `polars_io::cloud` + unity helpers where available

## 5) Layering Rules

1. Client code imports **only** Collections SDK.
2. SDK surface calls Polars and stays thin.
3. Stable SDK surface, evolving internals.

## 6) Folder Layout (Python‑shaped)

```
collections/
  dataframe/
    series/
    expressions/
    selectors/
    frame.rs
    lazy_frame.rs
  io/
  catalog/
```

## 7) Near‑term Focus

- Finish `SeriesModel` surface parity with py‑polars high‑use methods.
- Align `Expr` namespaces to Python function names.
- Keep selectors as ergonomic DSL.
- Keep IO and catalog as stable SDK endpoints.
