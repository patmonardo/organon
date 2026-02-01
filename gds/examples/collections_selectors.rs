//! Collections selectors example (selector expression usage).
//!
//! Run with:
//!   cargo run -p gds --example collections_selectors

use gds::collections::dataframe::{
    selector_all, selector_by_dtype, selector_contains, selector_matches, selector_numeric,
    selector_starts_with, selector_string, selector_temporal, PolarsDataFrameCollection,
};
use polars::prelude::{DataType, Field, Schema, TimeUnit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Schema::from_iter(vec![
        Field::new("abc".into(), DataType::UInt16),
        Field::new("bbb".into(), DataType::UInt32),
        Field::new("cde".into(), DataType::Float64),
        Field::new("def".into(), DataType::Float32),
        Field::new("eee".into(), DataType::Boolean),
        Field::new("fgg".into(), DataType::Boolean),
        Field::new("ghi".into(), DataType::Time),
        Field::new("JJK".into(), DataType::Date),
        Field::new("Lmn".into(), DataType::Duration(TimeUnit::Milliseconds)),
        Field::new(
            "opp".into(),
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
        Field::new("qqR".into(), DataType::String),
    ]);

    let df = PolarsDataFrameCollection::empty_with_schema(&schema);

    // Select the UNION of temporal, strings and columns that start with "e".
    let union = selector_temporal()
        .or(selector_string())
        .or(selector_starts_with("e"));
    let selected = df.select_selector(&union)?;
    let expected = Schema::from_iter(vec![
        Field::new("eee".into(), DataType::Boolean),
        Field::new("ghi".into(), DataType::Time),
        Field::new("JJK".into(), DataType::Date),
        Field::new("Lmn".into(), DataType::Duration(TimeUnit::Milliseconds)),
        Field::new(
            "opp".into(),
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
        Field::new("qqR".into(), DataType::String),
    ]);
    assert_eq!(selected.dataframe().schema().as_ref(), &expected);

    // Select the INTERSECTION of temporal and column names that match "opp" OR "JJK".
    let intersection = selector_temporal().and(selector_matches("opp|JJK")?);
    let selected = df.select_selector(&intersection)?;
    let expected = Schema::from_iter(vec![
        Field::new("JJK".into(), DataType::Date),
        Field::new(
            "opp".into(),
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
    ]);
    assert_eq!(selected.dataframe().schema().as_ref(), &expected);

    // Select the DIFFERENCE of temporal columns and columns that contain the name "opp" OR "JJK".
    let difference = selector_temporal().exclude(selector_matches("opp|JJK")?);
    let selected = df.select_selector(&difference)?;
    let expected = Schema::from_iter(vec![
        Field::new("ghi".into(), DataType::Time),
        Field::new("Lmn".into(), DataType::Duration(TimeUnit::Milliseconds)),
    ]);
    assert_eq!(selected.dataframe().schema().as_ref(), &expected);

    // Select the SYMMETRIC DIFFERENCE of numeric columns and columns that contain an "e".
    let contains_e = selector_contains("e");
    let numeric = selector_numeric();
    let symmetric = contains_e
        .clone()
        .or(numeric.clone())
        .exclude(contains_e.and(numeric));
    let selected = df.select_selector(&symmetric)?;
    let expected = Schema::from_iter(vec![
        Field::new("abc".into(), DataType::UInt16),
        Field::new("bbb".into(), DataType::UInt32),
        Field::new("eee".into(), DataType::Boolean),
    ]);
    assert_eq!(selected.dataframe().schema().as_ref(), &expected);

    // Select the COMPLEMENT of all columns of dtypes Duration and Time.
    let complement = selector_all().exclude(selector_by_dtype(&[
        DataType::Duration(TimeUnit::Milliseconds),
        DataType::Time,
    ]));
    let selected = df.select_selector(&complement)?;
    let expected = Schema::from_iter(vec![
        Field::new("abc".into(), DataType::UInt16),
        Field::new("bbb".into(), DataType::UInt32),
        Field::new("cde".into(), DataType::Float64),
        Field::new("def".into(), DataType::Float32),
        Field::new("eee".into(), DataType::Boolean),
        Field::new("fgg".into(), DataType::Boolean),
        Field::new("JJK".into(), DataType::Date),
        Field::new(
            "opp".into(),
            DataType::Datetime(TimeUnit::Milliseconds, None),
        ),
        Field::new("qqR".into(), DataType::String),
    ]);
    assert_eq!(selected.dataframe().schema().as_ref(), &expected);

    Ok(())
}
