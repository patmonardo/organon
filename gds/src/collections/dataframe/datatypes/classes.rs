//! Rust-native helpers inspired by polars.datatypes.classes.
//!
//! This seed pass exposes simple helpers around `polars::prelude::DataType`.

use polars::prelude::{
    polars_bail, CategoricalMapping, CategoricalPhysical, Categories as PolarsCategories, DataType,
    Field as PolarsField, FrozenCategories, PlSmallStr, PolarsResult,
};
use std::string::String as StdString;
use std::sync::Arc;

pub use polars::prelude::DataType::{
    Array, Binary, BinaryOffset, Boolean, Date, Datetime, Decimal, Duration, Float32, Float64,
    Int128, Int16, Int32, Int64, Int8, List, Null, Object, String, Time, UInt128, UInt16, UInt32,
    UInt64, UInt8, Unknown,
};

pub use polars::prelude::DataType::{Categorical as CategoricalDType, Enum as EnumDType};

#[allow(non_upper_case_globals)]
pub const Float16: DataType = DataType::Float32;

#[allow(non_upper_case_globals)]
pub const Utf8: DataType = DataType::String;

/// Mirror of PyPolars Field (lightweight Rust helper).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    name: StdString,
    dtype: GDSDataType,
}

impl Field {
    pub fn new(name: impl Into<StdString>, dtype: GDSDataType) -> Self {
        Self {
            name: name.into(),
            dtype,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dtype(&self) -> &GDSDataType {
        &self.dtype
    }

    pub fn to_polars(&self) -> PolarsField {
        PolarsField::new(PlSmallStr::from(self.name.clone()), self.dtype.clone())
    }
}

impl From<Field> for PolarsField {
    fn from(field: Field) -> Self {
        PolarsField::new(PlSmallStr::from(field.name), field.dtype)
    }
}

/// Mirror of PyPolars Struct (lightweight Rust helper).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    fields: Vec<Field>,
}

impl Struct {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn to_dtype(&self) -> DataType {
        let fields = self.fields.iter().cloned().map(PolarsField::from).collect();
        DataType::Struct(fields)
    }
}

/// Mirror of PyPolars Categories (helper).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Categories {
    name: Option<StdString>,
    namespace: StdString,
    physical: DataType,
}

impl Categories {
    pub fn new(
        name: Option<impl Into<StdString>>,
        namespace: impl Into<StdString>,
        physical: DataType,
    ) -> Self {
        Self {
            name: name.map(|value| value.into()),
            namespace: namespace.into(),
            physical,
        }
    }

    pub fn global() -> Self {
        Self {
            name: None,
            namespace: StdString::new(),
            physical: DataType::UInt32,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn physical(&self) -> &DataType {
        &self.physical
    }

    pub fn is_global(&self) -> bool {
        self.name.is_none()
    }

    pub fn to_polars(&self) -> PolarsResult<Arc<PolarsCategories>> {
        if self.name.is_none() {
            if !self.namespace.is_empty() {
                polars_bail!(ComputeError: "categories namespace requires a name")
            }
            return Ok(PolarsCategories::global());
        }

        let physical = match self.physical {
            DataType::UInt8 => CategoricalPhysical::U8,
            DataType::UInt16 => CategoricalPhysical::U16,
            DataType::UInt32 => CategoricalPhysical::U32,
            _ => {
                polars_bail!(
                    ComputeError: "categorical physical dtype must be UInt8, UInt16, or UInt32"
                )
            }
        };

        Ok(PolarsCategories::new(
            PlSmallStr::from(self.name.as_ref().unwrap().clone()),
            PlSmallStr::from(self.namespace.clone()),
            physical,
        ))
    }
}

/// Mirror of PyPolars Categorical (helper).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Categorical {
    categories: Option<Categories>,
}

impl Categorical {
    pub fn new(categories: Option<Categories>) -> Self {
        Self { categories }
    }

    pub fn categories(&self) -> Option<&Categories> {
        self.categories.as_ref()
    }

    pub fn is_global(&self) -> bool {
        self.categories.as_ref().map_or(true, Categories::is_global)
    }

    pub fn to_dtype(&self) -> PolarsResult<DataType> {
        let categories = match self.categories.as_ref() {
            Some(categories) => categories.to_polars()?,
            None => PolarsCategories::global(),
        };
        let mapping: Arc<CategoricalMapping> = categories.mapping();
        Ok(DataType::Categorical(categories, mapping))
    }
}

/// Mirror of PyPolars Enum (helper).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumType {
    categories: Vec<StdString>,
}

impl EnumType {
    pub fn new<I, S>(categories: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<StdString>,
    {
        Self {
            categories: categories.into_iter().map(|value| value.into()).collect(),
        }
    }

    pub fn categories(&self) -> &[StdString] {
        &self.categories
    }

    pub fn to_dtype(&self) -> PolarsResult<DataType> {
        let frozen = FrozenCategories::new(self.categories.iter().map(|value| value.as_str()))?;
        Ok(DataType::Enum(frozen.clone(), frozen.mapping().clone()))
    }
}

/// Marker struct mirroring PyPolars DataTypeClass.
#[derive(Debug, Clone, Copy, Default)]
pub struct DataTypeClass;

impl DataTypeClass {
    pub fn base_type(dtype: &DataType) -> DataType {
        base_type(dtype)
    }

    pub fn is_numeric(dtype: &DataType) -> bool {
        is_numeric(dtype)
    }

    pub fn is_decimal(dtype: &DataType) -> bool {
        is_decimal(dtype)
    }

    pub fn is_integer(dtype: &DataType) -> bool {
        is_integer(dtype)
    }

    pub fn is_object(dtype: &DataType) -> bool {
        is_object(dtype)
    }

    pub fn is_signed_integer(dtype: &DataType) -> bool {
        is_signed_integer(dtype)
    }

    pub fn is_unsigned_integer(dtype: &DataType) -> bool {
        is_unsigned_integer(dtype)
    }

    pub fn is_float(dtype: &DataType) -> bool {
        is_float(dtype)
    }

    pub fn is_temporal(dtype: &DataType) -> bool {
        is_temporal(dtype)
    }

    pub fn is_nested(dtype: &DataType) -> bool {
        is_nested(dtype)
    }
}

/// Marker struct mirroring PyPolars NumericType.
#[derive(Debug, Clone, Copy, Default)]
pub struct NumericType;

impl NumericType {
    pub fn contains(dtype: &DataType) -> bool {
        is_numeric(dtype)
    }
}

/// Marker struct mirroring PyPolars IntegerType.
#[derive(Debug, Clone, Copy, Default)]
pub struct IntegerType;

impl IntegerType {
    pub fn contains(dtype: &DataType) -> bool {
        is_integer(dtype)
    }
}

/// Marker struct mirroring PyPolars FloatType.
#[derive(Debug, Clone, Copy, Default)]
pub struct FloatType;

impl FloatType {
    pub fn contains(dtype: &DataType) -> bool {
        is_float(dtype)
    }
}

/// Marker struct mirroring PyPolars TemporalType.
#[derive(Debug, Clone, Copy, Default)]
pub struct TemporalType;

impl TemporalType {
    pub fn contains(dtype: &DataType) -> bool {
        is_temporal(dtype)
    }
}

/// Marker struct mirroring PyPolars NestedType.
#[derive(Debug, Clone, Copy, Default)]
pub struct NestedType;

impl NestedType {
    pub fn contains(dtype: &DataType) -> bool {
        is_nested(dtype)
    }
}

/// Marker struct mirroring PyPolars ObjectType.
#[derive(Debug, Clone, Copy, Default)]
pub struct ObjectType;

impl ObjectType {
    pub fn contains(dtype: &DataType) -> bool {
        is_object(dtype)
    }
}

use super::GDSDataType;

/// Base extension dtype description (seed pass).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BaseExtension {
    name: StdString,
    storage: GDSDataType,
    metadata: Option<StdString>,
}

impl BaseExtension {
    pub fn new(
        name: impl Into<StdString>,
        storage: GDSDataType,
        metadata: Option<StdString>,
    ) -> Self {
        Self {
            name: name.into(),
            storage,
            metadata,
        }
    }

    pub fn ext_name(&self) -> &str {
        &self.name
    }

    pub fn ext_storage(&self) -> &GDSDataType {
        &self.storage
    }

    pub fn ext_metadata(&self) -> Option<&str> {
        self.metadata.as_deref()
    }
}

/// Generic extension dtype (seed pass).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Extension(BaseExtension);

impl Extension {
    pub fn new(
        name: impl Into<StdString>,
        storage: GDSDataType,
        metadata: Option<StdString>,
    ) -> Self {
        Self(BaseExtension::new(name, storage, metadata))
    }

    pub fn base(&self) -> &BaseExtension {
        &self.0
    }
}

/// Return the base dtype. For the seed pass, this returns the input dtype.
pub fn base_type(dtype: &DataType) -> DataType {
    dtype.clone()
}

/// Check whether the dtype is numeric.
pub fn is_numeric(dtype: &DataType) -> bool {
    dtype.is_numeric()
}

/// Check whether the dtype is decimal.
pub fn is_decimal(dtype: &DataType) -> bool {
    dtype.is_decimal()
}

/// Check whether the dtype is integer.
pub fn is_integer(dtype: &DataType) -> bool {
    dtype.is_integer()
}

/// Check whether the dtype is signed integer.
pub fn is_signed_integer(dtype: &DataType) -> bool {
    dtype.is_signed_integer()
}

/// Check whether the dtype is unsigned integer.
pub fn is_unsigned_integer(dtype: &DataType) -> bool {
    dtype.is_unsigned_integer()
}

/// Check whether the dtype is float.
pub fn is_float(dtype: &DataType) -> bool {
    dtype.is_float()
}

/// Check whether the dtype is temporal.
pub fn is_temporal(dtype: &DataType) -> bool {
    dtype.is_temporal()
}

/// Check whether the dtype is nested.
pub fn is_nested(dtype: &DataType) -> bool {
    dtype.is_nested()
}

/// Check whether the dtype is object.
pub fn is_object(dtype: &DataType) -> bool {
    dtype.is_object()
}
