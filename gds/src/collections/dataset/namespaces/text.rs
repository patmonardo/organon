//! Text namespace for dataset-level data-op builders.
//!
//! These helpers create dataset data-op expressions scoped to the text domain.

use crate::collections::dataset::expressions::dataop::DatasetDataOpExpr;

#[derive(Debug, Clone, Default)]
pub struct TextNs;

impl TextNs {
    pub fn input(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_input(name)
    }

    pub fn encode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_encode(name)
    }

    pub fn transform(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_transform(name)
    }

    pub fn decode(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_decode(name)
    }

    pub fn output(name: impl Into<String>) -> DatasetDataOpExpr {
        DatasetDataOpExpr::text_output(name)
    }
}
