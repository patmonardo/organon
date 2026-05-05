//! Dataset Frame DSL core walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_frame_dsl

use gds::shell::*;

fn main() -> Result<(), GDSFrameError> {
    println!("== Dataset Frame DSL core ==");
    println!("Dataset authors the semantic program; DataFrame executes the analytic body.");

    let table = tbl_def!(
        (doc_id: i64 => [1, 2, 3]),
        (text: [
            "Reason gives form to experience",
            "Dataset frames semantic programs",
            "The kernel lowers language into tables"
        ]),
    )?;

    let ds_frame = ds_frame(table)
        .named("semantic-builder-demo")
        .artifact_kind(DatasetArtifactKind::Corpus)
        .facet("text")
        .source_io(io_path("memory://semantic-builder-demo"));

    let dataset = ds_frame.clone().into_dataset();
    println!("dataset columns: {:?}", dataset.column_names());

    let shell = GdsShell::from_dataset(dataset.clone());
    let descriptor = shell.descriptor();
    println!("shell register: {:?}", shell.register());
    println!("shell address: {:?}", shell.address());
    println!("shell has metapipeline: {}", descriptor.has_metapipeline());
    if let Some(seed) = shell.seed() {
        println!("shell seed columns: {:?}", seed.columns());
        println!("shell seed dtypes: {:?}", seed.dtypes());
        println!(
            "shell featstruct schema: {}",
            format_featstruct(seed.schema().feature_structure())
        );
    }

    let _lazy_shell = ds_frame.lazy();
    println!("2x2 shell entered through DataFrame.ds().lazy()");

    let pipeline = ds_frame
        .pipeline()
        .with_op(text_input("article.input"))
        .with_op(text_encode("article.encode"))
        .with_op(text_transform("article.transform"))
        .with_op(text_decode("article.decode"))
        .with_op(text_output("article.output"))
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

    let semantic_view = mutate!(
        dataset,
        normalized_text = { ds_col("text").text().lowercase() },
    )?;
    println!("semantic view:\n{}", semantic_view.table().fmt_table());

    let compilation = pipeline.to_compilation();
    compilation.validate().map_err(GDSFrameError::from)?;
    println!("compilation nodes: {}", compilation.nodes.len());
    println!("entrypoints: {}", compilation.entrypoints.len());

    Ok(())
}
