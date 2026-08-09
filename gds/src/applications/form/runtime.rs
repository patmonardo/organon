//! Empirical adapters for linked FormVM operations.
//!
//! Form core owns the instruction language. This module binds those instructions
//! to GraphFrame and TaskFrame because graph catalogs and runtime resources are
//! application/kernel concerns rather than PureForm state.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::collections::graphframe::{GraphExecutionIntent, GraphFrame, GraphProcedureExpr};
use crate::form::{FormVmOperationKind, LinkedExecutableForm};
use crate::projection::eval::algorithm::ExecutionMode;
use crate::shell::ShellComponentMode;
use crate::task::frame::{TaskFrame, TaskFramePolicy};
use crate::types::catalog::GraphCatalog;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormVmOperationStatus {
    Prepared,
    Delegated,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormVmOperationReceipt {
    pub operation_sequence: u64,
    pub operation: String,
    pub status: FormVmOperationStatus,
    pub evidence_kind: String,
    pub evidence_reference: String,
}

pub struct FormVmRuntimePreparation {
    graph_intent: Option<GraphExecutionIntent>,
    task_frame: Option<TaskFrame<GraphExecutionIntent>>,
    pub receipts: Vec<FormVmOperationReceipt>,
}

impl FormVmRuntimePreparation {
    pub fn graph_intent(&self) -> Option<&GraphExecutionIntent> {
        self.graph_intent.as_ref()
    }

    pub fn task_frame(&self) -> Option<&TaskFrame<GraphExecutionIntent>> {
        self.task_frame.as_ref()
    }
}

pub fn prepare_linked_form(
    executable: &LinkedExecutableForm,
    default_input: &Value,
    op_inputs: &HashMap<String, Value>,
    execution_mode: ExecutionMode,
    catalog: Arc<dyn GraphCatalog>,
) -> Result<FormVmRuntimePreparation, String> {
    let mut graph_intent = None;
    let mut task_frame = None;
    let mut receipts = Vec::new();

    for operation in &executable.operations {
        match &operation.kind {
            FormVmOperationKind::EstablishPrinciple => receipts.push(receipt(
                operation.sequence,
                "establish_principle",
                FormVmOperationStatus::Prepared,
                "pure_form_principle",
                &executable.form_id,
            )),
            FormVmOperationKind::ProjectApplication { application_form } => {
                receipts.push(receipt(
                    operation.sequence,
                    "project_application",
                    FormVmOperationStatus::Prepared,
                    "application_form",
                    application_form,
                ));
            }
            FormVmOperationKind::DetermineGraph { binding } => {
                let graph_name = graph_name(default_input)?;
                let store = catalog
                    .get(graph_name)
                    .ok_or_else(|| format!("graph `{graph_name}` was not found for FormVM"))?;
                let frame = GraphFrame::from_store(store)
                    .map_err(|error| format!("GraphFrame determination failed: {error}"))?;
                let mut plan = frame.plan();
                for candidate in &executable.operations {
                    let FormVmOperationKind::InvokeOperator { service, operator } = &candidate.kind
                    else {
                        continue;
                    };
                    if service != "form.algorithms" {
                        continue;
                    }
                    let alias = algorithm_alias(operator);
                    let mut procedure =
                        GraphProcedureExpr::new(alias.clone(), shell_mode(execution_mode)?);
                    if let Some(inputs) = op_inputs.get(&alias).and_then(Value::as_object) {
                        for (key, value) in inputs {
                            procedure = procedure.with_input(key, value.clone());
                        }
                    }
                    plan = plan.procedure(procedure);
                }
                let intent = plan.compile_execution_intent().map_err(|error| {
                    format!("GraphFrame execution-intent compilation failed: {error}")
                })?;
                receipts.push(receipt(
                    operation.sequence,
                    "determine_graph",
                    FormVmOperationStatus::Prepared,
                    "graph_execution_intent",
                    &format!("{binding}:{graph_name}:{}", intent.objective().identity()),
                ));
                graph_intent = Some(intent);
            }
            FormVmOperationKind::ConstituteTask { binding } => {
                let intent = graph_intent.as_ref().ok_or_else(|| {
                    "TaskFrame constitution requires a prior GraphExecutionIntent".to_string()
                })?;
                let concurrency = default_input
                    .get("concurrency")
                    .and_then(Value::as_u64)
                    .and_then(|value| usize::try_from(value).ok())
                    .unwrap_or(1)
                    .max(1);
                let frame =
                    TaskFrame::from_graph_intent(intent.clone(), TaskFramePolicy::new(concurrency))
                        .map_err(|error| format!("TaskFrame constitution failed: {error}"))?;
                let task_reference = frame
                    .task_name()
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("{binding}:anonymous"));
                receipts.push(receipt(
                    operation.sequence,
                    "constitute_task",
                    FormVmOperationStatus::Prepared,
                    "task_frame",
                    &task_reference,
                ));
                task_frame = Some(frame);
            }
            FormVmOperationKind::InvokeOperator { service, operator } => receipts.push(receipt(
                operation.sequence,
                "invoke_operator",
                FormVmOperationStatus::Delegated,
                service,
                operator,
            )),
            FormVmOperationKind::CollectEvidence { binding } => receipts.push(receipt(
                operation.sequence,
                "collect_evidence",
                FormVmOperationStatus::Prepared,
                "evidence_contract",
                binding,
            )),
            FormVmOperationKind::ReturnForm { binding } => receipts.push(receipt(
                operation.sequence,
                "return_form",
                FormVmOperationStatus::Prepared,
                "return_contract",
                binding,
            )),
            FormVmOperationKind::DeferredCompatibility { pattern } => receipts.push(receipt(
                operation.sequence,
                "deferred_compatibility",
                FormVmOperationStatus::Deferred,
                "compatibility_pattern",
                pattern,
            )),
        }
    }

    Ok(FormVmRuntimePreparation {
        graph_intent,
        task_frame,
        receipts,
    })
}

fn receipt(
    operation_sequence: u64,
    operation: &str,
    status: FormVmOperationStatus,
    evidence_kind: &str,
    evidence_reference: &str,
) -> FormVmOperationReceipt {
    FormVmOperationReceipt {
        operation_sequence,
        operation: operation.to_string(),
        status,
        evidence_kind: evidence_kind.to_string(),
        evidence_reference: evidence_reference.to_string(),
    }
}

fn graph_name(default_input: &Value) -> Result<&str, String> {
    default_input
        .get("graphName")
        .or_else(|| default_input.get("graph_name"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .ok_or_else(|| "DetermineGraph requires default_input.graphName".to_string())
}

fn algorithm_alias(pattern: &str) -> String {
    for prefix in ["algo.", "algorithm.", "applications.algorithms."] {
        if let Some(alias) = pattern.strip_prefix(prefix) {
            return alias.to_string();
        }
    }
    pattern.to_string()
}

fn shell_mode(mode: ExecutionMode) -> Result<ShellComponentMode, String> {
    match mode {
        ExecutionMode::Stream => Ok(ShellComponentMode::Stream),
        ExecutionMode::Stats => Ok(ShellComponentMode::Stats),
        ExecutionMode::MutateNodeProperty => Ok(ShellComponentMode::Mutate),
        ExecutionMode::WriteNodeProperty => Ok(ShellComponentMode::Write),
        ExecutionMode::Train
        | ExecutionMode::WriteRelationship
        | ExecutionMode::MutateRelationship => Err(format!(
            "GraphFrame does not support execution mode {mode:?}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use super::*;
    use crate::form::{
        ApplicationForm, Context, FormLinker, FormShape, Morph, ProgramSpec, Shape, Specification,
    };
    use crate::types::catalog::InMemoryGraphCatalog;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    fn linked_organon() -> LinkedExecutableForm {
        let program = ProgramSpec::new(
            FormShape::new(
                Shape::default(),
                Context::default(),
                Morph::new(vec!["algo.pagerank".to_string()]),
            ),
            Specification::new("form.organon".to_string(), None, HashMap::new()),
            vec![],
            vec![ApplicationForm::organon()],
            vec!["organon".to_string()],
        );
        FormLinker::new()
            .link(&program, Some("graph://runtime".to_string()))
            .expect("Organon Form should link")
    }

    #[test]
    fn prepares_real_graph_intent_and_task_frame_from_linked_operations() {
        let catalog = Arc::new(InMemoryGraphCatalog::new());
        catalog.set(
            "runtime",
            Arc::new(
                DefaultGraphStore::random(&RandomGraphConfig::seeded(42))
                    .expect("random graph should build"),
            ),
        );

        let preparation = prepare_linked_form(
            &linked_organon(),
            &json!({"graphName": "runtime", "concurrency": 2}),
            &HashMap::new(),
            ExecutionMode::Stream,
            catalog,
        )
        .expect("linked Form should prepare runtime frames");

        let intent = preparation
            .graph_intent()
            .expect("GraphExecutionIntent should be determined");
        assert_eq!(intent.program().len(), 1);
        assert_eq!(
            intent.program().calls()[0].component.as_str(),
            "gds.algorithms.centrality.pagerank"
        );

        let task_frame = preparation
            .task_frame()
            .expect("TaskFrame should be constituted");
        assert_eq!(task_frame.workflow().frames().len(), 2);
        assert!(preparation.receipts.iter().any(|receipt| {
            receipt.operation == "determine_graph"
                && receipt.status == FormVmOperationStatus::Prepared
        }));
        assert!(preparation.receipts.iter().any(|receipt| {
            receipt.operation == "constitute_task"
                && receipt.status == FormVmOperationStatus::Prepared
        }));
    }
}
