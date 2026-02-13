//! Column builders for DataFrame-backed tables.

use polars::prelude::Column;

use crate::collections::dataframe::series::series;

fn column_from_values<T, Phantom>(name: &str, values: T) -> Column
where
    polars::prelude::Series: polars::prelude::NamedFrom<T, Phantom>,
    Phantom: ?Sized,
{
    Column::from(series(name, values))
}

macro_rules! column_builder {
    ($( $fn_name:ident : $ty:ty ),+ $(,)?) => {
        $(
            pub fn $fn_name(name: &str, values: $ty) -> Column {
                column_from_values(name, values)
            }
        )+
    };
}

column_builder!(
    column_i64: &[i64],
    column_i64_opt: &[Option<i64>],
    column_i128: &[i128],
    column_i128_opt: &[Option<i128>],
    column_i32: &[i32],
    column_i32_opt: &[Option<i32>],
    column_i16: &[i16],
    column_i16_opt: &[Option<i16>],
    column_i8: &[i8],
    column_i8_opt: &[Option<i8>],
    column_u64: &[u64],
    column_u64_opt: &[Option<u64>],
    column_u32: &[u32],
    column_u32_opt: &[Option<u32>],
    column_u16: &[u16],
    column_u16_opt: &[Option<u16>],
    column_u8: &[u8],
    column_u8_opt: &[Option<u8>],
    column_f64: &[f64],
    column_f64_opt: &[Option<f64>],
    column_f32: &[f32],
    column_f32_opt: &[Option<f32>],
    column_bool: &[bool],
    column_bool_opt: &[Option<bool>],
    column_str: &[&str],
    column_str_opt: &[Option<&str>],
    column_string: &[String],
    column_string_opt: &[Option<String>],
    column_binary: &[&[u8]],
    column_binary_opt: &[Option<&[u8]>],
    column_bytes: &[Vec<u8>],
    column_bytes_opt: &[Option<Vec<u8>>],
);
