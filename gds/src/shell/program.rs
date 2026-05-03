//! Shell-facing program declarations.
//!
//! `form::ProgramFeatures` remains the stable declaration payload. The shell
//! wraps it so mediated program intent can be aligned with the root schema
//! instead of pretending ProgramFeatures alone know the whole system.

use crate::form::ProgramFeatures;

use super::ShellSchema;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellProgram {
    features: ProgramFeatures,
    schema: Option<ShellSchema>,
}

impl ShellProgram {
    pub fn new(features: ProgramFeatures) -> Self {
        Self {
            features,
            schema: None,
        }
    }

    pub fn with_schema(mut self, schema: ShellSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn features(&self) -> &ProgramFeatures {
        &self.features
    }

    pub fn schema(&self) -> Option<&ShellSchema> {
        self.schema.as_ref()
    }

    pub fn program_name(&self) -> &str {
        &self.features.program_name
    }

    pub fn feature_count(&self) -> usize {
        self.features.features.len()
    }

    pub fn into_features(self) -> ProgramFeatures {
        self.features
    }
}

impl From<ProgramFeatures> for ShellProgram {
    fn from(features: ProgramFeatures) -> Self {
        Self::new(features)
    }
}
