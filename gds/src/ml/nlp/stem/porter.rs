use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::stemmer::{suffix_replace, Stemmer};
use crate::collections::dataset::token::Token;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PorterMode {
    NltkExtensions,
    MartinExtensions,
    OriginalAlgorithm,
}

impl Default for PorterMode {
    fn default() -> Self {
        Self::NltkExtensions
    }
}

#[derive(Debug, Clone, Default)]
pub struct PorterStemmer {
    mode: PorterMode,
}

impl PorterStemmer {
    pub fn new(mode: PorterMode) -> Self {
        Self { mode }
    }

    pub fn stem_word(&self, word: &str) -> String {
        let mut stem = word.to_ascii_lowercase();
        if self.mode != PorterMode::OriginalAlgorithm && stem.len() <= 2 {
            return stem;
        }

        stem = self.step1a(&stem);
        stem = self.step1b(&stem);
        stem = self.step1c(&stem);
        stem = self.step2(&stem);
        stem = self.step3(&stem);
        stem = self.step4(&stem);
        stem = self.step5a(&stem);
        stem = self.step5b(&stem);
        stem
    }

    fn is_consonant(&self, word: &[char], idx: usize) -> bool {
        match word[idx] {
            'a' | 'e' | 'i' | 'o' | 'u' => false,
            'y' => {
                if idx == 0 {
                    true
                } else {
                    !self.is_consonant(word, idx - 1)
                }
            }
            _ => true,
        }
    }

    fn measure(&self, stem: &str) -> usize {
        let chars: Vec<char> = stem.chars().collect();
        if chars.is_empty() {
            return 0;
        }
        let mut seq = String::new();
        for i in 0..chars.len() {
            seq.push(if self.is_consonant(&chars, i) {
                'c'
            } else {
                'v'
            });
        }
        seq.matches("vc").count()
    }

    fn contains_vowel(&self, stem: &str) -> bool {
        let chars: Vec<char> = stem.chars().collect();
        (0..chars.len()).any(|i| !self.is_consonant(&chars, i))
    }

    fn ends_double_consonant(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() < 2 {
            return false;
        }
        let n = chars.len();
        chars[n - 1] == chars[n - 2] && self.is_consonant(&chars, n - 1)
    }

    fn ends_cvc(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() < 3 {
            return false;
        }
        let n = chars.len();
        self.is_consonant(&chars, n - 3)
            && !self.is_consonant(&chars, n - 2)
            && self.is_consonant(&chars, n - 1)
            && !matches!(chars[n - 1], 'w' | 'x' | 'y')
    }

    fn has_positive_measure(&self, stem: &str) -> bool {
        self.measure(stem) > 0
    }

    fn step1a(&self, word: &str) -> String {
        if self.mode == PorterMode::NltkExtensions && word.ends_with("ies") && word.len() == 4 {
            return suffix_replace(word, "ies", "ie");
        }
        if word.ends_with("sses") {
            return suffix_replace(word, "sses", "ss");
        }
        if word.ends_with("ies") {
            return suffix_replace(word, "ies", "i");
        }
        if word.ends_with("ss") {
            return word.to_string();
        }
        if word.ends_with('s') {
            return suffix_replace(word, "s", "");
        }
        word.to_string()
    }

    fn step1b(&self, word: &str) -> String {
        if word.ends_with("eed") {
            let stem = suffix_replace(word, "eed", "");
            if self.measure(&stem) > 0 {
                return format!("{stem}ee");
            }
            return word.to_string();
        }

        let mut stem = None;
        for suffix in ["ed", "ing"] {
            if word.ends_with(suffix) {
                let candidate = suffix_replace(word, suffix, "");
                if self.contains_vowel(&candidate) {
                    stem = Some(candidate);
                    break;
                }
            }
        }

        let Some(mut intermediate) = stem else {
            return word.to_string();
        };

        if intermediate.ends_with("at") {
            intermediate.push('e');
            return intermediate;
        }
        if intermediate.ends_with("bl") {
            intermediate.push('e');
            return intermediate;
        }
        if intermediate.ends_with("iz") {
            intermediate.push('e');
            return intermediate;
        }
        if self.ends_double_consonant(&intermediate)
            && !intermediate.ends_with('l')
            && !intermediate.ends_with('s')
            && !intermediate.ends_with('z')
        {
            intermediate.pop();
            return intermediate;
        }
        if self.measure(&intermediate) == 1 && self.ends_cvc(&intermediate) {
            intermediate.push('e');
            return intermediate;
        }
        intermediate
    }

    fn step1c(&self, word: &str) -> String {
        if !word.ends_with('y') {
            return word.to_string();
        }
        let stem = suffix_replace(word, "y", "");
        let cond = if self.mode == PorterMode::NltkExtensions {
            stem.len() > 1 && {
                let chars: Vec<char> = stem.chars().collect();
                self.is_consonant(&chars, chars.len() - 1)
            }
        } else {
            self.contains_vowel(&stem)
        };
        if cond {
            format!("{stem}i")
        } else {
            word.to_string()
        }
    }

    fn step2(&self, word: &str) -> String {
        let rules = [
            ("ational", "ate"),
            ("tional", "tion"),
            ("enci", "ence"),
            ("anci", "ance"),
            ("izer", "ize"),
            ("bli", "ble"),
            ("alli", "al"),
            ("entli", "ent"),
            ("eli", "e"),
            ("ousli", "ous"),
            ("ization", "ize"),
            ("ation", "ate"),
            ("ator", "ate"),
            ("alism", "al"),
            ("iveness", "ive"),
            ("fulness", "ful"),
            ("ousness", "ous"),
            ("aliti", "al"),
            ("iviti", "ive"),
            ("biliti", "ble"),
            ("logi", "log"),
        ];
        for (suffix, replacement) in rules {
            if word.ends_with(suffix) {
                let stem = suffix_replace(word, suffix, "");
                if self.has_positive_measure(&stem) {
                    return format!("{stem}{replacement}");
                }
                return word.to_string();
            }
        }
        word.to_string()
    }

    fn step3(&self, word: &str) -> String {
        let rules = [
            ("icate", "ic"),
            ("ative", ""),
            ("alize", "al"),
            ("iciti", "ic"),
            ("ical", "ic"),
            ("ful", ""),
            ("ness", ""),
        ];
        for (suffix, replacement) in rules {
            if word.ends_with(suffix) {
                let stem = suffix_replace(word, suffix, "");
                if self.has_positive_measure(&stem) {
                    return format!("{stem}{replacement}");
                }
                return word.to_string();
            }
        }
        word.to_string()
    }

    fn step4(&self, word: &str) -> String {
        let suffixes = [
            "al", "ance", "ence", "er", "ic", "able", "ible", "ant", "ement", "ment", "ent", "ou",
            "ism", "ate", "iti", "ous", "ive", "ize",
        ];
        for suffix in suffixes {
            if word.ends_with(suffix) {
                let stem = suffix_replace(word, suffix, "");
                if self.measure(&stem) > 1 {
                    return stem;
                }
                return word.to_string();
            }
        }

        if word.ends_with("ion") {
            let stem = suffix_replace(word, "ion", "");
            if self.measure(&stem) > 1 && (stem.ends_with('s') || stem.ends_with('t')) {
                return stem;
            }
        }
        word.to_string()
    }

    fn step5a(&self, word: &str) -> String {
        if word.ends_with('e') {
            let stem = suffix_replace(word, "e", "");
            let m = self.measure(&stem);
            if m > 1 || (m == 1 && !self.ends_cvc(&stem)) {
                return stem;
            }
        }
        word.to_string()
    }

    fn step5b(&self, word: &str) -> String {
        if word.ends_with("ll") && self.measure(suffix_replace(word, "l", "").as_str()) > 1 {
            return suffix_replace(word, "l", "");
        }
        word.to_string()
    }
}

impl Stemmer for PorterStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(self.stem_word(token.text()), token.span(), StemKind::Stem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn porter_step_examples() {
        let stemmer = PorterStemmer::default();
        assert_eq!(stemmer.stem_word("caresses"), "caress");
        assert_eq!(stemmer.stem_word("ponies"), "poni");
        assert_eq!(stemmer.stem_word("ties"), "tie");
        assert_eq!(stemmer.stem_word("caress"), "caress");
        assert_eq!(stemmer.stem_word("cats"), "cat");
    }
}
