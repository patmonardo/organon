//! First-order resolution-based theorem prover.
//!
//! Port of `nltk/inference/resolution.py`. A clausal/Robinson resolution
//! prover that satisfies the same [`crate::ml::nlp::inference::api::Prover`]
//! trait as [`crate::ml::nlp::inference::tableau::TableauProver`], so the two
//! provers can be swapped behind the same
//! [`crate::ml::nlp::inference::api::ProverCommand`] cache.
//!
//! ## Pipeline
//!
//! 1. Negate the goal and conjoin with the assumptions.
//! 2. [`clausify`] each: skolemize → CNF → list of [`Clause`]s →
//!    standardize-apart (rename free individual variables to fresh ones).
//! 3. Saturate by repeated pairwise resolution / demodulation
//!    ([`Clause::unify`]). The empty clause witnesses unsatisfiability,
//!    i.e. a successful refutation.
//!
//! ## Unification
//!
//! - **Resolution**: a positive literal `P(t̄)` and a negative literal
//!   `¬P(s̄)` are unified by [`most_general_unification`] (Robinson MGU)
//!   on individual variables.
//! - **Demodulation**: an `=`-literal `x = t` rewrites occurrences of
//!   `x` in the partner clause by binding `x ↦ t`.
//!
//! Subsumed clauses are removed after each resolution step
//! ([`Clause::subsumes`]).
//!
//! ## Notes vs. the Python original
//!
//! - Python's `IndividualVariableExpression` is split out at the parser
//!   level; we recover the same distinction with [`is_indvar_expr`],
//!   which gates both [`most_general_unification`] and demodulation.
//! - Python's `RuntimeError("maximum recursion depth exceeded")` escape
//!   hatch is replaced by an explicit [`ResolutionProver::max_steps`]
//!   bound on the saturation loop.

use std::collections::BTreeMap;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::ml::nlp::inference::api::{InferenceError, ProofResult, Prover};
use crate::ml::nlp::sem::logic::{Expression, Variable};
use crate::ml::nlp::sem::skolemize::skolemize;

// ---------------------------------------------------------------------------
// Helpers on Expression
// ---------------------------------------------------------------------------

/// True iff `e` is an [`Expression::Variable`] whose name follows the
/// NLTK individual-variable convention. Mirrors NLTK
/// `IndividualVariableExpression`.
pub fn is_indvar_expr(e: &Expression) -> bool {
    matches!(e, Expression::Variable(v) if Variable::is_individual_name(v.name()))
}

/// True iff `e` is a literal head — an [`Expression::Application`],
/// [`Expression::Variable`] (any kind), or [`Expression::Constant`].
/// These are the shapes the resolution prover treats as positive
/// atomic literals.
fn is_atom(e: &Expression) -> bool {
    matches!(
        e,
        Expression::Application(_, _) | Expression::Variable(_) | Expression::Constant(_)
    )
}

fn negate(e: &Expression) -> Expression {
    match e {
        Expression::Negated(inner) => (**inner).clone(),
        other => Expression::not(other.clone()),
    }
}

fn substitute_bindings(e: &Expression, b: &BindingDict) -> Expression {
    let mut cur = e.clone();
    for (var, expr) in b.iter() {
        cur = cur.replace(var, expr);
    }
    cur
}

// ---------------------------------------------------------------------------
// Binding dictionary + MGU
// ---------------------------------------------------------------------------

/// Failure of unification or of consistent variable binding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingError(pub String);

impl fmt::Display for BindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BindingError {}

/// Consistent variable bindings. Mirrors NLTK `BindingDict`.
///
/// `set` enforces that a variable already bound to value `e` may only
/// be re-bound to the same `e` (or its already-bound chain head). When
/// the new binding's RHS is itself an individual variable, the
/// dictionary tries to flip the binding to keep things consistent —
/// matching NLTK's "variable already bound, try the other direction"
/// move.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BindingDict {
    inner: BTreeMap<Variable, Expression>,
}

impl BindingDict {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pairs<I: IntoIterator<Item = (Variable, Expression)>>(
        pairs: I,
    ) -> Result<Self, BindingError> {
        let mut d = Self::new();
        for (v, e) in pairs {
            d.set(v, e)?;
        }
        Ok(d)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Variable, &Expression)> {
        self.inner.iter()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Look up a variable, following the chain of variable→variable
    /// bindings until the terminal expression is reached.
    pub fn get(&self, v: &Variable) -> Option<Expression> {
        let mut cur = self.inner.get(v)?.clone();
        loop {
            if let Expression::Variable(next) = &cur {
                if let Some(e2) = self.inner.get(next) {
                    cur = e2.clone();
                    continue;
                }
            }
            return Some(cur);
        }
    }

    pub fn set(&mut self, var: Variable, binding: Expression) -> Result<(), BindingError> {
        let existing = self.inner.get(&var).cloned();
        match existing {
            None => {
                self.inner.insert(var, binding);
                Ok(())
            }
            Some(e) if e == binding => Ok(()),
            Some(_) => {
                if let Expression::Variable(rhs_var) = &binding {
                    let other_existing = self.inner.get(rhs_var).cloned();
                    let flip = Expression::Variable(var.clone());
                    match other_existing {
                        None => {
                            self.inner.insert(rhs_var.clone(), flip);
                            Ok(())
                        }
                        Some(e2) if e2 == flip => Ok(()),
                        Some(_) => Err(BindingError(format!(
                            "Variable {} already bound to another value",
                            var
                        ))),
                    }
                } else {
                    Err(BindingError(format!(
                        "Variable {} already bound to another value",
                        var
                    )))
                }
            }
        }
    }

    /// Combine two binding dicts. Fails if they are inconsistent.
    pub fn combine(&self, other: &BindingDict) -> Result<BindingDict, BindingError> {
        let mut out = self.clone();
        for (v, e) in other.iter() {
            out.set(v.clone(), e.clone())?;
        }
        Ok(out)
    }
}

impl fmt::Display for BindingDict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self
            .inner
            .iter()
            .map(|(v, e)| format!("{}: {}", v, e))
            .collect();
        write!(f, "{{{}}}", parts.join(", "))
    }
}

/// Robinson most-general unification on individual variables. Mirrors
/// NLTK `most_general_unification`.
pub fn most_general_unification(
    a: &Expression,
    b: &Expression,
    bindings: &BindingDict,
) -> Result<BindingDict, BindingError> {
    if a == b {
        return Ok(bindings.clone());
    }
    if is_indvar_expr(a) {
        return mgu_var(a, b, bindings);
    }
    if is_indvar_expr(b) {
        return mgu_var(b, a, bindings);
    }
    if let (Expression::Application(af, ax), Expression::Application(bf, bx)) = (a, b) {
        let b1 = most_general_unification(af, bf, bindings)?;
        let b2 = most_general_unification(ax, bx, bindings)?;
        return b1.combine(&b2);
    }
    Err(BindingError(format!("'{}' cannot be bound to '{}'", a, b)))
}

fn mgu_var(
    var_expr: &Expression,
    expr: &Expression,
    bindings: &BindingDict,
) -> Result<BindingDict, BindingError> {
    let var = match var_expr {
        Expression::Variable(v) => v.clone(),
        _ => unreachable!(),
    };
    // Occurs check: var must not appear free or as constant in expr.
    let mut all = expr.free();
    for c in expr.constants() {
        all.insert(Variable::new(c));
    }
    if all.contains(&var) {
        return Err(BindingError(format!(
            "'{}' cannot be bound to '{}'",
            var_expr, expr
        )));
    }
    let mut new = BindingDict::new();
    new.set(var, expr.clone())?;
    new.combine(bindings)
}

// ---------------------------------------------------------------------------
// Clause
// ---------------------------------------------------------------------------

/// A disjunction of literals. Mirrors NLTK `Clause`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clause {
    pub atoms: Vec<Expression>,
}

impl Clause {
    pub fn new(atoms: Vec<Expression>) -> Self {
        Self { atoms }
    }

    pub fn empty() -> Self {
        Self { atoms: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.atoms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.atoms.is_empty()
    }

    /// `P` and `¬P` both present (ground match, no unification).
    pub fn is_tautology(&self) -> bool {
        for (i, a) in self.atoms.iter().enumerate() {
            for b in &self.atoms[i + 1..] {
                match (a, b) {
                    (Expression::Negated(t), other) if **t == *other => return true,
                    (other, Expression::Negated(t)) if **t == *other => return true,
                    _ => {}
                }
            }
        }
        false
    }

    pub fn replace(&self, var: &Variable, expr: &Expression) -> Clause {
        Clause::new(self.atoms.iter().map(|a| a.replace(var, expr)).collect())
    }

    pub fn substitute_bindings(&self, b: &BindingDict) -> Clause {
        Clause::new(
            self.atoms
                .iter()
                .map(|a| substitute_bindings(a, b))
                .collect(),
        )
    }

    /// Free individual variables across all literals.
    pub fn free(&self) -> std::collections::BTreeSet<Variable> {
        let mut out = std::collections::BTreeSet::new();
        for a in &self.atoms {
            for v in a.free() {
                out.insert(v);
            }
            for c in a.constants() {
                out.insert(Variable::new(c));
            }
        }
        out
    }

    pub fn is_subset_of(&self, other: &Clause) -> bool {
        self.atoms.iter().all(|a| other.atoms.contains(a))
    }

    /// True iff every literal in `self` can be unified with a literal
    /// in `other` (i.e. `self` is a stronger or equal claim).
    pub fn subsumes(&self, other: &Clause) -> bool {
        let negated_other: Vec<Expression> = other.atoms.iter().map(negate).collect();
        let neg_clause = Clause::new(negated_other);
        let mut results = Vec::new();
        iterate_first(
            &self.atoms,
            &neg_clause.atoms,
            &BindingDict::new(),
            (&[], &[]),
            (&[], &[]),
            &mut results,
            FinalizeMode::Subsumes,
        );
        !results.is_empty()
    }

    /// Try to resolve `self` and `other`. Returns the list of
    /// non-subsumed resulting clauses.
    pub fn unify(&self, other: &Clause) -> Vec<Clause> {
        let mut new_clauses: Vec<Clause> = Vec::new();
        let mut results = Vec::new();
        iterate_first(
            &self.atoms,
            &other.atoms,
            &BindingDict::new(),
            (&[], &[]),
            (&[], &[]),
            &mut results,
            FinalizeMode::Resolve,
        );
        for r in results {
            if let Some(c) = r {
                new_clauses.push(c);
            }
        }
        // Remove subsumed clauses.
        let mut keep: Vec<bool> = vec![true; new_clauses.len()];
        for i in 0..new_clauses.len() {
            if !keep[i] {
                continue;
            }
            for j in 0..new_clauses.len() {
                if i != j && keep[j] && new_clauses[i].subsumes(&new_clauses[j]) {
                    keep[j] = false;
                }
            }
        }
        new_clauses
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| if keep[i] { Some(c) } else { None })
            .collect()
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.atoms.iter().map(|a| a.to_string()).collect();
        write!(f, "{{{}}}", parts.join(", "))
    }
}

// ---------------------------------------------------------------------------
// Search through pairs of literals
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
enum FinalizeMode {
    Resolve,
    Subsumes,
}

type Atoms = Vec<Expression>;

fn iterate_first(
    first: &[Expression],
    second: &[Expression],
    bindings: &BindingDict,
    used: (&[Expression], &[Expression]),
    skipped: (&[Expression], &[Expression]),
    out: &mut Vec<Option<Clause>>,
    mode: FinalizeMode,
) {
    if first.is_empty() || second.is_empty() {
        finalize(first, second, bindings, used, skipped, out, mode);
        return;
    }

    // (a) explore: drill into 'second' for the current first[0]
    iterate_second(first, second, bindings, used, skipped, out, mode);

    // (b) skip: drop first[0] into the skipped bag, advance first
    let mut new_skipped_first: Atoms = skipped.0.to_vec();
    new_skipped_first.push(first[0].clone());
    iterate_first(
        &first[1..],
        second,
        bindings,
        used,
        (&new_skipped_first, skipped.1),
        out,
        mode,
    );

    // (c) attempt unification of first[0] and second[0]
    if let Ok((new_bindings, new_used, unused)) = unify_terms(&first[0], &second[0], bindings, used)
    {
        let mut newfirst: Atoms = first[1..].to_vec();
        newfirst.extend(skipped.0.iter().cloned());
        newfirst.extend(unused.0);
        let mut newsecond: Atoms = second[1..].to_vec();
        newsecond.extend(skipped.1.iter().cloned());
        newsecond.extend(unused.1);
        iterate_first(
            &newfirst,
            &newsecond,
            &new_bindings,
            (&new_used.0, &new_used.1),
            (&[], &[]),
            out,
            mode,
        );
    }
}

fn iterate_second(
    first: &[Expression],
    second: &[Expression],
    bindings: &BindingDict,
    used: (&[Expression], &[Expression]),
    skipped: (&[Expression], &[Expression]),
    out: &mut Vec<Option<Clause>>,
    mode: FinalizeMode,
) {
    if first.is_empty() || second.is_empty() {
        finalize(first, second, bindings, used, skipped, out, mode);
        return;
    }

    // (a) skip second[0]
    let mut new_skipped_second: Atoms = skipped.1.to_vec();
    new_skipped_second.push(second[0].clone());
    iterate_second(
        first,
        &second[1..],
        bindings,
        used,
        (skipped.0, &new_skipped_second),
        out,
        mode,
    );

    // (b) attempt unification
    if let Ok((new_bindings, new_used, unused)) = unify_terms(&first[0], &second[0], bindings, used)
    {
        let mut newfirst: Atoms = first[1..].to_vec();
        newfirst.extend(skipped.0.iter().cloned());
        newfirst.extend(unused.0);
        let mut newsecond: Atoms = second[1..].to_vec();
        newsecond.extend(skipped.1.iter().cloned());
        newsecond.extend(unused.1);
        iterate_first(
            &newfirst,
            &newsecond,
            &new_bindings,
            (&new_used.0, &new_used.1),
            (&[], &[]),
            out,
            mode,
        );
    }
}

fn finalize(
    first: &[Expression],
    second: &[Expression],
    bindings: &BindingDict,
    used: (&[Expression], &[Expression]),
    skipped: (&[Expression], &[Expression]),
    out: &mut Vec<Option<Clause>>,
    mode: FinalizeMode,
) {
    match mode {
        FinalizeMode::Resolve => {
            if !used.0.is_empty() || !used.1.is_empty() {
                let mut atoms: Atoms = skipped.0.to_vec();
                atoms.extend(skipped.1.iter().cloned());
                atoms.extend(first.iter().cloned());
                atoms.extend(second.iter().cloned());
                let new_clause = Clause::new(atoms).substitute_bindings(bindings);
                out.push(Some(new_clause));
            } else {
                out.push(None);
            }
        }
        FinalizeMode::Subsumes => {
            if skipped.0.is_empty() && first.is_empty() {
                out.push(Some(Clause::empty()));
            }
        }
    }
}

fn unify_terms(
    a: &Expression,
    b: &Expression,
    bindings: &BindingDict,
    used: (&[Expression], &[Expression]),
) -> Result<
    (
        BindingDict,
        (Vec<Expression>, Vec<Expression>),
        (Vec<Expression>, Vec<Expression>),
    ),
    BindingError,
> {
    // Resolution: negated literal vs positive atom (Application,
    // Variable, or Constant).
    if let Expression::Negated(at) = a {
        if is_atom(b) {
            let nb = most_general_unification(at, b, bindings)?;
            let mut nu0 = used.0.to_vec();
            nu0.push(a.clone());
            let mut nu1 = used.1.to_vec();
            nu1.push(b.clone());
            return Ok((nb, (nu0, nu1), (vec![], vec![])));
        }
    }
    if let Expression::Negated(bt) = b {
        if is_atom(a) {
            let nb = most_general_unification(a, bt, bindings)?;
            let mut nu0 = used.0.to_vec();
            nu0.push(a.clone());
            let mut nu1 = used.1.to_vec();
            nu1.push(b.clone());
            return Ok((nb, (nu0, nu1), (vec![], vec![])));
        }
    }
    // Demodulation
    if let Expression::Equality(lhs, rhs) = a {
        if let Expression::Variable(v) = lhs.as_ref() {
            let mut nb = BindingDict::new();
            nb.set(v.clone(), (**rhs).clone())?;
            let nb = nb.combine(bindings)?;
            let mut nu0 = used.0.to_vec();
            nu0.push(a.clone());
            return Ok((nb, (nu0, used.1.to_vec()), (vec![], vec![b.clone()])));
        }
    }
    if let Expression::Equality(lhs, rhs) = b {
        if let Expression::Variable(v) = lhs.as_ref() {
            let mut nb = BindingDict::new();
            nb.set(v.clone(), (**rhs).clone())?;
            let nb = nb.combine(bindings)?;
            let mut nu1 = used.1.to_vec();
            nu1.push(b.clone());
            return Ok((nb, (used.0.to_vec(), nu1), (vec![a.clone()], vec![])));
        }
    }
    Err(BindingError(format!("'{}' and '{}' do not resolve", a, b)))
}

// ---------------------------------------------------------------------------
// Clausify
// ---------------------------------------------------------------------------

static CLAUSIFY_VAR_COUNTER: AtomicU64 = AtomicU64::new(2);

fn fresh_clausify_var() -> Variable {
    let n = CLAUSIFY_VAR_COUNTER.fetch_add(1, Ordering::SeqCst);
    Variable::new(format!("z{}", n))
}

/// Skolemize, convert to CNF, split into clauses, and standardize the
/// free individual variables of each clause apart. Mirrors NLTK
/// `clausify`.
pub fn clausify(expression: &Expression) -> Result<Vec<Clause>, ClausifyError> {
    let s = skolemize(expression);
    let mut out = Vec::new();
    for clause in clausify_cnf(&s)? {
        let mut c = clause;
        let frees: Vec<Variable> = c
            .free()
            .into_iter()
            .filter(|v| Variable::is_individual_name(v.name()))
            .collect();
        for f in frees {
            let nv = fresh_clausify_var();
            c = c.replace(&f, &Expression::Variable(nv));
        }
        out.push(c);
    }
    Ok(out)
}

#[derive(Debug, Clone)]
pub struct ClausifyError(pub String);

impl fmt::Display for ClausifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ClausifyError {}

fn clausify_cnf(expr: &Expression) -> Result<Vec<Clause>, ClausifyError> {
    match expr {
        Expression::And(a, b) => {
            let mut out = clausify_cnf(a)?;
            out.extend(clausify_cnf(b)?);
            Ok(out)
        }
        Expression::Or(a, b) => {
            let cl_a = clausify_cnf(a)?;
            let cl_b = clausify_cnf(b)?;
            if cl_a.len() != 1 || cl_b.len() != 1 {
                return Err(ClausifyError(format!(
                    "expression not in CNF (disjunction over conjunction): {}",
                    expr
                )));
            }
            let mut atoms = cl_a.into_iter().next().unwrap().atoms;
            atoms.extend(cl_b.into_iter().next().unwrap().atoms);
            Ok(vec![Clause::new(atoms)])
        }
        Expression::Equality(_, _)
        | Expression::Application(_, _)
        | Expression::Variable(_)
        | Expression::Constant(_) => Ok(vec![Clause::new(vec![expr.clone()])]),
        Expression::Negated(inner) => match inner.as_ref() {
            Expression::Application(_, _)
            | Expression::Equality(_, _)
            | Expression::Variable(_)
            | Expression::Constant(_) => Ok(vec![Clause::new(vec![expr.clone()])]),
            _ => Err(ClausifyError(format!(
                "negation of non-atom in CNF: {}",
                expr
            ))),
        },
        _ => Err(ClausifyError(format!(
            "unexpected node in CNF tree: {}",
            expr
        ))),
    }
}

// ---------------------------------------------------------------------------
// ResolutionProver
// ---------------------------------------------------------------------------

/// Resolution prover. Mirrors NLTK `ResolutionProver`.
#[derive(Debug, Clone)]
pub struct ResolutionProver {
    /// Hard cap on the number of resolution steps before giving up.
    /// Replaces NLTK's reliance on Python recursion-depth exhaustion.
    pub max_steps: usize,
}

impl Default for ResolutionProver {
    fn default() -> Self {
        Self { max_steps: 50_000 }
    }
}

impl ResolutionProver {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_steps(max_steps: usize) -> Self {
        Self { max_steps }
    }

    /// Run resolution and return both the boolean result and the final
    /// clause list (useful for tracing / tests).
    pub fn prove_with_clauses(
        &self,
        goal: Option<&Expression>,
        assumptions: &[Expression],
    ) -> Result<(bool, Vec<Clause>), InferenceError> {
        let mut clauses: Vec<Clause> = Vec::new();
        if let Some(g) = goal {
            let neg = negate(g);
            for c in clausify(&neg).map_err(|e| InferenceError(e.0))? {
                clauses.push(c);
            }
        }
        for a in assumptions {
            for c in clausify(a).map_err(|e| InferenceError(e.0))? {
                clauses.push(c);
            }
        }
        let proved = self.attempt_proof(&mut clauses)?;
        Ok((proved, clauses))
    }

    fn attempt_proof(&self, clauses: &mut Vec<Clause>) -> Result<bool, InferenceError> {
        // tried[i] = list of j's already attempted against clause i
        let mut tried: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        let mut steps: usize = 0;
        let mut i: usize = 0;
        while i < clauses.len() {
            if steps > self.max_steps {
                return Err(InferenceError(format!(
                    "resolution exceeded max_steps ({})",
                    self.max_steps
                )));
            }
            steps += 1;
            if !clauses[i].is_tautology() {
                let mut j = match tried.get(&i).and_then(|v| v.last().copied()) {
                    Some(last) => last + 1,
                    None => i + 1,
                };
                let mut restarted = false;
                while j < clauses.len() {
                    if i != j && j > 0 && !clauses[j].is_tautology() {
                        tried.entry(i).or_default().push(j);
                        let new_clauses = clauses[i].unify(&clauses[j]);
                        if !new_clauses.is_empty() {
                            for nc in new_clauses {
                                let empty_found = nc.is_empty();
                                clauses.push(nc);
                                if empty_found {
                                    return Ok(true);
                                }
                            }
                            // restart from the top.
                            i = 0;
                            restarted = true;
                            break;
                        }
                    }
                    j += 1;
                }
                if restarted {
                    continue;
                }
            }
            i += 1;
        }
        Ok(false)
    }
}

impl Prover for ResolutionProver {
    fn prove(
        &self,
        goal: Option<&Expression>,
        assumptions: &[Expression],
        verbose: bool,
    ) -> Result<ProofResult, InferenceError> {
        let (proved, clauses) = self.prove_with_clauses(goal, assumptions)?;
        let proof = if verbose {
            decorate_clauses(&clauses)
        } else {
            String::new()
        };
        Ok(ProofResult { proved, proof })
    }
}

fn decorate_clauses(clauses: &[Clause]) -> String {
    let mut out = String::new();
    let width = clauses.len().to_string().len();
    for (i, c) in clauses.iter().enumerate() {
        let taut = if c.is_tautology() { " Tautology" } else { "" };
        out.push_str(&format!("[{:>w$}] {}{}\n", i + 1, c, taut, w = width));
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::nlp::sem::logic::parse;

    fn parsed(s: &str) -> Expression {
        parse(s).unwrap()
    }

    fn prove_str(goal: &str, assumptions: &[&str]) -> bool {
        let g = parsed(goal);
        let asx: Vec<Expression> = assumptions.iter().map(|s| parsed(s)).collect();
        let p = ResolutionProver::new();
        let r = p.prove(Some(&g), &asx, false).unwrap();
        r.proved
    }

    fn valid(s: &str) -> bool {
        let g = parsed(s);
        let p = ResolutionProver::new();
        let r = p.prove(Some(&g), &[], false).unwrap();
        r.proved
    }

    // ---- propositional tautologies ----

    #[test]
    fn prop_self_implication() {
        assert!(valid("(P -> P)"));
    }

    #[test]
    fn prop_excluded_middle() {
        assert!(valid("(P | -P)"));
    }

    #[test]
    fn prop_double_negation() {
        assert!(valid("(P -> --P)"));
    }

    #[test]
    fn prop_contradiction_negated() {
        assert!(valid("-(P & -P)"));
    }

    #[test]
    fn prop_iff_self() {
        assert!(valid("(P <-> P)"));
    }

    #[test]
    fn prop_modus_ponens() {
        assert!(prove_str("Q", &["(P -> Q)", "P"]));
    }

    #[test]
    fn prop_modus_tollens() {
        assert!(prove_str("-P", &["(P -> Q)", "-Q"]));
    }

    #[test]
    fn prop_unproved_disjunction() {
        // P alone does not prove Q.
        assert!(!prove_str("Q", &["P"]));
    }

    // ---- first-order ----

    #[test]
    fn fol_socrates_is_mortal() {
        // ∀x. (man(x) → mortal(x)),  man(Socrates)  ⊢  mortal(Socrates)
        assert!(prove_str(
            "mortal(Socrates)",
            &["all x.(man(x) -> mortal(x))", "man(Socrates)"],
        ));
    }

    #[test]
    fn fol_existential_witness() {
        // ∀x.(man(x) → walks(x)),  man(John)  ⊢  ∃y. walks(y)
        assert!(prove_str(
            "some y. walks(y)",
            &["all x.(man(x) -> walks(x))", "man(John)"],
        ));
    }

    #[test]
    fn fol_universal_alone_does_not_prove_instance() {
        // ∀x.man(x)  on its own is satisfiable but doesn't prove
        // arbitrary unrelated atoms.
        assert!(!prove_str("dog(Fido)", &["all x. man(x)"]));
    }

    #[test]
    fn fol_quantifier_swap_invalid() {
        // ∃x.∀y. F(x,y)  does NOT follow from  ∀y.∃x. F(x,y)
        // (and as a refutation: their conjunction is satisfiable).
        // We test the contrapositive: not a tautology.
        let p = ResolutionProver::with_max_steps(2_000);
        let g = parsed("(all y. some x. F(x, y) -> some x. all y. F(x, y))");
        let r = p.prove(Some(&g), &[], false).unwrap();
        assert!(!r.proved);
    }

    // ---- clausify shape ----

    #[test]
    fn clausify_atom_one_clause() {
        let cs = clausify(&parsed("man(adam)")).unwrap();
        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0].len(), 1);
    }

    #[test]
    fn clausify_disjunction_one_clause_two_literals() {
        let cs = clausify(&parsed("(P(x) | Q(x))")).unwrap();
        assert_eq!(cs.len(), 1);
        assert_eq!(cs[0].len(), 2);
    }

    #[test]
    fn clausify_conjunction_two_clauses() {
        let cs = clausify(&parsed("(P(a) & Q(a))")).unwrap();
        assert_eq!(cs.len(), 2);
    }

    #[test]
    fn clausify_or_over_and_distributes() {
        // ((P(a) & Q(a)) | R(a))  →  {P(a), R(a)}, {Q(a), R(a)}
        let cs = clausify(&parsed("((P(a) & Q(a)) | R(a))")).unwrap();
        assert_eq!(cs.len(), 2);
        for c in &cs {
            assert_eq!(c.len(), 2);
        }
    }

    // ---- mgu / bindings ----

    #[test]
    fn mgu_var_to_constant() {
        let a = parsed("x");
        let b = parsed("adam");
        let res = most_general_unification(&a, &b, &BindingDict::new()).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res.get(&Variable::new("x")), Some(parsed("adam")));
    }

    #[test]
    fn mgu_application() {
        let a = parsed("man(x)");
        let b = parsed("man(adam)");
        let res = most_general_unification(&a, &b, &BindingDict::new()).unwrap();
        assert_eq!(res.get(&Variable::new("x")), Some(parsed("adam")));
    }

    #[test]
    fn mgu_occurs_check_fails() {
        // x cannot be unified with f(x) — but we don't have a function
        // arity facility here; use an application that contains x.
        let a = parsed("x");
        let b = parsed("man(x)");
        let err = most_general_unification(&a, &b, &BindingDict::new()).unwrap_err();
        assert!(err.0.contains("cannot be bound"));
    }

    #[test]
    fn mgu_constants_disagree() {
        let a = parsed("adam");
        let b = parsed("betty");
        let err = most_general_unification(&a, &b, &BindingDict::new()).unwrap_err();
        assert!(err.0.contains("cannot be bound"));
    }

    // ---- clause facilities ----

    #[test]
    fn clause_tautology_detected() {
        let c = Clause::new(vec![parsed("P(a)"), parsed("-P(a)")]);
        assert!(c.is_tautology());
    }

    #[test]
    fn clause_subsumes_self() {
        let c = Clause::new(vec![parsed("P(a)")]);
        assert!(c.subsumes(&c.clone()));
    }
}
