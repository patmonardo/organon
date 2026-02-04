//! RustScript string namespace example (Polars-first).
//!
//! Run with:
//!   cargo run -p gds --example collections_string_namespace_rustscript

use gds::collections::dataframe::{col, str_ns, GDSDataFrame, GDSSeries};
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    let df = df!(
        "language" => ["English", "Dutch", "Portuguese", "Finish"],
        "fruit" => ["pear", "peer", "pêra", "päärynä"],
    )?;

    let result = df
        .clone()
        .lazy()
        .with_columns([
            str_ns(col("fruit")).len_bytes().alias("byte_count"),
            str_ns(col("fruit")).len_chars().alias("letter_count"),
            str_ns(col("fruit")).to_uppercase().alias("upper"),
        ])
        .collect()?;

    println!("{}", GDSDataFrame::new(result).fmt_table());

    // GDSSeries + StringNameSpace (no DataFrame needed).
    let fruit = GDSSeries::from("fruit", ["pear", "peer", "pêra", "päärynä"]);
    let upper = fruit.str().to_uppercase()?;
    let bytes = fruit.str().len_bytes()?;

    println!("{upper:?}");
    println!("{bytes:?}");

    Ok(())
}
