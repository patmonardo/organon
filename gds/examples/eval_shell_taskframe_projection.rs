//! Eval Workbench: Shell TaskFrame Projection.
//!
//! Run with:
//!   cargo run -p gds --example eval_shell_taskframe_projection

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::projection::eval::workbench::projection_eval_workbench_track;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let track =
        projection_eval_workbench_track("pe-shell-taskframe-projection").ok_or("missing track")?;

    let text = format!(
        "id: {}\ntitle: {}\nfocus: {}\nexemplar: {}\nstatus: {}\n\nprogram-feature-shape:\n- trackId\n- executionMode\n- boundaryLaw\n\ntaskframe-shape:\n- inputGraph\n- pipelineSymbol\n- mode\n- outputContract\n",
        track.id, track.title, track.focus, track.exemplar, track.status
    );

    let artifact = root.join("00-shell-taskframe-projection.txt");
    fs::write(&artifact, text)?;

    println!("fixture: {}", fixture_path(&artifact));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/eval/eval_shell_taskframe_projection")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/eval/eval_shell_taskframe_projection/{file_name}")
}
