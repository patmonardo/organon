//! Collections catalog scan + selector example (LazyFrame).
//!
//! Run with:
//!   cargo run -p gds --example collections_catalog_scan_select

use std::path::PathBuf;

use gds::collections::catalog::types::CollectionsIoFormat;
use gds::collections::dataframe::{GDSDataFrame, GDSFrameError};
use gds::collections::extensions::catalog::{CatalogExtension, CatalogExtensionConfig};

fn main() -> Result<(), GDSFrameError> {
    let root = PathBuf::from("target/collections_catalog_scan_select");

    let config = CatalogExtensionConfig {
        default_format: CollectionsIoFormat::Parquet,
        infer_schema_on_write: true,
        infer_schema_on_read: true,
        validate_on_read: false,
        eager: true,
        auto_save: true,
        ..Default::default()
    };

    let mut catalog = CatalogExtension::new(&root)?.with_config(config);

    let table = gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2]),
    )?;

    catalog.write_table("sample_table", &table, None)?;

    let selector = gds::sel!(by_name([id, score]));
    let lazy = catalog.scan_table_select("sample_table", &selector)?;
    let df = lazy
        .select([
            gds::col!(id),
            (gds::col!(score) * gds::lit!(2.0)).alias("score_x2"),
        ])
        .collect()?;

    println!(
        "Lazy scan + selector result:\n{}",
        GDSDataFrame::new(df).fmt_table()
    );

    Ok(())
}
