//! RustScript string namespace example (Polars-first).
//!
//! Run with:
//!   cargo run -p gds --example collections_string_namespace_rustscript

use gds::collections::dataframe::{col, str_ns, SeriesModel};
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

    println!("{result}");

    // SeriesModel + StringNameSpace (no DataFrame needed).
    let fruit = SeriesModel::from("fruit", ["pear", "peer", "pêra", "päärynä"]);
    let upper = fruit.str_().to_uppercase()?;
    let bytes = fruit.str_().len_bytes()?;

    println!("{upper:?}");
    println!("{bytes:?}");

    Ok(())
}
