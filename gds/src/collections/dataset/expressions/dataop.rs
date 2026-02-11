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
}
