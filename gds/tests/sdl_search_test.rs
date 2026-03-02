use gds::collections::dataframe::table::TableBuilder;
use gds::collections::dataset::search::SdlSearchQuery;
use gds::collections::dataset::semantic::{LanguageModelFocus, SdlSubgraph};
use gds::collections::dataset::Dataset;

#[test]
fn test_universal_graph_search() {
    // 1. Build a dummy Arrow-backed dataset that lacks full nested lists
    // but carries the required doc_id and text to test the extraction API loop
    let df = TableBuilder::new()
        .with_u64_column("doc_id", &[101, 102])
        .with_string_column(
            "text",
            &[
                "The quick brown fox jumps over the lazy dog.".to_string(),
                "The dog jumped.".to_string(),
            ],
        )
        .build()
        .expect("Failed to build dataframe");

    let mut dataset = Dataset::new(df);

    // 2. Wrap it with an SDL focus
    dataset = dataset.with_lm_focus(LanguageModelFocus::new().mark_sdl_compliant());

    assert!(
        dataset.is_sdl_compliant(),
        "Dataset must be marked SDL to run graph search"
    );

    // 3. For the purpose of this test, we define a structural query
    // that expects a specific FeatStruct match.
    // In actual application, try_extract_sdl_subgraph parses physical Arrow rows
    // into full semantic graph objects. Because our mock try_extract_sdl_subgraph
    // constructs empty graphs right now (the real extraction loops over ListChunked data),
    // this tests that the filter logic fundamentally compiles and executes
    // without dropping Arrow types natively.

    // We search for a single node constraint that unifies.
    // An empty graph will not have any nodes to match this constraint.
    let query = SdlSearchQuery::new()
        .with_node("[pos=\"VERB\", tense=\"PAST\"]")
        .expect("Failed to parse query constraint");

    let results = dataset.search_graph(&query).expect("Graph search panic");

    // 4. Validate Phase 2 Unification Drop-out.
    // Since our extracted subgraphs currently contain 0 nodes, they will fail
    // the semantic unification constraint and be omitted from the results.
    assert_eq!(
        results.row_count(),
        0,
        "Universal search should have excluded 0-node mock subgraphs"
    );
}
