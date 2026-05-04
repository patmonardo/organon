//! `Model` — partial valuations against a Feature schema (R4 in the doctrine).
//!
//! See `gds/doc/SEMANTIC-DATASET.md` (Five-Fold Synthesis), Root
//! Object **R4** and Principle **P3** (`Model:Feature`). NLTK Ch9 / Ch10 are
//! the reference texts; the DRS, the FeatStruct, and the Valuation are all
//! Models in this sense.
//!
//! ## Two senses of "Model" live in this crate
//!
//! 1. **Compute Model** — the [`Model`] trait below (Tagger / Parser /
//!    Segmenter / …). A *transform* that consumes a `Dataset` and produces
//!    a `ModelDelta`. This is the legacy/runtime sense.
//! 2. **R4 Model = Valuation** —
//!    [`crate::collections::dataset::valuation::Valuation`] and
//!    [`crate::collections::dataset::valuation::ValuationFrame`]. A *partial
//!    map* from Feature names to values, considered as satisfying a Feature
//!    schema. This is what doctrine R4 names. Has no `apply` method;
//!    carries data, not behavior.
//!
//! A compute Model produces and consumes Valuations. The two senses are
//! complementary, not in conflict; see `valuation.rs` for the R4 access
//! object and the Document/Valuation meeting-point.
//!
//! A `Model` is a *partial valuation*: a row, a region, or a sub-frame
//! considered as satisfying a `Feature` (R5) schema. A Model can exist
//! without ever being grounded in bytes — that is what distinguishes it from
//! a `Document` (R2), which carries a distinguished `source : Source`
//! Feature and lives on the extensional/evidentiary side. They meet at
//! exactly one place; see the §"Document / Model meeting-point" in the
//! doctrine note.
//!
//! Contract this module owes the kernel:
//!
//! - **Three** primitive ops, no more: `unify`, `subsumes`, `extend`. All
//!   higher-level Model operations (delta application, schema lifting,
//!   composition) are expressed in terms of these three.
//! - A Model is read against a Feature schema; the schema is part of how
//!   the Model is interpreted, not part of the Model's bytes.
//! - The current `ModelKind` / `ModelView` / `ModelSpec` machinery
//!   describes *kinds of Model*, not a parallel taxonomy. They classify R4
//!   instances; they do not replace them.
//!
//! Compiler note (preserved):
//! - FeatureSpace and ModelSpace are primary compilation targets.
//! - `Plan` (R6) is the executable graph that gives those targets their
//!   runtime shape; see `crate::collections::dataset::plan`.

pub mod exec;
pub mod image;
pub mod prep;

pub use exec::{
    execute_essence, execute_feature, execute_marked, ExecutedFeature, Execution, ExecutionAction,
};
pub use image::{realize_from_essence, realize_image, ImageOptions};
pub use prep::{
    prepare_model, FeatureMark, MarkRequirement, MarkedFeature, Modality, ModelEssence,
    ModelPrepExt, PreparationError, PreparationReport, PreparationStep,
};

use std::collections::BTreeMap;

use crate::collections::dataset::featstruct::{FeatPath, FeatStruct, FeatStructSet, FeatValue};
use crate::collections::dataset::parse::Parse;
use crate::collections::dataset::schema::{ModelSchema, SymbolTable};
use crate::collections::dataset::tag::Tag;
use crate::collections::dataset::tree::TreeValue;
use crate::collections::dataset::Dataset;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ModelId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelKind {
    Tagger,
    Parser,
    Segmenter,
    LanguageModel,
    Semantic,
    FeatureModel,
    Composite,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelView {
    Tokens,
    Tags,
    Parses,
    Trees,
    Features,
    Graph,
    Mixed(Vec<ModelView>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpec {
    pub id: ModelId,
    pub kind: ModelKind,
    pub input: ModelView,
    pub output: ModelView,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelContext {
    pub options: BTreeMap<String, FeatValue>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModelDelta {
    pub tags: Vec<Tag>,
    pub parses: Vec<Parse>,
    pub trees: Vec<TreeValue>,
    pub attributes: Vec<ModelAttributeUpdate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelAttributeUpdate {
    pub path: FeatPath,
    pub value: FeatValue,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ModelResult {
    pub delta: ModelDelta,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelState {
    pub data: BTreeMap<String, FeatValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelScore {
    pub metrics: BTreeMap<String, FeatValue>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelReport {
    pub score: ModelScore,
    pub notes: Vec<String>,
}

pub trait Model {
    fn spec(&self) -> &ModelSpec;

    fn apply(&self, dataset: &Dataset, ctx: &ModelContext) -> ModelResult;

    /// Essential relations the model brings to Box 1 preparation.
    ///
    /// Override to seed [`crate::collections::dataset::model::prep::prepare_model`]
    /// with the model's accumulated [`FeatStruct`] constraints. The default
    /// is the empty set (all features will be treated as having no prior
    /// essence to unify against).
    fn essential_constraints(&self) -> FeatStructSet {
        FeatStructSet::new()
    }

    /// Optional seed essence for Box 1 preparation.
    ///
    /// Distinct from [`Self::essential_constraints`]: returns a single
    /// [`FeatStruct`] (rather than the reentrance set) that becomes the
    /// initial accumulated essence in
    /// [`crate::collections::dataset::model::prep::prepare_model`].
    fn essence_seed(&self) -> Option<FeatStruct> {
        None
    }

    fn fit(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelState {
        ModelState::default()
    }

    fn train(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelState {
        ModelState::default()
    }

    fn update(&self, _state: &ModelState, _dataset: &Dataset, _ctx: &ModelContext) -> ModelState {
        ModelState::default()
    }

    fn score(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelScore {
        ModelScore::default()
    }

    fn evaluate(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelReport {
        ModelReport::default()
    }
}

/// A minimal registry for dataset models.
///
/// This is intentionally small and can act as the model/feature anchor for
/// Dataset DataOps when they need to lower into DataFrame UDT expressions.
#[derive(Default)]
pub struct ModelSpace {
    models: BTreeMap<ModelId, Box<dyn Model + Send + Sync>>,
    schema: ModelSchema,
    symbols: SymbolTable,
}

impl ModelSpace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn schema(&self) -> &ModelSchema {
        &self.schema
    }

    pub fn schema_mut(&mut self) -> &mut ModelSchema {
        &mut self.schema
    }

    pub fn with_schema(mut self, schema: ModelSchema) -> Self {
        self.schema = schema;
        self
    }

    pub fn symbols(&self) -> &SymbolTable {
        &self.symbols
    }

    pub fn symbols_mut(&mut self) -> &mut SymbolTable {
        &mut self.symbols
    }

    pub fn with_symbols(mut self, symbols: SymbolTable) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn register<M>(&mut self, model: M) -> Option<Box<dyn Model + Send + Sync>>
    where
        M: Model + Send + Sync + 'static,
    {
        let id = model.spec().id.clone();
        self.models.insert(id, Box::new(model))
    }

    pub fn get(&self, id: &ModelId) -> Option<&dyn Model> {
        self.models.get(id).map(|m| m.as_ref() as &dyn Model)
    }

    pub fn ids(&self) -> Vec<&ModelId> {
        self.models.keys().collect()
    }

    pub fn list(&self) -> Vec<&ModelSpec> {
        self.models.values().map(|m| m.spec()).collect()
    }
}

#[derive(Debug, Clone)]
pub struct NoOpTagger {
    spec: ModelSpec,
}

impl NoOpTagger {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            spec: ModelSpec {
                id: ModelId(id.into()),
                kind: ModelKind::Tagger,
                input: ModelView::Tokens,
                output: ModelView::Tags,
                description: Some("No-op tagger model".to_string()),
            },
        }
    }
}

impl Model for NoOpTagger {
    fn spec(&self) -> &ModelSpec {
        &self.spec
    }

    fn apply(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelResult {
        ModelResult::default()
    }
}

#[derive(Debug, Clone)]
pub struct NoOpParser {
    spec: ModelSpec,
}

impl NoOpParser {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            spec: ModelSpec {
                id: ModelId(id.into()),
                kind: ModelKind::Parser,
                input: ModelView::Tokens,
                output: ModelView::Parses,
                description: Some("No-op parser model".to_string()),
            },
        }
    }
}

impl Model for NoOpParser {
    fn spec(&self) -> &ModelSpec {
        &self.spec
    }

    fn apply(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelResult {
        ModelResult::default()
    }
}

#[derive(Debug, Clone)]
pub struct NoOpLanguageModel {
    spec: ModelSpec,
}

impl NoOpLanguageModel {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            spec: ModelSpec {
                id: ModelId(id.into()),
                kind: ModelKind::LanguageModel,
                input: ModelView::Tokens,
                output: ModelView::Features,
                description: Some("No-op language model".to_string()),
            },
        }
    }
}

impl Model for NoOpLanguageModel {
    fn spec(&self) -> &ModelSpec {
        &self.spec
    }

    fn apply(&self, _dataset: &Dataset, _ctx: &ModelContext) -> ModelResult {
        ModelResult::default()
    }
}
