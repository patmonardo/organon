use crate::collections::dataset::token::Token;
use crate::collections::dataset::tree::TreeValue;

use super::cfg::{Cfg, GrammarSymbol, ParseError, Production};

#[derive(Debug, Clone)]
enum StackItem {
    Token(String),
    Tree(TreeValue),
}

impl StackItem {
    fn as_tree_child(&self) -> TreeValue {
        match self {
            StackItem::Token(text) => TreeValue::leaf(text.clone()),
            StackItem::Tree(tree) => tree.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShiftReduceParser {
    grammar: Cfg,
    trace: u8,
}

impl ShiftReduceParser {
    pub fn new(grammar: Cfg) -> Self {
        Self { grammar, trace: 0 }
    }

    pub fn grammar(&self) -> &Cfg {
        &self.grammar
    }

    pub fn trace(&mut self, trace: u8) {
        self.trace = trace;
    }

    pub fn parse_tokens(&self, tokens: &[Token]) -> Result<Option<TreeValue>, ParseError> {
        let texts = tokens
            .iter()
            .map(|token| token.text().to_string())
            .collect::<Vec<_>>();
        self.parse_token_texts(texts)
    }

    pub fn parse_token_texts<I, S>(&self, tokens: I) -> Result<Option<TreeValue>, ParseError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tokens = tokens.into_iter().map(Into::into).collect::<Vec<_>>();
        self.grammar
            .check_coverage_texts(tokens.iter().map(String::as_str))?;

        let mut stack = Vec::new();
        let mut remaining_text = tokens;

        if self.trace > 0 {
            self.trace_stack(&stack, &remaining_text, ' ');
        }

        while !remaining_text.is_empty() {
            self.shift(&mut stack, &mut remaining_text);
            while self.reduce(&mut stack, &remaining_text, None).is_some() {}
        }

        if stack.len() != 1 {
            return Ok(None);
        }

        let Some(StackItem::Tree(tree)) = stack.into_iter().next() else {
            return Ok(None);
        };
        if tree.as_node().map(|node| node.label()) == Some(self.grammar.start()) {
            Ok(Some(tree))
        } else {
            Ok(None)
        }
    }

    fn shift(&self, stack: &mut Vec<StackItem>, remaining_text: &mut Vec<String>) {
        if remaining_text.is_empty() {
            return;
        }
        let token = remaining_text.remove(0);
        stack.push(StackItem::Token(token));
        if self.trace > 0 {
            self.trace_stack(stack, remaining_text, 'S');
        }
    }

    fn reduce(
        &self,
        stack: &mut Vec<StackItem>,
        remaining_text: &[String],
        production: Option<&Production>,
    ) -> Option<Production> {
        let productions = match production {
            Some(production) => vec![production.clone()],
            None => self.grammar.productions().to_vec(),
        };

        for production in productions {
            let rhs_len = production.rhs().len();
            if rhs_len > stack.len() {
                continue;
            }

            if self.match_rhs(production.rhs(), &stack[stack.len() - rhs_len..]) {
                let children = stack[stack.len() - rhs_len..]
                    .iter()
                    .map(StackItem::as_tree_child)
                    .collect::<Vec<_>>();
                stack.truncate(stack.len() - rhs_len);
                stack.push(StackItem::Tree(TreeValue::node(production.lhs(), children)));
                if self.trace > 0 {
                    self.trace_stack(stack, remaining_text, 'R');
                }
                return Some(production);
            }
        }

        None
    }

    fn match_rhs(&self, rhs: &[GrammarSymbol], rightmost_stack: &[StackItem]) -> bool {
        if rhs.len() != rightmost_stack.len() {
            return false;
        }

        for (rhs_symbol, stack_item) in rhs.iter().zip(rightmost_stack) {
            match (rhs_symbol, stack_item) {
                (GrammarSymbol::Terminal(rhs_token), StackItem::Token(token)) => {
                    if rhs_token != token {
                        return false;
                    }
                }
                (GrammarSymbol::Nonterminal(rhs_symbol), StackItem::Tree(tree)) => {
                    let Some(node) = tree.as_node() else {
                        return false;
                    };
                    if node.label() != rhs_symbol {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        true
    }

    fn trace_stack(&self, stack: &[StackItem], remaining_text: &[String], marker: char) {
        if self.trace == 0 {
            return;
        }
        let mut parts = Vec::new();
        for item in stack {
            match item {
                StackItem::Token(token) => parts.push(format!("'{token}'")),
                StackItem::Tree(tree) => {
                    let label = tree.as_node().map(|node| node.label()).unwrap_or("?");
                    parts.push(format!("<{label}>"));
                }
            }
        }
        let rem = remaining_text.join(" ");
        println!("  {marker} [ {} * {} ]", parts.join(" "), rem);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepOutcome {
    Shifted,
    Reduced(Production),
    NoOp,
}

#[derive(Debug, Clone)]
pub struct SteppingShiftReduceParser {
    parser: ShiftReduceParser,
    stack: Vec<StackItem>,
    remaining_text: Vec<String>,
    history: Vec<(Vec<StackItem>, Vec<String>)>,
}

impl SteppingShiftReduceParser {
    pub fn new(grammar: Cfg) -> Self {
        Self {
            parser: ShiftReduceParser::new(grammar),
            stack: Vec::new(),
            remaining_text: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn initialize_tokens(&mut self, tokens: &[Token]) -> Result<(), ParseError> {
        self.initialize_texts(tokens.iter().map(Token::text))
    }

    pub fn initialize_texts<'a>(
        &mut self,
        tokens: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), ParseError> {
        let texts = tokens
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        self.parser
            .grammar()
            .check_coverage_texts(texts.iter().map(String::as_str))?;
        self.stack.clear();
        self.remaining_text = texts;
        self.history.clear();
        Ok(())
    }

    pub fn step(&mut self) -> StepOutcome {
        match self.reduce(None) {
            StepOutcome::Reduced(production) => StepOutcome::Reduced(production),
            _ => self.shift(),
        }
    }

    pub fn shift(&mut self) -> StepOutcome {
        if self.remaining_text.is_empty() {
            return StepOutcome::NoOp;
        }
        self.history
            .push((self.stack.clone(), self.remaining_text.clone()));
        self.parser.shift(&mut self.stack, &mut self.remaining_text);
        StepOutcome::Shifted
    }

    pub fn reduce(&mut self, production: Option<&Production>) -> StepOutcome {
        self.history
            .push((self.stack.clone(), self.remaining_text.clone()));
        if let Some(production) =
            self.parser
                .reduce(&mut self.stack, &self.remaining_text, production)
        {
            StepOutcome::Reduced(production)
        } else {
            self.history.pop();
            StepOutcome::NoOp
        }
    }

    pub fn undo(&mut self) -> bool {
        let Some((stack, remaining_text)) = self.history.pop() else {
            return false;
        };
        self.stack = stack;
        self.remaining_text = remaining_text;
        true
    }

    pub fn reducible_productions(&self) -> Vec<Production> {
        self.parser
            .grammar()
            .productions()
            .iter()
            .filter(|production| {
                let rhs_len = production.rhs().len();
                rhs_len <= self.stack.len()
                    && self
                        .parser
                        .match_rhs(production.rhs(), &self.stack[self.stack.len() - rhs_len..])
            })
            .cloned()
            .collect()
    }

    pub fn parses(&self) -> Vec<TreeValue> {
        if self.remaining_text.is_empty()
            && self.stack.len() == 1
            && self
                .stack
                .first()
                .and_then(|item| match item {
                    StackItem::Tree(tree) => tree.as_node().map(|node| node.label().to_string()),
                    StackItem::Token(_) => None,
                })
                .as_deref()
                == Some(self.parser.grammar().start())
        {
            if let Some(StackItem::Tree(tree)) = self.stack.first() {
                return vec![tree.clone()];
            }
        }
        Vec::new()
    }

    pub fn stack_len(&self) -> usize {
        self.stack.len()
    }

    pub fn remaining_len(&self) -> usize {
        self.remaining_text.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::token::{TokenKind, TokenSpan};

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

    fn tokens() -> Vec<Token> {
        vec![
            Token::new("I", TokenSpan::new(0, 1), TokenKind::Word),
            Token::new("saw", TokenSpan::new(2, 5), TokenKind::Word),
            Token::new("her", TokenSpan::new(6, 9), TokenKind::Word),
        ]
    }

    #[test]
    fn shift_reduce_produces_single_parse() {
        let parser = ShiftReduceParser::new(grammar());
        let parse = parser
            .parse_tokens(&tokens())
            .expect("parsing should run")
            .expect("expected parse");
        assert_eq!(parse.format_bracketed(), "(S (NP I) (VP (V saw) (NP her)))");
    }

    #[test]
    fn stepping_parser_supports_undo() {
        let mut parser = SteppingShiftReduceParser::new(grammar());
        parser
            .initialize_tokens(&tokens())
            .expect("initialization should succeed");

        assert_eq!(parser.shift(), StepOutcome::Shifted);
        assert!(parser.stack_len() > 0);
        assert!(parser.undo());
        assert_eq!(parser.stack_len(), 0);
    }
}
