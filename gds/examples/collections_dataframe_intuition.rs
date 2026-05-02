//! DataFrame as Intuition: the first moment of the Dataset pipeline.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataframe_intuition

use gds::collections::dataframe::{col, lit, record};
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame as Intuition ==");
    println!("The first moment is the DataFrame itself: raw contents gathered into tabular consciousness.");

    let table = gds::tbl_def!(
        (row_id: i64 => [1, 2, 3]),
        (sensation: ["red mark", "blue trace", "red return"]),
        (channel: ["text", "text", "text"]),
    )?;

    println!("\n[being] DataFrame as immediate body");
    println!("shape: {:?}", table.shape());
    println!("series: {:?}", table.column_names());
    println!("dtypes: {:?}", table.dtypes());
    println!("{}", table.fmt_table());

    let dataset = Dataset::named("dataframe-intuition-demo", table.clone());
    let env = PlanEnv::new().bind_dataset("intuition", dataset.clone());

    let intuition_plan = Plan::new(Source::Var("intuition".to_string()))
        .named("intuition.expr-to-plan")
        .push_step(Step::WithColumns(vec![lit("intuition").alias("sphere")]))
        .project_item(
            record(vec![
                col("row_id"),
                col("sensation"),
                col("channel"),
                col("sphere"),
            ])
            .alias("item"),
        );
    let intuition_feature =
        Feature::requiring_item(intuition_plan.clone())?.named("intuition.series-to-feature");

    println!("\n[passage] Frame:Series::Expr -> Model:Feature::Plan");
    println!(
        "Frame  -> Model   : {}",
        dataset.name().unwrap_or("<unnamed>")
    );
    println!(
        "Series -> Feature : {}",
        intuition_feature.name().unwrap_or("<anon>")
    );
    println!(
        "Expr   -> Plan    : {}",
        intuition_plan.name().unwrap_or("<anon>")
    );

    let report = intuition_plan.attention_report(Some(&env), EvalMode::Preview);
    println!("plan steps: {}", report.steps.len());
    println!("observed columns: {:?}", report.observed_columns);

    let spec = ModelSpec {
        id: ModelId("intuition.model".to_string()),
        kind: ModelKind::FeatureModel,
        input: ModelView::Features,
        output: ModelView::Features,
        description: Some("DataFrame-as-Intuition first-moment exemplar".to_string()),
    };

    let seed = parse_mark("[channel='text']")?;
    let body_mark = parse_mark("[channel='text']")?;
    let reflected_mark = parse_mark("[sphere='intuition']")?;

    let essence = prepare_model(
        spec,
        Some(seed),
        vec![
            FeatureMark::required(intuition_feature, body_mark),
            FeatureMark::optional(feature_stub("intuition.sphere"), reflected_mark),
        ],
    )
    .map_err(|e| format!("model preparation failed: {e:?}"))?;

    println!("\n[essence] first stamp of the DataFrame body");
    for step in &essence.report.steps {
        println!(
            "- {} => {:?} ({})",
            step.feature_name.as_deref().unwrap_or("<anon>"),
            step.modality,
            step.note
        );
    }

    println!("\n[doctrine]");
    println!(
        "DataFrame is Intuition: Frame, Series, and Expr are the immediate forms of appearance."
    );
    println!("Essence reflects them as Model, Feature, and Plan.");
    println!("Concept receives them as Corpus, Language, and Semantics.");

    Ok(())
}

fn feature_stub(name: &str) -> Feature {
    Feature::new(Plan::new(Source::Var("intuition".to_string()))).named(name)
}

fn parse_mark(src: &str) -> Result<FeatStruct, String> {
    parse_featstruct(src).map_err(|e| format!("failed to parse mark `{src}`: {e:?}"))
}
