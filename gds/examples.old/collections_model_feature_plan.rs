//! Model:Feature::Plan middle-layer walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_model_feature_plan

use gds::collections::dataframe::{col, lit, record};
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Model:Feature::Plan middle-layer walkthrough ==");
    println!("The Middle is where DataFrame real being is mediated into Dataset ideal being.");

    let table = gds::tbl_def!(
        (form_id: i64 => [1, 2, 3]),
        (text: ["source becomes readable", "appearance remains partial", "concept returns to source"]),
        (role: ["ontology", "appearance", "ontology"]),
    )?;
    let dataset = Dataset::named("middle-layer-demo", table);
    let env = PlanEnv::new().bind_dataset("middle", dataset.clone());

    let item_plan = Plan::new(Source::Var("middle".to_string()))
        .named("middle.item-plan")
        .push_step(Step::WithColumns(vec![lit("ideal").alias("being")]))
        .project_item(
            record(vec![col("form_id"), col("text"), col("role"), col("being")]).alias("item"),
        );
    let item_feature = Feature::requiring_item(item_plan.clone())?.named("middle.item");

    let trace_plan = Plan::new(Source::Var("middle".to_string()))
        .named("middle.trace-plan")
        .push_step(Step::WithColumns(vec![lit("reflected").alias("trace")]))
        .project_item(record(vec![col("form_id"), col("trace")]).alias("item"));
    let trace_feature = Feature::requiring_item(trace_plan)?.named("middle.trace");

    println!("\n[meta plan] attention report");
    let report = item_plan.attention_report(Some(&env), EvalMode::Preview);
    println!("plan: {}", report.plan_name.as_deref().unwrap_or("<anon>"));
    println!("steps: {}", report.steps.len());
    println!("observed columns: {:?}", report.observed_columns);

    let spec = ModelSpec {
        id: ModelId("middle.model-feature-plan".to_string()),
        kind: ModelKind::FeatureModel,
        input: ModelView::Mixed(vec![ModelView::Tokens, ModelView::Features]),
        output: ModelView::Features,
        description: Some("Middle-layer Model:Feature::Plan exemplar".to_string()),
    };

    let seed = parse_mark("[role='ontology']")?;
    let necessary = parse_mark("[role='ontology']")?;
    let contingent = parse_mark("[trace='reflected']")?;
    let impossible = parse_mark("[role='appearance']")?;

    let essence = prepare_model(
        spec,
        Some(seed),
        vec![
            FeatureMark::required(item_feature, necessary),
            FeatureMark::optional(trace_feature, contingent),
            FeatureMark::required(feature_stub("middle.appearance-clash"), impossible),
        ],
    )
    .map_err(|e| format!("model preparation failed: {e:?}"))?;

    println!("\n[box 1] Model preparation");
    for step in &essence.report.steps {
        println!(
            "- {} => {:?} ({})",
            step.feature_name.as_deref().unwrap_or("<anon>"),
            step.modality,
            step.note
        );
    }

    let execution = execute_essence(&essence, dataset.table().lazy().into_lazyframe());
    println!("\n[box 2] Feature execution");
    for feature in &execution.features {
        println!(
            "- {} => {:?}, applied={}",
            feature.name.as_deref().unwrap_or("<anon>"),
            feature.action,
            feature.applied
        );
    }

    let materialized = execution.lazyframe.clone().collect()?;
    println!(
        "runtime rows after essence mediation: {}",
        materialized.height()
    );
    println!("runtime columns: {:?}", materialized.get_column_names());

    let image = realize_image(
        &essence,
        &execution,
        &ImageOptions {
            engine: "model-feature-plan".to_string(),
            substrate: "dataset/dataframe".to_string(),
            source: dataset.name().unwrap_or("middle-layer-demo").to_string(),
            generated_at_unix_ms: Some(0),
            ..Default::default()
        },
    );

    println!("\n[box 3] Image realization");
    println!("models: {}", image.tables.models.len());
    println!("features: {}", image.tables.features.len());
    println!("constraints: {}", image.tables.constraints.len());
    println!("provenance: {}", image.tables.provenance.len());

    println!("Interpretation: Model:Feature::Plan is the hard middle. It is not raw Polars and not yet SemDataset; it is the Meta Plan engine that mediates real DataFrame body into ideal Dataset form.");

    Ok(())
}

fn feature_stub(name: &str) -> Feature {
    Feature::new(Plan::new(Source::Var("middle".to_string()))).named(name)
}

fn parse_mark(src: &str) -> Result<FeatStruct, String> {
    parse_featstruct(src).map_err(|e| format!("failed to parse mark `{src}`: {e:?}"))
}
