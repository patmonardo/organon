use regex::Regex;

use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::stemmer::Stemmer;
use crate::collections::dataset::token::Token;

#[derive(Debug, Clone)]
pub struct RegexpStemmer {
    regexp: Regex,
    min: usize,
}

impl RegexpStemmer {
    pub fn new(pattern: impl AsRef<str>, min: usize) -> Result<Self, regex::Error> {
        Ok(Self {
            regexp: Regex::new(pattern.as_ref())?,
            min,
        })
    }

    pub fn stem_word(&self, word: &str) -> String {
        if word.len() < self.min {
            word.to_string()
        } else {
            self.regexp.replace_all(word, "").to_string()
        }
    }
}

impl Stemmer for RegexpStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(self.stem_word(token.text()), token.span(), StemKind::Stem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regexp_stemmer_nltk_examples() {
        let st = RegexpStemmer::new(r"ing$|s$|e$|able$", 4).expect("valid regex");
        assert_eq!(st.stem_word("cars"), "car");
        assert_eq!(st.stem_word("mass"), "mas");
        assert_eq!(st.stem_word("was"), "was");
        assert_eq!(st.stem_word("bee"), "bee");
        assert_eq!(st.stem_word("compute"), "comput");
        assert_eq!(st.stem_word("advisable"), "advis");
    }
}
