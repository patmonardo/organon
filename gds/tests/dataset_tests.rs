use gds::collections::dataframe::GDSDataFrame;
use gds::collections::dataset::corpus::Corpus;
use gds::collections::dataset::tree::{TreeCollection, TreeValue};
use gds::collections::dataset::{Dataset, LanguageModelFocus, SemanticArtifacts, WordNetwork};
use polars::prelude::{NamedFrom, PlSmallStr, Series};

#[test]
fn test_dataset_lm_focus() {
    let s = Series::new(PlSmallStr::from_static("col1"), &[1, 2, 3]);
    let df = GDSDataFrame::from_series(vec![s]).unwrap();
    let mut dataset = Dataset::new(df);

    // By default, no LM focus
    assert!(!dataset.is_language_model());
    assert!(dataset.lm_focus().is_none());

    let mut artifacts = SemanticArtifacts::new().with_xml("<doc>test</doc>");

    let nw = WordNetwork::new("test_network");
    artifacts = artifacts.add_word_network(nw);

    let focus = LanguageModelFocus::new().with_artifacts(artifacts);
    dataset = dataset.with_lm_focus(focus);

    assert!(dataset.is_language_model());
    let retrieved_focus = dataset.lm_focus().unwrap();
    assert_eq!(
        retrieved_focus.artifacts().xml_content.as_deref(),
        Some("<doc>test</doc>")
    );
    assert!(retrieved_focus
        .artifacts()
        .word_networks
        .contains_key("test_network"));
}

#[test]
fn test_corpus_semantic_artifacts() {
    let mut corpus = Corpus::from_texts(&["Hello world"]).unwrap();

    // Attach LM focus through Corpus API
    let trees =
        TreeCollection::from_values(vec![TreeValue::leaf("Hello"), TreeValue::leaf("world")]);

    corpus = corpus.map_semantic_artifacts(|mut artifacts| {
        artifacts.xml_content = Some("<root>Hello world</root>".to_string());
        artifacts.nlp_trees = Some(trees);
        artifacts
    });

    let ds = corpus.dataset();
    assert!(ds.is_language_model());
    let artifacts = ds.lm_focus().unwrap().artifacts();
    assert_eq!(
        artifacts.xml_content.as_deref(),
        Some("<root>Hello world</root>")
    );
    assert!(artifacts.nlp_trees.is_some());
    assert_eq!(artifacts.nlp_trees.as_ref().unwrap().len(), 2);
}
