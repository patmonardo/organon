//! Model-theoretic interpretation of FOL expressions.
//!
//! Port of `nltk/sem/evaluate.py` — the `satisfy()` interpreter. Given a
//! [`Model`] (domain + valuation of non-logical symbols) and an
//! [`Assignment`] for free variables, [`Model::satisfy`] returns the
//! semantic value of an [`Expression`] from [`crate::ml::nlp::sem::logic`].
//!
//! ## Two senses of "Valuation"
//!
//! This crate has **two** valuation types and they are not the same:
//!
//! - [`crate::collections::dataset::valuation::Valuation`] — row-shaped,
//!   `FeatureName → ValuationCell`. The R4 access object on the
//!   intensional side of the Five-Fold Synthesis.
//! - [`Valuation`] (this module) — symbol-table-shaped, `String →
//!   Denotation`, where each symbol denotes an individual, a truth
//!   value, or a relation (set of tuples). The classical Tarski model.
//!
//! Bridging the two is a future architectural conversation; see the
//! Document/Model meeting-point doctrine.
//!
//! ## Algorithm
//!
//! Standard recursive Tarski semantics:
//!
//! - Boolean connectives: ordinary truth tables.
//! - `P(t1, …, tn)`: collect arg values, check membership in `V(P)`.
//! - `∀x.φ`: true iff `φ` holds for every `x ↦ u` over the domain.
//! - `∃x.φ` / `ιx.φ`: true iff `φ` holds for some `x ↦ u`.
//! - `λx.φ`: returns a [`EvalValue::Function`] table from individuals to
//!   semantic values.
//! - Free variables are looked up in the [`Assignment`]; constants in
//!   the [`Valuation`]. An unknown symbol raises [`EvalError::Undefined`].

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::ml::nlp::sem::logic::{Expression, Variable};

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors raised by [`Model::satisfy`] / [`Model::evaluate`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalError {
    /// An atomic symbol was reached that is neither in the valuation
    /// nor in the assignment. Mirrors NLTK `Undefined`.
    Undefined(String),
    /// Tried to apply something whose value is not a relation or
    /// function table.
    NotApplicable(String),
    /// Domain mismatch when constructing an [`Assignment`].
    NotInDomain(String),
    /// Tried to assign a non-individual variable.
    NotAnIndividualVariable(String),
    /// Failed to parse a valuation literal.
    ParseError(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Undefined(s) => write!(f, "undefined symbol: {}", s),
            Self::NotApplicable(s) => write!(f, "not applicable: {}", s),
            Self::NotInDomain(s) => write!(f, "{} is not in the domain", s),
            Self::NotAnIndividualVariable(s) => write!(f, "{} is not an individual variable", s),
            Self::ParseError(s) => write!(f, "valuation parse error: {}", s),
        }
    }
}

impl std::error::Error for EvalError {}

// ---------------------------------------------------------------------------
// Denotation / EvalValue
// ---------------------------------------------------------------------------

/// Identifier of an individual in the model's domain. NLTK uses bare
/// strings (e.g. `"b1"`, `"g2"`); we follow that convention.
pub type Individual = String;

/// Value associated with a non-logical symbol in a [`Valuation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Denotation {
    /// A bare individual: e.g. `adam → "b1"`.
    Individual(Individual),
    /// A propositional truth value: e.g. `P → true`.
    Boolean(bool),
    /// An n-ary relation as a set of tuples of individuals. Unary
    /// predicates store unary tuples (e.g. `{("b1",), ("b2",)}`).
    Relation(BTreeSet<Vec<Individual>>),
}

/// Semantic value returned by [`Model::satisfy`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalValue {
    Bool(bool),
    Individual(Individual),
    /// A relation lifted from a predicate symbol's denotation.
    Relation(BTreeSet<Vec<Individual>>),
    /// A function table from domain individuals to semantic values —
    /// the result of evaluating a `λx. φ`.
    Function(BTreeMap<Individual, EvalValue>),
}

impl EvalValue {
    /// Coerce to a boolean. Returns `None` if this value is not
    /// boolean-shaped.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            EvalValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Valuation (symbol table)
// ---------------------------------------------------------------------------

/// Symbol-table valuation: `String → Denotation`. Mirrors NLTK
/// `nltk.sem.evaluate.Valuation`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Valuation {
    table: BTreeMap<String, Denotation>,
}

impl Valuation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pairs<I>(pairs: I) -> Self
    where
        I: IntoIterator<Item = (String, Denotation)>,
    {
        let mut v = Valuation::new();
        for (k, d) in pairs {
            v.set(k, d);
        }
        v
    }

    pub fn set(&mut self, symbol: impl Into<String>, denotation: Denotation) {
        self.table.insert(symbol.into(), denotation);
    }

    pub fn get(&self, symbol: &str) -> Option<&Denotation> {
        self.table.get(symbol)
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.table.contains_key(symbol)
    }

    pub fn symbols(&self) -> impl Iterator<Item = &str> {
        self.table.keys().map(|s| s.as_str())
    }

    /// All individuals appearing in the value space — both bare
    /// individuals and the elements of every relation tuple.
    pub fn domain(&self) -> BTreeSet<Individual> {
        let mut out = BTreeSet::new();
        for d in self.table.values() {
            match d {
                Denotation::Individual(i) => {
                    out.insert(i.clone());
                }
                Denotation::Boolean(_) => {}
                Denotation::Relation(rel) => {
                    for tup in rel {
                        for x in tup {
                            out.insert(x.clone());
                        }
                    }
                }
            }
        }
        out
    }

    /// Parse a NLTK-style valuation literal:
    ///
    /// ```text
    /// adam => b1
    /// boy  => {b1, b2}
    /// love => {(b1, g1), (b2, g2)}
    /// P    => true
    /// ```
    ///
    /// Lines starting with `#` and blank lines are skipped.
    pub fn from_str(s: &str) -> Result<Self, EvalError> {
        let mut v = Valuation::new();
        for (lineno, raw) in s.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let (sym, val) = line.split_once("=>").ok_or_else(|| {
                EvalError::ParseError(format!("line {}: missing `=>`", lineno + 1))
            })?;
            let symbol = sym.trim().to_string();
            let val = val.trim();
            let den = if val.eq_ignore_ascii_case("true") {
                Denotation::Boolean(true)
            } else if val.eq_ignore_ascii_case("false") {
                Denotation::Boolean(false)
            } else if let Some(inner) = val.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                Denotation::Relation(parse_relation(inner)?)
            } else {
                Denotation::Individual(val.to_string())
            };
            v.set(symbol, den);
        }
        Ok(v)
    }
}

fn parse_relation(s: &str) -> Result<BTreeSet<Vec<Individual>>, EvalError> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(BTreeSet::new());
    }
    let mut out = BTreeSet::new();
    // Distinguish tuple form `(a, b), (c, d)` from atom form `a, b`.
    if trimmed.contains('(') {
        // Tuple form.
        let mut chars = trimmed.chars().peekable();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || c == ',' {
                chars.next();
                continue;
            }
            if c != '(' {
                return Err(EvalError::ParseError(format!(
                    "expected `(` in tuple set near {:?}",
                    c
                )));
            }
            chars.next(); // (
            let mut tuple_chars = String::new();
            for c in chars.by_ref() {
                if c == ')' {
                    break;
                }
                tuple_chars.push(c);
            }
            let elems: Vec<Individual> = tuple_chars
                .split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect();
            if elems.is_empty() {
                return Err(EvalError::ParseError("empty tuple".into()));
            }
            out.insert(elems);
        }
    } else {
        // Bare individuals: `{a, b, c}` → `{(a,), (b,), (c,)}`.
        for piece in trimmed.split(',') {
            let p = piece.trim();
            if p.is_empty() {
                continue;
            }
            out.insert(vec![p.to_string()]);
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Assignment
// ---------------------------------------------------------------------------

/// Variable assignment: maps individual-variable names to elements of
/// the model's domain. Mirrors NLTK `Assignment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    domain: BTreeSet<Individual>,
    map: BTreeMap<String, Individual>,
}

impl Assignment {
    pub fn new(domain: BTreeSet<Individual>) -> Self {
        Self {
            domain,
            map: BTreeMap::new(),
        }
    }

    pub fn from_pairs<I>(domain: BTreeSet<Individual>, pairs: I) -> Result<Self, EvalError>
    where
        I: IntoIterator<Item = (String, Individual)>,
    {
        let mut a = Assignment::new(domain);
        for (k, v) in pairs {
            a.add(k, v)?;
        }
        Ok(a)
    }

    pub fn domain(&self) -> &BTreeSet<Individual> {
        &self.domain
    }

    /// Bind an individual variable. NLTK only allows the binder name to
    /// match the individual-variable convention (`u..z` + digits).
    pub fn add(&mut self, var: impl Into<String>, value: Individual) -> Result<(), EvalError> {
        let var = var.into();
        if !Variable::is_individual_name(&var) {
            return Err(EvalError::NotAnIndividualVariable(var));
        }
        if !self.domain.contains(&value) {
            return Err(EvalError::NotInDomain(value));
        }
        self.map.insert(var, value);
        Ok(())
    }

    pub fn get(&self, var: &str) -> Option<&Individual> {
        self.map.get(var)
    }

    /// Return a new assignment with `var ↦ value` set, leaving `self`
    /// untouched. Used by quantifier rules.
    pub fn extended(&self, var: impl Into<String>, value: Individual) -> Result<Self, EvalError> {
        let mut next = self.clone();
        next.add(var, value)?;
        Ok(next)
    }
}

// ---------------------------------------------------------------------------
// Model
// ---------------------------------------------------------------------------

/// First-order model: a set-of-individuals domain plus a [`Valuation`]
/// of non-logical symbols. Mirrors NLTK `Model`.
#[derive(Debug, Clone)]
pub struct Model {
    domain: BTreeSet<Individual>,
    valuation: Valuation,
}

impl Model {
    /// Build a model. The valuation's value-space domain must be a
    /// subset of `domain`; this is checked.
    pub fn new(domain: BTreeSet<Individual>, valuation: Valuation) -> Result<Self, EvalError> {
        let val_dom = valuation.domain();
        for x in &val_dom {
            if !domain.contains(x) {
                return Err(EvalError::NotInDomain(x.clone()));
            }
        }
        Ok(Self { domain, valuation })
    }

    pub fn domain(&self) -> &BTreeSet<Individual> {
        &self.domain
    }

    pub fn valuation(&self) -> &Valuation {
        &self.valuation
    }

    /// Parse `expr` and evaluate against this model + assignment.
    pub fn evaluate(&self, expr: &str, g: &Assignment) -> Result<EvalValue, EvalError> {
        let parsed = crate::ml::nlp::sem::logic::parse(expr)
            .map_err(|e| EvalError::ParseError(e.to_string()))?;
        self.satisfy(&parsed, g)
    }

    /// Recursive Tarski interpretation. Mirrors NLTK
    /// `Model.satisfy(parsed, g)`.
    pub fn satisfy(&self, expr: &Expression, g: &Assignment) -> Result<EvalValue, EvalError> {
        match expr {
            Expression::Variable(_) | Expression::Constant(_) => self.interpret_atom(expr, g),

            Expression::Application(_, _) => self.satisfy_application(expr, g),

            Expression::Negated(t) => {
                let v = self.satisfy(t, g)?;
                Ok(EvalValue::Bool(!truthy(&v)))
            }
            Expression::And(a, b) => {
                let av = self.satisfy(a, g)?;
                if !truthy(&av) {
                    return Ok(EvalValue::Bool(false));
                }
                let bv = self.satisfy(b, g)?;
                Ok(EvalValue::Bool(truthy(&bv)))
            }
            Expression::Or(a, b) => {
                let av = self.satisfy(a, g)?;
                if truthy(&av) {
                    return Ok(EvalValue::Bool(true));
                }
                let bv = self.satisfy(b, g)?;
                Ok(EvalValue::Bool(truthy(&bv)))
            }
            Expression::Imp(a, b) => {
                let av = self.satisfy(a, g)?;
                if !truthy(&av) {
                    return Ok(EvalValue::Bool(true));
                }
                let bv = self.satisfy(b, g)?;
                Ok(EvalValue::Bool(truthy(&bv)))
            }
            Expression::Iff(a, b) => {
                let av = self.satisfy(a, g)?;
                let bv = self.satisfy(b, g)?;
                Ok(EvalValue::Bool(truthy(&av) == truthy(&bv)))
            }
            Expression::Equality(a, b) => {
                let av = self.satisfy(a, g)?;
                let bv = self.satisfy(b, g)?;
                Ok(EvalValue::Bool(av == bv))
            }
            Expression::All(v, body) => {
                for u in self.domain.iter().cloned() {
                    let g2 = g.extended(v.name().to_string(), u)?;
                    let bv = self.satisfy(body, &g2)?;
                    if !truthy(&bv) {
                        return Ok(EvalValue::Bool(false));
                    }
                }
                Ok(EvalValue::Bool(true))
            }
            Expression::Exists(v, body) | Expression::Iota(v, body) => {
                for u in self.domain.iter().cloned() {
                    let g2 = g.extended(v.name().to_string(), u)?;
                    let bv = self.satisfy(body, &g2)?;
                    if truthy(&bv) {
                        return Ok(EvalValue::Bool(true));
                    }
                }
                Ok(EvalValue::Bool(false))
            }
            Expression::Lambda(v, body) => {
                let mut table = BTreeMap::new();
                for u in self.domain.iter().cloned() {
                    let g2 = g.extended(v.name().to_string(), u.clone())?;
                    let val = self.satisfy(body, &g2)?;
                    table.insert(u, val);
                }
                Ok(EvalValue::Function(table))
            }
        }
    }

    /// Predicate / function application.
    fn satisfy_application(
        &self,
        expr: &Expression,
        g: &Assignment,
    ) -> Result<EvalValue, EvalError> {
        let (head, args) = uncurry(expr);

        // Case 1: head is a non-logical symbol (variable or constant).
        // Look up its denotation; expect a Relation; check tuple
        // membership.
        if matches!(head, Expression::Variable(_) | Expression::Constant(_)) {
            let head_value = self.satisfy(&head, g)?;
            let mut argvals: Vec<Individual> = Vec::with_capacity(args.len());
            for a in &args {
                let v = self.satisfy(a, g)?;
                argvals.push(value_as_individual(&v)?);
            }
            return match head_value {
                EvalValue::Relation(rel) => Ok(EvalValue::Bool(rel.contains(&argvals))),
                EvalValue::Bool(b) if args.is_empty() => Ok(EvalValue::Bool(b)),
                other => Err(EvalError::NotApplicable(format!(
                    "head value {:?} is not a relation",
                    other
                ))),
            };
        }

        // Case 2: head is a lambda (or other complex form). Evaluate
        // function then argument and look up in the function table.
        match expr {
            Expression::Application(func, arg) => {
                let fv = self.satisfy(func, g)?;
                let av = self.satisfy(arg, g)?;
                let key = value_as_individual(&av)?;
                match fv {
                    EvalValue::Function(table) => table
                        .get(&key)
                        .cloned()
                        .ok_or_else(|| EvalError::Undefined(key)),
                    other => Err(EvalError::NotApplicable(format!(
                        "non-function head: {:?}",
                        other
                    ))),
                }
            }
            _ => unreachable!(),
        }
    }

    /// Interpret a bare atom (Variable or Constant). NLTK's `i()`:
    /// valuation lookup wins over assignment lookup.
    fn interpret_atom(&self, expr: &Expression, g: &Assignment) -> Result<EvalValue, EvalError> {
        let name = match expr {
            Expression::Variable(v) => v.name(),
            Expression::Constant(c) => c.as_str(),
            _ => unreachable!(),
        };
        if let Some(d) = self.valuation.get(name) {
            return Ok(match d.clone() {
                Denotation::Individual(i) => EvalValue::Individual(i),
                Denotation::Boolean(b) => EvalValue::Bool(b),
                Denotation::Relation(r) => EvalValue::Relation(r),
            });
        }
        if let Expression::Variable(v) = expr {
            if Variable::is_individual_name(v.name()) {
                if let Some(u) = g.get(v.name()) {
                    return Ok(EvalValue::Individual(u.clone()));
                }
            }
        }
        Err(EvalError::Undefined(name.to_string()))
    }
}

// ---------------------------------------------------------------------------
// Small helpers
// ---------------------------------------------------------------------------

/// Walk an [`Expression::Application`] tree left-spine, returning the
/// head and the list of arguments in left-to-right order. Mirrors
/// NLTK `ApplicationExpression.uncurry`.
fn uncurry(expr: &Expression) -> (Expression, Vec<Expression>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expression::Application(f, x) = cur {
        args.push((**x).clone());
        cur = f;
    }
    args.reverse();
    (cur.clone(), args)
}

fn truthy(v: &EvalValue) -> bool {
    match v {
        EvalValue::Bool(b) => *b,
        _ => true,
    }
}

fn value_as_individual(v: &EvalValue) -> Result<Individual, EvalError> {
    match v {
        EvalValue::Individual(i) => Ok(i.clone()),
        other => Err(EvalError::NotApplicable(format!(
            "expected individual, got {:?}",
            other
        ))),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn fol_model() -> (Model, Assignment) {
        // Mirrors NLTK's `folmodel` demo.
        let v = Valuation::from_str(
            r#"
            adam  => b1
            betty => g1
            fido  => d1
            girl  => {g1, g2}
            boy   => {b1, b2}
            dog   => {d1}
            love  => {(b1, g1), (b2, g2), (g1, b1), (g2, b1)}
            "#,
        )
        .unwrap();
        let dom = v.domain();
        let m = Model::new(dom.clone(), v).unwrap();
        let g = Assignment::from_pairs(
            dom,
            vec![
                ("x".to_string(), "b1".to_string()),
                ("y".to_string(), "g2".to_string()),
            ],
        )
        .unwrap();
        (m, g)
    }

    fn prop_model() -> (Model, Assignment) {
        let v = Valuation::from_pairs(vec![
            ("P".to_string(), Denotation::Boolean(true)),
            ("Q".to_string(), Denotation::Boolean(true)),
            ("R".to_string(), Denotation::Boolean(false)),
        ]);
        let m = Model::new(BTreeSet::new(), v).unwrap();
        let g = Assignment::new(BTreeSet::new());
        (m, g)
    }

    fn b(v: EvalValue) -> bool {
        v.as_bool().unwrap()
    }

    // ---- valuation parsing ----

    #[test]
    fn parse_individual() {
        let v = Valuation::from_str("adam => b1").unwrap();
        assert_eq!(v.get("adam"), Some(&Denotation::Individual("b1".into())));
    }

    #[test]
    fn parse_unary_relation() {
        let v = Valuation::from_str("boy => {b1, b2}").unwrap();
        let mut want = BTreeSet::new();
        want.insert(vec!["b1".to_string()]);
        want.insert(vec!["b2".to_string()]);
        assert_eq!(v.get("boy"), Some(&Denotation::Relation(want)));
    }

    #[test]
    fn parse_binary_relation() {
        let v = Valuation::from_str("love => {(b1, g1), (b2, g2)}").unwrap();
        let mut want = BTreeSet::new();
        want.insert(vec!["b1".to_string(), "g1".to_string()]);
        want.insert(vec!["b2".to_string(), "g2".to_string()]);
        assert_eq!(v.get("love"), Some(&Denotation::Relation(want)));
    }

    #[test]
    fn parse_truth() {
        let v = Valuation::from_str("P => true\nQ => false").unwrap();
        assert_eq!(v.get("P"), Some(&Denotation::Boolean(true)));
        assert_eq!(v.get("Q"), Some(&Denotation::Boolean(false)));
    }

    #[test]
    fn valuation_domain_collects_all_individuals() {
        let v = Valuation::from_str(
            r#"
            adam => b1
            love => {(b1, g1), (b2, g2)}
            "#,
        )
        .unwrap();
        let dom = v.domain();
        assert!(dom.contains("b1"));
        assert!(dom.contains("g1"));
        assert!(dom.contains("b2"));
        assert!(dom.contains("g2"));
    }

    // ---- propositional ----

    #[test]
    fn prop_and_truth_table() {
        let (m, g) = prop_model();
        assert!(b(m.evaluate("(P & Q)", &g).unwrap()));
        assert!(!b(m.evaluate("(P & R)", &g).unwrap()));
    }

    #[test]
    fn prop_or_truth_table() {
        let (m, g) = prop_model();
        assert!(b(m.evaluate("(P | R)", &g).unwrap()));
        assert!(!b(m.evaluate("(R | R)", &g).unwrap()));
    }

    #[test]
    fn prop_implication() {
        let (m, g) = prop_model();
        assert!(b(m.evaluate("(P -> Q)", &g).unwrap()));
        assert!(!b(m.evaluate("(P -> R)", &g).unwrap()));
        assert!(b(m.evaluate("(R -> P)", &g).unwrap()));
    }

    #[test]
    fn prop_iff() {
        let (m, g) = prop_model();
        assert!(b(m.evaluate("(P <-> P)", &g).unwrap()));
        assert!(!b(m.evaluate("(P <-> R)", &g).unwrap()));
    }

    #[test]
    fn prop_negation() {
        let (m, g) = prop_model();
        assert!(!b(m.evaluate("-P", &g).unwrap()));
        assert!(b(m.evaluate("-R", &g).unwrap()));
        assert!(b(m.evaluate("--P", &g).unwrap()));
    }

    // ---- atomic predicate application ----

    #[test]
    fn unary_predicate_membership() {
        let (m, g) = fol_model();
        assert!(b(m.evaluate("boy(adam)", &g).unwrap()));
        assert!(!b(m.evaluate("boy(betty)", &g).unwrap()));
        assert!(b(m.evaluate("girl(betty)", &g).unwrap()));
    }

    #[test]
    fn binary_predicate_membership() {
        let (m, g) = fol_model();
        assert!(b(m.evaluate("love(adam, betty)", &g).unwrap()));
        assert!(!b(m.evaluate("love(adam, fido)", &g).unwrap()));
    }

    #[test]
    fn equality_on_individuals() {
        let (m, g) = fol_model();
        assert!(b(m.evaluate("(adam = adam)", &g).unwrap()));
        assert!(!b(m.evaluate("(adam = betty)", &g).unwrap()));
    }

    // ---- quantifiers ----

    #[test]
    fn forall_with_implication() {
        // ∀x. (boy(x) → -girl(x))
        let (m, g) = fol_model();
        assert!(b(m.evaluate("all x. (boy(x) -> -girl(x))", &g).unwrap()));
    }

    #[test]
    fn exists_witness() {
        let (m, g) = fol_model();
        assert!(b(m.evaluate("some x. boy(x)", &g).unwrap()));
        assert!(b(m.evaluate("some x. love(x, betty)", &g).unwrap()));
    }

    #[test]
    fn exists_no_witness_returns_false() {
        // No element loves itself in the love relation.
        let (m, g) = fol_model();
        assert!(!b(m.evaluate("some x. love(x, x)", &g).unwrap()));
    }

    #[test]
    fn forall_fails_when_counterexample_exists() {
        // Not every individual is a boy.
        let (m, g) = fol_model();
        assert!(!b(m.evaluate("all x. boy(x)", &g).unwrap()));
    }

    #[test]
    fn nested_quantifier_with_individual_constant() {
        // ∀x. (boy(x) → ∃y. love(x, y))
        let (m, g) = fol_model();
        assert!(b(m
            .evaluate("all x. (boy(x) -> some y. love(x, y))", &g)
            .unwrap()));
    }

    // ---- variable assignment ----

    #[test]
    fn free_individual_variable_uses_assignment() {
        let (m, g) = fol_model();
        // x is bound by g to b1 (= adam).
        assert!(b(m.evaluate("boy(x)", &g).unwrap()));
        // y is bound by g to g2.
        assert!(b(m.evaluate("girl(y)", &g).unwrap()));
    }

    #[test]
    fn undefined_symbol_raises() {
        let (m, g) = fol_model();
        let err = m.evaluate("zonk(adam)", &g).unwrap_err();
        assert!(matches!(err, EvalError::Undefined(_)));
    }

    // ---- lambda ----

    #[test]
    fn lambda_application_evaluates() {
        // (λx. boy(x))(adam)  ≡  boy(adam)
        let (m, g) = fol_model();
        assert!(b(m.evaluate("(\\x. boy(x))(adam)", &g).unwrap()));
        assert!(!b(m.evaluate("(\\x. boy(x))(betty)", &g).unwrap()));
    }

    // ---- assignment guards ----

    #[test]
    fn assignment_rejects_non_individual_var() {
        let dom: BTreeSet<Individual> = vec!["b1".to_string()].into_iter().collect();
        let mut a = Assignment::new(dom);
        let err = a.add("P", "b1".to_string()).unwrap_err();
        assert!(matches!(err, EvalError::NotAnIndividualVariable(_)));
    }

    #[test]
    fn assignment_rejects_value_outside_domain() {
        let dom: BTreeSet<Individual> = vec!["b1".to_string()].into_iter().collect();
        let mut a = Assignment::new(dom);
        let err = a.add("x", "ghost".to_string()).unwrap_err();
        assert!(matches!(err, EvalError::NotInDomain(_)));
    }
}
