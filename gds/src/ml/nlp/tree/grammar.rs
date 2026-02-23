//! Grammar concepts such as Productions and Nonterminals,
//! designed for parity with `nltk.grammar`.

use crate::collections::dataset::tree::TreeValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nonterminal(pub String);

impl Nonterminal {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self(symbol.into())
    }

    pub fn symbol(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Nonterminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductionRhs {
    Nonterminal(Nonterminal),
    Terminal(String), // Or whatever terminal representation we want
}

impl std::fmt::Display for ProductionRhs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductionRhs::Nonterminal(nt) => write!(f, "{}", nt),
            ProductionRhs::Terminal(t) => write!(f, "'{}'", t),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Production {
    pub lhs: Nonterminal,
    pub rhs: Vec<ProductionRhs>,
}

impl Production {
    pub fn new(lhs: Nonterminal, rhs: Vec<ProductionRhs>) -> Self {
        Self { lhs, rhs }
    }
}

impl std::fmt::Display for Production {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ->", self.lhs)?;
        for r in &self.rhs {
            write!(f, " {}", r)?;
        }
        Ok(())
    }
}

pub trait GrammarTreeExt {
    /// Generate the productions that characterize this tree.
    fn productions(&self) -> Vec<Production>;
}

impl GrammarTreeExt for TreeValue {
    fn productions(&self) -> Vec<Production> {
        let mut prods = Vec::new();
        collect_productions(self, &mut prods);
        prods
    }
}

fn collect_productions(tree: &TreeValue, prods: &mut Vec<Production>) {
    if let TreeValue::Node(node) = tree {
        let lhs = Nonterminal::new(node.label());
        let mut rhs = Vec::new();

        for child in node.children() {
            match child {
                TreeValue::Node(c_node) => {
                    rhs.push(ProductionRhs::Nonterminal(Nonterminal::new(c_node.label())));
                }
                TreeValue::Leaf(leaf) => {
                    // Extract a string representation for the terminal
                    let t_val = match leaf {
                        crate::collections::dataset::tree::TreeLeafValue::Text(t) => t.clone(),
                        _ => format!("{:?}", leaf), // Fallback for other leaf types
                    };
                    rhs.push(ProductionRhs::Terminal(t_val));
                }
            }
        }

        prods.push(Production::new(lhs, rhs));

        // Recurse into children
        for child in node.children() {
            collect_productions(child, prods);
        }
    }
}
