//! DataFrame Pivot/Unpivot Operations workbench.
//!
//! This exemplar isolates reshape semantics (unpivot/melt/pivot boundary)
//! as a separate example from join operations.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_pivot_unpivot_operations

use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Pivot/Unpivot Operations ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source Table",
        "A compact wide table for reshape operations (no join dependency).",
    );

    let source = gds::tbl_def!(
        (customer_id: i64 => [1, 1, 2, 2, 3]),
        (segment: ["enterprise", "enterprise", "startup", "startup", "public"]),
        (qty: i64 => [3, 1, 2, 4, 5]),
        (revenue: f64 => [60.0, 20.0, 80.0, 120.0, 200.0]),
    )?;

    let s0_source = persist_csv(&source, &fixture_root, "00-source")?;
    println!("source shape: {:?}", source.shape());
    println!("persisted: {}", fixture_path(&s0_source));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Unpivot Macro",
        "unpivot! reshapes wide metric columns into long metric/value rows.",
    );

    let long = gds::unpivot!(
        source,
        on = [qty, revenue],
        index = [customer_id, segment],
        variable = "metric",
        value = "value"
    )?;

    println!("long rows: {}", long.height());
    println!("{}", long.fmt_table());
    let s1_long = persist_csv(&long, &fixture_root, "01-unpivot")?;
    println!("persisted: {}", fixture_path(&s1_long));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Melt Alias Macro",
        "melt! uses id/value naming as a semantic alias for unpivot.",
    );

    let melted = gds::melt!(
        source,
        id = [customer_id, segment],
        values = [qty, revenue],
        variable = "feature",
        value = "score"
    )?;

    println!("melted rows: {}", melted.height());
    println!("{}", melted.fmt_table());
    let s2_melt = persist_csv(&melted, &fixture_root, "02-melt")?;
    println!("persisted: {}", fixture_path(&s2_melt));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Grouped Long-Form Summary",
        "group_by! + agg! summarize each metric in long form.",
    );

    let summary = gds::group_by!(
        long,
        [metric],
        [
            gds::agg!(value.sum => "total_value"),
            gds::agg!(customer_id.count => "rows")
        ]
    )?;

    let summary = gds::arrange!(summary, [metric], asc)?;
    println!("summary shape: {:?}", summary.shape());
    println!("{}", summary.fmt_table());
    let s3_summary = persist_csv(&summary, &fixture_root, "03-summary")?;
    println!("persisted: {}", fixture_path(&s3_summary));
    println!();

    // ------------------------------------------------------------------ Stage 4
    stage(
        4,
        "Pivot Boundary",
        "pivot! captures current eager pivot boundary as an explicit contract.",
    );

    let eager_note = match gds::pivot!(source) {
        Ok(df) => format!("unexpected eager pivot success: shape {:?}", df.shape()),
        Err(err) => format!("expected eager pivot boundary: {err}"),
    };
    println!("{eager_note}");

    let s4_boundary = fixture_root.join("04-pivot-boundary.txt");
    fs::write(&s4_boundary, format!("{eager_note}\n"))?;
    println!("persisted: {}", fixture_path(&s4_boundary));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&s0_source, &s1_long, &s2_melt, &s3_summary, &s4_boundary),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_pivot_unpivot_operations")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_pivot_unpivot_operations/{file_name}")
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

fn manifest(
    s0_source: &Path,
    s1_long: &Path,
    s2_melt: &Path,
    s3_summary: &Path,
    s4_boundary: &Path,
) -> String {
    format!(
        "DataFrame Pivot/Unpivot Operations Fixture\n\n\
         Namespace: dataframe::reshape\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: baseline wide table for reshape operations.\n\n\
         01 Unpivot\n\
         artifact: {}\n\
         meaning: unpivot! converts qty/revenue into long metric/value rows.\n\n\
         02 Melt\n\
         artifact: {}\n\
         meaning: melt! alias path with id/value terminology.\n\n\
         03 Long Summary\n\
         artifact: {}\n\
         meaning: group_by! + agg! summarize long-form metric totals.\n\n\
         04 Pivot Boundary\n\
         artifact: {}\n\
         meaning: explicit eager pivot boundary note in wrapper surface.\n",
        fixture_path(s0_source),
        fixture_path(s1_long),
        fixture_path(s2_melt),
        fixture_path(s3_summary),
        fixture_path(s4_boundary),
    )
}
