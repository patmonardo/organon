//! Dataset ETL expressions.
//!
//! Declarative, deterministic ETL operations for dataset pipelines.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetEtlOp {
    Rename { from: String, to: String },
    Cast { column: String, dtype: String },
    Drop { columns: Vec<String> },
    Dedupe { columns: Option<Vec<String>> },
    NullCleanup { columns: Option<Vec<String>> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetEtlExpr {
    op: DatasetEtlOp,
}

impl DatasetEtlExpr {
    pub fn rename(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            op: DatasetEtlOp::Rename {
                from: from.into(),
                to: to.into(),
            },
        }
    }

    pub fn cast(column: impl Into<String>, dtype: impl Into<String>) -> Self {
        Self {
            op: DatasetEtlOp::Cast {
                column: column.into(),
                dtype: dtype.into(),
            },
        }
    }

    pub fn drop(columns: Vec<String>) -> Self {
        Self {
            op: DatasetEtlOp::Drop { columns },
        }
    }

    pub fn dedupe(columns: Option<Vec<String>>) -> Self {
        Self {
            op: DatasetEtlOp::Dedupe { columns },
        }
    }

    pub fn null_cleanup(columns: Option<Vec<String>>) -> Self {
        Self {
            op: DatasetEtlOp::NullCleanup { columns },
        }
    }

    pub fn op(&self) -> &DatasetEtlOp {
        &self.op
    }
}
