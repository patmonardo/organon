//! Polars DataFrame integration for Collections.

pub mod chunked;
pub mod collection;
pub mod column;
pub mod expr;
pub mod frame;
pub mod row;
pub mod selectors;
pub mod series;
pub mod streaming;
pub mod table;

pub use chunked::PolarsChunkedSeries;
pub use collection::{
    DataFrameCollection, PolarsColumn, PolarsDataFrameCollection, PolarsDataType, PolarsSeries,
};
pub use column::{
    column_bool, column_f32, column_f64, column_i16, column_i32, column_i64, column_i8, column_str,
    column_string, column_u16, column_u32, column_u64, column_u8,
};
pub use expr::{
    expr_alias, expr_and, expr_cast, expr_col, expr_cols, expr_count, expr_eq, expr_fill_null,
    expr_gt, expr_gte, expr_is_not_null, expr_is_null, expr_lit_bool, expr_lit_f32, expr_lit_f64,
    expr_lit_i16, expr_lit_i32, expr_lit_i64, expr_lit_i8, expr_lit_str, expr_lit_u16,
    expr_lit_u32, expr_lit_u64, expr_lit_u8, expr_lt, expr_lte, expr_max, expr_mean, expr_min,
    expr_neq, expr_not, expr_or, expr_sum, expr_when, PolarsExpr,
};
pub use frame::PolarsDataFrame;
pub use polars::prelude::SortMultipleOptions as PolarsSortMultipleOptions;
pub use row::{row_to_owned, PolarsRow, RowValue};
pub use selectors::{
    all as selector_all, binary as selector_binary, boolean as selector_boolean,
    by_dtype as selector_by_dtype, by_index as selector_by_index, by_name as selector_by_name,
    categorical as selector_categorical, contains as selector_contains, date as selector_date,
    datetime as selector_datetime, duration as selector_duration, ends_with as selector_ends_with,
    expand_exprs as selector_expand_exprs, expand_selector as selector_expand,
    first as selector_first, float as selector_float, integer as selector_integer,
    last as selector_last, list as selector_list, matches as selector_matches,
    numeric as selector_numeric, signed_integer as selector_signed_integer,
    starts_with as selector_starts_with, string as selector_string, struct_ as selector_struct,
    temporal as selector_temporal, time as selector_time,
    unsigned_integer as selector_unsigned_integer, Selector,
};
pub use series::{
    series_bool, series_f32, series_f64, series_i16, series_i32, series_i64, series_i8, series_str,
    series_string, series_u16, series_u32, series_u64, series_u8,
};
pub use streaming::PolarsStreamingFrame;
pub use table::{
    read_table_csv, read_table_ipc, read_table_parquet, scale_f64_column, write_table_csv,
    write_table_ipc, write_table_parquet, TableBuilder,
};
