//! Collections selectors example (selector expression usage).
//!
//! This is intentionally silent (assert-driven) so it can be used as a quick
//! correctness check. The comments are the narrative: read top-to-bottom to
//! learn the selector DSL and schema helpers.
//!
//! Run with:
//!   cargo run -p gds --example collections_selectors

#[macro_use]
extern crate gds;

use gds::collections::dataframe::{GDSDataFrame, GDSFrameError, Selector};
use polars::prelude::Schema;

fn main() -> Result<(), GDSFrameError> {
    // Build a schema using the `fields!` macro and Polars `Schema::from_iter`.
    // This keeps the example focused on selector composition (not Schema APIs).
    let schema = Schema::from_iter(fields!(
        abc => UInt16,
        bbb => UInt32,
        cde => Float64,
        def => Float32,
        eee => Boolean,
        fgg => Boolean,
        ghi => Time,
        JJK => Date,
        Lmn => Duration(Milliseconds),
        opp => Datetime(Milliseconds, None),
        qqR => String,
    ));

    let df = GDSDataFrame::empty_with_schema(&schema);

    // Helper: apply a selector, print the resulting schema, and assert the
    // result for a quick correctness check.
    let assert_schema =
        |label: &str, selector: Selector, expected: Schema| -> Result<(), GDSFrameError> {
            let selected = df.select_selector(&selector)?;
            let schema = selected.dataframe().schema();
            println!("\n{label}");
            println!("Selected columns:");
            for (name, dtype) in schema.iter() {
                println!("  - {name}: {dtype:?}");
            }
            assert_eq!(schema.as_ref(), &expected);
            Ok(())
        };

    // UNION (OR): temporal ∪ string ∪ starts_with("e").
    // Think "any of these selectors matches".
    assert_schema(
        "UNION (OR): temporal ∪ string ∪ starts_with(\"e\")",
        sel!(temporal | string | starts_with("e")),
        Schema::from_iter(fields!(
            eee => Boolean,
            ghi => Time,
            JJK => Date,
            Lmn => Duration(Milliseconds),
            opp => Datetime(Milliseconds, None),
            qqR => String,
        )),
    )?;

    // INTERSECTION (AND): temporal ∩ matches("opp|JJK").
    // Think "must be temporal AND match the regex".
    assert_schema(
        "INTERSECTION (AND): temporal ∩ matches(\"opp|JJK\")",
        sel!(temporal & matches("opp|JJK")),
        Schema::from_iter(fields!(
            JJK => Date,
            opp => Datetime(Milliseconds, None),
        )),
    )?;

    // DIFFERENCE (EXCLUDE): temporal \ matches("opp|JJK").
    // Think "temporal, but not the regex matches".
    assert_schema(
        "DIFFERENCE (EXCLUDE): temporal \\ matches(\"opp|JJK\")",
        sel!(temporal - matches("opp|JJK")),
        Schema::from_iter(fields!(
            ghi => Time,
            Lmn => Duration(Milliseconds),
        )),
    )?;

    // SYMMETRIC DIFFERENCE (XOR): numeric ⊕ contains("e").
    // We build it from union minus intersection to show how selectors compose.
    let contains_e = sel!(contains("e"));
    let numeric = sel!(numeric);
    let symmetric = contains_e
        .clone()
        .or(numeric.clone())
        .exclude(contains_e.and(numeric));
    assert_schema(
        "SYMMETRIC DIFFERENCE (XOR): numeric ⊕ contains(\"e\")",
        symmetric,
        Schema::from_iter(fields!(
            abc => UInt16,
            bbb => UInt32,
            eee => Boolean,
        )),
    )?;

    // COMPLEMENT: all \ by_dtype([Duration, Time]).
    // This is a common "drop these types" pattern.
    assert_schema(
        "COMPLEMENT: all \\ by_dtype([Duration, Time])",
        sel!(all - by_dtype([Duration(Milliseconds), Time])),
        Schema::from_iter(fields!(
            abc => UInt16,
            bbb => UInt32,
            cde => Float64,
            def => Float32,
            eee => Boolean,
            fgg => Boolean,
            JJK => Date,
            opp => Datetime(Milliseconds, None),
            qqR => String,
        )),
    )?;

    // NOT: negate a selector directly.
    assert_schema(
        "NOT: all \\ temporal (same as NOT temporal)",
        sel!(all - temporal),
        Schema::from_iter(fields!(
            abc => UInt16,
            bbb => UInt32,
            cde => Float64,
            def => Float32,
            eee => Boolean,
            fgg => Boolean,
            qqR => String,
        )),
    )?;

    Ok(())
}
