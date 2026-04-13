use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fixture_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/gdsl/absolute-concept-scientific-inference.gdsl"
    );

    let features = DatasetToolChain::program_features_from_gdsl_file(fixture_path)?;
    let compilation = DatasetToolChain::image_from_gdsl_file(fixture_path)?;
    let materialized = compilation.materialize_artifact_datasets(&features.program_name)?;

    println!("program: {}", features.program_name);
    println!("selected forms: {}", features.selected_forms.join(", "));
    println!("feature count: {}", features.features.len());
    println!("entrypoints: {}", compilation.entrypoints.len());
    println!("artifact rows: {}", materialized.artifacts.row_count());
    println!("relation rows: {}", materialized.relations.row_count());
    println!("property rows: {}", materialized.properties.row_count());

    for feature in features.features.iter().take(18) {
        println!("feature: {} [{}]", feature.value, feature.source);
    }

    Ok(())
}
