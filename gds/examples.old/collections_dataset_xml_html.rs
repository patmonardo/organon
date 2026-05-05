//! Dataset XML/HTML parsing walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example collections_dataset_xml_html

use gds::collections::dataset::prelude::*;

fn main() {
    println!("== Dataset XML/HTML walkthrough ==");
    println!("Markup can also be lowered into semantic Dataset form.");
    let text = "<doc><p id=\"a\">Hello</p><br/></doc>";
    let tokenizer = MarkupTokenizer::default();
    let tokens = tokenizer.tokenize(text);
    let parser = MarkupParser::default();
    let forest = parser.parse_tokens(&tokens);

    if let Some(parse) = forest.first() {
        println!("parsed semantic form: {}", parse.tree().format_bracketed());
    }
}
