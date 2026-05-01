//! Dataset FeatStruct / Model preparation walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_featstruct_model

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset FeatStruct / Model preparation walkthrough ==");
    println!(
        "A Dataset model is prepared as essence before it becomes a DataFrame execution image."
    );

    let table = gds::tbl_def!(
        (form_id: i64 => [1, 2, 3, 4]),
        (text: [
            "Ontology names its own execution",
            "Projected graph appearance remains useful",
            "Feature marks preserve interpretability",
            "Runtime tables are generated images"
        ]),
        (role: ["ontology", "appearance", "ontology", "runtime"]),
    )?;

    let dataset = Dataset::named("absolute-interpretability-demo", table);
    println!("dataset columns: {:?}", dataset.column_names());

    let spec = ModelSpec {
        id: ModelId("pureform.semantic-builder".to_string()),
        kind: ModelKind::FeatureModel,
        input: ModelView::Tokens,
        output: ModelView::Features,
        description: Some("FeatStruct marks prepared for PureForm execution".to_string()),
    };

    let seed = parse_featstruct("[role='ontology']").expect("valid seed FeatStruct");
    let required_truth = parse_featstruct("[role='ontology']").expect("valid truth mark");
    let optional_trace = parse_featstruct("[trace='interpretable']").expect("valid trace mark");
    let projected_appearance =
        parse_featstruct("[role='appearance']").expect("valid appearance mark");

    let essence = prepare_model(
        spec,
        Some(seed),
        vec![
            FeatureMark::required(feature("ontology.truth"), required_truth),
            FeatureMark::optional(feature("trace.interpretability"), optional_trace),
            FeatureMark::required(feature("graph.projected-appearance"), projected_appearance),
            FeatureMark::contingent(feature("runtime.generated-image")),
        ],
    )
    .expect("model preparation should stay total for this walkthrough");

    println!("\n[box 1] prepared model essence");
    println!(
        "accumulated essence: {}",
        essence
            .accumulated
            .as_ref()
            .map(format_featstruct)
            .unwrap_or_else(|| "<none>".to_string())
    );
    for step in &essence.report.steps {
        println!(
            "- {} => {:?} ({})",
            step.feature_name.as_deref().unwrap_or("<anon>"),
            step.modality,
            step.note
        );
    }

    let lf = dataset.table().lazy().into_lazyframe();
    let execution = execute_essence(&essence, lf);

    println!("\n[box 2] execution receipt");
    for feature in &execution.features {
        println!(
            "- {} => {:?}, applied={}",
            feature.name.as_deref().unwrap_or("<anon>"),
            feature.action,
            feature.applied
        );
    }

    let filtered = execution.lazyframe.clone().collect()?;
    println!(
        "runtime rows after necessary essence filters: {}",
        filtered.height()
    );
    println!("runtime image:\n{filtered}");

    let image = realize_image(
        &essence,
        &execution,
        &ImageOptions {
            engine: "pureform-polars".to_string(),
            substrate: "dataset/dataframe".to_string(),
            source: dataset.name().unwrap_or("<unnamed>").to_string(),
            generated_at_unix_ms: Some(0),
            ..Default::default()
        },
    );

    println!("\n[box 3] ontology DataFrame image");
    println!("models: {}", image.tables.models.len());
    println!("features: {}", image.tables.features.len());
    println!("constraints: {}", image.tables.constraints.len());
    println!("provenance: {}", image.tables.provenance.len());
    for row in &image.tables.features {
        println!("- {} [{}]", row.label, row.kind);
    }

    println!(
        "\nConclusion: the graph appearance can be skipped as contradiction while the Dataset keeps the interpretable model trace."
    );

    Ok(())
}

fn feature(name: &str) -> Feature {
    Feature::new(Plan::new(Source::Var("semantic_forms".to_string()))).named(name)
}
