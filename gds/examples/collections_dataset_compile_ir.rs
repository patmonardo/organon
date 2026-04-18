//! Dataset compilation IR walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_compile_ir

use gds::collections::dataset::prelude::*;
use gds::form::{ProgramFeature, ProgramFeatureKind, ProgramFeatures};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset compilation IR / artifact story ==");
    let features = ProgramFeatures::new(
        "gdsl.analytics".to_string(),
        vec!["centrality".to_string()],
        vec![
            ProgramFeature::new(
                ProgramFeatureKind::SpecificationBinding,
                "gdsl.analytics".to_string(),
                "specification::gdsl.analytics".to_string(),
            ),
            ProgramFeature::new(
                ProgramFeatureKind::OperatorPattern,
                "algo.pagerank".to_string(),
                "operator_pattern::algo.pagerank".to_string(),
            ),
            ProgramFeature::new(
                ProgramFeatureKind::Dependency,
                "collections.dataset".to_string(),
                "dependency::collections.dataset".to_string(),
            ),
        ],
    );

    let compilation = DatasetCompilation::from_program_features(&features);
    compilation.validate()?;

    let materialized = compilation.materialize_artifact_datasets(&features.program_name)?;

    let mut registry = DatasetRegistry::new(std::env::temp_dir().join("gds_dataset_registry_demo"));
    registry.register_dataset(&materialized.artifacts)?;
    registry.register_dataset(&materialized.relations)?;
    registry.register_dataset(&materialized.properties)?;

    println!("entrypoints: {}", compilation.entrypoints.len());
    println!("artifact rows: {}", materialized.artifacts.row_count());
    println!("relation rows: {}", materialized.relations.row_count());
    println!("property rows: {}", materialized.properties.row_count());
    println!("registry view:");

    for metadata in registry.list() {
        println!(
            "- {} [{}] {}",
            metadata.name,
            metadata.artifact_profile.primary_kind(),
            metadata.artifact_profile.facets().join(", ")
        );
    }

    Ok(())
}
