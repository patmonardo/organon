//! Dataset Create Semantic Dataset fixture.
//!
//! Practical workbench entrypoint for dataset-core platform engineering:
//! create a semantic dataset folder layout with a canonical frame and a
//! DataFrame-backed catalog frame.
//!
//! Run with:
//!   cargo run -p gds --example dataset_create_sem_dataset

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::DatasetCreateOptions;
use gds::collections::dataset::DatasetWorkspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Create Semantic Dataset ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Workspace",
        "Initialize a unix-style dataset workspace root.",
    );
    let workspace_root = fixture_root.join("workspace");
    let ws = DatasetWorkspace::new(&workspace_root)?;
    println!("workspace root: {}", ws.root().display());
    println!();

    stage(
        1,
        "Create",
        "Create a semantic dataset layout, semantic frame, and catalog frame.",
    );
    let report = ws.create_dataset(
        "demo_sem_dataset",
        DatasetCreateOptions {
            principle_triad: "model-feature-plan".to_string(),
            law_view: "corpus-language-logic".to_string(),
            ..DatasetCreateOptions::default()
        },
    )?;

    println!("dataset: {}", report.dataset);
    println!("dataset root: {}", report.layout.root.display());
    println!("semantic frame: {}", report.semantic_frame_path.display());
    println!("catalog frame: {}", report.catalog_frame_path.display());
    println!();

    stage(
        2,
        "Manifest",
        "Persist a tiny summary manifest for quick inspection.",
    );
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Dataset Create Semantic Dataset Fixture\n\n\
             workspace: {}\n\
             dataset: {}\n\
             semantic_frame: {}\n\
             catalog_frame: {}\n",
            path_string(&workspace_root),
            report.dataset,
            path_string(&report.semantic_frame_path),
            path_string(&report.catalog_frame_path),
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_create_sem_dataset")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_create_sem_dataset/{file_name}")
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
