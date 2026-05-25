//! Java: `ActivationFunction`.

use crate::ml::core::variable::VariableRef;

pub trait ActivationFunction: Send + Sync {
    fn apply(&self, input: VariableRef) -> VariableRef;
}
