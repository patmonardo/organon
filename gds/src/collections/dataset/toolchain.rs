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
//! - User Design Surfaces can be persisted as Entity/Property/Aspect records
//!   in external stores (for example Neo4j-backed Relative Form persistence).

use crate::collections::dataset::expressions::dataop::{
    DataFrameLoweringArtifact, DatasetAspectArtifact, DatasetDataOpExpr,
};
use crate::collections::dataset::compile_ir::{DatasetCompilation, DatasetNode, DatasetNodeKind};
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
use crate::form::ProgramFeatures;
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
