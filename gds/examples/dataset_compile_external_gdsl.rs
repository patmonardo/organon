//! Dataset external GDSL compile fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_compile_external_gdsl

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Compile External GDSL ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "External Program Source",
        "External .gdsl input is lowered into internal program features.",
    );
    let gdsl_source_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/shell.old/absolute-concept-scientific-inference.gdsl");
    let source_path = fixture_root.join("00-source.txt");
    fs::write(
        &source_path,
        format!("gdsl_path: {}\n", gdsl_source_path.to_string_lossy()),
    )?;
    println!("gdsl file: {}", gdsl_source_path.display());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Lowering",
        "Toolchain lowers external source into ProgramFeatures and compile graph.",
    );
    let features = DatasetToolChain::program_features_from_gdsl_file(&gdsl_source_path)?;
    let compilation = DatasetToolChain::image_from_gdsl_file(&gdsl_source_path)?;
    compilation
        .validate()
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;
    let lowering_path = fixture_root.join("01-lowering.txt");
    fs::write(
        &lowering_path,
        format!(
            "program: {}\nselected_forms: {:?}\nfeature_count: {}\nnodes: {}\nentrypoints: {}\n",
            features.program_name,
            features.selected_forms,
            features.features.len(),
            compilation.nodes.len(),
            compilation.entrypoints.len(),
        ),
    )?;
    println!("program: {}", features.program_name);
    println!("feature count: {}", features.features.len());
    println!("persisted: {}", fixture_path(&lowering_path));
    println!();

    stage(
        2,
        "Materialization",
        "Lowered graph materializes durable artifact datasets.",
    );
    let materialized = compilation.materialize_artifact_datasets(&features.program_name)?;
    let artifacts_path = persist_dataset(&materialized.artifacts, &fixture_root, "02-artifacts")?;
    let relations_path = persist_dataset(&materialized.relations, &fixture_root, "02-relations")?;
    let properties_path =
        persist_dataset(&materialized.properties, &fixture_root, "02-properties")?;
    println!("artifact rows: {}", materialized.artifacts.row_count());
    println!("relation rows: {}", materialized.relations.row_count());
    println!("property rows: {}", materialized.properties.row_count());
    println!("persisted: {}", fixture_path(&artifacts_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &lowering_path,
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
        .join("fixtures/collections/dataset/dataset_compile_external_gdsl")
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
    format!("fixtures/collections/dataset/dataset_compile_external_gdsl/{file_name}")
}

fn manifest(
    source_path: &Path,
    lowering_path: &Path,
    artifacts_path: &Path,
    relations_path: &Path,
    properties_path: &Path,
) -> String {
    format!(
        "Dataset Compile External GDSL Fixture\n\n\
         Namespace: dataset::toolchain (external)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: external .gdsl source reference.\n\n\
         01 Lowering\n\
         artifact: {}\n\
         meaning: lowered ProgramFeatures and validated compile graph.\n\n\
         02 Materialized\n\
         artifacts: {}\n\
         relations: {}\n\
         properties: {}\n\
         meaning: external source materialized as durable dataset tables.\n",
        fixture_path(source_path),
        fixture_path(lowering_path),
        fixture_path(artifacts_path),
        fixture_path(relations_path),
        fixture_path(properties_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
