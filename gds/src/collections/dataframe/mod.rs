//! Polars DataFrame integration for Collections.

pub mod chunked;
pub mod collection;
pub mod column;
pub mod construction;
pub mod datatypes;
pub mod expr;
pub mod frame;
pub mod getitem;
pub mod interchange;
pub mod parse;
pub mod row;
pub mod selectors;
pub mod series;
pub mod slice;
pub mod streaming;
pub mod table;

pub use chunked::PolarsChunkedSeries;
pub use collection::{
    DataFrameCollection, PolarsColumn, PolarsDataFrameCollection, PolarsDataType, PolarsSeries,
};
pub use column::{
    column_binary, column_binary_opt, column_bool, column_bool_opt, column_bytes, column_bytes_opt,
    column_f32, column_f32_opt, column_f64, column_f64_opt, column_i128, column_i128_opt,
    column_i16, column_i16_opt, column_i32, column_i32_opt, column_i64, column_i64_opt, column_i8,
    column_i8_opt, column_str, column_str_opt, column_string, column_string_opt, column_u16,
    column_u16_opt, column_u32, column_u32_opt, column_u64, column_u64_opt, column_u8,
    column_u8_opt,
};
pub use construction::{
    dataframe_from_columns, dataframe_from_columns_vec, dataframe_from_records,
    dataframe_from_rows, dataframe_from_series, schema_from_pairs, ConstructionOptions,
    DataOrientation, SchemaDefinition,
};
#[cfg(feature = "dtype-array")]
pub use datatypes::array as dtype_array;
#[cfg(feature = "dtype-decimal")]
pub use datatypes::decimal as dtype_decimal;
#[cfg(feature = "object")]
pub use datatypes::object as dtype_object;
pub use datatypes::{
    binary as dtype_binary, binary_offset as dtype_binary_offset, boolean as dtype_boolean,
    date as dtype_date, datetime as dtype_datetime, duration as dtype_duration,
    field as dtype_field, float32 as dtype_float32, float64 as dtype_float64,
    int128 as dtype_int128, int16 as dtype_int16, int32 as dtype_int32, int64 as dtype_int64,
    int8 as dtype_int8, list as dtype_list, null as dtype_null, string as dtype_string,
    struct_ as dtype_struct, time as dtype_time, uint16 as dtype_uint16, uint32 as dtype_uint32,
    uint64 as dtype_uint64, uint8 as dtype_uint8, utf8 as dtype_utf8, PolarsDataType as DataType,
};
pub use expr::{
    all, all_horizontal, any, any_horizontal, arange, arg_where, coalesce, col, cols, concat_arr,
    concat_list, concat_str, corr, cov, cum_fold, cum_reduce, date, date_range, date_ranges,
    datetime, datetime_range, datetime_ranges, dtype_of, duration, expr_alias, expr_and, expr_cast,
    expr_col, expr_cols, expr_count, expr_eq, expr_fill_null, expr_gt, expr_gte, expr_is_not_null,
    expr_is_null, expr_lit_bool, expr_lit_f32, expr_lit_f64, expr_lit_i16, expr_lit_i32,
    expr_lit_i64, expr_lit_i8, expr_lit_str, expr_lit_u16, expr_lit_u32, expr_lit_u64, expr_lit_u8,
    expr_lt, expr_lte, expr_max, expr_mean, expr_min, expr_neq, expr_not, expr_or, expr_sum,
    expr_when, fold, format, int_range, int_ranges, len, linear_space, linear_spaces, lit, max,
    max_horizontal, mean, mean_horizontal, min, min_horizontal, reduce, repeat, self_dtype,
    struct_, struct_with_fields, sum, sum_horizontal, time, time_range, time_ranges, when,
    PolarsExpr,
};
pub use frame::PolarsDataFrame;
pub use getitem::{
    get_df_item_by_key, get_series_item_by_key, ColSelector, DataFrameGetItem, DataFrameKey,
    RowSelector, SeriesGetItem, SeriesKey,
};
pub use interchange::{
    Buffer as InterchangeBuffer, Column as InterchangeColumn, ColumnBuffers, ColumnNullType,
    CompatLevel as InterchangeCompatLevel, CopyNotAllowedError as InterchangeCopyNotAllowedError,
    DataFrame as InterchangeDataFrame, Dtype as InterchangeDtype, DtypeKind as InterchangeDtypeKind,
    Endianness as InterchangeEndianness, InterchangeError, PolarsInterchangeBuffer,
    PolarsInterchangeColumn, PolarsInterchangeDataFrame, SupportsInterchange,
};
pub use parse::{
    parse_into_expression, parse_into_list_of_expressions, parse_into_list_of_expressions_for_df,
    parse_predicates_constraints_into_expression, ExprInput, ParseExprOptions,
};
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
    series_binary, series_binary_opt, series_bool, series_bool_opt, series_bytes, series_bytes_opt,
    series_f32, series_f32_opt, series_f64, series_f64_opt, series_i128, series_i128_opt,
    series_i16, series_i16_opt, series_i32, series_i32_opt, series_i64, series_i64_opt, series_i8,
    series_i8_opt, series_str, series_str_opt, series_string, series_string_opt, series_u16,
    series_u16_opt, series_u32, series_u32_opt, series_u64, series_u64_opt, series_u8,
    series_u8_opt,
};
pub use slice::{slice_dataframe, slice_lazyframe, slice_series, SliceSpec};
pub use streaming::PolarsStreamingFrame;
pub use table::{
    scale_f64_column, TableBuilder,
};
