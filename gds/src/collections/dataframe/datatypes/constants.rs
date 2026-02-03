//! Constants mirroring polars.datatypes.constants.

use polars::prelude::TimeUnit;

/// Number of rows to scan by default when inferring datatypes.
pub const N_INFER_DEFAULT: usize = 100;

/// Temporal units supported for datetime/timedelta inference.
pub const DTYPE_TEMPORAL_UNITS: [TimeUnit; 3] = [
    TimeUnit::Nanoseconds,
    TimeUnit::Microseconds,
    TimeUnit::Milliseconds,
];
