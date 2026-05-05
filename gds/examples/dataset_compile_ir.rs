//! Dataset Compile IR fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_compile_ir

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Compile IR ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Program Features",
        "Program features declare semantic intent before execution.",
    );
    let features = program_features(
        "rustscript.dataset.compile_ir",
        ["CompileIRImage"],
        [
            program_source(
                "frame",
                "fixtures/collections/dataset/dataset_compile_ir/00-source.csv",
            ),
            program_observation("term_id"),
            program_reflection("compile-ir-observation"),
            program_concept("CompileIRImage"),
            program_principle("program-features-compile-to-artifacts"),
            program_procedure("persist-compile-ir-artifacts"),
        ],
    );

    let source = gds::tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["source", "compile", "artifact"]),
    )?;
    let source_path = fixture_root.join("00-source.csv");
    source.write_csv(&path_string(&source_path))?;
    println!("program: {}", features.program_name);
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Compilation Graph",
        "Program features are lowered into typed compile nodes.",
    );
    let compilation = DatasetToolChain::image_from_program_features(&features);
    compilation
        .validate()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let graph_path = fixture_root.join("01-graph.txt");
    fs::write(
        &graph_path,
        format!(
            "nodes: {}\nentrypoints: {}\n",
            compilation.nodes.len(),
            compilation.entrypoints.len(),
        ),
    )?;
    println!("nodes: {}", compilation.nodes.len());
    println!("persisted: {}", fixture_path(&graph_path));
    println!();

    stage(
        2,
        "Artifact Materialization",
        "Compile nodes materialize artifact, relation, and property tables.",
    );
    let materialized = compilation.materialize_artifact_datasets(&features.program_name)?;
    let artifacts_path = persist_dataset(&materialized.artifacts, &fixture_root, "02-artifacts")?;
    let relations_path = persist_dataset(&materialized.relations, &fixture_root, "02-relations")?;
    let properties_path = persist_dataset(&materialized.properties, &fixture_root, "02-properties")?;
    println!("artifact rows: {}", materialized.artifacts.row_count());
    println!("relation rows: {}", materialized.relations.row_count());
    println!("property rows: {}", materialized.properties.row_count());
    println!("persisted: {}", fixture_path(&artifacts_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &graph_path,
            &artifacts_path,
            &relations_path,
            &properties_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_compile_ir")
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
    format!("fixtures/collections/dataset/dataset_compile_ir/{file_name}")
}

fn manifest(
    source_path: &Path,
    graph_path: &Path,
    artifacts_path: &Path,
    relations_path: &Path,
    properties_path: &Path,
) -> String {
    format!(
        "Dataset Compile IR Fixture\n\n\
         Namespace: dataset::compile\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: source frame referenced by program features.\n\n\
         01 Graph\n\
         artifact: {}\n\
         meaning: validated compilation graph shape.\n\n\
         02 Materialized\n\
         artifacts: {}\n\
         relations: {}\n\
         properties: {}\n\
         meaning: compile IR realized as durable dataset tables.\n",
        fixture_path(source_path),
        fixture_path(graph_path),
        fixture_path(artifacts_path),
        fixture_path(relations_path),
        fixture_path(properties_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
