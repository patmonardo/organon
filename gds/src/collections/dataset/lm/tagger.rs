//! Tagger traits and default implementations.

use std::collections::HashMap;

use regex::Regex;

use crate::collections::dataset::tag::Tag;
use crate::collections::dataset::token::{Token, TokenKind};

/// Pluggable tagger trait.
pub trait Tagger {
    /// Tag a slice of tokens.
    fn tag_tokens(&self, tokens: &[Token]) -> Vec<Tag>;

    /// Tag a batch of tokenized sentences.
    fn tag_sents(&self, sentences: &[Vec<Token>]) -> Vec<Vec<Tag>> {
        sentences.iter().map(|sent| self.tag_tokens(sent)).collect()
    }

    /// Compute token-level exact-tag accuracy against gold tagged sentences.
    fn accuracy(&self, gold: &[Vec<Tag>]) -> f64 {
        let mut correct = 0usize;
        let mut total = 0usize;

        for gold_sent in gold {
            let tokens = gold_sent
                .iter()
                .map(|tag| Token::new(tag.text(), tag.span(), TokenKind::Word))
                .collect::<Vec<Token>>();
            let predicted = self.tag_tokens(&tokens);
            for (g, p) in gold_sent.iter().zip(predicted.iter()) {
                if g.tag() == p.tag() {
                    correct += 1;
                }
                total += 1;
            }
        }

        if total == 0 {
            0.0
        } else {
            correct as f64 / total as f64
        }
    }
}

/// Default tagger: assigns a constant tag to all tokens.
#[derive(Debug, Clone)]
pub struct DefaultTagger {
    tag: String,
}

impl DefaultTagger {
    pub fn new(tag: impl Into<String>) -> Self {
        Self { tag: tag.into() }
    }
}

impl Default for DefaultTagger {
    fn default() -> Self {
        Self::new("UNK")
    }
}

impl Tagger for DefaultTagger {
    fn tag_tokens(&self, tokens: &[Token]) -> Vec<Tag> {
        tokens
            .iter()
            .map(|t| Tag::new(t.text().to_string(), self.tag.clone(), t.span()))
            .collect()
    }
}

/// Lookup-based tagger for exact token matches.
#[derive(Debug, Clone)]
pub struct LookupTagger {
    map: HashMap<String, String>,
    default: String,
}

impl LookupTagger {
    pub fn new(map: HashMap<String, String>) -> Self {
        Self {
            map,
            default: "UNK".to_string(),
        }
    }

    pub fn with_default(mut self, tag: impl Into<String>) -> Self {
        self.default = tag.into();
        self
    }

    pub fn insert(&mut self, token: impl Into<String>, tag: impl Into<String>) {
        self.map.insert(token.into(), tag.into());
    }
}

impl Tagger for LookupTagger {
    fn tag_tokens(&self, tokens: &[Token]) -> Vec<Tag> {
        tokens
            .iter()
            .map(|t| {
                let tag = self
                    .map
                    .get(t.text())
                    .cloned()
                    .unwrap_or_else(|| self.default.clone());
                Tag::new(t.text().to_string(), tag, t.span())
            })
            .collect()
    }
}

/// Regex-driven tagger: first match wins.
#[derive(Debug, Clone)]
pub struct RegexTagger {
    rules: Vec<(Regex, String)>,
    default: String,
}

impl RegexTagger {
    pub fn new(rules: Vec<(Regex, String)>) -> Self {
        Self {
            rules,
            default: "UNK".to_string(),
        }
    }

    pub fn with_default(mut self, tag: impl Into<String>) -> Self {
        self.default = tag.into();
        self
    }

    pub fn push_rule(&mut self, regex: Regex, tag: impl Into<String>) {
        self.rules.push((regex, tag.into()));
    }
}

impl Tagger for RegexTagger {
    fn tag_tokens(&self, tokens: &[Token]) -> Vec<Tag> {
        tokens
            .iter()
            .map(|t| {
                let mut found = None;
                for (regex, tag) in &self.rules {
                    if regex.is_match(t.text()) {
                        found = Some(tag.clone());
                        break;
                    }
                }
                let tag = found.unwrap_or_else(|| self.default.clone());
                Tag::new(t.text().to_string(), tag, t.span())
            })
            .collect()
    }
}

/// Unigram tagger with optional backoff.
pub struct UnigramTagger {
    lexicon: HashMap<String, String>,
    default: String,
    backoff: Option<Box<dyn Tagger>>,
}

impl UnigramTagger {
    pub fn new(lexicon: HashMap<String, String>) -> Self {
        Self {
            lexicon,
            default: "UNK".to_string(),
            backoff: None,
        }
    }

    pub fn with_default(mut self, tag: impl Into<String>) -> Self {
        self.default = tag.into();
        self
    }

    pub fn with_backoff(mut self, backoff: Box<dyn Tagger>) -> Self {
        self.backoff = Some(backoff);
        self
    }
}

impl Tagger for UnigramTagger {
    fn tag_tokens(&self, tokens: &[Token]) -> Vec<Tag> {
        let mut tags = if let Some(backoff) = &self.backoff {
            backoff.tag_tokens(tokens)
        } else {
            tokens
                .iter()
                .map(|t| Tag::new(t.text().to_string(), self.default.clone(), t.span()))
                .collect()
        };

        for (idx, token) in tokens.iter().enumerate() {
            if let Some(tag) = self.lexicon.get(token.text()) {
                tags[idx] = Tag::new(token.text().to_string(), tag.clone(), token.span());
            }
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{TokenKind, TokenSpan};

    #[test]
    fn default_tagger_assigns_constant_tag() {
        let tokens = vec![Token::new("Hi", TokenSpan::new(0, 2), TokenKind::Word)];
        let tags = DefaultTagger::default().tag_tokens(&tokens);
        assert_eq!(tags[0].tag(), "UNK");
    }

    #[test]
    fn lookup_tagger_matches_token() {
        let mut map = HashMap::new();
        map.insert("Hi".to_string(), "GREET".to_string());
        let tagger = LookupTagger::new(map).with_default("OTHER");
        let tokens = vec![Token::new("Hi", TokenSpan::new(0, 2), TokenKind::Word)];
        let tags = tagger.tag_tokens(&tokens);
        assert_eq!(tags[0].tag(), "GREET");
    }

    #[test]
    fn regex_tagger_applies_first_match() {
        let rule = (Regex::new(r"^\d+$").unwrap(), "NUM".to_string());
        let tagger = RegexTagger::new(vec![rule]).with_default("WORD");
        let tokens = vec![Token::new("123", TokenSpan::new(0, 3), TokenKind::Word)];
        let tags = tagger.tag_tokens(&tokens);
        assert_eq!(tags[0].tag(), "NUM");
    }

    #[test]
    fn unigram_tagger_uses_lexicon() {
        let mut lexicon = HashMap::new();
        lexicon.insert("Hello".to_string(), "GREET".to_string());
        let tagger = UnigramTagger::new(lexicon).with_default("OTHER");
        let tokens = vec![Token::new("Hello", TokenSpan::new(0, 5), TokenKind::Word)];
        let tags = tagger.tag_tokens(&tokens);
        assert_eq!(tags[0].tag(), "GREET");
    }

    #[test]
    fn tag_sents_batches_sentences() {
        let tagger = DefaultTagger::new("X");
        let sents = vec![
            vec![Token::new("a", TokenSpan::new(0, 1), TokenKind::Word)],
            vec![Token::new("b", TokenSpan::new(2, 3), TokenKind::Word)],
        ];
        let out = tagger.tag_sents(&sents);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0][0].tag(), "X");
        assert_eq!(out[1][0].tag(), "X");
    }

    #[test]
    fn accuracy_scores_exact_tags() {
        let tagger = DefaultTagger::new("NN");
        let gold = vec![vec![
            Tag::new("John", "NN", TokenSpan::new(0, 4)),
            Tag::new("walks", "VB", TokenSpan::new(5, 10)),
        ]];
        let score = tagger.accuracy(&gold);
        assert!((score - 0.5).abs() < f64::EPSILON);
    }
}
