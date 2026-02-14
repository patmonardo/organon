use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::form::{
    ApplicationForm, FormShape, ProgramExecutionPlan, ProgramSpec, ProgramSpecError, Specification,
};

/// Concept layer: compile and contract a Program Form into canonical meaning.
#[derive(Debug, Default)]
pub struct FormEvaluator;

impl FormEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// Compile and validate a Form program into a deterministic kernel execution plan.
    pub fn evaluate(&self, request: FormEvalRequest) -> Result<FormEvalResult, FormEvalError> {
        self.evaluate_with_appearance(request, None)
    }

    pub(crate) fn evaluate_with_appearance(
        &self,
        request: FormEvalRequest,
        appearance: Option<String>,
    ) -> Result<FormEvalResult, FormEvalError> {
        let program = request.program;
        let plan = program
            .compile_execution_plan()
            .map_err(FormEvalError::Program)?;
        let given_forms = program
            .given_forms(appearance)
            .map_err(FormEvalError::Program)?;

        Ok(FormEvalResult {
            plan,
            contract: FormContract::from_program_spec(&program),
            pre_eval: FormPreEvalTrace::from_given_forms(given_forms),
        })
    }
}

/// Input request for Form evaluation.
#[derive(Debug, Clone)]
pub struct FormEvalRequest {
    pub program: ProgramSpec,
}

impl FormEvalRequest {
    pub fn new(program: ProgramSpec) -> Self {
        Self { program }
    }
}

/// Form evaluation output.
#[derive(Debug, Clone)]
pub struct FormEvalResult {
    pub plan: ProgramExecutionPlan,
    pub contract: FormContract,
    pub pre_eval: FormPreEvalTrace,
}

pub(crate) fn appearance_from_input(default_input: &serde_json::Value) -> Option<String> {
    default_input
        .get("appearance")
        .or_else(|| default_input.get("graphName"))
        .and_then(serde_json::Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MonadicEvaluationState {
    Pending,
    Succeeded,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonadicEvaluationSlot {
    pub state: MonadicEvaluationState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormPreEvalTrace {
    pub given_forms: Vec<GivenFormSevenFoldTrace>,
}

impl FormPreEvalTrace {
    pub(crate) fn from_given_forms(given_forms: Vec<crate::form::GivenFormEnvelope>) -> Self {
        Self {
            given_forms: given_forms
                .into_iter()
                .map(GivenFormSevenFoldTrace::from_given_form)
                .collect(),
        }
    }

    pub(crate) fn set_monadic_state(&mut self, state: MonadicEvaluationState) {
        for given_form in &mut self.given_forms {
            given_form.monadic_evaluation = state.clone();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GivenFormSevenFoldTrace {
    pub application_form: String,
    pub principle_shape: String,
    pub principle_context: String,
    pub principle_morph: String,
    pub effect_entity: String,
    pub effect_property: String,
    pub effect_aspect: String,
    pub monadic_evaluation: MonadicEvaluationState,
}

impl GivenFormSevenFoldTrace {
    fn from_given_form(given_form: crate::form::GivenFormEnvelope) -> Self {
        let perfected = given_form.into_perfected();
        Self {
            application_form: perfected.envelope.principled_effect.application_form.name,
            principle_shape: "principle.shape".to_string(),
            principle_context: "principle.context".to_string(),
            principle_morph: "principle.morph".to_string(),
            effect_entity: perfected.entity,
            effect_property: perfected.property,
            effect_aspect: perfected.aspect,
            monadic_evaluation: MonadicEvaluationState::Pending,
        }
    }
}

/// JSON-safe, schema-contract representation intended for TS/Zod parity.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormContract {
    pub form: FormShapeContract,
    pub gdsl: SpecificationContract,
    pub sdsl: Vec<SpecificationContract>,
    pub application_forms: Vec<ApplicationFormContract>,
    pub selected_forms: Vec<String>,
}

impl FormContract {
    pub fn from_program_spec(program: &ProgramSpec) -> Self {
        Self {
            form: FormShapeContract::from(&program.form),
            gdsl: SpecificationContract::from(&program.gdsl),
            sdsl: program
                .sdsl
                .iter()
                .map(SpecificationContract::from)
                .collect(),
            application_forms: program
                .application_forms
                .iter()
                .map(ApplicationFormContract::from)
                .collect(),
            selected_forms: program.selected_forms.clone(),
        }
    }

    pub fn into_program_spec(self) -> ProgramSpec {
        ProgramSpec {
            form: self.form.into(),
            gdsl: self.gdsl.into(),
            sdsl: self.sdsl.into_iter().map(Into::into).collect(),
            application_forms: self.application_forms.into_iter().map(Into::into).collect(),
            selected_forms: self.selected_forms,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormShapeContract {
    pub shape: ShapeContract,
    pub context: ContextContract,
    pub morph: MorphContract,
}

impl From<&FormShape> for FormShapeContract {
    fn from(value: &FormShape) -> Self {
        Self {
            shape: ShapeContract {
                required_fields: value.shape.required_fields.clone(),
                optional_fields: value.shape.optional_fields.clone(),
                type_constraints: value.shape.type_constraints.clone(),
                validation_rules: value.shape.validation_rules.clone(),
            },
            context: ContextContract {
                dependencies: value.context.dependencies.clone(),
                execution_order: value.context.execution_order.clone(),
                runtime_strategy: value.context.runtime_strategy.clone(),
                conditions: value.context.conditions.clone(),
            },
            morph: MorphContract {
                patterns: value.morph.patterns.clone(),
            },
        }
    }
}

impl From<FormShapeContract> for FormShape {
    fn from(value: FormShapeContract) -> Self {
        FormShape {
            shape: crate::form::Shape {
                required_fields: value.shape.required_fields,
                optional_fields: value.shape.optional_fields,
                type_constraints: value.shape.type_constraints,
                validation_rules: value.shape.validation_rules,
            },
            context: crate::form::Context {
                dependencies: value.context.dependencies,
                execution_order: value.context.execution_order,
                runtime_strategy: value.context.runtime_strategy,
                conditions: value.context.conditions,
            },
            morph: crate::form::Morph {
                patterns: value.morph.patterns,
            },
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShapeContract {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub type_constraints: HashMap<String, String>,
    pub validation_rules: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContextContract {
    pub dependencies: Vec<String>,
    pub execution_order: Vec<String>,
    pub runtime_strategy: String,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MorphContract {
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecificationContract {
    pub name: String,
    pub version: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl From<&Specification> for SpecificationContract {
    fn from(value: &Specification) -> Self {
        Self {
            name: value.name.clone(),
            version: value.version.clone(),
            attributes: value.attributes.clone(),
        }
    }
}

impl From<SpecificationContract> for Specification {
    fn from(value: SpecificationContract) -> Self {
        Self {
            name: value.name,
            version: value.version,
            attributes: value.attributes,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplicationFormContract {
    pub name: String,
    pub domain: String,
    pub features: Vec<String>,
    pub patterns: Vec<String>,
    pub specifications: HashMap<String, String>,
}

impl From<&ApplicationForm> for ApplicationFormContract {
    fn from(value: &ApplicationForm) -> Self {
        Self {
            name: value.name.clone(),
            domain: value.domain.clone(),
            features: value.features.clone(),
            patterns: value.patterns.clone(),
            specifications: value.specifications.clone(),
        }
    }
}

impl From<ApplicationFormContract> for ApplicationForm {
    fn from(value: ApplicationFormContract) -> Self {
        Self {
            name: value.name,
            domain: value.domain,
            features: value.features,
            patterns: value.patterns,
            specifications: value.specifications,
        }
    }
}

#[derive(Debug)]
pub enum FormEvalError {
    Program(ProgramSpecError),
}

impl fmt::Display for FormEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Program(error) => write!(f, "form evaluation failed: {error}"),
        }
    }
}

impl Error for FormEvalError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Program(error) => Some(error),
        }
    }
}
