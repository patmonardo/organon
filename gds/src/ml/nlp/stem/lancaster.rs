use std::collections::HashMap;

use regex::Regex;

use crate::collections::dataset::stem::{Stem, StemKind};
use crate::collections::dataset::stemmer::Stemmer;
use crate::collections::dataset::token::Token;

#[derive(Debug, Clone)]
pub struct LancasterStemmer {
    rule_dictionary: HashMap<char, Vec<String>>,
    strip_prefix: bool,
    rules: Vec<String>,
}

impl Default for LancasterStemmer {
    fn default() -> Self {
        Self::new(None, false)
    }
}

impl LancasterStemmer {
    pub fn new(rule_tuple: Option<Vec<String>>, strip_prefix_flag: bool) -> Self {
        let rules = rule_tuple.unwrap_or_else(default_rules);
        let mut out = Self {
            rule_dictionary: HashMap::new(),
            strip_prefix: strip_prefix_flag,
            rules,
        };
        out.parse_rules();
        out
    }

    fn parse_rules(&mut self) {
        let valid_rule = Regex::new(r"^[a-z]+\*?\d[a-z]*[>\.]?$").expect("valid rule regex");
        self.rule_dictionary.clear();

        for rule in &self.rules {
            if !valid_rule.is_match(rule) {
                continue;
            }
            if let Some(first) = rule.chars().next() {
                self.rule_dictionary
                    .entry(first)
                    .or_default()
                    .push(rule.clone());
            }
        }
    }

    pub fn stem_word(&self, word: &str) -> String {
        let mut word = word.to_ascii_lowercase();
        if self.strip_prefix {
            word = self.strip_prefixes(&word);
        }
        let intact_word = word.clone();
        self.do_stemming(word, &intact_word)
    }

    fn do_stemming(&self, mut word: String, intact_word: &str) -> String {
        let valid_rule =
            Regex::new(r"^([a-z]+)(\*?)(\d)([a-z]*)([>\.]?)$").expect("valid rule parser");

        let mut proceed = true;
        while proceed {
            let last_letter_position = self.get_last_letter(&word);
            if last_letter_position < 0 {
                break;
            }
            let last_char = word
                .chars()
                .nth(last_letter_position as usize)
                .unwrap_or('\0');
            let Some(rules) = self.rule_dictionary.get(&last_char) else {
                break;
            };

            let mut rule_was_applied = false;
            for rule in rules {
                let Some(caps) = valid_rule.captures(rule) else {
                    continue;
                };
                let ending_string = caps.get(1).map(|m| m.as_str()).unwrap_or_default();
                let intact_flag = caps.get(2).map(|m| m.as_str()).unwrap_or_default();
                let remove_total = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(0);
                let append_string = caps.get(4).map(|m| m.as_str()).unwrap_or_default();
                let cont_flag = caps.get(5).map(|m| m.as_str()).unwrap_or_default();

                let reversed_ending: String = ending_string.chars().rev().collect();
                if !word.ends_with(&reversed_ending) {
                    continue;
                }

                if intact_flag == "*" {
                    if word == intact_word && self.is_acceptable(&word, remove_total) {
                        word = self.apply_rule(&word, remove_total, append_string);
                        rule_was_applied = true;
                        if cont_flag == "." {
                            proceed = false;
                        }
                        break;
                    }
                } else if self.is_acceptable(&word, remove_total) {
                    word = self.apply_rule(&word, remove_total, append_string);
                    rule_was_applied = true;
                    if cont_flag == "." {
                        proceed = false;
                    }
                    break;
                }
            }

            if !rule_was_applied {
                proceed = false;
            }
        }
        word
    }

    fn get_last_letter(&self, word: &str) -> isize {
        let mut last_letter = -1;
        for (position, ch) in word.chars().enumerate() {
            if ch.is_ascii_alphabetic() {
                last_letter = position as isize;
            } else {
                break;
            }
        }
        last_letter
    }

    fn is_acceptable(&self, word: &str, remove_total: usize) -> bool {
        if word.is_empty() || remove_total > word.len() {
            return false;
        }
        let kept_len = word.len() - remove_total;
        let chars: Vec<char> = word.chars().collect();
        if chars.first().is_some_and(|ch| "aeiouy".contains(*ch)) {
            kept_len >= 2
        } else {
            kept_len >= 3
                && (chars.get(1).is_some_and(|ch| "aeiouy".contains(*ch))
                    || chars.get(2).is_some_and(|ch| "aeiouy".contains(*ch)))
        }
    }

    fn apply_rule(&self, word: &str, remove_total: usize, append_string: &str) -> String {
        let new_len = word.len().saturating_sub(remove_total);
        let mut out = word[..new_len].to_string();
        if !append_string.is_empty() {
            out.push_str(append_string);
        }
        out
    }

    fn strip_prefixes(&self, word: &str) -> String {
        for prefix in [
            "kilo", "micro", "milli", "intra", "ultra", "mega", "nano", "pico", "pseudo",
        ] {
            if let Some(rest) = word.strip_prefix(prefix) {
                return rest.to_string();
            }
        }
        word.to_string()
    }
}

impl Stemmer for LancasterStemmer {
    fn stem_token(&self, token: &Token) -> Stem {
        Stem::new(self.stem_word(token.text()), token.span(), StemKind::Stem)
    }
}

fn default_rules() -> Vec<String> {
    [
        "ai*2.",
        "a*1.",
        "bb1.",
        "city3s.",
        "ci2>",
        "cn1t>",
        "dd1.",
        "dei3y>",
        "deec2ss.",
        "dee1.",
        "de2>",
        "dooh4>",
        "e1>",
        "feil1v.",
        "fi2>",
        "gni3>",
        "gai3y.",
        "ga2>",
        "gg1.",
        "ht*2.",
        "hsiug5ct.",
        "hsi3>",
        "i*1.",
        "i1y>",
        "ji1d.",
        "juf1s.",
        "ju1d.",
        "jo1d.",
        "jeh1r.",
        "jrev1t.",
        "jsim2t.",
        "jn1d.",
        "j1s.",
        "lbaifi6.",
        "lbai4y.",
        "lba3>",
        "lbi3.",
        "lib2l>",
        "lc1.",
        "lufi4y.",
        "luf3>",
        "lu2.",
        "lai3>",
        "lau3>",
        "la2>",
        "ll1.",
        "mui3.",
        "mu*2.",
        "msi3>",
        "mm1.",
        "nois4j>",
        "noix4ct.",
        "noi3>",
        "nai3>",
        "na2>",
        "nee0.",
        "ne2>",
        "nn1.",
        "pihs4>",
        "pp1.",
        "re2>",
        "rae0.",
        "ra2.",
        "ro2>",
        "ru2>",
        "rr1.",
        "rt1>",
        "rei3y>",
        "sei3y>",
        "sis2.",
        "si2>",
        "ssen4>",
        "ss0.",
        "suo3>",
        "su*2.",
        "s*1>",
        "s0.",
        "tacilp4y.",
        "ta2>",
        "tnem4>",
        "tne3>",
        "tna3>",
        "tpir2b.",
        "tpro2b.",
        "tcud1.",
        "tpmus2.",
        "tpec2iv.",
        "tulo2v.",
        "tsis0.",
        "tsi3>",
        "tt1.",
        "uqi3.",
        "ugo1.",
        "vis3j>",
        "vie0.",
        "vi2>",
        "ylb1>",
        "yli3y>",
        "ylp0.",
        "yl2>",
        "ygo1.",
        "yhp1.",
        "ymo1.",
        "ypo1.",
        "yti3>",
        "yte3>",
        "ytl2.",
        "yrtsi5.",
        "yra3>",
        "yro3>",
        "yfi3.",
        "ycn2t>",
        "yca3>",
        "zi2>",
        "zy1s.",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lancaster_examples() {
        let st = LancasterStemmer::default();
        assert_eq!(st.stem_word("maximum"), "maxim");
        assert_eq!(st.stem_word("presumably"), "presum");
        assert_eq!(st.stem_word("multiply"), "multiply");
        assert_eq!(st.stem_word("provision"), "provid");
        assert_eq!(st.stem_word("owed"), "ow");
        assert_eq!(st.stem_word("ear"), "ear");
    }

    #[test]
    fn lancaster_prefix_mode_example() {
        let st = LancasterStemmer::new(None, true);
        assert_eq!(st.stem_word("kilometer"), "met");
    }
}
