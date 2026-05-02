//! Tag DSL core types.

use polars::prelude::{lit, Expr};

use crate::collections::dataframe::record;
use crate::collections::dataset::token::TokenSpan;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag {
    text: String,
    tag: String,
    span: TokenSpan,
}

impl Tag {
    pub fn new(text: impl Into<String>, tag: impl Into<String>, span: TokenSpan) -> Self {
        Self {
            text: text.into(),
            tag: tag.into(),
            span,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn span(&self) -> TokenSpan {
        self.span
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = tag.into();
        self
    }

    pub fn with_span(mut self, span: TokenSpan) -> Self {
        self.span = span;
        self
    }

    pub fn to_struct_expr(&self) -> Expr {
        record(vec![
            lit(self.text.clone()).alias("text"),
            lit(self.tag.clone()).alias("tag"),
            lit(self.span.start() as u64).alias("start"),
            lit(self.span.end() as u64).alias("end"),
        ])
    }
}

// =============================================================================
// Tag string utilities — `word/TAG` parsing helpers (NLTK-flavored).
// =============================================================================

pub fn str2tuple(text: &str, sep: &str) -> (String, Option<String>) {
    if let Some(index) = text.rfind(sep) {
        let word = text[..index].to_string();
        let tag = text[index + sep.len()..].to_uppercase();
        (word, Some(tag))
    } else {
        (text.to_string(), None)
    }
}

pub fn tuple2str(tagged_token: (&str, Option<&str>), sep: &str) -> String {
    match tagged_token.1 {
        Some(tag) => {
            assert!(!tag.contains(sep), "tag may not contain sep");
            format!("{}{sep}{tag}", tagged_token.0)
        }
        None => tagged_token.0.to_string(),
    }
}

pub fn untag(tagged_sentence: &[(String, String)]) -> Vec<String> {
    tagged_sentence
        .iter()
        .map(|(word, _)| word.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str2tuple_splits_at_rightmost_separator() {
        assert_eq!(
            str2tuple("fly/NN", "/"),
            ("fly".to_string(), Some("NN".to_string()))
        );
        assert_eq!(
            str2tuple("a/b/nn", "/"),
            ("a/b".to_string(), Some("NN".to_string()))
        );
        assert_eq!(str2tuple("fly", "/"), ("fly".to_string(), None));
    }

    #[test]
    fn tuple2str_joins_word_and_tag() {
        assert_eq!(tuple2str(("fly", Some("NN")), "/"), "fly/NN");
        assert_eq!(tuple2str(("fly", None), "/"), "fly");
    }

    #[test]
    fn untag_extracts_words() {
        let sentence = vec![
            ("John".to_string(), "NNP".to_string()),
            ("walks".to_string(), "VBZ".to_string()),
        ];
        assert_eq!(
            untag(&sentence),
            vec!["John".to_string(), "walks".to_string()]
        );
    }
}
