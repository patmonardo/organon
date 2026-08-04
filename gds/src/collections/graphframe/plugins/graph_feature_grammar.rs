//! Graph feature grammar plugin adapter.

use crate::collections::dataset::core::DatasetLanguagePlugin;
use crate::collections::dataset::core::DatasetPluginDiagnostic;
use crate::collections::dataset::core::DatasetPluginError;
use crate::collections::dataset::core::DatasetPluginErrorClass;
use crate::collections::dataset::core::DatasetPluginValidationReport;
use crate::collections::dataset::core::DatasetPluginValidationRequest;
use crate::collections::graphframe::feature_grammar::validate_graph_feature_grammar;
use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;

pub const GRAPH_FEATURE_GRAMMAR_PLUGIN_ID: &str = "graphframe.graph_feature_grammar";
pub const GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID: &str = "graph_feature_grammar";

#[derive(Default)]
pub struct GraphFeatureGrammarPlugin;

impl GraphFeatureGrammarPlugin {
    pub fn new() -> Self {
        Self
    }
}

impl DatasetLanguagePlugin for GraphFeatureGrammarPlugin {
    fn plugin_id(&self) -> &'static str {
        GRAPH_FEATURE_GRAMMAR_PLUGIN_ID
    }

    fn language_id(&self) -> &'static str {
        GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID
    }

    fn validate(
        &self,
        request: &DatasetPluginValidationRequest,
    ) -> Result<DatasetPluginValidationReport, DatasetPluginError> {
        if request.language_id != self.language_id() {
            return Err(DatasetPluginError::new(
                DatasetPluginErrorClass::InvalidPayload,
                format!(
                    "plugin {} expected language_id={}, got {}",
                    self.plugin_id(),
                    self.language_id(),
                    request.language_id
                ),
            ));
        }

        let form = request
            .payload
            .downcast_ref::<GraphFeatureGrammarForm>()
            .ok_or_else(|| {
                DatasetPluginError::new(
                    DatasetPluginErrorClass::InvalidPayload,
                    "payload is not GraphFeatureGrammarForm",
                )
            })?;

        match validate_graph_feature_grammar(form.clone()) {
            Ok(checked) => Ok(DatasetPluginValidationReport::success(
                self.plugin_id(),
                "graph feature grammar is valid",
            )
            .with_fact("workflow", request.workflow_id.clone())
            .with_fact("grammar_name", checked.form().name.clone())
            .with_fact("grammar_version", checked.form().version.clone())
            .with_fact("rule_graph_digest", checked.rule_graph_digest().to_string())),
            Err(err) => Ok(DatasetPluginValidationReport::failure(
                self.plugin_id(),
                "graph feature grammar validation failed",
            )
            .with_diagnostic(DatasetPluginDiagnostic::error(
                err.class.as_str(),
                err.message.clone(),
            ))
            .with_fact("workflow", request.workflow_id.clone())
            .with_fact("error_class", err.class.as_str().to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GraphFeatureGrammarPlugin;
    use super::GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID;

    use crate::collections::dataset::core::DatasetLanguagePlugin;
    use crate::collections::dataset::core::DatasetPluginPayload;
    use crate::collections::dataset::core::DatasetPluginValidationRequest;
    use crate::collections::graphframe::feature_grammar::GraphFeatureAddress;
    use crate::collections::graphframe::feature_grammar::GraphFeatureCardinality;
    use crate::collections::graphframe::feature_grammar::GraphFeatureDerivationKind;
    use crate::collections::graphframe::feature_grammar::GraphFeatureDerivationRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;
    use crate::collections::graphframe::feature_grammar::GraphFeatureRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
    use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;

    fn valid_form() -> GraphFeatureGrammarForm {
        GraphFeatureGrammarForm::new("citation_graph", "v1")
            .with_feature_rule(GraphFeatureRule::new(
                GraphFeatureStratum::Graph,
                "density",
                GraphFeatureValueType::Scalar,
                true,
                GraphFeatureCardinality::One,
            ))
            .with_feature_rule(GraphFeatureRule::new(
                GraphFeatureStratum::Edge,
                "weight",
                GraphFeatureValueType::Scalar,
                false,
                GraphFeatureCardinality::One,
            ))
            .with_derivation(GraphFeatureDerivationRule::new(
                vec![GraphFeatureAddress::new(GraphFeatureStratum::Edge, "weight")],
                GraphFeatureAddress::new(GraphFeatureStratum::Graph, "density"),
                GraphFeatureDerivationKind::Aggregate,
                true,
            ))
    }

    #[test]
    fn plugin_reports_success_for_valid_form() {
        let plugin = GraphFeatureGrammarPlugin::new();
        let request = DatasetPluginValidationRequest::new(
            GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID,
            "workflow.graph.alpha",
            DatasetPluginPayload::typed(valid_form()),
        );

        let report = plugin.validate(&request).expect("plugin validation call");
        assert!(report.passed);
        assert_eq!(report.facts.get("grammar_name"), Some(&"citation_graph".to_string()));
    }

    #[test]
    fn plugin_reports_failure_for_invalid_form() {
        let plugin = GraphFeatureGrammarPlugin::new();
        let invalid = valid_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![GraphFeatureAddress::new(GraphFeatureStratum::Edge, "weight")],
            GraphFeatureAddress::new(GraphFeatureStratum::Node, "pagerank"),
            GraphFeatureDerivationKind::Aggregate,
            true,
        ));

        let request = DatasetPluginValidationRequest::new(
            GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID,
            "workflow.graph.alpha",
            DatasetPluginPayload::typed(invalid),
        );

        let report = plugin.validate(&request).expect("plugin validation call");
        assert!(!report.passed);
        assert_eq!(report.facts.get("error_class"), Some(&"TypeCollapse".to_string()));
    }
}
