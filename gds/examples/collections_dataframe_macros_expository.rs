//! Expository DataFrame macro walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataframe_macros_expository

use std::error::Error;

use gds::collections::dataframe::GDSDataFrame;

fn print_step(name: &str, df: &GDSDataFrame) {
    println!("\n== {name} ==");
    println!("rows: {}, cols: {}", df.row_count(), df.column_count());
    println!("columns: {:?}", df.column_names());
    println!("{:?}", df.dataframe());
}

fn main() -> Result<(), Box<dyn Error>> {
    let students = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [62.0, 74.5, 91.0, 88.0, 97.0]),
        (cohort: ["A", "A", "B", "B", "A"]),
    )?;
    print_step("students", &students);

    let enriched = gds::mutate!(
        students,
        score_x2 = (score * 2.0),
        passed = (score >= 70.0),
        honors = (score >= 90.0),
    )?;
    print_step("enriched", &enriched);

    let shortlisted = gds::where_!(enriched, passed && (score_x2 > 150.0))?;
    print_step("shortlisted", &shortlisted);

    let ordered = gds::arrange!(shortlisted, [score_x2], desc)?;
    print_step("ordered", &ordered);

    let summary = gds::summarize!(ordered, id, cohort, score, score_x2, passed, honors)?;
    print_step("summary", &summary);

    let labels = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (label: ["bronze", "silver", "gold", "gold", "platinum"]),
    )?;
    print_step("labels", &labels);

    let joined = gds::join!(summary, labels, on = [id], how = left)?;
    print_step("joined", &joined);

    let piped = gds::pipe!(
        joined.clone(),
        |t: GDSDataFrame| gds::where_!(t, score >= 85.0).expect("pipeline filter"),
        |t: GDSDataFrame| gds::arrange!(t, [score], desc).expect("pipeline order"),
    );
    print_step("piped", &piped);

    let lazy_out = gds::q!(
        piped
            => with_columns_exprs(vec![
                gds::otherwise!(
                    gds::then!(gds::expr!(score >= 90.0) => gds::lit!("A")),
                    gds::lit!("B")
                )
                .alias("grade")
            ])
            => select_exprs(vec![
                gds::col!(id),
                gds::col!(cohort),
                gds::col!(score),
                gds::col!(label),
                gds::col!(grade),
            ])
    )
    .collect()?;
    let lazy_out_df = GDSDataFrame::new(lazy_out.clone());
    print_step("lazy_out", &lazy_out_df);

    let roundtrip_bytes = lazy_out_df.serialize()?;
    let roundtrip_df = GDSDataFrame::deserialize(&roundtrip_bytes)?;
    print_step("roundtrip_df", &roundtrip_df);

    // IO parity note: eager write_* hooks are currently explicit placeholders.
    // This demonstrates the current behavior in examples, without hiding it.
    match roundtrip_df.write_parquet("target/collections_dataframe_macros_expository.parquet") {
        Ok(_) => println!("write_parquet succeeded"),
        Err(error) => println!("write_parquet currently unavailable in wrapper path: {error}"),
    }

    Ok(())
}
