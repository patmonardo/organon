//! Dataset JSON IO fixture.
//!
//! Exercises JSON/NDJSON persistence + Dataset::from_json loading, format
//! detection, and namespace expression constructors (io_path / io_url).
//!
//! Run with:
//!   cargo run -p gds --example dataset_io_json

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::io::detect_format_from_path;
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset IO JSON ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Emit JSON/NDJSON",
        "Write tabular data to JSON and JSON Lines.",
    );

    let table = gds::tbl_def!(
        (doc_id: i64 => [1, 2, 3, 4]),
        (text: ["alpha", "beta", "gamma", "delta"]),
        (score: f64 => [0.91, 0.76, 0.88, 0.95]),
    )?;

    let json_path = fixture_root.join("00-source.json");
    let ndjson_path = fixture_root.join("00-source.ndjson");
    table.write_json(&path_string(&json_path))?;
    table.write_ndjson(&path_string(&ndjson_path))?;

    println!("source table shape: {:?}", table.shape());
    println!("persisted: {}", fixture_path(&json_path));
    println!("persisted: {}", fixture_path(&ndjson_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Format Detection",
        "detect_format_from_path maps file extensions to CollectionsIoFormat.",
    );

    let f_json = detect_format_from_path(&json_path);
    let f_ndjson = detect_format_from_path(&ndjson_path);
    let f_csv = detect_format_from_path(Path::new("dummy.csv"));

    println!("json   -> {:?}", f_json);
    println!("ndjson -> {:?}", f_ndjson);
    println!("csv    -> {:?}", f_csv);

    let detect_path = fixture_root.join("01-detect.txt");
    fs::write(
        &detect_path,
        format!(
            "json={:?}\nndjson={:?}\ncsv={:?}\n",
            f_json, f_ndjson, f_csv
        ),
    )?;
    println!("persisted: {}", fixture_path(&detect_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Dataset::from_json",
        "Load NDJSON into Dataset and verify row/column shape.",
    );

    let dataset = Dataset::from_json(&ndjson_path)?;
    println!("loaded row_count: {}", dataset.row_count());
    println!("loaded table shape: {:?}", dataset.table().shape());
    println!("{}", dataset.table().fmt_table());

    let loaded_csv = fixture_root.join("02-loaded.csv");
    dataset.table().write_csv(&path_string(&loaded_csv))?;
    println!("persisted: {}", fixture_path(&loaded_csv));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Namespace IO Expressions",
        "Construct Dataset IO expressions with io_path/io_url and DatasetNs.",
    );

    let expr_fn_path = io_path(path_string(&json_path));
    let expr_fn_url = io_url("https://example.org/demo.json");
    let expr_ns_path = DatasetNs::io_path(path_string(&ndjson_path));

    println!("io_path(...)      => {:?}", expr_fn_path);
    println!("io_url(...)       => {:?}", expr_fn_url);
    println!("DatasetNs::io_path => {:?}", expr_ns_path);

    let ns_path = fixture_root.join("03-namespace-io.txt");
    fs::write(
        &ns_path,
        format!(
            "io_path={:?}\nio_url={:?}\nDatasetNs::io_path={:?}\n",
            expr_fn_path, expr_fn_url, expr_ns_path
        ),
    )?;
    println!("persisted: {}", fixture_path(&ns_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &json_path,
            &ndjson_path,
            &detect_path,
            &loaded_csv,
            &ns_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/dataset/dataset_io_json")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_io_json/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
}

fn manifest(json: &Path, ndjson: &Path, detect: &Path, loaded: &Path, ns: &Path) -> String {
    format!(
        "Dataset IO JSON Fixture\n\n\
         Namespace: dataset::io + dataset::namespaces::dataset\n\n\
         00 Source JSON\n\
         artifacts: {}, {}\n\
         meaning: Same table written in JSON and NDJSON formats.\n\n\
         01 Detect\n\
         artifact: {}\n\
         meaning: detect_format_from_path results for json/ndjson/csv.\n\n\
         02 Loaded\n\
         artifact: {}\n\
         meaning: Dataset::from_json round-trip into tabular view.\n\n\
         03 Namespace IO\n\
         artifact: {}\n\
         meaning: io_path/io_url constructors for Dataset IO expressions.\n",
        fixture_path(json),
        fixture_path(ndjson),
        fixture_path(detect),
        fixture_path(loaded),
        fixture_path(ns),
    )
}
