//! Text-domain alias of [`super::dataop::DataOpNs`].
//!
//! `TextNs` is a thin convenience surface that mirrors the `text_*` family on
//! `DataOpNs` so toolchain authoring code can spell text-only stages without
//! the `text_` prefix on every call. It deliberately exposes no other
//! methods; for non-text stages, lowering helpers, or artifact accessors,
//! reach for [`super::dataop::DataOpNs`] directly.

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
