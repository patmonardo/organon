//! TaskFrame Catalog Slice.
//!
//! Run with:
//!   cargo run -p gds --example taskframe_catalog_slice

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::task::workbench::run_task_workbench_catalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let text = run_task_workbench_catalog();

    let catalog_path = root.join("00-task-workbench-catalog.txt");
    fs::write(&catalog_path, &text)?;

    let manifest_path = root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "TaskFrame Catalog Slice Fixture\n\n00 Catalog\nartifact: {}\nmeaning: task workbench slice rendered from catalog module convenience runner.\n",
            fixture_path(&catalog_path)
        ),
    )?;

    println!("fixture: {}", fixture_path(&catalog_path));
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/task/workbench/taskframe_catalog_slice")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/task/workbench/taskframe_catalog_slice/{file_name}")
}
