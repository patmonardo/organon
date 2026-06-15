//! TaskFrame Shell Model Feature Plan.
//!
//! Run with:
//!   cargo run -p gds --example taskframe_shell_model_feature_plan

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::task::concurrency::Concurrency;
use gds::task::memory::MemoryRange;
use gds::task::runtime::{TaskFrame, TaskRuntime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let frame = TaskFrame::new(
        "shell".to_string(),
        "pipeline::ModelFeaturePlan".to_string(),
        vec![
            "dataframe.seed".to_string(),
            "dataset.model".to_string(),
            "dataset.feature".to_string(),
            "dataset.plan".to_string(),
            "shell.concept-return".to_string(),
        ],
        512,
        Concurrency::of(4),
    )
    .with_memory_range(MemoryRange::of_range(16 * 1024, 256 * 1024));

    println!("track: taskframe-shell-model-feature-plan");
    println!("description: {}", frame.description());
    println!("steps: {:?}", frame.steps());

    let mut runtime = TaskRuntime::from_frame(frame);
    runtime.begin();
    runtime.progress(128);
    runtime.progress(128);
    runtime.progress(256);
    runtime.end();

    println!("status: completed");

    let summary_path = root.join("00-frame-summary.txt");
    fs::write(
        &summary_path,
        format!(
            "track: taskframe-shell-model-feature-plan\ndescription: {}\nsteps: {:?}\nvolume: {}\n",
            runtime.frame().description(),
            runtime.frame().steps(),
            runtime.frame().estimated_volume(),
        ),
    )?;

    let manifest_path = root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "TaskFrame Shell Model Feature Plan Fixture\n\n00 Frame Summary\nartifact: {}\nmeaning: mediated shell path from model-feature-plan into top-level task runtime.\n",
            fixture_path(&summary_path)
        ),
    )?;

    println!("fixture: {}", fixture_path(&summary_path));
    println!("manifest: {}", fixture_path(&manifest_path));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/task/workbench/taskframe_shell_model_feature_plan")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/task/workbench/taskframe_shell_model_feature_plan/{file_name}")
}
