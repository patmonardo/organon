//! Model namespace for dataset-level DSL authoring.

use crate::collections::dataset::model::ModelContext;
use crate::collections::dataset::model::ModelId;
use crate::collections::dataset::model::ModelKind;
use crate::collections::dataset::model::ModelSpace;
use crate::collections::dataset::model::ModelSpec;
use crate::collections::dataset::model::ModelView;
use crate::collections::dataset::model::NoOpLanguageModel;
use crate::collections::dataset::model::NoOpParser;
use crate::collections::dataset::model::NoOpTagger;

#[derive(Debug, Clone, Default)]
pub struct ModelNs;

impl ModelNs {
    pub fn id(id: impl Into<String>) -> ModelId {
        ModelId(id.into())
    }

    pub fn kind_custom(kind: impl Into<String>) -> ModelKind {
        ModelKind::Custom(kind.into())
    }

    pub fn view_mixed(views: impl IntoIterator<Item = ModelView>) -> ModelView {
        ModelView::Mixed(views.into_iter().collect())
    }

    pub fn spec(
        id: impl Into<String>,
        kind: ModelKind,
        input: ModelView,
        output: ModelView,
    ) -> ModelSpec {
        ModelSpec {
            id: Self::id(id),
            kind,
            input,
            output,
            description: None,
        }
    }

    pub fn described_spec(
        id: impl Into<String>,
        kind: ModelKind,
        input: ModelView,
        output: ModelView,
        description: impl Into<String>,
    ) -> ModelSpec {
        ModelSpec {
            id: Self::id(id),
            kind,
            input,
            output,
            description: Some(description.into()),
        }
    }

    pub fn context() -> ModelContext {
        ModelContext::default()
    }

    pub fn space() -> ModelSpace {
        ModelSpace::new()
    }

    pub fn noop_tagger(id: impl Into<String>) -> NoOpTagger {
        NoOpTagger::new(id)
    }

    pub fn noop_parser(id: impl Into<String>) -> NoOpParser {
        NoOpParser::new(id)
    }

    pub fn noop_language_model(id: impl Into<String>) -> NoOpLanguageModel {
        NoOpLanguageModel::new(id)
    }

    pub fn tagger_spec(id: impl Into<String>) -> ModelSpec {
        Self::spec(id, ModelKind::Tagger, ModelView::Tokens, ModelView::Tags)
    }

    pub fn parser_spec(id: impl Into<String>) -> ModelSpec {
        Self::spec(id, ModelKind::Parser, ModelView::Tokens, ModelView::Parses)
    }

    pub fn language_model_spec(id: impl Into<String>) -> ModelSpec {
        Self::spec(
            id,
            ModelKind::LanguageModel,
            ModelView::Tokens,
            ModelView::Features,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::collections::dataset::model::Model;

    #[test]
    fn test_model_ns_builds_specs_and_noop_models() {
        let spec = ModelNs::tagger_spec("pos");
        assert_eq!(spec.id.0, "pos");
        assert!(matches!(spec.kind, ModelKind::Tagger));
        assert!(matches!(spec.input, ModelView::Tokens));
        assert!(matches!(spec.output, ModelView::Tags));

        let tagger = ModelNs::noop_tagger("noop-pos");
        assert_eq!(tagger.spec().id.0, "noop-pos");
        assert!(matches!(tagger.spec().kind, ModelKind::Tagger));
    }
}
