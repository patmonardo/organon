//! GraphForm subsystem for GraphFrame semantic persistence.
//!
//! This module compiles GraphFrame declarations into a Dataset-backed
//! component-model manifest so GraphStore-driven semantics can be materialized
//! before a dedicated DataFrame backend is complete.

use std::collections::BTreeSet;

use serde_json::json;

use crate::collections::dataframe::GDSFrameError;
use crate::collections::dataset::lab::compile::DatasetCompilation;
use crate::collections::dataset::lab::compile::DatasetCompilationArtifacts;
use crate::collections::dataset::lab::compile::DatasetNode;
use crate::collections::dataset::lab::compile::DatasetNodeKind;
use crate::collections::dataset::lab::protocol::dataop::DatasetDataOpExpr;
use crate::collections::dataset::lab::protocol::metadata::DatasetMetadataExpr;
use crate::collections::dataset::lab::protocol::projection::DatasetProjectionExpr;
use crate::collections::dataset::lab::toolchain::DatasetPipeline;
use crate::collections::dataset::lab::toolchain::FeatureSpecRef;
use crate::collections::dataset::lab::toolchain::GenusSpecies;
use crate::collections::dataset::lab::toolchain::LogicalEngineIntent;
use crate::collections::dataset::lab::toolchain::ModelSpecRef;
use crate::collections::dataset::lab::toolchain::MvcEngineIntent;
use crate::collections::dataset::lab::toolchain::SdslSpecification;
use crate::collections::graphframe::expr::GraphFrameExpr;
use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked;
use crate::collections::graphframe::rational_language::lower_graph_semantic_program;
use crate::collections::graphframe::rational_language::GraphRationalLanguageError;
use crate::collections::graphframe::rational_language::GraphSemanticProgram;
use crate::shell::ShellComponentExecutionKind;
use crate::shell::ShellComponentMode;
use crate::shell::ShellComponentPlan;
use crate::task::frame::TaskFrame;
use crate::task::frame::TaskReturnContract;
use crate::task::spec::TaskSpecError;

#[derive(Debug, thiserror::Error)]
pub enum GraphFormError {
    #[error(transparent)]
    Rational(#[from] GraphRationalLanguageError),

    #[error(transparent)]
    GraphFrame(#[from] crate::collections::graphframe::frame::GraphFrameError),

    #[error(transparent)]
    TaskSpec(#[from] TaskSpecError),

    #[error("graph form compilation is invalid: {0}")]
    InvalidCompilation(String),

    #[error("graph automation contract is invalid: {0}")]
    InvalidAutomationContract(String),

    #[error(transparent)]
    DataFrame(#[from] GDSFrameError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphTransmissionTarget {
    TaskDaemon,
}

impl GraphTransmissionTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaskDaemon => "task-daemon",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphExecutionSurface {
    Algorithm,
    Pipeline,
    StoreApi,
}

impl GraphExecutionSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Algorithm => "algorithm",
            Self::Pipeline => "pipeline",
            Self::StoreApi => "store_api",
        }
    }

    pub fn from_surface(value: &str) -> Option<Self> {
        match value {
            "algorithm" => Some(Self::Algorithm),
            "pipeline" => Some(Self::Pipeline),
            "store_api" => Some(Self::StoreApi),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphExecutionSurfaceMode {
    None,
    Single,
    Hybrid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphDatasetFraming {
    DatasetSdkDataFrame,
    GmlDataset,
}

impl GraphDatasetFraming {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DatasetSdkDataFrame => "dataset_sdk_dataframe",
            Self::GmlDataset => "gml_dataset",
        }
    }

    pub fn from_framing(value: &str) -> Option<Self> {
        match value {
            "dataset_sdk_dataframe" => Some(Self::DatasetSdkDataFrame),
            "gml_dataset" => Some(Self::GmlDataset),
            _ => None,
        }
    }
}

impl GraphExecutionSurfaceMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Single => "single",
            Self::Hybrid => "hybrid",
        }
    }

    pub fn from_surfaces(execution_surfaces: &[String]) -> Self {
        match execution_surfaces.len() {
            0 => Self::None,
            1 => Self::Single,
            _ => Self::Hybrid,
        }
    }

    pub fn from_mode(value: &str) -> Self {
        match value {
            "single" => Self::Single,
            "hybrid" => Self::Hybrid,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphGdslTransmissionSpec {
    pub principle: String,
    pub target: GraphTransmissionTarget,
    pub namespace: String,
    pub objective_source: String,
    pub objective_identity: String,
    pub workflow_stage_count: usize,
    pub return_policy: String,
    pub expected_outputs: Vec<String>,
    pub execution_surfaces: Vec<String>,
    pub primary_execution_surface: Option<String>,
    pub execution_surface_mode: String,
    pub dataset_framings: Vec<String>,
    pub primary_dataset_framing: Option<String>,
    pub plan_logics: Vec<String>,
    pub graph_forms: Vec<String>,
    pub graph_store_components: Vec<String>,
    pub synthetic_moments: Vec<String>,
    pub empirical_outflows: Vec<GraphEmpiricalOutflowAspect>,
}

impl GraphGdslTransmissionSpec {
    pub fn primary_execution_surface_kind(&self) -> Option<GraphExecutionSurface> {
        self.primary_execution_surface
            .as_deref()
            .and_then(GraphExecutionSurface::from_surface)
    }

    pub fn execution_surface_mode_kind(&self) -> GraphExecutionSurfaceMode {
        GraphExecutionSurfaceMode::from_mode(&self.execution_surface_mode)
    }

    pub fn execution_surface_kinds(&self) -> Vec<GraphExecutionSurface> {
        self.execution_surfaces
            .iter()
            .filter_map(|value| GraphExecutionSurface::from_surface(value))
            .collect()
    }

    pub fn routes_via_store_api(&self) -> bool {
        self.execution_surface_kinds()
            .iter()
            .any(|surface| *surface == GraphExecutionSurface::StoreApi)
    }

    pub fn is_hybrid_execution(&self) -> bool {
        self.execution_surface_mode_kind() == GraphExecutionSurfaceMode::Hybrid
    }

    pub fn primary_dataset_framing_kind(&self) -> Option<GraphDatasetFraming> {
        self.primary_dataset_framing
            .as_deref()
            .and_then(GraphDatasetFraming::from_framing)
    }

    pub fn dataset_framing_kinds(&self) -> Vec<GraphDatasetFraming> {
        self.dataset_framings
            .iter()
            .filter_map(|value| GraphDatasetFraming::from_framing(value))
            .collect()
    }

    pub fn is_dataset_sdk_rooted(&self) -> bool {
        self.dataset_framing_kinds()
            .iter()
            .any(|framing| *framing == GraphDatasetFraming::DatasetSdkDataFrame)
    }

    pub fn includes_gml_dataset_framing(&self) -> bool {
        self.dataset_framing_kinds()
            .iter()
            .any(|framing| *framing == GraphDatasetFraming::GmlDataset)
    }

    /// Initial GML Dataset realization for Enterprise G-DSL contracts.
    pub fn to_initial_gml_dataset_pipeline(&self) -> DatasetPipeline {
        let graph_ref = format!("{}::{}", self.objective_source, self.objective_identity);
        let specification = SdslSpecification {
            id: format!(
                "gdsl.enterprise.gml.{}.{}",
                sanitize_contract_segment(&self.namespace),
                sanitize_contract_segment(&self.objective_identity)
            ),
            title: format!(
                "Enterprise Graph DSL GML Dataset {}",
                self.objective_identity
            ),
            classification: GenusSpecies {
                genus: "dataset_sdk".to_string(),
                species: "gml_dataset".to_string(),
            },
            gdsl_source: Some(self.principle.clone()),
            models: vec![ModelSpecRef {
                id: graph_ref.clone(),
                label: self.objective_identity.clone(),
            }],
            features: self
                .graph_forms
                .iter()
                .map(|form| FeatureSpecRef {
                    id: format!("form:{}", sanitize_contract_segment(form)),
                    label: form.clone(),
                    model_id: Some(graph_ref.clone()),
                })
                .collect(),
            logical_engine: LogicalEngineIntent::RelativeForm,
            mvc_engine: MvcEngineIntent::ReactNext,
        };

        DatasetPipeline::new()
            .with_specification(specification)
            .with_metadata(DatasetMetadataExpr::new(
                "gdsl.contract.kind",
                "enterprise_graph_dsl",
            ))
            .with_metadata(DatasetMetadataExpr::new(
                "gdsl.contract.namespace",
                self.namespace.clone(),
            ))
            .with_metadata(DatasetMetadataExpr::new(
                "gdsl.dataset.primary_framing",
                self.primary_dataset_framing.clone().unwrap_or_else(|| {
                    GraphDatasetFraming::DatasetSdkDataFrame
                        .as_str()
                        .to_string()
                }),
            ))
            .with_metadata(DatasetMetadataExpr::new(
                "gdsl.dataset.framings",
                json!(self.dataset_framings),
            ))
            .with_metadata(DatasetMetadataExpr::new(
                "gdsl.execution.surfaces",
                json!(self.execution_surfaces),
            ))
            .with_op(DatasetDataOpExpr::input_with(
                format!("{}.gml.input", self.objective_identity),
                json!({
                    "domain": "graph",
                    "framing": "gml_dataset",
                    "graph_ref": graph_ref,
                }),
            ))
            .with_op(DatasetDataOpExpr::encode_with(
                format!("{}.gml.encode", self.objective_identity),
                json!({
                    "domain": "graph",
                    "encoding": "gml",
                    "semantics": "dataset_sdk_dataframe",
                }),
            ))
            .with_op(DatasetDataOpExpr::transform_with(
                format!("{}.gml.transform", self.objective_identity),
                json!({
                    "domain": "graph",
                    "policy": self.execution_surface_mode,
                    "components": self.graph_store_components,
                }),
            ))
            .with_op(DatasetDataOpExpr::decode_with(
                format!("{}.gml.decode", self.objective_identity),
                json!({
                    "domain": "graph",
                    "target": "dataframe",
                }),
            ))
            .with_op(DatasetDataOpExpr::output_with(
                format!("{}.gml.output", self.objective_identity),
                json!({
                    "domain": "graph",
                    "outputs": self.expected_outputs,
                }),
            ))
            .with_projection(DatasetProjectionExpr::graph(vec![
                "node_id".to_string(),
                "edge_id".to_string(),
                "src_id".to_string(),
                "dst_id".to_string(),
                "type".to_string(),
                "props".to_string(),
            ]))
    }
}

pub fn build_initial_gml_dataset_pipeline(spec: &GraphGdslTransmissionSpec) -> DatasetPipeline {
    spec.to_initial_gml_dataset_pipeline()
}

fn sanitize_contract_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch.to_ascii_lowercase());
        } else {
            out.push('_');
        }
    }

    if out.is_empty() {
        "unnamed".to_string()
    } else {
        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphPlanLogic {
    AlgorithmMediation,
    PipelineMediation,
    StoreApiMediation,
    PersistenceMediation,
    OutputMediation,
}

impl GraphPlanLogic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlgorithmMediation => "algorithm-mediation",
            Self::PipelineMediation => "pipeline-mediation",
            Self::StoreApiMediation => "store-api-mediation",
            Self::PersistenceMediation => "persistence-mediation",
            Self::OutputMediation => "output-mediation",
        }
    }
}

pub fn derive_plan_logics<Program>(
    task_frame: &TaskFrame<Program>,
    outflows: &[GraphEmpiricalOutflowAspect],
) -> Vec<GraphPlanLogic> {
    let mut logics = BTreeSet::new();

    for aspect in outflows {
        match aspect.channel {
            GraphOutflowChannel::Algorithm => {
                logics.insert(GraphPlanLogic::AlgorithmMediation);
            }
            GraphOutflowChannel::Pipeline => {
                logics.insert(GraphPlanLogic::PipelineMediation);
            }
            GraphOutflowChannel::StoreApi => {
                logics.insert(GraphPlanLogic::StoreApiMediation);
            }
            GraphOutflowChannel::Output => {
                logics.insert(GraphPlanLogic::OutputMediation);
            }
            GraphOutflowChannel::Persistence => {}
        }
    }

    if task_frame.return_contract().requires_persistence() {
        logics.insert(GraphPlanLogic::PersistenceMediation);
    }

    logics.into_iter().collect()
}

pub fn derive_graph_store_components<Program>(task_frame: &TaskFrame<Program>) -> Vec<String> {
    let mut components = BTreeSet::new();

    for stage in task_frame.workflow().frames() {
        for step in stage.steps() {
            if let Some(component) = step.strip_prefix("graphframe.procedure.") {
                if !component.is_empty() {
                    components.insert(component.to_string());
                }
            }
        }
    }

    components.into_iter().collect()
}

pub fn derive_execution_surfaces(outflows: &[GraphEmpiricalOutflowAspect]) -> Vec<String> {
    let mut surfaces = BTreeSet::new();
    for aspect in outflows {
        let value = match aspect.channel {
            GraphOutflowChannel::Algorithm => Some("algorithm"),
            GraphOutflowChannel::Pipeline => Some("pipeline"),
            GraphOutflowChannel::StoreApi => Some("store_api"),
            GraphOutflowChannel::Persistence | GraphOutflowChannel::Output => None,
        };
        if let Some(value) = value {
            surfaces.insert(value.to_string());
        }
    }
    surfaces.into_iter().collect()
}

pub fn derive_primary_execution_surface(execution_surfaces: &[String]) -> Option<String> {
    execution_surfaces.first().cloned()
}

pub fn derive_execution_surface_mode(execution_surfaces: &[String]) -> String {
    GraphExecutionSurfaceMode::from_surfaces(execution_surfaces)
        .as_str()
        .to_string()
}

pub fn derive_dataset_framings(outflows: &[GraphEmpiricalOutflowAspect]) -> Vec<String> {
    let mut framings = BTreeSet::new();
    // Graph G-DSL is rooted in the Dataset SDK/DataFrame authority.
    framings.insert(
        GraphDatasetFraming::DatasetSdkDataFrame
            .as_str()
            .to_string(),
    );

    let has_graph_surface = outflows.iter().any(|aspect| {
        matches!(
            aspect.channel,
            GraphOutflowChannel::Algorithm
                | GraphOutflowChannel::Pipeline
                | GraphOutflowChannel::StoreApi
        )
    });

    if has_graph_surface {
        // GML dataset form is one framing of the broader Dataset SDK root.
        framings.insert(GraphDatasetFraming::GmlDataset.as_str().to_string());
    }

    framings.into_iter().collect()
}

pub fn derive_primary_dataset_framing(dataset_framings: &[String]) -> Option<String> {
    dataset_framings.first().cloned()
}

pub fn graph_forms() -> &'static [GraphFormKind] {
    &[
        GraphFormKind::GraphConstitutionForm,
        GraphFormKind::GraphFeatureGrammarForm,
        GraphFormKind::GraphMediationPlanForm,
    ]
}

pub fn build_gdsl_transmission_spec<Program>(
    task_frame: &TaskFrame<Program>,
    synthetic: &[GraphSyntheticMoment],
    outflows: &[GraphEmpiricalOutflowAspect],
) -> GraphGdslTransmissionSpec {
    let execution_surfaces = derive_execution_surfaces(outflows);
    let dataset_framings = derive_dataset_framings(outflows);

    let return_policy = if task_frame.return_contract().requires_persistence() {
        "persisted"
    } else {
        "ephemeral"
    }
    .to_string();

    GraphGdslTransmissionSpec {
        principle: "rational-principle:plan-workflow-outflow".to_string(),
        target: GraphTransmissionTarget::TaskDaemon,
        namespace: task_frame.namespace().to_string(),
        objective_source: task_frame.objective().source().to_string(),
        objective_identity: task_frame.objective().identity().to_string(),
        workflow_stage_count: task_frame.workflow().len(),
        return_policy,
        expected_outputs: task_frame.return_contract().outputs().to_vec(),
        primary_execution_surface: derive_primary_execution_surface(&execution_surfaces),
        execution_surface_mode: derive_execution_surface_mode(&execution_surfaces),
        execution_surfaces,
        primary_dataset_framing: derive_primary_dataset_framing(&dataset_framings),
        dataset_framings,
        plan_logics: derive_plan_logics(task_frame, outflows)
            .iter()
            .map(|logic| logic.as_str().to_string())
            .collect(),
        graph_forms: graph_forms()
            .iter()
            .map(|form| form.as_str().to_string())
            .collect(),
        graph_store_components: derive_graph_store_components(task_frame),
        synthetic_moments: synthetic
            .iter()
            .map(|moment| moment.as_str().to_string())
            .collect(),
        empirical_outflows: outflows.to_vec(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphSyntheticMoment {
    Principle,
    Plan,
    Workflow,
    Outflow,
}

impl GraphSyntheticMoment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Principle => "principle",
            Self::Plan => "plan",
            Self::Workflow => "workflow",
            Self::Outflow => "outflow",
        }
    }
}

pub fn synthetic_moments() -> &'static [GraphSyntheticMoment] {
    &[
        GraphSyntheticMoment::Principle,
        GraphSyntheticMoment::Plan,
        GraphSyntheticMoment::Workflow,
        GraphSyntheticMoment::Outflow,
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphOutflowChannel {
    Algorithm,
    Pipeline,
    StoreApi,
    Persistence,
    Output,
}

impl GraphOutflowChannel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Algorithm => "algorithm",
            Self::Pipeline => "pipeline",
            Self::StoreApi => "store_api",
            Self::Persistence => "persistence",
            Self::Output => "output",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphEmpiricalOutflowAspect {
    pub channel: GraphOutflowChannel,
    pub key: String,
    pub value: String,
}

impl GraphEmpiricalOutflowAspect {
    pub fn new(
        channel: GraphOutflowChannel,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            channel,
            key: key.into(),
            value: value.into(),
        }
    }
}

pub fn derive_empirical_outflows(
    plan: &ShellComponentPlan,
    return_contract: &TaskReturnContract,
) -> Vec<GraphEmpiricalOutflowAspect> {
    let has_algorithm = plan.has_algorithm_components();
    let has_pipeline = plan.has_pipeline_components();
    let has_store_api = plan.has_store_api_components();

    let mut outflows = Vec::new();
    if has_algorithm {
        outflows.push(GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::Algorithm,
            "execution_kind",
            "algorithm",
        ));
    }
    if has_pipeline {
        outflows.push(GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::Pipeline,
            "execution_kind",
            "pipeline",
        ));
    }
    if has_store_api {
        outflows.push(GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::StoreApi,
            "execution_kind",
            "store_api",
        ));
    }

    outflows.push(GraphEmpiricalOutflowAspect::new(
        GraphOutflowChannel::Persistence,
        "return_policy",
        if return_contract.requires_persistence() {
            "persisted"
        } else {
            "ephemeral"
        },
    ));

    for output in return_contract.outputs() {
        outflows.push(GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::Output,
            "output",
            output.clone(),
        ));
    }

    outflows
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphAutomationProfile {
    /// Agent-facing graph analytics scripting: no direct invoke mode.
    AgentAnalytics,
    /// Agent-facing pipeline orchestration over pipeline components.
    AgentPipelines,
    /// Combined automation profile: analytics plus pipelines.
    AgentHybrid,
}

impl GraphAutomationProfile {
    fn allows_mode(self, mode: ShellComponentMode) -> bool {
        match self {
            Self::AgentAnalytics => matches!(
                mode,
                ShellComponentMode::Stream
                    | ShellComponentMode::Stats
                    | ShellComponentMode::Estimate
                    | ShellComponentMode::Mutate
                    | ShellComponentMode::Write
            ),
            Self::AgentPipelines => mode == ShellComponentMode::Invoke,
            Self::AgentHybrid => true,
        }
    }

    fn allows_kind(self, kind: ShellComponentExecutionKind) -> bool {
        match self {
            Self::AgentAnalytics => kind == ShellComponentExecutionKind::Algorithm,
            Self::AgentPipelines => kind == ShellComponentExecutionKind::Pipeline,
            Self::AgentHybrid => true,
        }
    }
}

pub fn validate_automation_shell_plan(
    plan: &ShellComponentPlan,
    profile: GraphAutomationProfile,
) -> Result<(), GraphFormError> {
    if plan.calls().is_empty() {
        return Err(GraphFormError::InvalidAutomationContract(
            "plan must contain at least one component call".to_string(),
        ));
    }

    for (index, call) in plan.calls().iter().enumerate() {
        let descriptor = call.descriptor().ok_or_else(|| {
            GraphFormError::InvalidAutomationContract(format!(
                "step {index} references unknown component '{}'",
                call.component.as_str()
            ))
        })?;

        if !profile.allows_mode(call.mode) {
            return Err(GraphFormError::InvalidAutomationContract(format!(
                "step {index} component '{}' mode {:?} is disallowed for profile {:?}",
                descriptor.alias, call.mode, profile
            )));
        }

        if !profile.allows_kind(descriptor.execution_kind) {
            return Err(GraphFormError::InvalidAutomationContract(format!(
                "step {index} component '{}' execution kind {:?} is disallowed for profile {:?}",
                descriptor.alias, descriptor.execution_kind, profile
            )));
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphFormKind {
    GraphConstitutionForm,
    GraphFeatureGrammarForm,
    GraphMediationPlanForm,
}

impl GraphFormKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GraphConstitutionForm => "GraphConstitutionForm",
            Self::GraphFeatureGrammarForm => "GraphFeatureGrammarForm",
            Self::GraphMediationPlanForm => "GraphMediationPlanForm",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphFormCompilation {
    semantic_program: GraphSemanticProgram,
    compilation: DatasetCompilation,
}

impl GraphFormCompilation {
    pub fn semantic_program(&self) -> &GraphSemanticProgram {
        &self.semantic_program
    }

    pub fn compilation(&self) -> &DatasetCompilation {
        &self.compilation
    }

    pub fn materialize_component_model(
        &self,
        base_name: impl AsRef<str>,
    ) -> Result<DatasetCompilationArtifacts, GraphFormError> {
        self.compilation
            .validate()
            .map_err(GraphFormError::InvalidCompilation)?;
        Ok(self.compilation.materialize_artifact_datasets(base_name)?)
    }
}

pub fn compile_graph_form(
    expressions: &[GraphFrameExpr],
    grammar: &GraphFeatureGrammarChecked,
) -> Result<GraphFormCompilation, GraphFormError> {
    let semantic_program = lower_graph_semantic_program(expressions, grammar)?;
    let grammar_name = semantic_program.grammar().form().name.clone();
    let grammar_version = semantic_program.grammar().form().version.clone();

    let mut compilation = DatasetCompilation::new();
    let root_id = format!(
        "graphform.image:{}:{}",
        sanitize_segment(&grammar_name),
        sanitize_segment(&grammar_version)
    );
    let mut root = DatasetNode::new(
        root_id.clone(),
        format!("GraphForm {} {}", grammar_name, grammar_version),
        DatasetNodeKind::Image,
    )
    .with_meta(
        "graphform.kind",
        GraphFormKind::GraphConstitutionForm.as_str(),
    )
    .with_meta("graphform.grammar", grammar_name.clone())
    .with_meta("graphform.grammar.version", grammar_version.clone())
    .with_meta(
        "graphform.gdsl.transmission_target",
        GraphTransmissionTarget::TaskDaemon.as_str(),
    )
    .with_meta(
        "graphform.gdsl.principle",
        "rational-principle:plan-workflow-outflow",
    )
    .with_meta(
        "graphform.synthetic.moments",
        synthetic_moments()
            .iter()
            .map(|moment| moment.as_str())
            .collect::<Vec<_>>()
            .join(","),
    );

    let grammar_node_id = format!(
        "graphform.grammar:{}:{}",
        sanitize_segment(&grammar_name),
        sanitize_segment(&grammar_version)
    );
    let grammar_node = DatasetNode::new(
        grammar_node_id.clone(),
        format!("{}:{}", grammar_name, grammar_version),
        DatasetNodeKind::Feature,
    )
    .with_meta(
        "graphform.kind",
        GraphFormKind::GraphFeatureGrammarForm.as_str(),
    )
    .with_meta(
        "graphform.feature.rule.count",
        semantic_program
            .grammar()
            .form()
            .feature_rules
            .len()
            .to_string(),
    )
    .with_meta(
        "graphform.derivation.rule.count",
        semantic_program
            .grammar()
            .form()
            .derivations
            .len()
            .to_string(),
    );
    compilation.add_node(grammar_node);
    root = root.with_dep(grammar_node_id.clone());

    let mut template_node_ids = BTreeSet::new();
    for template in semantic_program.shared_plan_templates() {
        let template_node_id = format!(
            "graphform.template:{}:{}",
            sanitize_segment(template.plan_id()),
            sanitize_segment(template.feature_id())
        );
        if template_node_ids.insert(template_node_id.clone()) {
            let template_node = DatasetNode::new(
                template_node_id.clone(),
                template.plan_id().to_string(),
                DatasetNodeKind::Function,
            )
            .with_meta(
                "graphform.kind",
                GraphFormKind::GraphMediationPlanForm.as_str(),
            )
            .with_meta(
                "graphform.template.feature_id",
                template.feature_id().to_string(),
            )
            .with_meta(
                "graphform.template.triad",
                template.triad().as_str().to_string(),
            );
            compilation.add_node(template_node);
            root = root.with_dep(template_node_id);
        }
    }

    for lowering in semantic_program.lowerings() {
        let model_id = lowering.model().id.0.clone();
        let feature = lowering.feature();
        let feature_id = feature
            .id()
            .map(|id| id.as_str().to_string())
            .unwrap_or_else(|| "unknown-feature".to_string());
        let plan_id = feature.plan().name().unwrap_or("unnamed-plan").to_string();

        let model_node_id = format!("graphform.model:{}", sanitize_segment(&model_id));
        let model_node = DatasetNode::new(
            model_node_id.clone(),
            model_id.clone(),
            DatasetNodeKind::Model,
        )
        .with_meta(
            "graphform.kind",
            GraphFormKind::GraphConstitutionForm.as_str(),
        )
        .with_meta(
            "graphform.model.view.input",
            format!("{:?}", lowering.model().input),
        )
        .with_meta(
            "graphform.model.view.output",
            format!("{:?}", lowering.model().output),
        );
        compilation.add_node(model_node);
        root = root.with_dep(model_node_id.clone());

        let feature_node_id = format!(
            "graphform.feature:{}:{}",
            sanitize_segment(&model_id),
            sanitize_segment(&feature_id)
        );
        let feature_node = DatasetNode::new(
            feature_node_id.clone(),
            feature_id.clone(),
            DatasetNodeKind::Feature,
        )
        .with_dep(model_node_id.clone())
        .with_dep(grammar_node_id.clone())
        .with_meta(
            "graphform.kind",
            GraphFormKind::GraphFeatureGrammarForm.as_str(),
        )
        .with_meta("graphform.model.anchor", model_id.clone())
        .with_meta(
            "graphform.feature.anchor.count",
            feature.plan().synthesis().feature_anchors.len().to_string(),
        );
        compilation.add_node(feature_node);
        root = root.with_dep(feature_node_id.clone());

        let plan_node_id = format!(
            "graphform.plan:{}:{}",
            sanitize_segment(&model_id),
            sanitize_segment(&plan_id)
        );
        let plan_node = DatasetNode::new(plan_node_id.clone(), plan_id, DatasetNodeKind::Expr)
            .with_dep(feature_node_id)
            .with_meta(
                "graphform.kind",
                GraphFormKind::GraphMediationPlanForm.as_str(),
            )
            .with_meta(
                "graphform.plan.model_anchor",
                feature
                    .plan()
                    .synthesis()
                    .model_anchor
                    .clone()
                    .unwrap_or_default(),
            )
            .with_meta(
                "graphform.plan.feature_anchors",
                feature.plan().synthesis().feature_anchors.join(","),
            )
            .with_meta(
                "graphform.plan.rational_mode",
                feature
                    .plan()
                    .principle()
                    .map(|principle| principle.mode.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            );
        compilation.add_node(plan_node);
        root = root.with_dep(plan_node_id);
    }

    compilation.add_node(root);
    compilation.add_entrypoint(root_id);

    Ok(GraphFormCompilation {
        semantic_program,
        compilation,
    })
}

fn sanitize_segment(value: &str) -> String {
    let mapped = value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect::<String>();
    mapped.trim_matches('-').to_lowercase()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::collections::dataset::DatasetProjectionKind;
    use crate::collections::graphframe::feature_grammar::validate_graph_feature_grammar;
    use crate::collections::graphframe::feature_grammar::GraphFeatureCardinality;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;
    use crate::collections::graphframe::feature_grammar::GraphFeatureRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
    use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::collections::graphframe::model::GraphFrameModelExt;
    use crate::collections::graphframe::plan::GraphFramePlanExt;
    use crate::shell::ShellAddress;
    use crate::shell::ShellAlgebra;
    use crate::shell::ShellComponentMode;
    use crate::shell::ShellComponentPlan;
    use crate::shell::ShellPipeline;
    use crate::shell::ShellRegister;
    use crate::task::concurrency::Concurrency;
    use crate::task::frame::TaskFrame;
    use crate::task::frame::TaskObjectiveRef;
    use crate::task::frame::TaskReturnContract;
    use crate::task::runtime::TaskFrameKind;
    use crate::task::runtime::TaskStage;
    use crate::task::spec::TaskWorkflow;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::build_gdsl_transmission_spec;
    use super::build_initial_gml_dataset_pipeline;
    use super::compile_graph_form;
    use super::derive_dataset_framings;
    use super::derive_empirical_outflows;
    use super::derive_execution_surface_mode;
    use super::derive_execution_surfaces;
    use super::derive_graph_store_components;
    use super::derive_plan_logics;
    use super::derive_primary_dataset_framing;
    use super::derive_primary_execution_surface;
    use super::graph_forms;
    use super::synthetic_moments;
    use super::validate_automation_shell_plan;
    use super::GraphAutomationProfile;
    use super::GraphDatasetFraming;
    use super::GraphEmpiricalOutflowAspect;
    use super::GraphExecutionSurface;
    use super::GraphExecutionSurfaceMode;
    use super::GraphOutflowChannel;
    use super::GraphPlanLogic;
    use super::GraphTransmissionTarget;

    fn deterministic_frame() -> GraphFrame {
        let store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded graph store should build"),
        );
        GraphFrame::from_store(store).expect("graph frame should build")
    }

    fn density_grammar(
    ) -> crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked {
        validate_graph_feature_grammar(
            GraphFeatureGrammarForm::new("graph_theory", "v1").with_feature_rule(
                GraphFeatureRule::new(
                    GraphFeatureStratum::Graph,
                    "density",
                    GraphFeatureValueType::Scalar,
                    true,
                    GraphFeatureCardinality::One,
                ),
            ),
        )
        .expect("density grammar should be valid")
    }

    #[test]
    fn graph_form_compilation_materializes_component_model_datasets() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .model("graph-theory.density-model.v2")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.shared")
            .into_plan();
        let compilation = compile_graph_form(plan.expressions(), &density_grammar())
            .expect("graph form should compile");
        let datasets = compilation
            .materialize_component_model("graphframe.graphform.component")
            .expect("component model datasets should materialize");

        assert!(datasets.artifacts.row_count() > 0);
        assert!(datasets.relations.row_count() > 0);
        assert!(datasets.properties.row_count() > 0);
    }

    #[test]
    fn automation_profile_rejects_invoke_for_analytics() {
        let plan = ShellComponentPlan::new(ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        ))
        .component("to_undirected", ShellComponentMode::Invoke)
        .expect("invoke component should build")
        .finish();

        let error = validate_automation_shell_plan(&plan, GraphAutomationProfile::AgentAnalytics)
            .expect_err("analytics profile should reject invoke mode");
        assert!(matches!(
            error,
            super::GraphFormError::InvalidAutomationContract(_)
        ));
    }

    #[test]
    fn automation_profile_accepts_algorithm_stats_for_analytics() {
        let plan = ShellComponentPlan::new(ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        ))
        .component("pagerank", ShellComponentMode::Stats)
        .expect("stats component should build")
        .finish();

        validate_automation_shell_plan(&plan, GraphAutomationProfile::AgentAnalytics)
            .expect("analytics profile should accept algorithm stats mode");
    }

    #[test]
    fn empirical_outflows_capture_execution_policy_and_outputs() {
        let plan = ShellComponentPlan::new(ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        ))
        .component("pagerank", ShellComponentMode::Stats)
        .expect("stats component should build")
        .finish();
        let return_contract = TaskReturnContract::ephemeral(vec![
            "graphframe.compute.result".to_string(),
            "graphframe.dataset.graph".to_string(),
        ]);

        let outflows = derive_empirical_outflows(&plan, &return_contract);
        assert!(outflows.iter().any(|aspect| {
            aspect
                == &GraphEmpiricalOutflowAspect::new(
                    GraphOutflowChannel::Algorithm,
                    "execution_kind",
                    "algorithm",
                )
        }));
        assert!(outflows.iter().any(|aspect| {
            aspect
                == &GraphEmpiricalOutflowAspect::new(
                    GraphOutflowChannel::Persistence,
                    "return_policy",
                    "ephemeral",
                )
        }));
        assert!(outflows.iter().any(|aspect| {
            aspect.channel == GraphOutflowChannel::Output
                && aspect.value == "graphframe.compute.result"
        }));
    }

    #[test]
    fn gdsl_transmission_spec_targets_task_daemon() {
        let stage = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            vec!["graphframe.procedure.pagerank".to_string()],
            1,
            Concurrency::of(1),
        )
        .with_image_kind(TaskFrameKind::GraphAlgorithm)
        .with_inputs(vec!["graphframe.seed".to_string()])
        .with_outputs(vec!["graphframe.compute.result".to_string()]);
        let task_frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "view::all"),
            TaskWorkflow::new(vec![stage]).expect("workflow should build"),
            "program",
            TaskReturnContract::ephemeral(vec!["graphframe.compute.result".to_string()]),
        );
        let outflows = vec![GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::Algorithm,
            "execution_kind",
            "algorithm",
        )];

        let spec = build_gdsl_transmission_spec(&task_frame, synthetic_moments(), &outflows);
        assert_eq!(spec.target, GraphTransmissionTarget::TaskDaemon);
        assert_eq!(spec.namespace, "graphframe");
        assert_eq!(spec.workflow_stage_count, 1);
        assert_eq!(spec.return_policy, "ephemeral");
        assert_eq!(spec.synthetic_moments.len(), 4);
        assert_eq!(spec.execution_surfaces, vec!["algorithm"]);
        assert_eq!(spec.primary_execution_surface.as_deref(), Some("algorithm"));
        assert_eq!(spec.execution_surface_mode, "single");
        assert_eq!(
            spec.primary_dataset_framing.as_deref(),
            Some("dataset_sdk_dataframe")
        );
        assert_eq!(
            spec.primary_dataset_framing_kind(),
            Some(GraphDatasetFraming::DatasetSdkDataFrame)
        );
        assert!(spec.is_dataset_sdk_rooted());
        assert!(spec.includes_gml_dataset_framing());
        assert_eq!(
            spec.dataset_framings,
            vec!["dataset_sdk_dataframe", "gml_dataset"]
        );
        assert_eq!(
            spec.primary_execution_surface_kind(),
            Some(GraphExecutionSurface::Algorithm)
        );
        assert_eq!(
            spec.execution_surface_mode_kind(),
            GraphExecutionSurfaceMode::Single
        );
        assert!(!spec.routes_via_store_api());
        assert!(!spec.is_hybrid_execution());
        assert!(spec
            .plan_logics
            .iter()
            .any(|logic| logic == "algorithm-mediation"));
        assert_eq!(spec.graph_forms.len(), 3);
        assert_eq!(spec.graph_store_components, vec!["pagerank"]);
    }

    #[test]
    fn graph_plan_logics_and_graphstore_components_follow_workflow_and_outflows() {
        let stage = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            vec![
                "graphframe.procedure.pagerank".to_string(),
                "graphframe.procedure.leiden".to_string(),
            ],
            2,
            Concurrency::of(2),
        );
        let task_frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "view::all"),
            TaskWorkflow::new(vec![stage]).expect("workflow should build"),
            "program",
            TaskReturnContract::persisted(vec!["graphframe.dataset.graph".to_string()]),
        );
        let outflows = vec![
            GraphEmpiricalOutflowAspect::new(
                GraphOutflowChannel::Algorithm,
                "execution_kind",
                "algorithm",
            ),
            GraphEmpiricalOutflowAspect::new(
                GraphOutflowChannel::Output,
                "output",
                "graphframe.dataset.graph",
            ),
        ];

        let logics = derive_plan_logics(&task_frame, &outflows);
        assert!(logics.contains(&GraphPlanLogic::AlgorithmMediation));
        assert!(logics.contains(&GraphPlanLogic::OutputMediation));
        assert!(logics.contains(&GraphPlanLogic::PersistenceMediation));

        let components = derive_graph_store_components(&task_frame);
        assert_eq!(components, vec!["leiden", "pagerank"]);

        let forms = graph_forms()
            .iter()
            .map(|form| form.as_str().to_string())
            .collect::<Vec<_>>();
        assert_eq!(forms.len(), 3);

        let surfaces = derive_execution_surfaces(&outflows);
        assert_eq!(surfaces, vec!["algorithm"]);
        let primary = derive_primary_execution_surface(&surfaces);
        assert_eq!(primary.as_deref(), Some("algorithm"));
        assert_eq!(derive_execution_surface_mode(&surfaces), "single");
    }

    #[test]
    fn store_api_outflows_and_plan_logics_are_explicit() {
        let plan = ShellComponentPlan::new(ShellAddress::new(
            ShellRegister::Unified,
            ShellPipeline::ModelFeaturePlan,
            ShellAlgebra::ProgramFeature,
        ))
        .component("drop_graph", ShellComponentMode::Mutate)
        .expect("store api component should build")
        .finish();
        let return_contract = TaskReturnContract::persisted(vec![
            "graphframe.dataset.graph".to_string(),
            "graphframe.dataset.node".to_string(),
        ]);

        let outflows = derive_empirical_outflows(&plan, &return_contract);
        assert!(outflows.iter().any(|aspect| {
            aspect
                == &GraphEmpiricalOutflowAspect::new(
                    GraphOutflowChannel::StoreApi,
                    "execution_kind",
                    "store_api",
                )
        }));

        let stage = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            vec!["graphframe.procedure.drop_graph".to_string()],
            1,
            Concurrency::of(1),
        )
        .with_image_kind(TaskFrameKind::ShellProgram)
        .with_outputs(vec!["graphframe.dataset.graph".to_string()]);
        let task_frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "view::all"),
            TaskWorkflow::new(vec![stage]).expect("workflow should build"),
            "program",
            return_contract,
        );

        let logics = derive_plan_logics(&task_frame, &outflows);
        assert!(logics.contains(&GraphPlanLogic::StoreApiMediation));
        let surfaces = derive_execution_surfaces(&outflows);
        assert_eq!(surfaces, vec!["store_api"]);
        let primary = derive_primary_execution_surface(&surfaces);
        assert_eq!(primary.as_deref(), Some("store_api"));
        assert_eq!(derive_execution_surface_mode(&surfaces), "single");

        let spec = build_gdsl_transmission_spec(&task_frame, synthetic_moments(), &outflows);
        assert!(spec.routes_via_store_api());
        assert_eq!(
            spec.primary_execution_surface_kind(),
            Some(GraphExecutionSurface::StoreApi)
        );
    }

    #[test]
    fn hybrid_execution_surface_mode_is_reported() {
        let outflows = vec![
            GraphEmpiricalOutflowAspect::new(
                GraphOutflowChannel::Algorithm,
                "execution_kind",
                "algorithm",
            ),
            GraphEmpiricalOutflowAspect::new(
                GraphOutflowChannel::StoreApi,
                "execution_kind",
                "store_api",
            ),
        ];

        let surfaces = derive_execution_surfaces(&outflows);
        assert_eq!(surfaces, vec!["algorithm", "store_api"]);
        let dataset_framings = derive_dataset_framings(&outflows);
        assert_eq!(
            dataset_framings,
            vec!["dataset_sdk_dataframe", "gml_dataset"]
        );
        let primary_dataset_framing = derive_primary_dataset_framing(&dataset_framings);
        assert_eq!(
            primary_dataset_framing.as_deref(),
            Some("dataset_sdk_dataframe")
        );
        let primary = derive_primary_execution_surface(&surfaces);
        assert_eq!(primary.as_deref(), Some("algorithm"));
        assert_eq!(derive_execution_surface_mode(&surfaces), "hybrid");

        let stage = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            vec![
                "graphframe.procedure.pagerank".to_string(),
                "graphframe.procedure.drop_graph".to_string(),
            ],
            2,
            Concurrency::of(2),
        )
        .with_image_kind(TaskFrameKind::ShellProgram)
        .with_outputs(vec!["graphframe.dataset.graph".to_string()]);
        let task_frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "view::all"),
            TaskWorkflow::new(vec![stage]).expect("workflow should build"),
            "program",
            TaskReturnContract::persisted(vec!["graphframe.dataset.graph".to_string()]),
        );
        let spec = build_gdsl_transmission_spec(&task_frame, synthetic_moments(), &outflows);
        assert_eq!(
            spec.execution_surface_mode_kind(),
            GraphExecutionSurfaceMode::Hybrid
        );
        assert!(spec.is_hybrid_execution());
        assert!(spec.is_dataset_sdk_rooted());
        assert!(spec.includes_gml_dataset_framing());
    }

    #[test]
    fn enterprise_contract_builds_initial_gml_dataset_pipeline() {
        let stage = TaskStage::new(
            "graphframe".to_string(),
            "pipeline::GraphFrameCompute".to_string(),
            vec!["graphframe.procedure.pagerank".to_string()],
            1,
            Concurrency::of(1),
        )
        .with_image_kind(TaskFrameKind::GraphAlgorithm)
        .with_outputs(vec!["graphframe.compute.result".to_string()]);
        let task_frame = TaskFrame::new(
            "graphframe",
            TaskObjectiveRef::new("graphstore", "view::all"),
            TaskWorkflow::new(vec![stage]).expect("workflow should build"),
            "program",
            TaskReturnContract::ephemeral(vec!["graphframe.compute.result".to_string()]),
        );
        let outflows = vec![GraphEmpiricalOutflowAspect::new(
            GraphOutflowChannel::Algorithm,
            "execution_kind",
            "algorithm",
        )];

        let spec = build_gdsl_transmission_spec(&task_frame, synthetic_moments(), &outflows);
        let pipeline = build_initial_gml_dataset_pipeline(&spec);

        let built_spec = pipeline
            .specification
            .as_ref()
            .expect("pipeline should carry an SDSL specification");
        assert_eq!(built_spec.classification.genus, "dataset_sdk");
        assert_eq!(built_spec.classification.species, "gml_dataset");
        assert_eq!(pipeline.ops.len(), 5);
        assert!(pipeline
            .metadata
            .iter()
            .any(|entry| entry.key() == "gdsl.contract.kind"));
        assert!(pipeline
            .metadata
            .iter()
            .any(|entry| entry.key() == "gdsl.dataset.framings"));
        assert_eq!(
            pipeline
                .projection
                .as_ref()
                .expect("pipeline should include graph projection")
                .kind(),
            &DatasetProjectionKind::GraphToFrame
        );
    }
}
