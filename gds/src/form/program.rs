//! Minimal Form program types.
//!
//! This module exists to carry a small, stable “program shape” into the Form evaluator.
//! At runtime, the only required field today is `Morph.patterns`, which drives the
//! `FormOperator` chain.
//!
//! Dialectical convention (semantic mapping; not enforced by the kernel):
//! - `Shape`   → Essence (the program's essential envelope / what is “there” as form)
//! - `Context` → Determination of Essence / Reflection (how the form is determined)
//! - `Morph`   → Ground as **Active Ground** (the operator chain that actualizes the determination)

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/// Form program payload passed into the Form evaluator.
///
/// `FormShape` is the being-manifestation of a `PureFormPrinciple`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FormShape {
    pub shape: Shape,
    pub context: Context,
    pub morph: Morph,
}

impl FormShape {
    pub fn new(shape: Shape, context: Context, morph: Morph) -> Self {
        Self {
            shape,
            context,
            morph,
        }
    }

    pub fn from_principle(principle: PureFormPrinciple) -> Self {
        principle.into()
    }

    pub fn as_principle(&self) -> PureFormPrinciple {
        PureFormPrinciple::new(self.shape.clone(), self.context.clone(), self.morph.clone())
    }
}

/// Pure Form Principle: Shape/Context/Morph as principled triad.
///
/// Semantic mapping:
/// - `shape`   = first-seen form/appearance envelope
/// - `context` = reflective determination
/// - `morph`   = grounding operation (kriya of realization)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PureFormPrinciple {
    pub shape: Shape,
    pub context: Context,
    pub morph: Morph,
}

impl PureFormPrinciple {
    pub fn new(shape: Shape, context: Context, morph: Morph) -> Self {
        Self {
            shape,
            context,
            morph,
        }
    }
}

impl From<PureFormPrinciple> for FormShape {
    fn from(principle: PureFormPrinciple) -> Self {
        Self {
            shape: principle.shape,
            context: principle.context,
            morph: principle.morph,
        }
    }
}

impl From<FormShape> for PureFormPrinciple {
    fn from(shape: FormShape) -> Self {
        Self {
            shape: shape.shape,
            context: shape.context,
            morph: shape.morph,
        }
    }
}

/// Optional structural metadata about the program.
///
/// Semantic convention: this is the program's *Essence* envelope.
///
/// Currently unused by the evaluator, but retained as a small, non-opinionated envelope
/// for schema-first expansion.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Shape {
    pub required_fields: Vec<String>,
    pub optional_fields: Vec<String>,
    pub type_constraints: HashMap<String, String>,
    pub validation_rules: HashMap<String, String>,
}

impl Shape {
    pub fn new(
        required_fields: Vec<String>,
        optional_fields: Vec<String>,
        type_constraints: HashMap<String, String>,
        validation_rules: HashMap<String, String>,
    ) -> Self {
        Self {
            required_fields,
            optional_fields,
            type_constraints,
            validation_rules,
        }
    }
}

/// Execution context metadata.
///
/// Semantic convention: this carries *Determination of Essence / Reflection* inputs.
///
/// Currently unused by the evaluator; kept lightweight.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Context {
    pub dependencies: Vec<String>,
    pub execution_order: Vec<String>,
    pub runtime_strategy: String,
    pub conditions: Vec<String>,
}

impl Context {
    pub fn new(
        dependencies: Vec<String>,
        execution_order: Vec<String>,
        runtime_strategy: String,
        conditions: Vec<String>,
    ) -> Self {
        Self {
            dependencies,
            execution_order,
            runtime_strategy,
            conditions,
        }
    }
}

/// The only part of the program required by the current Form ISA.
///
/// Semantic convention: `Morph` is **Active Ground** — the effective ground that
/// realizes the program through the selected operator chain.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Morph {
    /// Names of Form operators to run, in order.
    pub patterns: Vec<String>,
}

impl Morph {
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns }
    }
}

/// Domain specification envelope used for GDSL/SDSL representations in-kernel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Specification {
    pub name: String,
    pub version: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Specification {
    pub fn new(name: String, version: Option<String>, attributes: HashMap<String, String>) -> Self {
        Self {
            name,
            version,
            attributes,
        }
    }

    /// Attach structured embedding metadata into namespaced specification attributes.
    ///
    /// This keeps current struct shape stable while allowing progressive FormDB/GDSL/SDSL
    /// modeling in-kernel.
    pub fn with_embedding(mut self, embedding: &SpecificationEmbedding) -> Self {
        embedding.write_to_attributes(&mut self.attributes);
        self
    }

    /// Read structured embedding metadata from namespaced attributes.
    pub fn embedding(&self) -> SpecificationEmbedding {
        SpecificationEmbedding::from_attributes(&self.attributes)
    }
}

const ATTR_KIND: &str = "formdb.kind";
const ATTR_SCHEMA_REF: &str = "formdb.schema_ref";
const ATTR_CYPHER_FACT_GRAPH: &str = "formdb.cypher.fact_graph";
const ATTR_CYPHER_KNOWLEDGE_GRAPH: &str = "formdb.cypher.knowledge_graph";
const ATTR_CYPHER_LABELS: &str = "formdb.cypher.labels";
const ATTR_CYPHER_REL_TYPES: &str = "formdb.cypher.relationship_types";
const ATTR_ENGINE: &str = "formdb.engine";
const ATTR_CLIENT_SDK: &str = "formdb.client_sdk";
const ATTR_INTEROP_SURFACE: &str = "formdb.interop_surface";
const ATTR_HUMAN_LOOP: &str = "formdb.human_in_loop";

/// Classification for FormDB specification role in the GDSL/SDSL pathway.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecificationKind {
    Gdsl,
    Sdsl,
    Unknown,
}

impl Default for SpecificationKind {
    fn default() -> Self {
        Self::Unknown
    }
}

impl SpecificationKind {
    fn as_attr(&self) -> &'static str {
        match self {
            Self::Gdsl => "gdsl",
            Self::Sdsl => "sdsl",
            Self::Unknown => "unknown",
        }
    }

    fn from_attr(value: Option<&str>) -> Self {
        match value {
            Some("gdsl") => Self::Gdsl,
            Some("sdsl") => Self::Sdsl,
            _ => Self::Unknown,
        }
    }
}

/// Structured metadata that embeds FormDB semantics into a specification.
///
/// This is a Rust-side mock/progressive model that aligns with TS/Zod-first schemas
/// without forcing immediate deep type coupling.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SpecificationEmbedding {
    pub kind: SpecificationKind,
    pub schema_ref: Option<String>,
    pub cypher_fact_graph: Option<String>,
    pub cypher_knowledge_graph: Option<String>,
    pub cypher_labels: Vec<String>,
    pub cypher_relationship_types: Vec<String>,
    pub engine: Option<String>,
    pub client_sdk: Option<String>,
    pub interop_surface: Option<String>,
    pub human_in_loop: bool,
}

impl SpecificationEmbedding {
    pub fn write_to_attributes(&self, attributes: &mut HashMap<String, String>) {
        attributes.insert(ATTR_KIND.to_string(), self.kind.as_attr().to_string());

        write_opt_attr(attributes, ATTR_SCHEMA_REF, self.schema_ref.as_deref());
        write_opt_attr(
            attributes,
            ATTR_CYPHER_FACT_GRAPH,
            self.cypher_fact_graph.as_deref(),
        );
        write_opt_attr(
            attributes,
            ATTR_CYPHER_KNOWLEDGE_GRAPH,
            self.cypher_knowledge_graph.as_deref(),
        );
        write_opt_attr(attributes, ATTR_ENGINE, self.engine.as_deref());
        write_vec_attr(attributes, ATTR_CYPHER_LABELS, &self.cypher_labels);
        write_vec_attr(
            attributes,
            ATTR_CYPHER_REL_TYPES,
            &self.cypher_relationship_types,
        );
        write_opt_attr(attributes, ATTR_CLIENT_SDK, self.client_sdk.as_deref());
        write_opt_attr(
            attributes,
            ATTR_INTEROP_SURFACE,
            self.interop_surface.as_deref(),
        );
        attributes.insert(ATTR_HUMAN_LOOP.to_string(), self.human_in_loop.to_string());
    }

    pub fn from_attributes(attributes: &HashMap<String, String>) -> Self {
        let kind = SpecificationKind::from_attr(attributes.get(ATTR_KIND).map(String::as_str));

        Self {
            kind,
            schema_ref: attributes.get(ATTR_SCHEMA_REF).cloned(),
            cypher_fact_graph: attributes.get(ATTR_CYPHER_FACT_GRAPH).cloned(),
            cypher_knowledge_graph: attributes.get(ATTR_CYPHER_KNOWLEDGE_GRAPH).cloned(),
            cypher_labels: read_vec_attr(attributes, ATTR_CYPHER_LABELS),
            cypher_relationship_types: read_vec_attr(attributes, ATTR_CYPHER_REL_TYPES),
            engine: attributes.get(ATTR_ENGINE).cloned(),
            client_sdk: attributes.get(ATTR_CLIENT_SDK).cloned(),
            interop_surface: attributes.get(ATTR_INTEROP_SURFACE).cloned(),
            human_in_loop: attributes
                .get(ATTR_HUMAN_LOOP)
                .map(|value| value == "true")
                .unwrap_or(false),
        }
    }
}

fn write_opt_attr(attributes: &mut HashMap<String, String>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        attributes.insert(key.to_string(), value.to_string());
    } else {
        attributes.remove(key);
    }
}

fn write_vec_attr(attributes: &mut HashMap<String, String>, key: &str, values: &[String]) {
    if values.is_empty() {
        attributes.remove(key);
    } else {
        attributes.insert(key.to_string(), values.join(","));
    }
}

fn read_vec_attr(attributes: &HashMap<String, String>, key: &str) -> Vec<String> {
    attributes
        .get(key)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// A named application-level Form module.
///
/// Each form contributes feature descriptors and one or more operator patterns
/// that can be compiled into `Morph.patterns` for execution.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ApplicationForm {
    pub name: String,
    pub domain: String,
    pub features: Vec<String>,
    pub patterns: Vec<String>,
    pub specifications: HashMap<String, String>,
}

/// Principled-effect: the contained applied moment of a Given Form.
///
/// This is the Application Form as applied-to-appearance (typically dataset-facing SDSL).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PrincipledEffect {
    pub application_form: ApplicationForm,
    pub appearance: Option<String>,
}

impl PrincipledEffect {
    pub fn new(application_form: ApplicationForm, appearance: Option<String>) -> Self {
        Self {
            application_form,
            appearance,
        }
    }
}

/// Ordered Given Form moments for evaluation staging.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GivenFormStage {
    /// Principle container (PureForm/GDSL moment)
    GdslPrinciple,
    /// Principled-effect contained in appearance (SDSL moment)
    SdslPrincipledEffect,
    /// Evaluation moment that unifies principle and effect
    Eval,
}

/// The explicit seven-layer perfected Given Form decomposition:
/// 3 Principle + 3 Principled-Effect + 1 Evaluation (monadic).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfectedGivenFormLayer {
    PrincipleShape,
    PrincipleContext,
    PrincipleMorph,
    EffectEntity,
    EffectProperty,
    EffectAspect,
    EvaluationMonadic,
}

/// Full Given Form envelope used prior to evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GivenFormEnvelope {
    pub principle: PureFormPrinciple,
    pub principled_effect: PrincipledEffect,
}

impl GivenFormEnvelope {
    pub fn new(principle: PureFormPrinciple, principled_effect: PrincipledEffect) -> Self {
        Self {
            principle,
            principled_effect,
        }
    }

    pub fn stage_order() -> [GivenFormStage; 3] {
        [
            GivenFormStage::GdslPrinciple,
            GivenFormStage::SdslPrincipledEffect,
            GivenFormStage::Eval,
        ]
    }

    pub fn into_perfected(self) -> PerfectedGivenForm {
        PerfectedGivenForm::from_envelope(self)
    }
}

/// Explicit decomposition of a Given Form into seven layers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PerfectedGivenForm {
    pub envelope: GivenFormEnvelope,
    pub entity: String,
    pub property: String,
    pub aspect: String,
}

impl PerfectedGivenForm {
    pub fn from_envelope(envelope: GivenFormEnvelope) -> Self {
        let form_name = envelope.principled_effect.application_form.name.clone();
        let entity = format!("entity::{form_name}");
        let property = format!("property::{form_name}");
        let aspect = envelope
            .principled_effect
            .appearance
            .clone()
            .map(|appearance| format!("aspect::{appearance}"))
            .unwrap_or_else(|| format!("aspect::{form_name}"));

        Self {
            envelope,
            entity,
            property,
            aspect,
        }
    }

    pub fn layers() -> [PerfectedGivenFormLayer; 7] {
        [
            PerfectedGivenFormLayer::PrincipleShape,
            PerfectedGivenFormLayer::PrincipleContext,
            PerfectedGivenFormLayer::PrincipleMorph,
            PerfectedGivenFormLayer::EffectEntity,
            PerfectedGivenFormLayer::EffectProperty,
            PerfectedGivenFormLayer::EffectAspect,
            PerfectedGivenFormLayer::EvaluationMonadic,
        ]
    }
}

impl ApplicationForm {
    pub fn new(
        name: String,
        domain: String,
        features: Vec<String>,
        patterns: Vec<String>,
        specifications: HashMap<String, String>,
    ) -> Self {
        Self {
            name,
            domain,
            features,
            patterns,
            specifications,
        }
    }
}

/// Program-level FormDB projection used by the Rust kernel.
///
/// `form` is the stable evaluator payload.
/// `gdsl` and `sdsl` carry canonical/specification metadata.
/// `application_forms` and `selected_forms` model the feature suite projected into execution.
///
/// Design note:
/// - Compiler concern: produce a stable Program Compile-IR from a ProgramSpec.
/// - Evaluator concern: execute/interpret that IR inside a projection world.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramSpec {
    pub form: FormShape,
    pub gdsl: Specification,
    pub sdsl: Vec<Specification>,
    pub application_forms: Vec<ApplicationForm>,
    pub selected_forms: Vec<String>,
}

impl ProgramSpec {
    pub fn new(
        form: FormShape,
        gdsl: Specification,
        sdsl: Vec<Specification>,
        application_forms: Vec<ApplicationForm>,
        selected_forms: Vec<String>,
    ) -> Self {
        Self {
            form,
            gdsl,
            sdsl,
            application_forms,
            selected_forms,
        }
    }

    /// Compiles a deterministic execution plan by resolving selected application forms
    /// and appending their patterns after the base `form.morph.patterns` chain.
    pub fn compile_execution_plan(&self) -> Result<ProgramExecutionPlan, ProgramSpecError> {
        let registry = ApplicationFormRegistry::new(self.application_forms.clone())?;
        registry.compile(&self.form.morph, &self.selected_forms)
    }

    /// Compile into an explicit IR artifact.
    ///
    /// This is the compiler-facing boundary used by Dataset/SDSL and codegen work.
    pub fn compile_ir(&self) -> Result<ProgramCompileIr, ProgramSpecError> {
        let features = self.define_features()?;
        Ok(ProgramCompileIr::from_features(features))
    }

    /// Define canonical Program Features before compiler artifacts are constructed.
    ///
    /// This feature surface is the intended composition boundary for Dataset/DataFrame
    /// systems and acts as the semantic contract that compiler outputs must conform to.
    pub fn define_features(&self) -> Result<ProgramFeatures, ProgramSpecError> {
        let plan = self.compile_execution_plan()?;
        let mut features = Vec::new();

        for form in &plan.selected_forms {
            features.push(ProgramFeature::new(
                ProgramFeatureKind::ApplicationForm,
                form.clone(),
                format!("application_form::{form}"),
            ));
        }

        for pattern in &plan.patterns {
            features.push(ProgramFeature::new(
                ProgramFeatureKind::OperatorPattern,
                pattern.clone(),
                format!("operator_pattern::{pattern}"),
            ));
        }

        for dependency in &self.form.context.dependencies {
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Dependency,
                dependency.clone(),
                format!("context_dependency::{dependency}"),
            ));
        }

        for condition in &self.form.context.conditions {
            features.push(ProgramFeature::new(
                ProgramFeatureKind::Condition,
                condition.clone(),
                format!("context_condition::{condition}"),
            ));
        }

        features.push(ProgramFeature::new(
            ProgramFeatureKind::SpecificationBinding,
            self.gdsl.name.clone(),
            format!("specification::{}", self.gdsl.name),
        ));

        Ok(ProgramFeatures::new(
            self.gdsl.name.clone(),
            plan.selected_forms,
            features,
        ))
    }

    /// Build Given Form envelopes for each selected Application Form.
    ///
    /// This models the pre-eval bridge:
    /// 1) PureForm principle (GDSL moment)
    /// 2) Principled-effect in appearance (SDSL moment)
    /// 3) Eval unification moment
    pub fn given_forms(
        &self,
        appearance: Option<String>,
    ) -> Result<Vec<GivenFormEnvelope>, ProgramSpecError> {
        let registry = ApplicationFormRegistry::new(self.application_forms.clone())?;
        let requested = if self.selected_forms.is_empty() {
            registry.ordered_names.clone()
        } else {
            self.selected_forms.clone()
        };

        let principle = self.form.as_principle();
        let mut out = Vec::with_capacity(requested.len());

        for name in requested {
            let app = registry
                .forms
                .get(&name)
                .cloned()
                .ok_or_else(|| ProgramSpecError::UnknownApplicationForm(name.clone()))?;

            out.push(GivenFormEnvelope::new(
                principle.clone(),
                PrincipledEffect::new(app, appearance.clone()),
            ));
        }

        Ok(out)
    }
}

impl From<FormShape> for ProgramSpec {
    fn from(form: FormShape) -> Self {
        Self {
            form,
            gdsl: Specification::default(),
            sdsl: Vec::new(),
            application_forms: Vec::new(),
            selected_forms: Vec::new(),
        }
    }
}

impl From<ProgramSpec> for FormShape {
    fn from(program: ProgramSpec) -> Self {
        program.form
    }
}

/// Compiled runtime plan produced from a `ProgramSpec`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramExecutionPlan {
    pub selected_forms: Vec<String>,
    pub patterns: Vec<String>,
}

/// Canonical feature kinds that define what a Program means prior to compilation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgramFeatureKind {
    ApplicationForm,
    OperatorPattern,
    Dependency,
    Condition,
    SpecificationBinding,
}

/// Atomic program feature used by Dataset/DataFrame composition and compiler inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProgramFeature {
    pub kind: ProgramFeatureKind,
    pub value: String,
    pub source: String,
}

impl ProgramFeature {
    pub fn new(kind: ProgramFeatureKind, value: String, source: String) -> Self {
        Self {
            kind,
            value,
            source,
        }
    }
}

/// Feature-level definition of a Program; compiler artifacts must conform to this.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramFeatures {
    pub program_name: String,
    pub selected_forms: Vec<String>,
    pub features: Vec<ProgramFeature>,
}

impl ProgramFeatures {
    pub fn new(
        program_name: String,
        selected_forms: Vec<String>,
        features: Vec<ProgramFeature>,
    ) -> Self {
        Self {
            program_name,
            selected_forms,
            features,
        }
    }

    pub fn opcodes(&self) -> Vec<String> {
        self.features
            .iter()
            .filter(|feature| matches!(feature.kind, ProgramFeatureKind::OperatorPattern))
            .map(|feature| feature.value.clone())
            .collect()
    }
}

/// Compiler artifact for Program Features.
///
/// This is intentionally runtime-agnostic and can be consumed by codegen,
/// Dataset/SDSL infrastructure, or evaluator runtimes.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProgramCompileIr {
    pub program_name: String,
    pub selected_forms: Vec<String>,
    pub opcodes: Vec<String>,
}

impl ProgramCompileIr {
    pub fn new(program_name: String, selected_forms: Vec<String>, opcodes: Vec<String>) -> Self {
        Self {
            program_name,
            selected_forms,
            opcodes,
        }
    }

    pub fn into_execution_plan(self) -> ProgramExecutionPlan {
        ProgramExecutionPlan::new(self.selected_forms, self.opcodes)
    }

    pub fn from_features(features: ProgramFeatures) -> Self {
        let opcodes = features.opcodes();
        Self {
            program_name: features.program_name,
            selected_forms: features.selected_forms,
            opcodes,
        }
    }
}

/// Thin compiler facade for ProgramSpec -> ProgramCompileIr.
#[derive(Debug, Clone, Default)]
pub struct ProgramCompiler;

impl ProgramCompiler {
    pub fn new() -> Self {
        Self
    }

    pub fn compile(&self, program: &ProgramSpec) -> Result<ProgramCompileIr, ProgramSpecError> {
        program.compile_ir()
    }

    pub fn define_features(
        &self,
        program: &ProgramSpec,
    ) -> Result<ProgramFeatures, ProgramSpecError> {
        program.define_features()
    }
}

impl ProgramExecutionPlan {
    pub fn new(selected_forms: Vec<String>, patterns: Vec<String>) -> Self {
        Self {
            selected_forms,
            patterns,
        }
    }
}

/// Registry/executor for application forms.
#[derive(Debug, Clone, Default)]
pub struct ApplicationFormRegistry {
    forms: HashMap<String, ApplicationForm>,
    ordered_names: Vec<String>,
}

impl ApplicationFormRegistry {
    pub fn new(forms: Vec<ApplicationForm>) -> Result<Self, ProgramSpecError> {
        let mut registry = HashMap::with_capacity(forms.len());
        let mut ordered_names = Vec::with_capacity(forms.len());

        for form in forms {
            if registry.contains_key(&form.name) {
                return Err(ProgramSpecError::DuplicateApplicationForm(form.name));
            }
            ordered_names.push(form.name.clone());
            registry.insert(form.name.clone(), form);
        }

        Ok(Self {
            forms: registry,
            ordered_names,
        })
    }

    pub fn compile(
        &self,
        base_morph: &Morph,
        selected_forms: &[String],
    ) -> Result<ProgramExecutionPlan, ProgramSpecError> {
        let requested = if selected_forms.is_empty() {
            self.ordered_names.clone()
        } else {
            selected_forms.to_vec()
        };

        let mut seen = HashSet::with_capacity(requested.len());
        let mut selected = Vec::with_capacity(requested.len());
        let mut patterns = base_morph.patterns.clone();

        for form_name in requested {
            if !seen.insert(form_name.clone()) {
                return Err(ProgramSpecError::DuplicateFormSelection(form_name));
            }

            let form = self
                .forms
                .get(&form_name)
                .ok_or_else(|| ProgramSpecError::UnknownApplicationForm(form_name.clone()))?;

            selected.push(form_name);
            patterns.extend(form.patterns.iter().cloned());
        }

        Ok(ProgramExecutionPlan::new(selected, patterns))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramSpecError {
    DuplicateApplicationForm(String),
    UnknownApplicationForm(String),
    DuplicateFormSelection(String),
}

impl fmt::Display for ProgramSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateApplicationForm(name) => {
                write!(f, "duplicate application form registered: {name}")
            }
            Self::UnknownApplicationForm(name) => {
                write!(f, "unknown application form requested: {name}")
            }
            Self::DuplicateFormSelection(name) => {
                write!(f, "duplicate application form selected: {name}")
            }
        }
    }
}

impl Error for ProgramSpecError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shape() -> FormShape {
        FormShape::new(
            Shape::default(),
            Context::new(vec![], vec![], "kernel".to_string(), vec![]),
            Morph::new(vec!["base.normalize".to_string()]),
        )
    }

    fn app_form(name: &str, pattern: &str) -> ApplicationForm {
        ApplicationForm::new(
            name.to_string(),
            "graph-ml".to_string(),
            vec![format!("feature.{name}")],
            vec![pattern.to_string()],
            HashMap::new(),
        )
    }

    #[test]
    fn compiles_selected_forms_in_order() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![
                app_form("centrality", "algo.pagerank"),
                app_form("community", "algo.leiden"),
            ],
            vec!["community".to_string(), "centrality".to_string()],
        );

        let plan = program
            .compile_execution_plan()
            .expect("plan should compile");

        assert_eq!(plan.selected_forms, vec!["community", "centrality"]);
        assert_eq!(
            plan.patterns,
            vec!["base.normalize", "algo.leiden", "algo.pagerank"]
        );
    }

    #[test]
    fn compiles_all_forms_when_selection_empty() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![app_form("a", "op.a"), app_form("b", "op.b")],
            vec![],
        );

        let plan = program
            .compile_execution_plan()
            .expect("plan should compile");
        assert_eq!(plan.selected_forms, vec!["a", "b"]);
        assert_eq!(plan.patterns, vec!["base.normalize", "op.a", "op.b"]);
    }

    #[test]
    fn errors_on_unknown_selected_form() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::default(),
            vec![],
            vec![app_form("a", "op.a")],
            vec!["missing".to_string()],
        );

        let error = program
            .compile_execution_plan()
            .expect_err("missing form should fail");
        assert_eq!(
            error,
            ProgramSpecError::UnknownApplicationForm("missing".to_string())
        );
    }

    #[test]
    fn compiles_program_ir_for_compiler_boundary() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::new("gdsl.analytics".to_string(), None, HashMap::new()),
            vec![],
            vec![app_form("centrality", "algo.pagerank")],
            vec!["centrality".to_string()],
        );

        let compiler = ProgramCompiler::new();
        let ir = compiler
            .compile(&program)
            .expect("compile ir should succeed");

        assert_eq!(ir.program_name, "gdsl.analytics");
        assert_eq!(ir.selected_forms, vec!["centrality"]);
        assert_eq!(ir.opcodes, vec!["base.normalize", "algo.pagerank"]);

        let plan = ir.into_execution_plan();
        assert_eq!(plan.patterns, vec!["base.normalize", "algo.pagerank"]);
    }

    #[test]
    fn defines_program_features_before_compiler_ir() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::new("gdsl.analytics".to_string(), None, HashMap::new()),
            vec![],
            vec![app_form("centrality", "algo.pagerank")],
            vec!["centrality".to_string()],
        );

        let features = program
            .define_features()
            .expect("program features should be defined");

        assert_eq!(features.program_name, "gdsl.analytics");
        assert_eq!(features.selected_forms, vec!["centrality"]);
        assert_eq!(features.opcodes(), vec!["base.normalize", "algo.pagerank"]);
        assert!(features.features.iter().any(|feature| {
            feature.kind == ProgramFeatureKind::ApplicationForm && feature.value == "centrality"
        }));
        assert!(features.features.iter().any(|feature| {
            feature.kind == ProgramFeatureKind::SpecificationBinding
                && feature.value == "gdsl.analytics"
        }));
    }

    #[test]
    fn compiler_ir_conforms_to_program_features() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::new("gdsl.analytics".to_string(), None, HashMap::new()),
            vec![],
            vec![app_form("centrality", "algo.pagerank")],
            vec!["centrality".to_string()],
        );

        let compiler = ProgramCompiler::new();
        let features = compiler
            .define_features(&program)
            .expect("feature definition should succeed");
        let ir = ProgramCompileIr::from_features(features.clone());

        assert_eq!(ir.program_name, features.program_name);
        assert_eq!(ir.selected_forms, features.selected_forms);
        assert_eq!(ir.opcodes, features.opcodes());
    }

    #[test]
    fn specification_embedding_round_trips_through_attributes() {
        let mut attributes = HashMap::new();
        attributes.insert("other".to_string(), "keep".to_string());

        let spec = Specification::new(
            "logic.core".to_string(),
            Some("0.1.0".to_string()),
            attributes,
        )
        .with_embedding(&SpecificationEmbedding {
            kind: SpecificationKind::Gdsl,
            schema_ref: Some("@organon/logic/src/schema/index.ts".to_string()),
            cypher_fact_graph: Some("fact_store".to_string()),
            cypher_knowledge_graph: Some("knowledge_store".to_string()),
            cypher_labels: vec!["FormNode".to_string(), "DatasetNode".to_string()],
            cypher_relationship_types: vec!["MAPS_TO".to_string(), "SPEC_OF".to_string()],
            engine: Some("formdb".to_string()),
            client_sdk: Some("rustscript-polars-client-sdk".to_string()),
            interop_surface: Some("ts-json/mcp".to_string()),
            human_in_loop: true,
        });

        let embedding = spec.embedding();
        assert_eq!(embedding.kind, SpecificationKind::Gdsl);
        assert_eq!(
            embedding.schema_ref,
            Some("@organon/logic/src/schema/index.ts".to_string())
        );
        assert_eq!(embedding.cypher_fact_graph, Some("fact_store".to_string()));
        assert_eq!(
            embedding.cypher_knowledge_graph,
            Some("knowledge_store".to_string())
        );
        assert_eq!(
            embedding.cypher_labels,
            vec!["FormNode".to_string(), "DatasetNode".to_string()]
        );
        assert_eq!(
            embedding.cypher_relationship_types,
            vec!["MAPS_TO".to_string(), "SPEC_OF".to_string()]
        );
        assert_eq!(embedding.engine, Some("formdb".to_string()));
        assert_eq!(
            embedding.client_sdk,
            Some("rustscript-polars-client-sdk".to_string())
        );
        assert_eq!(embedding.interop_surface, Some("ts-json/mcp".to_string()));
        assert!(embedding.human_in_loop);
        assert_eq!(spec.attributes.get("other"), Some(&"keep".to_string()));
    }

    #[test]
    fn given_form_envelope_has_principle_and_principled_effect() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::new("gdsl.analytics".to_string(), None, HashMap::new()),
            vec![],
            vec![app_form("centrality", "algo.pagerank")],
            vec!["centrality".to_string()],
        );

        let given_forms = program
            .given_forms(Some("dataset://appearance/main".to_string()))
            .expect("given forms should compile");

        assert_eq!(
            GivenFormEnvelope::stage_order()[0],
            GivenFormStage::GdslPrinciple
        );
        assert_eq!(
            GivenFormEnvelope::stage_order()[1],
            GivenFormStage::SdslPrincipledEffect
        );
        assert_eq!(GivenFormEnvelope::stage_order()[2], GivenFormStage::Eval);

        assert_eq!(given_forms.len(), 1);
        assert_eq!(
            given_forms[0].principled_effect.application_form.name,
            "centrality"
        );
        assert_eq!(
            given_forms[0].principled_effect.appearance,
            Some("dataset://appearance/main".to_string())
        );
        assert_eq!(
            given_forms[0].principle.morph.patterns,
            vec!["base.normalize".to_string()]
        );
    }

    #[test]
    fn perfected_given_form_has_seven_layers() {
        let program = ProgramSpec::new(
            sample_shape(),
            Specification::new("gdsl.analytics".to_string(), None, HashMap::new()),
            vec![],
            vec![app_form("centrality", "algo.pagerank")],
            vec!["centrality".to_string()],
        );

        let envelope = program
            .given_forms(Some("dataset://appearance/main".to_string()))
            .expect("given forms should compile")
            .into_iter()
            .next()
            .expect("at least one given form envelope expected");

        let perfected = envelope.into_perfected();
        let layers = PerfectedGivenForm::layers();

        assert_eq!(layers.len(), 7);
        assert_eq!(layers[0], PerfectedGivenFormLayer::PrincipleShape);
        assert_eq!(layers[1], PerfectedGivenFormLayer::PrincipleContext);
        assert_eq!(layers[2], PerfectedGivenFormLayer::PrincipleMorph);
        assert_eq!(layers[3], PerfectedGivenFormLayer::EffectEntity);
        assert_eq!(layers[4], PerfectedGivenFormLayer::EffectProperty);
        assert_eq!(layers[5], PerfectedGivenFormLayer::EffectAspect);
        assert_eq!(layers[6], PerfectedGivenFormLayer::EvaluationMonadic);

        assert_eq!(perfected.entity, "entity::centrality");
        assert_eq!(perfected.property, "property::centrality");
        assert_eq!(perfected.aspect, "aspect::dataset://appearance/main");
    }

    #[test]
    fn pure_form_principle_round_trips_to_form_shape() {
        let principle = PureFormPrinciple::new(
            Shape::new(
                vec!["a".to_string()],
                vec![],
                HashMap::new(),
                HashMap::new(),
            ),
            Context::new(vec!["dep".to_string()], vec![], "kriya".to_string(), vec![]),
            Morph::new(vec!["op.project".to_string()]),
        );

        let form_shape = FormShape::from_principle(principle.clone());
        let roundtrip = form_shape.as_principle();

        assert_eq!(roundtrip, principle);
    }

    #[test]
    fn pure_form_principle_and_form_shape_from_impls_match() {
        let form_shape = sample_shape();
        let principle: PureFormPrinciple = form_shape.clone().into();
        let remanifested: FormShape = principle.into();

        assert_eq!(remanifested, form_shape);
    }
}
