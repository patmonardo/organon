use std::collections::HashSet;

use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::stemmer::Stemmer;
use crate::collections::dataset::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WordNetPos {
    Noun,
    Verb,
    Adjective,
    Adverb,
    SatelliteAdjective,
}

impl WordNetPos {
    pub fn from_char(pos: char) -> Option<Self> {
        match pos {
            'n' => Some(Self::Noun),
            'v' => Some(Self::Verb),
            'a' => Some(Self::Adjective),
            'r' => Some(Self::Adverb),
            's' => Some(Self::SatelliteAdjective),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct WordNetLemmatizer;

impl WordNetLemmatizer {
    pub fn new() -> Self {
        Self
    }

    pub fn morphy(
        &self,
        form: &str,
        pos: Option<WordNetPos>,
        check_exceptions: bool,
    ) -> Option<String> {
        if let Some(pos) = pos {
            return self._morphy(form, pos, check_exceptions).into_iter().next();
        }

        [
            WordNetPos::Noun,
            WordNetPos::Verb,
            WordNetPos::Adjective,
            WordNetPos::Adverb,
        ]
        .into_iter()
        .find_map(|candidate_pos| {
            self._morphy(form, candidate_pos, check_exceptions)
                .into_iter()
                .next()
        })
    }

    pub fn lemmatize(&self, word: &str, pos: WordNetPos) -> String {
        let lemmas = self._morphy(word, pos, true);
        if lemmas.is_empty() {
            word.to_string()
        } else {
            lemmas
                .into_iter()
                .min_by_key(|lemma| lemma.len())
                .unwrap_or_else(|| word.to_string())
        }
    }

    pub fn _morphy(&self, form: &str, pos: WordNetPos, check_exceptions: bool) -> Vec<String> {
        let mut out = Vec::new();
        let mut seen = HashSet::new();

        if check_exceptions {
            for exception in exceptions(form, pos) {
                if seen.insert(exception.to_string()) {
                    out.push(exception.to_string());
                }
            }
        }

        for lemma in apply_substitutions(form, pos) {
            if seen.insert(lemma.clone()) {
                out.push(lemma);
            }
        }

        out
    }
}

impl Stemmer for WordNetLemmatizer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(
            self.lemmatize(token.text(), WordNetPos::Noun),
            token.span(),
            StemKind::Lemma,
        )
    }
}

fn exceptions(form: &str, pos: WordNetPos) -> &'static [&'static str] {
    match pos {
        WordNetPos::Noun => match form {
            "us" => &["us", "u"],
            "children" => &["child"],
            "men" => &["man"],
            "women" => &["woman"],
            "teeth" => &["tooth"],
            "geese" => &["goose"],
            "mice" => &["mouse"],
            "abaci" => &["abacus"],
            _ => &[],
        },
        WordNetPos::Verb => match form {
            "was" | "were" | "am" | "is" | "are" => &["be"],
            "has" | "had" => &["have"],
            "did" => &["do"],
            _ => &[],
        },
        WordNetPos::Adjective | WordNetPos::SatelliteAdjective => match form {
            "better" => &["good"],
            "best" => &["good"],
            "worse" => &["bad"],
            "worst" => &["bad"],
            _ => &[],
        },
        WordNetPos::Adverb => &[],
    }
}

fn apply_substitutions(form: &str, pos: WordNetPos) -> Vec<String> {
    let rules: &[(&str, &str)] = match pos {
        WordNetPos::Noun => &[
            ("s", ""),
            ("ses", "s"),
            ("ves", "f"),
            ("xes", "x"),
            ("zes", "z"),
            ("ches", "ch"),
            ("shes", "sh"),
            ("men", "man"),
            ("ies", "y"),
        ],
        WordNetPos::Verb => &[
            ("s", ""),
            ("ies", "y"),
            ("es", "e"),
            ("es", ""),
            ("ed", "e"),
            ("ed", ""),
            ("ing", "e"),
            ("ing", ""),
        ],
        WordNetPos::Adjective | WordNetPos::SatelliteAdjective => {
            &[("er", ""), ("est", ""), ("er", "e"), ("est", "e")]
        }
        WordNetPos::Adverb => &[],
    };

    let mut out = Vec::new();
    for (old_suffix, new_suffix) in rules {
        if form.ends_with("ss") && *old_suffix == "s" {
            continue;
        }
        if form.len() <= old_suffix.len() {
            continue;
        }
        if let Some(stem) = form.strip_suffix(old_suffix) {
            out.push(format!("{stem}{new_suffix}"));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{Token, TokenKind, TokenSpan};

    #[test]
    fn wordnet_lemmatize_noun_examples() {
        let wnl = WordNetLemmatizer::new();
        assert_eq!(wnl.lemmatize("dogs", WordNetPos::Noun), "dog");
        assert_eq!(wnl.lemmatize("churches", WordNetPos::Noun), "church");
        assert_eq!(wnl.lemmatize("aardwolves", WordNetPos::Noun), "aardwolf");
        assert_eq!(wnl.lemmatize("abaci", WordNetPos::Noun), "abacus");
        assert_eq!(wnl.lemmatize("hardrock", WordNetPos::Noun), "hardrock");
    }

    #[test]
    fn wordnet_morphy_examples() {
        let wnl = WordNetLemmatizer::new();
        assert_eq!(
            wnl.morphy("us", Some(WordNetPos::Noun), true),
            Some("us".to_string())
        );
        assert_eq!(wnl.morphy("catss", None, true), None);
    }

    #[test]
    fn wordnet_private_morphy_includes_shortest_candidate() {
        let wnl = WordNetLemmatizer::new();
        let lemmas = wnl._morphy("us", WordNetPos::Noun, true);
        assert_eq!(lemmas, vec!["us".to_string(), "u".to_string()]);
    }

    #[test]
    fn wordnet_implements_dataset_stemmer_trait() {
        let wnl = WordNetLemmatizer::new();
        let token = Token::new("dogs", TokenSpan::new(0, 4), TokenKind::Word);
        let stem = wnl.stem_token(&token);
        assert_eq!(stem.text(), "dog");
        assert_eq!(stem.kind(), &StemKind::Lemma);
    }
}
