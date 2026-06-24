use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use super::*;

fn temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("gds_{prefix}_{nanos}"));
    fs::create_dir_all(&dir).expect("temp dir");
    dir
}

#[test]
fn workspace_initializes_semantic_layout() {
    let root = temp_dir("workspace_layout");
    let ws = DatasetWorkspace::new(&root).expect("workspace");
    let layout = ws.ensure_dataset_layout("demo").expect("layout");
    assert!(layout.semantic_dir.exists());
    assert!(layout.model_dir.exists());
    assert!(layout.feature_dir.exists());
    assert!(layout.plan_dir.exists());
    assert!(layout.corpus_dir.exists());
    assert!(layout.language_dir.exists());
    assert!(layout.logic_dir.exists());
}

#[test]
fn workspace_writes_frame_and_catalog() {
    let root = temp_dir("workspace_catalog");
    let ws = DatasetWorkspace::new(&root).expect("workspace");
    let frame = ws
        .write_semantic_frame("demo", "model-feature-plan", "corpus-language-logic")
        .expect("frame");
    let frame_body = fs::read_to_string(&frame).expect("read frame");
    assert!(frame_body.contains("\"seven_fold_support\""));
    assert!(frame_body.contains("\"model\""));
    assert!(frame_body.contains("\"feature\""));
    assert!(frame_body.contains("\"plan\""));
    let layout = ws.ensure_dataset_layout("demo").expect("layout");
    let catalog = ws.write_catalog_frame("demo", &layout).expect("catalog");
    assert!(frame.exists());
    assert!(catalog.exists());
}

#[test]
fn workspace_create_dataset_builder_flow() {
    let root = temp_dir("workspace_create");
    let ws = DatasetWorkspace::new(&root).expect("workspace");
    let report = ws
        .create_dataset(
            "demo",
            DatasetCreateOptions {
                ..DatasetCreateOptions::default()
            },
        )
        .expect("create");

    assert_eq!(report.dataset, "demo");
    assert!(report.layout.root.exists());
    assert!(report.semantic_frame_path.exists());
    assert!(report.catalog_frame_path.exists());
}

#[test]
fn workspace_stages_local_archive() {
    let root = temp_dir("workspace_stage");
    let ws = DatasetWorkspace::new(&root).expect("workspace");
    let source = root.join("sample.zip");
    fs::write(&source, b"placeholder").expect("source file");

    let source_string = source.to_string_lossy().into_owned();
    let report = ws
        .stage_archive("demo", &source_string)
        .expect("stage archive");

    assert_eq!(report.dataset, "demo");
    assert!(report.staged_path.exists());
    assert!(report.staged_path.starts_with(ws.staging_root()));
    assert_eq!(report.transferred.bytes, 11);
}

#[test]
fn workspace_indexes_raw_contents() {
    let root = temp_dir("workspace_inventory");
    let ws = DatasetWorkspace::new(&root).expect("workspace");
    let layout = ws.ensure_dataset_layout("demo").expect("layout");
    fs::create_dir_all(layout.raw_dir.join("docs")).expect("raw docs dir");
    fs::write(layout.raw_dir.join("docs/doc1.txt"), b"hello inventory").expect("raw file");

    let inventory = ws.index_raw_contents("demo").expect("inventory");
    assert!(inventory.exists());
    assert!(inventory.ends_with("semantic/raw_inventory.csv"));
}
