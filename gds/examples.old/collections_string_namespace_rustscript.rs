//! RustScript string namespace example (GDS DataFrame DSL).
//!
//! Run with:
//!   cargo run -p gds --example collections_string_namespace_rustscript

use gds::collections::dataframe::{str_ns, GDSFrameError, GDSSeries};

fn main() -> Result<(), GDSFrameError> {
    let df = gds::tbl_def!(
        (language: ["English", "Dutch", "Portuguese", "Finish"]),
        (fruit: ["pear", "peer", "pêra", "päärynä"]),
    )?;

    let result = gds::mutate!(
        df,
        byte_count = { str_ns(gds::col!(fruit)).len_bytes() },
        letter_count = { str_ns(gds::col!(fruit)).len_chars() },
        upper = { str_ns(gds::col!(fruit)).to_uppercase() },
    )?;

    println!("{}", result.fmt_table());

    // GDSSeries + StringNameSpace (no DataFrame needed).
    let fruit = GDSSeries::from("fruit", ["pear", "peer", "pêra", "päärynä"]);
    let upper = fruit.str().to_uppercase()?;
    let bytes = fruit.str().len_bytes()?;

    println!("{upper:?}");
    println!("{bytes:?}");

    Ok(())
}
