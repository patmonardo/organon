use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::stemmer::{suffix_replace, Stemmer};
use crate::collections::dataset::token::Token;

use super::porter::{PorterMode, PorterStemmer};

pub const SNOWBALL_LANGUAGES: &[&str] = &[
    "arabic",
    "danish",
    "dutch",
    "english",
    "finnish",
    "french",
    "german",
    "hungarian",
    "italian",
    "norwegian",
    "porter",
    "portuguese",
    "romanian",
    "russian",
    "spanish",
    "swedish",
];

#[derive(Debug, Clone)]
pub struct SnowballStemmer {
    language: String,
    ignore_stopwords: bool,
    porter: PorterStemmer,
}

impl SnowballStemmer {
    pub fn new(language: &str, ignore_stopwords: bool) -> Result<Self, String> {
        let normalized = language.to_ascii_lowercase();
        if !SNOWBALL_LANGUAGES.contains(&normalized.as_str()) {
            return Err(format!("Unsupported snowball language: {language}"));
        }
        Ok(Self {
            language: normalized,
            ignore_stopwords,
            porter: PorterStemmer::new(PorterMode::NltkExtensions),
        })
    }

    pub fn language(&self) -> &str {
        &self.language
    }

    pub fn ignore_stopwords(&self) -> bool {
        self.ignore_stopwords
    }

    pub fn stem_word(&self, token: &str) -> String {
        match self.language.as_str() {
            "english" | "porter" => self.porter.stem_word(token),
            "german" => stem_german_heuristic(token),
            _ => token.to_ascii_lowercase(),
        }
    }
}

impl Stemmer for SnowballStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(
            self.stem_word(token.text()),
            token.span(),
            StemKind::Stem,
        )
    }
}

fn stem_german_heuristic(word: &str) -> String {
    let mut out = word.to_ascii_lowercase();
    out = out
        .replace('ä', "a")
        .replace('ö', "o")
        .replace('ü', "u")
        .replace('ß', "ss");

    for suffix in ["ern", "em", "er", "en", "es", "e", "n", "s"] {
        if out.len() > suffix.len() + 2 && out.ends_with(suffix) {
            out = suffix_replace(&out, suffix, "");
            break;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snowball_language_list_contains_expected_entries() {
        assert!(SNOWBALL_LANGUAGES.contains(&"english"));
        assert!(SNOWBALL_LANGUAGES.contains(&"german"));
        assert!(SNOWBALL_LANGUAGES.contains(&"porter"));
    }

    #[test]
    fn snowball_english_uses_porter() {
        let stemmer = SnowballStemmer::new("english", false).expect("valid language");
        assert_eq!(stemmer.stem_word("running"), "run");
    }

    #[test]
    fn snowball_german_example() {
        let stemmer = SnowballStemmer::new("german", false).expect("valid language");
        assert_eq!(stemmer.stem_word("Autobahnen"), "autobahn");
    }
}
