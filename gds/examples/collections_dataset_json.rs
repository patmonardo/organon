//! Dataset JSON parsing walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_json

use gds::collections::dataset::prelude::*;

fn main() {
    println!("== Dataset JSON walkthrough ==");
    println!("Even structured JSON can enter the Dataset world as parsed semantic form, not just as opaque bytes.");
    let text = r#"{"a":1,"b":["x",true]}"#;
    let tokenizer = JsonTokenizer::default();
    let tokens = tokenizer.tokenize(text);
    let parser = JsonParser::default();
    let forest = parser.parse_tokens(&tokens);

    if let Some(parse) = forest.first() {
        println!("parsed semantic form: {}", parse.tree().format_bracketed());
    }
}
