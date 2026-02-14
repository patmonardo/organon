use std::error::Error;
use std::fmt;

use crate::collections::dataset::token::Token;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GrammarSymbol {
    Nonterminal(String),
    Terminal(String),
}

impl GrammarSymbol {
    pub fn nonterminal(symbol: impl Into<String>) -> Self {
        Self::Nonterminal(symbol.into())
    }

    pub fn terminal(symbol: impl Into<String>) -> Self {
        Self::Terminal(symbol.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
    lhs: String,
    rhs: Vec<GrammarSymbol>,
    prob: Option<String>,
}

impl Production {
    pub fn new(lhs: impl Into<String>, rhs: Vec<GrammarSymbol>) -> Self {
        Self {
            lhs: lhs.into(),
            rhs,
            prob: None,
        }
    }

    pub fn with_prob(mut self, prob: f64) -> Self {
        self.prob = Some(format!("{prob:.12}"));
        self
    }

    pub fn lhs(&self) -> &str {
        &self.lhs
    }

    pub fn rhs(&self) -> &[GrammarSymbol] {
        &self.rhs
    }

    pub fn prob(&self) -> Option<f64> {
        self.prob.as_deref().and_then(|v| v.parse::<f64>().ok())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    EmptyStart,
    EmptyProductions,
    EmptyRhs { lhs: String },
    UncoveredToken(String),
    InvalidHeadIndex { index: usize, head: isize },
    InvalidDependencyRow(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyStart => write!(f, "grammar start symbol cannot be empty"),
            ParseError::EmptyProductions => write!(f, "grammar must contain productions"),
            ParseError::EmptyRhs { lhs } => {
                write!(f, "epsilon productions are not supported for lhs '{lhs}'")
            }
            ParseError::UncoveredToken(token) => {
                write!(f, "token '{token}' is not covered by grammar terminals")
            }
            ParseError::InvalidHeadIndex { index, head } => {
                write!(f, "invalid dependency head {head} for token index {index}")
            }
            ParseError::InvalidDependencyRow(row) => {
                write!(f, "invalid dependency row: '{row}'")
            }
        }
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
pub struct Cfg {
    start: String,
    productions: Vec<Production>,
}

impl Cfg {
    pub fn new(start: impl Into<String>, productions: Vec<Production>) -> Result<Self, ParseError> {
        let start = start.into();
        if start.trim().is_empty() {
            return Err(ParseError::EmptyStart);
        }
        if productions.is_empty() {
            return Err(ParseError::EmptyProductions);
        }
        for production in &productions {
            if production.rhs.is_empty() {
                return Err(ParseError::EmptyRhs {
                    lhs: production.lhs.clone(),
                });
            }
        }
        Ok(Self { start, productions })
    }

    pub fn start(&self) -> &str {
        &self.start
    }

    pub fn productions(&self) -> &[Production] {
        &self.productions
    }

    pub fn check_coverage_tokens(&self, tokens: &[Token]) -> Result<(), ParseError> {
        self.check_coverage_texts(tokens.iter().map(Token::text))
    }

    pub fn check_coverage_texts<'a>(
        &self,
        tokens: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), ParseError> {
        for token in tokens {
            let covered = self.productions.iter().any(|production| {
                production
                    .rhs
                    .iter()
                    .any(|rhs| matches!(rhs, GrammarSymbol::Terminal(t) if t == token))
            });
            if !covered {
                return Err(ParseError::UncoveredToken(token.to_string()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cfg_rejects_empty_rhs() {
        let grammar = Cfg::new("S", vec![Production::new("S", Vec::new())]);
        assert!(matches!(grammar, Err(ParseError::EmptyRhs { .. })));
    }

    #[test]
    fn coverage_works_for_terminals() {
        let grammar = Cfg::new(
            "S",
            vec![
                Production::new("S", vec![GrammarSymbol::nonterminal("NP")]),
                Production::new("NP", vec![GrammarSymbol::terminal("dog")]),
            ],
        )
        .expect("valid grammar");

        assert!(grammar.check_coverage_texts(["dog"]).is_ok());
        assert!(matches!(
            grammar.check_coverage_texts(["cat"]),
            Err(ParseError::UncoveredToken(token)) if token == "cat"
        ));
    }
}
