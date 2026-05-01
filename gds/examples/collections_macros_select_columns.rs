//! Collections macro selection example.
//!
//! Run with:
//!   cargo run -p gds --example collections_macros_select_columns

use gds::collections::dataframe::GDSFrameError;

fn main() -> Result<(), GDSFrameError> {
    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3]),
        (score: f64 => [10.0, 25.5, 7.2]),
        ("group": ["A", "B", "A"]),
    )?;

    // Build an expression with the `expr!` macro.
    // This expands to `col("score").gt(lit(20.0))`.
    let expr = gds::expr!(score > 20.0);

    // Use the Python-like `sc!` shorthand to select columns from the table.
    // Accepts either bare selectors or a bracketed list.
    let view1 = gds::sc!(table, id, score)?;
    let view2 = gds::sc!(table, [id, "group"])?;

    // Evaluate the `expr` as a filter and show results.
    let filtered = table.filter_expr(expr)?;

    println!("--- original table ---");
    println!("columns: {:?}", table.column_names());
    println!("dtypes: {:?}", table.dtypes());
    println!("{}", table.fmt_table());

    println!("--- selected view1 ---");
    println!("{}", view1.fmt_table());

    println!("--- selected view2 ---");
    println!("{}", view2.fmt_table());

    println!("--- filtered (score > 20) ---");
    println!("{}", filtered.fmt_table());

    Ok(())
}
