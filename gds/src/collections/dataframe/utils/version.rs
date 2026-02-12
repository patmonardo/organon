//! Surface Polars version info (mirrors the Python helper).

/// Return the version string of the linked Polars crate.
pub fn get_polars_version() -> &'static str {
    polars::VERSION
}
