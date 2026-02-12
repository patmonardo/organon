//! Formatting helpers for dtype display (PyPolars parity).

use polars::prelude::{DataType, Field};

/// Convert a Polars dtype to a prefixed string representation.
pub fn dtype_to_init_repr(dtype: &DataType, prefix: &str) -> String {
    match dtype {
        DataType::List(inner) => dtype_to_init_repr_list(inner, prefix),
        DataType::Array(inner, width) => dtype_to_init_repr_array(inner, *width, prefix),
        DataType::Struct(fields) => dtype_to_init_repr_struct(fields, prefix),
        _ => format!("{prefix}{dtype:?}"),
    }
}

fn dtype_to_init_repr_list(inner: &DataType, prefix: &str) -> String {
    let inner_repr = if matches!(inner, DataType::Null) {
        String::new()
    } else {
        dtype_to_init_repr(inner, prefix)
    };
    format!("{prefix}List({inner_repr})")
}

fn dtype_to_init_repr_array(inner: &DataType, width: usize, prefix: &str) -> String {
    let inner_repr = if matches!(inner, DataType::Null) {
        String::new()
    } else {
        dtype_to_init_repr(inner, prefix)
    };
    format!("{prefix}Array({inner_repr}, shape={width})")
}

fn dtype_to_init_repr_struct(fields: &[Field], prefix: &str) -> String {
    let inner_list: Vec<String> = fields
        .iter()
        .map(|field| {
            let field_name = field.name();
            let field_dtype = dtype_to_init_repr(field.dtype(), prefix);
            format!("{field_name:?}: {field_dtype}")
        })
        .collect();
    let inner_repr = format!("{{{}}}", inner_list.join(", "));
    format!("{prefix}Struct({inner_repr})")
}
