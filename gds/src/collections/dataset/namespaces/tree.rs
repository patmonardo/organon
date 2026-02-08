//! Tree namespace for dataset-level DSL.
//!
//! Builders here are thin constructors for TreeExpr nodes.

use crate::collections::dataset::expressions::tree::{
    TreeExpr, TreeLabel, TreeLeafExpr, TreeOp, TreePos, TreeSpan,
};

#[derive(Debug, Clone, Default)]
pub struct TreeNs;

impl TreeNs {
    pub fn node(label: impl Into<TreeLabel>, children: Vec<TreeExpr>) -> TreeExpr {
        TreeExpr::Node {
            label: label.into(),
            children,
        }
    }

    pub fn leaf(value: impl Into<TreeLeafExpr>) -> TreeExpr {
        TreeExpr::Leaf(value.into())
    }

    pub fn pos(path: impl Into<Vec<usize>>) -> TreeExpr {
        TreeExpr::Pos(TreePos::new(path))
    }

    pub fn span(start: usize, end: usize) -> TreeExpr {
        TreeExpr::Span(TreeSpan::new(start, end))
    }

    pub fn transform(input: TreeExpr, op: TreeOp) -> TreeExpr {
        TreeExpr::Transform {
            input: Box::new(input),
            op,
        }
    }
}
