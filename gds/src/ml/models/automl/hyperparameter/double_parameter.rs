use super::concrete_parameter::ConcreteParameter;
use super::concrete_parameter::ConcreteParameterValue;
use serde::Deserialize;
use serde::Serialize;

/// Double parameter value
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DoubleParameter(pub f64);

impl DoubleParameter {
    /// Create a new DoubleParameter
    ///
    pub fn of(value: f64) -> Self {
        Self(value)
    }

    /// Get the value
    ///
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl ConcreteParameter for DoubleParameter {
    fn value(&self) -> ConcreteParameterValue {
        ConcreteParameterValue::Double(self.0)
    }
}
