use std::fs;
use std::path::PathBuf;

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = demo_root();
    fs::create_dir_all(&root)?;

    fs::write(root.join("doc1.txt"), "Hello world.\nSecond line.")?;
    fs::write(root.join("words.txt"), "alpha\nbeta\n#comment\n")?;
    fs::write(
        root.join("trees.txt"),
        "(S (NP Mary) (VP walks))\n(S (VP runs))",
    )?;
    fs::write(root.join("doc.xml"), "<doc><p>Hello <b>world</b></p></doc>")?;

    let text_files = CorpusFiles::from_root(&root, Some(r".*\.txt"))?;
    let text_reader = PlaintextCorpusReader::new(text_files);
    let words = text_reader.words(None)?;
    println!("words: {}", words.len());

    let word_files = CorpusFiles::from_root(&root, Some(r"words\.txt"))?;
    let word_reader = WordListCorpusReader::new(word_files);
    let wordlist = word_reader.words(None, "#")?;
    println!("wordlist: {}", wordlist.len());

    let tree_files = CorpusFiles::from_root(&root, Some(r"trees\.txt"))?;
    let tree_reader = BracketedCorpusReader::new(tree_files)?;
    let parses = tree_reader.parses(None)?;
    println!("parses: {}", parses.len());

    let xml_files = CorpusFiles::from_root(&root, Some(r"doc\.xml"))?;
    let xml_reader = XmlCorpusReader::new(xml_files);
    let xml_words = xml_reader.words_all(None)?;
    println!("xml_words: {}", xml_words.len());

    Ok(())
}

fn demo_root() -> PathBuf {
    std::env::temp_dir().join("gds_corpus_demo")
}
