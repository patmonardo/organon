//! TaskFrame Runtime Lifecycle.
//!
//! Run with:
//!   cargo run -p gds --example taskframe_runtime_lifecycle

use std::fs;
use std::path::Path;
use std::path::PathBuf;

use gds::task::concurrency::Concurrency;
use gds::task::runtime::{TaskFrame, TaskRuntime};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let frame = TaskFrame::new(
        "task".to_string(),
        "runtime::Lifecycle".to_string(),
        vec!["runtime.begin".to_string(), "runtime.end".to_string()],
        2,
        Concurrency::of(1),
    );

    let mut runtime = TaskRuntime::from_frame(frame);

    runtime.begin();
    runtime.progress(1);
    runtime.progress(1);
    runtime.end();

    println!("TaskFrame runtime lifecycle completed.");

    let summary_path = root.join("00-runtime-lifecycle.txt");
    fs::write(
        &summary_path,
        format!(
            "track: taskframe-runtime-lifecycle\ndescription: {}\nsteps: {:?}\nvolume: {}\n",
            runtime.frame().description(),
            runtime.frame().steps(),
            runtime.frame().estimated_volume(),
        ),
    )?;

    let manifest_path = root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "TaskFrame Runtime Lifecycle Fixture\n\n00 Runtime Lifecycle\nartifact: {}\nmeaning: begin/progress/end boundaries for top-level task runtime.\n",
            fixture_path(&summary_path)
        ),
    )?;

    println!("fixture: {}", fixture_path(&summary_path));
    println!("manifest: {}", fixture_path(&manifest_path));
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/task/workbench/taskframe_runtime_lifecycle")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/task/workbench/taskframe_runtime_lifecycle/{file_name}")
}
