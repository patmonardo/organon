//! Dataset Model:Feature::Plan doctrinal fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_model_feature_plan

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Model:Feature::Plan ==");
    println!(
        "Doctrine exemplar 027: the Essence middle where DataFrame body becomes Dataset form."
    );
    println!("This example uses the internal Rust DSL and persists every important artifact.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Immediate Beginning",
        "DataFrame body supplies Frame:Series::Expr as EssentialBeing.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3, 4]),
        (term: ["frame", "model", "feature", "plan"]),
        (moment: ["being", "essence", "essence", "essence"]),
        (role: ["body", "name", "binding", "retention"]),
        (exposition: [
            "formed immediate body",
            "model names the body",
            "feature binds addressable determinations",
            "plan retains and orders the path"
        ]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("artifact: GDSDataFrame");
    println!(
        "shape: {} rows x {} columns",
        table.row_count(),
        table.column_count()
    );
    println!("columns: {:?}", table.column_names());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "First Mediation",
        "Dataset gives the same body a name, artifact profile, and fixture address.",
    );
    let ds_frame = ds_frame(table)
        .named("dataset-model-feature-plan")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:027-model-feature-plan-middle")
        .facet("internal-dsl")
        .source_io(io_path(
            "fixtures/collections/dataset/dataset_model_feature_plan/00-frame.csv",
        ));
    println!("artifact: DatasetDataFrameNameSpace");
    println!("meaning: DataFrame execution remains intact while Dataset starts mediation.");
    println!();

    stage(
        2,
        "Essence Middle",
        "Model, Feature, and Plan are authored as internal Rust DSL ProgramFeatures.",
    );
    let program = program_features(
        "rustscript.dataset.model_feature_plan",
        ["ModelFeaturePlanMiddle"],
        [
            program_source(
                "frame",
                "fixtures/collections/dataset/dataset_model_feature_plan/00-frame.csv",
            ),
            program_observation("term_id"),
            program_retain("term_id-term-moment-role-exposition"),
            program_reflection("model-feature-plan-middle"),
            program_mark("model", "term-as-named-body"),
            program_mark("feature", "role-as-addressable-determination"),
            program_mark("plan", "retained-and-ordered-mediation"),
            program_concept("ModelFeaturePlanMiddle"),
            program_principle("essence-middle-requires-mutual-ground"),
            program_judgment("model-feature-plan-derive-together"),
            program_middle("frame-as-essentialbeing"),
            program_infer("semantic-readiness", "plan-opens-toward-semdataset"),
            program_procedure("persist-model-feature-plan-artifacts"),
        ],
    );
    let compilation = DatasetToolChain::image_from_program_features(&program);
    compilation.validate()?;
    let artifacts = compilation.materialize_artifact_datasets(&program.program_name)?;
    let artifact_path = persist_dataset(&artifacts.artifacts, &fixture_root, "02-artifacts")?;
    let relation_path = persist_dataset(&artifacts.relations, &fixture_root, "02-relations")?;
    let property_path = persist_dataset(&artifacts.properties, &fixture_root, "02-properties")?;
    println!("artifact: ProgramFeatures -> DatasetCompilation");
    println!("program: {}", program.program_name);
    println!("selected forms: {:?}", program.selected_forms);
    println!("feature count: {}", program.features.len());
    println!("artifact rows: {}", artifacts.artifacts.row_count());
    println!("relation rows: {}", artifacts.relations.row_count());
    println!("property rows: {}", artifacts.properties.row_count());
    println!("persisted artifacts: {}", fixture_path(&artifact_path));
    println!("persisted relations: {}", fixture_path(&relation_path));
    println!("persisted properties: {}", fixture_path(&property_path));
    println!();

    stage(
        3,
        "Shell Knowledge",
        "Shell reads the immediate register and the mediated middle at one address.",
    );
    let shell = ds_frame
        .into_shell_with_program_features(program)
        .materialize_semdataset_from_texts(&[
            "Model names the DataFrame body.",
            "Feature binds determinate addresses.",
            "Plan retains and orders what enters mediation.",
            "SemDataset is the Concept return opened by the middle.",
        ])?;
    if let Some(frame) = shell.dataframe_knowledge() {
        println!("frame knowledge columns: {:?}", frame.columns());
        println!(
            "frame knowledge shape: {} x {}",
            frame.row_count(),
            frame.column_count()
        );
    }
    if let Some(middle) = shell.model_feature_plan_knowledge() {
        println!("middle model: {:?}", middle.model_name());
        println!("middle principles: {:?}", middle.principles());
        println!("middle concepts: {:?}", middle.concepts());
        println!("middle plan steps: {:?}", middle.plan_steps());
    }
    println!();

    stage(
        4,
        "Concept Return",
        "SemDataset and PureForm return make the middle auditable.",
    );
    let semantic = shell.semantic_pipeline_knowledge();
    let learning = shell.learning_report();
    let trace = shell.validate_projection_trace();
    let descriptor = shell.descriptor();
    let pureform_return = descriptor.to_pure_form_return();
    let principle = pureform_return.principle();
    println!("semdataset ready: {}", semantic.semdataset_ready());
    println!(
        "pureform return ready: {}",
        semantic.pureform_return_ready()
    );
    println!("learning readiness: {}", learning.readiness_score());
    println!("trace valid: {}", trace.is_valid());
    println!("pureform fields: {:?}", principle.shape.required_fields);
    let return_path = fixture_root.join("04-pureform-return.txt");
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
    println!("persisted return: {}", fixture_path(&return_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &frame_path,
            &artifact_path,
            &relation_path,
            &property_path,
            &return_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_model_feature_plan")
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
    format!("fixtures/collections/dataset/dataset_model_feature_plan/{file_name}")
}

fn manifest(
    frame_path: &Path,
    artifact_path: &Path,
    relation_path: &Path,
    property_path: &Path,
    return_path: &Path,
) -> String {
    format!(
        "Dataset Model:Feature::Plan Fixture\n\n\
         Doctrine: EXEMPLARS/027-model-feature-plan-middle.md\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: DataFrame body as EssentialBeing for Dataset mediation.\n\n\
         02 Essence Middle\n\
         artifacts: {}\n\
         relations: {}\n\
         properties: {}\n\
         meaning: internal Rust DSL ProgramFeatures materialized as durable Dataset tables.\n\n\
         04 Return\n\
         artifact: {}\n\
         meaning: Shell trace and PureForm return witness for the middle.\n",
        fixture_path(frame_path),
        fixture_path(artifact_path),
        fixture_path(relation_path),
        fixture_path(property_path),
        fixture_path(return_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
