//! Eval Workbench: Predict Template Law.
//!
//! Run with:
//!   cargo run -p gds --example eval_predict_template_law

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::projection::eval::workbench::projection_eval_workbench_track;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let track =
        projection_eval_workbench_track("pe-predict-template-law").ok_or("missing track")?;
    let sequence = [
        "load trained model",
        "validate prediction parameter space",
        "resolve target graph and node set",
        "run prediction pipeline",
        "emit stream/mutate/write outputs",
        "clear transient prediction state",
    ];

    let text = format!(
        "id: {}\ntitle: {}\nfocus: {}\nexemplar: {}\nstatus: {}\n\npredict-sequence:\n- {}\n- {}\n- {}\n- {}\n- {}\n- {}\n",
        track.id,
        track.title,
        track.focus,
        track.exemplar,
        track.status,
        sequence[0],
        sequence[1],
        sequence[2],
        sequence[3],
        sequence[4],
        sequence[5]
    );

    let artifact = root.join("00-predict-template-law.txt");
    fs::write(&artifact, text)?;

    println!("fixture: {}", fixture_path(&artifact));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/eval/eval_predict_template_law")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/eval/eval_predict_template_law/{file_name}")
}
