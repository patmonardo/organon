//! Dataset Source Corpus fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_source_corpus

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Source Corpus ==");

    let fixture_root = fixture_root();
    let source_root = fixture_root.join("source");
    fs::create_dir_all(&source_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Source Material",
        "Corpus intake begins from explicit source files.",
    );
    fs::write(source_root.join("doc1.txt"), "Hello world.\nSecond line.")?;
    fs::write(
        source_root.join("doc2.txt"),
        "Dataset source identity must be stable.",
    )?;
    fs::write(source_root.join("words.txt"), "alpha\nbeta\n#comment\n")?;
    let sources_path = fixture_root.join("00-sources.txt");
    fs::write(
        &sources_path,
        "source/doc1.txt\nsource/doc2.txt\nsource/words.txt\n",
    )?;
    println!("persisted: {}", fixture_path(&sources_path));
    println!();

    stage(
        1,
        "Corpus Materialization",
        "Source files become a corpus object with stable document count.",
    );
    let corpus = scan_text_dir(&source_root)?;
    let corpus_path = fixture_root.join("01-corpus.txt");
    fs::write(
        &corpus_path,
        format!(
            "documents: {}\nlen: {}\n",
            corpus.document_count(),
            corpus.len(),
        ),
    )?;
    println!("documents: {}", corpus.document_count());
    println!("persisted: {}", fixture_path(&corpus_path));
    println!();

    stage(
        2,
        "Reader Surface",
        "Stdlib readers expose deterministic source views.",
    );
    let text_files = CorpusFiles::from_root(&source_root, Some(r".*\.txt"))?;
    let plaintext = PlaintextCorpusReader::new(text_files);
    let words = plaintext.words(None)?;

    let word_files = CorpusFiles::from_root(&source_root, Some(r"words\.txt"))?;
    let word_reader = WordListCorpusReader::new(word_files);
    let wordlist = word_reader.words(None, "#")?;

    let readers_path = fixture_root.join("02-readers.txt");
    fs::write(
        &readers_path,
        format!(
            "plaintext_word_count: {}\nwordlist_count: {}\n",
            words.len(),
            wordlist.len(),
        ),
    )?;
    println!("plaintext words: {}", words.len());
    println!("wordlist entries: {}", wordlist.len());
    println!("persisted: {}", fixture_path(&readers_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&sources_path, &corpus_path, &readers_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_source_corpus")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_source_corpus/{file_name}")
}

fn manifest(sources_path: &Path, corpus_path: &Path, readers_path: &Path) -> String {
    format!(
        "Dataset Source Corpus Fixture\n\n\
         Namespace: dataset::source\n\n\
         00 Sources\n\
         artifact: {}\n\
         meaning: concrete source files for corpus intake.\n\n\
         01 Corpus\n\
         artifact: {}\n\
         meaning: source files materialized into corpus identity.\n\n\
         02 Readers\n\
         artifact: {}\n\
         meaning: deterministic stdlib reader views over source files.\n",
        fixture_path(sources_path),
        fixture_path(corpus_path),
        fixture_path(readers_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
