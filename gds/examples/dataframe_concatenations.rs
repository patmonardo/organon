//! DataFrame Concatenations walkthrough.
//!
//! Focused exemplar for vertical concatenation pipelines using DSL macros
//! and expression helper functions.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_concatenations

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Concatenations ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Batches",
        "Monthly batches share one schema and are ready for row-wise concatenation.",
    );

    let jan = gds::tbl_def!(
        (order_id: i64 => [1001, 1002, 1003]),
        (region: ["north", "north", "west"]),
        (channel: ["web", "store", "web"]),
        (amount: f64 => [120.0, 80.0, 65.0]),
    )?;

    let feb = gds::tbl_def!(
        (order_id: i64 => [1004, 1005, 1006]),
        (region: ["south", "north", "west"]),
        (channel: ["web", "store", "store"]),
        (amount: f64 => [90.0, 140.0, 75.0]),
    )?;

    let mar = gds::tbl_def!(
        (order_id: i64 => [1007, 1008]),
        (region: ["south", "west"]),
        (channel: ["store", "web"]),
        (amount: f64 => [110.0, 95.0]),
    )?;

    let s0_jan = persist_csv(&jan, &fixture_root, "00-jan")?;
    let s0_feb = persist_csv(&feb, &fixture_root, "00-feb")?;
    let s0_mar = persist_csv(&mar, &fixture_root, "00-mar")?;

    println!("jan shape: {:?}", jan.shape());
    println!("feb shape: {:?}", feb.shape());
    println!("mar shape: {:?}", mar.shape());
    println!("persisted: {}", fixture_path(&s0_jan));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "concat_rows Macro",
        "Row-wise concatenation is expressed in one macro call over batch frames.",
    );

    let all_sales = gds::concat_rows!(jan, feb, mar)?;

    println!("all-sales shape: {:?}", all_sales.shape());
    println!("all-sales rows: {}", all_sales.height());
    let s1_all = persist_csv(&all_sales, &fixture_root, "01-all-sales")?;
    println!("persisted: {}", fixture_path(&s1_all));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Macro + Function Enrichment",
        "mutate! uses expression helper functions to build scriptable route labels.",
    );

    let labeled = gds::mutate!(
        all_sales,
        route = {
            gds::collections::dataframe::functions::concat_str(
                &[gds::col!("region"), gds::lit!("::"), gds::col!("channel")],
                "",
                true,
            )
        },
        amount_k = { gds::expr!(amount / 1000.0) },
    )?;

    let ordered = gds::arrange!(labeled, [(region, asc), (amount, desc)])?;

    println!("ordered shape: {:?}", ordered.shape());
    println!("{}", ordered.fmt_table());
    let s2_ordered = persist_csv(&ordered, &fixture_root, "02-ordered")?;
    println!("persisted: {}", fixture_path(&s2_ordered));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Grouped Rollup",
        "group_by! + agg! produce route-level totals over concatenated data.",
    );

    let rollup = gds::group_by!(
        ordered,
        [route],
        [
            gds::agg!(amount.sum => "amount_total"),
            gds::agg!(order_id.count => "rows")
        ]
    )?;

    let rollup_sorted = gds::arrange!(rollup, [amount_total], desc)?;

    println!("rollup shape: {:?}", rollup_sorted.shape());
    println!("{}", rollup_sorted.fmt_table());
    let s3_rollup = persist_csv(&rollup_sorted, &fixture_root, "03-rollup")?;
    println!("persisted: {}", fixture_path(&s3_rollup));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&s0_jan, &s0_feb, &s0_mar, &s1_all, &s2_ordered, &s3_rollup),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_concatenations")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_concatenations/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn persist_csv(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
    println!();
}

fn manifest(
    s0_jan: &Path,
    s0_feb: &Path,
    s0_mar: &Path,
    s1_all: &Path,
    s2_ordered: &Path,
    s3_rollup: &Path,
) -> String {
    format!(
        "DataFrame Concatenations Fixture\n\n\
         Namespace: dataframe::concatenations\n\n\
         00 Source Batches\n\
         jan: {}\n\
         feb: {}\n\
         mar: {}\n\
         meaning: monthly partitions with one stable schema.\n\n\
         01 Concatenated Rows\n\
         artifact: {}\n\
         meaning: concat_rows! stacks all batch rows in one frame.\n\n\
         02 Enriched/Ordered\n\
         artifact: {}\n\
         meaning: mutate! + concat_str helper create route labels.\n\n\
         03 Route Rollup\n\
         artifact: {}\n\
         meaning: group_by! with agg! summarizes totals after concatenation.\n",
        fixture_path(s0_jan),
        fixture_path(s0_feb),
        fixture_path(s0_mar),
        fixture_path(s1_all),
        fixture_path(s2_ordered),
        fixture_path(s3_rollup),
    )
}
