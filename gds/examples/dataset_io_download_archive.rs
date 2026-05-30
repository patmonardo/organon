//! Dataset IO Download Archive fixture.
//!
//! Practical workbench entrypoint for archive intake:
//! stage archive bytes, copy to cache, and extract into raw dataset storage.
//!
//! Run with:
//!   cargo run -p gds --example dataset_io_download_archive

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use gds::collections::dataset::DatasetCreateOptions;
use gds::collections::dataset::DatasetWorkspace;
use zip::write::FileOptions;
use zip::CompressionMethod;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset IO Download Archive ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Archive Staging",
        "Prepare a local archive without assuming semantic table structure.",
    );
    let staging_root = fixture_root.join("staging");
    fs::create_dir_all(&staging_root)?;
    let archive_path = staging_root.join("source_bundle.zip");
    write_sample_zip(&archive_path)?;
    println!("staged archive: {}", archive_path.display());
    println!();

    stage(
        1,
        "Workspace",
        "Initialize semantic dataset workspace and create the folder layout.",
    );
    let workspace_root = fixture_root.join("workspace");
    let ws = DatasetWorkspace::new(&workspace_root)?;
    let created = ws.create_dataset(
        "demo_archive_dataset",
        DatasetCreateOptions {
            principle_triad: "model-feature-plan".to_string(),
            law_view: "corpus-language-logic".to_string(),
        },
    )?;
    println!("workspace root: {}", ws.root().display());
    println!("dataset root: {}", created.layout.root.display());
    println!();

    stage(
        2,
        "Stage Archive",
        "Move archive source into workspace staging before cache/extract.",
    );
    let source = archive_path.to_string_lossy().into_owned();
    let staged = ws.stage_archive("demo_archive_dataset", &source)?;
    println!("staged archive: {}", staged.staged_path.display());
    println!("staged bytes: {}", staged.transferred.bytes);
    println!();

    stage(
        3,
        "Ingest Archive",
        "Copy staged archive into cache and unzip into raw without semantic coercion.",
    );
    let ingest = ws.ingest_archive("demo_archive_dataset", &source)?;
    println!("cached archive: {}", ingest.archive_path.display());
    println!("staging source: {}", ingest.staged_path.display());
    println!("raw destination: {}", ingest.extracted_to.display());
    println!("raw inventory: {}", ingest.raw_inventory_path.display());
    println!("files extracted: {}", ingest.extracted.files);
    if let Some(downloaded) = ingest.downloaded.as_ref() {
        println!("copied bytes: {}", downloaded.bytes);
    }
    println!();

    stage(
        4,
        "Manifest",
        "Persist fixture summary and extracted file inventory.",
    );
    let extracted_docs = ingest.extracted_to.join("docs/doc1.txt");
    let extracted_csv = ingest.extracted_to.join("tables/seeds.csv");
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Dataset IO Download Archive Fixture\n\n\
             workspace: {}\n\
             dataset: {}\n\
             archive_staged: {}\n\
             archive_workspace_staged: {}\n\
             archive_cached: {}\n\
             raw_destination: {}\n\
             raw_inventory: {}\n\
             extracted_files: {}\n\
             extracted_doc: {}\n\
             extracted_table: {}\n",
            path_string(&workspace_root),
            created.dataset,
            path_string(&archive_path),
            path_string(&staged.staged_path),
            path_string(&ingest.archive_path),
            path_string(&ingest.extracted_to),
            path_string(&ingest.raw_inventory_path),
            ingest.extracted.files,
            path_string(&extracted_docs),
            path_string(&extracted_csv),
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn write_sample_zip(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = fs::File::create(path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    zip.start_file("docs/doc1.txt", options)?;
    zip.write_all(b"archive intake should stay source-shaped\n")?;

    zip.start_file("tables/seeds.csv", options)?;
    zip.write_all(b"id,label\n1,alpha\n2,beta\n")?;

    zip.finish()?;
    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_io_download_archive")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_io_download_archive/{file_name}")
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
