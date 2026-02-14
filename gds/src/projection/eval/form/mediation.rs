use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::applications::services::applications_dispatch;
use crate::projection::eval::algorithm::{
    AlgorithmError, AlgorithmSpec, ComputationResult, ConfigError, ConsumerError, ExecutionContext,
    ExecutionMode, ProcedureExecutor,
};
use crate::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
use crate::types::prelude::GraphStore;

use super::judgement::FormProgramError;

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

pub(crate) fn apply_execution_plan(
    patterns: &[String],
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

    for pattern in patterns {
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

pub(crate) fn normalize_algorithm_op(pattern: &str) -> Option<String> {
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

pub(crate) fn resolve_spec_binding(pattern: &str, op: &str) -> String {
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
