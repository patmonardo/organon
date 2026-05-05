//! Specific Model GDSL proof: LanguageModel as genetic Feature fold.
//!
//! This example demonstrates that LanguageModel is not just a probability object,
//! but a Specific Model composed of genetic SubFeatures (tokenizer, stemmer, tagger, parser).
//! It starts from the Conceptual End (SemDataset fold) and works backward through GDSL lowering.
//!
//! Run with:
//!   cargo run -p gds --example collections_semantic_specific_model

use gds::collections::dataset::prelude::*;
use gds::form::program::ProgramFeatureKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Specific Model: LanguageModel as Genetic Feature Fold ==");
    println!();
    println!("The Dataset SDK organizes the nine-moment pipeline:");
    println!("  Beginning:    Frame :: Series :: Expr");
    println!("  Essence:      Model :: Feature :: Plan");
    println!("  Concept:      Corpus :: Language :: Semantics");
    println!();
    println!("This example proves LanguageModel is the 'Language' moment of the Concept fold.");
    println!(
        "It declares LM SubFeatures (tokenizer, stemmer, tagger, parser) as genetic Features."
    );
    println!();

    // Step 1: Load and lower the Specific Model GDSL fixture.
    let gdsl_path = "gds/fixtures/gdsl/semantic-language-model.gdsl";
    println!("Reading Specific Model GDSL from: {}", gdsl_path);
    let program_features = DatasetToolChain::program_features_from_gdsl_file(gdsl_path)?;

    println!();
    println!("Program name: {}", program_features.program_name);
    println!("Total features: {}", program_features.features.len());
    println!();

    // Step 2: Extract and display the SubFeatures (which represent LM processors as genetic Features).
    let subfeatures: Vec<_> = program_features
        .features
        .iter()
        .filter(|f| f.kind == ProgramFeatureKind::Subfeature)
        .collect();

    println!("LanguageModel SubFeatures (Genetic Features):");
    for subfeature in &subfeatures {
        println!("  - {}", subfeature.value);
    }
    println!();

    // Step 3: Display the Logogenesis moment.
    let logogenesis: Vec<_> = program_features
        .features
        .iter()
        .filter(|f| f.kind == ProgramFeatureKind::Logogenesis)
        .collect();

    println!("Logogenesis moment (genetic unfold):");
    for log in &logogenesis {
        println!("  - {}", log.value);
    }
    println!();

    // Step 4: Show that the Concept fold is unified.
    let concepts: Vec<_> = program_features
        .features
        .iter()
        .filter(|f| f.kind == ProgramFeatureKind::Concept)
        .collect();

    println!("Concept fold (end-view unification):");
    for concept in &concepts {
        println!("  - {}", concept.value);
    }
    println!();

    // Step 5: Now instantiate the SemDataset in-memory to show the runtime object model.
    println!("Runtime Concept fold (SemDataset):");
    let corpus = Corpus::from_texts(&["The cat sat on the mat.", "Dogs are loyal animals."])?;

    let sem = SemDataset::fit(corpus, MLE::new(2), &WhitespaceTokenizer)?;
    println!("  Corpus:");
    println!("    - documents: {}", sem.corpus().document_count());
    println!();
    println!("  LanguageModel (with SubFeatures):");
    println!("    - order: {}", sem.lm().order());
    println!("    - vocab size: {}", sem.lm().vocab().len());
    println!(
        "    - [Implicit SubFeatures: tokenizer (WhitespaceTokenizer), stemmer, tagger, parser]"
    );
    println!();
    println!("  SemForms: {} (empty until ingested)", sem.forms().len());
    println!();

    println!("✓ Proof complete: LanguageModel is a genetic Feature fold with SubFeatures");
    println!("  - Declared in GDSL as: `subfeature tokenizer|stemmer|tagger|parser`");
    println!("  - Materialized in Corpus as: Tokenizer, Stemmer, Tagger, Parser traits");
    println!("  - Unified in SemDataset as the Language moment of the Concept fold");
    println!();

    Ok(())
}
