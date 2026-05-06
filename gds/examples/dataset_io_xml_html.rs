//! Dataset IO XML/HTML fixture.
//!
//! Exercises markup Observation flow: MarkupTokenizer + MarkupParser over XML/HTML
//! sources, plus fixture persistence of tokens and parsed tree forms.
//!
//! Run with:
//!   cargo run -p gds --example dataset_io_xml_html

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset IO XML/HTML ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Markup Sources",
        "Define compact XML and HTML sources for Observation parsing.",
    );

    let xml = r#"<doc id=\"1\"><title>Logic</title><p>Form becomes concept.</p></doc>"#;
    let html = r#"<html><body><h1>Organon</h1><p>Dataset semantic pipeline.</p></body></html>"#;

    let xml_src = fixture_root.join("00-source.xml");
    let html_src = fixture_root.join("00-source.html");
    fs::write(&xml_src, xml)?;
    fs::write(&html_src, html)?;

    println!("persisted: {}", fixture_path(&xml_src));
    println!("persisted: {}", fixture_path(&html_src));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Tokenization",
        "MarkupTokenizer emits deterministic token streams from both sources.",
    );

    let tokenizer = MarkupTokenizer::new();
    let xml_tokens = tokenizer.tokenize(xml);
    let html_tokens = tokenizer.tokenize(html);

    println!("xml token count : {}", xml_tokens.len());
    println!("html token count: {}", html_tokens.len());

    let token_path = fixture_root.join("01-tokens.txt");
    fs::write(
        &token_path,
        format!(
            "xml_count={}\nhtml_count={}\nxml_tokens={:?}\nhtml_tokens={:?}\n",
            xml_tokens.len(),
            html_tokens.len(),
            xml_tokens.iter().map(|t| t.text()).collect::<Vec<_>>(),
            html_tokens.iter().map(|t| t.text()).collect::<Vec<_>>(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&token_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Parsing",
        "MarkupParser lowers token streams into parse trees (Observation forms).",
    );

    let parser = MarkupParser::default();
    let xml_forest = parser.parse_tokens(&xml_tokens);
    let html_forest = parser.parse_tokens(&html_tokens);

    if xml_forest.is_empty() || html_forest.is_empty() {
        return Err("markup parsing produced an empty parse forest".into());
    }

    let xml_tree = xml_forest[0].tree();
    let html_tree = html_forest[0].tree();

    let xml_bracketed = xml_tree.format_bracketed();
    let html_bracketed = html_tree.format_bracketed();

    println!("xml tree height : {}", xml_tree.height());
    println!("html tree height: {}", html_tree.height());

    let parse_path = fixture_root.join("02-parse.txt");
    fs::write(
        &parse_path,
        format!(
            "xml_bracketed={}\nhtml_bracketed={}\nxml_height={}\nhtml_height={}\n",
            xml_bracketed,
            html_bracketed,
            xml_tree.height(),
            html_tree.height(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&parse_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&xml_src, &html_src, &token_path, &parse_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_io_xml_html")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_io_xml_html/{file_name}")
}

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("── Stage {n}: {name} ──────────────────────────────────────────");
    println!("   {doctrine}");
    println!();
}

fn manifest(xml: &Path, html: &Path, token: &Path, parse: &Path) -> String {
    format!(
        "Dataset IO XML/HTML Fixture\n\n\
         Namespace: dataset::io (markup observation path)\n\n\
         00 Sources\n\
         artifacts: {}, {}\n\
         meaning: compact XML/HTML inputs for deterministic observation.\n\n\
         01 Tokens\n\
         artifact: {}\n\
         meaning: MarkupTokenizer stream over XML/HTML sources.\n\n\
         02 Parse\n\
         artifact: {}\n\
         meaning: MarkupParser forest lowered into bracketed tree forms.\n",
        fixture_path(xml),
        fixture_path(html),
        fixture_path(token),
        fixture_path(parse),
    )
}
