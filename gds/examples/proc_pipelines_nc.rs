//! Node classification pipelines setup exemplar.
//!
//! Run with:
//!   cargo run -p gds --example proc_pipelines_nc

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gds::procedures::pipelines::{
    AnyMap, LocalPipelinesProcedureFacade, PipelinesProcedureFacade, RequestScopedDependencies,
};
use gds::projection::eval::pipeline::PipelineCatalog;
use gds::types::user::User;
use serde::Serialize;
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Pipelines Node Classification Setup ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Procedure Boundary",
        "Application code enters node classification through the pipelines facade.",
    );
    let facade = LocalPipelinesProcedureFacade::new(
        RequestScopedDependencies::new(User::from("alice")),
        Arc::new(PipelineCatalog::new()),
    );
    let facade_path = fixture_root.join("00-facade.txt");
    write_text(
        &facade_path,
        concat!(
            "entrypoint: LocalPipelinesProcedureFacade::node_classification\n",
            "scope: procedures::pipelines\n",
            "purpose: create and configure a node classification pipeline before training or prediction.\n",
        ),
    )?;
    println!("persisted: {}", fixture_path(&facade_path));
    println!();

    stage(
        1,
        "Create Pipeline",
        "The facade creates a named node classification training pipeline in the catalog.",
    );
    let created = facade.node_classification().create_pipeline("nc_demo");
    let create_path = fixture_root.join("01-create-pipeline.json");
    write_json(&create_path, &created)?;
    println!("pipeline name: {}", created[0].name);
    println!("persisted: {}", fixture_path(&create_path));
    println!();

    stage(
        2,
        "Split Configuration",
        "Split configuration is normalized into the pipeline's typed config surface.",
    );
    let split_config = make_map(&[("testFraction", json!(0.2)), ("validationFolds", json!(5))]);
    let configured_split = facade
        .node_classification()
        .configure_split("nc_demo", split_config);
    let split_path = fixture_root.join("02-configure-split.json");
    write_json(&split_path, &configured_split)?;
    println!("test fraction: 0.2");
    println!("validation folds: 5");
    println!("persisted: {}", fixture_path(&split_path));
    println!();

    stage(
        3,
        "Auto Tuning",
        "Auto tuning is another catalog mutation on the same pipeline object.",
    );
    let auto_tuning_config = make_map(&[("maxTrials", json!(12))]);
    let configured_auto_tuning = facade
        .node_classification()
        .configure_auto_tuning("nc_demo", auto_tuning_config);
    let auto_tuning_path = fixture_root.join("03-configure-auto-tuning.json");
    write_json(&auto_tuning_path, &configured_auto_tuning)?;
    println!("max trials: 12");
    println!("persisted: {}", fixture_path(&auto_tuning_path));
    println!();

    stage(
        4,
        "Node Property Step",
        "Node property steps are added as explicit pipeline inputs before training.",
    );
    let node_property_config =
        make_map(&[("mutateProperty", json!("pagerank")), ("value", json!(1.0))]);
    let with_node_property = facade.node_classification().add_node_property(
        "nc_demo",
        "gds.debug.writeConstantDouble.mutate",
        node_property_config,
    );
    let node_property_path = fixture_root.join("04-add-node-property.json");
    write_json(&node_property_path, &with_node_property)?;
    println!("added node property step: gds.debug.writeConstantDouble.mutate");
    println!("persisted: {}", fixture_path(&node_property_path));
    println!();

    stage(
        5,
        "Feature Selection",
        "Feature selection replaces the feature list used by the training pipeline.",
    );
    let selected_features = facade.node_classification().select_features(
        "nc_demo",
        Value::Array(vec![
            Value::String("pagerank".to_string()),
            Value::String("age".to_string()),
        ]),
    );
    let features_path = fixture_root.join("05-select-features.json");
    write_json(&features_path, &selected_features)?;
    println!(
        "feature count: {}",
        selected_features[0].feature_properties.len()
    );
    println!("persisted: {}", fixture_path(&features_path));
    println!();

    stage(
        6,
        "Training Methods",
        "Training methods extend the parameter space and mirror the Java facade shape.",
    );
    let logistic_regression = facade
        .node_classification()
        .add_logistic_regression("nc_demo", AnyMap::new());
    let random_forest = facade
        .node_classification()
        .add_random_forest("nc_demo", AnyMap::new());
    let mlp = facade
        .node_classification()
        .add_mlp("nc_demo", AnyMap::new());
    let methods_path = fixture_root.join("06-training-methods.json");
    write_json(
        &methods_path,
        &json!({
            "logisticRegression": logistic_regression,
            "randomForest": random_forest,
            "mlp": mlp,
        }),
    )?;
    println!("parameter-space methods: logistic regression, random forest, mlp");
    println!("persisted: {}", fixture_path(&methods_path));
    println!();

    stage(
        7,
        "Train Request Shape",
        "Training is driven by a raw procedure map; this example records the exact setup.",
    );
    let train_request = make_map(&[
        ("pipeline", json!("nc_demo")),
        ("targetNodeLabels", json!(["Person"])),
        ("relationshipTypes", json!(["KNOWS"])),
        ("targetProperty", json!("label")),
        ("randomSeed", json!(42)),
        ("concurrency", json!(4)),
        ("metrics", json!(["F1(class=*)"])),
    ]);
    let train_request_path = fixture_root.join("07-train-request.json");
    write_json(&train_request_path, &train_request)?;
    println!("would call: facade.node_classification().train(\"demo-graph\", train_request)");
    println!("persisted: {}", fixture_path(&train_request_path));
    println!();

    stage(
        8,
        "Prediction Request Shapes",
        "Stream, mutate, and write all reuse the same model-backed config surface.",
    );
    let stream_request = make_map(&[
        ("graphName", json!("demo-graph")),
        ("modelName", json!("nc_demo_model")),
        ("modelUser", json!("alice")),
        ("targetNodeLabels", json!(["Person"])),
        ("relationshipTypes", json!(["KNOWS"])),
        ("includePredictedProbabilities", json!(true)),
    ]);
    let mutate_request = make_map(&[
        ("graphName", json!("demo-graph")),
        ("modelName", json!("nc_demo_model")),
        ("modelUser", json!("alice")),
        ("mutateProperty", json!("predictedClass")),
        (
            "predictedProbabilityProperty",
            json!("predictedProbability"),
        ),
        ("writeConcurrency", json!(4)),
    ]);
    let write_request = make_map(&[
        ("graphName", json!("demo-graph")),
        ("modelName", json!("nc_demo_model")),
        ("modelUser", json!("alice")),
        ("writeProperty", json!("predictedClass")),
        (
            "predictedProbabilityProperty",
            json!("predictedProbability"),
        ),
        ("writeConcurrency", json!(4)),
    ]);
    let stream_request_path = fixture_root.join("08-stream-request.json");
    let mutate_request_path = fixture_root.join("09-mutate-request.json");
    let write_request_path = fixture_root.join("10-write-request.json");
    write_json(&stream_request_path, &stream_request)?;
    write_json(&mutate_request_path, &mutate_request)?;
    write_json(&write_request_path, &write_request)?;
    println!("stream request, mutate request, and write request shapes were captured.");
    println!("persisted: {}", fixture_path(&stream_request_path));
    println!("persisted: {}", fixture_path(&mutate_request_path));
    println!("persisted: {}", fixture_path(&write_request_path));

    let manifest_path = fixture_root.join("README.txt");
    write_text(
        &manifest_path,
        &manifest(
            &facade_path,
            &create_path,
            &split_path,
            &auto_tuning_path,
            &node_property_path,
            &features_path,
            &methods_path,
            &train_request_path,
            &stream_request_path,
            &mutate_request_path,
            &write_request_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
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

fn make_map(entries: &[(&str, Value)]) -> AnyMap {
    let mut map = AnyMap::new();
    for (key, value) in entries {
        map.insert((*key).to_string(), value.clone());
    }
    map
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/procedures/033-proc-pipelines-nc")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/procedures/033-proc-pipelines-nc/{file_name}")
}

fn manifest(
    facade_path: &Path,
    create_path: &Path,
    split_path: &Path,
    auto_tuning_path: &Path,
    node_property_path: &Path,
    features_path: &Path,
    methods_path: &Path,
    train_request_path: &Path,
    stream_request_path: &Path,
    mutate_request_path: &Path,
    write_request_path: &Path,
) -> String {
    format!(
        r#"Pipelines Node Classification Setup Fixture

Namespace: procedures::pipelines::node_classification

00 Facade
artifact: {}
meaning: the top-level facade boundary that exposes node classification setup.

01 Create Pipeline
artifact: {}
meaning: the named pipeline created in the facade catalog.

02 Configure Split
artifact: {}
meaning: the split configuration applied to the pipeline.

03 Auto Tuning
artifact: {}
meaning: the auto-tuning configuration applied to the pipeline.

04 Node Property Step
artifact: {}
meaning: a node-property step added before training.

05 Select Features
artifact: {}
meaning: the feature selection list used for node classification.

06 Training Methods
artifact: {}
meaning: the parameter-space setup for training methods.

07 Train Request
artifact: {}
meaning: the raw request map shape that would be passed to nodeClassification.train.

08 Stream Request
artifact: {}
meaning: the raw request map shape for nodeClassification.stream.

09 Mutate Request
artifact: {}
meaning: the raw request map shape for nodeClassification.mutate.

10 Write Request
artifact: {}
meaning: the raw request map shape for nodeClassification.write.
"#,
        fixture_path(facade_path),
        fixture_path(create_path),
        fixture_path(split_path),
        fixture_path(auto_tuning_path),
        fixture_path(node_property_path),
        fixture_path(features_path),
        fixture_path(methods_path),
        fixture_path(train_request_path),
        fixture_path(stream_request_path),
        fixture_path(mutate_request_path),
        fixture_path(write_request_path),
    )
}
