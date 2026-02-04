//! Collections selectors example (selector expression usage).
//!
//! Run with:
//!   cargo run -p gds --example collections_selectors

use gds::collections::dataframe::{
    selector_all, selector_by_dtype, selector_contains, selector_matches, selector_numeric,
    selector_starts_with, selector_string, selector_temporal, GDSDataFrame, GDSFrameError,
    Selector,
};
use polars::prelude::{DataType, Field, Schema, TimeUnit};

fn main() -> Result<(), GDSFrameError> {
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

    let df = GDSDataFrame::empty_with_schema(&schema);

    let assert_schema = |selector: Selector, expected: Schema| -> Result<(), GDSFrameError> {
        let selected = df.select_selector(&selector)?;
        assert_eq!(selected.dataframe().schema().as_ref(), &expected);
        Ok(())
    };

    // Select the UNION of temporal, strings and columns that start with "e".
    assert_schema(
        selector_temporal()
            .or(selector_string())
            .or(selector_starts_with("e")),
        Schema::from_iter(vec![
            Field::new("eee".into(), DataType::Boolean),
            Field::new("ghi".into(), DataType::Time),
            Field::new("JJK".into(), DataType::Date),
            Field::new("Lmn".into(), DataType::Duration(TimeUnit::Milliseconds)),
            Field::new(
                "opp".into(),
                DataType::Datetime(TimeUnit::Milliseconds, None),
            ),
            Field::new("qqR".into(), DataType::String),
        ]),
    )?;

    // Select the INTERSECTION of temporal and column names that match "opp" OR "JJK".
    assert_schema(
        selector_temporal().and(selector_matches("opp|JJK")?),
        Schema::from_iter(vec![
            Field::new("JJK".into(), DataType::Date),
            Field::new(
                "opp".into(),
                DataType::Datetime(TimeUnit::Milliseconds, None),
            ),
        ]),
    )?;

    // Select the DIFFERENCE of temporal columns and columns that contain the name "opp" OR "JJK".
    assert_schema(
        selector_temporal().exclude(selector_matches("opp|JJK")?),
        Schema::from_iter(vec![
            Field::new("ghi".into(), DataType::Time),
            Field::new("Lmn".into(), DataType::Duration(TimeUnit::Milliseconds)),
        ]),
    )?;

    // Select the SYMMETRIC DIFFERENCE of numeric columns and columns that contain an "e".
    let contains_e = selector_contains("e");
    let numeric = selector_numeric();
    let symmetric = contains_e
        .clone()
        .or(numeric.clone())
        .exclude(contains_e.and(numeric));
    assert_schema(
        symmetric,
        Schema::from_iter(vec![
            Field::new("abc".into(), DataType::UInt16),
            Field::new("bbb".into(), DataType::UInt32),
            Field::new("eee".into(), DataType::Boolean),
        ]),
    )?;

    // Select the COMPLEMENT of all columns of dtypes Duration and Time.
    assert_schema(
        selector_all().exclude(selector_by_dtype(&[
            DataType::Duration(TimeUnit::Milliseconds),
            DataType::Time,
        ])),
        Schema::from_iter(vec![
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
        ]),
    )?;

    Ok(())
}
