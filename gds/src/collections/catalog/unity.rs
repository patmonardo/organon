//! Unity catalog-style metadata models (py-polars parity layer).

use crate::collections::catalog::schema::{CollectionsField, CollectionsSchema};
use crate::types::ValueType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogInfo {
    pub name: String,
    pub comment: Option<String>,
    pub properties: Vec<(String, String)>,
    pub options: Vec<(String, String)>,
    pub storage_location: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceInfo {
    pub name: String,
    pub comment: Option<String>,
    pub properties: Vec<(String, String)>,
    pub storage_location: Option<String>,
    pub created_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableInfo {
    pub name: String,
    pub comment: Option<String>,
    pub table_id: String,
    pub table_type: TableType,
    pub storage_location: Option<String>,
    pub data_source_format: Option<DataSourceFormat>,
    pub columns: Vec<ColumnInfo>,
    pub properties: Vec<(String, String)>,
    pub created_at: Option<String>,
    pub created_by: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by: Option<String>,
}

impl TableInfo {
    pub fn get_collections_schema(&self) -> Option<CollectionsSchema> {
        if self.columns.is_empty() {
            return None;
        }
        let fields = self
            .columns
            .iter()
            .map(|col| col.to_collections_field())
            .collect();
        Some(CollectionsSchema { fields })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnInfo {
    pub name: String,
    pub type_name: String,
    pub type_text: String,
    pub type_json: String,
    pub position: Option<usize>,
    pub comment: Option<String>,
    pub partition_index: Option<usize>,
}

impl ColumnInfo {
    pub fn to_collections_field(&self) -> CollectionsField {
        let value_type = value_type_from_unity_type(&self.type_name, &self.type_text);
        CollectionsField {
            name: self.name.clone(),
            value_type,
            nullable: true,
            time_unit: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableType {
    Managed,
    External,
    View,
    MaterializedView,
    StreamingTable,
    ManagedShallowClone,
    Foreign,
    ExternalShallowClone,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSourceFormat {
    Delta,
    Csv,
    Json,
    Avro,
    Parquet,
    Orc,
    Text,
    UnityCatalog,
    DeltaSharing,
    DatabricksFormat,
    RedshiftFormat,
    SnowflakeFormat,
    SqldwFormat,
    SalesforceFormat,
    BigqueryFormat,
    NetsuiteFormat,
    WorkdayRaasFormat,
    HiveSerde,
    HiveCustom,
    VectorIndexFormat,
    Unknown,
}

fn value_type_from_unity_type(type_name: &str, type_text: &str) -> ValueType {
    let type_name = type_name.to_ascii_lowercase();
    let type_text = type_text.to_ascii_lowercase();
    match type_name.as_str() {
        "byte" | "int8" => ValueType::Byte,
        "short" | "int16" => ValueType::Short,
        "int" | "integer" | "int32" => ValueType::Int,
        "long" | "int64" | "bigint" => ValueType::Long,
        "float" | "float32" => ValueType::Float,
        "double" | "float64" => ValueType::Double,
        "boolean" | "bool" => ValueType::Boolean,
        "string" | "varchar" | "char" => ValueType::String,
        "date" => ValueType::Date,
        "timestamp" | "datetime" => ValueType::DateTime,
        "binary" | "bytes" => ValueType::ByteArray,
        "decimal" => ValueType::Decimal,
        _ => {
            if type_text.contains("array") || type_text.contains("list") {
                ValueType::UntypedArray
            } else {
                ValueType::Unknown
            }
        }
    }
}
