use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// Integer parameter value
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntegerParameter(pub i32);

impl IntegerParameter {
    /// Create a new IntegerParameter
    ///
    pub fn of(value: i32) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl ConcreteParameter for IntegerParameter {
    fn value(&self) -> ConcreteParameterValue {
        ConcreteParameterValue::Integer(self.0)
    }
}
