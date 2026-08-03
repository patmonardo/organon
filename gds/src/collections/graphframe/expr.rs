//! Graph-native expressions for the GraphFrame internal DSL.

use std::collections::BTreeMap;
use std::collections::HashSet;

use serde_json::Value;

use crate::projection::Orientation;
use crate::projection::RelationshipType;
use crate::shell::ShellComponentMode;

#[derive(Debug, Clone, PartialEq)]
pub enum GraphFrameExpr {
    View(GraphViewExpr),
    Procedure(GraphProcedureExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum GraphViewExpr {
    RelationshipTypes(HashSet<RelationshipType>),
    RelationshipProperty {
        relationship_type: RelationshipType,
        property_key: String,
    },
    Orientation(Orientation),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphProcedureExpr {
    component: String,
    mode: ShellComponentMode,
    inputs: BTreeMap<String, Value>,
}

impl GraphProcedureExpr {
    pub fn new(component: impl Into<String>, mode: ShellComponentMode) -> Self {
        Self {
            component: component.into(),
            mode,
            inputs: BTreeMap::new(),
        }
    }

    pub fn with_input(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.inputs.insert(key.into(), value.into());
        self
    }

    pub fn component(&self) -> &str {
        &self.component
    }

    pub fn mode(&self) -> ShellComponentMode {
        self.mode
    }

    pub fn inputs(&self) -> &BTreeMap<String, Value> {
        &self.inputs
    }
}

impl From<GraphViewExpr> for GraphFrameExpr {
    fn from(expr: GraphViewExpr) -> Self {
        Self::View(expr)
    }
}

impl From<GraphProcedureExpr> for GraphFrameExpr {
    fn from(expr: GraphProcedureExpr) -> Self {
        Self::Procedure(expr)
    }
}
