use gds::collections::dataset::prelude::*;

fn main() {
    let text = r#"{"a":1,"b":["x",true]}"#;
    let tokenizer = JsonTokenizer::default();
    let tokens = tokenizer.tokenize(text);
    let parser = JsonParser::default();
    let forest = parser.parse_tokens(&tokens);

    if let Some(parse) = forest.first() {
        println!("{}", parse.tree().format_bracketed());
    }
}
