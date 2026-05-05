//! DataFrame Frame:Series::Expr doctrinal fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_intuition

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{col, lit, record};
use gds::collections::dataset::prelude::*;
use gds::shell::GdsShell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Frame:Series::Expr ==");
    println!("Doctrine exemplar 028: DataFrame as Intuition and EssentialBeing.");
    println!("This is the Beginning fold before Dataset mediation takes over.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame",
        "The whole table body is the first formed enclosure of live contents.",
    );
    let frame = gds::tbl_def!(
        (row_id: i64 => [1, 2, 3]),
        (series: ["sensation", "trace", "return"]),
        (channel: ["text", "text", "text"]),
        (expr_hint: ["select", "filter", "project"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    frame.write_csv(&path_string(&frame_path))?;
    println!("artifact: GDSDataFrame");
    println!(
        "shape: {} rows x {} columns",
        frame.row_count(),
        frame.column_count()
    );
    println!("columns: {:?}", frame.column_names());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "Series",
        "Series are named typed lines of appearance inside the Frame.",
    );
    let dataset = Dataset::named("dataframe-intuition", frame.clone());
    let shell = GdsShell::from_dataset(dataset.clone());
    let seed_path = fixture_root.join("01-shell-seed.txt");
    if let Some(seed) = shell.seed() {
        fs::write(
            &seed_path,
            format!(
                "columns: {:?}\ndtypes: {:?}\nrow_count: {}\ncolumn_count: {}\n",
                seed.columns(),
                seed.dtypes(),
                seed.row_count(),
                seed.column_count(),
            ),
        )?;
        println!("seed columns: {:?}", seed.columns());
        println!("seed dtypes: {:?}", seed.dtypes());
    }
    println!("persisted: {}", fixture_path(&seed_path));
    println!();

    stage(
        2,
        "Expr",
        "Expr is executable operation still in the sphere of Being.",
    );
    let env = PlanEnv::new().bind_dataset("intuition", dataset);
    let plan = Plan::new(Source::Var("intuition".to_string()))
        .named("frame-series-expr.preview")
        .push_step(Step::WithColumns(vec![lit("being").alias("sphere")]))
        .project_item(
            record(vec![
                col("row_id"),
                col("series"),
                col("channel"),
                col("expr_hint"),
                col("sphere"),
            ])
            .alias("item"),
        );
    let report = plan.attention_report(Some(&env), EvalMode::Preview);
    let plan_path = fixture_root.join("02-expr-plan.txt");
    fs::write(
        &plan_path,
        format!(
            "plan: {:?}\nsteps: {}\nplanned_columns: {:?}\nobserved_columns: {:?}\n",
            plan.name(),
            report.steps.len(),
            report.planned_columns,
            report.observed_columns,
        ),
    )?;
    println!("plan: {:?}", plan.name());
    println!("steps: {}", report.steps.len());
    println!("observed columns: {:?}", report.observed_columns);
    println!("persisted: {}", fixture_path(&plan_path));
    println!();

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &seed_path, &plan_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/dataframe_intuition")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe_intuition/{file_name}")
}

fn manifest(frame_path: &Path, seed_path: &Path, plan_path: &Path) -> String {
    format!(
        "DataFrame Frame:Series::Expr Fixture\n\n\
         Doctrine: EXEMPLARS/028-dataframe-intuition.md\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: immediate DataFrame body.\n\n\
         01 Series\n\
         artifact: {}\n\
         meaning: Shell seed records names, dtypes, and shape.\n\n\
         02 Expr\n\
         artifact: {}\n\
         meaning: Expr appears as deferred operation before Dataset mediation.\n",
        fixture_path(frame_path),
        fixture_path(seed_path),
        fixture_path(plan_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
