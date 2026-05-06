//! Dataset Source Stdlib fixture.
//!
//! Exercises stdlib dataset-source helpers and corpus readers without
//! external network fetches.
//!
//! Run with:
//!   cargo run -p gds --example dataset_source_stdlib

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Source Stdlib ==");

    let fixture_root = fixture_root();
    let source_root = fixture_root.join("source");
    fs::create_dir_all(&source_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Local Source Material",
        "Build local corpus files consumed by stdlib readers.",
    );

    fs::write(source_root.join("doc1.txt"), "Dialectic begins in form.")?;
    fs::write(source_root.join("doc2.txt"), "Ground returns as concept.")?;
    fs::write(source_root.join("words.txt"), "alpha\nbeta\n#skip\ngamma\n")?;
    fs::write(
        source_root.join("sample.xml"),
        "<doc><title>Organon</title><p>Source stdlib witness.</p></doc>",
    )?;

    let sources_path = fixture_root.join("00-sources.txt");
    fs::write(
        &sources_path,
        "source/doc1.txt\nsource/doc2.txt\nsource/words.txt\nsource/sample.xml\n",
    )?;
    println!("persisted: {}", fixture_path(&sources_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Stdlib Resource Helpers",
        "Inspect stdlib registry + URL normalization without fetching resources.",
    );

    let resources = list_resources();
    let data_home = data_home_with(fixture_root.join("data_home"))?;

    let file_proto = split_resource_url("file:/tmp/demo.txt")?;
    let nltk_proto = split_resource_url("nltk:corpora/treebank")?;
    let normalized_file = normalize_resource_url("file:/tmp/demo.txt")?;
    let normalized_nltk = normalize_resource_url("nltk:corpora/treebank")?;

    let helper_path = fixture_root.join("01-stdlib-helpers.txt");
    fs::write(
        &helper_path,
        format!(
            "resource_count={}\nfirst_resources={:?}\ndata_home={}\nfile_proto={:?}\nnltk_proto={:?}\nnormalized_file={}\nnormalized_nltk={}\n",
            resources.len(),
            resources
                .iter()
                .take(5)
                .map(|r| r.name)
                .collect::<Vec<_>>(),
            data_home.to_string_lossy(),
            file_proto,
            nltk_proto,
            normalized_file,
            normalized_nltk,
        ),
    )?;
    println!("stdlib resources known: {}", resources.len());
    println!("persisted: {}", fixture_path(&helper_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Stdlib Readers",
        "CorpusFiles + Plaintext/WordList/Xml readers over local source files.",
    );

    let text_files = CorpusFiles::from_root(&source_root, Some(r".*\.txt"))?;
    let plaintext = PlaintextCorpusReader::new(text_files);
    let words = plaintext.words(None)?;

    let word_files = CorpusFiles::from_root(&source_root, Some(r"words\.txt"))?;
    let word_reader = WordListCorpusReader::new(word_files);
    let wordlist = word_reader.words(None, "#")?;

    let xml_files = CorpusFiles::from_root(&source_root, Some(r"sample\.xml"))?;
    let xml_reader = XmlCorpusReader::new(xml_files);
    let xml_words = xml_reader.words_all(None)?;

    let reader_path = fixture_root.join("02-readers.txt");
    fs::write(
        &reader_path,
        format!(
            "plaintext_word_count={}\nwordlist_count={}\nxml_word_count={}\nwordlist={:?}\n",
            words.len(),
            wordlist.len(),
            xml_words.len(),
            wordlist,
        ),
    )?;
    println!("plaintext words: {}", words.len());
    println!("wordlist entries: {}", wordlist.len());
    println!("xml words: {}", xml_words.len());
    println!("persisted: {}", fixture_path(&reader_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&sources_path, &helper_path, &reader_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_source_stdlib")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_source_stdlib/{file_name}")
}

fn manifest(sources_path: &Path, helper_path: &Path, reader_path: &Path) -> String {
    format!(
        "Dataset Source Stdlib Fixture\n\n\
         Namespace: dataset::source::stdlib\n\n\
         00 Sources\n\
         artifact: {}\n\
         meaning: local files used as deterministic stdlib corpus inputs.\n\n\
         01 Stdlib Helpers\n\
         artifact: {}\n\
         meaning: resource registry and URL helper normalization.\n\n\
         02 Readers\n\
         artifact: {}\n\
         meaning: Plaintext/WordList/Xml reader surfaces over source files.\n",
        fixture_path(sources_path),
        fixture_path(helper_path),
        fixture_path(reader_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
