//! Shell Workbench Catalog.
//!
//! Run with:
//!   cargo run -p gds --example shell_workbench_catalog

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::run_shell_workbench_catalog;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Workbench Catalog ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;

    let text = run_shell_workbench_catalog();

    let catalog_path = fixture_root.join("00-shell-workbench-catalog.txt");
    fs::write(&catalog_path, &text)?;

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Shell Workbench Catalog Fixture\n\n00 Catalog\nartifact: {}\nmeaning: shell workbench track slice rendered from shell::workbench::catalog.\n",
            fixture_path(&catalog_path)
        ),
    )?;

    println!("persisted: {}", fixture_path(&catalog_path));
    println!("manifest: {}", fixture_path(&manifest_path));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/shell/shell_workbench_catalog")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_workbench_catalog/{file_name}")
}
