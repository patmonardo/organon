//! Dataset Frame DSL fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_frame_dsl

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Frame DSL ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame Entry",
        "Dataset starts from a named DataFrame body with semantic addressing.",
    );
    let table = gds::tbl_def!(
        (doc_id: i64 => [1, 2, 3]),
        (text: [
            "Reason gives form to experience",
            "Dataset frames semantic programs",
            "The kernel lowers language into tables"
        ]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    let ds = ds_frame(table)
        .named("dataset-frame-dsl")
        .artifact_kind(DatasetArtifactKind::Corpus)
        .facet("doctrine:001-frame-dsl")
        .source_io(io_path(
            "fixtures/collections/dataset/dataset_frame_dsl/00-frame.csv",
        ));

    stage(
        1,
        "Pipeline Authoring",
        "Functions::shell and macro surface author the intake pipeline.",
    );
    let pipeline = ds
        .pipeline()
        .with_op(text_input("article.input"))
        .with_op(text_encode("article.encode"))
        .with_op(text_transform("article.transform"))
        .with_op(text_decode("article.decode"))
        .with_op(text_output("article.output"))
        .with_metadata(metadata("framework", "dataset-frame-dsl"))
        .with_projection(project_text(["doc_id", "text", "token_count"]))
        .with_report(report_summary().with_title("Dataset frame DSL summary"));

    let artifacts = pipeline.build_artifacts("text");
    let lowered = pipeline.lower_to_dataframe_exprs("text");
    let pipeline_path = fixture_root.join("01-pipeline.txt");
    fs::write(
        &pipeline_path,
        format!(
            "ops: {}\ndataset_aspects: {}\ndataframe_lowerings: {}\nlowered_exprs: {}\n",
            pipeline.ops.len(),
            artifacts.dataset_aspects.len(),
            artifacts.dataframe_lowerings.len(),
            lowered.len(),
        ),
    )?;
    println!("pipeline ops: {}", pipeline.ops.len());
    println!("persisted: {}", fixture_path(&pipeline_path));
    println!();

    stage(
        2,
        "Compilation",
        "Pipeline is lowered into Dataset compilation nodes.",
    );
    let compilation = pipeline.to_compilation();
    compilation
        .validate()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let compile_path = fixture_root.join("02-compilation.txt");
    fs::write(
        &compile_path,
        format!(
            "nodes: {}\nentrypoints: {}\n",
            compilation.nodes.len(),
            compilation.entrypoints.len(),
        ),
    )?;
    println!("compilation nodes: {}", compilation.nodes.len());
    println!("persisted: {}", fixture_path(&compile_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &pipeline_path, &compile_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/dataset/dataset_frame_dsl")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_frame_dsl/{file_name}")
}

fn manifest(frame_path: &Path, pipeline_path: &Path, compile_path: &Path) -> String {
    format!(
        "Dataset Frame DSL Fixture\n\n\
         Namespace: dataset::functions::shell / dataset::macro\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: DataFrame body named for dataset-side intake.\n\n\
         01 Pipeline\n\
         artifact: {}\n\
         meaning: typed shell/dataop pipeline with metadata and projection.\n\n\
         02 Compilation\n\
         artifact: {}\n\
         meaning: pipeline lowered into compilation-node graph.\n",
        fixture_path(frame_path),
        fixture_path(pipeline_path),
        fixture_path(compile_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
