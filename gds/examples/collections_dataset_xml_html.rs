use gds::collections::dataset::prelude::*;

fn main() {
    let text = "<doc><p id=\"a\">Hello</p><br/></doc>";
    let tokenizer = MarkupTokenizer::default();
    let tokens = tokenizer.tokenize(text);
    let parser = MarkupParser::default();
    let forest = parser.parse_tokens(&tokens);

    if let Some(parse) = forest.first() {
        println!("{}", parse.tree().format_bracketed());
    }
}
