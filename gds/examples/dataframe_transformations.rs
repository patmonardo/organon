//! DataFrame Transformations workbench.
//!
//! A large, staged workbench exemplar for transformation parity review.
//! It focuses on eager DataFrame surfaces currently available in GDS wrappers,
//! while making boundary gaps explicit (for example, eager pivot).
//!
//! Run with:
//!   cargo run -p gds --example dataframe_transformations

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::{
    col, FillNullStrategy, PolarsSortMultipleOptions, UniqueKeepStrategy,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Transformations ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source",
        "A compact modeling table with duplicated entities, wide metrics, and NaN values.",
    );

    let source = gds::tbl_def!(
        (doc_id: i64 => [100, 101, 102, 103, 104, 105, 106, 107]),
        (domain: ["news", "news", "biomed", "biomed", "news", "legal", "legal", "news"]),
        (split: ["train", "train", "train", "dev", "dev", "train", "dev", "dev"]),
        (metric_a: f64 => [0.81, 0.72, 0.91, 0.65, 0.79, 0.88, 0.61, 0.79]),
        (metric_b: f64 => [0.42, 0.33, 0.51, 0.48, 0.39, 0.55, 0.28, 0.39]),
        (quality: f64 => [0.92, f64::NAN, 0.88, 0.71, f64::NAN, 0.84, 0.63, 0.76]),
    )?;

    println!("shape: {:?}", source.shape());
    println!("{}", source.fmt_table());
    let source_path = persist_csv(&source, &fixture_root, "00-source")?;
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Order + Mutate",
        "Sort rows and derive weighted fields used by downstream transformations.",
    );

    let ordered = gds::arrange!(source, [(domain, asc), (metric_a, desc)])?;
    let enriched = gds::mutate!(
        ordered,
        weighted = (metric_a * 0.7 + metric_b * 0.3),
        confidence = (quality * 100.0),
    )?;

    println!("ordered/enriched shape: {:?}", enriched.shape());
    println!("{}", enriched.fmt_table());
    let stage1_path = persist_csv(&enriched, &fixture_root, "01-order-mutate")?;
    println!("persisted: {}", fixture_path(&stage1_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Unpivot",
        "Reshape wide metric columns into long form for metric-oriented pipelines.",
    );

    let long = gds::unpivot!(
        enriched,
        on = [metric_a, metric_b, weighted],
        index = [doc_id, domain, split],
        variable = "metric",
        value = "value"
    )?;

    println!("long shape: {:?}", long.shape());
    println!("{}", long.head(12).fmt_table());
    let stage2_path = persist_csv(&long, &fixture_root, "02-unpivot")?;
    println!("persisted: {}", fixture_path(&stage2_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Melt Alias",
        "Melt is treated as unpivot alias with id/value terminology.",
    );

    let melted = gds::melt!(
        enriched,
        id = [doc_id, domain, split],
        values = [metric_a, metric_b, weighted],
        variable = "feature",
        value = "score"
    )?;

    println!("melted shape: {:?}", melted.shape());
    println!("{}", melted.head(12).fmt_table());
    let stage3_path = persist_csv(&melted, &fixture_root, "03-melt")?;
    println!("persisted: {}", fixture_path(&stage3_path));
    println!();

    // ------------------------------------------------------------------ Stage 4
    stage(
        4,
        "Null/NaN Normalization",
        "Repair NaN quality values and drop rows that still violate required fields.",
    );

    let repaired_nan = enriched.fill_nan(gds::lit!(0.0))?;
    let repaired_null = repaired_nan.fill_null(gds::lit!(0.0))?;
    let cleaned = repaired_null.drop_nulls(Some(&["quality", "metric_a", "metric_b"]))?;

    println!("cleaned shape: {:?}", cleaned.shape());
    println!("{}", cleaned.fmt_table());
    let stage4_path = persist_csv(&cleaned, &fixture_root, "04-cleaned")?;
    println!("persisted: {}", fixture_path(&stage4_path));
    println!();

    // ------------------------------------------------------------------ Stage 5
    stage(
        5,
        "Distinct + Group",
        "Build deduplicated slices and grouped summaries for dataset planning.",
    );

    let distinct_docs = cleaned.unique(
        Some(&["domain", "split", "doc_id"]),
        UniqueKeepStrategy::First,
    )?;
    let summary = gds::group_by!(
        cleaned,
        [domain, split],
        [
            gds::agg!(weighted.mean => "mean_weighted"),
            gds::agg!(quality.mean => "mean_quality"),
            gds::agg!(doc_id.count => "rows")
        ]
    )?;

    println!("distinct shape: {:?}", distinct_docs.shape());
    println!("summary shape: {:?}", summary.shape());
    println!("{}", summary.fmt_table());
    let stage5a_path = persist_csv(&distinct_docs, &fixture_root, "05-distinct")?;
    let stage5b_path = persist_csv(&summary, &fixture_root, "05-summary")?;
    println!("persisted: {}", fixture_path(&stage5b_path));
    println!();

    // ------------------------------------------------------------------ Stage 6
    stage(
        6,
        "Ranked Views",
        "Top-K and Bottom-K create boundary slices over weighted quality.",
    );

    let top3 = cleaned.top_k(
        3,
        &[col("weighted")],
        PolarsSortMultipleOptions::default().with_order_descending(false),
    )?;
    let bottom3 = cleaned.bottom_k(
        3,
        &[col("weighted")],
        PolarsSortMultipleOptions::default().with_order_descending(false),
    )?;

    println!("top3:");
    println!("{}", top3.fmt_table());
    println!("bottom3:");
    println!("{}", bottom3.fmt_table());
    let stage6a_path = persist_csv(&top3, &fixture_root, "06-top-k")?;
    let stage6b_path = persist_csv(&bottom3, &fixture_root, "06-bottom-k")?;
    println!("persisted: {}", fixture_path(&stage6a_path));
    println!();

    // ------------------------------------------------------------------ Stage 7
    stage(
        7,
        "Dummies + Sampling",
        "Feature-like one-hot columns and deterministic sampling for quick experiments.",
    );

    let dummies = cleaned.to_dummies(Some(&["domain", "split"]), Some("__"), false, false)?;
    let sampled = cleaned.sample(Some(4), None, false, true, Some(7))?;
    let every2 = cleaned.gather_every(2, 0)?;

    println!("dummies columns: {}", dummies.column_count());
    println!("sampled shape: {:?}", sampled.shape());
    println!("every2 shape: {:?}", every2.shape());
    let stage7a_path = persist_csv(&dummies, &fixture_root, "07-dummies")?;
    let stage7b_path = persist_csv(&sampled, &fixture_root, "07-sampled")?;
    let stage7c_path = persist_csv(&every2, &fixture_root, "07-gather-every")?;
    println!("persisted: {}", fixture_path(&stage7a_path));
    println!();

    // ------------------------------------------------------------------ Stage 8
    stage(
        8,
        "Boundary: Eager Pivot",
        "Pivot remains an explicit wrapper gap and is captured as a fixture contract.",
    );

    let pivot_note = match cleaned.pivot() {
        Ok(df) => format!("unexpected success: pivot shape {:?}", df.shape()),
        Err(err) => format!("expected boundary: {err}"),
    };
    println!("{pivot_note}");

    let stage8_path = fixture_root.join("08-pivot-boundary.txt");
    fs::write(&stage8_path, format!("{pivot_note}\n"))?;
    println!("persisted: {}", fixture_path(&stage8_path));
    println!();

    // ------------------------------------------------------------------ Stage 9
    stage(
        9,
        "Strategy Fill",
        "fill_null_strategy is included to document strategy-path behavior.",
    );

    let strategy_filled = cleaned.fill_null_strategy(FillNullStrategy::Forward(None))?;
    println!("strategy-filled shape: {:?}", strategy_filled.shape());
    let stage9_path = persist_csv(&strategy_filled, &fixture_root, "09-fill-null-strategy")?;
    println!("persisted: {}", fixture_path(&stage9_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &stage1_path,
            &stage2_path,
            &stage3_path,
            &stage4_path,
            &stage5a_path,
            &stage5b_path,
            &stage6a_path,
            &stage6b_path,
            &stage7a_path,
            &stage7b_path,
            &stage7c_path,
            &stage8_path,
            &stage9_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_transformations")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_transformations/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn persist_csv(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
    println!();
}

#[allow(clippy::too_many_arguments)]
fn manifest(
    source: &Path,
    s1: &Path,
    s2: &Path,
    s3: &Path,
    s4: &Path,
    s5a: &Path,
    s5b: &Path,
    s6a: &Path,
    s6b: &Path,
    s7a: &Path,
    s7b: &Path,
    s7c: &Path,
    s8: &Path,
    s9: &Path,
) -> String {
    format!(
        "DataFrame Transformations Fixture\n\n\
         Namespace: dataframe::frame (transformations workbook)\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: baseline wide table with duplicates and NaN quality values.\n\n\
         01 Order + Mutate\n\
         artifact: {}\n\
         meaning: sorted rows plus weighted/confidence derived columns.\n\n\
         02 Unpivot\n\
         artifact: {}\n\
         meaning: wide metrics reshaped into long metric/value records.\n\n\
         03 Melt\n\
         artifact: {}\n\
         meaning: alias path to unpivot with feature/score naming.\n\n\
         04 Null/NaN Normalization\n\
         artifact: {}\n\
         meaning: NaN/null repair and required-field null dropping.\n\n\
         05 Distinct\n\
         artifact: {}\n\
         meaning: deduplicated domain/split/doc rows.\n\n\
         05 Summary\n\
         artifact: {}\n\
         meaning: grouped means and row counts per domain/split.\n\n\
         06 Top-K\n\
         artifact: {}\n\
         meaning: highest weighted rows.\n\n\
         06 Bottom-K\n\
         artifact: {}\n\
         meaning: lowest weighted rows.\n\n\
         07 Dummies\n\
         artifact: {}\n\
         meaning: one-hot expansion of domain/split categories.\n\n\
         07 Sampled\n\
         artifact: {}\n\
         meaning: deterministic sample for quick iteration.\n\n\
         07 Gather Every\n\
         artifact: {}\n\
         meaning: stride-based thinning of rows.\n\n\
         08 Pivot Boundary\n\
         artifact: {}\n\
         meaning: explicit wrapper gap contract for eager pivot.\n\n\
         09 Fill Strategy\n\
         artifact: {}\n\
         meaning: strategy-path null fill surface documentation.\n",
        fixture_path(source),
        fixture_path(s1),
        fixture_path(s2),
        fixture_path(s3),
        fixture_path(s4),
        fixture_path(s5a),
        fixture_path(s5b),
        fixture_path(s6a),
        fixture_path(s6b),
        fixture_path(s7a),
        fixture_path(s7b),
        fixture_path(s7c),
        fixture_path(s8),
        fixture_path(s9),
    )
}
