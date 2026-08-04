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
    FeatureGrammar(GraphFeatureGrammarExpr),
    Model(GraphModelExpr),
    Plan(GraphPlanExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphFeatureGrammarExpr {
    grammar_name: String,
    grammar_version: Option<String>,
}

impl GraphFeatureGrammarExpr {
    pub fn new(grammar_name: impl Into<String>) -> Self {
        Self {
            grammar_name: grammar_name.into(),
            grammar_version: None,
        }
    }

    pub fn with_version(mut self, grammar_version: impl Into<String>) -> Self {
        self.grammar_version = Some(grammar_version.into());
        self
    }

    pub fn grammar_name(&self) -> &str {
        &self.grammar_name
    }

    pub fn grammar_version(&self) -> Option<&str> {
        self.grammar_version.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphModelExpr {
    model_id: String,
}

impl GraphModelExpr {
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
        }
    }

    pub fn model_id(&self) -> &str {
        &self.model_id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GraphPlanExpr {
    plan_id: String,
}

impl GraphPlanExpr {
    pub fn new(plan_id: impl Into<String>) -> Self {
        Self {
            plan_id: plan_id.into(),
        }
    }

    pub fn plan_id(&self) -> &str {
        &self.plan_id
    }
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

impl From<GraphFeatureGrammarExpr> for GraphFrameExpr {
    fn from(expr: GraphFeatureGrammarExpr) -> Self {
        Self::FeatureGrammar(expr)
    }
}

impl From<GraphModelExpr> for GraphFrameExpr {
    fn from(expr: GraphModelExpr) -> Self {
        Self::Model(expr)
    }
}

impl From<GraphPlanExpr> for GraphFrameExpr {
    fn from(expr: GraphPlanExpr) -> Self {
        Self::Plan(expr)
    }
}
