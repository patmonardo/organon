use gds::shell::*;

#[test]
fn dataset_frame_namespace_enters_shell_with_program_features(
) -> Result<(), Box<dyn std::error::Error>> {
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["being", "essence", "concept"]),
        (moment: ["immediate", "mediated", "absolute"]),
    )?;

    let program = program_features(
        "gdsl.shell.bridge",
        ["ShellBridge"],
        [
            program_source("dataframe", "immediate-register"),
            program_reflection("dataset-mediated-register"),
            program_principle("dataframe-dataset-unity"),
            program_concept("ShellBridge"),
            program_procedure("interface-pureform"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-bridge-test")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("test")
        .source_io(io_path("memory://shell-bridge-test"))
        .into_shell_with_program_features(program);

    assert_eq!(shell.register_kind(), ShellRegister::Unified);
    assert_eq!(shell.pipeline_axis(), ShellPipeline::ModelFeaturePlan);
    assert_eq!(shell.algebra(), ShellAlgebra::ProgramFeature);

    let knowledge = shell
        .dataframe_knowledge()
        .expect("shell should know the DataFrame body");
    assert_eq!(knowledge.address().register, ShellRegister::Unified);
    assert_eq!(knowledge.columns(), &["term_id", "term", "moment"]);
    assert_eq!(knowledge.row_count(), 3);
    assert_eq!(knowledge.column_count(), 3);
    assert_eq!(knowledge.dataset_name(), Some("shell-bridge-test"));
    assert_eq!(knowledge.facets(), &["test"]);
    assert_eq!(knowledge.program_name(), Some("gdsl.shell.bridge"));

    let middle = shell
        .model_feature_plan_knowledge()
        .expect("shell should know the Dataset middle");
    assert_eq!(middle.address().pipeline, ShellPipeline::ModelFeaturePlan);
    assert_eq!(middle.model_name(), Some("shell-bridge-test"));
    assert_eq!(middle.selected_forms(), &["ShellBridge"]);
    assert_eq!(middle.principles(), &["principle::dataframe-dataset-unity"]);
    assert_eq!(middle.concepts(), &["concept::shellbridge"]);
    assert_eq!(middle.procedures(), &["procedure::interface-pureform"]);
    assert!(middle.plan_steps().contains(&"dataset.model".to_string()));
    assert!(middle.plan_steps().contains(&"dataset.feature".to_string()));
    assert!(middle.plan_steps().contains(&"dataset.plan".to_string()));
    assert!(middle.returns_to_concept());

    let semantic = shell.semantic_pipeline_knowledge();
    assert_eq!(semantic.address().pipeline, ShellPipeline::ModelFeaturePlan);
    assert!(semantic.frame_ready());
    assert!(semantic.middle_ready());
    assert!(semantic.semdataset_ready());
    assert!(semantic.pureform_return_ready());
    assert!(semantic.semantic_feature_count() > 0);
    assert!(semantic
        .semantic_feature_kinds()
        .contains(&"concept".to_string()));
    assert!(semantic
        .capabilities()
        .contains(&ShellSemanticCapability::FormIngestion));
    assert!(semantic
        .capabilities()
        .contains(&ShellSemanticCapability::LogicalFormParsing));
    assert!(semantic
        .capabilities()
        .contains(&ShellSemanticCapability::CorpusLanguagePairing));
    assert!(semantic
        .capabilities()
        .contains(&ShellSemanticCapability::LanguageModelFitting));
    assert!(semantic
        .capabilities()
        .contains(&ShellSemanticCapability::PrincipleGatedConceptReturn));
    assert!(semantic
        .evolution_path()
        .contains(&"frame.ready".to_string()));
    assert!(semantic
        .evolution_path()
        .contains(&"middle.model-feature-plan.ready".to_string()));
    assert!(semantic
        .evolution_path()
        .contains(&"semdataset.ready".to_string()));
    assert!(semantic
        .evolution_path()
        .contains(&"pureform.return.ready".to_string()));

    let projection = shell.validate_projection_trace();
    assert!(projection.is_valid());
    assert_eq!(
        projection.required_trace(),
        &[
            "Frame".to_string(),
            "Model:Feature::Plan".to_string(),
            "SemDataset".to_string(),
            "PureForm return".to_string()
        ]
    );
    assert_eq!(projection.observed_trace(), projection.required_trace());
    assert!(projection.missing_steps().is_empty());
    assert!(shell.is_projection_trace_valid());

    let learning = shell.learning_report();
    assert_eq!(learning.address().pipeline, ShellPipeline::ModelFeaturePlan);
    assert_eq!(learning.dataset_name(), Some("shell-bridge-test"));
    assert_eq!(learning.feature_count(), 5);
    assert_eq!(
        learning.gained_principles(),
        &["principle::dataframe-dataset-unity".to_string()]
    );
    assert_eq!(
        learning.gained_concepts(),
        &["concept::shellbridge".to_string()]
    );
    assert_eq!(
        learning.gained_procedures(),
        &["procedure::interface-pureform".to_string()]
    );
    assert!(learning.logical_feature_kinds().is_empty());
    assert!(learning.unresolved_forms().is_empty());
    assert!(learning.fundamental_nlp_graph_ready());
    assert!(!learning.mathematical_logic_ready());
    assert!(learning.kg_ready());
    assert_eq!(learning.readiness_score(), 100);

    let descriptor = shell.descriptor();
    assert!(descriptor.has_immediate_body());
    assert!(descriptor.has_mediated_body());
    assert!(descriptor.has_metapipeline());

    let pureform_return = descriptor.to_pure_form_return();
    assert_eq!(
        pureform_return.pipeline_kind(),
        ShellPipelineKind::DataPipeline
    );
    assert_eq!(pureform_return.address().register, ShellRegister::Unified);
    let principle = pureform_return.principle();
    assert!(principle
        .shape
        .required_fields
        .contains(&"term_id".to_string()));
    assert!(principle
        .shape
        .required_fields
        .contains(&"term".to_string()));
    assert!(principle
        .shape
        .required_fields
        .contains(&"moment".to_string()));
    assert!(principle
        .context
        .dependencies
        .contains(&"dataframe".to_string()));
    assert!(principle
        .context
        .dependencies
        .contains(&"dataset".to_string()));

    Ok(())
}

#[test]
fn projection_trace_reports_missing_steps_without_program_features(
) -> Result<(), Box<dyn std::error::Error>> {
    let table = tbl_def!(
        (term_id: i64 => [1, 2]),
        (term: ["being", "essence"]),
        (moment: ["immediate", "mediated"]),
    )?;

    let shell = GdsShell::from_dataframe(table);

    let projection = shell.validate_projection_trace();
    assert!(!projection.is_valid());
    assert_eq!(projection.observed_trace(), &["Frame".to_string()]);
    assert_eq!(
        projection.missing_steps(),
        &[
            "Model:Feature::Plan".to_string(),
            "SemDataset".to_string(),
            "PureForm return".to_string()
        ]
    );
    assert!(!shell.is_projection_trace_valid());

    let learning = shell.learning_report();
    assert_eq!(learning.dataset_name(), None);
    assert_eq!(learning.feature_count(), 0);
    assert!(learning.gained_principles().is_empty());
    assert!(learning.gained_concepts().is_empty());
    assert!(learning.gained_procedures().is_empty());
    assert!(learning.logical_feature_kinds().is_empty());
    assert_eq!(
        learning.unresolved_forms(),
        &[
            "principle".to_string(),
            "concept".to_string(),
            "procedure".to_string()
        ]
    );
    assert!(!learning.fundamental_nlp_graph_ready());
    assert!(!learning.mathematical_logic_ready());
    assert!(!learning.kg_ready());
    assert_eq!(learning.readiness_score(), 25);

    Ok(())
}
