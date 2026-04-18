//! GDSL absolute-concept Dataset walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_gdsl_absolute_concept

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== GDSL -> Dataset artifact walkthrough ==");
    println!("A semantic source file compiles into a Dataset-centered image with artifact, relation, and property tables.");
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

    println!("sample features:");
    for feature in features.features.iter().take(18) {
        println!("- {} [{}]", feature.value, feature.source);
    }

    Ok(())
}
