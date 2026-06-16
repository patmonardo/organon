//! Eval Workbench: Node Classification Train Predict.
//!
//! Run with:
//!   cargo run -p gds --example eval_node_classification_train_predict

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use gds::procedures::pipelines::AnyMap;
use gds::procedures::pipelines::LocalPipelinesProcedureFacade;
use gds::procedures::pipelines::PipelinesProcedureFacade;
use gds::procedures::pipelines::RequestScopedDependencies;
use gds::projection::eval::pipeline::PipelineCatalog;
use gds::types::catalog::GraphCatalog;
use gds::types::catalog::InMemoryGraphCatalog;
use gds::types::graph_store::DefaultGraphStore;
use gds::types::random::RandomGraphConfig;
use gds::types::random::RandomRelationshipConfig;
use gds::types::user::User;
use serde::Serialize;
use serde_json::json;
use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Eval Node Classification Train Predict ==");

    let root = fixture_root();
    fs::create_dir_all(&root)?;

    let graph_catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
    let pipeline_catalog = Arc::new(PipelineCatalog::new());
    let user = User::from("alice");

    let graph_store = Arc::new(make_graph_store()?);
    graph_catalog.set("nc_graph", Arc::clone(&graph_store));

    let facade = LocalPipelinesProcedureFacade::new(
        RequestScopedDependencies::with_graph_catalog(user, Arc::clone(&graph_catalog)),
        pipeline_catalog,
    );

    stage(
        0,
        "Graph Fixture",
        "Random graph plus label/age node properties.",
    );
    let graph_summary = json!({
        "graphName": "nc_graph",
        "nodeCount": 24,
        "targetProperty": "label",
        "featureProperties": ["age"],
        "classes": [0, 1]
    });
    let graph_summary_path = root.join("00-graph-summary.json");
    write_json(&graph_summary_path, &graph_summary)?;

    stage(
        1,
        "Pipeline Setup",
        "Create pipeline, select features, add training method.",
    );
    let created = facade.node_classification().create_pipeline("nc_demo");
    let selected_features = facade.node_classification().select_features(
        "nc_demo",
        Value::Array(vec![Value::String("age".to_string())]),
    );
    let logistic_regression = facade
        .node_classification()
        .add_logistic_regression("nc_demo", AnyMap::new());

    let setup_path = root.join("01-pipeline-setup.json");
    write_json(
        &setup_path,
        &json!({
            "created": created,
            "selectedFeatures": selected_features,
            "logisticRegression": logistic_regression,
        }),
    )?;

    stage(
        2,
        "Train Request",
        "Train the node classification model through the procedure facade.",
    );
    let train_request = make_map(&[
        ("pipeline", json!("nc_demo")),
        ("modelName", json!("nc_demo_model")),
        ("targetNodeLabels", json!(["*"])),
        ("targetProperty", json!("label")),
        ("randomSeed", json!(42)),
        ("concurrency", json!(2)),
        ("metrics", json!(["F1_MACRO"])),
    ]);
    let train_request_path = root.join("02-train-request.json");
    write_json(&train_request_path, &train_request)?;

    let train_result = facade
        .node_classification()
        .train("nc_graph", train_request);
    let train_result_path = root.join("03-train-result.json");
    write_json(&train_result_path, &train_result)?;

    stage(
        3,
        "Predict Request",
        "Stream predictions from the trained in-process model.",
    );
    let stream_request = make_map(&[
        ("modelName", json!("nc_demo_model")),
        ("modelUser", json!("alice")),
        ("targetNodeLabels", json!(["*"])),
        ("includePredictedProbabilities", json!(true)),
        ("concurrency", json!(2)),
    ]);
    let stream_request_path = root.join("04-stream-request.json");
    write_json(&stream_request_path, &stream_request)?;

    let stream_result = facade
        .node_classification()
        .stream("nc_graph", stream_request);
    let stream_result_path = root.join("05-stream-result.json");
    write_json(&stream_result_path, &stream_result)?;

    let notes_path = root.join("06-notes.txt");
    write_text(
        &notes_path,
        concat!(
            "Pattern\n",
            "Facade train/predict -> Eval train/predict templates -> ML classifier training/prediction.\n\n",
            "Happy path used here\n",
            "- feature selection only (no node-property step)\n",
            "- logistic regression trainer\n",
            "- stream prediction, not mutate/write\n",
            "- in-memory graph catalog and in-process trained model store\n",
        ),
    )?;

    let manifest_path = root.join("README.txt");
    write_text(
        &manifest_path,
        &manifest(
            &graph_summary_path,
            &setup_path,
            &train_request_path,
            &train_result_path,
            &stream_request_path,
            &stream_result_path,
            &notes_path,
        ),
    )?;

    println!("train rows: {}", train_result.len());
    println!("predictions: {}", stream_result.len());
    println!("fixture: {}", fixture_path(&manifest_path));

    Ok(())
}

fn make_graph_store() -> Result<DefaultGraphStore, Box<dyn std::error::Error>> {
    let config = RandomGraphConfig {
        node_count: 24,
        relationships: vec![RandomRelationshipConfig::new("RELATES", 0.2)],
        seed: Some(42),
        ..RandomGraphConfig::default()
    };

    let mut graph_store = DefaultGraphStore::random(&config)?;

    let labels: Vec<i64> = (0..24).map(|idx| (idx % 2) as i64).collect();
    let ages: Vec<f64> = (0..24).map(|idx| 20.0 + (idx as f64 * 0.5)).collect();

    graph_store.add_node_property_i64("label".to_string(), labels)?;
    graph_store.add_node_property_f64("age".to_string(), ages)?;

    Ok(graph_store)
}

fn stage(index: usize, title: &str, description: &str) {
    println!("[{index}] {title}");
    println!("{description}");
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}

fn write_text(path: &Path, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(path, text)?;
    Ok(())
}

fn make_map(entries: &[(impl AsRef<str>, Value)]) -> AnyMap {
    let mut map = AnyMap::new();
    for (key, value) in entries {
        map.insert(key.as_ref().to_string(), value.clone());
    }
    map
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/eval/eval_node_classification_train_predict")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/eval/eval_node_classification_train_predict/{file_name}")
}

fn manifest(
    graph_summary_path: &Path,
    setup_path: &Path,
    train_request_path: &Path,
    train_result_path: &Path,
    stream_request_path: &Path,
    stream_result_path: &Path,
    notes_path: &Path,
) -> String {
    format!(
        r#"Eval Node Classification Train Predict Fixture

00 Graph Summary
artifact: {}
meaning: graph fixture and property contract for the train/predict run.

01 Pipeline Setup
artifact: {}
meaning: created pipeline, selected features, and training method binding.

02 Train Request
artifact: {}
meaning: raw nodeClassification.train request map.

03 Train Result
artifact: {}
meaning: rendered model creation result from the procedure facade.

04 Stream Request
artifact: {}
meaning: raw nodeClassification.stream request map.

05 Stream Result
artifact: {}
meaning: predicted classes and probabilities for all nodes.

06 Notes
artifact: {}
meaning: short interpretation of the end-to-end pattern used here.
"#,
        fixture_path(graph_summary_path),
        fixture_path(setup_path),
        fixture_path(train_request_path),
        fixture_path(train_result_path),
        fixture_path(stream_request_path),
        fixture_path(stream_result_path),
        fixture_path(notes_path),
    )
}
