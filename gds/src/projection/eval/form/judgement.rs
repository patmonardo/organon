use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::form::ProgramSpec;
use crate::projection::eval::algorithm::ExecutionMode;
use crate::types::catalog::GraphCatalog;

use super::concept::{
    appearance_from_input, FormContract, FormEvalError, FormEvalRequest, FormEvalResult,
    FormEvaluator, FormPreEvalTrace, MonadicEvaluationSlot, MonadicEvaluationState,
};
use super::mediation::{
    apply_execution_plan, ProgramFormApplyBackend, ProgramFormApplyPrint, ProgramFormFailure,
};

/// Judgement layer: orchestrates Eval(Form) -> Apply(Form) -> Print.
#[derive(Debug, Default)]
pub struct ProgramFormApi {
    evaluator: FormEvaluator,
}

impl ProgramFormApi {
    pub fn new() -> Self {
        Self {
            evaluator: FormEvaluator::new(),
        }
    }

    pub fn evaluate(&self, program: ProgramSpec) -> Result<FormEvalResult, FormProgramError> {
        self.evaluator
            .evaluate(FormEvalRequest::new(program))
            .map_err(FormProgramError::Evaluate)
    }

    pub fn evaluate_apply_print(
        &self,
        request: ProgramFormRequest,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<ProgramFormPrint, FormProgramError> {
        let ProgramFormRequest {
            program,
            default_input,
            op_inputs,
            username,
            execution_mode,
            apply_backend,
            fail_fast,
        } = request;

        let appearance = appearance_from_input(&default_input);
        let eval = self
            .evaluator
            .evaluate_with_appearance(FormEvalRequest::new(program), appearance)
            .map_err(FormProgramError::Evaluate)?;

        let apply = apply_execution_plan(
            &eval.plan.patterns,
            &default_input,
            &op_inputs,
            &username,
            execution_mode,
            apply_backend,
            fail_fast,
            catalog,
        )?;

        let monadic_state = if apply.failed.is_empty() {
            MonadicEvaluationState::Succeeded
        } else {
            MonadicEvaluationState::Failed
        };

        let mut pre_eval = eval.pre_eval;
        pre_eval.set_monadic_state(monadic_state.clone());

        let organic_unity = OrganicUnityCheck::evaluate(
            apply_backend,
            &ProgramFormEvalPrint {
                selected_forms: eval.plan.selected_forms.clone(),
                patterns: eval.plan.patterns.clone(),
            },
            &apply,
            &pre_eval,
            &MonadicEvaluationSlot {
                state: monadic_state.clone(),
            },
            apply.failed.is_empty(),
        );

        Ok(ProgramFormPrint {
            ok: apply.failed.is_empty(),
            backend: apply_backend,
            eval: ProgramFormEvalPrint {
                selected_forms: eval.plan.selected_forms,
                patterns: eval.plan.patterns,
            },
            apply,
            contract: eval.contract,
            pre_eval,
            evaluation_slot: MonadicEvaluationSlot {
                state: monadic_state,
            },
            organic_unity,
        })
    }

    /// RootAgent establishes a projection world; SingleAgents execute ProgramForms inside it.
    pub fn start_agent_session(
        &self,
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> ProgramFormSession {
        ProgramFormSession::for_agent_framework(projection, single_agent)
    }

    /// External/public callers can execute in the same world model through facade semantics.
    pub fn start_public_session(
        &self,
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> ProgramFormSession {
        ProgramFormSession::for_public_facade(projection, single_agent)
    }

    /// Execute a ProgramForm request within a pre-conceived projection world.
    pub fn execute_session(
        &self,
        session: &ProgramFormSession,
        program: ProgramSpec,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<ProgramFormPrint, FormProgramError> {
        let mut request = match session.apply_backend {
            ProgramFormApplyBackend::ExecuteSpec => {
                ProgramFormRequest::for_agent_framework(program, session.single_agent.clone())
            }
            ProgramFormApplyBackend::DirectCompute => {
                ProgramFormRequest::for_public_facade(program, session.single_agent.clone())
            }
        };

        request.execution_mode = session.execution_mode;
        request.fail_fast = session.fail_fast;
        request.default_input = session.default_input();

        self.evaluate_apply_print(request, catalog)
    }
}

/// Projection world established by a RootAgent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RootProjectionContext {
    pub projection_id: String,
    pub graph_name: String,
    pub root_agent: String,
    pub pipeline_path: Vec<String>,
}

impl RootProjectionContext {
    pub fn new(
        projection_id: impl Into<String>,
        graph_name: impl Into<String>,
        root_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection_id: projection_id.into(),
            graph_name: graph_name.into(),
            root_agent: root_agent.into(),
            pipeline_path: vec![
                "codegen".to_string(),
                "factory".to_string(),
                "eval".to_string(),
            ],
        }
    }

    pub fn with_pipeline_path(mut self, pipeline_path: Vec<String>) -> Self {
        self.pipeline_path = pipeline_path;
        self
    }
}

/// Agent session scoped to a single projection world.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramFormSession {
    pub projection: RootProjectionContext,
    pub single_agent: String,
    pub apply_backend: ProgramFormApplyBackend,
    pub execution_mode: ExecutionMode,
    pub fail_fast: bool,
}

impl ProgramFormSession {
    pub fn for_agent_framework(
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection,
            single_agent: single_agent.into(),
            apply_backend: ProgramFormApplyBackend::ExecuteSpec,
            execution_mode: ExecutionMode::Stream,
            fail_fast: true,
        }
    }

    pub fn for_public_facade(
        projection: RootProjectionContext,
        single_agent: impl Into<String>,
    ) -> Self {
        Self {
            projection,
            single_agent: single_agent.into(),
            apply_backend: ProgramFormApplyBackend::DirectCompute,
            execution_mode: ExecutionMode::Stream,
            fail_fast: true,
        }
    }

    fn default_input(&self) -> Value {
        json!({
            "graphName": self.projection.graph_name,
            "projectionId": self.projection.projection_id,
            "rootAgent": self.projection.root_agent,
            "pipelinePath": self.projection.pipeline_path,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProgramFormRequest {
    pub program: ProgramSpec,
    pub default_input: Value,
    pub op_inputs: HashMap<String, Value>,
    pub username: String,
    pub execution_mode: ExecutionMode,
    pub apply_backend: ProgramFormApplyBackend,
    pub fail_fast: bool,
}

impl ProgramFormRequest {
    pub fn new(program: ProgramSpec) -> Self {
        Self {
            program,
            default_input: json!({}),
            op_inputs: HashMap::new(),
            username: "form-api".to_string(),
            execution_mode: ExecutionMode::Stream,
            apply_backend: ProgramFormApplyBackend::ExecuteSpec,
            fail_fast: true,
        }
    }

    /// Constructor for the Agent framework / FormDB world.
    ///
    /// This always chooses specification-driven execution (`ExecuteSpec`).
    pub fn for_agent_framework(program: ProgramSpec, username: impl Into<String>) -> Self {
        let mut request = Self::new(program);
        request.username = username.into();
        request.apply_backend = ProgramFormApplyBackend::ExecuteSpec;
        request
    }

    /// Constructor for external/public facade usage.
    ///
    /// This uses direct application compute dispatch (`DirectCompute`).
    pub fn for_public_facade(program: ProgramSpec, username: impl Into<String>) -> Self {
        let mut request = Self::new(program);
        request.username = username.into();
        request.apply_backend = ProgramFormApplyBackend::DirectCompute;
        request
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormPrint {
    pub ok: bool,
    pub backend: ProgramFormApplyBackend,
    pub eval: ProgramFormEvalPrint,
    pub apply: ProgramFormApplyPrint,
    pub contract: FormContract,
    pub pre_eval: FormPreEvalTrace,
    pub evaluation_slot: MonadicEvaluationSlot,
    pub organic_unity: OrganicUnityReport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormEvalPrint {
    pub selected_forms: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrganicUnityStatus {
    Coherent,
    Illusory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrganicUnityReport {
    pub status: OrganicUnityStatus,
    pub reasons: Vec<String>,
}

impl OrganicUnityReport {
    fn coherent() -> Self {
        Self {
            status: OrganicUnityStatus::Coherent,
            reasons: Vec::new(),
        }
    }

    fn illusory(reasons: Vec<String>) -> Self {
        Self {
            status: OrganicUnityStatus::Illusory,
            reasons,
        }
    }
}

pub(crate) struct OrganicUnityCheck;

impl OrganicUnityCheck {
    pub(crate) fn evaluate(
        backend: ProgramFormApplyBackend,
        eval: &ProgramFormEvalPrint,
        apply: &ProgramFormApplyPrint,
        pre_eval: &FormPreEvalTrace,
        evaluation_slot: &MonadicEvaluationSlot,
        ok: bool,
    ) -> OrganicUnityReport {
        let mut reasons = Vec::new();

        if pre_eval.given_forms.is_empty() {
            reasons.push("No given-form decomposition was produced".to_string());
        }

        let selected: std::collections::HashSet<&str> =
            eval.selected_forms.iter().map(String::as_str).collect();

        for gf in &pre_eval.given_forms {
            if !selected.contains(gf.application_form.as_str()) {
                reasons.push(format!(
                    "Given-form '{}' is not present in selected forms",
                    gf.application_form
                ));
            }

            if gf.effect_entity.trim().is_empty()
                || gf.effect_property.trim().is_empty()
                || gf.effect_aspect.trim().is_empty()
            {
                reasons.push(format!(
                    "Given-form '{}' has an incomplete principled-effect decomposition",
                    gf.application_form
                ));
            }

            if gf.monadic_evaluation != evaluation_slot.state {
                reasons.push(format!(
                    "Given-form '{}' monadic state is out of sync with evaluation slot",
                    gf.application_form
                ));
            }
        }

        for executed in &apply.executed {
            if executed.spec_binding.trim().is_empty() {
                reasons.push(format!(
                    "Pattern '{}' has no spec binding provenance",
                    executed.pattern
                ));
            }
        }

        if backend.is_spec_driven()
            && apply
                .executed
                .iter()
                .any(|item| !item.spec_binding.starts_with("spec."))
        {
            reasons.push("Spec-driven execution includes non-spec provenance bindings".to_string());
        }

        if ok && !apply.failed.is_empty() {
            reasons.push("Print marked ok but failed executions are present".to_string());
        }

        match evaluation_slot.state {
            MonadicEvaluationState::Succeeded if !ok => {
                reasons.push("Monadic slot is Succeeded while print is not ok".to_string())
            }
            MonadicEvaluationState::Failed if ok => {
                reasons.push("Monadic slot is Failed while print is ok".to_string())
            }
            _ => {}
        }

        if reasons.is_empty() {
            OrganicUnityReport::coherent()
        } else {
            OrganicUnityReport::illusory(reasons)
        }
    }
}

#[derive(Debug)]
pub enum FormProgramError {
    Evaluate(FormEvalError),
    InvalidInputShape(String),
    Apply(ProgramFormFailure),
}

impl fmt::Display for FormProgramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evaluate(error) => write!(f, "program form evaluate stage failed: {error}"),
            Self::InvalidInputShape(field) => {
                write!(f, "program form request field must be JSON object: {field}")
            }
            Self::Apply(failure) => write!(
                f,
                "program form apply stage failed for pattern '{}' (op='{}')",
                failure.pattern, failure.op
            ),
        }
    }
}

impl Error for FormProgramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Evaluate(error) => Some(error),
            Self::InvalidInputShape(_) | Self::Apply(_) => None,
        }
    }
}
