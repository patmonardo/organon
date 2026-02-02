# RustScript string examples (Polars-first)

These examples use the GDS Collections wrappers to keep Rust usage close to the Python ergonomics.

## String length (bytes vs chars)

```rust
use polars::prelude::*;
use gds::collections::dataframe::{col, SeriesModel};

fn main() -> PolarsResult<()> {
    let df = df!(
        "language" => ["English", "Dutch", "Portuguese", "Finish"],
        "fruit" => ["pear", "peer", "pêra", "päärynä"],
    )?;

    let result = df
        .clone()
        .lazy()
        .with_columns([
            col("fruit").str_().len_bytes().alias("byte_count"),
            col("fruit").str_().len_chars().alias("letter_count"),
        ])
        .collect()?;

    println!("{result}");

    // Pythonic Series-level variant (no DataFrame needed).
    let s = SeriesModel::from("fruit", ["pear", "peer", "pêra", "päärynä"]);
    let byte_count = s.str_().len_bytes()?;
    let letter_count = s.str_().len_chars()?;

    println!("{byte_count:?}");
    println!("{letter_count:?}");

    Ok(())
}
```
