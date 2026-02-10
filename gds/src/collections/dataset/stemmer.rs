//! Stemmer trait and default implementations.

use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::token::Token;

/// Pluggable stemmer trait.
pub trait Stemmer {
    /// Stem a single token.
    fn stem_token(&self, token: &Token) -> Stem;

    /// Stem a slice of tokens.
    fn stem_tokens(&self, tokens: &[Token]) -> Vec<Stem> {
        tokens.iter().map(|t| self.stem_token(t)).collect()
    }
}

/// Identity stemmer: returns the token text as-is.
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityStemmer;

impl Stemmer for IdentityStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(token.text().to_string(), token.span(), StemKind::Stem)
    }
}

/// Lowercase stemmer: lowercases ASCII text.
#[derive(Debug, Clone, Copy, Default)]
pub struct LowercaseStemmer;

impl Stemmer for LowercaseStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(
            token.text().to_ascii_lowercase(),
            token.span(),
            StemKind::Stem,
        )
    }
}

/// Simple English suffix stemmer with a small, conservative rule set.
#[derive(Debug, Clone, Copy)]
pub struct SimpleSuffixStemmer {
    lowercase: bool,
}

impl SimpleSuffixStemmer {
    pub fn new(lowercase: bool) -> Self {
        Self { lowercase }
    }

    fn stem_text(&self, text: &str) -> String {
        let mut base = if self.lowercase {
            text.to_ascii_lowercase()
        } else {
            text.to_string()
        };

        let suffixes = [
            "ization", "ational", "fulness", "ousness", "iveness", "ment", "ness", "able", "ible",
            "tion", "sion", "ing", "edly", "edly", "ed", "ly", "es", "s",
        ];

        for suffix in suffixes {
            if base.len() > suffix.len() + 1 && base.ends_with(suffix) {
                let new_len = base.len() - suffix.len();
                base.truncate(new_len);
                break;
            }
        }

        base
    }
}

impl Default for SimpleSuffixStemmer {
    fn default() -> Self {
        Self::new(true)
    }
}

impl Stemmer for SimpleSuffixStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(self.stem_text(token.text()), token.span(), StemKind::Stem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{TokenKind, TokenSpan};

    #[test]
    fn identity_stemmer_keeps_text() {
        let tok = Token::new("Hello", TokenSpan::new(0, 5), TokenKind::Word);
        let stem = IdentityStemmer::default().stem_token(&tok);
        assert_eq!(stem.text(), "Hello");
    }

    #[test]
    fn lowercase_stemmer_lowercases_ascii() {
        let tok = Token::new("Hello", TokenSpan::new(0, 5), TokenKind::Word);
        let stem = LowercaseStemmer::default().stem_token(&tok);
        assert_eq!(stem.text(), "hello");
    }

    #[test]
    fn simple_suffix_stemmer_strips_suffixes() {
        let tok = Token::new("running", TokenSpan::new(0, 7), TokenKind::Word);
        let stem = SimpleSuffixStemmer::default().stem_token(&tok);
        assert_eq!(stem.text(), "runn");
    }
}
