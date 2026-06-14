//! Core GDS Shell state.

use std::fmt;

use polars::prelude::DataType;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::corpus::CorpusError;
use crate::collections::dataset::frame::DatasetDataFrameNs;
use crate::collections::dataset::{
    Corpus, Dataset, DatasetPipeline, LanguageModel, LogicError, LogicFrame, WhitespaceTokenizer,
    MLE,
};
use crate::core::graph_dimensions::ConcreteGraphDimensions;
use crate::task::progress::{TaskProgressTracker, Tasks};
use crate::form::{ProgramFeatureKind, ProgramFeatures};
use crate::task::memory::{MemoryEstimations, MemoryRange, MemoryTree};

use super::{
    ShellAddress, ShellAlgebra, ShellFold, ShellHelp, ShellMoment, ShellMomentKind, ShellPipeline,
    ShellPipelineDescriptor, ShellPipelineFacade, ShellProgram, ShellRegister, ShellSchema,
};

/// Lightweight seed extracted from the immediate DataFrame body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellSeed {
    columns: Vec<String>,
    dtypes: Vec<DataType>,
    row_count: usize,
    column_count: usize,
    schema: ShellSchema,
}

impl ShellSeed {
    pub fn from_dataframe(dataframe: &GDSDataFrame) -> Self {
        Self {
            columns: dataframe.column_names(),
            dtypes: dataframe.dtypes(),
            row_count: dataframe.row_count(),
            column_count: dataframe.column_count(),
            schema: ShellSchema::from_dataframe(dataframe),
        }
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn dtypes(&self) -> &[DataType] {
        &self.dtypes
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn schema(&self) -> &ShellSchema {
        &self.schema
    }
}

/// Shell-readable account of what a DataFrame body contributes to executable Form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellDataFrameKnowledge {
    address: ShellAddress,
    columns: Vec<String>,
    dtypes: Vec<DataType>,
    row_count: usize,
    column_count: usize,
    schema: ShellSchema,
    dataset_name: Option<String>,
    artifact_kind: Option<String>,
    facets: Vec<String>,
    program_name: Option<String>,
    program_feature_count: Option<usize>,
}

impl ShellDataFrameKnowledge {
    fn from_shell(shell: &GdsShell, seed: &ShellSeed) -> Self {
        let (dataset_name, artifact_kind, facets) = if let Some(dataset) = shell.dataset() {
            (
                dataset.name().map(ToOwned::to_owned),
                Some(format!("{:?}", dataset.artifact_kind())),
                dataset.artifact_profile().facets().to_vec(),
            )
        } else {
            (None, None, Vec::new())
        };

        Self {
            address: shell.address(),
            columns: seed.columns().to_vec(),
            dtypes: seed.dtypes().to_vec(),
            row_count: seed.row_count(),
            column_count: seed.column_count(),
            schema: seed.schema().clone(),
            dataset_name,
            artifact_kind,
            facets,
            program_name: shell
                .program()
                .map(|program| program.program_name().to_string()),
            program_feature_count: shell.program().map(ShellProgram::feature_count),
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn columns(&self) -> &[String] {
        &self.columns
    }

    pub fn dtypes(&self) -> &[DataType] {
        &self.dtypes
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn column_count(&self) -> usize {
        self.column_count
    }

    pub fn schema(&self) -> &ShellSchema {
        &self.schema
    }

    pub fn dataset_name(&self) -> Option<&str> {
        self.dataset_name.as_deref()
    }

    pub fn artifact_kind(&self) -> Option<&str> {
        self.artifact_kind.as_deref()
    }

    pub fn facets(&self) -> &[String] {
        &self.facets
    }

    pub fn program_name(&self) -> Option<&str> {
        self.program_name.as_deref()
    }

    pub fn program_feature_count(&self) -> Option<usize> {
        self.program_feature_count
    }
}

/// Shell-readable account of the Dataset middle: Model:Feature::Plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellModelFeaturePlanKnowledge {
    address: ShellAddress,
    model_name: Option<String>,
    model_artifact_kind: Option<String>,
    selected_forms: Vec<String>,
    program_name: Option<String>,
    feature_declarations: Vec<String>,
    principles: Vec<String>,
    concepts: Vec<String>,
    procedures: Vec<String>,
    plan_steps: Vec<String>,
    returns_to_concept: bool,
}

impl ShellModelFeaturePlanKnowledge {
    fn from_shell(shell: &GdsShell) -> Option<Self> {
        let dataset = shell.dataset();
        let program = shell.program();
        if dataset.is_none() && program.is_none() {
            return None;
        }

        let (model_name, model_artifact_kind) = if let Some(dataset) = dataset {
            (
                dataset.name().map(ToOwned::to_owned),
                Some(format!("{:?}", dataset.artifact_kind())),
            )
        } else {
            (None, None)
        };

        let selected_forms = program
            .map(|program| program.features().selected_forms.clone())
            .unwrap_or_default();
        let program_name = program.map(|program| program.program_name().to_string());
        let feature_declarations = program
            .map(|program| {
                program
                    .features()
                    .features
                    .iter()
                    .map(|feature| format!("{}::{}", feature.kind.as_str(), feature.value))
                    .collect()
            })
            .unwrap_or_default();
        let principles = features_by_kind(program, ProgramFeatureKind::Principle);
        let concepts = features_by_kind(program, ProgramFeatureKind::Concept);
        let procedures = features_by_kind(program, ProgramFeatureKind::Procedure);

        Some(Self {
            address: shell.address(),
            model_name,
            model_artifact_kind,
            selected_forms,
            program_name,
            feature_declarations,
            principles,
            concepts,
            procedures,
            plan_steps: shell.concept_return_plan_steps(),
            returns_to_concept: program.is_some(),
        })
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn model_name(&self) -> Option<&str> {
        self.model_name.as_deref()
    }

    pub fn model_artifact_kind(&self) -> Option<&str> {
        self.model_artifact_kind.as_deref()
    }

    pub fn selected_forms(&self) -> &[String] {
        &self.selected_forms
    }

    pub fn program_name(&self) -> Option<&str> {
        self.program_name.as_deref()
    }

    pub fn feature_declarations(&self) -> &[String] {
        &self.feature_declarations
    }

    pub fn principles(&self) -> &[String] {
        &self.principles
    }

    pub fn concepts(&self) -> &[String] {
        &self.concepts
    }

    pub fn procedures(&self) -> &[String] {
        &self.procedures
    }

    pub fn plan_steps(&self) -> &[String] {
        &self.plan_steps
    }

    pub fn returns_to_concept(&self) -> bool {
        self.returns_to_concept
    }
}

/// Semantic capability classes exposed by the Shell for LogicFrame evolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellSemanticCapability {
    FormIngestion,
    LogicalFormParsing,
    CorpusLanguagePairing,
    LanguageModelFitting,
    PrincipleGatedConceptReturn,
}

impl ShellSemanticCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FormIngestion => "form-ingestion",
            Self::LogicalFormParsing => "logical-form-parsing",
            Self::CorpusLanguagePairing => "corpus-language-pairing",
            Self::LanguageModelFitting => "language-model-fitting",
            Self::PrincipleGatedConceptReturn => "principle-gated-concept-return",
        }
    }
}

/// Shell-readable account of semantic-pipeline evolution toward LogicFrame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellSemanticPipelineKnowledge {
    address: ShellAddress,
    frame_ready: bool,
    middle_ready: bool,
    logic_frame_ready: bool,
    pureform_return_ready: bool,
    semantic_feature_count: usize,
    semantic_feature_kinds: Vec<String>,
    capabilities: Vec<ShellSemanticCapability>,
    evolution_path: Vec<String>,
}

impl ShellSemanticPipelineKnowledge {
    fn from_shell(shell: &GdsShell) -> Self {
        let frame_ready = shell.dataframe().is_some();
        let middle_ready = shell.model_feature_plan_knowledge().is_some();

        let semantic_feature_kinds = semantic_feature_kinds(shell.program());
        let semantic_feature_count = semantic_feature_kinds.len();
        let has_concept = semantic_feature_kinds.iter().any(|kind| kind == "concept");
        let has_principle = semantic_feature_kinds
            .iter()
            .any(|kind| kind == "principle");

        let logic_frame_ready = middle_ready && semantic_feature_count > 0 && has_concept;
        let pureform_return_ready = logic_frame_ready && shell.program().is_some();

        let mut capabilities = Vec::new();
        if semantic_feature_count > 0 {
            capabilities.push(ShellSemanticCapability::FormIngestion);
            capabilities.push(ShellSemanticCapability::LogicalFormParsing);
        }
        if middle_ready {
            capabilities.push(ShellSemanticCapability::CorpusLanguagePairing);
            capabilities.push(ShellSemanticCapability::LanguageModelFitting);
        }
        if has_principle && has_concept {
            capabilities.push(ShellSemanticCapability::PrincipleGatedConceptReturn);
        }

        let mut evolution_path = Vec::new();
        if frame_ready {
            evolution_path.push("frame.ready".to_string());
        }
        if middle_ready {
            evolution_path.push("middle.model-feature-plan.ready".to_string());
        }
        if logic_frame_ready {
            evolution_path.push("logic_frame.ready".to_string());
        }
        if pureform_return_ready {
            evolution_path.push("pureform.return.ready".to_string());
        }

        Self {
            address: shell.address(),
            frame_ready,
            middle_ready,
            logic_frame_ready,
            pureform_return_ready,
            semantic_feature_count,
            semantic_feature_kinds,
            capabilities,
            evolution_path,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn frame_ready(&self) -> bool {
        self.frame_ready
    }

    pub fn middle_ready(&self) -> bool {
        self.middle_ready
    }

    pub fn logic_frame_ready(&self) -> bool {
        self.logic_frame_ready
    }

    pub fn pureform_return_ready(&self) -> bool {
        self.pureform_return_ready
    }

    pub fn semantic_feature_count(&self) -> usize {
        self.semantic_feature_count
    }

    pub fn semantic_feature_kinds(&self) -> &[String] {
        &self.semantic_feature_kinds
    }

    pub fn capabilities(&self) -> &[ShellSemanticCapability] {
        &self.capabilities
    }

    pub fn evolution_path(&self) -> &[String] {
        &self.evolution_path
    }
}

/// Machine-checkable validation report for the canonical Shell projection trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellProjectionTraceValidation {
    required_trace: Vec<String>,
    observed_trace: Vec<String>,
    missing_steps: Vec<String>,
    valid: bool,
}

impl ShellProjectionTraceValidation {
    pub fn required_trace(&self) -> &[String] {
        &self.required_trace
    }

    pub fn observed_trace(&self) -> &[String] {
        &self.observed_trace
    }

    pub fn missing_steps(&self) -> &[String] {
        &self.missing_steps
    }

    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Personal learning report for Dataset -> LogicFrame progression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellLearningReport {
    address: ShellAddress,
    dataset_name: Option<String>,
    feature_count: usize,
    gained_principles: Vec<String>,
    gained_concepts: Vec<String>,
    gained_procedures: Vec<String>,
    logical_feature_kinds: Vec<String>,
    unresolved_forms: Vec<String>,
    fundamental_nlp_graph_ready: bool,
    mathematical_logic_ready: bool,
    kg_ready: bool,
    readiness_score: u8,
}

impl ShellLearningReport {
    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn dataset_name(&self) -> Option<&str> {
        self.dataset_name.as_deref()
    }

    pub fn feature_count(&self) -> usize {
        self.feature_count
    }

    pub fn gained_principles(&self) -> &[String] {
        &self.gained_principles
    }

    pub fn gained_concepts(&self) -> &[String] {
        &self.gained_concepts
    }

    pub fn gained_procedures(&self) -> &[String] {
        &self.gained_procedures
    }

    pub fn logical_feature_kinds(&self) -> &[String] {
        &self.logical_feature_kinds
    }

    pub fn unresolved_forms(&self) -> &[String] {
        &self.unresolved_forms
    }

    pub fn fundamental_nlp_graph_ready(&self) -> bool {
        self.fundamental_nlp_graph_ready
    }

    pub fn mathematical_logic_ready(&self) -> bool {
        self.mathematical_logic_ready
    }

    pub fn kg_ready(&self) -> bool {
        self.kg_ready
    }

    pub fn readiness_score(&self) -> u8 {
        self.readiness_score
    }
}

/// Runtime report for an explicitly materialized Corpus -> LogicFrame path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCorpusReport {
    materialized: bool,
    document_count: usize,
    lm_order: usize,
    lm_vocab_size: usize,
    logic_form_count: usize,
    parsed_form_count: usize,
}

impl ShellCorpusReport {
    fn from_logic_frame(logic_frame: &LogicFrame<MLE>) -> Self {
        Self {
            materialized: true,
            document_count: logic_frame.corpus().document_count(),
            lm_order: logic_frame.lm().order(),
            lm_vocab_size: logic_frame.lm().vocab().len(),
            logic_form_count: logic_frame.forms().len(),
            parsed_form_count: logic_frame
                .forms()
                .iter()
                .filter(|form| form.parsed())
                .count(),
        }
    }

    pub fn materialized(&self) -> bool {
        self.materialized
    }

    pub fn document_count(&self) -> usize {
        self.document_count
    }

    pub fn lm_order(&self) -> usize {
        self.lm_order
    }

    pub fn lm_vocab_size(&self) -> usize {
        self.lm_vocab_size
    }

    pub fn logic_form_count(&self) -> usize {
        self.logic_form_count
    }

    pub fn parsed_form_count(&self) -> usize {
        self.parsed_form_count
    }
}

/// Errors raised while explicitly materializing Corpus -> LogicFrame in Shell.
#[derive(Debug)]
pub enum ShellCorpusError {
    Corpus(CorpusError),
    Sem(LogicError),
}

impl From<CorpusError> for ShellCorpusError {
    fn from(error: CorpusError) -> Self {
        Self::Corpus(error)
    }
}

impl From<LogicError> for ShellCorpusError {
    fn from(error: LogicError) -> Self {
        Self::Sem(error)
    }
}

impl fmt::Display for ShellCorpusError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Corpus(error) => write!(formatter, "corpus materialization error: {error}"),
            Self::Sem(error) => write!(formatter, "semantic materialization error: {error}"),
        }
    }
}

impl std::error::Error for ShellCorpusError {}

/// Memory-estimation report for a Shell pipeline instance.
#[derive(Debug, Clone)]
pub struct ShellMemoryEstimate {
    concurrency: usize,
    node_count: usize,
    relationship_count: usize,
    memory_range: MemoryRange,
    memory_tree: MemoryTree,
}

impl ShellMemoryEstimate {
    fn new(
        concurrency: usize,
        node_count: usize,
        relationship_count: usize,
        memory_range: MemoryRange,
        memory_tree: MemoryTree,
    ) -> Self {
        Self {
            concurrency,
            node_count,
            relationship_count,
            memory_range,
            memory_tree,
        }
    }

    pub fn concurrency(&self) -> usize {
        self.concurrency
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn relationship_count(&self) -> usize {
        self.relationship_count
    }

    pub fn memory_range(&self) -> &MemoryRange {
        &self.memory_range
    }

    pub fn memory_tree(&self) -> &MemoryTree {
        &self.memory_tree
    }

    pub fn render_tree(&self) -> String {
        self.memory_tree.render()
    }
}

/// Capability bands that let Shell read the whole platform without executing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellCapabilityBand {
    Immediate,
    Mediation,
    Recursive,
}

impl ShellCapabilityBand {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Mediation => "mediation",
            Self::Recursive => "recursive",
        }
    }
}

/// Platform capabilities visible to the Shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellPlatformCapability {
    FrameRegister,
    DataPipeline,
    PureFormReturn,
    ModelFeaturePlan,
    ProgressTracking,
    MemoryEstimation,
    ConcurrencyRuntime,
    PregelRuntime,
    DefaultGraphStore,
    CorpusMaterialization,
    LogicFrameLearning,
    MathematicalLogicReadiness,
}

impl ShellPlatformCapability {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FrameRegister => "frame-register",
            Self::DataPipeline => "data-pipeline",
            Self::PureFormReturn => "pureform-return",
            Self::ModelFeaturePlan => "model-feature-plan",
            Self::ProgressTracking => "progress-tracking",
            Self::MemoryEstimation => "memory-estimation",
            Self::ConcurrencyRuntime => "concurrency-runtime",
            Self::PregelRuntime => "pregel-runtime",
            Self::DefaultGraphStore => "default-graph-store",
            Self::CorpusMaterialization => "corpus-materialization",
            Self::LogicFrameLearning => "logic_frame-learning",
            Self::MathematicalLogicReadiness => "mathematical-logic-readiness",
        }
    }
}

/// One Shell-visible capability with availability and activation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCapabilityState {
    band: ShellCapabilityBand,
    capability: ShellPlatformCapability,
    available: bool,
    active: bool,
}

impl ShellCapabilityState {
    fn new(
        band: ShellCapabilityBand,
        capability: ShellPlatformCapability,
        available: bool,
        active: bool,
    ) -> Self {
        Self {
            band,
            capability,
            available,
            active,
        }
    }

    pub fn band(&self) -> ShellCapabilityBand {
        self.band
    }

    pub fn capability(&self) -> ShellPlatformCapability {
        self.capability
    }

    pub fn available(&self) -> bool {
        self.available
    }

    pub fn active(&self) -> bool {
        self.active
    }
}

/// Read-only map of what the Shell can see and what this instance has activated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellCapabilityMap {
    address: ShellAddress,
    states: Vec<ShellCapabilityState>,
}

/// Shell-readable report for the Model moment of Model:Feature::Plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellModelMomentKnowledge {
    address: ShellAddress,
    doctrine_resource: &'static str,
    moment: ShellMomentKind,
    fold: ShellFold,
    algebra: ShellAlgebra,
    dataset_name: Option<String>,
    model_artifact_kind: Option<String>,
    program_name: Option<String>,
    selected_forms: Vec<String>,
    model_vocab_entries: usize,
    model_topics: Vec<String>,
    descends_to_example: &'static str,
    ai_assistance_presupposed: bool,
}

impl ShellModelMomentKnowledge {
    fn from_shell(shell: &GdsShell) -> Self {
        let help = ShellHelp::new();
        let moment = ShellMomentKind::Model;
        let middle = shell.model_feature_plan_knowledge();

        let model_topics = help
            .for_moment(moment)
            .into_iter()
            .map(|entry| entry.name.to_string())
            .collect();

        Self {
            address: shell.address(),
            doctrine_resource: "shell::help::model-feature-plan/model",
            moment,
            fold: ShellFold::ModelFeaturePlan,
            algebra: shell.algebra(),
            dataset_name: middle
                .as_ref()
                .and_then(|knowledge| knowledge.model_name().map(ToOwned::to_owned)),
            model_artifact_kind: middle
                .as_ref()
                .and_then(|knowledge| knowledge.model_artifact_kind().map(ToOwned::to_owned)),
            program_name: middle
                .as_ref()
                .and_then(|knowledge| knowledge.program_name().map(ToOwned::to_owned)),
            selected_forms: middle
                .as_ref()
                .map(|knowledge| knowledge.selected_forms().to_vec())
                .unwrap_or_default(),
            model_vocab_entries: help.for_moment(moment).len(),
            model_topics,
            descends_to_example: "gds/examples/dataset_model_feature_plan.rs",
            ai_assistance_presupposed: true,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn doctrine_resource(&self) -> &'static str {
        self.doctrine_resource
    }

    pub fn moment(&self) -> ShellMomentKind {
        self.moment
    }

    pub fn fold(&self) -> ShellFold {
        self.fold
    }

    pub fn algebra(&self) -> ShellAlgebra {
        self.algebra
    }

    pub fn dataset_name(&self) -> Option<&str> {
        self.dataset_name.as_deref()
    }

    pub fn model_artifact_kind(&self) -> Option<&str> {
        self.model_artifact_kind.as_deref()
    }

    pub fn program_name(&self) -> Option<&str> {
        self.program_name.as_deref()
    }

    pub fn selected_forms(&self) -> &[String] {
        &self.selected_forms
    }

    pub fn model_vocab_entries(&self) -> usize {
        self.model_vocab_entries
    }

    pub fn model_topics(&self) -> &[String] {
        &self.model_topics
    }

    pub fn descends_to_example(&self) -> &'static str {
        self.descends_to_example
    }

    pub fn ai_assistance_presupposed(&self) -> bool {
        self.ai_assistance_presupposed
    }
}

/// Shell-readable report for the Feature moment of Model:Feature::Plan.
///
/// Feature is the companion of Model: Model establishes essential commitments,
/// Feature supplies the structured predicates those commitments act upon.
///
/// This is also where FeatStruct operations live. It is the primary
/// "dragon zone" of structural unification and subsumption.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellFeatureMomentKnowledge {
    address: ShellAddress,
    doctrine_resource: &'static str,
    moment: ShellMomentKind,
    companion_moment: ShellMomentKind,
    fold: ShellFold,
    algebra: ShellAlgebra,
    feature_vocab_entries: usize,
    feature_topics: Vec<String>,
    program_feature_law: Vec<&'static str>,
    featstruct_primitives: Vec<&'static str>,
    dragon_zone_note: &'static str,
    descends_to_example: &'static str,
    ai_assistance_presupposed: bool,
}

impl ShellFeatureMomentKnowledge {
    fn from_shell(shell: &GdsShell) -> Self {
        let help = ShellHelp::new();
        let moment = ShellMomentKind::Feature;
        let feature_topics = help
            .for_moment(moment)
            .into_iter()
            .map(|entry| entry.name.to_string())
            .collect();

        Self {
            address: shell.address(),
            doctrine_resource: "shell::help::model-feature-plan/feature",
            moment,
            companion_moment: ShellMomentKind::Model,
            fold: ShellFold::ModelFeaturePlan,
            algebra: shell.algebra(),
            feature_vocab_entries: help.for_moment(moment).len(),
            feature_topics,
            program_feature_law: vec![
                "Source",
                "Observation",
                "Reflection",
                "Logogenesis",
                "Principle",
                "Condition",
                "Concept",
                "Judgment",
                "Syllogism",
                "Inference",
                "Query",
                "Procedure",
            ],
            featstruct_primitives: vec![
                "parse_featstruct",
                "parse_featvalue",
                "format_featstruct",
                "unify_featstruct",
                "subsumes_featstruct",
                "cyclic_featstruct",
            ],
            dragon_zone_note:
                "FeatStruct is the structural magic layer: unification, subsumption, reentrance, and contradiction edges live here.",
            descends_to_example: "gds/examples/dataset_model_feature_plan.rs",
            ai_assistance_presupposed: true,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn doctrine_resource(&self) -> &'static str {
        self.doctrine_resource
    }

    pub fn moment(&self) -> ShellMomentKind {
        self.moment
    }

    pub fn companion_moment(&self) -> ShellMomentKind {
        self.companion_moment
    }

    pub fn fold(&self) -> ShellFold {
        self.fold
    }

    pub fn algebra(&self) -> ShellAlgebra {
        self.algebra
    }

    pub fn feature_vocab_entries(&self) -> usize {
        self.feature_vocab_entries
    }

    pub fn feature_topics(&self) -> &[String] {
        &self.feature_topics
    }

    pub fn program_feature_law(&self) -> &[&'static str] {
        &self.program_feature_law
    }

    pub fn featstruct_primitives(&self) -> &[&'static str] {
        &self.featstruct_primitives
    }

    pub fn dragon_zone_note(&self) -> &'static str {
        self.dragon_zone_note
    }

    pub fn descends_to_example(&self) -> &'static str {
        self.descends_to_example
    }

    pub fn ai_assistance_presupposed(&self) -> bool {
        self.ai_assistance_presupposed
    }
}

/// Shell-readable report for the Plan moment of Model:Feature::Plan.
///
/// Plan is where the Principle/Concept content becomes executable ordering:
/// pipeline declarations, operation steps, and runtime readiness surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPlanMomentKnowledge {
    address: ShellAddress,
    doctrine_resource: &'static str,
    moment: ShellMomentKind,
    fold: ShellFold,
    algebra: ShellAlgebra,
    plan_vocab_entries: usize,
    plan_topics: Vec<String>,
    plan_runtime_surfaces: Vec<&'static str>,
    pipeline_axis: ShellPipeline,
    has_dataset_pipeline: bool,
    has_metapipeline: bool,
    descends_to_example: &'static str,
    ai_assistance_presupposed: bool,
}

impl ShellPlanMomentKnowledge {
    fn from_shell(shell: &GdsShell) -> Self {
        let help = ShellHelp::new();
        let moment = ShellMomentKind::Plan;
        let plan_topics = help
            .for_moment(moment)
            .into_iter()
            .map(|entry| entry.name.to_string())
            .collect();

        Self {
            address: shell.address(),
            doctrine_resource: "shell::help::model-feature-plan/plan",
            moment,
            fold: ShellFold::ModelFeaturePlan,
            algebra: shell.algebra(),
            plan_vocab_entries: help.for_moment(moment).len(),
            plan_topics,
            plan_runtime_surfaces: vec![
                "pipeline_progress_tracker",
                "estimate_pipeline_memory",
                "capability_map",
            ],
            pipeline_axis: shell.pipeline_axis(),
            has_dataset_pipeline: shell.dataset_pipeline().is_some(),
            has_metapipeline: shell.descriptor().has_metapipeline(),
            descends_to_example: "gds/examples/dataset_model_feature_plan.rs",
            ai_assistance_presupposed: true,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn doctrine_resource(&self) -> &'static str {
        self.doctrine_resource
    }

    pub fn moment(&self) -> ShellMomentKind {
        self.moment
    }

    pub fn fold(&self) -> ShellFold {
        self.fold
    }

    pub fn algebra(&self) -> ShellAlgebra {
        self.algebra
    }

    pub fn plan_vocab_entries(&self) -> usize {
        self.plan_vocab_entries
    }

    pub fn plan_topics(&self) -> &[String] {
        &self.plan_topics
    }

    pub fn plan_runtime_surfaces(&self) -> &[&'static str] {
        &self.plan_runtime_surfaces
    }

    pub fn pipeline_axis(&self) -> ShellPipeline {
        self.pipeline_axis
    }

    pub fn has_dataset_pipeline(&self) -> bool {
        self.has_dataset_pipeline
    }

    pub fn has_metapipeline(&self) -> bool {
        self.has_metapipeline
    }

    pub fn descends_to_example(&self) -> &'static str {
        self.descends_to_example
    }

    pub fn ai_assistance_presupposed(&self) -> bool {
        self.ai_assistance_presupposed
    }
}

/// One moment of the three-fold Model genesis (Preparation / Execution / Image).
///
/// These map to the three primitive operations the Dataset substrate owes the kernel:
/// unify, subsumes, extend — reflected as Box 1 (Identity), Box 2 (Difference),
/// Box 3 (Ground) in Essence logic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellModelGenesisMoment {
    pub box_number: u8,
    pub name: &'static str,
    /// Essence-logic role: "Identity", "Difference", or "Ground".
    pub essence_role: &'static str,
    pub primary_input: &'static str,
    pub primary_output: &'static str,
    /// Entry function from `dataset::model`.
    pub entry_fn: &'static str,
    pub doctrine_note: &'static str,
}

/// Shell-readable account of the three-fold genesis of a Model.
///
/// The Model moment of Model:Feature::Plan is internally triadic:
///
/// ```text
/// Box 1 — Preparation (Identity)
///   ModelSpec + feature marks → ModelEssence + PreparationReport
///   prepare_model stamps each feature with Modality.
///
/// Box 2 — Execution (Difference)
///   ModelEssence + MarkedFeatures → Execution (LazyFrame Expr)
///   execute_essence lowers features into Polars expressions.
///
/// Box 3 — Image (Ground)
///   Execution results → OntologyDataFrameImage + Provenance
///   realize_from_essence returns the model to itself as concrete artifact.
/// ```
///
/// `program_feature_law` names the compulsory ProgramFeatureKind arc — the
/// grammar of any scientific language that runs through the Shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellModelGenesisKnowledge {
    address: ShellAddress,
    fold: ShellFold,
    moment: ShellMomentKind,
    doctrine_resource: &'static str,
    /// The compulsory arc: every scientific language program must declare its
    /// feature kinds in this sequence to be evaluable by the Shell.
    program_feature_law: Vec<&'static str>,
    genesis_moments: Vec<ShellModelGenesisMoment>,
    descends_to_example: &'static str,
    ai_assistance_presupposed: bool,
}

impl ShellModelGenesisKnowledge {
    fn new(shell: &GdsShell) -> Self {
        Self {
            address: shell.address(),
            fold: ShellFold::ModelFeaturePlan,
            moment: ShellMomentKind::Model,
            doctrine_resource: "shell::help::model-genesis/prep-exec-image",
            program_feature_law: vec![
                "Source",
                "Observation",
                "Reflection",
                "Logogenesis",
                "Principle",
                "Condition",
                "Concept",
                "Judgment",
                "Syllogism",
                "Inference",
                "Query",
                "Procedure",
            ],
            genesis_moments: vec![
                ShellModelGenesisMoment {
                    box_number: 1,
                    name: "Preparation",
                    essence_role: "Identity",
                    primary_input: "ModelSpec + feature marks",
                    primary_output: "ModelEssence + PreparationReport",
                    entry_fn: "prepare_model",
                    doctrine_note: "Model establishes its essential relations; each feature receives a Modality (Necessary / Contingent / Possible / Impossible / Unknown). Contradictions are detected here — not resolved.",
                },
                ShellModelGenesisMoment {
                    box_number: 2,
                    name: "Execution",
                    essence_role: "Difference",
                    primary_input: "ModelEssence + MarkedFeatures",
                    primary_output: "Execution (LazyFrame Expr)",
                    entry_fn: "execute_essence",
                    doctrine_note: "Model goes out into the field of data; features become executable Polars expressions. Impossible features are skipped; their contradiction is deferred to Box 3.",
                },
                ShellModelGenesisMoment {
                    box_number: 3,
                    name: "Image",
                    essence_role: "Ground",
                    primary_input: "Execution results",
                    primary_output: "OntologyDataFrameImage + Provenance",
                    entry_fn: "realize_from_essence",
                    doctrine_note: "Model returns to itself as a concrete realized artifact. Provenance records what was Necessary, Contingent, or Impossible — the model's full self-knowledge.",
                },
            ],
            descends_to_example: "gds/examples/dataset_model_feature_plan.rs",
            ai_assistance_presupposed: true,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn fold(&self) -> ShellFold {
        self.fold
    }

    pub fn moment(&self) -> ShellMomentKind {
        self.moment
    }

    pub fn doctrine_resource(&self) -> &'static str {
        self.doctrine_resource
    }

    pub fn program_feature_law(&self) -> &[&'static str] {
        &self.program_feature_law
    }

    pub fn genesis_moments(&self) -> &[ShellModelGenesisMoment] {
        &self.genesis_moments
    }

    pub fn descends_to_example(&self) -> &'static str {
        self.descends_to_example
    }

    pub fn ai_assistance_presupposed(&self) -> bool {
        self.ai_assistance_presupposed
    }
}

impl ShellCapabilityMap {
    fn from_shell(shell: &GdsShell) -> Self {
        let descriptor = shell.descriptor();
        let semantic = shell.semantic_pipeline_knowledge();
        let learning = shell.learning_report();
        let projection_valid = shell.is_projection_trace_valid();
        let logic_frame_materialized = shell.logic_frame().is_some();

        let states = vec![
            ShellCapabilityState::new(
                ShellCapabilityBand::Immediate,
                ShellPlatformCapability::FrameRegister,
                true,
                shell.dataframe().is_some(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Immediate,
                ShellPlatformCapability::DataPipeline,
                true,
                shell.dataset_pipeline().is_some(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Immediate,
                ShellPlatformCapability::PureFormReturn,
                true,
                descriptor.has_metapipeline(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Mediation,
                ShellPlatformCapability::ModelFeaturePlan,
                true,
                shell.model_feature_plan_knowledge().is_some(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Mediation,
                ShellPlatformCapability::ProgressTracking,
                true,
                shell.dataset_pipeline().is_some(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Mediation,
                ShellPlatformCapability::MemoryEstimation,
                true,
                shell.dataset_pipeline().is_some(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Mediation,
                ShellPlatformCapability::ConcurrencyRuntime,
                true,
                projection_valid,
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Recursive,
                ShellPlatformCapability::PregelRuntime,
                true,
                logic_frame_materialized,
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Recursive,
                ShellPlatformCapability::DefaultGraphStore,
                true,
                logic_frame_materialized,
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Recursive,
                ShellPlatformCapability::CorpusMaterialization,
                true,
                logic_frame_materialized,
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Recursive,
                ShellPlatformCapability::LogicFrameLearning,
                true,
                semantic.logic_frame_ready(),
            ),
            ShellCapabilityState::new(
                ShellCapabilityBand::Recursive,
                ShellPlatformCapability::MathematicalLogicReadiness,
                true,
                learning.mathematical_logic_ready(),
            ),
        ];

        Self {
            address: shell.address(),
            states,
        }
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn states(&self) -> &[ShellCapabilityState] {
        &self.states
    }

    pub fn band(&self, band: ShellCapabilityBand) -> Vec<&ShellCapabilityState> {
        self.states
            .iter()
            .filter(|state| state.band() == band)
            .collect()
    }

    pub fn active_capabilities(&self) -> Vec<ShellPlatformCapability> {
        self.states
            .iter()
            .filter(|state| state.active())
            .map(ShellCapabilityState::capability)
            .collect()
    }

    pub fn has_active(&self, capability: ShellPlatformCapability) -> bool {
        self.states
            .iter()
            .any(|state| state.capability() == capability && state.active())
    }
}

fn features_by_kind(program: Option<&ShellProgram>, kind: ProgramFeatureKind) -> Vec<String> {
    program
        .map(|program| {
            program
                .features()
                .features
                .iter()
                .filter(|feature| feature.kind == kind)
                .map(|feature| feature.value.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn semantic_feature_kinds(program: Option<&ShellProgram>) -> Vec<String> {
    let Some(program) = program else {
        return Vec::new();
    };

    let mut kinds: Vec<String> = Vec::new();
    for feature in &program.features().features {
        if is_semantic_feature_kind(feature.kind) {
            let kind = feature.kind.as_str().to_string();
            if !kinds.contains(&kind) {
                kinds.push(kind);
            }
        }
    }

    kinds
}

fn is_semantic_feature_kind(kind: ProgramFeatureKind) -> bool {
    matches!(
        kind,
        ProgramFeatureKind::Source
            | ProgramFeatureKind::Observation
            | ProgramFeatureKind::Reflection
            | ProgramFeatureKind::Logogenesis
            | ProgramFeatureKind::Subfeature
            | ProgramFeatureKind::Principle
            | ProgramFeatureKind::Condition
            | ProgramFeatureKind::Mark
            | ProgramFeatureKind::Concept
            | ProgramFeatureKind::Judgment
            | ProgramFeatureKind::Syllogism
            | ProgramFeatureKind::Inference
            | ProgramFeatureKind::Query
            | ProgramFeatureKind::Procedure
    )
}

/// First concrete shell object over the immediate and mediated registers.
#[derive(Debug, Clone, Default)]
pub struct GdsShell {
    dataframe: Option<GDSDataFrame>,
    dataset: Option<Dataset>,
    program: Option<ShellProgram>,
    seed: Option<ShellSeed>,
    logic_frame: Option<LogicFrame<MLE>>,
}

impl GdsShell {
    const CANONICAL_PROJECTION_TRACE: [&'static str; 4] = [
        "Frame",
        "Model:Feature::Plan",
        "LogicFrame",
        "PureForm return",
    ];

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_dataframe(dataframe: GDSDataFrame) -> Self {
        let seed = ShellSeed::from_dataframe(&dataframe);
        Self {
            dataframe: Some(dataframe),
            dataset: None,
            program: None,
            seed: Some(seed),
            logic_frame: None,
        }
    }

    pub fn from_dataset(dataset: Dataset) -> Self {
        let dataframe = dataset.table().clone();
        let seed = ShellSeed::from_dataframe(&dataframe);
        Self {
            dataframe: Some(dataframe),
            dataset: Some(dataset),
            program: None,
            seed: Some(seed),
            logic_frame: None,
        }
    }

    pub fn with_program_features(mut self, program_features: ProgramFeatures) -> Self {
        let mut program = ShellProgram::new(program_features);
        if let Some(seed) = &self.seed {
            program = program.with_schema(seed.schema().clone());
        }
        self.program = Some(program);
        self.logic_frame = None;
        self
    }

    pub fn with_program(mut self, program: ShellProgram) -> Self {
        let program = if program.schema().is_none() {
            if let Some(seed) = &self.seed {
                program.with_schema(seed.schema().clone())
            } else {
                program
            }
        } else {
            program
        };
        self.program = Some(program);
        self.logic_frame = None;
        self
    }

    pub fn dataframe(&self) -> Option<&GDSDataFrame> {
        self.dataframe.as_ref()
    }

    pub fn dataset(&self) -> Option<&Dataset> {
        self.dataset.as_ref()
    }

    pub fn program_features(&self) -> Option<&ProgramFeatures> {
        self.program.as_ref().map(ShellProgram::features)
    }

    pub fn program(&self) -> Option<&ShellProgram> {
        self.program.as_ref()
    }

    pub fn seed(&self) -> Option<&ShellSeed> {
        self.seed.as_ref()
    }

    pub fn logic_frame(&self) -> Option<&LogicFrame<MLE>> {
        self.logic_frame.as_ref()
    }

    pub fn materialize_logic_frame_from_texts<T>(
        mut self,
        texts: &[T],
    ) -> Result<Self, ShellCorpusError>
    where
        T: AsRef<str>,
    {
        let corpus = Corpus::from_texts(texts)?;
        let mut logic_frame = LogicFrame::fit(corpus, MLE::new(2), &WhitespaceTokenizer)?;
        if let Some(program) = &self.program {
            logic_frame.ingest_forms(program.features().features.clone());
            logic_frame.parse_forms();
        }
        self.logic_frame = Some(logic_frame);
        Ok(self)
    }

    pub fn corpus_report(&self) -> Option<ShellCorpusReport> {
        self.logic_frame
            .as_ref()
            .map(ShellCorpusReport::from_logic_frame)
    }

    pub fn capability_map(&self) -> ShellCapabilityMap {
        ShellCapabilityMap::from_shell(self)
    }

    /// Return the first Shell-first report for Model:Feature::Plan: the Model moment.
    pub fn model_moment_knowledge(&self) -> ShellModelMomentKnowledge {
        ShellModelMomentKnowledge::from_shell(self)
    }

    /// Build a short prompt for an AI-assisted pass over the Shell Model moment.
    pub fn model_help_prompt(&self) -> String {
        let knowledge = self.model_moment_knowledge();
        format!(
            "Shell Model moment\nresource: {}\naddress: {:?}\nfold: {}\nmoment: {}\nalgebra: {:?}\ndataset: {:?}\nprogram: {:?}\nselected forms: {:?}\nmodel vocabulary entries: {}\nmodel topics: {}\ndescends to: {}\nai assistance presupposed: {}",
            knowledge.doctrine_resource(),
            knowledge.address(),
            knowledge.fold(),
            knowledge.moment(),
            knowledge.algebra(),
            knowledge.dataset_name(),
            knowledge.program_name(),
            knowledge.selected_forms(),
            knowledge.model_vocab_entries(),
            knowledge.model_topics().join(", "),
            knowledge.descends_to_example(),
            knowledge.ai_assistance_presupposed(),
        )
    }

    /// Return the Shell-first report for the Feature companion moment.
    pub fn feature_moment_knowledge(&self) -> ShellFeatureMomentKnowledge {
        ShellFeatureMomentKnowledge::from_shell(self)
    }

    /// Build a short prompt for an AI-assisted pass over the Shell Feature moment.
    pub fn feature_help_prompt(&self) -> String {
        let knowledge = self.feature_moment_knowledge();
        format!(
            "Shell Feature moment\nresource: {}\naddress: {:?}\nfold: {}\nmoment: {}\ncompanion moment: {}\nalgebra: {:?}\nfeature vocabulary entries: {}\nfeature topics: {}\nprogram feature law: {}\nfeatstruct primitives: {}\ndragon note: {}\ndescends to: {}\nai assistance presupposed: {}",
            knowledge.doctrine_resource(),
            knowledge.address(),
            knowledge.fold(),
            knowledge.moment(),
            knowledge.companion_moment(),
            knowledge.algebra(),
            knowledge.feature_vocab_entries(),
            knowledge.feature_topics().join(", "),
            knowledge.program_feature_law().join(" -> "),
            knowledge.featstruct_primitives().join(", "),
            knowledge.dragon_zone_note(),
            knowledge.descends_to_example(),
            knowledge.ai_assistance_presupposed(),
        )
    }

    /// Return the Shell-first report for the Plan moment.
    pub fn plan_moment_knowledge(&self) -> ShellPlanMomentKnowledge {
        ShellPlanMomentKnowledge::from_shell(self)
    }

    /// Build a short prompt for an AI-assisted pass over the Shell Plan moment.
    pub fn plan_help_prompt(&self) -> String {
        let knowledge = self.plan_moment_knowledge();
        format!(
            "Shell Plan moment\nresource: {}\naddress: {:?}\nfold: {}\nmoment: {}\nalgebra: {:?}\nplan vocabulary entries: {}\nplan topics: {}\nruntime surfaces: {}\npipeline axis: {:?}\ndataset pipeline active: {}\nmetapipeline active: {}\ndescends to: {}\nai assistance presupposed: {}",
            knowledge.doctrine_resource(),
            knowledge.address(),
            knowledge.fold(),
            knowledge.moment(),
            knowledge.algebra(),
            knowledge.plan_vocab_entries(),
            knowledge.plan_topics().join(", "),
            knowledge.plan_runtime_surfaces().join(", "),
            knowledge.pipeline_axis(),
            knowledge.has_dataset_pipeline(),
            knowledge.has_metapipeline(),
            knowledge.descends_to_example(),
            knowledge.ai_assistance_presupposed(),
        )
    }

    /// Build a request-local progress tracker for this shell pipeline.
    /// Return the three-fold genesis account of the Model moment.
    ///
    /// Box 1 (Preparation / Identity) → Box 2 (Execution / Difference)
    /// → Box 3 (Image / Ground). These are the three primitive ops that
    /// constitute a Model coming into being in the Dataset substrate.
    pub fn model_genesis_knowledge(&self) -> ShellModelGenesisKnowledge {
        ShellModelGenesisKnowledge::new(self)
    }

    /// Build a short prompt for an AI-assisted pass over the Model genesis arc.
    pub fn model_genesis_prompt(&self) -> String {
        let g = self.model_genesis_knowledge();
        let moments: Vec<String> = g
            .genesis_moments()
            .iter()
            .map(|m| {
                format!(
                    "  Box {} — {} ({}): {} → {}  [{}]",
                    m.box_number,
                    m.name,
                    m.essence_role,
                    m.primary_input,
                    m.primary_output,
                    m.entry_fn
                )
            })
            .collect();
        format!(
                "Shell Model genesis\nresource: {}\nfold: {}\nmoment: {:?}\nprogram feature law: {}\ngenesis:\n{}\ndescends to: {}\nai assistance presupposed: {}",
                g.doctrine_resource(),
                g.fold(),
                g.moment(),
                g.program_feature_law().join(" → "),
                moments.join("\n"),
                g.descends_to_example(),
                g.ai_assistance_presupposed(),
            )
    }

    /// Build a request-local progress tracker for this shell pipeline.
    pub fn pipeline_progress_tracker(&self) -> TaskProgressTracker {
        let description = format!("shell.pipeline::{:?}", self.pipeline());
        let leaf = Tasks::leaf_with_volume(description, self.estimated_pipeline_volume());
        TaskProgressTracker::new(leaf)
    }

    /// Estimate memory for this shell pipeline using kernel memory-estimation machinery.
    pub fn estimate_pipeline_memory(&self, concurrency: usize) -> ShellMemoryEstimate {
        let concurrency = concurrency.max(1);
        let dimensions = self.estimated_graph_dimensions();

        let per_node_bytes = self
            .seed
            .as_ref()
            .map(|seed| seed.column_count().max(1) * 64)
            .unwrap_or(128);
        let feature_bytes = self
            .program
            .as_ref()
            .map(|program| program.feature_count() * 96)
            .unwrap_or(0);
        let semantic_form_bytes = self
            .logic_frame
            .as_ref()
            .map(|logic_frame| logic_frame.forms().len() * 128)
            .unwrap_or(0);

        let estimation = MemoryEstimations::builder("shell.pipeline")
            .fixed("pipeline.overhead", 1024)
            .range_per_graph_dimension("frame.seed-buffer", move |dim, _| {
                let base = dim.node_count().saturating_mul(per_node_bytes);
                MemoryRange::of_range(base, base.saturating_mul(2))
            })
            .fixed("program.features", feature_bytes)
            .fixed("semantic.forms", semantic_form_bytes)
            .build();

        let memory_tree = estimation.estimate(&dimensions, concurrency);

        ShellMemoryEstimate::new(
            concurrency,
            dimensions.node_count,
            dimensions.relationship_count,
            *memory_tree.memory_usage(),
            memory_tree,
        )
    }

    pub fn dataframe_knowledge(&self) -> Option<ShellDataFrameKnowledge> {
        self.seed
            .as_ref()
            .map(|seed| ShellDataFrameKnowledge::from_shell(self, seed))
    }

    pub fn model_feature_plan_knowledge(&self) -> Option<ShellModelFeaturePlanKnowledge> {
        ShellModelFeaturePlanKnowledge::from_shell(self)
    }

    pub fn semantic_pipeline_knowledge(&self) -> ShellSemanticPipelineKnowledge {
        ShellSemanticPipelineKnowledge::from_shell(self)
    }

    pub fn validate_projection_trace(&self) -> ShellProjectionTraceValidation {
        let semantic = self.semantic_pipeline_knowledge();

        let mut observed_trace = Vec::new();
        if semantic.frame_ready() {
            observed_trace.push("Frame".to_string());
        }
        if semantic.middle_ready() {
            observed_trace.push("Model:Feature::Plan".to_string());
        }
        if semantic.logic_frame_ready() {
            observed_trace.push("LogicFrame".to_string());
        }
        if semantic.pureform_return_ready() {
            observed_trace.push("PureForm return".to_string());
        }

        let required_trace = Self::CANONICAL_PROJECTION_TRACE
            .iter()
            .map(|step| (*step).to_string())
            .collect::<Vec<_>>();

        let missing_steps = required_trace
            .iter()
            .filter(|step| !observed_trace.contains(step))
            .cloned()
            .collect::<Vec<_>>();

        ShellProjectionTraceValidation {
            required_trace,
            observed_trace,
            valid: missing_steps.is_empty(),
            missing_steps,
        }
    }

    pub fn is_projection_trace_valid(&self) -> bool {
        self.validate_projection_trace().is_valid()
    }

    pub fn learning_report(&self) -> ShellLearningReport {
        let semantic = self.semantic_pipeline_knowledge();
        let middle = self.model_feature_plan_knowledge();
        let principles = features_by_kind(self.program(), ProgramFeatureKind::Principle);
        let concepts = features_by_kind(self.program(), ProgramFeatureKind::Concept);
        let procedures = features_by_kind(self.program(), ProgramFeatureKind::Procedure);

        let required_forms = ["principle", "concept", "procedure"];
        let unresolved_forms = required_forms
            .iter()
            .filter_map(|required| {
                let present = semantic
                    .semantic_feature_kinds()
                    .iter()
                    .any(|kind| kind == required);
                if present {
                    None
                } else {
                    Some((*required).to_string())
                }
            })
            .collect::<Vec<_>>();

        let logical_feature_kinds = semantic
            .semantic_feature_kinds()
            .iter()
            .filter(|kind| {
                matches!(
                    kind.as_str(),
                    "condition" | "judgment" | "syllogism" | "inference" | "query"
                )
            })
            .cloned()
            .collect::<Vec<_>>();

        let fundamental_nlp_graph_ready = [
            ShellSemanticCapability::FormIngestion,
            ShellSemanticCapability::LogicalFormParsing,
            ShellSemanticCapability::CorpusLanguagePairing,
            ShellSemanticCapability::LanguageModelFitting,
        ]
        .iter()
        .all(|capability| semantic.capabilities().contains(capability));

        let mathematical_logic_ready = !logical_feature_kinds.is_empty();
        let kg_ready = semantic.logic_frame_ready() && unresolved_forms.is_empty();

        let readiness_score = [
            semantic.frame_ready(),
            semantic.middle_ready(),
            semantic.logic_frame_ready(),
            semantic.pureform_return_ready(),
        ]
        .into_iter()
        .filter(|ready| *ready)
        .count() as u8
            * 25;

        ShellLearningReport {
            address: semantic.address(),
            dataset_name: middle
                .and_then(|knowledge| knowledge.model_name().map(ToOwned::to_owned)),
            feature_count: semantic.semantic_feature_count(),
            gained_principles: principles,
            gained_concepts: concepts,
            gained_procedures: procedures,
            logical_feature_kinds,
            unresolved_forms,
            fundamental_nlp_graph_ready,
            mathematical_logic_ready,
            kg_ready,
            readiness_score,
        }
    }

    pub fn moment(&self) -> ShellMoment {
        if self.program.is_some() {
            ShellMoment::ModelFeaturePlan
        } else {
            ShellMoment::Frame
        }
    }

    pub fn register(&self) -> ShellRegister {
        match (&self.dataframe, &self.dataset) {
            (Some(_), Some(_)) => ShellRegister::Unified,
            (Some(_), None) => ShellRegister::ImmediateDataFrame,
            (None, Some(_)) => ShellRegister::MediatedDataset,
            (None, None) => ShellRegister::ImmediateDataFrame,
        }
    }

    pub fn register_kind(&self) -> ShellRegister {
        self.register()
    }

    pub fn address(&self) -> ShellAddress {
        ShellAddress::new(self.register(), self.pipeline(), self.algebra())
    }

    pub fn pipeline_axis(&self) -> ShellPipeline {
        self.pipeline()
    }

    pub fn pipeline(&self) -> ShellPipeline {
        self.moment().into()
    }

    pub fn algebra(&self) -> ShellAlgebra {
        if self.program.is_some() {
            ShellAlgebra::ProgramFeature
        } else if self
            .seed
            .as_ref()
            .is_some_and(|seed| !seed.columns().is_empty())
        {
            ShellAlgebra::FeatStruct
        } else {
            ShellAlgebra::Schema
        }
    }

    pub fn dataset_frame(&self) -> Option<DatasetDataFrameNs> {
        if let Some(dataset) = &self.dataset {
            return Some(dataset.frame());
        }

        self.dataframe
            .as_ref()
            .cloned()
            .map(DatasetDataFrameNs::new)
    }

    pub fn dataset_pipeline(&self) -> Option<DatasetPipeline> {
        self.dataset_frame().map(|ds_frame| ds_frame.pipeline())
    }

    pub fn descriptor(&self) -> ShellPipelineDescriptor {
        let mut descriptor = ShellPipelineDescriptor::new(self.address());
        if let Some(seed) = &self.seed {
            descriptor = descriptor.with_schema(seed.schema().clone());
        }
        if let Some(program) = &self.program {
            descriptor = descriptor.with_program(program.clone());
        }
        if self.dataframe.is_some() {
            descriptor = descriptor.with_immediate_body();
        }
        if self.dataset.is_some() {
            descriptor = descriptor.with_mediated_body();
        }
        descriptor
    }

    pub fn pipeline_facade(&self) -> ShellPipelineFacade {
        ShellPipelineFacade::from_shell(self)
    }

    fn concept_return_plan_steps(&self) -> Vec<String> {
        let mut steps = Vec::new();
        if self.dataframe.is_some() {
            steps.push("dataframe.seed".to_string());
        }
        if self.dataset.is_some() {
            steps.push("dataset.model".to_string());
            steps.push("dataset.feature".to_string());
            steps.push("dataset.plan".to_string());
        }
        if self.program.is_some() {
            steps.push("shell.program".to_string());
            steps.push("shell.concept-return".to_string());
        }
        steps
    }

    pub fn into_dataset(self) -> Option<Dataset> {
        if let Some(dataset) = self.dataset {
            Some(dataset)
        } else {
            self.dataframe.map(Dataset::new)
        }
    }

    fn estimated_pipeline_volume(&self) -> usize {
        let frame_volume = self.seed.as_ref().map_or(1, |seed| seed.row_count().max(1));
        let feature_volume = self.program.as_ref().map_or(0, ShellProgram::feature_count);
        let semantic_volume = self
            .logic_frame
            .as_ref()
            .map_or(0, |logic_frame| logic_frame.forms().len());

        frame_volume
            .saturating_add(feature_volume)
            .saturating_add(semantic_volume)
            .max(1)
    }

    fn estimated_graph_dimensions(&self) -> ConcreteGraphDimensions {
        let node_count = self.seed.as_ref().map_or(1, |seed| seed.row_count().max(1));
        let relationship_count = self
            .seed
            .as_ref()
            .map_or(0, |seed| {
                seed.row_count().saturating_mul(seed.column_count().max(1))
            })
            .saturating_add(self.program.as_ref().map_or(0, ShellProgram::feature_count))
            .saturating_add(
                self.logic_frame
                    .as_ref()
                    .map_or(0, |logic_frame| logic_frame.forms().len()),
            );

        ConcreteGraphDimensions::of(node_count, relationship_count)
    }
}
