//! Dataset Framing Curation walkthrough.
//!
//! The first Dataset moment is a Framing proxy over the DataFrame body. This
//! example curates the dataframe_select_filter fixture shape as a Dataset
//! production surface while keeping DataFrame execution directly available.
//!
//! Run with:
//!   cargo run -p gds --example dataset_framing_curation

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::DataFrameDatasetExt;
use gds::collections::dataset::DatasetArtifactKind;
use gds::collections::dataset::DATAFRAME_FRAMING_NAMESPACE;
use gds::collections::dataset::DATASET_FRAMING_NAMESPACE;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Framing Curation ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame Proxy",
        "Dataset Framing enters through a registered proxy to the DataFrame body.",
    );
    let source = source_frame()?;
    let frame = source
        .clone()
        .ds()
        .named("dataset-framing-curation")
        .artifact_kind(DatasetArtifactKind::Table)
        .facet("curated-fixture")
        .with_registered_framing()?;

    let source_path = persist_frame(frame.dataframe(), &fixture_root, "00-source")?;
    println!("dataset namespace: {DATASET_FRAMING_NAMESPACE}");
    println!("dataframe namespace: {DATAFRAME_FRAMING_NAMESPACE}");
    println!("proxy name: {:?}", frame.name());
    println!("proxy shape: {:?}", frame.dataframe().shape());
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    stage(
        1,
        "Select",
        "Curation narrows the proxy body while preserving DataFrame execution.",
    );
    let selected = gds::select_columns!(frame.dataframe().clone(), [id, score])?;
    let selected_path = persist_frame(&selected, &fixture_root, "01-selected")?;
    println!("selected columns: {:?}", selected.column_names());
    println!("persisted: {}", fixture_path(&selected_path));
    println!();

    stage(
        2,
        "Filter",
        "Curation keeps rows selected by a Dataset-facing framing judgment.",
    );
    let filtered = gds::filter!(frame.dataframe().clone(), score > 20.0)?;
    let filtered_path = persist_frame(&filtered, &fixture_root, "02-filtered")?;
    println!("filtered shape: {:?}", filtered.shape());
    println!("persisted: {}", fixture_path(&filtered_path));
    println!();

    stage(
        3,
        "Projection",
        "Projected columns become a curated Dataset product over the Frame proxy.",
    );
    let projected = gds::select!(frame.dataframe().clone(), id, (score * 2.0) as "score_x2")?;
    let projected_path = persist_frame(&projected, &fixture_root, "03-projected")?;
    println!("projected columns: {:?}", projected.column_names());
    println!("persisted: {}", fixture_path(&projected_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &source_path,
            &selected_path,
            &filtered_path,
            &projected_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn source_frame() -> Result<gds::collections::dataframe::GDSDataFrame, Box<dyn std::error::Error>> {
    Ok(gds::tbl_def!(
        (id: i64 => [1, 2, 3, 4, 5]),
        (score: f64 => [10.0, 25.0, 40.0, 15.0, 30.0]),
        (weight: f64 => [1.1, 0.8, 1.5, 1.0, 1.2]),
        (group: ["A", "B", "A", "B", "A"]),
    )?)
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_framing_curation")
}

fn persist_frame(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    file_stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{file_stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_framing_curation/{file_name}")
}

fn manifest(
    source_path: &Path,
    selected_path: &Path,
    filtered_path: &Path,
    projected_path: &Path,
) -> String {
    format!(
        "Dataset Framing Curation Fixture\n\n\
         Namespace: dataset::frame::{DATASET_FRAMING_NAMESPACE}\n\
         DataFrame proxy namespace: {DATAFRAME_FRAMING_NAMESPACE}\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: Dataset Frame proxy over the source DataFrame body.\n\n\
         01 Selected\n\
         artifact: {}\n\
         meaning: Curated column subset from the framed DataFrame body.\n\n\
         02 Filtered\n\
         artifact: {}\n\
         meaning: Curated row membership under a score threshold.\n\n\
         03 Projected\n\
         artifact: {}\n\
         meaning: Curated projection product ready for Model work.\n",
        fixture_path(source_path),
        fixture_path(selected_path),
        fixture_path(filtered_path),
        fixture_path(projected_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
