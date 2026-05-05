//! DataFrame Join Operations walkthrough.
//!
//! Exercises the join surface on GDSDataFrame: inner join, left join,
//! full outer join (cross-key), and join_on (same key column on both sides).
//!
//! Run with:
//!   cargo run -p gds --example dataframe_join_operations

use std::fs;
use std::path::{Path, PathBuf};

use polars::prelude::{JoinArgs, JoinType};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Join Operations ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Tables",
        "Two related tables used for all join stages.",
    );

    // Products table
    let products = gds::tbl_def!(
        (product_id: i64 => [1, 2, 3, 4]),
        (name: ["Widget", "Gadget", "Doohickey", "Thingamajig"]),
        (price: f64 => [9.99, 19.99, 4.99, 29.99]),
    )?;

    // Orders table (product_id 1 and 2 appear; 5 is unknown)
    let orders = gds::tbl_def!(
        (order_id: i64 => [101, 102, 103, 104, 105]),
        (product_id: i64 => [1, 2, 1, 3, 5]),
        (qty: i64 => [2, 1, 3, 1, 2]),
    )?;

    println!("products shape: {:?}", products.shape());
    println!("{}", products.fmt_table());
    println!("orders shape: {:?}", orders.shape());
    println!("{}", orders.fmt_table());

    let prod_path = persist_csv(&products, &fixture_root, "00-products")?;
    let ord_path = persist_csv(&orders, &fixture_root, "00-orders")?;
    println!("persisted: {}", fixture_path(&prod_path));
    println!("persisted: {}", fixture_path(&ord_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Inner Join",
        "Only rows whose product_id exists in both tables are retained.",
    );

    let inner = orders.join(
        &products,
        &["product_id"],
        &["product_id"],
        JoinArgs::new(JoinType::Inner),
        None,
    )?;
    println!("inner join (orders ⋈ products on product_id):");
    println!("{}", inner.fmt_table());
    println!(
        "rows: {} (orders with unknown product_id=5 excluded)",
        inner.height()
    );

    let inner_path = persist_csv(&inner, &fixture_root, "01-inner")?;
    println!("persisted: {}", fixture_path(&inner_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Left Join",
        "All orders retained; product columns null for unknown product_id=5.",
    );

    let left = orders.join(
        &products,
        &["product_id"],
        &["product_id"],
        JoinArgs::new(JoinType::Left),
        None,
    )?;
    println!("left join (orders LEFT JOIN products on product_id):");
    println!("{}", left.fmt_table());
    println!("rows: {} (all orders present)", left.height());

    let left_path = persist_csv(&left, &fixture_root, "02-left")?;
    println!("persisted: {}", fixture_path(&left_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Full Outer Join",
        "All rows from both tables; nulls fill missing sides.",
    );

    let full = orders.join(
        &products,
        &["product_id"],
        &["product_id"],
        JoinArgs::new(JoinType::Full),
        None,
    )?;
    println!("full outer join:");
    println!("{}", full.fmt_table());
    println!("rows: {}", full.height());

    let full_path = persist_csv(&full, &fixture_root, "03-full")?;
    println!("persisted: {}", fixture_path(&full_path));
    println!();

    // ------------------------------------------------------------------ Stage 4
    stage(
        4,
        "join_on (same-key convenience)",
        "join_on uses the same column name on both sides — no duplication.",
    );

    // Re-create a small inventory table sharing product_id name
    let inventory = gds::tbl_def!(
        (product_id: i64 => [1, 2, 3]),
        (stock: i64 => [100, 50, 0]),
    )?;

    let with_stock = products.join_on(&inventory, &["product_id"], JoinType::Left, None)?;
    println!("products LEFT JOIN inventory ON product_id:");
    println!("{}", with_stock.fmt_table());

    let stock_path = persist_csv(&with_stock, &fixture_root, "04-join-on")?;
    println!("persisted: {}", fixture_path(&stock_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &prod_path,
            &ord_path,
            &inner_path,
            &left_path,
            &full_path,
            &stock_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_join_operations")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_join_operations/{file_name}")
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

fn manifest(
    prod: &Path,
    ord: &Path,
    inner: &Path,
    left: &Path,
    full: &Path,
    stock: &Path,
) -> String {
    format!(
        "DataFrame Join Operations Fixture\n\n\
         Namespace: dataframe::frame (join / join_on)\n\n\
         00 Source Tables\n\
         artifacts: {}, {}\n\
         meaning: Products (4 rows) and Orders (5 rows) tables.\n\n\
         01 Inner Join\n\
         artifact: {}\n\
         meaning: Only orders with a matching product survive; product_id=5 excluded.\n\n\
         02 Left Join\n\
         artifact: {}\n\
         meaning: All orders retained; product cols null for unknown product_id=5.\n\n\
         03 Full Outer Join\n\
         artifact: {}\n\
         meaning: All rows from both sides; nulls fill missing keys.\n\n\
         04 join_on\n\
         artifact: {}\n\
         meaning: Convenience same-key join; products LEFT JOIN inventory on product_id.\n",
        fixture_path(prod),
        fixture_path(ord),
        fixture_path(inner),
        fixture_path(left),
        fixture_path(full),
        fixture_path(stock),
    )
}
