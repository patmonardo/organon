//! Form evaluation entrypoint.
//!
//! This module provides a kernel-level Form evaluator that treats `ProgramSpec`
//! as the canonical runtime projection, while also exposing a JSON-safe contract
//! shape intended to mirror the TS/Zod FormDB surface.

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::applications::services::applications_dispatch;
use crate::form::{
    ApplicationForm, FormShape, ProgramExecutionPlan, ProgramSpec, ProgramSpecError, Specification,
};
use crate::projection::eval::algorithm::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, ProcedureExecutor,
};
use crate::types::catalog::GraphCatalog;
use crate::types::catalog::InMemoryGraphCatalog;
use crate::types::prelude::GraphStore;

/// Top-level evaluator for Program + ApplicationForm execution planning.
#[derive(Debug, Default)]
pub struct FormEvaluator;

impl FormEvaluator {
    pub fn new() -> Self {
        Self
    }

    /// Compile and validate a Form program into a deterministic kernel execution plan.
    pub fn evaluate(&self, request: FormEvalRequest) -> Result<FormEvalResult, FormEvalError> {
        let program = request.program;
        let plan = program
            .compile_execution_plan()
            .map_err(FormEvalError::Program)?;

        Ok(FormEvalResult {
            plan,
            contract: FormContract::from_program_spec(&program),
        })
    }
}

/// Top-level Program Form API that follows Eval(Form) -> Apply(Form) -> Print.
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

        let eval = self.evaluate(program)?;
        let apply = self.apply(
            &eval.plan,
            &default_input,
            &op_inputs,
            &username,
            execution_mode,
            apply_backend,
            fail_fast,
            catalog,
        )?;

        Ok(ProgramFormPrint {
            ok: apply.failed.is_empty(),
            backend: apply_backend,
            eval: ProgramFormEvalPrint {
                selected_forms: eval.plan.selected_forms,
                patterns: eval.plan.patterns,
            },
            apply,
            contract: eval.contract,
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

    fn apply(
        &self,
        plan: &ProgramExecutionPlan,
        default_input: &Value,
        op_inputs: &HashMap<String, Value>,
        username: &str,
        execution_mode: ExecutionMode,
        apply_backend: ProgramFormApplyBackend,
        fail_fast: bool,
        catalog: Arc<dyn GraphCatalog>,
    ) -> Result<ProgramFormApplyPrint, FormProgramError> {
        let mut executed = Vec::new();
        let mut failed = Vec::new();
        let mut skipped = Vec::new();

        for pattern in &plan.patterns {
            let Some(op) = normalize_algorithm_op(pattern) else {
                skipped.push(pattern.clone());
                continue;
            };

            let payload = build_algorithm_request(&op, default_input, op_inputs.get(&op))?;

            let (response, spec_binding) = match apply_backend {
                ProgramFormApplyBackend::ExecuteSpec => {
                    let execution = execute_via_algorithm_spec(
                        pattern,
                        &op,
                        &payload,
                        username,
                        execution_mode,
                        catalog.clone(),
                    );
                    (execution.response, execution.spec_binding)
                }
                ProgramFormApplyBackend::DirectCompute => (
                    applications_dispatch::handle_algorithms(&payload, catalog.clone()),
                    "direct_compute".to_string(),
                ),
            };
            let ok = response.get("ok").and_then(Value::as_bool).unwrap_or(false);

            executed.push(ProgramFormExecution {
                pattern: pattern.clone(),
                op: op.clone(),
                spec_binding: spec_binding.clone(),
                response: response.clone(),
            });

            if !ok {
                let failure = ProgramFormFailure {
                    pattern: pattern.clone(),
                    op: op.clone(),
                    spec_binding,
                    response,
                };

                if fail_fast {
                    return Err(FormProgramError::Apply(failure));
                }

                failed.push(failure);
            }
        }

        Ok(ProgramFormApplyPrint {
            executed,
            failed,
            skipped,
        })
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgramFormApplyBackend {
    /// Agent/FormDB path: AlgorithmSpec `execute` runtime.
    ExecuteSpec,
    /// External/public facade path: direct application `compute` dispatcher.
    DirectCompute,
}

impl ProgramFormApplyBackend {
    pub fn is_spec_driven(self) -> bool {
        matches!(self, Self::ExecuteSpec)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormPrint {
    pub ok: bool,
    pub backend: ProgramFormApplyBackend,
    pub eval: ProgramFormEvalPrint,
    pub apply: ProgramFormApplyPrint,
    pub contract: FormContract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormEvalPrint {
    pub selected_forms: Vec<String>,
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormApplyPrint {
    pub executed: Vec<ProgramFormExecution>,
    pub failed: Vec<ProgramFormFailure>,
    pub skipped: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormExecution {
    pub pattern: String,
    pub op: String,
    pub spec_binding: String,
    pub response: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramFormFailure {
    pub pattern: String,
    pub op: String,
    pub spec_binding: String,
    pub response: Value,
}

fn normalize_algorithm_op(pattern: &str) -> Option<String> {
    let trimmed = pattern.trim();
    if trimmed.is_empty() {
        return None;
    }

    let prefixes = ["algo.", "algorithm.", "applications.algorithms."];
    for prefix in prefixes {
        if let Some(rest) = trimmed.strip_prefix(prefix) {
            return Some(rest.trim().to_string()).filter(|s| !s.is_empty());
        }
    }

    if trimmed.contains('.') {
        return None;
    }

    Some(trimmed.to_string())
}

fn build_algorithm_request(
    op: &str,
    default_input: &Value,
    op_input: Option<&Value>,
) -> Result<Value, FormProgramError> {
    let mut payload = value_as_object(default_input, "default_input")?;
    if let Some(input) = op_input {
        let op_obj = value_as_object(input, "op_inputs[op]")?;
        payload.extend(op_obj);
    }

    payload.insert("op".to_string(), Value::String(op.to_string()));
    Ok(Value::Object(payload))
}

fn value_as_object(value: &Value, field: &str) -> Result<Map<String, Value>, FormProgramError> {
    match value {
        Value::Object(map) => Ok(map.clone()),
        _ => Err(FormProgramError::InvalidInputShape(field.to_string())),
    }
}

struct SpecExecutionResult {
    spec_binding: String,
    response: Value,
}

fn execute_via_algorithm_spec(
    pattern: &str,
    op: &str,
    payload: &Value,
    username: &str,
    execution_mode: ExecutionMode,
    catalog: Arc<dyn GraphCatalog>,
) -> SpecExecutionResult {
    let spec_binding = resolve_spec_binding(pattern, op);

    let graph_name = match graph_name_from_payload(payload) {
        Ok(name) => name,
        Err(response) => {
            return SpecExecutionResult {
                spec_binding,
                response,
            };
        }
    };

    let resolved = resolve_algorithm_spec(pattern, op, graph_name);

    let context = ExecutionContext::new(username.to_string()).with_catalog(catalog);
    let mut executor = ProcedureExecutor::new(context, execution_mode);
    let mut algorithm = resolved.spec;

    let response = match executor.compute(&mut algorithm, payload) {
        Ok(output) => output,
        Err(error) => json!({
            "ok": false,
            "op": op,
            "error": {
                "code": "EXECUTE_ERROR",
                "message": error.to_string(),
            }
        }),
    };

    SpecExecutionResult {
        spec_binding: resolved.spec_binding,
        response,
    }
}

fn canonical_algorithm_pattern(pattern: &str, op: &str) -> String {
    let normalized_op = op.trim().to_ascii_lowercase();
    let trimmed_pattern = pattern.trim().to_ascii_lowercase();
    if trimmed_pattern.starts_with("algo.")
        || trimmed_pattern.starts_with("algorithm.")
        || trimmed_pattern.starts_with("applications.algorithms.")
    {
        format!("algo.{normalized_op}")
    } else {
        trimmed_pattern
    }
}

struct ResolvedAlgorithmSpec {
    spec_binding: String,
    spec: FormAlgorithmSpec,
}

fn resolve_algorithm_spec(pattern: &str, op: &str, graph_name: String) -> ResolvedAlgorithmSpec {
    let spec_binding = resolve_spec_binding(pattern, op);
    let key = canonical_algorithm_pattern(pattern, op);
    match key.as_str() {
        "algo.pagerank" => ResolvedAlgorithmSpec {
            spec_binding,
            spec: FormAlgorithmSpec::PageRank(PageRankAlgorithmSpec::new(graph_name)),
        },
        "algo.leiden" => ResolvedAlgorithmSpec {
            spec_binding,
            spec: FormAlgorithmSpec::Leiden(LeidenAlgorithmSpec::new(graph_name)),
        },
        "algo.knn" => ResolvedAlgorithmSpec {
            spec_binding,
            spec: FormAlgorithmSpec::Knn(KnnAlgorithmSpec::new(graph_name)),
        },
        _ => ResolvedAlgorithmSpec {
            spec_binding,
            spec: FormAlgorithmSpec::Generic(DispatchAlgorithmSpec::new(
                op.to_string(),
                graph_name,
            )),
        },
    }
}

fn resolve_spec_binding(pattern: &str, op: &str) -> String {
    match canonical_algorithm_pattern(pattern, op).as_str() {
        "algo.pagerank" => "spec.pagerank".to_string(),
        "algo.leiden" => "spec.leiden".to_string(),
        "algo.knn" => "spec.knn".to_string(),
        _ => format!("spec.generic.{op}"),
    }
}

fn graph_name_from_payload(payload: &Value) -> Result<String, Value> {
    payload
        .get("graphName")
        .or_else(|| payload.get("graph_name"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            json!({
                "ok": false,
                "op": payload.get("op").and_then(Value::as_str).unwrap_or(""),
                "error": {
                    "code": "INVALID_REQUEST",
                    "message": "graphName missing or empty for execute backend",
                }
            })
        })
}

struct DispatchAlgorithmSpec {
    op: String,
    graph_name: String,
}

impl DispatchAlgorithmSpec {
    fn new(op: String, graph_name: String) -> Self {
        Self { op, graph_name }
    }
}

enum FormAlgorithmSpec {
    PageRank(PageRankAlgorithmSpec),
    Leiden(LeidenAlgorithmSpec),
    Knn(KnnAlgorithmSpec),
    Generic(DispatchAlgorithmSpec),
}

impl AlgorithmSpec for FormAlgorithmSpec {
    type Output = Value;

    fn name(&self) -> &str {
        match self {
            Self::PageRank(spec) => spec.name(),
            Self::Leiden(spec) => spec.name(),
            Self::Knn(spec) => spec.name(),
            Self::Generic(spec) => spec.name(),
        }
    }

    fn graph_name(&self) -> &str {
        match self {
            Self::PageRank(spec) => spec.graph_name(),
            Self::Leiden(spec) => spec.graph_name(),
            Self::Knn(spec) => spec.graph_name(),
            Self::Generic(spec) => spec.graph_name(),
        }
    }

    fn parse_config(&self, input: &Value) -> Result<Value, ConfigError> {
        match self {
            Self::PageRank(spec) => spec.parse_config(input),
            Self::Leiden(spec) => spec.parse_config(input),
            Self::Knn(spec) => spec.parse_config(input),
            Self::Generic(spec) => spec.parse_config(input),
        }
    }

    fn execute<G: GraphStore>(
        &self,
        graph_store: &G,
        config: &Value,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        match self {
            Self::PageRank(spec) => spec.execute(graph_store, config, context),
            Self::Leiden(spec) => spec.execute(graph_store, config, context),
            Self::Knn(spec) => spec.execute(graph_store, config, context),
            Self::Generic(spec) => spec.execute(graph_store, config, context),
        }
    }

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        match self {
            Self::PageRank(spec) => spec.consume_result(result, mode),
            Self::Leiden(spec) => spec.consume_result(result, mode),
            Self::Knn(spec) => spec.consume_result(result, mode),
            Self::Generic(spec) => spec.consume_result(result, mode),
        }
    }
}

struct PageRankAlgorithmSpec {
    inner: DispatchAlgorithmSpec,
}

impl PageRankAlgorithmSpec {
    fn new(graph_name: String) -> Self {
        Self {
            inner: DispatchAlgorithmSpec::new("pagerank".to_string(), graph_name),
        }
    }
}

struct LeidenAlgorithmSpec {
    inner: DispatchAlgorithmSpec,
}

impl LeidenAlgorithmSpec {
    fn new(graph_name: String) -> Self {
        Self {
            inner: DispatchAlgorithmSpec::new("leiden".to_string(), graph_name),
        }
    }
}

struct KnnAlgorithmSpec {
    inner: DispatchAlgorithmSpec,
}

impl KnnAlgorithmSpec {
    fn new(graph_name: String) -> Self {
        Self {
            inner: DispatchAlgorithmSpec::new("knn".to_string(), graph_name),
        }
    }
}

macro_rules! impl_dedicated_algorithm_spec {
    ($t:ty) => {
        impl AlgorithmSpec for $t {
            type Output = Value;

            fn name(&self) -> &str {
                self.inner.name()
            }

            fn graph_name(&self) -> &str {
                self.inner.graph_name()
            }

            fn parse_config(&self, input: &Value) -> Result<Value, ConfigError> {
                self.inner.parse_config(input)
            }

            fn execute<G: GraphStore>(
                &self,
                graph_store: &G,
                config: &Value,
                context: &ExecutionContext,
            ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
                self.inner.execute(graph_store, config, context)
            }

            fn consume_result(
                &self,
                result: ComputationResult<Self::Output>,
                mode: &ExecutionMode,
            ) -> Result<Self::Output, ConsumerError> {
                self.inner.consume_result(result, mode)
            }
        }
    };
}

impl_dedicated_algorithm_spec!(PageRankAlgorithmSpec);
impl_dedicated_algorithm_spec!(LeidenAlgorithmSpec);
impl_dedicated_algorithm_spec!(KnnAlgorithmSpec);

impl AlgorithmSpec for DispatchAlgorithmSpec {
    type Output = Value;

    fn name(&self) -> &str {
        &self.op
    }

    fn graph_name(&self) -> &str {
        &self.graph_name
    }

    fn parse_config(&self, input: &Value) -> Result<Value, ConfigError> {
        match input {
            Value::Object(_) => Ok(input.clone()),
            _ => Err(ConfigError::TypeMismatch {
                param: "config".to_string(),
                expected: "object".to_string(),
                actual: input.to_string(),
            }),
        }
    }

    fn execute<G: GraphStore>(
        &self,
        _graph_store: &G,
        config: &Value,
        context: &ExecutionContext,
    ) -> Result<ComputationResult<Self::Output>, AlgorithmError> {
        let store = context
            .load_graph(&self.graph_name)
            .map_err(|error| AlgorithmError::Graph(error.to_string()))?;

        let ephemeral_catalog = Arc::new(InMemoryGraphCatalog::new());
        ephemeral_catalog.set(&self.graph_name, store);

        let response = applications_dispatch::handle_algorithms(config, ephemeral_catalog);
        Ok(ComputationResult::new(response, std::time::Duration::ZERO))
    }

    fn consume_result(
        &self,
        result: ComputationResult<Self::Output>,
        _mode: &ExecutionMode,
    ) -> Result<Self::Output, ConsumerError> {
        Ok(result.into_result())
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::form::{Context, Morph, Shape};
    use crate::types::catalog::InMemoryGraphCatalog;

    fn sample_program() -> ProgramSpec {
        ProgramSpec {
            form: FormShape::new(
                Shape::default(),
                Context::new(vec![], vec![], "kernel".to_string(), vec![]),
                Morph::new(vec!["base.normalize".to_string()]),
            ),
            gdsl: Specification {
                name: "gdsl.core".to_string(),
                version: Some("0.1.0".to_string()),
                attributes: HashMap::new(),
            },
            sdsl: vec![],
            application_forms: vec![ApplicationForm {
                name: "centrality".to_string(),
                domain: "graph-ml".to_string(),
                features: vec!["feature.centrality".to_string()],
                patterns: vec!["algo.pagerank".to_string()],
                specifications: HashMap::new(),
            }],
            selected_forms: vec!["centrality".to_string()],
        }
    }

    #[test]
    fn evaluates_program_into_plan_and_contract() {
        let evaluator = FormEvaluator::new();
        let result = evaluator
            .evaluate(FormEvalRequest::new(sample_program()))
            .expect("evaluation should succeed");

        assert_eq!(
            result.plan.patterns,
            vec!["base.normalize", "algo.pagerank"]
        );
        assert_eq!(result.contract.selected_forms, vec!["centrality"]);
    }

    #[test]
    fn round_trips_contract_to_program_spec() {
        let program = sample_program();
        let contract = FormContract::from_program_spec(&program);
        let rebuilt = contract.into_program_spec();

        assert_eq!(program, rebuilt);
    }

    #[test]
    fn program_form_api_runs_eval_apply_print() {
        let api = ProgramFormApi::new();
        let mut request = ProgramFormRequest::new(sample_program());
        request.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());

        let print = api
            .evaluate_apply_print(request, catalog)
            .expect("eval/apply/print should complete");

        assert_eq!(print.eval.patterns, vec!["base.normalize", "algo.pagerank"]);
        assert_eq!(print.backend, ProgramFormApplyBackend::ExecuteSpec);
        assert_eq!(print.apply.skipped, vec!["base.normalize"]);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].op, "pagerank");
        assert_eq!(print.apply.executed[0].spec_binding, "spec.pagerank");
        assert_eq!(print.apply.failed.len(), 1);
        assert_eq!(print.apply.failed[0].spec_binding, "spec.pagerank");
        assert!(!print.ok);
    }

    #[test]
    fn program_form_api_supports_direct_compute_backend() {
        let api = ProgramFormApi::new();
        let mut request = ProgramFormRequest::new(sample_program());
        request.fail_fast = false;
        request.apply_backend = ProgramFormApplyBackend::DirectCompute;

        let catalog = Arc::new(InMemoryGraphCatalog::new());

        let print = api
            .evaluate_apply_print(request, catalog)
            .expect("eval/apply/print should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::DirectCompute);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "direct_compute");
    }

    #[test]
    fn resolves_dedicated_pattern_bindings() {
        let pagerank = resolve_algorithm_spec("algorithm.pagerank", "pagerank", "g".to_string());
        assert_eq!(pagerank.spec_binding, "spec.pagerank");

        let leiden = resolve_algorithm_spec("algo.leiden", "leiden", "g".to_string());
        assert_eq!(leiden.spec_binding, "spec.leiden");

        let generic = resolve_algorithm_spec("algo.node2vec", "node2vec", "g".to_string());
        assert_eq!(generic.spec_binding, "spec.generic.node2vec");
    }

    #[test]
    fn normalize_algorithm_op_handles_prefixes() {
        assert_eq!(
            normalize_algorithm_op("algo.pagerank"),
            Some("pagerank".to_string())
        );
        assert_eq!(
            normalize_algorithm_op("algorithm.leiden"),
            Some("leiden".to_string())
        );
        assert_eq!(
            normalize_algorithm_op("applications.algorithms.knn"),
            Some("knn".to_string())
        );
        assert_eq!(normalize_algorithm_op("base.normalize"), None);
    }

    #[test]
    fn request_constructors_encode_world_boundary() {
        let agent = ProgramFormRequest::for_agent_framework(sample_program(), "agent-kernel");
        assert_eq!(agent.username, "agent-kernel");
        assert_eq!(agent.apply_backend, ProgramFormApplyBackend::ExecuteSpec);

        let public = ProgramFormRequest::for_public_facade(sample_program(), "public-api");
        assert_eq!(public.username, "public-api");
        assert_eq!(public.apply_backend, ProgramFormApplyBackend::DirectCompute);
    }

    #[test]
    fn backend_marks_spec_driven_world() {
        assert!(ProgramFormApplyBackend::ExecuteSpec.is_spec_driven());
        assert!(!ProgramFormApplyBackend::DirectCompute.is_spec_driven());
    }

    #[test]
    fn session_executes_program_in_projection_world() {
        let api = ProgramFormApi::new();
        let projection = RootProjectionContext::new("proj-1", "world_graph", "root-agent")
            .with_pipeline_path(vec![
                "codegen".to_string(),
                "factory".to_string(),
                "eval".to_string(),
            ]);

        let mut session = api.start_agent_session(projection, "single-agent-a");
        session.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());
        let print = api
            .execute_session(&session, sample_program(), catalog)
            .expect("session execution should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::ExecuteSpec);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "spec.pagerank");
    }

    #[test]
    fn public_session_uses_direct_compute_backend() {
        let api = ProgramFormApi::new();
        let projection = RootProjectionContext::new("proj-2", "world_graph", "root-agent");

        let mut session = api.start_public_session(projection, "external-agent");
        session.fail_fast = false;

        let catalog = Arc::new(InMemoryGraphCatalog::new());
        let print = api
            .execute_session(&session, sample_program(), catalog)
            .expect("public session execution should complete");

        assert_eq!(print.backend, ProgramFormApplyBackend::DirectCompute);
        assert_eq!(print.apply.executed.len(), 1);
        assert_eq!(print.apply.executed[0].spec_binding, "direct_compute");
    }
}
