//! Typed instruction language for the root FormVM.
//!
//! GDSL patterns remain authoring/compiler terms. The linker lowers them into
//! these operations so the runtime never has to infer architectural meaning
//! from an untyped string.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormVmOperationKind {
    EstablishPrinciple,
    ProjectApplication { application_form: String },
    DetermineGraph { binding: String },
    ConstituteTask { binding: String },
    InvokeOperator { service: String, operator: String },
    CollectEvidence { binding: String },
    ReturnForm { binding: String },
    DeferredCompatibility { pattern: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmOperation {
    pub sequence: u64,
    pub source_pattern: Option<String>,
    pub source_application: Option<String>,
    pub kind: FormVmOperationKind,
}

impl FormVmOperation {
    pub fn new(
        sequence: u64,
        source_pattern: Option<String>,
        source_application: Option<String>,
        kind: FormVmOperationKind,
    ) -> Self {
        Self {
            sequence,
            source_pattern,
            source_application,
            kind,
        }
    }
}
