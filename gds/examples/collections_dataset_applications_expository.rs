//! Expository walkthrough of the Dataset-first Collections Applications facade.
//!
//! This example is intentionally verbose. It is not a silent test; it narrates
//! the API as a user-facing service story:
//!
//! - a named Semantic Dataset is introduced into the platform,
//! - the platform catalogs it,
//! - schema and preview are served from the Dataset boundary,
//! - feature evaluation and attention reporting run through the application layer,
//! - and a tiny GDSL source is compiled into derived Dataset artifacts.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_applications_expository

use std::fs;
use std::path::PathBuf;

use gds::applications::collections::loaders::GDS_COLLECTIONS_ROOT_ENV;
use gds::applications::services::collections_dispatch::handle_collections;
use serde_json::{json, Map, Value};

fn main() {
    if let Err(err) = run() {
        eprintln!("collections_dataset_applications_expository failed: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let root = demo_root();
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(&root)?;
    std::env::set_var(GDS_COLLECTIONS_ROOT_ENV, &root);

    let user = "researcher";
    let database = "semantic_lab";
    let dataset_name = "corpus.semantic_notes";

    println!("== Dataset-first Applications / Collections story ==");
    println!("catalog root: {}", root.display());
    println!(
        "thesis: the platform serves a named Semantic Dataset, not just an anonymous DataFrame."
    );

    let _ = dispatch(
        user,
        database,
        "datasetIngestTable",
        json!({
            "datasetName": dataset_name,
            "format": "parquet",
            "table": [
                {
                    "doc_id": 1,
                    "text": "Dialectic turns raw text into structured relation.",
                    "speaker": "hegel",
                    "score": 0.91
                },
                {
                    "doc_id": 2,
                    "text": "A dataset can be the semantic eye of a platform.",
                    "speaker": "organon",
                    "score": 0.97
                },
                {
                    "doc_id": 3,
                    "text": "Feature evaluation should serve research, not hide it.",
                    "speaker": "assistant",
                    "score": 0.88
                }
            ]
        }),
    )?;

    println!("\nInterpretation: the Dataset has entered the platform as a stable semantic object.");

    let _ = dispatch(user, database, "datasetListCatalog", json!({}))?;
    let _ = dispatch(
        user,
        database,
        "datasetSchema",
        json!({ "datasetName": dataset_name }),
    )?;
    let _ = dispatch(
        user,
        database,
        "datasetPreview",
        json!({ "datasetName": dataset_name, "limit": 2 }),
    )?;

    println!(
        "\nInterpretation: the DataFrame appears here as the Dataset's analytic body — visible, previewable, and still governed by the Dataset boundary."
    );

    let _ = dispatch(
        user,
        database,
        "datasetEvalFeature",
        json!({
            "datasetName": dataset_name,
            "featureSpec": { "kind": "identity", "name": "semantic.identity" },
            "evalMode": "preview",
            "previewRows": 2,
            "runId": "demo-run-001"
        }),
    )?;

    let _ = dispatch(
        user,
        database,
        "datasetFeatureAttentionReport",
        json!({
            "datasetName": dataset_name,
            "featureSpec": { "kind": "identity", "name": "semantic.identity" },
            "evalMode": "preview",
            "previewRows": 2
        }),
    )?;

    let _ = dispatch(
        user,
        database,
        "datasetCapabilities",
        json!({ "datasetName": dataset_name }),
    )?;

    println!(
        "\nInterpretation: the application layer is now serving semantic inspection, feature evaluation, and execution metadata as base platform access."
    );

    let _ = dispatch(
        user,
        database,
        "datasetCompileGdslSource",
        json!({
            "compilationName": "semantic_demo_image",
            "gdslSource": "module semantic_demo\nfeature significance : signal\n"
        }),
    )?;

    let _ = dispatch(
        user,
        database,
        "datasetMaterializeCompilation",
        json!({
            "baseName": "semantic_demo",
            "compilationName": "semantic_demo_image"
        }),
    )?;

    println!(
        "\nInterpretation: compilation now yields Dataset-derived artifacts rather than bypassing the Dataset layer."
    );

    let _ = dispatch(
        user,
        database,
        "datasetRemove",
        json!({ "datasetName": dataset_name }),
    )?;
    let _ = dispatch(user, database, "datasetListCatalog", json!({}))?;

    println!(
        "\nConclusion: the Dataset is the semantic peak of Collections — the serviceable source from which previews, features, reports, and derived artifacts unfold."
    );

    Ok(())
}

fn dispatch(
    username: &str,
    database_id: &str,
    op: &str,
    extra: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut request = json!({
        "facade": "collections",
        "op": op,
        "user": { "username": username, "isAdmin": true },
        "databaseId": database_id,
    });

    if let Some(extra_obj) = extra.as_object() {
        let request_obj = request
            .as_object_mut()
            .expect("collections request must be an object");
        extend_object(request_obj, extra_obj);
    }

    let response = handle_collections(&request);

    println!("\n== {op} ==");
    println!("request:\n{}", serde_json::to_string_pretty(&request)?);
    println!("response:\n{}", serde_json::to_string_pretty(&response)?);

    if response.get("ok").and_then(Value::as_bool) != Some(true) {
        return Err(format!(
            "operation '{op}' failed:\n{}",
            serde_json::to_string_pretty(&response)?
        )
        .into());
    }

    Ok(response)
}

fn extend_object(target: &mut Map<String, Value>, extra: &Map<String, Value>) {
    for (key, value) in extra {
        target.insert(key.clone(), value.clone());
    }
}

fn demo_root() -> PathBuf {
    std::env::temp_dir().join("gds-collections-dataset-applications-expository")
}
