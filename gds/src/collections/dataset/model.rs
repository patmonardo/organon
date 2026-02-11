//! Dataset model abstractions (NLTK parity).
//!
//! A model is a schema-to-schema transform over the dataset graph. It consumes
//! selected views (tokens, tags, parses, trees) and emits structured updates
//! that remain anchored to the schema.
//!
//! Compiler note:
//! - FeatureSpace and ModelSpace are primary compilation targets.
//! - Plan is the executable graph that gives those targets their runtime shape.

use std::collections::BTreeMap;

use crate::collections::dataset::featstruct::{FeatPath, FeatValue};
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
