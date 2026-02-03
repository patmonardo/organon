//! Shared construction utilities (seed pass).
//!
//! Mirrors py-polars: polars/_utils/construction/utils.py

/// Return the first non-None value from a slice of Options.
pub fn get_first_non_none<T: Copy>(values: &[Option<T>]) -> Option<T> {
    values.iter().copied().flatten().next()
}

/// Placeholder for nested detection.
#[allow(dead_code)]
pub fn contains_nested<T>(_value: &T) -> bool {
    todo!("seed pass: contains_nested")
}

/// Placeholder for namedtuple detection.
#[allow(dead_code)]
pub fn is_namedtuple<T>(_value: &T) -> bool {
    todo!("seed pass: is_namedtuple")
}

/// Placeholder for pydantic model detection.
#[allow(dead_code)]
pub fn is_pydantic_model<T>(_value: &T) -> bool {
    todo!("seed pass: is_pydantic_model")
}

/// Placeholder for sqlalchemy row detection.
#[allow(dead_code)]
pub fn is_sqlalchemy_row<T>(_value: &T) -> bool {
    todo!("seed pass: is_sqlalchemy_row")
}
