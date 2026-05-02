//! Dataset ToolChain facade (UserLand boundary).
//!
//! This module exposes a tight, top-level pipeline surface for dataset DSL
//! authoring without requiring callers to traverse internal module layout.
//!
//! Architectural intent:
//! - ToolChain authoring is SDSL/GDSL-first and can be serialized as TS-JSON.
//! - UI adaptation (MVC React/Next) is downstream of that protocol surface.
//! - Logic inference/semantics may live in Relative Logic engines while this
//!   facade provides a stable kernel-adjacent vocabulary.
//! - User Design Surfaces are persisted primarily as storable plans/programs:
//!   ontological and epistemological artifacts that can later be projected into
//!   Entity/Property/Aspect records in external stores if needed.

use std::fs::read_to_string;
use std::path::Path;

use crate::collections::dataset::compile::{
    ontology_image_from_program_features, DatasetCompilation, DatasetNode, DatasetNodeKind,
    OntologyDataFrameImage,
};
use crate::collections::dataset::expressions::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOpExpr,
};
use crate::collections::dataset::expressions::io::DatasetIoExpr;
use crate::collections::dataset::expressions::metadata::DatasetMetadataExpr;
use crate::collections::dataset::expressions::projection::DatasetProjectionExpr;
use crate::collections::dataset::expressions::registry::DatasetRegistryExpr;
use crate::collections::dataset::expressions::reporting::DatasetReportExpr;
use crate::collections::dataset::namespaces;
use crate::collections::dataset::namespaces::dataset::DatasetNs;
use crate::collections::dataset::namespaces::feature::FeatureNs;
use crate::collections::dataset::namespaces::text::TextNs;
use crate::collections::dataset::namespaces::tree::TreeNs;
use crate::form::{ProgramFeature, ProgramFeatureKind, ProgramFeatures};
use polars::prelude::{col, Expr};

/// Ancient-science-style specification marker for SDSL artifacts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenusSpecies {
    pub genus: String,
    pub species: String,
}

/// Logical engine intent for a ToolChain specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalEngineIntent {
    RelativeForm,
    Custom(String),
}

/// MVC engine intent for UX realization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MvcEngineIntent {
    ReactNext,
    Custom(String),
}

#[derive(Debug, thiserror::Error)]
pub enum GdslSourceLoweringError {
    #[error("GDSL source is empty")]
    EmptySource,
    #[error("GDSL module declaration is missing")]
    MissingModule,
    #[error("GDSL declaration is invalid: {0}")]
    InvalidDeclaration(String),
    #[error("GDSL source IO error: {0}")]
    Io(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpecRef {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureSpecRef {
    pub id: String,
    pub label: String,
    pub model_id: Option<String>,
}

/// SDSL specification envelope authored from GDSL and serialized as TS-JSON.
///
/// This is the canonical stored artifact for Dataset-side extraction: a plan or
/// program image that represents discursive structure prior to any graph-ML or
/// graph-analytics interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SdslSpecification {
    pub id: String,
    pub title: String,
    pub classification: GenusSpecies,
    pub gdsl_source: Option<String>,
    pub models: Vec<ModelSpecRef>,
    pub features: Vec<FeatureSpecRef>,
    pub logical_engine: LogicalEngineIntent,
    pub mvc_engine: MvcEngineIntent,
}

/// Declarative UserLand pipeline envelope for dataset tooling.
///
/// The pipeline is intentionally program-centric: it stores how a semantic plan
/// is authored and lowered, rather than treating extracted dataset content as a
/// GraphFrame-native entity inventory.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DatasetPipeline {
    pub specification: Option<SdslSpecification>,
    pub registry: Option<DatasetRegistryExpr>,
    pub io: Option<DatasetIoExpr>,
    pub metadata: Vec<DatasetMetadataExpr>,
    pub ops: Vec<DatasetDataOpExpr>,
    pub projection: Option<DatasetProjectionExpr>,
    pub report: Option<DatasetReportExpr>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DatasetPipelineArtifacts {
    pub specification: Option<SdslSpecification>,
    pub dataset_aspects: Vec<DatasetAspectArtifact>,
    pub dataframe_lowerings: Vec<DataFrameLoweringArtifact>,
}

impl DatasetPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_registry(mut self, registry: DatasetRegistryExpr) -> Self {
        self.registry = Some(registry);
        self
    }

    pub fn with_specification(mut self, specification: SdslSpecification) -> Self {
        self.specification = Some(specification);
        self
    }

    pub fn with_io(mut self, io: DatasetIoExpr) -> Self {
        self.io = Some(io);
        self
    }

    pub fn with_metadata(mut self, metadata: DatasetMetadataExpr) -> Self {
        self.metadata.push(metadata);
        self
    }

    pub fn with_op(mut self, op: DatasetDataOpExpr) -> Self {
        self.ops.push(op);
        self
    }

    pub fn with_projection(mut self, projection: DatasetProjectionExpr) -> Self {
        self.projection = Some(projection);
        self
    }

    pub fn with_report(mut self, report: DatasetReportExpr) -> Self {
        self.report = Some(report);
        self
    }

    /// Canonical text-domain lifecycle scaffold:
    /// Input -> Encode -> Transform -> Decode -> Output.
    pub fn text_lifecycle(base_name: impl AsRef<str>) -> Self {
        let name = base_name.as_ref();
        Self::new()
            .with_op(DatasetDataOpExpr::text_input(format!("{name}.input")))
            .with_op(DatasetDataOpExpr::text_encode(format!("{name}.encode")))
            .with_op(DatasetDataOpExpr::text_transform(format!(
                "{name}.transform"
            )))
            .with_op(DatasetDataOpExpr::text_decode(format!("{name}.decode")))
            .with_op(DatasetDataOpExpr::text_output(format!("{name}.output")))
    }

    /// Lower pipeline operations into DataFrame expressions in evaluation order.
    ///
    /// The returned list is incremental: each entry is the running expression
    /// after applying one additional pipeline operation.
    pub fn lower_to_dataframe_exprs(&self, input_column: &str) -> Vec<Expr> {
        let mut out = Vec::with_capacity(self.ops.len());
        let mut current = col(input_column);

        for op in &self.ops {
            current = op.as_dataframe_expr(current);
            out.push(current.clone());
        }

        out
    }

    /// Build generated artifacts for both sides of the SDSL bridge:
    /// - Dataset-native aspect artifacts
    /// - DataFrame lowering artifacts
    pub fn build_artifacts(&self, input_column: &str) -> DatasetPipelineArtifacts {
        let mut dataset_aspects = Vec::with_capacity(self.ops.len());
        let mut dataframe_lowerings = Vec::with_capacity(self.ops.len());

        for op in &self.ops {
            dataset_aspects.push(op.as_dataset_aspect_artifact());
            dataframe_lowerings.push(op.lower_to_dataframe_artifact(input_column));
        }

        DatasetPipelineArtifacts {
            specification: self.specification.clone(),
            dataset_aspects,
            dataframe_lowerings,
        }
    }

    /// Build the Dataset compilation graph from specification + pipeline.
    ///
    /// The ToolChain specification is the source of truth; data-op pipeline
    /// stages are emitted as expression nodes linked to that specification.
    /// The resulting graph is a stored program image for SDSL/GDSL artifacts,
    /// not a substitute for a downstream graph-analytics runtime.
    pub fn to_compilation(&self) -> DatasetCompilation {
        let mut compilation = DatasetCompilation::new();
        let mut spec_root_id: Option<String> = None;

        if let Some(spec) = &self.specification {
            let root_id = format!("spec:{}", spec.id);
            let mut root = DatasetNode::new(&root_id, &spec.title, DatasetNodeKind::Macro)
                .with_meta("classification.genus", &spec.classification.genus)
                .with_meta("classification.species", &spec.classification.species);

            if let Some(gdsl) = &spec.gdsl_source {
                root = root.with_meta("gdsl.source", gdsl);
            }

            let logical_engine = match &spec.logical_engine {
                LogicalEngineIntent::RelativeForm => "relative-form".to_string(),
                LogicalEngineIntent::Custom(value) => value.clone(),
            };
            let mvc_engine = match &spec.mvc_engine {
                MvcEngineIntent::ReactNext => "react-next".to_string(),
                MvcEngineIntent::Custom(value) => value.clone(),
            };

            root = root
                .with_meta("engine.logical", logical_engine)
                .with_meta("engine.mvc", mvc_engine);

            compilation.add_node(root);
            spec_root_id = Some(root_id.clone());

            for model in &spec.models {
                let mut node = DatasetNode::new(
                    format!("model:{}", model.id),
                    &model.label,
                    DatasetNodeKind::Model,
                );
                if let Some(root_id) = &spec_root_id {
                    node = node.with_dep(root_id);
                }
                compilation.add_node(node);
            }

            for feature in &spec.features {
                let mut node = DatasetNode::new(
                    format!("feature:{}", feature.id),
                    &feature.label,
                    DatasetNodeKind::Feature,
                );
                if let Some(root_id) = &spec_root_id {
                    node = node.with_dep(root_id);
                }
                if let Some(model_id) = &feature.model_id {
                    node = node.with_dep(format!("model:{}", model_id));
                }
                compilation.add_node(node);
            }
        }

        let mut previous_expr_id: Option<String> = None;
        for (index, op) in self.ops.iter().enumerate() {
            let node_id = format!("expr:{}:{}", index, op.name());
            let mut node = DatasetNode::new(&node_id, op.name(), DatasetNodeKind::Expr)
                .with_meta("stage", op.stage());

            if let Some(domain) = op.domain() {
                node = node.with_meta("domain", domain);
            }
            if let Some(previous) = &previous_expr_id {
                node = node.with_dep(previous);
            }
            if let Some(root_id) = &spec_root_id {
                node = node.with_dep(root_id);
            }

            compilation.add_node(node);
            previous_expr_id = Some(node_id);
        }

        if let Some(last_expr_id) = previous_expr_id {
            compilation.add_entrypoint(last_expr_id);
        } else if let Some(root_id) = spec_root_id {
            compilation.add_entrypoint(root_id);
        }

        compilation
    }

    /// Build a dataset compilation and merge Program Feature image formation.
    ///
    /// This composes pipeline artifacts with a Unix-style program image rooted in
    /// `ProgramFeatures`, preserving feature-first semantics for execution.
    pub fn to_compilation_with_program_features(
        &self,
        program_features: &ProgramFeatures,
    ) -> DatasetCompilation {
        let mut compilation = self.to_compilation();
        let image = DatasetCompilation::from_program_features(program_features);
        compilation.merge(image);
        compilation
    }

    /// Merge a pre-built ontology image manifest into the dataset compilation graph.
    pub fn to_compilation_with_ontology_image(
        &self,
        ontology_image: &OntologyDataFrameImage,
    ) -> DatasetCompilation {
        let mut compilation = self.to_compilation();
        let image = DatasetCompilation::from_ontology_image(ontology_image);
        compilation.merge(image);
        compilation
    }
}

/// Top-level UserLand ToolChain facade.
#[derive(Debug, Clone, Copy, Default)]
pub struct DatasetToolChain;

impl DatasetToolChain {
    pub fn dataset() -> DatasetNs {
        DatasetNs
    }

    pub fn feature() -> FeatureNs {
        FeatureNs
    }

    pub fn text() -> TextNs {
        TextNs
    }

    pub fn dataop() -> crate::collections::dataset::namespaces::dataop::DataOpNs {
        crate::collections::dataset::namespaces::dataop::DataOpNs
    }

    pub fn tree() -> TreeNs {
        TreeNs
    }

    pub fn pipeline() -> DatasetPipeline {
        DatasetPipeline::new()
    }

    /// Compile a Program Feature set directly into a dataset image compilation.
    pub fn image_from_program_features(program_features: &ProgramFeatures) -> DatasetCompilation {
        DatasetCompilation::from_program_features(program_features)
    }

    /// Lower a minimal textual GDSL source into the current compact ProgramFeatures surface.
    ///
    /// This is intentionally a transitional adapter: it preserves the authored
    /// semantic stages in feature values and sources so existing compilation and
    /// materialization paths can already be exercised against `.gdsl` fixtures.
    pub fn program_features_from_gdsl_source(
        gdsl_source: &str,
    ) -> Result<ProgramFeatures, GdslSourceLoweringError> {
        lower_gdsl_source_to_program_features(gdsl_source)
    }

    /// Read a `.gdsl` source file and lower it into ProgramFeatures.
    pub fn program_features_from_gdsl_file(
        path: impl AsRef<Path>,
    ) -> Result<ProgramFeatures, GdslSourceLoweringError> {
        let gdsl_source = read_to_string(path.as_ref())
            .map_err(|err| GdslSourceLoweringError::Io(err.to_string()))?;
        Self::program_features_from_gdsl_source(&gdsl_source)
    }

    /// Compile a textual GDSL source directly into the current Dataset image graph.
    pub fn image_from_gdsl_source(
        gdsl_source: &str,
    ) -> Result<DatasetCompilation, GdslSourceLoweringError> {
        let features = Self::program_features_from_gdsl_source(gdsl_source)?;
        Ok(Self::image_from_program_features(&features))
    }

    /// Compile a `.gdsl` source file directly into the current Dataset image graph.
    pub fn image_from_gdsl_file(
        path: impl AsRef<Path>,
    ) -> Result<DatasetCompilation, GdslSourceLoweringError> {
        let features = Self::program_features_from_gdsl_file(path)?;
        Ok(Self::image_from_program_features(&features))
    }

    /// Build an ontology dataframe image manifest from textual GDSL source.
    pub fn ontology_image_from_gdsl_source(
        gdsl_source: &str,
    ) -> Result<OntologyDataFrameImage, GdslSourceLoweringError> {
        let features = Self::program_features_from_gdsl_source(gdsl_source)?;
        Ok(ontology_image_from_program_features(&features))
    }

    /// Build an ontology dataframe image manifest from a `.gdsl` source file.
    pub fn ontology_image_from_gdsl_file(
        path: impl AsRef<Path>,
    ) -> Result<OntologyDataFrameImage, GdslSourceLoweringError> {
        let features = Self::program_features_from_gdsl_file(path)?;
        Ok(ontology_image_from_program_features(&features))
    }

    /// Build an ontology dataframe image manifest from Program Features.
    pub fn ontology_image_from_program_features(
        program_features: &ProgramFeatures,
    ) -> OntologyDataFrameImage {
        ontology_image_from_program_features(program_features)
    }

    /// Compile a manifest-level ontology image directly into dataset compilation nodes.
    pub fn image_from_ontology_manifest(
        ontology_image: &OntologyDataFrameImage,
    ) -> DatasetCompilation {
        DatasetCompilation::from_ontology_image(ontology_image)
    }

    pub fn specification(
        id: impl Into<String>,
        title: impl Into<String>,
        genus: impl Into<String>,
        species: impl Into<String>,
    ) -> SdslSpecification {
        SdslSpecification {
            id: id.into(),
            title: title.into(),
            classification: GenusSpecies {
                genus: genus.into(),
                species: species.into(),
            },
            gdsl_source: None,
            models: Vec::new(),
            features: Vec::new(),
            logical_engine: LogicalEngineIntent::RelativeForm,
            mvc_engine: MvcEngineIntent::ReactNext,
        }
    }

    pub fn text_pipeline(base_name: impl AsRef<str>) -> DatasetPipeline {
        DatasetPipeline::text_lifecycle(base_name)
    }

    pub fn register_namespace(name: &str) -> Result<(), namespaces::NameSpaceError> {
        namespaces::register_dataset_namespace(name)
    }

    pub fn is_namespace_registered(name: &str) -> bool {
        namespaces::is_dataset_namespace_registered(name)
    }
}

impl SdslSpecification {
    pub fn with_gdsl_source(mut self, gdsl_source: impl Into<String>) -> Self {
        self.gdsl_source = Some(gdsl_source.into());
        self
    }

    pub fn with_logical_engine(mut self, engine: LogicalEngineIntent) -> Self {
        self.logical_engine = engine;
        self
    }

    pub fn with_mvc_engine(mut self, engine: MvcEngineIntent) -> Self {
        self.mvc_engine = engine;
        self
    }

    pub fn with_model(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
        self.models.push(ModelSpecRef {
            id: id.into(),
            label: label.into(),
        });
        self
    }

    pub fn with_feature(
        mut self,
        id: impl Into<String>,
        label: impl Into<String>,
        model_id: Option<String>,
    ) -> Self {
        self.features.push(FeatureSpecRef {
            id: id.into(),
            label: label.into(),
            model_id,
        });
        self
    }
}

fn lower_gdsl_source_to_program_features(
    gdsl_source: &str,
) -> Result<ProgramFeatures, GdslSourceLoweringError> {
    if gdsl_source.trim().is_empty() {
        return Err(GdslSourceLoweringError::EmptySource);
    }

    let mut program_name: Option<String> = None;
    let mut selected_forms = Vec::new();
    let mut features = Vec::new();

    for raw_line in gdsl_source.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if let Some(module) = line.strip_prefix("module ") {
            let module = strip_trailing_semicolon(module).trim();
            if module.is_empty() {
                return Err(GdslSourceLoweringError::InvalidDeclaration(
                    line.to_string(),
                ));
            }
            program_name = Some(module.to_string());
            continue;
        }

        if let Some(import_name) = line.strip_prefix("use ") {
            let import_name = strip_trailing_semicolon(import_name).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Import,
                format!("use::{import_name}"),
                format!("use::{import_name}"),
            ));
            continue;
        }

        if line.starts_with("source ") {
            let name = parse_nth_token(line, 2)?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Source,
                format!("source::{name}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if line.starts_with("appearance ") {
            let name = parse_named_declaration(line, "appearance", Some("from"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Observation,
                format!("appearance::{name}"),
                format!("appearance::{name}"),
            ));
            continue;
        }

        if line.starts_with("reflection ") {
            let name = parse_named_declaration(line, "reflection", Some("for"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Reflection,
                format!("reflection::{name}"),
                format!("reflection::{name}"),
            ));
            continue;
        }

        if line.starts_with("logogenesis ") {
            let name = parse_named_declaration(line, "logogenesis", Some("for"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Logogenesis,
                format!("logogenesis::{name}"),
                format!("logogenesis::{name}"),
            ));
            continue;
        }
        if line.starts_with("subfeature ") {
            let rest = line.strip_prefix("subfeature ").unwrap().trim();
            let name = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Subfeature,
                format!("subfeature::{name}"),
                format!("subfeature::{name}"),
            ));
            continue;
        }
        if let Some(rest) = line.strip_prefix("derive ") {
            let derived_name = rest
                .split('=')
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Mark,
                format!("derive::{derived_name}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("key ") {
            let key = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Observation,
                format!("key::{key}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("retain ") {
            let retained = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Observation,
                format!("retain::{}", normalize_gdsl_phrase(retained)),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if line.starts_with("mark ") {
            let name = parse_named_declaration(line, "mark", Some("on"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Mark,
                format!("mark::{name}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if line.starts_with("concept ") {
            let name = parse_named_declaration(line, "concept", Some("from"))?;
            if !selected_forms.iter().any(|existing| existing == name) {
                selected_forms.push(name.to_string());
            }
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Concept,
                format!("concept::{name}"),
                format!("concept::{name}"),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("identity ") {
            let identity = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Concept,
                format!("identity::{identity}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if line.starts_with("judgment ") {
            let name = parse_named_declaration(line, "judgment", Some("for"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Judgment,
                format!("judgment::{name}"),
                format!("judgment::{name}"),
            ));
            continue;
        }

        if line.starts_with("syllogism ") {
            let name = parse_named_declaration(line, "syllogism", Some("for"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Syllogism,
                format!("syllogism::{name}"),
                format!("syllogism::{name}"),
            ));
            continue;
        }

        if line.starts_with("principle ") {
            let name = parse_named_declaration(line, "principle", Some("for"))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Principle,
                format!("principle::{name}"),
                format!("principle::{name}"),
            ));
            continue;
        }

        if line.starts_with("query ") {
            let name = parse_named_declaration(line, "query", Some(":="))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Query,
                format!("query::{name}"),
                format!("query::{name}"),
            ));
            continue;
        }

        if line.starts_with("procedure ") {
            let name = parse_named_declaration(line, "procedure", None)?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Procedure,
                format!("procedure::{name}"),
                format!("procedure::{name}"),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("infer ") {
            let infer_name = rest
                .split(" when ")
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Inference,
                format!("infer::{infer_name}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("conclude ") {
            let conclusion = rest
                .split(" when ")
                .next()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))?;
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Inference,
                format!("conclude::{conclusion}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("select ") {
            let selected = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Query,
                format!("select::{}", normalize_gdsl_phrase(selected)),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("middle ") {
            let middle = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Syllogism,
                format!("middle::{middle}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("stage ") {
            let stage = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Reflection,
                format!("stage::{stage}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("preserve ") {
            let preserved = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Reflection,
                format!("preserve::{preserved}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("culminate ") {
            let culmination = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Reflection,
                format!("culminate::{culmination}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("from ") {
            let source = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Source,
                format!("from::{source}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("unfold ") {
            let unfolded = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Logogenesis,
                format!("unfold::{unfolded}"),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("require ") {
            let requirement = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Condition,
                format!("require::{}", normalize_gdsl_phrase(requirement)),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("where ") {
            let condition = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Condition,
                format!("where::{}", normalize_gdsl_phrase(condition)),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("unify ") {
            let unify_expr = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Principle,
                format!("unify::{}", normalize_gdsl_phrase(unify_expr)),
                normalize_gdsl_phrase(line),
            ));
            continue;
        }

        if let Some(rest) = line.strip_prefix("emit ") {
            let emitted = strip_trailing_semicolon(rest).trim();
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Procedure,
                format!("emit::{}", normalize_gdsl_phrase(emitted)),
                normalize_gdsl_phrase(line),
            ));
        }
    }

    let program_name = program_name.ok_or(GdslSourceLoweringError::MissingModule)?;
    let mut lowered_features = Vec::with_capacity(features.len() + 1);
    lowered_features.push(ProgramFeature::new(
        ProgramFeatureKind::SpecificationBinding,
        program_name.clone(),
        format!("specification::{program_name}"),
    ));
    lowered_features.extend(features);

    Ok(ProgramFeatures::new(
        program_name,
        selected_forms,
        lowered_features,
    ))
}

fn parse_nth_token(line: &str, index: usize) -> Result<&str, GdslSourceLoweringError> {
    line.split_whitespace()
        .nth(index)
        .map(strip_trailing_semicolon)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))
}

fn parse_named_declaration<'a>(
    line: &'a str,
    keyword: &str,
    boundary: Option<&str>,
) -> Result<&'a str, GdslSourceLoweringError> {
    let rest = line
        .strip_prefix(&format!("{keyword} "))
        .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))?;

    let head = if let Some(boundary) = boundary {
        rest.split(boundary).next().unwrap_or(rest)
    } else {
        rest.split('{').next().unwrap_or(rest)
    };

    head.split_whitespace()
        .next()
        .map(strip_trailing_semicolon)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| GdslSourceLoweringError::InvalidDeclaration(line.to_string()))
}

fn strip_trailing_semicolon(value: &str) -> &str {
    value.strip_suffix(';').unwrap_or(value)
}

fn normalize_gdsl_phrase(value: &str) -> String {
    value
        .trim()
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, ':' | '_' | '-' | '.') {
                ch.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    const ABSOLUTE_CONCEPT_FIXTURE: &str =
        include_str!("../../../fixtures/gdsl/absolute-concept-scientific-inference.gdsl");

    #[test]
    fn lowers_absolute_concept_gdsl_to_program_features() {
        let lowered = DatasetToolChain::program_features_from_gdsl_source(ABSOLUTE_CONCEPT_FIXTURE)
            .expect("fixture should lower to ProgramFeatures");

        assert_eq!(
            lowered.program_name,
            "org.hegel.epistemology.absolute_concept.scientific_inference"
        );
        assert!(lowered
            .selected_forms
            .contains(&"AppearanceObject".to_string()));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Mark
                && feature.value == "mark::quality"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Judgment
                && feature.value == "judgment::determinate_appearance"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Syllogism
                && feature.value == "syllogism::scientific_inference"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Reflection
                && feature.value == "reflection::essence_path"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Logogenesis
                && feature.value == "logogenesis::scientific_genesis"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Logogenesis
                && feature.value == "unfold::essential_relation"));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Source));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Observation));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Principle));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Concept));
        assert!(lowered
            .features
            .iter()
            .any(|feature| feature.kind == ProgramFeatureKind::Procedure));
    }

    #[test]
    fn compiles_absolute_concept_gdsl_into_valid_dataset_image() {
        let compilation = DatasetToolChain::image_from_gdsl_source(ABSOLUTE_CONCEPT_FIXTURE)
            .expect("fixture should compile into DatasetCompilation");

        assert!(compilation.validate().is_ok());
        assert!(compilation
            .entrypoints
            .iter()
            .any(|entry| entry.starts_with("image:")));
    }
}
