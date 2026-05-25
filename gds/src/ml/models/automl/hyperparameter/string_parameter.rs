use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// String parameter value
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StringParameter(pub String);

impl StringParameter {
    /// Create a new StringParameter
    ///
    pub fn of(value: String) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ConcreteParameter for StringParameter {
    fn value(&self) -> ConcreteParameterValue {
        ConcreteParameterValue::String(self.0.clone())
    }
}
