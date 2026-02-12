//! Series builders and helpers (py-polars shaped).

use polars::prelude::{DataType, NamedFrom, Series};

use crate::collections::dataframe::namespaces::array::ArrayNameSpace;
use crate::collections::dataframe::namespaces::binary::BinaryNameSpace;
use crate::collections::dataframe::namespaces::categorical::CategoricalNameSpace;
use crate::collections::dataframe::namespaces::datetime::DateTimeNameSpace;
use crate::collections::dataframe::namespaces::ext::ExtNameSpace;
use crate::collections::dataframe::namespaces::list::ListNameSpace;
use crate::collections::dataframe::namespaces::string::StringNameSpace;
use crate::collections::dataframe::namespaces::structure::StructNameSpace;
use crate::collections::dataframe::utils::udfs::{
    suggest_from_function_name, MapTarget, UdfError, UdfRewriteSuggestion,
};

/// Python-shaped Series facade for the Collections SDK.
#[derive(Debug, Clone)]
pub struct GDSSeries {
    series: Series,
}

macro_rules! series_ns_getter {
    ($method:ident, $ns:ident) => {
        pub fn $method(&self) -> $ns {
            $ns::new(self.series.clone())
        }
    };
}

impl GDSSeries {
    pub fn new(series: Series) -> Self {
        Self { series }
    }

    pub fn from<T, Phantom>(name: &str, values: T) -> Self
    where
        Series: NamedFrom<T, Phantom>,
        Phantom: ?Sized,
    {
        Self {
            series: Series::new(name.into(), values),
        }
    }

    pub fn from_list_i64(name: &str, values: &[Vec<i64>]) -> Self {
        Self {
            series: series_list_i64(name, values),
        }
    }

    pub fn series(&self) -> &Series {
        &self.series
    }

    pub fn into_series(self) -> Series {
        self.series
    }

    pub fn name(&self) -> &str {
        self.series.name()
    }

    pub fn dtype(&self) -> &DataType {
        self.series.dtype()
    }

    series_ns_getter!(bin, BinaryNameSpace);

    series_ns_getter!(cat, CategoricalNameSpace);

    series_ns_getter!(str, StringNameSpace);

    series_ns_getter!(list, ListNameSpace);

    series_ns_getter!(dt, DateTimeNameSpace);

    series_ns_getter!(structure, StructNameSpace);

    pub fn record(&self) -> StructNameSpace {
        self.structure()
    }

    series_ns_getter!(arr, ArrayNameSpace);

    series_ns_getter!(ext, ExtNameSpace);

    /// Suggest a native expression rewrite for a Series-level UDF name.
    ///
    /// This is a Rust-side analogue of py-polars inefficient-map guidance.
    pub fn suggest_udf_rewrite(
        &self,
        function_name: &str,
    ) -> Result<Option<UdfRewriteSuggestion>, UdfError> {
        suggest_from_function_name(function_name, self.name(), MapTarget::Series)
    }
}

pub fn series_list_i64(name: &str, values: &[Vec<i64>]) -> Series {
    let inner: Vec<Series> = values
        .iter()
        .map(|value| {
            if value.is_empty() {
                Series::new_empty("".into(), &DataType::Int64)
            } else {
                Series::new("".into(), value.as_slice())
            }
        })
        .collect();
    Series::new(name.into(), inner.as_slice())
}

pub fn series<T, Phantom>(name: &str, values: T) -> Series
where
    Series: NamedFrom<T, Phantom>,
    Phantom: ?Sized,
{
    Series::new(name.into(), values)
}
