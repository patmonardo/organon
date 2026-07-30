//! Typed Semantic Dataset automation through the GDS Shell.
//!
//! Run with:
//!   cargo run -p gds --example shell_knowledge_script

use gds::collections::dataframe::{col, lit};
use gds::collections::dataset::prelude::*;
use gds::shell::{GdsShell, ShellScript};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let frame = gds::tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["model", "feature", "plan"]),
        (role: ["name", "structure", "order"]),
    )?;
    let program = program_features(
        "shell.knowledge-script",
        ["KnowledgeScript"],
        [
            program_principle("dataset-governs-dataframe-executes"),
            program_concept("KnowledgeScript"),
            program_procedure("derive-mediated-view"),
        ],
    );
    let shell =
        GdsShell::from_dataset(Dataset::named("knowledge", frame)).with_program_features(program);

    let script = ShellScript::new("derive-mediated-view")
        .with_run_id("shell-knowledge-script")
        .with_columns([lit("mediated").alias("state")])
        .select([col("term_id"), col("term"), col("role"), col("state")]);

    let result = shell.run_script(&script)?;
    let knowledge = result.shell().dataframe_knowledge();

    println!("script:\n{}", script.describe());
    println!("address: {:?}", result.after_address());
    println!("rows: {}", knowledge.expect("knowledge body").row_count());
    println!("columns: {:?}", result.report().observed_columns);
    println!("print: {}", result.print().id);

    Ok(())
}
