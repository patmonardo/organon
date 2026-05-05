//! Dataset IO Catalog (Extensible) fixture.
//!
//! Exercises the dataset registry surface: DatasetRegistry, DatasetMetadata,
//! DatasetArtifact, DatasetSplit, and the extensible catalog pattern.
//!
//! Run with:
//!   cargo run -p gds --example dataset_io_catalog_extensible

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset IO Catalog (Extensible) ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Registry Bootstrap",
        "DatasetRegistry holds named dataset roots and resolves artifact splits.",
    );

    let registry_root = fixture_root.join("registry");
    let mut registry = DatasetRegistry::new(&registry_root);
    registry.register_names(&["imdb", "ptb", "conll2003", "squad"]);

    let listing: Vec<String> = registry.list().iter().map(|m| m.name.clone()).collect();
    println!("registered: {:?}", listing);

    let listing_path = fixture_root.join("00-registry.txt");
    fs::write(
        &listing_path,
        format!("root: {}\nentries:\n{}\n", registry_root.display(), listing.join("\n")),
    )?;
    println!("persisted: {}", fixture_path(&listing_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Artifact Resolution",
        "dataset_artifact resolves split-aware paths from registry root.",
    );

    let train_artifact = registry.dataset_artifact("imdb", DatasetSplit::Train);
    let test_artifact = registry.dataset_artifact("imdb", DatasetSplit::Test);
    let val_artifact = registry.dataset_artifact("ptb", DatasetSplit::Validation);
    let all_artifact = registry.dataset_artifact("conll2003", DatasetSplit::All);

    println!("imdb/train path: {}", train_artifact.path.display());
    println!("imdb/test  path: {}", test_artifact.path.display());
    println!("ptb/val    path: {}", val_artifact.path.display());
    println!("conll/all  path: {}", all_artifact.path.display());

    let artifacts_path = fixture_root.join("01-artifacts.txt");
    fs::write(
        &artifacts_path,
        format!(
            "imdb/train: {}\n\
             imdb/test:  {}\n\
             ptb/val:    {}\n\
             conll/all:  {}\n",
            train_artifact.path.display(),
            test_artifact.path.display(),
            val_artifact.path.display(),
            all_artifact.path.display(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&artifacts_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Metadata Extension",
        "DatasetMetadata carries description, homepage, and tags.",
    );

    let mut registry2 = DatasetRegistry::new(&registry_root);
    let mut meta = DatasetMetadata::new("glue");
    meta.description = Some("General Language Understanding Evaluation benchmark.".to_string());
    meta.homepage = Some("https://gluebenchmark.com".to_string());
    meta.tags = vec!["nlp".to_string(), "benchmark".to_string(), "classification".to_string()];
    registry2.register(meta);

    let glue_meta = registry2.get("glue").unwrap();
    println!("name: {}", glue_meta.name);
    println!("desc: {}", glue_meta.description.as_deref().unwrap_or(""));
    println!("tags: {:?}", glue_meta.tags);

    let meta_path = fixture_root.join("02-metadata.txt");
    fs::write(
        &meta_path,
        format!(
            "name: {}\ndesc: {}\ntags: {}\nhomepage: {}\n",
            glue_meta.name,
            glue_meta.description.as_deref().unwrap_or(""),
            glue_meta.tags.join(", "),
            glue_meta.homepage.as_deref().unwrap_or(""),
        ),
    )?;
    println!("persisted: {}", fixture_path(&meta_path));

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Custom Split",
        "DatasetSplit::Custom supports project-specific partitions.",
    );

    let custom_artifact = registry.dataset_artifact(
        "squad",
        DatasetSplit::Custom("adversarial".to_string()),
    );
    println!("custom split path: {}", custom_artifact.path.display());

    let custom_path = fixture_root.join("03-custom-split.txt");
    fs::write(
        &custom_path,
        format!(
            "dataset: squad\nsplit: adversarial\npath: {}\n",
            custom_artifact.path.display(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&custom_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&listing_path, &artifacts_path, &meta_path, &custom_path),
    )?;
    println!("\nmanifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_io_catalog_extensible")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_io_catalog_extensible/{file_name}")
}

fn manifest(
    listing_path: &Path,
    artifacts_path: &Path,
    meta_path: &Path,
    custom_path: &Path,
) -> String {
    format!(
        "Dataset IO Catalog (Extensible) Fixture\n\n\
         Namespace: dataset::registry\n\n\
         00 Registry\n\
         artifact: {}\n\
         meaning: DatasetRegistry bootstrap with named entries.\n\n\
         01 Artifacts\n\
         artifact: {}\n\
         meaning: dataset_artifact resolves split-aware paths.\n\n\
         02 Metadata\n\
         artifact: {}\n\
         meaning: DatasetMetadata description, homepage, tags extension.\n\n\
         03 Custom Split\n\
         artifact: {}\n\
         meaning: DatasetSplit::Custom for project-specific partitions.\n",
        fixture_path(listing_path),
        fixture_path(artifacts_path),
        fixture_path(meta_path),
        fixture_path(custom_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
