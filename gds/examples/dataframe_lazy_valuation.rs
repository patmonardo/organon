//! DataFrame Lazy valuation doctrinal fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_lazy_valuation

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{col, lit, GDSDataFrame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Lazy Valuation ==");
    println!("Lazy is deferred valuation: a DataFrame computation held before collection.");
    println!("This is the hinge where DataFrame Expr begins to look toward Dataset Plan.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Source Frame",
        "Lazy valuation starts from a concrete Frame.",
    );
    let frame = gds::tbl_def!(
        (row_id: i64 => [1, 2, 3, 4, 5]),
        (term: ["frame", "series", "expr", "lazy", "plan"]),
        (score: i64 => [5, 15, 25, 35, 45]),
        (phase: ["being", "being", "judgment", "deferred", "essence"]),
    )?;
    let source_path = persist_frame(&frame, &fixture_root, "00-source-frame")?;
    println!("source shape: {:?}", frame.shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Build Lazy",
        "Lazy stores an unevaluated expression graph over Frame.",
    );
    let lazy = frame
        .lazy()
        .with_columns_exprs(vec![
            (col("score") + lit(1)).alias("score_plus_one"),
            lit("lazy-valuation").alias("valuation"),
        ])
        .filter(col("score_plus_one").gt(lit(20)))
        .select_exprs(vec![
            col("row_id"),
            col("term"),
            col("phase"),
            col("score_plus_one"),
            col("valuation"),
        ]);

    let unoptimized = lazy.explain(false)?;
    let optimized = lazy.explain(true)?;
    let explain_path = fixture_root.join("01-explain.txt");
    fs::write(
        &explain_path,
        format!("unoptimized:\n{unoptimized}\n\noptimized:\n{optimized}\n"),
    )?;
    println!("artifact: LazyFrame logical plan");
    println!("persisted: {}", fixture_path(&explain_path));
    println!();

    stage(
        2,
        "Inspect Schema",
        "Lazy can reveal projected schema before full collection.",
    );
    let mut schema_lazy = lazy.clone();
    let schema = schema_lazy.collect_schema()?;
    let schema_path = fixture_root.join("02-lazy-schema.txt");
    fs::write(&schema_path, format!("schema: {:?}\n", schema))?;
    println!("schema fields: {}", schema.len());
    println!("persisted: {}", fixture_path(&schema_path));
    println!();

    stage(
        3,
        "Collect",
        "Collection turns deferred valuation back into a material Frame.",
    );
    let collected = GDSDataFrame::from(lazy.collect()?);
    let collected_path = persist_frame(&collected, &fixture_root, "03-collected")?;
    println!("collected shape: {:?}", collected.shape());
    println!("collected columns: {:?}", collected.column_names());
    println!("persisted: {}", fixture_path(&collected_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &explain_path, &schema_path, &collected_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/dataframe_lazy_valuation")
}

fn persist_frame(
    frame: &GDSDataFrame,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe_lazy_valuation/{file_name}")
}

fn manifest(
    source_path: &Path,
    explain_path: &Path,
    schema_path: &Path,
    collected_path: &Path,
) -> String {
    format!(
        "DataFrame Lazy Valuation Fixture\n\n\
         Namespace: dataframe::lazy\n\n\
         00 Source Frame\n\
         artifact: {}\n\
         meaning: concrete Frame before deferred valuation.\n\n\
         01 Explain\n\
         artifact: {}\n\
         meaning: unoptimized and optimized Lazy plans.\n\n\
         02 Lazy Schema\n\
         artifact: {}\n\
         meaning: projected schema before collection.\n\n\
         03 Collected Frame\n\
         artifact: {}\n\
         meaning: deferred valuation materialized back into Frame.\n",
        fixture_path(source_path),
        fixture_path(explain_path),
        fixture_path(schema_path),
        fixture_path(collected_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
