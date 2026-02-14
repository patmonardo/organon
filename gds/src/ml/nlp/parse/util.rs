use crate::collections::dataset::tree::TreeValue;

use super::cfg::{Cfg, ParseError};
use super::recursivedescent::RecursiveDescentParser;
use super::shiftreduce::ShiftReduceParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserFlavor {
    RecursiveDescent,
    ShiftReduce,
}

#[derive(Debug, Clone)]
pub enum LoadedParser {
    RecursiveDescent(RecursiveDescentParser),
    ShiftReduce(ShiftReduceParser),
}

impl LoadedParser {
    pub fn parse_token_texts<'a>(
        &self,
        tokens: impl IntoIterator<Item = &'a str>,
    ) -> Result<Vec<TreeValue>, ParseError> {
        let tokens = tokens
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        match self {
            LoadedParser::RecursiveDescent(parser) => parser.parse_token_texts(tokens),
            LoadedParser::ShiftReduce(parser) => {
                let parse = parser.parse_token_texts(tokens)?;
                Ok(parse.into_iter().collect())
            }
        }
    }
}

pub fn load_parser(grammar: Cfg, flavor: ParserFlavor) -> LoadedParser {
    match flavor {
        ParserFlavor::RecursiveDescent => {
            LoadedParser::RecursiveDescent(RecursiveDescentParser::new(grammar))
        }
        ParserFlavor::ShiftReduce => LoadedParser::ShiftReduce(ShiftReduceParser::new(grammar)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpectedParse {
    Any,
    Grammatical(bool),
    Count(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarTestSentence {
    pub tokens: Vec<String>,
    pub expected: ExpectedParse,
}

impl GrammarTestSentence {
    pub fn from_tokens(tokens: Vec<String>) -> Self {
        Self {
            tokens,
            expected: ExpectedParse::Any,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarTestOutcome {
    pub test: GrammarTestSentence,
    pub parse_count: usize,
    pub passed: bool,
}

#[derive(Debug, Clone)]
pub struct GrammarTestRunner {
    parser: LoadedParser,
    suite: Vec<GrammarTestSentence>,
}

impl GrammarTestRunner {
    pub fn new(parser: LoadedParser, suite: Vec<GrammarTestSentence>) -> Self {
        Self { parser, suite }
    }

    pub fn run(&self) -> Result<Vec<GrammarTestOutcome>, ParseError> {
        let mut results = Vec::with_capacity(self.suite.len());

        for sentence in &self.suite {
            let parses = self
                .parser
                .parse_token_texts(sentence.tokens.iter().map(String::as_str))?;
            let parse_count = parses.len();
            let passed = match sentence.expected {
                ExpectedParse::Any => true,
                ExpectedParse::Grammatical(true) => parse_count > 0,
                ExpectedParse::Grammatical(false) => parse_count == 0,
                ExpectedParse::Count(count) => parse_count == count,
            };

            results.push(GrammarTestOutcome {
                test: sentence.clone(),
                parse_count,
                passed,
            });
        }

        Ok(results)
    }
}

pub fn extract_test_sentences(input: &str, comment_chars: &str) -> Vec<GrammarTestSentence> {
    let mut out = Vec::new();

    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            continue;
        }

        let Some(first) = line.chars().next() else {
            continue;
        };
        if comment_chars.contains(first) {
            continue;
        }

        let mut expected = ExpectedParse::Any;
        let mut sentence_text = line;

        if let Some((lhs, rhs)) = line.split_once(':') {
            let lhs = lhs.trim().to_ascii_lowercase();
            let rhs = rhs.trim();
            if rhs.is_empty() {
                continue;
            }

            if lhs == "true" || lhs == "yes" || lhs == "accept" {
                expected = ExpectedParse::Grammatical(true);
                sentence_text = rhs;
            } else if lhs == "false" || lhs == "no" || lhs == "reject" {
                expected = ExpectedParse::Grammatical(false);
                sentence_text = rhs;
            } else if let Ok(count) = lhs.parse::<usize>() {
                expected = ExpectedParse::Count(count);
                sentence_text = rhs;
            }
        }

        let tokens = sentence_text
            .split_whitespace()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }

        out.push(GrammarTestSentence { tokens, expected });
    }

    out
}

pub fn tagged_sentence_to_conll<'a>(
    sentence: impl IntoIterator<Item = (&'a str, &'a str)>,
) -> Vec<String> {
    sentence
        .into_iter()
        .enumerate()
        .map(|(index, (word, tag))| {
            [
                (index + 1).to_string(),
                word.to_string(),
                "_".to_string(),
                tag.to_string(),
                tag.to_string(),
                "_".to_string(),
                "0".to_string(),
                "a".to_string(),
                "_".to_string(),
                "_".to_string(),
            ]
            .join("\t")
        })
        .collect()
}

pub fn tagged_sentences_to_conll<'a>(
    sentences: impl IntoIterator<Item = Vec<(&'a str, &'a str)>>,
) -> Vec<String> {
    let mut lines = Vec::new();
    for sentence in sentences {
        lines.extend(tagged_sentence_to_conll(sentence));
        lines.push(String::new());
        lines.push(String::new());
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::nlp::parse::cfg::{GrammarSymbol, Production};

    fn grammar() -> Cfg {
        Cfg::new(
            "S",
            vec![
                Production::new("NP", vec![GrammarSymbol::terminal("I")]),
                Production::new("NP", vec![GrammarSymbol::terminal("her")]),
                Production::new("V", vec![GrammarSymbol::terminal("saw")]),
                Production::new(
                    "VP",
                    vec![
                        GrammarSymbol::nonterminal("V"),
                        GrammarSymbol::nonterminal("NP"),
                    ],
                ),
                Production::new(
                    "S",
                    vec![
                        GrammarSymbol::nonterminal("NP"),
                        GrammarSymbol::nonterminal("VP"),
                    ],
                ),
            ],
        )
        .expect("valid grammar")
    }

    #[test]
    fn extract_test_sentences_parses_prefixes() {
        let text = "# comment\ntrue: I saw her\n0: her saw I\nplain sentence";
        let suite = extract_test_sentences(text, "#%;");
        assert_eq!(suite.len(), 3);
        assert_eq!(suite[0].expected, ExpectedParse::Grammatical(true));
        assert_eq!(suite[1].expected, ExpectedParse::Count(0));
        assert_eq!(suite[2].expected, ExpectedParse::Any);
    }

    #[test]
    fn grammar_test_runner_checks_expected_counts() {
        let parser = load_parser(grammar(), ParserFlavor::RecursiveDescent);
        let suite = vec![
            GrammarTestSentence {
                tokens: vec!["I".into(), "saw".into(), "her".into()],
                expected: ExpectedParse::Grammatical(true),
            },
            GrammarTestSentence {
                tokens: vec!["saw".into(), "I".into(), "her".into()],
                expected: ExpectedParse::Count(0),
            },
        ];
        let runner = GrammarTestRunner::new(parser, suite);
        let results = runner.run().expect("runner should succeed");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|result| result.passed));
    }

    #[test]
    fn tagged_sentence_to_conll_shapes_rows() {
        let rows = tagged_sentence_to_conll(vec![("This", "DT"), ("works", "VBZ")]);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].starts_with("1\tThis\t_\tDT\tDT"));
    }
}
