//! Eval Workbench: Catalog Symbolics.
//!
//! Run with:
//!   cargo run -p gds --example eval_catalog_symbolics

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::projection::eval::workbench::projection_eval_workbench_track;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let track = projection_eval_workbench_track("pe-catalog-symbolics").ok_or("missing track")?;

    let text = format!(
        "id: {}\ntitle: {}\nfocus: {}\nexemplar: {}\nstatus: {}\n\nchecks:\n- pipeline symbol is namespaced by user\n- pipeline type is recorded for runtime downcast safety\n- typed retrieval must match expected concrete pipeline type\n",
        track.id, track.title, track.focus, track.exemplar, track.status
    );

    let artifact = root.join("00-catalog-symbolics.txt");
    fs::write(&artifact, text)?;

    println!("fixture: {}", fixture_path(&artifact));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/eval/eval_catalog_symbolics")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/eval/eval_catalog_symbolics/{file_name}")
}
