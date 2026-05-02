//! Semantic Meta Pipeline walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_semantic_meta_pipeline

use gds::collections::dataset::prelude::*;
use gds::form::program::{ProgramFeature, ProgramFeatureKind};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Semantic Meta Pipeline walkthrough ==");
    println!("SemDataset is the Corpus:LM end-view where Program Features become logical forms.");

    let corpus = Corpus::from_texts(&[
        "Socrates is human.",
        "All humans are mortal.",
        "Socrates is mortal.",
    ])?;
    let mut sem = SemDataset::fit(corpus, MLE::new(2), &WhitespaceTokenizer)?;

    println!("documents: {}", sem.corpus().document_count());
    println!("lm order: {}", sem.lm().order());
    println!("vocab size: {}", sem.lm().vocab().len());

    sem.ingest_forms(vec![
        ProgramFeature::new(
            ProgramFeatureKind::Judgment,
            "all x. (human(x) -> mortal(x))".to_string(),
            "doctrine://semantic-meta-pipeline/judgment".to_string(),
        ),
        ProgramFeature::new(
            ProgramFeatureKind::Inference,
            "human(socrates) -> mortal(socrates)".to_string(),
            "doctrine://semantic-meta-pipeline/inference".to_string(),
        ),
        ProgramFeature::new(
            ProgramFeatureKind::Principle,
            "all x.".to_string(),
            "doctrine://semantic-meta-pipeline/bad-form".to_string(),
        ),
    ]);

    let parsed = sem.parse_forms();
    println!("forms: {}", sem.forms().len());
    println!("parsed: {}", parsed);

    for form in sem.forms() {
        let status = if form.parsed() { "parsed" } else { "error" };
        println!("- {} {} [{}]", form.kind.as_str(), status, form.source);
        match (&form.expr, &form.error) {
            (Some(expr), _) => println!("  expr: {}", expr),
            (_, Some(error)) => println!("  error: {}", error),
            _ => println!("  pending"),
        }
    }

    println!("Interpretation: DataFrame remains the real runtime body; SemDataset is the ideal semantic controller that can later dispatch to DirectCompute when GLM/GNN scale is required.");

    Ok(())
}
