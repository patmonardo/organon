//! First-order semantic-tableau prover.
//!
//! Native Rust port of `nltk/inference/tableau.py`. Self-contained — no
//! external solver dependency.
//!
//! ## Algorithm sketch
//!
//! Negate the goal, push it together with the assumptions onto a
//! priority-categorized agenda. Repeatedly:
//!
//! 1. Pop the highest-priority entry.
//! 2. Apply its rule (alpha = no branching, beta = branching).
//! 3. Close a branch when both `A` and `¬A` are present (literal clash)
//!    or when an inequality `¬(t = t)` appears.
//! 4. The proof succeeds iff every branch closes.
//!
//! Quantifier rules:
//!
//! - `∃x. P(x)` → instantiate with a fresh constant.
//! - `∀x. P(x)` → instantiate against each accessible variable on the
//!   branch; re-queue with a `used_vars` memo to avoid re-using the same
//!   witness.
//!
//! Equality `s = t` (with `s` a variable) is a substitution — rewrite
//! the entire agenda mapping `s ↦ t` and clear the atom store (since
//! literals must be re-checked after the rewrite).
//!
//! ## Differences from NLTK
//!
//! - NLTK's higher-order `APP`/`N_APP` lambda-context manipulation is
//!   not ported. Our atoms are first-order (variables / constants /
//!   applications-of-symbol-to-atom-args).
//! - We use a compile-time `max_depth` guard instead of Python's
//!   `RuntimeError: maximum recursion depth exceeded`.

use crate::ml::nlp::inference::api::{InferenceError, ProofResult, Prover};
use crate::ml::nlp::sem::logic::{Expression, Variable};
use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

// ---------------------------------------------------------------------------
// Public prover
// ---------------------------------------------------------------------------

/// Configuration for the tableau prover.
#[derive(Debug, Clone)]
pub struct TableauProver {
    /// Maximum number of inference steps before giving up. Returning
    /// `false` on exceedance matches NLTK's `_assume_false` behavior on
    /// recursion-depth exhaustion.
    pub max_steps: u64,
    /// Maximum number of distinct fresh witnesses introduced per ∀ on a
    /// single branch. Bounds the Herbrand expansion.
    pub max_universal_instantiations: usize,
}

impl Default for TableauProver {
    fn default() -> Self {
        Self {
            max_steps: 10_000,
            max_universal_instantiations: 8,
        }
    }
}

impl TableauProver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Prover for TableauProver {
    fn prove(
        &self,
        goal: Option<&Expression>,
        assumptions: &[Expression],
        verbose: bool,
    ) -> Result<ProofResult, InferenceError> {
        let mut agenda = Agenda::new();
        if let Some(g) = goal {
            agenda.put(negate(g.clone()));
        }
        for a in assumptions {
            agenda.put(a.clone());
        }
        let mut debug = Debug::new(verbose);
        let mut steps: u64 = 0;
        let proved = attempt_proof(
            self,
            &mut agenda,
            BTreeSet::new(),
            BTreeSet::new(),
            &mut debug,
            0,
            &mut steps,
        )?;
        Ok(ProofResult {
            proved,
            proof: debug.lines.join("\n"),
        })
    }
}

// ---------------------------------------------------------------------------
// Categories
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    Atom,
    NAtom,
    NEq,
    DNeg,
    NAll,
    NExists,
    And,
    NOr,
    NImp,
    Or,
    Imp,
    NAnd,
    Iff,
    NIff,
    Eq,
    Exists,
    All,
}

const PRIORITY_ORDER: &[Category] = &[
    Category::Atom,
    Category::NAtom,
    Category::NEq,
    Category::DNeg,
    Category::NAll,
    Category::NExists,
    Category::And,
    Category::NOr,
    Category::NImp,
    Category::Or,
    Category::Imp,
    Category::NAnd,
    Category::Iff,
    Category::NIff,
    Category::Eq,
    Category::Exists,
    Category::All,
];

fn categorize(e: &Expression) -> Category {
    if let Expression::Negated(inner) = e {
        return categorize_negated(inner);
    }
    if is_atom(e) {
        return Category::Atom;
    }
    match e {
        Expression::All(_, _) => Category::All,
        Expression::Exists(_, _) => Category::Exists,
        Expression::And(_, _) => Category::And,
        Expression::Or(_, _) => Category::Or,
        Expression::Imp(_, _) => Category::Imp,
        Expression::Iff(_, _) => Category::Iff,
        Expression::Equality(_, _) => Category::Eq,
        // Iota and Lambda have no native tableau rule; treat as
        // un-decomposable atoms (they will sit forever; harmless unless
        // their negation also appears).
        _ => Category::Atom,
    }
}

fn categorize_negated(inner: &Expression) -> Category {
    if matches!(inner, Expression::Negated(_)) {
        return Category::DNeg;
    }
    if is_atom(inner) {
        return Category::NAtom;
    }
    match inner {
        Expression::All(_, _) => Category::NAll,
        Expression::Exists(_, _) => Category::NExists,
        Expression::And(_, _) => Category::NAnd,
        Expression::Or(_, _) => Category::NOr,
        Expression::Imp(_, _) => Category::NImp,
        Expression::Iff(_, _) => Category::NIff,
        Expression::Equality(_, _) => Category::NEq,
        _ => Category::NAtom,
    }
}

// ---------------------------------------------------------------------------
// Helpers on Expression (kept local; not added to logic.rs to avoid surface
// expansion before the type system lands)
// ---------------------------------------------------------------------------

fn negate(e: Expression) -> Expression {
    if let Expression::Negated(inner) = e {
        *inner
    } else {
        Expression::not(e)
    }
}

fn is_atom(e: &Expression) -> bool {
    match e {
        Expression::Variable(_) | Expression::Constant(_) => true,
        Expression::Application(f, x) => is_atom(f) && is_atom(x),
        _ => false,
    }
}

/// Collect free terms that appear as arguments inside an atom — used to
/// seed the accessible-vars set for ∀ instantiation.
fn atom_args(e: &Expression, out: &mut BTreeSet<Expression>) {
    match e {
        Expression::Application(f, x) => {
            atom_args(f, out);
            // Argument itself becomes an accessible term if it's atomic.
            if is_atom(x) {
                out.insert((**x).clone());
            } else {
                atom_args(x, out);
            }
        }
        Expression::Variable(_) | Expression::Constant(_) => {
            out.insert(e.clone());
        }
        _ => {}
    }
}

/// Substitute every free occurrence of `var` with `replacement` in `e`.
/// Wraps `Expression::replace`.
fn subst(e: &Expression, var: &Variable, replacement: &Expression) -> Expression {
    e.replace(var, replacement)
}

// ---------------------------------------------------------------------------
// Agenda
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
struct AllEntry {
    expr: Expression,
    used_vars: BTreeSet<Expression>,
    exhausted: bool,
}

#[derive(Debug, Clone, Default)]
struct Agenda {
    /// Plain entries keyed by category, except `All` and `NEq` which
    /// need extra per-entry state.
    plain: std::collections::BTreeMap<CategoryKey, Vec<Expression>>,
    alls: Vec<AllEntry>,
    /// `¬(s = t)` entries that have already been observed and added to
    /// `accessible_vars`; kept on the agenda only so that an `Eq` rule
    /// can re-trigger checks if the agenda is rewritten.
    n_eqs: Vec<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CategoryKey(usize);

fn key_for(c: Category) -> CategoryKey {
    let idx = PRIORITY_ORDER.iter().position(|x| *x == c).unwrap();
    CategoryKey(idx)
}

impl Agenda {
    fn new() -> Self {
        Self::default()
    }

    fn put(&mut self, e: Expression) {
        let cat = categorize(&e);
        match cat {
            Category::All => self.alls.push(AllEntry {
                expr: e,
                used_vars: BTreeSet::new(),
                exhausted: false,
            }),
            Category::NEq => {
                self.plain
                    .entry(key_for(Category::NEq))
                    .or_default()
                    .push(e);
            }
            other => {
                self.plain.entry(key_for(other)).or_default().push(e);
            }
        }
    }

    /// Pop the next highest-priority entry. Returns `None` if the
    /// agenda holds nothing actionable (only exhausted ∀ remain).
    fn pop(&mut self) -> Option<Popped> {
        for c in PRIORITY_ORDER {
            if *c == Category::All {
                // pick a non-exhausted ∀
                if let Some(idx) = self.alls.iter().position(|e| !e.exhausted) {
                    let entry = self.alls.remove(idx);
                    return Some(Popped::All(entry));
                }
                continue;
            }
            let key = key_for(*c);
            if let Some(vec) = self.plain.get_mut(&key) {
                if let Some(e) = vec.pop() {
                    return Some(Popped::Plain(*c, e));
                }
            }
        }
        None
    }

    fn mark_alls_fresh(&mut self) {
        for a in &mut self.alls {
            a.exhausted = false;
        }
    }

    /// Rewrite every entry replacing `var` with `replacement`. Used by
    /// the equality rule.
    fn rewrite(&mut self, var: &Variable, replacement: &Expression) {
        for vec in self.plain.values_mut() {
            for e in vec.iter_mut() {
                *e = subst(e, var, replacement);
            }
        }
        for a in &mut self.alls {
            a.expr = subst(&a.expr, var, replacement);
            a.exhausted = false;
            // used_vars are concrete witnesses — also rewrite them
            let new_used: BTreeSet<Expression> = a
                .used_vars
                .iter()
                .map(|w| subst(w, var, replacement))
                .collect();
            a.used_vars = new_used;
        }
        for e in &mut self.n_eqs {
            *e = subst(e, var, replacement);
        }
    }

    /// Re-add literals from the atom store after an equality rewrite,
    /// so that newly-clashing pairs can fire `Atom`/`NAtom` closure.
    fn put_atoms(&mut self, atoms: &BTreeSet<(Expression, bool)>) {
        for (atom, neg) in atoms {
            if *neg {
                self.put(Expression::not(atom.clone()));
            } else {
                self.put(atom.clone());
            }
        }
    }

    /// Drain the deferred `¬(s = t)` bag back into the active agenda so
    /// that an equality rewrite can re-check inequalities for closure.
    fn requeue_n_eqs(&mut self) {
        let drained = std::mem::take(&mut self.n_eqs);
        for e in drained {
            self.put(e);
        }
    }
}

enum Popped {
    Plain(Category, Expression),
    All(AllEntry),
}

// ---------------------------------------------------------------------------
// Debug trace
// ---------------------------------------------------------------------------

struct Debug {
    verbose: bool,
    lines: Vec<String>,
}

impl Debug {
    fn new(verbose: bool) -> Self {
        Self {
            verbose,
            lines: Vec::new(),
        }
    }

    fn line(&mut self, indent: usize, msg: impl AsRef<str>) {
        let prefix = "  ".repeat(indent);
        let line = format!("{}{}", prefix, msg.as_ref());
        if self.verbose {
            eprintln!("{}", line);
        }
        self.lines.push(line);
    }
}

// ---------------------------------------------------------------------------
// Fresh variable generator
// ---------------------------------------------------------------------------

static SKOLEM_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Allocate a fresh witness term. We use NLTK's individual-variable
/// naming convention (`z` + integer ≥ 2) so that the resulting term is
/// recognised as a `Variable` by [`Expression::replace`] — necessary
/// for the equality-rewrite rule to fire when both sides are fresh
/// witnesses.
fn fresh_skolem() -> Expression {
    let n = SKOLEM_COUNTER.fetch_add(1, Ordering::Relaxed) + 2;
    Expression::Variable(Variable::new(format!("z{}", n)))
}

// ---------------------------------------------------------------------------
// Core proof loop
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn attempt_proof(
    cfg: &TableauProver,
    agenda: &mut Agenda,
    mut accessible_vars: BTreeSet<Expression>,
    mut atoms: BTreeSet<(Expression, bool)>,
    debug: &mut Debug,
    indent: usize,
    steps: &mut u64,
) -> Result<bool, InferenceError> {
    *steps += 1;
    if *steps > cfg.max_steps {
        debug.line(indent, "STEP LIMIT");
        return Ok(false);
    }

    let popped = match agenda.pop() {
        Some(p) => p,
        None => {
            debug.line(indent, "AGENDA EMPTY");
            return Ok(false);
        }
    };

    match popped {
        Popped::Plain(cat, current) => {
            debug.line(indent, format!("[{:?}] {}", cat, current));
            handle_plain(
                cfg,
                cat,
                current,
                agenda,
                &mut accessible_vars,
                &mut atoms,
                debug,
                indent,
                steps,
            )
        }
        Popped::All(entry) => {
            debug.line(indent, format!("[All] {}", entry.expr));
            handle_all(
                cfg,
                entry,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_plain(
    cfg: &TableauProver,
    cat: Category,
    current: Expression,
    agenda: &mut Agenda,
    accessible_vars: &mut BTreeSet<Expression>,
    atoms: &mut BTreeSet<(Expression, bool)>,
    debug: &mut Debug,
    indent: usize,
    steps: &mut u64,
) -> Result<bool, InferenceError> {
    match cat {
        // ---- atom literal closure ----
        Category::Atom => {
            // Already-stored as negative? → CLOSED
            if atoms.contains(&(current.clone(), true)) {
                debug.line(indent + 1, "CLOSED");
                return Ok(true);
            }
            atom_args(&current, accessible_vars);
            atoms.insert((current, false));
            agenda.mark_alls_fresh();
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }
        Category::NAtom => {
            let inner = match &current {
                Expression::Negated(t) => (**t).clone(),
                _ => unreachable!(),
            };
            if atoms.contains(&(inner.clone(), false)) {
                debug.line(indent + 1, "CLOSED");
                return Ok(true);
            }
            atom_args(&inner, accessible_vars);
            atoms.insert((inner, true));
            agenda.mark_alls_fresh();
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- ¬(s = t) ----
        Category::NEq => {
            // ¬(t = t) closes the branch.
            let (s, t) = match &current {
                Expression::Negated(inner) => match inner.as_ref() {
                    Expression::Equality(a, b) => ((**a).clone(), (**b).clone()),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            if s == t {
                debug.line(indent + 1, "CLOSED (a≠a)");
                return Ok(true);
            }
            accessible_vars.insert(s);
            accessible_vars.insert(t);
            agenda.n_eqs.push(current);
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- ¬¬A → A ----
        Category::DNeg => {
            let inner = match current {
                Expression::Negated(t) => match *t {
                    Expression::Negated(tt) => *tt,
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            agenda.put(inner);
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- ¬∀x.P → ∃x.¬P ----
        Category::NAll => {
            let (var, body) = match current {
                Expression::Negated(t) => match *t {
                    Expression::All(v, b) => (v, *b),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            agenda.put(Expression::exists(var, Expression::not(body)));
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- ¬∃x.P → ∀x.¬P ----
        Category::NExists => {
            let (var, body) = match current {
                Expression::Negated(t) => match *t {
                    Expression::Exists(v, b) => (v, *b),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            agenda.put(Expression::all(var, Expression::not(body)));
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- alpha rules (no branching) ----
        Category::And => {
            let (a, b) = match current {
                Expression::And(x, y) => (*x, *y),
                _ => unreachable!(),
            };
            agenda.put(a);
            agenda.put(b);
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }
        Category::NOr => {
            let (a, b) = match current {
                Expression::Negated(t) => match *t {
                    Expression::Or(x, y) => (*x, *y),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            agenda.put(Expression::not(a));
            agenda.put(Expression::not(b));
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }
        Category::NImp => {
            let (a, b) = match current {
                Expression::Negated(t) => match *t {
                    Expression::Imp(x, y) => (*x, *y),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            agenda.put(a);
            agenda.put(Expression::not(b));
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- beta rules (branch) ----
        Category::Or => {
            let (a, b) = match current {
                Expression::Or(x, y) => (*x, *y),
                _ => unreachable!(),
            };
            beta_branch(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
                a,
                b,
            )
        }
        Category::Imp => {
            let (a, b) = match current {
                Expression::Imp(x, y) => (*x, *y),
                _ => unreachable!(),
            };
            beta_branch(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
                Expression::not(a),
                b,
            )
        }
        Category::NAnd => {
            let (a, b) = match current {
                Expression::Negated(t) => match *t {
                    Expression::And(x, y) => (*x, *y),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            beta_branch(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
                Expression::not(a),
                Expression::not(b),
            )
        }
        Category::Iff => {
            let (a, b) = match current {
                Expression::Iff(x, y) => (*x, *y),
                _ => unreachable!(),
            };
            // Branch 1: A & B    Branch 2: ¬A & ¬B
            beta_pair(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
                (a.clone(), b.clone()),
                (Expression::not(a), Expression::not(b)),
            )
        }
        Category::NIff => {
            let (a, b) = match current {
                Expression::Negated(t) => match *t {
                    Expression::Iff(x, y) => (*x, *y),
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
            beta_pair(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent,
                steps,
                (a.clone(), Expression::not(b.clone())),
                (Expression::not(a), b),
            )
        }

        // ---- equality rewrite ----
        Category::Eq => {
            let (lhs, rhs) = match current {
                Expression::Equality(a, b) => (*a, *b),
                _ => unreachable!(),
            };
            // NLTK: only variable LHS triggers a substitution.
            if let Expression::Variable(v) = &lhs {
                agenda.put_atoms(atoms);
                agenda.requeue_n_eqs();
                agenda.rewrite(v, &rhs);
                accessible_vars.remove(&lhs);
                let mut new_acc = accessible_vars.clone();
                new_acc.insert(rhs.clone());
                return attempt_proof(
                    cfg,
                    agenda,
                    new_acc,
                    BTreeSet::new(),
                    debug,
                    indent + 1,
                    steps,
                );
            }
            // Non-variable equality — try the symmetric direction.
            if let Expression::Variable(v) = &rhs {
                agenda.put_atoms(atoms);
                agenda.requeue_n_eqs();
                agenda.rewrite(v, &lhs);
                accessible_vars.remove(&rhs);
                let mut new_acc = accessible_vars.clone();
                new_acc.insert(lhs.clone());
                return attempt_proof(
                    cfg,
                    agenda,
                    new_acc,
                    BTreeSet::new(),
                    debug,
                    indent + 1,
                    steps,
                );
            }
            // Neither side is a variable — record both as accessible
            // and continue (no rewrite possible).
            accessible_vars.insert(lhs);
            accessible_vars.insert(rhs);
            attempt_proof(
                cfg,
                agenda,
                accessible_vars.clone(),
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        // ---- ∃x.P → P[x := fresh] ----
        Category::Exists => {
            let (var, body) = match current {
                Expression::Exists(v, b) => (v, *b),
                _ => unreachable!(),
            };
            let fresh = fresh_skolem();
            let instantiated = subst(&body, &var, &fresh);
            agenda.put(instantiated);
            agenda.mark_alls_fresh();
            let mut new_acc = accessible_vars.clone();
            new_acc.insert(fresh);
            attempt_proof(
                cfg,
                agenda,
                new_acc,
                atoms.clone(),
                debug,
                indent + 1,
                steps,
            )
        }

        Category::All => unreachable!("Category::All routed through Popped::All"),
    }
}

#[allow(clippy::too_many_arguments)]
fn beta_branch(
    cfg: &TableauProver,
    agenda: &mut Agenda,
    accessible_vars: &mut BTreeSet<Expression>,
    atoms: &mut BTreeSet<(Expression, bool)>,
    debug: &mut Debug,
    indent: usize,
    steps: &mut u64,
    left: Expression,
    right: Expression,
) -> Result<bool, InferenceError> {
    let mut left_agenda = agenda.clone();
    left_agenda.put(left);
    let left_ok = attempt_proof(
        cfg,
        &mut left_agenda,
        accessible_vars.clone(),
        atoms.clone(),
        debug,
        indent + 1,
        steps,
    )?;
    if !left_ok {
        return Ok(false);
    }
    let mut right_agenda = agenda.clone();
    right_agenda.put(right);
    attempt_proof(
        cfg,
        &mut right_agenda,
        accessible_vars.clone(),
        atoms.clone(),
        debug,
        indent + 1,
        steps,
    )
}

#[allow(clippy::too_many_arguments)]
fn beta_pair(
    cfg: &TableauProver,
    agenda: &mut Agenda,
    accessible_vars: &mut BTreeSet<Expression>,
    atoms: &mut BTreeSet<(Expression, bool)>,
    debug: &mut Debug,
    indent: usize,
    steps: &mut u64,
    left: (Expression, Expression),
    right: (Expression, Expression),
) -> Result<bool, InferenceError> {
    let mut left_agenda = agenda.clone();
    left_agenda.put(left.0);
    left_agenda.put(left.1);
    let left_ok = attempt_proof(
        cfg,
        &mut left_agenda,
        accessible_vars.clone(),
        atoms.clone(),
        debug,
        indent + 1,
        steps,
    )?;
    if !left_ok {
        return Ok(false);
    }
    let mut right_agenda = agenda.clone();
    right_agenda.put(right.0);
    right_agenda.put(right.1);
    attempt_proof(
        cfg,
        &mut right_agenda,
        accessible_vars.clone(),
        atoms.clone(),
        debug,
        indent + 1,
        steps,
    )
}

#[allow(clippy::too_many_arguments)]
fn handle_all(
    cfg: &TableauProver,
    mut entry: AllEntry,
    agenda: &mut Agenda,
    accessible_vars: BTreeSet<Expression>,
    atoms: BTreeSet<(Expression, bool)>,
    debug: &mut Debug,
    indent: usize,
    steps: &mut u64,
) -> Result<bool, InferenceError> {
    let (var, body) = match &entry.expr {
        Expression::All(v, b) => (v.clone(), (**b).clone()),
        _ => unreachable!(),
    };

    if accessible_vars.is_empty() {
        // No witness yet — introduce a fresh one, like `Exists`.
        let fresh = fresh_skolem();
        let instantiated = subst(&body, &var, &fresh);
        entry.used_vars.insert(fresh.clone());
        agenda.put(instantiated);
        agenda.alls.push(entry);
        agenda.mark_alls_fresh();
        let mut new_acc = accessible_vars.clone();
        new_acc.insert(fresh);
        return attempt_proof(cfg, agenda, new_acc, atoms, debug, indent + 1, steps);
    }

    let available: Vec<Expression> = accessible_vars
        .iter()
        .filter(|w| !entry.used_vars.contains(*w))
        .cloned()
        .collect();

    if let Some(witness) = available.into_iter().next() {
        if entry.used_vars.len() >= cfg.max_universal_instantiations {
            debug.line(indent + 1, "ALL: max instantiations reached");
            entry.exhausted = true;
            agenda.alls.push(entry);
            return attempt_proof(
                cfg,
                agenda,
                accessible_vars,
                atoms,
                debug,
                indent + 1,
                steps,
            );
        }
        debug.line(indent + 1, format!("--> using {}", witness));
        let instantiated = subst(&body, &var, &witness);
        entry.used_vars.insert(witness);
        agenda.put(instantiated);
        agenda.alls.push(entry);
        attempt_proof(
            cfg,
            agenda,
            accessible_vars,
            atoms,
            debug,
            indent + 1,
            steps,
        )
    } else {
        debug.line(indent + 1, "ALL: no fresh witnesses, exhausted");
        entry.exhausted = true;
        agenda.alls.push(entry);
        attempt_proof(
            cfg,
            agenda,
            accessible_vars,
            atoms,
            debug,
            indent + 1,
            steps,
        )
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ml::nlp::sem::logic::parse;

    fn p(s: &str) -> Expression {
        parse(s).unwrap_or_else(|e| panic!("parse {:?}: {}", s, e))
    }

    fn prove(goal: &str, assumptions: &[&str]) -> bool {
        let prover = TableauProver::new();
        let g = p(goal);
        let a: Vec<Expression> = assumptions.iter().map(|s| p(s)).collect();
        prover.prove(Some(&g), &a, false).unwrap().proved
    }

    // ---- propositional ----

    #[test]
    fn law_of_excluded_middle() {
        assert!(prove("P | -P", &[]));
    }

    #[test]
    fn contradiction_proves_anything_via_assumptions() {
        assert!(prove("Q", &["P", "-P"]));
    }

    #[test]
    fn modus_ponens() {
        assert!(prove("Q", &["P", "(P -> Q)"]));
    }

    #[test]
    fn modus_tollens() {
        assert!(prove("-P", &["(P -> Q)", "-Q"]));
    }

    #[test]
    fn double_negation_elimination() {
        assert!(prove("P", &["--P"]));
    }

    #[test]
    fn de_morgan_and() {
        assert!(prove("(-P | -Q)", &["-(P & Q)"]));
    }

    #[test]
    fn de_morgan_or() {
        assert!(prove("(-P & -Q)", &["-(P | Q)"]));
    }

    #[test]
    fn iff_unfolds() {
        assert!(prove("(P -> Q)", &["(P <-> Q)"]));
        assert!(prove("(Q -> P)", &["(P <-> Q)"]));
    }

    #[test]
    fn invalid_propositional_not_proved() {
        // P |- Q is invalid.
        assert!(!prove("Q", &["P"]));
    }

    #[test]
    fn affirming_the_consequent_invalid() {
        assert!(!prove("P", &["(P -> Q)", "Q"]));
    }

    // ---- first-order ----

    #[test]
    fn socrates_syllogism() {
        assert!(prove(
            "mortal(socrates)",
            &["all x.(man(x) -> mortal(x))", "man(socrates)"]
        ));
    }

    #[test]
    fn existential_witness() {
        // all x. (man(x) -> walks(x)),  man(john)  |-  some y. walks(y)
        assert!(prove(
            "some y. walks(y)",
            &["all x.(man(x) -> walks(x))", "man(john)"]
        ));
    }

    #[test]
    fn man_implies_man_tautology() {
        assert!(prove("(man(x) -> man(x))", &[]));
    }

    #[test]
    fn negated_contradiction_tautology() {
        assert!(prove("-(man(x) & -man(x))", &[]));
    }

    #[test]
    fn man_or_not_man_tautology() {
        assert!(prove("(man(x) | -man(x))", &[]));
    }

    #[test]
    fn negated_universal_to_existential() {
        // -all x.P(x)  |-  some x.-P(x)
        assert!(prove("some x. -P(x)", &["-all x. P(x)"]));
    }

    #[test]
    fn negated_existential_to_universal() {
        assert!(prove("all x. -P(x)", &["-some x. P(x)"]));
    }

    // ---- equality ----

    #[test]
    fn reflexivity_self() {
        assert!(prove("(a = a)", &[]));
    }

    #[test]
    fn equality_symmetry_universal() {
        assert!(prove("all x. all y. ((x = y) -> (y = x))", &[]));
    }

    #[test]
    fn equality_substitution_in_predicate() {
        // (x = y) & walks(y)  |-  walks(x)
        assert!(prove("walks(x)", &["((x = y) & walks(y))"]));
    }

    #[test]
    fn equality_transitivity() {
        assert!(prove("(x = w)", &["((x = y) & ((y = z) & (z = w)))"]));
    }

    // ---- non-theorems ----

    #[test]
    fn empty_conjunction_is_not_proved_alone() {
        // Just `P` is not a tautology.
        assert!(!prove("P", &[]));
    }
}
