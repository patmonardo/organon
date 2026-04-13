use gds::collections::dataframe::GDSDataFrame;
use gds::collections::dataset::artifact::DatasetArtifactKind;
use gds::collections::dataset::corpus::Corpus;
use gds::collections::dataset::{Dataset, LanguageModelFocus};
use polars::prelude::{NamedFrom, PlSmallStr, Series};

#[test]
fn test_dataset_lm_focus() {
    let s = Series::new(PlSmallStr::from_static("col1"), &[1, 2, 3]);
    let df = GDSDataFrame::from_series(vec![s]).unwrap();
    let mut dataset = Dataset::new(df);

    // By default, no LM focus
    assert!(!dataset.is_language_model());
    assert!(dataset.lm_focus().is_none());
    assert!(!dataset.has_artifact_facet("language-model-focus"));

    let focus = LanguageModelFocus::new().mark_sdl_compliant();
    dataset = dataset.with_lm_focus(focus);

    assert!(dataset.is_language_model());
    let retrieved_focus = dataset.lm_focus().unwrap();
    assert!(retrieved_focus.is_sdl_compliant);
    assert!(dataset.has_artifact_facet("language-model-focus"));
    assert!(dataset.has_artifact_facet("sdl-compliant"));
}

#[test]
fn test_corpus_language_model_focus_preserves_corpus_profile() {
    let corpus = Corpus::from_texts(&["Hello world"])
        .unwrap()
        .with_language_model_focus(LanguageModelFocus::new());

    let ds = corpus.dataset();
    assert_eq!(ds.artifact_kind(), &DatasetArtifactKind::Corpus);
    assert!(ds.is_language_model());
    assert!(!ds.is_sdl_compliant());
    assert!(ds.has_artifact_facet("language-model-focus"));
    assert!(!ds.has_artifact_facet("sdl-compliant"));
}

#[test]
fn test_dataset_extracts_minimal_sdl_subgraph() {
    let doc_ids = Series::new(PlSmallStr::from_static("doc_id"), &[7_u64]);
    let texts = Series::new(PlSmallStr::from_static("text"), &["Hello world"]);
    let df = GDSDataFrame::from_series(vec![doc_ids, texts]).unwrap();
    let dataset = Dataset::new(df).with_lm_focus(LanguageModelFocus::new().mark_sdl_compliant());

    let subgraph = dataset.try_extract_sdl_subgraph(0).unwrap();

    assert_eq!(subgraph.doc_id, 7);
    assert_eq!(subgraph.text, "Hello world");
    assert!(subgraph.nodes.is_empty());
    assert!(subgraph.edges.is_empty());
}
