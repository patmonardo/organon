//! DataType grouping helpers (seed pass), inspired by polars.datatypes.group.

use std::sync::OnceLock;

use polars::prelude::{DataType, TimeUnit};

#[derive(Debug, Clone)]
pub struct DataTypeGroup {
    items: Vec<DataType>,
    match_base_type: bool,
}

impl DataTypeGroup {
    pub fn new(items: Vec<DataType>, match_base_type: bool) -> Self {
        Self {
            items,
            match_base_type,
        }
    }

    pub fn contains(&self, dtype: &DataType) -> bool {
        if self.match_base_type {
            let base = base_type(dtype);
            self.items.iter().any(|it| it == dtype || it == &base)
        } else {
            self.items.iter().any(|it| it == dtype)
        }
    }

    pub fn items(&self) -> &[DataType] {
        &self.items
    }
}

fn base_type(dtype: &DataType) -> DataType {
    match dtype {
        DataType::Datetime(_, _) => DataType::Datetime(TimeUnit::Microseconds, None),
        DataType::Duration(_) => DataType::Duration(TimeUnit::Microseconds),
        DataType::List(_) => DataType::List(Box::new(DataType::Null)),
        DataType::Array(_, _) => DataType::Array(Box::new(DataType::Null), 0),
        DataType::Struct(_) => DataType::Struct(Vec::new()),
        _ => dtype.clone(),
    }
}

pub fn signed_integer_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        DataTypeGroup::new(
            vec![
                DataType::Int8,
                DataType::Int16,
                DataType::Int32,
                DataType::Int64,
                DataType::Int128,
            ],
            true,
        )
    })
}

pub fn unsigned_integer_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        DataTypeGroup::new(
            vec![
                DataType::UInt8,
                DataType::UInt16,
                DataType::UInt32,
                DataType::UInt64,
                DataType::UInt128,
            ],
            true,
        )
    })
}

pub fn integer_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        let mut items = Vec::new();
        items.extend_from_slice(signed_integer_dtypes().items());
        items.extend_from_slice(unsigned_integer_dtypes().items());
        DataTypeGroup::new(items, true)
    })
}

pub fn float_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| DataTypeGroup::new(vec![DataType::Float32, DataType::Float64], true))
}

pub fn numeric_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        let mut items = Vec::new();
        items.extend_from_slice(float_dtypes().items());
        items.extend_from_slice(integer_dtypes().items());
        items.push(DataType::Decimal(38, 0));
        DataTypeGroup::new(items, true)
    })
}

pub fn datetime_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        DataTypeGroup::new(
            vec![
                DataType::Datetime(TimeUnit::Milliseconds, None),
                DataType::Datetime(TimeUnit::Microseconds, None),
                DataType::Datetime(TimeUnit::Nanoseconds, None),
            ],
            true,
        )
    })
}

pub fn duration_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        DataTypeGroup::new(
            vec![
                DataType::Duration(TimeUnit::Milliseconds),
                DataType::Duration(TimeUnit::Microseconds),
                DataType::Duration(TimeUnit::Nanoseconds),
            ],
            true,
        )
    })
}

pub fn temporal_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        let mut items = vec![DataType::Date, DataType::Time];
        items.extend_from_slice(datetime_dtypes().items());
        items.extend_from_slice(duration_dtypes().items());
        DataTypeGroup::new(items, true)
    })
}

pub fn nested_dtypes() -> &'static DataTypeGroup {
    static GROUP: OnceLock<DataTypeGroup> = OnceLock::new();
    GROUP.get_or_init(|| {
        let mut items = vec![DataType::List(Box::new(DataType::Null))];
        items.push(DataType::Struct(Vec::new()));
        items.push(DataType::Array(Box::new(DataType::Null), 0));
        DataTypeGroup::new(items, true)
    })
}
