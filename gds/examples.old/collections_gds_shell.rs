//! GDS Shell walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_gds_shell

use gds::form::ProgramFeatureKind;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== GDS Shell as compute graph ==");
    println!("DataFrame and Dataset are unified in Shell, then projected toward PureForm.");

    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["being", "essence", "concept"]),
        (moment: ["immediate", "mediated", "absolute"]),
    )?;

    let ds_frame = ds_frame(table)
        .named("shell-concept-demo")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("dialectical-cube")
        .source_io(io_path("memory://shell-concept-demo"));

    let program = program_features(
        "gdsl.shell.concept",
        ["ShellComputeGraph"],
        [
            program_source("dataframe", "immediate-register"),
            program_reflection("dataset-mediated-register"),
            program_principle("dataframe-dataset-unity"),
            program_concept("ShellComputeGraph"),
            program_feature(
                ProgramFeatureKind::Judgment,
                "all x. (human(x) -> mortal(x))",
                "doctrine://shell/judgment",
            ),
            program_feature(
                ProgramFeatureKind::Inference,
                "human(socrates) -> mortal(socrates)",
                "doctrine://shell/inference",
            ),
            program_procedure("interface-pureform"),
        ],
    );

    let shell = ds_frame
        .into_shell_with_program_features(program)
        .materialize_semdataset_from_texts(&[
            "Socrates is human.",
            "All humans are mortal.",
            "Socrates is mortal.",
        ])?;
    let descriptor = shell.descriptor();
    let pureform_return = descriptor.to_pure_form_return();
    let principle = pureform_return.principle();

    println!("shell register: {:?}", shell.register_kind());
    println!("shell pipeline axis: {:?}", shell.pipeline_axis());
    println!("shell algebra: {:?}", shell.algebra());
    println!("shell address: {:?}", descriptor.address());
    if let Some(knowledge) = shell.dataframe_knowledge() {
        println!("dataframe knowledge address: {:?}", knowledge.address());
        println!("dataframe knowledge columns: {:?}", knowledge.columns());
        println!(
            "dataframe knowledge shape: {} rows x {} cols",
            knowledge.row_count(),
            knowledge.column_count()
        );
        println!(
            "dataframe knowledge dataset: {:?}",
            knowledge.dataset_name()
        );
        println!(
            "dataframe knowledge artifact: {:?}",
            knowledge.artifact_kind()
        );
        println!(
            "dataframe knowledge program: {:?}",
            knowledge.program_name()
        );
    }
    if let Some(middle) = shell.model_feature_plan_knowledge() {
        println!("middle knowledge model: {:?}", middle.model_name());
        println!(
            "middle knowledge selected forms: {:?}",
            middle.selected_forms()
        );
        println!("middle knowledge principles: {:?}", middle.principles());
        println!("middle knowledge concepts: {:?}", middle.concepts());
        println!("middle knowledge procedures: {:?}", middle.procedures());
        println!("middle knowledge plan: {:?}", middle.plan_steps());
        println!("middle returns to concept: {}", middle.returns_to_concept());
    }
    let semantic = shell.semantic_pipeline_knowledge();
    println!("semantic address: {:?}", semantic.address());
    println!("semantic frame ready: {}", semantic.frame_ready());
    println!("semantic middle ready: {}", semantic.middle_ready());
    println!("semantic semdataset ready: {}", semantic.semdataset_ready());
    println!(
        "semantic pureform return ready: {}",
        semantic.pureform_return_ready()
    );
    println!(
        "semantic feature kinds: {:?} ({})",
        semantic.semantic_feature_kinds(),
        semantic.semantic_feature_count()
    );
    println!("semantic capabilities: {:?}", semantic.capabilities());
    println!("semantic evolution path: {:?}", semantic.evolution_path());
    let learning = shell.learning_report();
    println!("learning address: {:?}", learning.address());
    println!("learning dataset: {:?}", learning.dataset_name());
    println!("learning feature count: {}", learning.feature_count());
    println!(
        "learning gained principles: {:?}",
        learning.gained_principles()
    );
    println!("learning gained concepts: {:?}", learning.gained_concepts());
    println!(
        "learning gained procedures: {:?}",
        learning.gained_procedures()
    );
    println!(
        "learning logical feature kinds: {:?}",
        learning.logical_feature_kinds()
    );
    println!(
        "learning unresolved forms: {:?}",
        learning.unresolved_forms()
    );
    println!(
        "learning fundamental NLP graph ready: {}",
        learning.fundamental_nlp_graph_ready()
    );
    println!(
        "learning mathematical logic ready: {}",
        learning.mathematical_logic_ready()
    );
    println!("learning KG ready: {}", learning.kg_ready());
    println!("learning readiness score: {}", learning.readiness_score());
    if let Some(corpus) = shell.corpus_report() {
        println!("corpus materialized: {}", corpus.materialized());
        println!("corpus documents: {}", corpus.document_count());
        println!("corpus lm order: {}", corpus.lm_order());
        println!("corpus lm vocab size: {}", corpus.lm_vocab_size());
        println!("corpus semforms: {}", corpus.semform_count());
        println!("corpus parsed forms: {}", corpus.parsed_form_count());
    }
    let capability_map = shell.capability_map();
    println!("capability map address: {:?}", capability_map.address());
    for state in capability_map.states() {
        println!(
            "capability {}:{} available={} active={}",
            state.band().as_str(),
            state.capability().as_str(),
            state.available(),
            state.active()
        );
    }
    if let Some(seed) = shell.seed() {
        println!("seed columns: {:?}", seed.columns());
        println!("seed dtypes: {:?}", seed.dtypes());
        println!(
            "seed shape: {} rows x {} cols",
            seed.row_count(),
            seed.column_count()
        );
    }
    println!("has immediate body: {}", descriptor.has_immediate_body());
    println!("has mediated body: {}", descriptor.has_mediated_body());
    println!("has metapipeline: {}", descriptor.has_metapipeline());
    println!(
        "pureform return kind: {:?}",
        pureform_return.pipeline_kind()
    );
    println!("pureform return address: {:?}", pureform_return.address());
    println!("pureform fields: {:?}", principle.shape.required_fields);
    println!("pureform strategy: {}", principle.context.runtime_strategy);
    println!("pureform order: {:?}", principle.context.execution_order);
    println!("pureform morph: {:?}", principle.morph.patterns);

    if let Some(dataset_pipeline) = shell.dataset_pipeline() {
        let pipeline = dataset_pipeline
            .with_op(text_input("shell.input"))
            .with_op(text_encode("shell.encode"))
            .with_op(text_transform("shell.transform"))
            .with_op(text_output("shell.output"));

        println!("dataset execution ops: {}", pipeline.ops.len());
    }

    Ok(())
}
