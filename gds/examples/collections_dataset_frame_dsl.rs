//! Dataset Frame DSL core walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_frame_dsl

use gds::collections::dataframe::GDSFrameError;
use gds::collections::dataset::namespace::*;
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), GDSFrameError> {
    println!("== Dataset Frame DSL core ==");
    println!("Dataset authors the semantic program; DataFrame executes the analytic body.");

    let table = gds::tbl_def!(
        (doc_id: i64 => [1, 2, 3]),
        (text: [
            "Reason gives form to experience",
            "Dataset frames semantic programs",
            "The kernel lowers language into tables"
        ]),
    )?;

    let dataset = Dataset::named("semantic-builder-demo", table);
    println!("dataset columns: {:?}", dataset.column_names());

    let ds_frame = dataset.table().clone().ds();
    let _lazy_shell = ds_frame.lazy();
    println!("2x2 shell entered through DataFrame.ds().lazy()");

    let pipeline = gds::pipeline!(text "article")
        .with_io(io_path("memory://semantic-builder-demo"))
        .with_metadata(metadata("framework", "dataset-frame-dsl"))
        .with_projection(project_text(["doc_id", "text", "token_count"]))
        .with_report(report_summary().with_title("Semantic builder summary"));

    let artifacts = pipeline.build_artifacts("text");
    println!("pipeline ops: {}", pipeline.ops.len());
    println!(
        "dataset aspect artifacts: {}",
        artifacts.dataset_aspects.len()
    );
    println!(
        "dataframe lowerings: {}",
        artifacts.dataframe_lowerings.len()
    );

    let lowered = pipeline.lower_to_dataframe_exprs("text");
    for (index, expr) in lowered.iter().enumerate() {
        println!("lowering[{index}]: {expr:?}");
    }

    let semantic_view = dataset.with_columns(&[gds::col!(text)
        .ds()
        .text()
        .lowercase()
        .alias("normalized_text")])?;
    println!("semantic view:\n{}", semantic_view.table().fmt_table());

    let compilation = pipeline.to_compilation();
    compilation.validate().map_err(GDSFrameError::from)?;
    println!("compilation nodes: {}", compilation.nodes.len());
    println!("entrypoints: {}", compilation.entrypoints.len());

    Ok(())
}
