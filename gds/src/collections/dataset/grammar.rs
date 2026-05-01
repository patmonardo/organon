//! Context-free grammars — port of NLTK `nltk/grammar.py`.
//!
//! ## Scope of this slice
//!
//! Ported:
//! - [`Nonterminal`] and [`nonterminals`] — symbol type for non-leaf
//!   parse-tree nodes.
//! - [`Production`] — `lhs -> rhs` rules, where `rhs` is a sequence
//!   of [`RhsItem`]s (nonterminals or terminal strings).
//! - [`CFG`] — start symbol + production list + indexed access.
//! - [`ProbabilisticProduction`] and [`PCFG`] — with probability sums
//!   verified to 1.0 within tolerance.
//! - [`FeatStructNonterminal`] and [`FeatureGrammar`] — feature-based
//!   nonterminals (uses [`crate::collections::dataset::featstruct`]).
//! - [`induce_pcfg`] — count productions and normalize per-LHS.
//! - [`read_grammar`] — small text reader for the standard NLTK
//!   grammar format (`S -> NP VP`, `'word' | DET N`, optional
//!   `[0.6]` probability suffix, `# comment`).
//!
//! Deferred (not needed for the dataset feature-grammar slice):
//! - leftcorner / leftcorner-parents indices,
//! - CNF transformation (we have FOL CNF elsewhere via
//!   [`crate::ml::nlp::sem::skolemize`]),
//! - separate dependency-grammar parser (we have richer
//!   dependency-parser graph types under [`crate::ml::nlp::parse::dependency`] already).
//!
//! Lives at the dataset top level, alongside [`super::featstruct`] and
//! [`super::tree`], so the "text + tree + feature-grammar" world is in
//! one place.

use std::collections::BTreeMap;
use std::fmt;

use crate::collections::dataset::featstruct::{
    parse_featstruct, FeatStruct, FeatStructParseError, FeatValue,
};

// ---------------------------------------------------------------------------
// Nonterminal
// ---------------------------------------------------------------------------

/// A non-leaf grammar symbol (e.g. `S`, `NP`, `VP`). Mirrors NLTK
/// `Nonterminal`. The wrapper exists so the grammar engine can
/// distinguish a node-value `"NP"` from a leaf token `"NP"`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Nonterminal {
    symbol: String,
}

impl Nonterminal {
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl fmt::Display for Nonterminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.symbol)
    }
}

/// Split a comma- or whitespace-delimited symbol string into a list of
/// fresh [`Nonterminal`]s. Mirrors NLTK `nonterminals`.
pub fn nonterminals(symbols: &str) -> Vec<Nonterminal> {
    let parts: Vec<&str> = if symbols.contains(',') {
        symbols.split(',').collect()
    } else {
        symbols.split_whitespace().collect()
    };
    parts
        .into_iter()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(Nonterminal::new)
        .collect()
}

// ---------------------------------------------------------------------------
// RhsItem and Production
// ---------------------------------------------------------------------------

/// Right-hand-side element: either a nonterminal child or a terminal
/// string literal. NLTK allows arbitrary hashable terminals; we
/// restrict to strings, which covers the dominant grammar-text
/// authoring path.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RhsItem {
    Nt(Nonterminal),
    Terminal(String),
}

impl RhsItem {
    pub fn is_nonterminal(&self) -> bool {
        matches!(self, RhsItem::Nt(_))
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, RhsItem::Terminal(_))
    }

    pub fn as_nonterminal(&self) -> Option<&Nonterminal> {
        match self {
            RhsItem::Nt(n) => Some(n),
            _ => None,
        }
    }

    pub fn as_terminal(&self) -> Option<&str> {
        match self {
            RhsItem::Terminal(s) => Some(s.as_str()),
            _ => None,
        }
    }
}

impl fmt::Display for RhsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RhsItem::Nt(n) => write!(f, "{}", n),
            RhsItem::Terminal(t) => write!(f, "'{}'", t),
        }
    }
}

/// A single grammar rule `lhs -> rhs`. Mirrors NLTK `Production`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Production {
    lhs: Nonterminal,
    rhs: Vec<RhsItem>,
}

impl Production {
    pub fn new(lhs: Nonterminal, rhs: Vec<RhsItem>) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &Nonterminal {
        &self.lhs
    }

    pub fn rhs(&self) -> &[RhsItem] {
        &self.rhs
    }

    /// True iff the RHS is exactly one terminal — i.e. a lexical
    /// (preterminal) rule like `N -> 'dog'`.
    pub fn is_lexical(&self) -> bool {
        self.rhs.len() == 1 && self.rhs[0].is_terminal()
    }

    /// True iff every RHS item is a nonterminal.
    pub fn is_nonlexical(&self) -> bool {
        !self.rhs.is_empty() && self.rhs.iter().all(RhsItem::is_nonterminal)
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parts: Vec<String> = self.rhs.iter().map(|i| i.to_string()).collect();
        if parts.is_empty() {
            write!(f, "{} -> ", self.lhs)
        } else {
            write!(f, "{} -> {}", self.lhs, parts.join(" "))
        }
    }
}

// ---------------------------------------------------------------------------
// CFG
// ---------------------------------------------------------------------------

/// A context-free grammar: start symbol + ordered list of productions.
/// Mirrors NLTK `CFG`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CFG {
    start: Nonterminal,
    productions: Vec<Production>,
    /// Productions indexed by LHS symbol (stable order preserved by
    /// the underlying [`BTreeMap`], filled by [`Self::reindex`]).
    by_lhs: BTreeMap<String, Vec<usize>>,
    /// Productions indexed by the symbol of their first RHS item, when
    /// that item is a [`Nonterminal`]. Useful for top-down parsers.
    by_rhs0_nt: BTreeMap<String, Vec<usize>>,
    /// Productions indexed by their first RHS item when it is a
    /// terminal string (lexical lookup).
    by_rhs0_term: BTreeMap<String, Vec<usize>>,
}

impl CFG {
    pub fn new(start: Nonterminal, productions: Vec<Production>) -> Self {
        let mut g = Self {
            start,
            productions,
            by_lhs: BTreeMap::new(),
            by_rhs0_nt: BTreeMap::new(),
            by_rhs0_term: BTreeMap::new(),
        };
        g.reindex();
        g
    }

    fn reindex(&mut self) {
        self.by_lhs.clear();
        self.by_rhs0_nt.clear();
        self.by_rhs0_term.clear();
        for (i, p) in self.productions.iter().enumerate() {
            self.by_lhs.entry(p.lhs.symbol.clone()).or_default().push(i);
            if let Some(first) = p.rhs.first() {
                match first {
                    RhsItem::Nt(n) => {
                        self.by_rhs0_nt.entry(n.symbol.clone()).or_default().push(i);
                    }
                    RhsItem::Terminal(t) => {
                        self.by_rhs0_term.entry(t.clone()).or_default().push(i);
                    }
                }
            }
        }
    }

    pub fn start(&self) -> &Nonterminal {
        &self.start
    }

    pub fn all_productions(&self) -> &[Production] {
        &self.productions
    }

    /// Productions matching an LHS and/or a leading RHS item. Mirrors
    /// the most-used calling convention of NLTK
    /// `CFG.productions(lhs=, rhs=)`.
    pub fn productions(
        &self,
        lhs: Option<&Nonterminal>,
        rhs: Option<&RhsItem>,
    ) -> Vec<&Production> {
        let mut out = Vec::new();
        let candidates: Box<dyn Iterator<Item = usize>> = match lhs {
            Some(l) => Box::new(
                self.by_lhs
                    .get(&l.symbol)
                    .into_iter()
                    .flat_map(|v| v.iter().copied()),
            ),
            None => Box::new(0..self.productions.len()),
        };
        for i in candidates {
            let p = &self.productions[i];
            if let Some(r) = rhs {
                if p.rhs.first() != Some(r) {
                    continue;
                }
            }
            out.push(p);
        }
        out
    }

    /// True iff every token in `tokens` appears as a terminal on the
    /// RHS of at least one production (i.e. the lexicon covers it).
    pub fn check_coverage<'a, I>(&self, tokens: I) -> Result<(), Vec<String>>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut missing = Vec::new();
        for t in tokens {
            if !self.by_rhs0_term.contains_key(t)
                && !self.productions.iter().any(|p| {
                    p.rhs
                        .iter()
                        .any(|i| matches!(i, RhsItem::Terminal(s) if s == t))
                })
            {
                missing.push(t.to_string());
            }
        }
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }

    pub fn is_lexical(&self) -> bool {
        self.productions.iter().all(|p| p.is_lexical())
    }

    pub fn is_nonlexical(&self) -> bool {
        self.productions.iter().all(|p| p.is_nonlexical())
    }

    pub fn is_nonempty(&self) -> bool {
        self.productions.iter().all(|p| !p.rhs.is_empty())
    }

    /// Every production has at most two RHS items. NLTK calls this
    /// "binarised".
    pub fn is_binarised(&self) -> bool {
        self.productions.iter().all(|p| p.rhs.len() <= 2)
    }

    pub fn min_len(&self) -> usize {
        self.productions
            .iter()
            .map(|p| p.rhs.len())
            .min()
            .unwrap_or(0)
    }

    pub fn max_len(&self) -> usize {
        self.productions
            .iter()
            .map(|p| p.rhs.len())
            .max()
            .unwrap_or(0)
    }
}

impl fmt::Display for CFG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Grammar with {} productions:", self.productions.len())?;
        for p in &self.productions {
            writeln!(f, "    {}", p)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Probabilistic productions and PCFG
// ---------------------------------------------------------------------------

/// A production with an attached probability. Mirrors NLTK
/// `ProbabilisticProduction`.
#[derive(Debug, Clone, PartialEq)]
pub struct ProbabilisticProduction {
    production: Production,
    prob: f64,
}

impl ProbabilisticProduction {
    pub fn new(production: Production, prob: f64) -> Self {
        Self { production, prob }
    }

    pub fn production(&self) -> &Production {
        &self.production
    }

    pub fn lhs(&self) -> &Nonterminal {
        self.production.lhs()
    }

    pub fn rhs(&self) -> &[RhsItem] {
        self.production.rhs()
    }

    pub fn prob(&self) -> f64 {
        self.prob
    }
}

impl fmt::Display for ProbabilisticProduction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.production, self.prob)
    }
}

/// Probabilistic CFG. Verifies that, for each LHS, the probabilities
/// of all productions sum to 1.0 within [`PCFG_TOLERANCE`].
#[derive(Debug, Clone)]
pub struct PCFG {
    start: Nonterminal,
    productions: Vec<ProbabilisticProduction>,
}

/// Tolerance for the per-LHS probability sum check.
pub const PCFG_TOLERANCE: f64 = 1e-6;

impl PCFG {
    pub fn new(
        start: Nonterminal,
        productions: Vec<ProbabilisticProduction>,
    ) -> Result<Self, GrammarError> {
        let mut sums: BTreeMap<String, f64> = BTreeMap::new();
        for p in &productions {
            *sums.entry(p.lhs().symbol.clone()).or_insert(0.0) += p.prob;
        }
        for (lhs, sum) in &sums {
            if (sum - 1.0).abs() > PCFG_TOLERANCE {
                return Err(GrammarError(format!(
                    "probabilities for LHS {} sum to {} (must be 1)",
                    lhs, sum
                )));
            }
        }
        Ok(Self { start, productions })
    }

    pub fn start(&self) -> &Nonterminal {
        &self.start
    }

    pub fn productions(&self) -> &[ProbabilisticProduction] {
        &self.productions
    }
}

impl fmt::Display for PCFG {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PCFG with {} productions:", self.productions.len())?;
        for p in &self.productions {
            writeln!(f, "    {}", p)?;
        }
        Ok(())
    }
}

/// Induce a PCFG from a flat list of productions: count occurrences
/// per LHS and normalize. Mirrors NLTK `induce_pcfg`.
pub fn induce_pcfg(start: Nonterminal, productions: &[Production]) -> Result<PCFG, GrammarError> {
    let mut counts: BTreeMap<&Production, usize> = BTreeMap::new();
    let mut lhs_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for p in productions {
        *counts.entry(p).or_insert(0) += 1;
        *lhs_counts.entry(p.lhs.symbol.as_str()).or_insert(0) += 1;
    }
    let mut prob_prods = Vec::new();
    for (p, c) in counts {
        let total = lhs_counts[p.lhs.symbol.as_str()] as f64;
        prob_prods.push(ProbabilisticProduction::new(p.clone(), c as f64 / total));
    }
    PCFG::new(start, prob_prods)
}

// ---------------------------------------------------------------------------
// Feature-based nonterminals and grammars
// ---------------------------------------------------------------------------

/// A nonterminal whose identity is a [`FeatStruct`]. Mirrors NLTK
/// `FeatStructNonterminal`. The "symbol" used for type-indexing is the
/// value of the `TYPE` feature when present, else the formatted feature
/// structure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatStructNonterminal {
    fs: FeatStruct,
}

/// Conventional feature name that names the syntactic category of a
/// [`FeatStructNonterminal`] (e.g. `S`, `NP`, `VP`). Mirrors NLTK's
/// `TYPE` constant.
pub const TYPE_FEATURE: &str = "TYPE";

impl FeatStructNonterminal {
    pub fn new(fs: FeatStruct) -> Self {
        Self { fs }
    }

    pub fn fs(&self) -> &FeatStruct {
        &self.fs
    }

    /// Category name: the value of the `TYPE` feature if it is a
    /// string atom, else the structure's `Display` form.
    pub fn symbol(&self) -> String {
        if let FeatStruct::Dict(map) = &self.fs {
            if let Some(FeatValue::Text(a)) = map.get(TYPE_FEATURE) {
                return a.clone();
            }
        }
        format!("{:?}", self.fs)
    }
}

impl fmt::Display for FeatStructNonterminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// Feature-based grammar: productions whose LHS and nonterminal RHS
/// items are [`FeatStructNonterminal`]s. Mirrors NLTK `FeatureGrammar`.
///
/// Productions are stored indexed by the LHS *category* (the `TYPE`
/// feature) so the parser can quickly find candidate expansions.
/// Feature unification on those candidates is the parser's job —
/// kept out of the grammar object so the grammar stays a pure value.
#[derive(Debug, Clone)]
pub struct FeatureGrammar {
    start: FeatStructNonterminal,
    productions: Vec<FeatureProduction>,
    by_lhs_type: BTreeMap<String, Vec<usize>>,
}

/// A production over feature-structured nonterminals. Terminals on the
/// RHS are still bare strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureProduction {
    lhs: FeatStructNonterminal,
    rhs: Vec<FeatureRhsItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FeatureRhsItem {
    Nt(FeatStructNonterminal),
    Terminal(String),
}

impl FeatureProduction {
    pub fn new(lhs: FeatStructNonterminal, rhs: Vec<FeatureRhsItem>) -> Self {
        Self { lhs, rhs }
    }

    pub fn lhs(&self) -> &FeatStructNonterminal {
        &self.lhs
    }

    pub fn rhs(&self) -> &[FeatureRhsItem] {
        &self.rhs
    }
}

impl FeatureGrammar {
    pub fn new(start: FeatStructNonterminal, productions: Vec<FeatureProduction>) -> Self {
        let mut by_lhs_type: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, p) in productions.iter().enumerate() {
            by_lhs_type.entry(p.lhs.symbol()).or_default().push(i);
        }
        Self {
            start,
            productions,
            by_lhs_type,
        }
    }

    pub fn start(&self) -> &FeatStructNonterminal {
        &self.start
    }

    pub fn productions(&self) -> &[FeatureProduction] {
        &self.productions
    }

    /// Candidate productions whose LHS shares the given category name.
    pub fn productions_for_type(&self, type_name: &str) -> Vec<&FeatureProduction> {
        self.by_lhs_type
            .get(type_name)
            .map(|ix| ix.iter().map(|&i| &self.productions[i]).collect())
            .unwrap_or_default()
    }
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Grammar construction or parsing error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrammarError(pub String);

impl fmt::Display for GrammarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for GrammarError {}

impl From<FeatStructParseError> for GrammarError {
    fn from(e: FeatStructParseError) -> Self {
        GrammarError(format!("featstruct parse: {:?}", e))
    }
}

// ---------------------------------------------------------------------------
// Text reader
// ---------------------------------------------------------------------------

/// What kind of grammar to construct from `read_grammar` input.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrammarKind {
    Cfg,
    Pcfg,
}

/// Read a grammar from text. The text format is:
///
/// ```text
/// # comments start with '#'
/// S  -> NP VP
/// NP -> DET N | 'John' | 'Mary'
/// DET -> 'the' | 'a'
/// N  -> 'dog' | 'cat'
/// VP -> V NP
/// V  -> 'chased'
/// ```
///
/// For PCFGs append a probability per alternative:
///
/// ```text
/// S  -> NP VP [1.0]
/// NP -> DET N [0.6] | 'John' [0.2] | 'Mary' [0.2]
/// ```
///
/// Single-quoted RHS items are terminals; bare identifiers are
/// nonterminals. The first nonterminal seen on any LHS is used as the
/// start symbol unless `start` is provided.
pub fn read_grammar(
    input: &str,
    kind: GrammarKind,
    start: Option<Nonterminal>,
) -> Result<ReadGrammar, GrammarError> {
    let mut productions: Vec<Production> = Vec::new();
    let mut prob_productions: Vec<ProbabilisticProduction> = Vec::new();
    let mut first_lhs: Option<Nonterminal> = None;

    for (lineno, raw) in input.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (lhs_str, rhs_str) = line.split_once("->").ok_or_else(|| {
            GrammarError(format!("line {}: missing `->` in {:?}", lineno + 1, raw))
        })?;
        let lhs = Nonterminal::new(lhs_str.trim());
        if first_lhs.is_none() {
            first_lhs = Some(lhs.clone());
        }
        for alt in rhs_str.split('|') {
            let (items, prob) = parse_rhs_alternative(alt, lineno + 1)?;
            match kind {
                GrammarKind::Cfg => {
                    if prob.is_some() {
                        return Err(GrammarError(format!(
                            "line {}: probability not allowed in CFG",
                            lineno + 1
                        )));
                    }
                    productions.push(Production::new(lhs.clone(), items));
                }
                GrammarKind::Pcfg => {
                    let p = prob.ok_or_else(|| {
                        GrammarError(format!(
                            "line {}: missing probability for PCFG production",
                            lineno + 1
                        ))
                    })?;
                    prob_productions.push(ProbabilisticProduction::new(
                        Production::new(lhs.clone(), items),
                        p,
                    ));
                }
            }
        }
    }

    let start = start.or(first_lhs).ok_or_else(|| {
        GrammarError("grammar contains no productions; cannot infer start symbol".into())
    })?;

    Ok(match kind {
        GrammarKind::Cfg => ReadGrammar::Cfg(CFG::new(start, productions)),
        GrammarKind::Pcfg => ReadGrammar::Pcfg(PCFG::new(start, prob_productions)?),
    })
}

/// The result of [`read_grammar`].
#[derive(Debug, Clone)]
pub enum ReadGrammar {
    Cfg(CFG),
    Pcfg(PCFG),
}

fn parse_rhs_alternative(
    alt: &str,
    lineno: usize,
) -> Result<(Vec<RhsItem>, Option<f64>), GrammarError> {
    let alt = alt.trim();
    let (body, prob) = match (alt.rfind('['), alt.rfind(']')) {
        (Some(lb), Some(rb)) if rb == alt.len() - 1 && rb > lb => {
            let p_str = &alt[lb + 1..rb];
            let p: f64 = p_str.trim().parse().map_err(|_| {
                GrammarError(format!("line {}: invalid probability {:?}", lineno, p_str))
            })?;
            (alt[..lb].trim_end(), Some(p))
        }
        _ => (alt, None),
    };

    let items = tokenize_rhs(body, lineno)?;
    Ok((items, prob))
}

fn tokenize_rhs(s: &str, lineno: usize) -> Result<Vec<RhsItem>, GrammarError> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == b'\'' || c == b'"' {
            let quote = c;
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != quote {
                j += 1;
            }
            if j >= bytes.len() {
                return Err(GrammarError(format!(
                    "line {}: unterminated string in RHS",
                    lineno
                )));
            }
            let term = std::str::from_utf8(&bytes[start..j])
                .map_err(|_| GrammarError(format!("line {}: invalid UTF-8 in RHS", lineno)))?;
            out.push(RhsItem::Terminal(term.to_string()));
            i = j + 1;
        } else {
            let start = i;
            while i < bytes.len()
                && !bytes[i].is_ascii_whitespace()
                && bytes[i] != b'\''
                && bytes[i] != b'"'
            {
                i += 1;
            }
            let name = std::str::from_utf8(&bytes[start..i])
                .map_err(|_| GrammarError(format!("line {}: invalid UTF-8 in RHS", lineno)))?;
            out.push(RhsItem::Nt(Nonterminal::new(name)));
        }
    }
    Ok(out)
}

/// Read a feature-based grammar from text. Each non-comment line has
/// the same `LHS -> RHS [| RHS …]` shape as [`read_grammar`], but
/// nonterminals may carry an inline feature structure in square
/// brackets (NLTK convention), e.g. `NP[NUM=sg] -> DET N[NUM=sg]`. If
/// no feature structure is given, the symbol becomes the value of the
/// `TYPE` feature.
pub fn read_feature_grammar(
    input: &str,
    start: Option<FeatStructNonterminal>,
) -> Result<FeatureGrammar, GrammarError> {
    let mut productions: Vec<FeatureProduction> = Vec::new();
    let mut first_lhs: Option<FeatStructNonterminal> = None;

    for (lineno, raw) in input.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let (lhs_str, rhs_str) = line
            .split_once("->")
            .ok_or_else(|| GrammarError(format!("line {}: missing `->`", lineno + 1)))?;
        let lhs = parse_feature_nonterminal(lhs_str.trim(), lineno + 1)?;
        if first_lhs.is_none() {
            first_lhs = Some(lhs.clone());
        }
        for alt in rhs_str.split('|') {
            let items = tokenize_feature_rhs(alt.trim(), lineno + 1)?;
            productions.push(FeatureProduction::new(lhs.clone(), items));
        }
    }

    let start = start.or(first_lhs).ok_or_else(|| {
        GrammarError("feature grammar is empty; cannot infer start symbol".into())
    })?;
    Ok(FeatureGrammar::new(start, productions))
}

fn parse_feature_nonterminal(
    s: &str,
    lineno: usize,
) -> Result<FeatStructNonterminal, GrammarError> {
    if let Some(lb) = s.find('[') {
        let rb = s
            .rfind(']')
            .ok_or_else(|| GrammarError(format!("line {}: unmatched '[' in {:?}", lineno, s)))?;
        if rb < lb {
            return Err(GrammarError(format!(
                "line {}: malformed feature nonterminal {:?}",
                lineno, s
            )));
        }
        let cat = s[..lb].trim().to_string();
        let fs_text = &s[lb..=rb];
        let fs = parse_featstruct(fs_text)?;
        let fs = inject_type(fs, &cat);
        Ok(FeatStructNonterminal::new(fs))
    } else {
        let mut map: BTreeMap<String, FeatValue> = BTreeMap::new();
        map.insert(
            TYPE_FEATURE.to_string(),
            FeatValue::Text(s.trim().to_string()),
        );
        Ok(FeatStructNonterminal::new(FeatStruct::Dict(map)))
    }
}

fn inject_type(fs: FeatStruct, cat: &str) -> FeatStruct {
    match fs {
        FeatStruct::Dict(mut map) => {
            map.entry(TYPE_FEATURE.to_string())
                .or_insert_with(|| FeatValue::Text(cat.to_string()));
            FeatStruct::Dict(map)
        }
        other => other,
    }
}

fn tokenize_feature_rhs(s: &str, lineno: usize) -> Result<Vec<FeatureRhsItem>, GrammarError> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c.is_ascii_whitespace() {
            i += 1;
            continue;
        }
        if c == b'\'' || c == b'"' {
            let quote = c;
            let start = i + 1;
            let mut j = start;
            while j < bytes.len() && bytes[j] != quote {
                j += 1;
            }
            if j >= bytes.len() {
                return Err(GrammarError(format!(
                    "line {}: unterminated string in RHS",
                    lineno
                )));
            }
            let term = std::str::from_utf8(&bytes[start..j])
                .map_err(|_| GrammarError(format!("line {}: invalid UTF-8", lineno)))?;
            out.push(FeatureRhsItem::Terminal(term.to_string()));
            i = j + 1;
        } else {
            // Read identifier, then optionally a bracketed feature
            // structure attached to it.
            let start = i;
            while i < bytes.len()
                && !bytes[i].is_ascii_whitespace()
                && bytes[i] != b'\''
                && bytes[i] != b'"'
                && bytes[i] != b'['
            {
                i += 1;
            }
            let mut name_end = i;
            // Optional [...] block
            if i < bytes.len() && bytes[i] == b'[' {
                let mut depth = 0;
                while i < bytes.len() {
                    match bytes[i] {
                        b'[' => depth += 1,
                        b']' => {
                            depth -= 1;
                            if depth == 0 {
                                i += 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                    i += 1;
                }
                if depth != 0 {
                    return Err(GrammarError(format!(
                        "line {}: unmatched '[' in RHS",
                        lineno
                    )));
                }
                name_end = i;
            }
            let token = std::str::from_utf8(&bytes[start..name_end])
                .map_err(|_| GrammarError(format!("line {}: invalid UTF-8", lineno)))?;
            out.push(FeatureRhsItem::Nt(parse_feature_nonterminal(
                token, lineno,
            )?));
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn nt(s: &str) -> Nonterminal {
        Nonterminal::new(s)
    }

    fn term(s: &str) -> RhsItem {
        RhsItem::Terminal(s.to_string())
    }

    fn ntr(s: &str) -> RhsItem {
        RhsItem::Nt(nt(s))
    }

    // ---- nonterminal basics ----

    #[test]
    fn nonterminals_split_by_comma_or_whitespace() {
        let xs = nonterminals("S, NP, VP");
        assert_eq!(xs, vec![nt("S"), nt("NP"), nt("VP")]);
        let ys = nonterminals("S NP VP");
        assert_eq!(ys, vec![nt("S"), nt("NP"), nt("VP")]);
    }

    #[test]
    fn rhs_item_kind_predicates() {
        assert!(ntr("NP").is_nonterminal());
        assert!(term("dog").is_terminal());
        assert_eq!(ntr("NP").as_nonterminal(), Some(&nt("NP")));
        assert_eq!(term("dog").as_terminal(), Some("dog"));
    }

    // ---- production shape ----

    #[test]
    fn production_lexical_vs_nonlexical() {
        let lex = Production::new(nt("N"), vec![term("dog")]);
        assert!(lex.is_lexical());
        assert!(!lex.is_nonlexical());

        let nl = Production::new(nt("S"), vec![ntr("NP"), ntr("VP")]);
        assert!(!nl.is_lexical());
        assert!(nl.is_nonlexical());
    }

    #[test]
    fn production_display() {
        let p = Production::new(nt("S"), vec![ntr("NP"), ntr("VP")]);
        assert_eq!(p.to_string(), "S -> NP VP");
        let l = Production::new(nt("N"), vec![term("dog")]);
        assert_eq!(l.to_string(), "N -> 'dog'");
    }

    // ---- CFG construction and queries ----

    fn toy_cfg() -> CFG {
        CFG::new(
            nt("S"),
            vec![
                Production::new(nt("S"), vec![ntr("NP"), ntr("VP")]),
                Production::new(nt("NP"), vec![ntr("DET"), ntr("N")]),
                Production::new(nt("NP"), vec![term("John")]),
                Production::new(nt("VP"), vec![ntr("V"), ntr("NP")]),
                Production::new(nt("DET"), vec![term("the")]),
                Production::new(nt("N"), vec![term("dog")]),
                Production::new(nt("N"), vec![term("cat")]),
                Production::new(nt("V"), vec![term("saw")]),
            ],
        )
    }

    #[test]
    fn cfg_index_by_lhs() {
        let g = toy_cfg();
        let np_prods = g.productions(Some(&nt("NP")), None);
        assert_eq!(np_prods.len(), 2);
    }

    #[test]
    fn cfg_index_by_rhs0() {
        let g = toy_cfg();
        let starts_with_np = g.productions(None, Some(&ntr("NP")));
        assert_eq!(starts_with_np.len(), 1);
        assert_eq!(starts_with_np[0].lhs(), &nt("S"));
    }

    #[test]
    fn cfg_check_coverage_succeeds() {
        let g = toy_cfg();
        assert!(g.check_coverage(["the", "dog", "saw", "John"]).is_ok());
    }

    #[test]
    fn cfg_check_coverage_reports_missing() {
        let g = toy_cfg();
        let err = g.check_coverage(["the", "elephant"]).unwrap_err();
        assert_eq!(err, vec!["elephant".to_string()]);
    }

    #[test]
    fn cfg_kind_predicates() {
        let g = toy_cfg();
        assert!(g.is_nonempty());
        assert!(!g.is_lexical());
        assert!(!g.is_nonlexical());
        assert!(g.is_binarised());
        assert_eq!(g.min_len(), 1);
        assert_eq!(g.max_len(), 2);
    }

    // ---- PCFG ----

    #[test]
    fn pcfg_accepts_normalized() {
        let p1 =
            ProbabilisticProduction::new(Production::new(nt("S"), vec![ntr("NP"), ntr("VP")]), 1.0);
        let p2 = ProbabilisticProduction::new(Production::new(nt("NP"), vec![term("a")]), 0.4);
        let p3 = ProbabilisticProduction::new(Production::new(nt("NP"), vec![term("b")]), 0.6);
        let g = PCFG::new(nt("S"), vec![p1, p2, p3]).unwrap();
        assert_eq!(g.productions().len(), 3);
    }

    #[test]
    fn pcfg_rejects_unnormalized() {
        let p1 = ProbabilisticProduction::new(Production::new(nt("NP"), vec![term("a")]), 0.4);
        let p2 = ProbabilisticProduction::new(Production::new(nt("NP"), vec![term("b")]), 0.4);
        let err = PCFG::new(nt("NP"), vec![p1, p2]).unwrap_err();
        assert!(err.0.contains("sum to"));
    }

    #[test]
    fn induce_pcfg_normalizes_per_lhs() {
        // 4 NP -> 'a', 1 NP -> 'b'  ⟹  0.8 / 0.2
        let prods = vec![
            Production::new(nt("NP"), vec![term("a")]),
            Production::new(nt("NP"), vec![term("a")]),
            Production::new(nt("NP"), vec![term("a")]),
            Production::new(nt("NP"), vec![term("a")]),
            Production::new(nt("NP"), vec![term("b")]),
        ];
        let g = induce_pcfg(nt("NP"), &prods).unwrap();
        let mut got: BTreeMap<String, f64> = BTreeMap::new();
        for p in g.productions() {
            got.insert(p.production().to_string(), p.prob());
        }
        assert!((got["NP -> 'a'"] - 0.8).abs() < 1e-9);
        assert!((got["NP -> 'b'"] - 0.2).abs() < 1e-9);
    }

    // ---- text reader ----

    #[test]
    fn read_grammar_simple_cfg() {
        let g = read_grammar(
            "
            # toy
            S  -> NP VP
            NP -> DET N | 'John'
            DET -> 'the'
            N  -> 'dog' | 'cat'
            VP -> V NP
            V  -> 'saw'
            ",
            GrammarKind::Cfg,
            None,
        )
        .unwrap();
        match g {
            ReadGrammar::Cfg(cfg) => {
                assert_eq!(cfg.start(), &nt("S"));
                assert_eq!(cfg.productions(Some(&nt("N")), None).len(), 2);
                assert_eq!(cfg.productions(Some(&nt("NP")), None).len(), 2);
            }
            _ => panic!("expected CFG"),
        }
    }

    #[test]
    fn read_grammar_pcfg_normalized() {
        let g = read_grammar(
            "
            S  -> NP VP [1.0]
            NP -> DET N [0.6] | 'John' [0.4]
            DET -> 'the' [1.0]
            N  -> 'dog' [0.5] | 'cat' [0.5]
            VP -> V NP [1.0]
            V  -> 'saw' [1.0]
            ",
            GrammarKind::Pcfg,
            None,
        )
        .unwrap();
        match g {
            ReadGrammar::Pcfg(p) => {
                assert_eq!(p.productions().len(), 8);
            }
            _ => panic!("expected PCFG"),
        }
    }

    #[test]
    fn read_grammar_rejects_unmatched_quote() {
        let err = read_grammar("N -> 'dog", GrammarKind::Cfg, None).unwrap_err();
        assert!(err.0.contains("unterminated"));
    }

    // ---- feature grammar ----

    #[test]
    fn feature_nonterminal_symbol_uses_type() {
        let mut map: BTreeMap<String, FeatValue> = BTreeMap::new();
        map.insert(TYPE_FEATURE.into(), FeatValue::Text("NP".into()));
        map.insert("NUM".into(), FeatValue::Text("sg".into()));
        let n = FeatStructNonterminal::new(FeatStruct::Dict(map));
        assert_eq!(n.symbol(), "NP");
    }

    #[test]
    fn read_feature_grammar_basic() {
        let g = read_feature_grammar(
            "
            S -> NP VP
            NP -> DET N
            DET -> 'the'
            N -> 'dog' | 'cat'
            VP -> V NP
            V -> 'saw'
            ",
            None,
        )
        .unwrap();
        // start should be S
        assert_eq!(g.start().symbol(), "S");
        let n_prods = g.productions_for_type("N");
        assert_eq!(n_prods.len(), 2);
    }
}
