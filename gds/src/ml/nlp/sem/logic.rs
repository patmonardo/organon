//! Lambda calculus + first-order logic — port of NLTK `nltk/sem/logic.py`.
//!
//! ## Scope of this slice
//!
//! Ported:
//! - [`Variable`], [`Expression`] (single enum: variables, constants,
//!   application, lambda, quantifiers, booleans, equality, negation).
//! - Capture-avoiding substitution ([`Expression::replace`]).
//! - Free variables, constants, all variables.
//! - Alpha-conversion ([`Expression::alpha_convert`]).
//! - Beta-reduction + structural simplification ([`Expression::simplify`]).
//! - Pretty-printing matching NLTK conventions.
//! - [`LogicParser`] — recursive-descent parser with explicit precedence.
//! - NLTK conventions for individual / function / event variables
//!   (`u..z`, uppercase, `e\d*`).
//!
//! Deferred (will be added in follow-on slices):
//! - The full type system (`Type`, `ComplexType`, `typecheck`).
//! - Skolemization (own module: `sem::skolemize`).
//! - `SubstituteBindingsI` integration with
//!   [`crate::collections::dataset::featstruct::FeatBindings`].
//! - Quote-character handling in the parser
//!   (NLTK `LogicParser.quote_chars`).

use std::collections::BTreeSet;
use std::fmt;

// ---------------------------------------------------------------------------
// Variable
// ---------------------------------------------------------------------------

/// A logical variable identified by name. Mirrors NLTK `Variable`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns true if `name` matches NLTK's individual-variable convention:
    /// a single letter `u..z` optionally followed by digits.
    pub fn is_individual_name(name: &str) -> bool {
        let mut chars = name.chars();
        match chars.next() {
            Some(c) if matches!(c, 'u'..='z') => chars.all(|c| c.is_ascii_digit()),
            _ => false,
        }
    }

    /// Returns true if `name` matches NLTK's function-variable convention:
    /// an uppercase ASCII letter optionally followed by digits.
    pub fn is_function_name(name: &str) -> bool {
        let mut chars = name.chars();
        match chars.next() {
            Some(c) if c.is_ascii_uppercase() => chars.all(|c| c.is_ascii_digit()),
            _ => false,
        }
    }

    /// Returns true if `name` matches NLTK's event-variable convention:
    /// `e` followed by zero or more digits.
    pub fn is_event_name(name: &str) -> bool {
        let mut chars = name.chars();
        match chars.next() {
            Some('e') => chars.all(|c| c.is_ascii_digit()),
            _ => false,
        }
    }

    /// Returns true if this name is treated as a substitutable variable
    /// (rather than a constant) by the NLTK conventions.
    pub fn is_variable_name(name: &str) -> bool {
        Self::is_individual_name(name) || Self::is_function_name(name) || Self::is_event_name(name)
    }
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}

/// Generate a fresh variable not in `used`. Mirrors NLTK
/// `unique_variable`. Default base is `"z"`; collisions are resolved by
/// appending an integer suffix.
pub fn unique_variable(base: Option<&str>, used: &BTreeSet<Variable>) -> Variable {
    let raw_base = base.unwrap_or("z");
    // Strip trailing digits so we restart numbering cleanly.
    let stem: String = raw_base
        .trim_end_matches(|c: char| c.is_ascii_digit())
        .to_string();
    let stem = if stem.is_empty() {
        "z".to_string()
    } else {
        stem
    };
    let mut n: u64 = 2;
    loop {
        let candidate = Variable::new(format!("{}{}", stem, n));
        if !used.contains(&candidate) {
            return candidate;
        }
        n += 1;
    }
}

// ---------------------------------------------------------------------------
// Expression AST
// ---------------------------------------------------------------------------

/// First-order / lambda expression. A single enum subsumes NLTK's
/// `Expression` class hierarchy:
///
/// | NLTK class                          | Variant                  |
/// |-------------------------------------|--------------------------|
/// | `IndividualVariableExpression`      | `Variable`               |
/// | `FunctionVariableExpression`        | `Variable`               |
/// | `EventVariableExpression`           | `Variable`               |
/// | `ConstantExpression`                | `Constant`               |
/// | `ApplicationExpression`             | `Application`            |
/// | `LambdaExpression`                  | `Lambda`                 |
/// | `ExistsExpression`                  | `Exists`                 |
/// | `AllExpression`                     | `All`                    |
/// | `IotaExpression`                    | `Iota`                   |
/// | `NegatedExpression`                 | `Negated`                |
/// | `AndExpression`                     | `And`                    |
/// | `OrExpression`                      | `Or`                     |
/// | `ImpExpression`                     | `Imp`                    |
/// | `IffExpression`                     | `Iff`                    |
/// | `EqualityExpression`                | `Equality`               |
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Expression {
    Variable(Variable),
    Constant(String),
    Application(Box<Expression>, Box<Expression>),
    Lambda(Variable, Box<Expression>),
    Exists(Variable, Box<Expression>),
    All(Variable, Box<Expression>),
    Iota(Variable, Box<Expression>),
    Negated(Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Imp(Box<Expression>, Box<Expression>),
    Iff(Box<Expression>, Box<Expression>),
    Equality(Box<Expression>, Box<Expression>),
}

impl Expression {
    // ----- constructors ----------------------------------------------------

    pub fn var(name: impl Into<String>) -> Self {
        Expression::Variable(Variable::new(name))
    }

    pub fn constant(name: impl Into<String>) -> Self {
        Expression::Constant(name.into())
    }

    /// Build a leaf from a name, classifying it as variable or constant by
    /// NLTK convention.
    pub fn from_atom(name: impl Into<String>) -> Self {
        let name = name.into();
        if Variable::is_variable_name(&name) {
            Expression::Variable(Variable::new(name))
        } else {
            Expression::Constant(name)
        }
    }

    pub fn app(f: Expression, x: Expression) -> Self {
        Expression::Application(Box::new(f), Box::new(x))
    }

    pub fn lambda(var: Variable, body: Expression) -> Self {
        Expression::Lambda(var, Box::new(body))
    }

    pub fn exists(var: Variable, body: Expression) -> Self {
        Expression::Exists(var, Box::new(body))
    }

    pub fn all(var: Variable, body: Expression) -> Self {
        Expression::All(var, Box::new(body))
    }

    pub fn iota(var: Variable, body: Expression) -> Self {
        Expression::Iota(var, Box::new(body))
    }

    pub fn not(e: Expression) -> Self {
        Expression::Negated(Box::new(e))
    }

    pub fn and(a: Expression, b: Expression) -> Self {
        Expression::And(Box::new(a), Box::new(b))
    }

    pub fn or(a: Expression, b: Expression) -> Self {
        Expression::Or(Box::new(a), Box::new(b))
    }

    pub fn imp(a: Expression, b: Expression) -> Self {
        Expression::Imp(Box::new(a), Box::new(b))
    }

    pub fn iff(a: Expression, b: Expression) -> Self {
        Expression::Iff(Box::new(a), Box::new(b))
    }

    pub fn eq(a: Expression, b: Expression) -> Self {
        Expression::Equality(Box::new(a), Box::new(b))
    }

    // ----- queries ---------------------------------------------------------

    /// Free variables (constants are not included). Mirrors NLTK
    /// `Expression.free()`.
    pub fn free(&self) -> BTreeSet<Variable> {
        let mut out = BTreeSet::new();
        self.collect_free(&BTreeSet::new(), &mut out);
        out
    }

    fn collect_free(&self, bound: &BTreeSet<Variable>, out: &mut BTreeSet<Variable>) {
        match self {
            Expression::Variable(v) => {
                if !bound.contains(v) {
                    out.insert(v.clone());
                }
            }
            Expression::Constant(_) => {}
            Expression::Application(f, x) => {
                f.collect_free(bound, out);
                x.collect_free(bound, out);
            }
            Expression::Negated(e) => e.collect_free(bound, out),
            Expression::And(a, b)
            | Expression::Or(a, b)
            | Expression::Imp(a, b)
            | Expression::Iff(a, b)
            | Expression::Equality(a, b) => {
                a.collect_free(bound, out);
                b.collect_free(bound, out);
            }
            Expression::Lambda(v, body)
            | Expression::Exists(v, body)
            | Expression::All(v, body)
            | Expression::Iota(v, body) => {
                let mut bound = bound.clone();
                bound.insert(v.clone());
                body.collect_free(&bound, out);
            }
        }
    }

    /// Constants used in this expression. Mirrors NLTK
    /// `Expression.constants()`.
    pub fn constants(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        self.collect_constants(&mut out);
        out
    }

    fn collect_constants(&self, out: &mut BTreeSet<String>) {
        match self {
            Expression::Constant(c) => {
                out.insert(c.clone());
            }
            Expression::Variable(_) => {}
            Expression::Application(f, x) => {
                f.collect_constants(out);
                x.collect_constants(out);
            }
            Expression::Negated(e) => e.collect_constants(out),
            Expression::And(a, b)
            | Expression::Or(a, b)
            | Expression::Imp(a, b)
            | Expression::Iff(a, b)
            | Expression::Equality(a, b) => {
                a.collect_constants(out);
                b.collect_constants(out);
            }
            Expression::Lambda(_, body)
            | Expression::Exists(_, body)
            | Expression::All(_, body)
            | Expression::Iota(_, body) => body.collect_constants(out),
        }
    }

    /// All variables occurring anywhere in the expression (free or bound).
    /// Mirrors NLTK `Expression.variables()`.
    pub fn variables(&self) -> BTreeSet<Variable> {
        let mut out = BTreeSet::new();
        self.collect_variables(&mut out);
        out
    }

    fn collect_variables(&self, out: &mut BTreeSet<Variable>) {
        match self {
            Expression::Variable(v) => {
                out.insert(v.clone());
            }
            Expression::Constant(_) => {}
            Expression::Application(f, x) => {
                f.collect_variables(out);
                x.collect_variables(out);
            }
            Expression::Negated(e) => e.collect_variables(out),
            Expression::And(a, b)
            | Expression::Or(a, b)
            | Expression::Imp(a, b)
            | Expression::Iff(a, b)
            | Expression::Equality(a, b) => {
                a.collect_variables(out);
                b.collect_variables(out);
            }
            Expression::Lambda(v, body)
            | Expression::Exists(v, body)
            | Expression::All(v, body)
            | Expression::Iota(v, body) => {
                out.insert(v.clone());
                body.collect_variables(out);
            }
        }
    }

    // ----- substitution ----------------------------------------------------

    /// Capture-avoiding substitution: replace free occurrences of `var`
    /// with `replacement`. Mirrors NLTK
    /// `Expression.replace(var, replacement, replace_bound=False)` for the
    /// `replace_bound=False` case (the standard one).
    pub fn replace(&self, var: &Variable, replacement: &Expression) -> Expression {
        self.replace_inner(var, replacement, &replacement.free())
    }

    fn replace_inner(
        &self,
        var: &Variable,
        replacement: &Expression,
        repl_free: &BTreeSet<Variable>,
    ) -> Expression {
        match self {
            Expression::Variable(v) if v == var => replacement.clone(),
            Expression::Variable(_) | Expression::Constant(_) => self.clone(),
            Expression::Application(f, x) => Expression::app(
                f.replace_inner(var, replacement, repl_free),
                x.replace_inner(var, replacement, repl_free),
            ),
            Expression::Negated(e) => Expression::not(e.replace_inner(var, replacement, repl_free)),
            Expression::And(a, b) => Expression::and(
                a.replace_inner(var, replacement, repl_free),
                b.replace_inner(var, replacement, repl_free),
            ),
            Expression::Or(a, b) => Expression::or(
                a.replace_inner(var, replacement, repl_free),
                b.replace_inner(var, replacement, repl_free),
            ),
            Expression::Imp(a, b) => Expression::imp(
                a.replace_inner(var, replacement, repl_free),
                b.replace_inner(var, replacement, repl_free),
            ),
            Expression::Iff(a, b) => Expression::iff(
                a.replace_inner(var, replacement, repl_free),
                b.replace_inner(var, replacement, repl_free),
            ),
            Expression::Equality(a, b) => Expression::eq(
                a.replace_inner(var, replacement, repl_free),
                b.replace_inner(var, replacement, repl_free),
            ),
            Expression::Lambda(bv, body) => {
                bind_replace(bv, body, var, replacement, repl_free, Expression::Lambda)
            }
            Expression::Exists(bv, body) => {
                bind_replace(bv, body, var, replacement, repl_free, Expression::Exists)
            }
            Expression::All(bv, body) => {
                bind_replace(bv, body, var, replacement, repl_free, Expression::All)
            }
            Expression::Iota(bv, body) => {
                bind_replace(bv, body, var, replacement, repl_free, Expression::Iota)
            }
        }
    }

    /// α-convert: rename a binder's variable. Walks the body with
    /// capture-avoiding substitution.
    pub fn alpha_convert(&self, new_var: Variable) -> Option<Expression> {
        match self {
            Expression::Lambda(v, body) => Some(Expression::lambda(
                new_var.clone(),
                body.replace(v, &Expression::Variable(new_var)),
            )),
            Expression::Exists(v, body) => Some(Expression::exists(
                new_var.clone(),
                body.replace(v, &Expression::Variable(new_var)),
            )),
            Expression::All(v, body) => Some(Expression::all(
                new_var.clone(),
                body.replace(v, &Expression::Variable(new_var)),
            )),
            Expression::Iota(v, body) => Some(Expression::iota(
                new_var.clone(),
                body.replace(v, &Expression::Variable(new_var)),
            )),
            _ => None,
        }
    }

    // ----- simplification --------------------------------------------------

    /// Recursively β-reduce: `(λx. M) N → M[x := N]`. Mirrors NLTK
    /// `ApplicationExpression.simplify` composed structurally.
    pub fn simplify(&self) -> Expression {
        match self {
            Expression::Application(f, x) => {
                let f_s = f.simplify();
                let x_s = x.simplify();
                if let Expression::Lambda(v, body) = &f_s {
                    body.replace(v, &x_s).simplify()
                } else {
                    Expression::app(f_s, x_s)
                }
            }
            Expression::Negated(e) => Expression::not(e.simplify()),
            Expression::And(a, b) => Expression::and(a.simplify(), b.simplify()),
            Expression::Or(a, b) => Expression::or(a.simplify(), b.simplify()),
            Expression::Imp(a, b) => Expression::imp(a.simplify(), b.simplify()),
            Expression::Iff(a, b) => Expression::iff(a.simplify(), b.simplify()),
            Expression::Equality(a, b) => Expression::eq(a.simplify(), b.simplify()),
            Expression::Lambda(v, body) => Expression::lambda(v.clone(), body.simplify()),
            Expression::Exists(v, body) => Expression::exists(v.clone(), body.simplify()),
            Expression::All(v, body) => Expression::all(v.clone(), body.simplify()),
            Expression::Iota(v, body) => Expression::iota(v.clone(), body.simplify()),
            Expression::Variable(_) | Expression::Constant(_) => self.clone(),
        }
    }
}

// Helper: alpha-rename binder if needed to avoid capturing `repl_free`,
// then recurse into the body.
fn bind_replace(
    bv: &Variable,
    body: &Expression,
    var: &Variable,
    replacement: &Expression,
    repl_free: &BTreeSet<Variable>,
    ctor: fn(Variable, Box<Expression>) -> Expression,
) -> Expression {
    // If the binder shadows the variable being substituted, no work to do
    // inside this scope.
    if bv == var {
        return ctor(bv.clone(), Box::new(body.clone()));
    }
    // If the binder name would capture a free variable of the
    // replacement, alpha-rename it first.
    let (binder, body_renamed) = if repl_free.contains(bv) {
        let mut used = repl_free.clone();
        used.extend(body.variables());
        used.insert(var.clone());
        used.insert(bv.clone());
        let fresh = unique_variable(Some(bv.name()), &used);
        let renamed_body = body.replace(bv, &Expression::Variable(fresh.clone()));
        (fresh, renamed_body)
    } else {
        (bv.clone(), body.clone())
    };
    ctor(
        binder,
        Box::new(body_renamed.replace_inner(var, replacement, repl_free)),
    )
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Variable(v) => write!(f, "{}", v),
            Expression::Constant(c) => write!(f, "{}", c),
            Expression::Application(func, arg) => write!(f, "{}({})", func, arg),
            Expression::Lambda(v, body) => write!(f, "\\{}.{}", v, body),
            Expression::Exists(v, body) => write!(f, "exists {}.{}", v, body),
            Expression::All(v, body) => write!(f, "all {}.{}", v, body),
            Expression::Iota(v, body) => write!(f, "iota {}.{}", v, body),
            Expression::Negated(e) => write!(f, "-{}", e),
            Expression::And(a, b) => write!(f, "({} & {})", a, b),
            Expression::Or(a, b) => write!(f, "({} | {})", a, b),
            Expression::Imp(a, b) => write!(f, "({} -> {})", a, b),
            Expression::Iff(a, b) => write!(f, "({} <-> {})", a, b),
            Expression::Equality(a, b) => write!(f, "({} = {})", a, b),
        }
    }
}

// ---------------------------------------------------------------------------
// Parser
// ---------------------------------------------------------------------------

/// A parse error from [`LogicParser`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicParseError {
    message: String,
    position: usize,
}

impl LogicParseError {
    pub fn new(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn position(&self) -> usize {
        self.position
    }
}

impl fmt::Display for LogicParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (at position {})", self.message, self.position)
    }
}

impl std::error::Error for LogicParseError {}

/// Recursive-descent parser for the NLTK logic language.
///
/// Supported syntax:
///
/// - Atoms: identifiers (`john`, `walks`, `x`, `P`, `e2`).
/// - Application: `P(x)`, `walks(john)`, multi-arg `loves(john, mary)`
///   desugars to `loves(john)(mary)` (left-associative curry).
/// - Lambda: `\x. body` or `\x.body`.
/// - Quantifiers: `exists x. body`, `all x. body`, `iota x. body`
///   (also `some`, `forall` accepted).
/// - Negation: `-body` or `not body` or `! body`.
/// - Boolean connectives: `&` / `and`, `|` / `or`, `->` / `=>` / `implies`,
///   `<->` / `<=>` / `iff`.
/// - Equality: `=` / `==`, inequality `!=` (lowered to `-(a = b)`).
/// - Parentheses: `( … )`.
pub struct LogicParser;

impl LogicParser {
    pub fn new() -> Self {
        LogicParser
    }

    pub fn parse(&self, input: &str) -> Result<Expression, LogicParseError> {
        let tokens = tokenize(input)?;
        let mut p = Parser {
            tokens: &tokens,
            pos: 0,
        };
        let expr = p.parse_expr(0)?;
        if p.pos < p.tokens.len() {
            return Err(LogicParseError::new(
                format!("unexpected trailing token {:?}", p.tokens[p.pos].kind),
                p.tokens[p.pos].position,
            ));
        }
        Ok(expr)
    }
}

impl Default for LogicParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience: parse with a fresh default [`LogicParser`].
pub fn parse(input: &str) -> Result<Expression, LogicParseError> {
    LogicParser::new().parse(input)
}

// ---- token types ----------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum TokKind {
    Ident(String),
    Lambda,
    Dot,
    Comma,
    LParen,
    RParen,
    Not,
    And,
    Or,
    Imp,
    Iff,
    Eq,
    Neq,
    Exists,
    All,
    Iota,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tok {
    kind: TokKind,
    position: usize,
}

// ---- tokenizer ------------------------------------------------------------

fn tokenize(input: &str) -> Result<Vec<Tok>, LogicParseError> {
    let bytes: Vec<char> = input.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        // Multi-char punctuation first.
        if c == '<'
            && bytes.get(i + 1).copied() == Some('-')
            && bytes.get(i + 2).copied() == Some('>')
        {
            out.push(Tok {
                kind: TokKind::Iff,
                position: i,
            });
            i += 3;
            continue;
        }
        if c == '<'
            && bytes.get(i + 1).copied() == Some('=')
            && bytes.get(i + 2).copied() == Some('>')
        {
            out.push(Tok {
                kind: TokKind::Iff,
                position: i,
            });
            i += 3;
            continue;
        }
        if c == '-' && bytes.get(i + 1).copied() == Some('>') {
            out.push(Tok {
                kind: TokKind::Imp,
                position: i,
            });
            i += 2;
            continue;
        }
        if c == '=' && bytes.get(i + 1).copied() == Some('>') {
            out.push(Tok {
                kind: TokKind::Imp,
                position: i,
            });
            i += 2;
            continue;
        }
        if c == '=' && bytes.get(i + 1).copied() == Some('=') {
            out.push(Tok {
                kind: TokKind::Eq,
                position: i,
            });
            i += 2;
            continue;
        }
        if c == '!' && bytes.get(i + 1).copied() == Some('=') {
            out.push(Tok {
                kind: TokKind::Neq,
                position: i,
            });
            i += 2;
            continue;
        }
        // Single-char punctuation.
        let single = match c {
            '\\' => Some(TokKind::Lambda),
            '.' => Some(TokKind::Dot),
            ',' => Some(TokKind::Comma),
            '(' => Some(TokKind::LParen),
            ')' => Some(TokKind::RParen),
            '-' | '!' => Some(TokKind::Not),
            '&' | '^' => Some(TokKind::And),
            '|' => Some(TokKind::Or),
            '=' => Some(TokKind::Eq),
            _ => None,
        };
        if let Some(kind) = single {
            out.push(Tok { kind, position: i });
            i += 1;
            continue;
        }
        // Identifier-like token.
        if is_ident_start(c) {
            let start = i;
            while i < bytes.len() && is_ident_continue(bytes[i]) {
                i += 1;
            }
            let text: String = bytes[start..i].iter().collect();
            let kind = match text.as_str() {
                "and" => TokKind::And,
                "or" => TokKind::Or,
                "not" => TokKind::Not,
                "implies" => TokKind::Imp,
                "iff" => TokKind::Iff,
                "exists" | "exist" | "some" => TokKind::Exists,
                "all" | "forall" => TokKind::All,
                "iota" => TokKind::Iota,
                _ => TokKind::Ident(text),
            };
            out.push(Tok {
                kind,
                position: start,
            });
            continue;
        }
        return Err(LogicParseError::new(
            format!("unexpected character {:?}", c),
            i,
        ));
    }
    Ok(out)
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

// ---- parser core ----------------------------------------------------------

struct Parser<'a> {
    tokens: &'a [Tok],
    pos: usize,
}

// Precedence table (higher = binds tighter):
//   iff  : 1
//   imp  : 2
//   or   : 3
//   and  : 4
//   eq/neq : 5
//   (negation, application: handled at unary/postfix layers)
const PREC_IFF: u8 = 1;
const PREC_IMP: u8 = 2;
const PREC_OR: u8 = 3;
const PREC_AND: u8 = 4;
const PREC_EQ: u8 = 5;

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<&Tok> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<&Tok> {
        let t = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(t)
    }

    fn err_eof<T>(&self, expected: &str) -> Result<T, LogicParseError> {
        let pos = self
            .tokens
            .last()
            .map(|t| t.position + 1)
            .unwrap_or_default();
        Err(LogicParseError::new(
            format!("expected {} but reached end of input", expected),
            pos,
        ))
    }

    fn expect(&mut self, kind: &TokKind, label: &str) -> Result<(), LogicParseError> {
        match self.peek() {
            Some(t) if &t.kind == kind => {
                self.pos += 1;
                Ok(())
            }
            Some(t) => Err(LogicParseError::new(
                format!("expected {} but got {:?}", label, t.kind),
                t.position,
            )),
            None => self.err_eof(label),
        }
    }

    /// Parse an expression at the given minimum precedence.
    fn parse_expr(&mut self, min_prec: u8) -> Result<Expression, LogicParseError> {
        let mut lhs = self.parse_unary()?;
        loop {
            let Some(t) = self.peek() else { break };
            let (prec, right_assoc) = match &t.kind {
                TokKind::Iff => (PREC_IFF, true),
                TokKind::Imp => (PREC_IMP, true),
                TokKind::Or => (PREC_OR, false),
                TokKind::And => (PREC_AND, false),
                TokKind::Eq | TokKind::Neq => (PREC_EQ, false),
                _ => break,
            };
            if prec < min_prec {
                break;
            }
            let op = self.bump().unwrap().kind.clone();
            let next_min = if right_assoc { prec } else { prec + 1 };
            let rhs = self.parse_expr(next_min)?;
            lhs = match op {
                TokKind::Iff => Expression::iff(lhs, rhs),
                TokKind::Imp => Expression::imp(lhs, rhs),
                TokKind::Or => Expression::or(lhs, rhs),
                TokKind::And => Expression::and(lhs, rhs),
                TokKind::Eq => Expression::eq(lhs, rhs),
                TokKind::Neq => Expression::not(Expression::eq(lhs, rhs)),
                _ => unreachable!(),
            };
        }
        Ok(lhs)
    }

    /// Parse negation, lambda, quantifiers, or fall through to application.
    fn parse_unary(&mut self) -> Result<Expression, LogicParseError> {
        let Some(t) = self.peek() else {
            return self.err_eof("expression");
        };
        match &t.kind {
            TokKind::Not => {
                self.bump();
                let body = self.parse_unary()?;
                Ok(Expression::not(body))
            }
            TokKind::Lambda => self.parse_binder(BinderKind::Lambda),
            TokKind::Exists => self.parse_binder(BinderKind::Exists),
            TokKind::All => self.parse_binder(BinderKind::All),
            TokKind::Iota => self.parse_binder(BinderKind::Iota),
            _ => self.parse_app(),
        }
    }

    fn parse_binder(&mut self, kind: BinderKind) -> Result<Expression, LogicParseError> {
        self.bump(); // consume binder keyword
                     // Allow `\x y. body` as sugar for `\x. \y. body`.
        let mut vars = Vec::new();
        loop {
            let Some(t) = self.peek() else {
                return self.err_eof("variable");
            };
            match &t.kind {
                TokKind::Ident(name) => {
                    let v = Variable::new(name.clone());
                    self.pos += 1;
                    vars.push(v);
                }
                TokKind::Dot => break,
                other => {
                    return Err(LogicParseError::new(
                        format!("expected variable or '.' but got {:?}", other),
                        t.position,
                    ));
                }
            }
        }
        if vars.is_empty() {
            return Err(LogicParseError::new(
                "binder requires at least one variable",
                self.tokens.get(self.pos).map(|t| t.position).unwrap_or(0),
            ));
        }
        self.expect(&TokKind::Dot, "'.'")?;
        let mut body = self.parse_expr(0)?;
        for v in vars.into_iter().rev() {
            body = match kind {
                BinderKind::Lambda => Expression::lambda(v, body),
                BinderKind::Exists => Expression::exists(v, body),
                BinderKind::All => Expression::all(v, body),
                BinderKind::Iota => Expression::iota(v, body),
            };
        }
        Ok(body)
    }

    /// Parse an atom and any trailing `(...)` application chains.
    fn parse_app(&mut self) -> Result<Expression, LogicParseError> {
        let mut head = self.parse_atom()?;
        while let Some(t) = self.peek() {
            if t.kind != TokKind::LParen {
                break;
            }
            self.bump(); // (
                         // empty arg list not allowed: `f()` rejected.
            let first = self.parse_expr(0)?;
            head = Expression::app(head, first);
            while let Some(t) = self.peek() {
                if t.kind == TokKind::Comma {
                    self.bump();
                    let next = self.parse_expr(0)?;
                    head = Expression::app(head, next);
                } else {
                    break;
                }
            }
            self.expect(&TokKind::RParen, "')'")?;
        }
        Ok(head)
    }

    fn parse_atom(&mut self) -> Result<Expression, LogicParseError> {
        let Some(t) = self.peek() else {
            return self.err_eof("atom");
        };
        match &t.kind {
            TokKind::LParen => {
                self.bump();
                let e = self.parse_expr(0)?;
                self.expect(&TokKind::RParen, "')'")?;
                Ok(e)
            }
            TokKind::Ident(name) => {
                let e = Expression::from_atom(name.clone());
                self.pos += 1;
                Ok(e)
            }
            other => Err(LogicParseError::new(
                format!("expected atom but got {:?}", other),
                t.position,
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum BinderKind {
    Lambda,
    Exists,
    All,
    Iota,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn p(s: &str) -> Expression {
        parse(s).unwrap_or_else(|e| panic!("parse {:?}: {}", s, e))
    }

    fn v(name: &str) -> Variable {
        Variable::new(name)
    }

    // --- variable conventions ----------------------------------------------

    #[test]
    fn variable_name_classification() {
        assert!(Variable::is_individual_name("x"));
        assert!(Variable::is_individual_name("z2"));
        assert!(!Variable::is_individual_name("a"));
        assert!(Variable::is_function_name("P"));
        assert!(Variable::is_function_name("F12"));
        assert!(!Variable::is_function_name("john"));
        assert!(Variable::is_event_name("e"));
        assert!(Variable::is_event_name("e3"));
        assert!(!Variable::is_event_name("ex"));
    }

    #[test]
    fn from_atom_classifies() {
        assert!(matches!(
            Expression::from_atom("x"),
            Expression::Variable(_)
        ));
        assert!(matches!(
            Expression::from_atom("P"),
            Expression::Variable(_)
        ));
        assert!(matches!(
            Expression::from_atom("john"),
            Expression::Constant(_)
        ));
        assert!(matches!(
            Expression::from_atom("walks"),
            Expression::Constant(_)
        ));
    }

    // --- parser: atoms & application ---------------------------------------

    #[test]
    fn parse_constant() {
        assert_eq!(p("john"), Expression::constant("john"));
    }

    #[test]
    fn parse_application() {
        assert_eq!(
            p("walks(john)"),
            Expression::app(Expression::constant("walks"), Expression::constant("john"))
        );
    }

    #[test]
    fn parse_multi_arg_application_left_curries() {
        // loves(john, mary)  ==  loves(john)(mary)
        let e = p("loves(john, mary)");
        let expected = Expression::app(
            Expression::app(Expression::constant("loves"), Expression::constant("john")),
            Expression::constant("mary"),
        );
        assert_eq!(e, expected);
    }

    #[test]
    fn parse_function_variable_application() {
        let e = p("P(x)");
        assert_eq!(
            e,
            Expression::app(Expression::var("P"), Expression::var("x"))
        );
    }

    // --- parser: connectives & precedence ----------------------------------

    #[test]
    fn parse_and_or_precedence() {
        // a & b | c   ==  (a & b) | c
        let e = p("walks(john) & walks(mary) | walks(sue)");
        match e {
            Expression::Or(lhs, _) => match *lhs {
                Expression::And(_, _) => {}
                other => panic!("expected and, got {:?}", other),
            },
            other => panic!("expected or, got {:?}", other),
        }
    }

    #[test]
    fn parse_imp_right_associative() {
        // a -> b -> c  ==  a -> (b -> c)
        let e = p("walks(a) -> walks(b) -> walks(c)");
        match e {
            Expression::Imp(_, rhs) => match *rhs {
                Expression::Imp(_, _) => {}
                other => panic!("expected imp, got {:?}", other),
            },
            other => panic!("expected imp, got {:?}", other),
        }
    }

    #[test]
    fn parse_negation() {
        let e = p("-walks(john)");
        assert_eq!(
            e,
            Expression::not(Expression::app(
                Expression::constant("walks"),
                Expression::constant("john")
            ))
        );
    }

    #[test]
    fn parse_keyword_negation() {
        assert_eq!(p("not walks(john)"), p("-walks(john)"));
    }

    #[test]
    fn parse_equality() {
        let e = p("john = mary");
        assert_eq!(
            e,
            Expression::eq(Expression::constant("john"), Expression::constant("mary"))
        );
    }

    #[test]
    fn parse_inequality_lowers_to_not_eq() {
        let e = p("john != mary");
        assert_eq!(
            e,
            Expression::not(Expression::eq(
                Expression::constant("john"),
                Expression::constant("mary")
            ))
        );
    }

    // --- parser: binders ---------------------------------------------------

    #[test]
    fn parse_lambda() {
        let e = p("\\x. walks(x)");
        assert_eq!(
            e,
            Expression::lambda(
                v("x"),
                Expression::app(Expression::constant("walks"), Expression::var("x"))
            )
        );
    }

    #[test]
    fn parse_lambda_multi_var_curries() {
        // \x y. P(x, y)
        let e = p("\\x y. P(x, y)");
        let expected = Expression::lambda(
            v("x"),
            Expression::lambda(
                v("y"),
                Expression::app(
                    Expression::app(Expression::var("P"), Expression::var("x")),
                    Expression::var("y"),
                ),
            ),
        );
        assert_eq!(e, expected);
    }

    #[test]
    fn parse_exists_and_all() {
        let e = p("exists x. walks(x)");
        assert!(matches!(e, Expression::Exists(_, _)));
        let e = p("all x. (walks(x) -> runs(x))");
        assert!(matches!(e, Expression::All(_, _)));
    }

    #[test]
    fn parse_some_synonym_for_exists() {
        assert_eq!(p("some x. walks(x)"), p("exists x. walks(x)"));
    }

    #[test]
    fn parse_forall_synonym_for_all() {
        assert_eq!(p("forall x. walks(x)"), p("all x. walks(x)"));
    }

    // --- free variables ----------------------------------------------------

    #[test]
    fn free_variables_simple() {
        let free = p("P(x)").free();
        assert_eq!(
            free.iter()
                .map(|v| v.name().to_string())
                .collect::<Vec<_>>(),
            vec!["P".to_string(), "x".to_string()]
        );
    }

    #[test]
    fn free_variables_excludes_bound() {
        let free = p("\\x. P(x, y)").free();
        let names: Vec<_> = free.iter().map(|v| v.name().to_string()).collect();
        assert!(!names.contains(&"x".to_string()));
        assert!(names.contains(&"y".to_string()));
        assert!(names.contains(&"P".to_string()));
    }

    #[test]
    fn constants_excludes_variables() {
        let cs = p("walks(john) & P(x)").constants();
        assert!(cs.contains("walks"));
        assert!(cs.contains("john"));
        assert!(!cs.contains("P"));
        assert!(!cs.contains("x"));
    }

    // --- substitution / alpha-conversion -----------------------------------

    #[test]
    fn replace_substitutes_free_var() {
        let e = p("P(x)");
        let result = e.replace(&v("x"), &Expression::constant("john"));
        assert_eq!(
            result,
            Expression::app(Expression::var("P"), Expression::constant("john"))
        );
    }

    #[test]
    fn replace_does_not_substitute_bound_var() {
        let e = p("\\x. P(x)");
        let result = e.replace(&v("x"), &Expression::constant("john"));
        assert_eq!(result, e); // x is bound, untouched
    }

    #[test]
    fn replace_alpha_renames_to_avoid_capture() {
        // (\x. P(x, y))[y := x]  must alpha-rename the bound x.
        let e = p("\\x. P(x, y)");
        let result = e.replace(&v("y"), &Expression::var("x"));
        // The result should NOT contain a Lambda binding `x` whose body
        // references the *substituted* free `x`. After alpha-rename, the
        // binder will be some fresh name; both occurrences inside should
        // refer to the new bound var.
        match result {
            Expression::Lambda(bv, body) => {
                assert_ne!(bv, v("x"), "expected alpha-rename of bound x");
                // The second arg of P should be the substituted x (free
                // outside the binder), not the rebound one.
                let free = body.free();
                assert!(
                    free.contains(&v("x")),
                    "free x should appear after substitution: {:?}",
                    free
                );
            }
            other => panic!("expected lambda, got {:?}", other),
        }
    }

    #[test]
    fn alpha_convert_renames_binder() {
        let e = p("\\x. P(x)");
        let renamed = e.alpha_convert(v("z2")).unwrap();
        assert_eq!(
            renamed,
            Expression::lambda(
                v("z2"),
                Expression::app(Expression::var("P"), Expression::var("z2"))
            )
        );
    }

    // --- beta reduction ----------------------------------------------------

    #[test]
    fn simplify_beta_reduces_application() {
        // (\x. walks(x))(john)  →  walks(john)
        let e = p("(\\x. walks(x))(john)");
        let simplified = e.simplify();
        assert_eq!(
            simplified,
            Expression::app(Expression::constant("walks"), Expression::constant("john"))
        );
    }

    #[test]
    fn simplify_nested_beta() {
        // (\x. \y. P(x, y))(john)(mary)  →  P(john, mary)
        let e = p("(\\x. \\y. P(x, y))(john)(mary)");
        let simplified = e.simplify();
        assert_eq!(
            simplified,
            Expression::app(
                Expression::app(Expression::var("P"), Expression::constant("john")),
                Expression::constant("mary")
            )
        );
    }

    #[test]
    fn simplify_avoids_capture() {
        // ((\x. \y. P(x, y))(y))(john)  must NOT capture the free y.
        // After β: (\y'. P(y, y'))(john)  →  P(y, john).
        let e = p("((\\x. \\y. P(x, y))(y))(john)");
        let simplified = e.simplify();
        match simplified {
            Expression::Application(f, arg) => {
                assert_eq!(*arg, Expression::constant("john"));
                match *f {
                    Expression::Application(p_expr, y_expr) => {
                        assert_eq!(*p_expr, Expression::var("P"));
                        // The original outer-y must remain free.
                        assert_eq!(*y_expr, Expression::var("y"));
                    }
                    other => panic!("expected P(y), got {:?}", other),
                }
            }
            other => panic!("expected application, got {:?}", other),
        }
    }

    #[test]
    fn simplify_descends_under_quantifier() {
        // exists x. (\y. walks(y))(x)  →  exists x. walks(x)
        let e = p("exists x. (\\y. walks(y))(x)");
        let simplified = e.simplify();
        assert_eq!(
            simplified,
            Expression::exists(
                v("x"),
                Expression::app(Expression::constant("walks"), Expression::var("x"))
            )
        );
    }

    // --- display roundtrip-ish ---------------------------------------------

    #[test]
    fn display_lambda() {
        let e = Expression::lambda(
            v("x"),
            Expression::app(Expression::constant("walks"), Expression::var("x")),
        );
        assert_eq!(format!("{}", e), "\\x.walks(x)");
    }

    #[test]
    fn display_quantified_implication() {
        let e = Expression::all(
            v("x"),
            Expression::imp(
                Expression::app(Expression::constant("walks"), Expression::var("x")),
                Expression::app(Expression::constant("runs"), Expression::var("x")),
            ),
        );
        assert_eq!(format!("{}", e), "all x.(walks(x) -> runs(x))");
    }
}
