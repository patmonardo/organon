// collections_macros_select_columns.rs
// Example demonstrating collections macros `s!` and `sc!`.

// use gds::collections::dataframe::prelude::*;
// use std::error::Error;

fn main() {
    // Build a tiny table using the existing table builder macro. This returns
    // a Result from the builder, so we `expect` for brevity in the example.
    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3]),
        (score: f64 => [10.0, 25.5, 7.2]),
        ("group": ["A", "B", "A"]),
    )
    .expect("build table");

    // Build an expression with the `expr!` macro.
    // This expands to `col("score").gt(lit(20.0))`.
    let expr = gds::expr!(score > 20.0);

    // Use the Python-like `sc!` shorthand to select columns from the table.
    // Accepts either bare selectors or a bracketed list.
    let view1 = gds::sc!(table, id, score).expect("select cols");
    let view2 = gds::sc!(table, [id, "group"]).expect("select cols");

    // Evaluate the `expr` as a filter and show results.
    let filtered = table.filter_expr(expr).expect("filter expr");

    println!("--- original table ---");
    println!("columns: {:?}", table.column_names());
    println!("dtypes: {:?}", table.dtypes());
    println!("{:?}", table.dataframe());

    println!("--- selected view1 ---");
    println!("{:?}", view1.dataframe());

    println!("--- selected view2 ---");
    println!("{:?}", view2.dataframe());

    println!("--- filtered (score > 20) ---");
    println!("{:?}", filtered.dataframe());

    let _ = (view1, view2, filtered);
}
