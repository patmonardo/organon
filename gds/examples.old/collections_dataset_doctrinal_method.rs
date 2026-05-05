//! Dataset Doctrinal Method walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_doctrinal_method

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Doctrinal Method ==");
    println!("The example builds one path across Form, Shell, DataFrame, and Dataset.");
    println!("Each stage produces a concrete artifact and explains what Shell should know.");
    println!();

    let artifact_root = persistent_artifact_root();
    fs::create_dir_all(&artifact_root)?;
    println!("persistent artifact root: {}", artifact_root.display());
    println!();

    stage(
        0,
        "Immediate Beginning",
        "EssentialBeing",
        "The first object is a formed DataFrame body: Frame:Series::Expr.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3, 4]),
        (term: ["frame", "model", "language", "return"]),
        (moment: ["being", "essence", "concept", "principle"]),
        (artifact: ["DataFrame", "Dataset", "LM application", "PureForm"]),
        (exposition: [
            "formed immediate body",
            "mediated model-feature-plan middle",
            "compiled semantic application",
            "principle-gated return"
        ]),
    )?;
    println!("artifact: GDSDataFrame");
    println!("meaning: rows and columns are determinate enough for mediation.");
    println!(
        "shape: {} rows x {} columns",
        table.row_count(),
        table.column_count()
    );
    println!("columns: {:?}", table.column_names());
    let frame_path = artifact_root.join("00-immediate-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("persisted: {}", frame_path.display());
    println!();

    stage(
        1,
        "First Mediation",
        "Named Body",
        "Dataset names the same body without replacing DataFrame execution.",
    );
    let ds_frame = ds_frame(table)
        .named("doctrinal-method-demo")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("form-shell-dataframe-dataset")
        .facet("doctrinal-method")
        .source_io(io_path("memory://doctrinal-method-demo"));
    println!("artifact: DatasetDataFrameNameSpace");
    println!("meaning: the DataFrame body receives Dataset identity and artifact profile.");
    println!("shell expectation: preserve columns, dtypes, shape, name, and facets.");
    println!();

    stage(
        2,
        "Essence Middle",
        "Model:Feature::Plan",
        "The middle names, binds, and orders what the DataFrame body will retain.",
    );
    let program = program_features(
        "rustscript.doctrinal_method.semantic_dataset",
        ["DoctrinalMethodSemDataset"],
        [
            program_source(
                "doctrinal_frame",
                "fixtures/collections/doctrinal-method-pipeline/00-immediate-frame.csv",
            ),
            program_observation("term_id"),
            program_retain("term_id-term-moment-artifact-exposition"),
            program_logogenesis("feature-struct-algebra"),
            program_subfeature("tokenizer"),
            program_subfeature("tagger"),
            program_subfeature("parser"),
            program_subfeature("semantic-application"),
            program_concept("DoctrinalMethodSemDataset"),
            program_identity("term_id"),
            program_principle("doctrinal-method-governs-return"),
            program_judgment("feature-structs-compile-into-lm-applications"),
            program_infer(
                "semantic-readiness",
                "feature-struct-algebra-fit-over-corpus",
            ),
            program_procedure("persist-doctrinal-pipeline-artifacts"),
        ],
    );
    let compilation = DatasetToolChain::image_from_program_features(&program);
    compilation.validate()?;
    let materialized = compilation.materialize_artifact_datasets(&program.program_name)?;
    let artifacts_path = persist_dataset(&materialized.artifacts, &artifact_root, "02-artifacts")?;
    let relations_path = persist_dataset(&materialized.relations, &artifact_root, "02-relations")?;
    let properties_path =
        persist_dataset(&materialized.properties, &artifact_root, "02-properties")?;
    println!("artifact: ProgramFeatures from internal Rust DSL");
    println!("meaning: Feature and FeatStruct algebra has become an inspectable program image.");
    println!("program: {}", program.program_name);
    println!("selected forms: {:?}", program.selected_forms);
    println!("feature declarations: {}", program.features.len());
    println!(
        "artifact dataset rows: {}",
        materialized.artifacts.row_count()
    );
    println!(
        "relation dataset rows: {}",
        materialized.relations.row_count()
    );
    println!(
        "property dataset rows: {}",
        materialized.properties.row_count()
    );
    println!("persisted artifacts: {}", artifacts_path.display());
    println!("persisted relations: {}", relations_path.display());
    println!("persisted properties: {}", properties_path.display());
    println!();

    stage(
        3,
        "Program Commitment",
        "Logical Self-Declaration",
        "ProgramFeature commitments tell Shell what the Dataset intends to become.",
    );
    let shell = ds_frame
        .into_shell_with_program_features(program)
        .materialize_semdataset_from_texts(&[
            "A tokenizer segments the corpus into tokens.",
            "A tagger assigns roles to observed tokens.",
            "A parser compiles structured text into semantic form.",
            "A SemDataset carries evidence, application, and form state together.",
        ])?;
    if let Some(middle) = shell.model_feature_plan_knowledge() {
        println!("artifact: ShellModelFeaturePlanKnowledge");
        println!("meaning: Shell can now inspect the middle instead of trusting it blindly.");
        println!("model: {:?}", middle.model_name());
        println!("features: {:?}", middle.feature_declarations());
        println!("principles: {:?}", middle.principles());
        println!("concepts: {:?}", middle.concepts());
        println!("plan steps: {:?}", middle.plan_steps());
    }
    println!();

    stage(
        4,
        "Concept Formation",
        "Corpus + LM",
        "LM compiles Feature and FeatStruct commitments into a deployable application over evidence.",
    );
    if let Some(corpus) = shell.corpus_report() {
        println!("artifact: ShellCorpusReport");
        println!(
            "meaning: Corpus is extensional evidence; LM is the fitted intensional application."
        );
        println!("documents: {}", corpus.document_count());
        println!("lm order: {}", corpus.lm_order());
        println!("lm vocabulary size: {}", corpus.lm_vocab_size());
        println!("semforms: {}", corpus.semform_count());
        println!("parsed forms: {}", corpus.parsed_form_count());
    }
    println!();

    stage(
        5,
        "Concept Return",
        "SemDataset",
        "SemDataset holds evidence, compiled LM application, and logical form state together.",
    );
    let semantic = shell.semantic_pipeline_knowledge();
    let learning = shell.learning_report();
    println!("artifact: ShellSemanticPipelineKnowledge + ShellLearningReport");
    println!("meaning: Shell records whether the path has actually reached semantic readiness.");
    println!("frame ready: {}", semantic.frame_ready());
    println!("middle ready: {}", semantic.middle_ready());
    println!("semdataset ready: {}", semantic.semdataset_ready());
    println!(
        "pureform return ready: {}",
        semantic.pureform_return_ready()
    );
    println!("capabilities: {:?}", semantic.capabilities());
    println!("evolution path: {:?}", semantic.evolution_path());
    println!("learning readiness score: {}", learning.readiness_score());
    println!("unresolved forms: {:?}", learning.unresolved_forms());
    println!();

    stage(
        6,
        "Principle-Gated Return",
        "PureForm Principle",
        "The path is valid only when Shell can trace Frame to SemDataset to PureForm return.",
    );
    let descriptor = shell.descriptor();
    let pureform_return = descriptor.to_pure_form_return();
    let principle = pureform_return.principle();
    let trace = shell.validate_projection_trace();
    println!("artifact: ShellPipelineDescriptor -> ShellPureFormReturn -> PureFormPrinciple");
    println!(
        "meaning: Shell carries the doctrine as traceable return, not as external commentary."
    );
    println!("address: {:?}", descriptor.address());
    println!("required trace: {:?}", trace.required_trace());
    println!("observed trace: {:?}", trace.observed_trace());
    println!("missing trace steps: {:?}", trace.missing_steps());
    println!("trace valid: {}", trace.is_valid());
    println!("pureform fields: {:?}", principle.shape.required_fields);
    println!("pureform strategy: {}", principle.context.runtime_strategy);
    let return_path = artifact_root.join("06-pureform-return.txt");
    fs::write(
        &return_path,
        format!(
            "address: {:?}\nrequired_trace: {:?}\nobserved_trace: {:?}\ntrace_valid: {}\nfields: {:?}\nstrategy: {}\n",
            descriptor.address(),
            trace.required_trace(),
            trace.observed_trace(),
            trace.is_valid(),
            principle.shape.required_fields,
            principle.context.runtime_strategy,
        ),
    )?;
    println!("persisted: {}", return_path.display());
    println!();

    println!("== Shell Carrying Doctrine ==");
    if let Some(frame) = shell.dataframe_knowledge() {
        explain_artifact(
            "ShellDataFrameKnowledge",
            "Stage 0",
            "Shell knows the immediate register: schema, shape, and Dataset name.",
        );
        println!("  columns: {:?}", frame.columns());
        println!("  dtypes: {:?}", frame.dtypes());
        println!(
            "  shape: {} rows x {} columns",
            frame.row_count(),
            frame.column_count()
        );
    }
    explain_artifact(
        "ShellCapabilityMap",
        "All stages",
        "Shell knows which platform powers are merely available and which are active in this path.",
    );
    for state in shell.capability_map().states() {
        println!(
            "  {}:{} available={} active={}",
            state.band().as_str(),
            state.capability().as_str(),
            state.available(),
            state.active()
        );
    }

    let manifest_path = artifact_root.join("README.txt");
    fs::write(
        &manifest_path,
        artifact_manifest(
            &frame_path,
            &artifacts_path,
            &relations_path,
            &properties_path,
            &return_path,
        ),
    )?;
    println!("persistent manifest: {}", manifest_path.display());

    Ok(())
}

fn persistent_artifact_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/doctrinal-method-pipeline")
}

fn persist_dataset(
    dataset: &Dataset,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    dataset.table().write_csv(&path_string(&path))?;
    Ok(path)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/doctrinal-method-pipeline/{file_name}")
}

fn artifact_manifest(
    frame_path: &Path,
    artifacts_path: &Path,
    relations_path: &Path,
    properties_path: &Path,
    return_path: &Path,
) -> String {
    format!(
        "Dataset Doctrinal Method Persistent Fixture Pipeline\n\n\
         00 Immediate Beginning\n\
         artifact: {}\n\
         meaning: persisted DataFrame body; Shell can seed columns, dtypes, and shape from it.\n\n\
         02 Essence Middle\n\
         artifacts: {}\n\
         relations: {}\n\
         properties: {}\n\
         meaning: ProgramFeatures authored in the internal Rust DSL have become durable Dataset artifact tables.\n\n\
         06 Principle-Gated Return\n\
         artifact: {}\n\
         meaning: Shell projection trace and PureForm return are written as the durable return witness.\n",
        fixture_path(frame_path),
        fixture_path(artifacts_path),
        fixture_path(relations_path),
        fixture_path(properties_path),
        fixture_path(return_path),
    )
}

fn stage(number: u8, name: &str, function: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("function: {function}");
    println!("doctrine: {doctrine}");
}

fn explain_artifact(name: &str, stage: &str, meaning: &str) {
    println!("artifact: {name}");
    println!("stage: {stage}");
    println!("meaning: {meaning}");
}
