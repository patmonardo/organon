//! Dataset data-ops expressions.
//!
//! These are lightweight dataset-facing ops that model the Input -> Encode ->
//! Transform -> Decode -> Output lifecycle. They are not a planner or compute
//! graph by themselves, but provide a stable client-side vocabulary that can be
//! interpreted by Plan or higher-level evaluators.
//!
//! UDT bridge note:
//! - Dataset data-ops are metadata about how data should be interpreted.
//! - Concrete typing/casting (e.g., to_str, to_float) lives in DataFrame UDT
//!   expressions and is invoked by higher layers when they lower into frames.
//! - Text is the canonical Dataset datatype; numeric and other UDTs remain in
//!   the DataFrame layer.

use serde_json::{json, Value as JsonValue};

use polars::prelude::{col, Expr};

use crate::collections::dataset::expressions::text::{
    lowercase_expr_from, token_count_expr_from, tokenize_expr_from,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataFrameLoweringArtifact {
    pub stage: String,
    pub op_name: String,
    pub domain: Option<String>,
    pub input_column: String,
    pub expr_debug: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatasetAspectArtifact {
    pub stage: String,
    pub op_name: String,
    pub detail: JsonValue,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetDataOp {
    Input {
        name: String,
        detail: Option<JsonValue>,
    },
    Encode {
        name: String,
        detail: Option<JsonValue>,
    },
    Transform {
        name: String,
        detail: Option<JsonValue>,
    },
    Decode {
        name: String,
        detail: Option<JsonValue>,
    },
    Output {
        name: String,
        detail: Option<JsonValue>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetDataOpExpr {
    op: DatasetDataOp,
}

impl DatasetDataOpExpr {
    pub fn input(name: impl Into<String>) -> Self {
        Self {
            op: DatasetDataOp::Input {
                name: name.into(),
                detail: None,
            },
        }
    }

    pub fn input_with(name: impl Into<String>, detail: JsonValue) -> Self {
        Self {
            op: DatasetDataOp::Input {
                name: name.into(),
                detail: Some(detail),
            },
        }
    }

    pub fn text_input(name: impl Into<String>) -> Self {
        Self::input_with(name, json!({"domain": "text"}))
    }

    pub fn encode(name: impl Into<String>) -> Self {
        Self {
            op: DatasetDataOp::Encode {
                name: name.into(),
                detail: None,
            },
        }
    }

    pub fn encode_with(name: impl Into<String>, detail: JsonValue) -> Self {
        Self {
            op: DatasetDataOp::Encode {
                name: name.into(),
                detail: Some(detail),
            },
        }
    }

    pub fn text_encode(name: impl Into<String>) -> Self {
        Self::encode_with(name, json!({"domain": "text"}))
    }

    pub fn transform(name: impl Into<String>) -> Self {
        Self {
            op: DatasetDataOp::Transform {
                name: name.into(),
                detail: None,
            },
        }
    }

    pub fn transform_with(name: impl Into<String>, detail: JsonValue) -> Self {
        Self {
            op: DatasetDataOp::Transform {
                name: name.into(),
                detail: Some(detail),
            },
        }
    }

    pub fn text_transform(name: impl Into<String>) -> Self {
        Self::transform_with(name, json!({"domain": "text"}))
    }

    pub fn decode(name: impl Into<String>) -> Self {
        Self {
            op: DatasetDataOp::Decode {
                name: name.into(),
                detail: None,
            },
        }
    }

    pub fn decode_with(name: impl Into<String>, detail: JsonValue) -> Self {
        Self {
            op: DatasetDataOp::Decode {
                name: name.into(),
                detail: Some(detail),
            },
        }
    }

    pub fn text_decode(name: impl Into<String>) -> Self {
        Self::decode_with(name, json!({"domain": "text"}))
    }

    pub fn output(name: impl Into<String>) -> Self {
        Self {
            op: DatasetDataOp::Output {
                name: name.into(),
                detail: None,
            },
        }
    }

    pub fn output_with(name: impl Into<String>, detail: JsonValue) -> Self {
        Self {
            op: DatasetDataOp::Output {
                name: name.into(),
                detail: Some(detail),
            },
        }
    }

    pub fn text_output(name: impl Into<String>) -> Self {
        Self::output_with(name, json!({"domain": "text"}))
    }

    pub fn op(&self) -> &DatasetDataOp {
        &self.op
    }

    pub fn name(&self) -> &str {
        match &self.op {
            DatasetDataOp::Input { name, .. }
            | DatasetDataOp::Encode { name, .. }
            | DatasetDataOp::Transform { name, .. }
            | DatasetDataOp::Decode { name, .. }
            | DatasetDataOp::Output { name, .. } => name,
        }
    }

    pub fn detail(&self) -> Option<&JsonValue> {
        match &self.op {
            DatasetDataOp::Input { detail, .. }
            | DatasetDataOp::Encode { detail, .. }
            | DatasetDataOp::Transform { detail, .. }
            | DatasetDataOp::Decode { detail, .. }
            | DatasetDataOp::Output { detail, .. } => detail.as_ref(),
        }
    }

    pub fn domain(&self) -> Option<&str> {
        self.detail()
            .and_then(|d| d.get("domain"))
            .and_then(|d| d.as_str())
    }

    pub fn stage(&self) -> &'static str {
        match self.op() {
            DatasetDataOp::Input { .. } => "input",
            DatasetDataOp::Encode { .. } => "encode",
            DatasetDataOp::Transform { .. } => "transform",
            DatasetDataOp::Decode { .. } => "decode",
            DatasetDataOp::Output { .. } => "output",
        }
    }

    /// Lower this dataset data-op into a DataFrame expression step.
    ///
    /// This provides compatibility with the DataFrame engine by making
    /// dataset-level operations executable as Polars `Expr` transforms.
    pub fn as_dataframe_expr(&self, expr: Expr) -> Expr {
        let is_text = self.domain() == Some("text");

        match self.op() {
            DatasetDataOp::Input { .. } => expr,
            DatasetDataOp::Encode { .. } if is_text => lowercase_expr_from(expr),
            DatasetDataOp::Transform { .. } if is_text => tokenize_expr_from(expr),
            DatasetDataOp::Decode { .. } if is_text => token_count_expr_from(expr),
            DatasetDataOp::Output { .. } => expr,
            _ => expr,
        }
    }

    pub fn as_dataframe_expr_for_column(&self, column: &str) -> Expr {
        self.as_dataframe_expr(col(column))
    }

    /// Capture this operation as a Dataset-level SDSL aspect artifact.
    pub fn as_dataset_aspect_artifact(&self) -> DatasetAspectArtifact {
        DatasetAspectArtifact {
            stage: self.stage().to_string(),
            op_name: self.name().to_string(),
            detail: self.detail().cloned().unwrap_or_else(|| json!({})),
        }
    }

    /// Lower this operation to a DataFrame expression and capture a
    /// DataFrame-lowering artifact describing the generated representation.
    pub fn lower_to_dataframe_artifact(&self, input_column: &str) -> DataFrameLoweringArtifact {
        let lowered = self.as_dataframe_expr_for_column(input_column);
        DataFrameLoweringArtifact {
            stage: self.stage().to_string(),
            op_name: self.name().to_string(),
            domain: self.domain().map(str::to_string),
            input_column: input_column.to_string(),
            expr_debug: format!("{lowered:?}"),
        }
    }
}
