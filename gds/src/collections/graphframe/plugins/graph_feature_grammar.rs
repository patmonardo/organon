//! Graph feature grammar plugin adapter.

use polars::df;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::DatasetArtifactKind;
use crate::collections::dataset::core::DatasetArtifactProfile;
use crate::collections::dataset::core::DatasetLanguagePlugin;
use crate::collections::dataset::core::DatasetPluginDiagnostic;
use crate::collections::dataset::core::DatasetPluginError;
use crate::collections::dataset::core::DatasetPluginErrorClass;
use crate::collections::dataset::core::DatasetPluginValidationReport;
use crate::collections::dataset::core::DatasetPluginValidationRequest;
use crate::collections::dataset::core::{
    DatasetPluginArtifact, DatasetPluginCapabilities, DatasetPluginOperation, DatasetPluginRequest,
    DatasetPluginResponse,
};
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

    fn capabilities(&self) -> DatasetPluginCapabilities {
        DatasetPluginCapabilities::new(vec![
            DatasetPluginOperation::Validate,
            DatasetPluginOperation::Compile,
            DatasetPluginOperation::Materialize,
        ])
        .with_consumes(vec![DatasetArtifactKind::Table])
        .with_produces(vec![DatasetArtifactKind::FeatureMap])
        .with_polars_lazy(true)
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

    fn execute(
        &self,
        request: &DatasetPluginRequest,
    ) -> Result<DatasetPluginResponse, DatasetPluginError> {
        let report = self.validate(&request.validation)?;
        let mut response = DatasetPluginResponse::new(request.operation, report)
            .with_provenance("plugin_id", self.plugin_id())
            .with_provenance("workflow_id", request.validation.workflow_id.clone());

        if !response.report.passed || matches!(request.operation, DatasetPluginOperation::Validate)
        {
            return Ok(response);
        }

        let form = request
            .validation
            .payload
            .downcast_ref::<GraphFeatureGrammarForm>()
            .expect("successful validation establishes GraphFeatureGrammarForm payload");
        let checked = validate_graph_feature_grammar(form.clone()).map_err(|error| {
            DatasetPluginError::new(DatasetPluginErrorClass::PluginExecution, error.message)
        })?;
        let artifact_id = format!("graph-feature-grammar:{}", checked.form().name);
        let table = df!(
            "grammar_name" => [checked.form().name.as_str()],
            "grammar_version" => [checked.form().version.as_str()],
            "rule_graph_digest" => [checked.rule_graph_digest().to_string()],
        )
        .map(GDSDataFrame::new)
        .map_err(|error| {
            DatasetPluginError::new(
                DatasetPluginErrorClass::PluginExecution,
                format!("failed to materialize graph feature grammar: {error}"),
            )
        })?;

        response = response.with_artifact(DatasetPluginArtifact::new(
            artifact_id,
            table,
            DatasetArtifactProfile::new(DatasetArtifactKind::FeatureMap)
                .with_facet("graph-feature-grammar"),
        ));
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::GraphFeatureGrammarPlugin;
    use super::GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID;

    use crate::collections::dataframe::GDSDataFrame;
    use crate::collections::dataset::concept_bus::{
        ConceptEnvelope, ConceptJudgment, ConceptRevision,
    };
    use crate::collections::dataset::core::DatasetLanguagePlugin;
    use crate::collections::dataset::core::DatasetPluginPayload;
    use crate::collections::dataset::core::DatasetPluginValidationRequest;
    use crate::collections::dataset::core::{
        DatasetConcept, DatasetEmpiricalConcept, DatasetOrb, DatasetPluginOperation,
        DatasetPluginRequest, DatasetRationalConcept,
    };
    use crate::collections::dataset::oculus::OculusHandle;
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
                vec![GraphFeatureAddress::new(
                    GraphFeatureStratum::Edge,
                    "weight",
                )],
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
        assert_eq!(
            report.facts.get("grammar_name"),
            Some(&"citation_graph".to_string())
        );
    }

    #[test]
    fn plugin_reports_failure_for_invalid_form() {
        let plugin = GraphFeatureGrammarPlugin::new();
        let invalid = valid_form().with_derivation(GraphFeatureDerivationRule::new(
            vec![GraphFeatureAddress::new(
                GraphFeatureStratum::Edge,
                "weight",
            )],
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
        assert_eq!(
            report.facts.get("error_class"),
            Some(&"TypeCollapse".to_string())
        );
    }

    #[test]
    fn oculus_mediates_graph_concept_and_commits_feature_artifact() {
        let body = GDSDataFrame::new(df!("edge" => ["a->b"]).unwrap());
        let concept = DatasetConcept::new(
            DatasetRationalConcept::new(
                "model:citation-graph",
                vec!["feature:density".into()],
                "plan:derive-density",
            ),
            DatasetEmpiricalConcept::new(
                "corpus:citation-edges",
                "lm:graph-distribution",
                "logic:graph-feature-laws",
            ),
        );
        let validation = DatasetPluginValidationRequest::new(
            GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID,
            "workflow.graph.oculus",
            DatasetPluginPayload::typed(valid_form()),
        );
        let request = DatasetPluginRequest::new(
            super::GRAPH_FEATURE_GRAMMAR_PLUGIN_ID,
            DatasetPluginOperation::Compile,
            concept,
            validation,
        )
        .with_output_artifact("graph-feature-grammar:citation_graph");

        let eye = OculusHandle::new();
        let mediation = eye
            .mediate_dataframe(
                "graph-world",
                body,
                std::sync::Arc::new(GraphFeatureGrammarPlugin::new()),
                &request,
            )
            .expect("Oculus DataFrame mediation");

        assert!(mediation.evaluation().response().report.passed);
        assert_eq!(mediation.evaluation().mediation_trace().len(), 3);
        assert_eq!(
            mediation.output().column_names(),
            vec!["grammar_name", "grammar_version", "rule_graph_digest"]
        );
        assert!(mediation
            .dataset()
            .artifact("graph-feature-grammar:citation_graph")
            .is_some());
    }

    #[test]
    fn concept_bus_repeatedly_mediates_one_dataset_world() {
        let body = GDSDataFrame::new(df!("edge" => ["a->b"]).unwrap());
        let mut dataset = DatasetOrb::named("graph-world", body);
        let eye = OculusHandle::new();
        eye.register_plugin(
            &mut dataset,
            std::sync::Arc::new(GraphFeatureGrammarPlugin::new()),
        )
        .expect("register plugin once in the Dataset world");

        let concept = DatasetConcept::new(
            DatasetRationalConcept::new(
                "model:citation-graph",
                vec!["feature:density".into()],
                "plan:derive-density",
            ),
            DatasetEmpiricalConcept::new(
                "corpus:citation-edges",
                "lm:graph-distribution",
                "logic:graph-feature-laws",
            ),
        );
        let compile = DatasetPluginRequest::new(
            super::GRAPH_FEATURE_GRAMMAR_PLUGIN_ID,
            DatasetPluginOperation::Compile,
            concept.clone(),
            DatasetPluginValidationRequest::new(
                GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID,
                "workflow.graph.compile",
                DatasetPluginPayload::typed(valid_form()),
            ),
        )
        .with_output_artifact("graph-feature-grammar:citation_graph");
        let compile_receipt = eye
            .mediate(
                &mut dataset,
                ConceptEnvelope::new("concept.graph", compile).with_revision(3_u64),
            )
            .expect("compile over Concept Bus");

        assert_eq!(compile_receipt.correlation_id().as_str(), "concept.graph");
        assert_eq!(compile_receipt.revision(), ConceptRevision::new(3));
        assert_eq!(compile_receipt.judgment(), ConceptJudgment::Admitted);
        assert_eq!(
            compile_receipt.produced_artifact_ids(),
            &["graph-feature-grammar:citation_graph".to_string()]
        );

        let validate = DatasetPluginRequest::new(
            super::GRAPH_FEATURE_GRAMMAR_PLUGIN_ID,
            DatasetPluginOperation::Validate,
            concept,
            DatasetPluginValidationRequest::new(
                GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID,
                "workflow.graph.validate",
                DatasetPluginPayload::typed(valid_form()),
            ),
        );
        let validate_receipt = eye
            .mediate(
                &mut dataset,
                ConceptEnvelope::new("concept.graph", validate).with_revision(4_u64),
            )
            .expect("validate again over the same Concept Bus");

        assert!(validate_receipt.judgment().is_admitted());
        assert_eq!(validate_receipt.revision(), ConceptRevision::new(4));
        assert!(validate_receipt.produced_artifact_ids().is_empty());
        assert_eq!(dataset.plugins().len(), 1);
        assert!(dataset
            .artifact("graph-feature-grammar:citation_graph")
            .is_some());
    }
}
