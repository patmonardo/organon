use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use crate::collections::dataset::dependency::DependencyGraph;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluationError {
    SentenceCountMismatch,
    TokenCountMismatch { sentence_index: usize },
    TokenSequenceMismatch { sentence_index: usize, token_index: usize },
    EmptyScoringSet,
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvaluationError::SentenceCountMismatch => {
                write!(f, "parsed and gold dependency sentence counts differ")
            }
            EvaluationError::TokenCountMismatch { sentence_index } => {
                write!(f, "token counts differ for sentence index {sentence_index}")
            }
            EvaluationError::TokenSequenceMismatch {
                sentence_index,
                token_index,
            } => write!(
                f,
                "token mismatch at sentence {sentence_index}, token {token_index}"
            ),
            EvaluationError::EmptyScoringSet => {
                write!(f, "no non-punctuation tokens available for scoring")
            }
        }
    }
}

impl Error for EvaluationError {}

#[derive(Debug, Clone)]
pub struct DependencyEvaluator {
    parsed_sents: Vec<DependencyGraph>,
    gold_sents: Vec<DependencyGraph>,
}

impl DependencyEvaluator {
    pub fn new(parsed_sents: Vec<DependencyGraph>, gold_sents: Vec<DependencyGraph>) -> Self {
        Self {
            parsed_sents,
            gold_sents,
        }
    }

    pub fn eval(&self) -> Result<(f64, f64), EvaluationError> {
        if self.parsed_sents.len() != self.gold_sents.len() {
            return Err(EvaluationError::SentenceCountMismatch);
        }

        let mut corr = 0usize;
        let mut corr_l = 0usize;
        let mut total = 0usize;

        for (sentence_index, (parsed, gold)) in self
            .parsed_sents
            .iter()
            .zip(self.gold_sents.iter())
            .enumerate()
        {
            if parsed.nodes().len() != gold.nodes().len() {
                return Err(EvaluationError::TokenCountMismatch { sentence_index });
            }

            let parsed_map = dep_map(parsed);
            let gold_map = dep_map(gold);

            for token_index in 0..parsed.nodes().len() {
                let parsed_node = &parsed.nodes()[token_index];
                let gold_node = &gold.nodes()[token_index];

                if parsed_node.text() != gold_node.text() {
                    return Err(EvaluationError::TokenSequenceMismatch {
                        sentence_index,
                        token_index,
                    });
                }

                if remove_punct(parsed_node.text()).is_empty() {
                    continue;
                }

                total += 1;

                let parsed_dep = parsed_map
                    .get(&token_index)
                    .cloned()
                    .unwrap_or((None, None));
                let gold_dep = gold_map.get(&token_index).cloned().unwrap_or((None, None));

                if parsed_dep.0 == gold_dep.0 {
                    corr += 1;
                    if parsed_dep.1 == gold_dep.1 {
                        corr_l += 1;
                    }
                }
            }
        }

        if total == 0 {
            return Err(EvaluationError::EmptyScoringSet);
        }

        Ok((corr_l as f64 / total as f64, corr as f64 / total as f64))
    }
}

fn dep_map(graph: &DependencyGraph) -> HashMap<usize, (Option<usize>, Option<String>)> {
    let mut map = HashMap::new();

    for edge in graph.edges() {
        map.insert(
            edge.dep(),
            (Some(edge.head()), Some(edge.label().to_string())),
        );
    }

    if let Some(root) = graph.root() {
        map.entry(root).or_insert((None, Some("ROOT".to_string())));
    }

    map
}

fn remove_punct(input: &str) -> String {
    input
        .chars()
        .filter(|ch| !is_punctuation(*ch))
        .collect::<String>()
}

fn is_punctuation(ch: char) -> bool {
    let cat = unicode_category(ch);
    matches!(cat, 'P')
}

fn unicode_category(ch: char) -> char {
    if ch.is_ascii_punctuation() {
        'P'
    } else if ch.is_alphanumeric() || ch.is_whitespace() {
        'A'
    } else {
        let code = ch as u32;
        if (0x2000..=0x206F).contains(&code)
            || (0x2E00..=0x2E7F).contains(&code)
            || (0x3000..=0x303F).contains(&code)
        {
            'P'
        } else {
            'A'
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::nlp::parse::parse_malt_tab;

    #[test]
    fn evaluator_scores_perfect_parse() {
        let gold = parse_malt_tab("John N 2 nsubj\nloves V 0 ROOT\nMary N 2 obj")
            .expect("valid graph");
        let parsed = parse_malt_tab("John N 2 nsubj\nloves V 0 ROOT\nMary N 2 obj")
            .expect("valid graph");

        let evaluator = DependencyEvaluator::new(vec![parsed], vec![gold]);
        let (las, uas) = evaluator.eval().expect("evaluation should succeed");
        assert!((las - 1.0).abs() < 1e-9);
        assert!((uas - 1.0).abs() < 1e-9);
    }

    #[test]
    fn evaluator_detects_relation_drop() {
        let gold = parse_malt_tab("John N 2 nsubj\nloves V 0 ROOT\nMary N 2 obj")
            .expect("valid graph");
        let parsed = parse_malt_tab("John N 2 nsubj\nloves V 0 ROOT\nMary N 2 dep")
            .expect("valid graph");

        let evaluator = DependencyEvaluator::new(vec![parsed], vec![gold]);
        let (las, uas) = evaluator.eval().expect("evaluation should succeed");
        assert!(las < uas);
        assert!((uas - 1.0).abs() < 1e-9);
    }
}
