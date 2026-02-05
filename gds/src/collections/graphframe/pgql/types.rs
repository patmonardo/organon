//! PGQL -> Polars/GDS datatype mapping helpers

use polars_core::datatypes::{DataType, TimeUnit};
use polars_error::{polars_bail, PolarsResult};

/// PGQL datatype wrapper
///
/// We return `PgqlDataType` to allow representing both standard Polars datatypes
/// and pseudo-PGQL graph types (e.g., `GraphFrame`, `Vertex`, `Edge`, `Path`).
#[derive(Debug, PartialEq, Clone)]
pub enum PgqlDataType {
    /// Standard Polars datatype
    Polars(DataType),
    /// GraphFrame / graph-typed value (placeholder - semantics handled at GraphFrame layer)
    GraphFrame,
}
use sqlparser::ast::{DataType as PGQLDataType, Ident, ObjectName, ObjectNamePart};

/// Map a PGQL (pseudo-PGQL) data type to either a Polars `DataType` or a
/// `PGQLDataType::GraphFrame` marker for graph-specific types.
/// This mirrors the behaviour and style of `polars-sql::types::map_sql_dtype_to_polars`.
pub fn map_pgql_dtype(dtype: &PGQLDataType) -> PolarsResult<PGQLDataType> {
    Ok(match dtype {
        PGQLDataType::Boolean | PGQLDataType::Bool => PGQLDataType::Polars(DataType::Boolean),
        PGQLDataType::Int(_) | PGQLDataType::Integer(_) => PGQLDataType::Polars(DataType::Int32),
        PGQLDataType::SmallInt(_) => PGQLDataType::Polars(DataType::Int16),
        PGQLDataType::BigInt(_) => PGQLDataType::Polars(DataType::Int64),
        PGQLDataType::Float4 | PGQLDataType::Real | PGQLDataType::Float(_) => {
            PGQLDataType::Polars(DataType::Float32)
        }
        PGQLDataType::Double(_) | PGQLDataType::Float8 => PGQLDataType::Polars(DataType::Float64),
        PGQLDataType::Char(_)
        | PGQLDataType::Varchar(_)
        | PGQLDataType::String(_)
        | PGQLDataType::Text => PGQLDataType::Polars(DataType::String),
        PGQLDataType::Date => PGQLDataType::Polars(DataType::Date),
        PGQLDataType::Time(_, _) => PGQLDataType::Polars(DataType::Time),
        PGQLDataType::Timestamp(_, _) => {
            PGQLDataType::Polars(DataType::Datetime(TimeUnit::Microseconds, None))
        }
        PGQLDataType::Array(inner) => {
            // Map array inner type recursively if possible
            match inner.as_ref() {
                sqlparser::ast::ArrayElemTypeDef::AngleBracket(inner_type)
                | sqlparser::ast::ArrayElemTypeDef::SquareBracket(inner_type, _) => {
                    let inner = map_pgql_dtype(inner_type)?;
                    match inner {
                        PGQLDataType::Polars(dt) => {
                            PGQLDataType::Polars(DataType::List(Box::new(dt)))
                        }
                        PGQLDataType::GraphFrame => {
                            // list of graph elements: treat as graphframe placeholder
                            PGQLDataType::GraphFrame
                        }
                    }
                }
            }
        }
        PGQLDataType::Custom(ObjectName(idents), _) => match idents.as_slice() {
            [ObjectNamePart::Identifier(Ident { value, .. })] => {
                match value.to_lowercase().as_str() {
                    // pseudo-pgql graph types
                    "graph" | "graphframe" | "vertex" | "edge" | "path" => PGQLDataType::GraphFrame,
                    // unsigned / nonstandard integer names
                    "int1" => PGQLDataType::Polars(DataType::Int8),
                    "uint1" | "utinyint" => PGQLDataType::Polars(DataType::UInt8),
                    "uint2" | "usmallint" => PGQLDataType::Polars(DataType::UInt16),
                    "uint4" | "uinteger" | "uint" => PGQLDataType::Polars(DataType::UInt32),
                    "uint8" | "ubigint" => PGQLDataType::Polars(DataType::UInt64),
                    _ => {
                        polars_bail!(SQLInterface: "datatype {:?} is not currently supported", value)
                    }
                }
            }
            _ => polars_bail!(SQLInterface: "datatype {:?} is not currently supported", idents),
        },
        _ => polars_bail!(SQLInterface: "datatype {:?} is not currently supported", dtype),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::ast::{DataType as PGQLDataType, Ident, ObjectName, ObjectNamePart};

    #[test]
    fn map_basic_types() {
        assert_eq!(
            map_pgql_dtype(&PGQLDataType::Int(None)).unwrap(),
            PGQLDataType::Polars(DataType::Int32)
        );
        assert_eq!(
            map_pgql_dtype(&PGQLDataType::Varchar(Some(255))).unwrap(),
            PGQLDataType::Polars(DataType::String)
        );
        assert_eq!(
            map_pgql_dtype(&PGQLDataType::Boolean).unwrap(),
            PGQLDataType::Polars(DataType::Boolean)
        );
    }

    #[test]
    fn graphframe_type() {
        let obj = ObjectName(vec![ObjectNamePart::Identifier(Ident {
            value: "GraphFrame".into(),
            quote_style: None,
        })]);
        let dt = PGQLDataType::Custom(obj, None);
        assert_eq!(map_pgql_dtype(&dt).unwrap(), PGQLDataType::GraphFrame);
    }
}
