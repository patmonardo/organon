//! DataFrame Order and Group walkthrough.
//!
//! Exercises sort/order_by and group_by aggregation methods on GDSDataFrame:
//! ascending/descending sort, multi-column sort, group count, group sum/mean,
//! and the full `group_by` expression API.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_order_group

use std::fs;
use std::path::{Path, PathBuf};

use polars::prelude::{col, SortMultipleOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Order and Group ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(0, "Source", "Sales table used for all sort/group stages.");

    let table = gds::tbl_def!(
        (region: ["West", "East", "West", "East", "West", "East"]),
        (product: ["A", "B", "A", "A", "B", "B"]),
        (units: i64 => [10, 20, 15, 5, 8, 30]),
        (price: f64 => [1.5, 2.0, 1.5, 2.0, 2.5, 2.0]),
    )?;

    println!("shape: {:?}", table.shape());
    println!("{}", table.fmt_table());
    let source_path = persist_csv(&table, &fixture_root, "00-source")?;
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Sort Ascending / Descending",
        "order_by_columns with SortMultipleOptions controls direction per column.",
    );

    // Sort by units ascending
    let sorted_asc = table.order_by_columns(
        &["units"],
        SortMultipleOptions::default().with_order_descending(false),
    )?;
    println!("sorted by units ASC:");
    println!("{}", sorted_asc.fmt_table());

    // Sort by region asc, then units desc
    let sorted_multi = table.order_by_columns(
        &["region", "units"],
        SortMultipleOptions::default().with_order_descending_multi([false, true]),
    )?;
    println!("sorted by region ASC / units DESC:");
    println!("{}", sorted_multi.fmt_table());

    let sort_path = persist_csv(&sorted_multi, &fixture_root, "01-sorted")?;
    println!("persisted: {}", fixture_path(&sort_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Group Count and Sum",
        "group_by_count and group_by_sum aggregate by region.",
    );

    // Row count per region
    let counts = table.group_by_count(&["region"], Some("row_count"))?;
    println!("group_by_count by region:");
    println!("{}", counts.fmt_table());

    // Sum numeric columns per region (explicit — avoids summing string cols)
    let sums = table.group_by_columns(
        &["region"],
        &[
            col("units").sum().alias("total_units"),
            col("price").sum().alias("total_price"),
        ],
    )?;
    println!("group_by sum by region:");
    println!("{}", sums.fmt_table());

    let count_path = persist_csv(&counts, &fixture_root, "02-count")?;
    persist_csv(&sums, &fixture_root, "02-sum")?;
    println!("persisted: {}", fixture_path(&count_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Group Expression API",
        "group_by_columns with explicit Expr aggregations: mean price per product.",
    );

    let product_mean = table.group_by_columns(
        &["product"],
        &[
            col("price").mean().alias("mean_price"),
            col("units").sum().alias("total_units"),
        ],
    )?;
    println!("mean price + total units by product:");
    println!("{}", product_mean.fmt_table());

    let group_path = persist_csv(&product_mean, &fixture_root, "03-group-expr")?;
    println!("persisted: {}", fixture_path(&group_path));
    println!();

    // ------------------------------------------------------------------ Stage 4
    stage(
        4,
        "Top-K / Bottom-K",
        "top_k and bottom_k select k rows by a ranking expression.",
    );

    // top_k with descending=false picks the k largest values
    let top3 = table.top_k(
        3,
        &[col("units")],
        SortMultipleOptions::default().with_order_descending(false),
    )?;
    println!("top 3 by units (largest):");
    println!("{}", top3.fmt_table());

    // bottom_k with descending=false picks the k smallest values
    let bot3 = table.bottom_k(
        3,
        &[col("units")],
        SortMultipleOptions::default().with_order_descending(false),
    )?;
    println!("bottom 3 by units (smallest):");
    println!("{}", bot3.fmt_table());

    let topk_path = persist_csv(&top3, &fixture_root, "04-top-k")?;
    persist_csv(&bot3, &fixture_root, "04-bottom-k")?;
    println!("persisted: {}", fixture_path(&topk_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &sort_path,
            &count_path,
            &group_path,
            &topk_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_order_group")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_order_group/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn persist_csv(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
}

fn manifest(source: &Path, sort: &Path, count: &Path, group: &Path, topk: &Path) -> String {
    format!(
        "DataFrame Order and Group Fixture\n\n\
         Namespace: dataframe::frame (sort / group_by)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: 6-row sales table (region, product, units, price).\n\n\
         01 Sorted\n\
         artifact: {}\n\
         meaning: Multi-column sort: region ASC, units DESC.\n\n\
         02 Count\n\
         artifact: {}\n\
         meaning: Row count per region (group_by_count).\n\n\
         03 Group Expr\n\
         artifact: {}\n\
         meaning: Mean price and total units per product via expression API.\n\n\
         04 Top-K\n\
         artifact: {}\n\
         meaning: Top 3 rows by units using top_k.\n",
        fixture_path(source),
        fixture_path(sort),
        fixture_path(count),
        fixture_path(group),
        fixture_path(topk),
    )
}
