//! Shell Model First: begin Model:Feature::Plan from the Shell side.
//!
//! Run with:
//!   cargo run -p gds --example shell_model_first

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Model First ==");
    println!("Doctrine thread: Shell begins with the Model moment, then descends into Dataset.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Shell First",
        "Shell receives a framed body and reads only the Model moment for now.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["model", "feature", "plan"]),
        (role: ["name", "binding", "ordering"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "Model Moment",
        "The Shell names the body as a Model before expanding Feature or Plan.",
    );
    let program = program_features(
        "rustscript.shell.model_first",
        ["ShellModelFirst"],
        [
            program_source(
                "frame",
                "fixtures/collections/shell/shell_model_first/00-frame.csv",
            ),
            program_reflection("model-moment-only"),
            program_principle("model-names-the-mediated-body"),
            program_concept("ShellModelFirst"),
            program_procedure("descend-to-dataset-model-feature-plan"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-model-first")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:shell-model-first")
        .facet("model-moment")
        .source_io(io_path(
            "fixtures/collections/shell/shell_model_first/00-frame.csv",
        ))
        .into_shell_with_program_features(program);

    let model = shell.model_moment_knowledge();
    let model_path = fixture_root.join("01-shell-model-moment.txt");
    fs::write(
        &model_path,
        format!(
            "resource: {}\naddress: {:?}\nfold: {}\nmoment: {}\nalgebra: {:?}\ndataset: {:?}\nprogram: {:?}\nselected_forms: {:?}\nmodel_topics: {:?}\nai_assistance_presupposed: {}\n",
            model.doctrine_resource(),
            model.address(),
            model.fold(),
            model.moment(),
            model.algebra(),
            model.dataset_name(),
            model.program_name(),
            model.selected_forms(),
            model.model_topics(),
            model.ai_assistance_presupposed(),
        ),
    )?;
    println!("model topics: {:?}", model.model_topics());
    println!("persisted: {}", fixture_path(&model_path));
    println!();

    stage(
        2,
        "Descend Into Dataset",
        "The next Doctrinal Method step is the fuller Dataset Model:Feature::Plan example.",
    );
    let descent_path = fixture_root.join("02-descend-dataset-model-feature-plan.txt");
    fs::write(
        &descent_path,
        format!(
            "next_example: {}\ndoctrine: gds/Doctrine/EXEMPLARS/dataset/027-model-feature-plan-middle.md\nmethod: Shell starts with Model; Dataset expands Model:Feature::Plan.\n",
            model.descends_to_example(),
        ),
    )?;
    println!("next example: {}", model.descends_to_example());
    println!("persisted: {}", fixture_path(&descent_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &model_path, &descent_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/shell/shell_model_first")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_model_first/{file_name}")
}

fn manifest(frame_path: &Path, model_path: &Path, descent_path: &Path) -> String {
    format!(
        "Shell Model First Fixture\n\n\
         Namespace: shell::model\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: framed body supplied to Shell.\n\n\
         01 Shell Model Moment\n\
         artifact: {}\n\
         meaning: Shell-readable Model moment report.\n\n\
         02 Dataset Descent\n\
         artifact: {}\n\
         meaning: Doctrinal Method handoff to dataset_model_feature_plan.rs.\n",
        fixture_path(frame_path),
        fixture_path(model_path),
        fixture_path(descent_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
