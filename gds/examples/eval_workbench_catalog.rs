//! Eval Workbench Catalog.
//!
//! Run with:
//!   cargo run -p gds --example eval_workbench_catalog

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::projection::eval::workbench::run_projection_eval_workbench_catalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Eval Workbench Catalog ==");

    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let text = run_projection_eval_workbench_catalog();

    let catalog_path = root.join("00-eval-workbench-catalog.txt");
    fs::write(&catalog_path, &text)?;

    let manifest_path = root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Eval Workbench Catalog Fixture\n\n00 Catalog\nartifact: {}\nmeaning: projection eval workbench track slice rendered from projection::eval::workbench::catalog.\n",
            fixture_path(&catalog_path)
        ),
    )?;

    println!("fixture: {}", fixture_path(&catalog_path));
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/eval/eval_workbench_catalog")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/eval/eval_workbench_catalog/{file_name}")
}
