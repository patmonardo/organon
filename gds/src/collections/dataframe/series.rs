//! Series builders and helpers (py-polars shaped).

use polars::prelude::{DataType, NamedFrom, Series};

use crate::collections::dataframe::namespaces::array::ArrayNameSpace;
use crate::collections::dataframe::namespaces::binary::BinaryNameSpace;
use crate::collections::dataframe::namespaces::categorical::CategoricalNameSpace;
use crate::collections::dataframe::namespaces::datetime::DateTimeNameSpace;
use crate::collections::dataframe::namespaces::ext::ExtNameSpace;
use crate::collections::dataframe::namespaces::list::ListNameSpace;
use crate::collections::dataframe::namespaces::string::StringNameSpace;
use crate::collections::dataframe::namespaces::struct_::StructNameSpace;

/// Python-shaped Series facade for the Collections SDK.
#[derive(Debug, Clone)]
pub struct SeriesModel {
    series: Series,
}

impl SeriesModel {
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

    pub fn bin(&self) -> BinaryNameSpace {
        BinaryNameSpace::new(self.series.clone())
    }

    pub fn cat(&self) -> CategoricalNameSpace {
        CategoricalNameSpace::new(self.series.clone())
    }

    pub fn str_(&self) -> StringNameSpace {
        StringNameSpace::new(self.series.clone())
    }

    pub fn list(&self) -> ListNameSpace {
        ListNameSpace::new(self.series.clone())
    }

    pub fn dt(&self) -> DateTimeNameSpace {
        DateTimeNameSpace::new(self.series.clone())
    }

    pub fn struct_(&self) -> StructNameSpace {
        StructNameSpace::new(self.series.clone())
    }

    pub fn arr(&self) -> ArrayNameSpace {
        ArrayNameSpace::new(self.series.clone())
    }

    pub fn ext(&self) -> ExtNameSpace {
        ExtNameSpace::new(self.series.clone())
    }
}

pub fn series<T, Phantom>(name: &str, values: T) -> Series
where
    Series: NamedFrom<T, Phantom>,
    Phantom: ?Sized,
{
    Series::new(name.into(), values)
}
