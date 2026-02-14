use std::collections::{HashMap, HashSet};

use regex::Regex;

use super::util::FeatureValue;

#[derive(Debug, Clone)]
pub struct RtePair {
    pub text: String,
    pub hyp: String,
    pub value: bool,
}

#[derive(Debug, Clone)]
pub struct RteFeatureExtractor {
    stop: bool,
    stopwords: HashSet<String>,
    negwords: HashSet<String>,
    text_tokens: Vec<String>,
    hyp_tokens: Vec<String>,
    text_words: HashSet<String>,
    hyp_words: HashSet<String>,
    overlap_cache: HashSet<String>,
    hyp_extra_cache: HashSet<String>,
}

impl RteFeatureExtractor {
    pub fn new(rtepair: &RtePair, stop: bool) -> Self {
        let stopwords = [
            "a", "the", "it", "they", "of", "in", "to", "is", "have", "are", "were", "and", "very",
            ".", ",",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

        let negwords = ["no", "not", "never", "failed", "rejected", "denied"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let tokenizer =
            Regex::new(r"[\w.@:/]+|\w+|\$[\d.]+").expect("RTE tokenizer regex should compile");

        let text_tokens = tokenize_with_regex(&tokenizer, &rtepair.text);
        let hyp_tokens = tokenize_with_regex(&tokenizer, &rtepair.hyp);

        let mut text_words: HashSet<String> = text_tokens.iter().cloned().collect();
        let mut hyp_words: HashSet<String> = hyp_tokens.iter().cloned().collect();

        if stop {
            text_words = text_words
                .difference(&stopwords)
                .cloned()
                .collect::<HashSet<_>>();
            hyp_words = hyp_words
                .difference(&stopwords)
                .cloned()
                .collect::<HashSet<_>>();
        }

        let overlap_cache = hyp_words.intersection(&text_words).cloned().collect();
        let hyp_extra_cache = hyp_words.difference(&text_words).cloned().collect();

        Self {
            stop,
            stopwords,
            negwords,
            text_tokens,
            hyp_tokens,
            text_words,
            hyp_words,
            overlap_cache,
            hyp_extra_cache,
        }
    }

    pub fn overlap(&self, toktype: TokenType) -> HashSet<String> {
        match toktype {
            TokenType::NamedEntity => self
                .overlap_cache
                .iter()
                .filter(|t| Self::is_named_entity(t))
                .cloned()
                .collect(),
            TokenType::Word => self
                .overlap_cache
                .iter()
                .filter(|t| !Self::is_named_entity(t))
                .cloned()
                .collect(),
        }
    }

    pub fn hyp_extra(&self, toktype: TokenType) -> HashSet<String> {
        match toktype {
            TokenType::NamedEntity => self
                .hyp_extra_cache
                .iter()
                .filter(|t| Self::is_named_entity(t))
                .cloned()
                .collect(),
            TokenType::Word => self
                .hyp_extra_cache
                .iter()
                .filter(|t| !Self::is_named_entity(t))
                .cloned()
                .collect(),
        }
    }

    pub fn neg_txt(&self) -> usize {
        self.negwords.intersection(&self.text_words).count()
    }

    pub fn neg_hyp(&self) -> usize {
        self.negwords.intersection(&self.hyp_words).count()
    }

    pub fn token_counts(&self) -> (usize, usize) {
        (self.text_tokens.len(), self.hyp_tokens.len())
    }

    pub fn stop_enabled(&self) -> bool {
        self.stop
    }

    pub fn stopword_count(&self) -> usize {
        self.stopwords.len()
    }

    fn is_named_entity(token: &str) -> bool {
        token
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
            || token.chars().all(|c| c.is_uppercase())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    NamedEntity,
    Word,
}

pub fn rte_features(rtepair: &RtePair) -> HashMap<String, FeatureValue> {
    let extractor = RteFeatureExtractor::new(rtepair, true);

    let mut features = HashMap::new();
    features.insert("alwayson".to_string(), FeatureValue::Bool(true));
    features.insert(
        "word_overlap".to_string(),
        FeatureValue::Int(extractor.overlap(TokenType::Word).len() as i64),
    );
    features.insert(
        "word_hyp_extra".to_string(),
        FeatureValue::Int(extractor.hyp_extra(TokenType::Word).len() as i64),
    );
    features.insert(
        "ne_overlap".to_string(),
        FeatureValue::Int(extractor.overlap(TokenType::NamedEntity).len() as i64),
    );
    features.insert(
        "ne_hyp_extra".to_string(),
        FeatureValue::Int(extractor.hyp_extra(TokenType::NamedEntity).len() as i64),
    );
    features.insert(
        "neg_txt".to_string(),
        FeatureValue::Int(extractor.neg_txt() as i64),
    );
    features.insert(
        "neg_hyp".to_string(),
        FeatureValue::Int(extractor.neg_hyp() as i64),
    );
    features
}

pub fn rte_featurize(rte_pairs: &[RtePair]) -> Vec<(HashMap<String, FeatureValue>, bool)> {
    rte_pairs
        .iter()
        .map(|pair| (rte_features(pair), pair.value))
        .collect()
}

fn tokenize_with_regex(regex: &Regex, text: &str) -> Vec<String> {
    regex
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rte_feature_extractor_counts_overlap() {
        let pair = RtePair {
            text: "John denied the report in London".to_string(),
            hyp: "John denied report".to_string(),
            value: true,
        };

        let extractor = RteFeatureExtractor::new(&pair, true);
        let word_overlap = extractor.overlap(TokenType::Word);
        assert!(!word_overlap.is_empty());

        let features = rte_features(&pair);
        assert!(features.contains_key("alwayson"));
        assert!(features.contains_key("word_overlap"));
    }
}
