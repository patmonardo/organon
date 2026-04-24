//! Collocation discovery — port of NLTK `nltk/collocations.py`
//! (and the bigram/trigram association measures from
//! `nltk/metrics/association.py`).
//!
//! ## Scope
//!
//! Ported:
//! - [`BigramCollocationFinder`] and [`TrigramCollocationFinder`] —
//!   build n-gram + marginal counts from a token stream with an
//!   optional sliding-window size for bigrams.
//! - [`BigramAssocMeasures`] — a small set of well-tested scorers:
//!   `raw_freq`, `pmi`, `student_t`, `chi_sq`, `likelihood_ratio`.
//! - [`TrigramAssocMeasures`] — `raw_freq`, `pmi`, `student_t`.
//! - Filters: minimum frequency, custom predicate.
//!
//! Deferred (rarely used and adds significant surface):
//! - QuadgramCollocationFinder,
//! - the long tail of fringe association measures
//!   (jaccard, dice, mi_like, fisher, …),
//! - contingency-table builders other than the bigram 2×2 used by the
//!   classical scorers.
//!
//! No external numerical dependency is used; the few specials we need
//! are implemented inline.

use std::collections::BTreeMap;

use crate::collections::dataset::probability::FreqDist;

// ---------------------------------------------------------------------------
// Bigram finder
// ---------------------------------------------------------------------------

/// Sliding-window bigram + unigram counts. Mirrors NLTK
/// `BigramCollocationFinder`.
#[derive(Debug, Clone)]
pub struct BigramCollocationFinder {
    word_fd: FreqDist<String>,
    bigram_fd: FreqDist<(String, String)>,
    /// Total bigram tokens — equal to `bigram_fd.total()` but kept
    /// explicit because filtering may shrink the map without touching
    /// the marginal count needed by the scorers.
    n_all: u64,
    window_size: usize,
}

impl BigramCollocationFinder {
    /// Build from an iterator of tokens with the default window (2 —
    /// adjacent pairs only).
    pub fn from_words<I, S>(tokens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::from_words_window(tokens, 2)
    }

    /// `window_size = N` counts every pair `(w_i, w_j)` with
    /// `0 < j - i < N`. NLTK default is 2.
    pub fn from_words_window<I, S>(tokens: I, window_size: usize) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        assert!(window_size >= 2, "window_size must be >= 2");
        let words: Vec<String> = tokens.into_iter().map(Into::into).collect();
        let mut word_fd = FreqDist::new();
        let mut bigram_fd = FreqDist::new();
        for w in &words {
            word_fd.inc(w.clone(), 1);
        }
        for (i, w1) in words.iter().enumerate() {
            let end = (i + window_size).min(words.len());
            for w2 in &words[i + 1..end] {
                bigram_fd.inc((w1.clone(), w2.clone()), 1);
            }
        }
        let n_all = bigram_fd.total();
        Self {
            word_fd,
            bigram_fd,
            n_all,
            window_size,
        }
    }

    pub fn window_size(&self) -> usize {
        self.window_size
    }

    pub fn word_fd(&self) -> &FreqDist<String> {
        &self.word_fd
    }

    pub fn ngram_fd(&self) -> &FreqDist<(String, String)> {
        &self.bigram_fd
    }

    /// Total bigram observations used as the sample size by scorers.
    pub fn n_all(&self) -> u64 {
        self.n_all
    }

    /// Drop bigrams whose count is below `min_freq`.
    pub fn apply_freq_filter(&mut self, min_freq: u64) {
        let drop: Vec<(String, String)> = self
            .bigram_fd
            .iter()
            .filter(|(_, c)| c < &min_freq)
            .map(|(k, _)| k.clone())
            .collect();
        // Rebuild with surviving entries (preserves total via inc).
        let mut new_fd = FreqDist::new();
        for (k, c) in self.bigram_fd.iter() {
            if !drop.contains(k) {
                new_fd.inc(k.clone(), c);
            }
        }
        self.bigram_fd = new_fd;
    }

    /// Drop bigrams matching `predicate(w1, w2)`.
    pub fn apply_ngram_filter<F: Fn(&str, &str) -> bool>(&mut self, predicate: F) {
        let mut new_fd = FreqDist::new();
        for (k, c) in self.bigram_fd.iter() {
            if !predicate(&k.0, &k.1) {
                new_fd.inc(k.clone(), c);
            }
        }
        self.bigram_fd = new_fd;
    }

    /// Return `(bigram, score)` pairs sorted by descending score.
    pub fn score_ngrams<S: BigramScorer>(&self, scorer: &S) -> Vec<((String, String), f64)> {
        let mut out: Vec<((String, String), f64)> = Vec::new();
        for (k, c) in self.bigram_fd.iter() {
            let n_iw = self.word_fd.get(&k.0);
            let n_wi = self.word_fd.get(&k.1);
            let s = scorer.score(c, n_iw, n_wi, self.n_all);
            out.push((k.clone(), s));
        }
        out.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        out
    }

    /// Top-`n` bigrams by `scorer`.
    pub fn nbest<S: BigramScorer>(&self, scorer: &S, n: usize) -> Vec<(String, String)> {
        self.score_ngrams(scorer)
            .into_iter()
            .take(n)
            .map(|(k, _)| k)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Bigram association measures
// ---------------------------------------------------------------------------

/// Trait implemented by bigram scorers consumed by
/// [`BigramCollocationFinder::score_ngrams`].
///
/// Inputs to `score`:
/// - `n_ii`: count of the bigram (`w1, w2`),
/// - `n_iw`: marginal count of `w1` (left side),
/// - `n_wi`: marginal count of `w2` (right side),
/// - `n_total`: total bigram observations (sample size).
pub trait BigramScorer {
    fn score(&self, n_ii: u64, n_iw: u64, n_wi: u64, n_total: u64) -> f64;
}

/// Marker types for the standard NLTK measures, grouped so callers
/// can write `BigramAssocMeasures::pmi()`.
pub struct BigramAssocMeasures;

impl BigramAssocMeasures {
    pub fn raw_freq() -> RawFreqBigram {
        RawFreqBigram
    }
    pub fn pmi() -> PmiBigram {
        PmiBigram
    }
    pub fn student_t() -> StudentTBigram {
        StudentTBigram
    }
    pub fn chi_sq() -> ChiSqBigram {
        ChiSqBigram
    }
    pub fn likelihood_ratio() -> LikelihoodRatioBigram {
        LikelihoodRatioBigram
    }
}

pub struct RawFreqBigram;
impl BigramScorer for RawFreqBigram {
    fn score(&self, n_ii: u64, _: u64, _: u64, n_total: u64) -> f64 {
        if n_total == 0 {
            0.0
        } else {
            n_ii as f64 / n_total as f64
        }
    }
}

pub struct PmiBigram;
impl BigramScorer for PmiBigram {
    fn score(&self, n_ii: u64, n_iw: u64, n_wi: u64, n_total: u64) -> f64 {
        // pmi = log2( N * n_ii / (n_iw * n_wi) )
        if n_ii == 0 || n_iw == 0 || n_wi == 0 || n_total == 0 {
            f64::NEG_INFINITY
        } else {
            ((n_total as f64) * (n_ii as f64) / ((n_iw as f64) * (n_wi as f64))).log2()
        }
    }
}

pub struct StudentTBigram;
impl BigramScorer for StudentTBigram {
    fn score(&self, n_ii: u64, n_iw: u64, n_wi: u64, n_total: u64) -> f64 {
        if n_total == 0 || n_ii == 0 {
            return 0.0;
        }
        let expected = (n_iw as f64) * (n_wi as f64) / (n_total as f64);
        (n_ii as f64 - expected) / (n_ii as f64).sqrt()
    }
}

pub struct ChiSqBigram;
impl BigramScorer for ChiSqBigram {
    fn score(&self, n_ii: u64, n_iw: u64, n_wi: u64, n_total: u64) -> f64 {
        // 2x2 contingency:
        //         w2     ¬w2
        //   w1   n_ii    n_io
        //  ¬w1   n_oi    n_oo
        if n_total == 0 {
            return 0.0;
        }
        let n_total = n_total as f64;
        let n_ii = n_ii as f64;
        let n_io = (n_iw as f64) - n_ii;
        let n_oi = (n_wi as f64) - n_ii;
        let n_oo = n_total - n_ii - n_io - n_oi;
        let denom = (n_ii + n_io) * (n_ii + n_oi) * (n_io + n_oo) * (n_oi + n_oo);
        if denom <= 0.0 {
            return 0.0;
        }
        let num = (n_ii * n_oo - n_io * n_oi).powi(2);
        n_total * num / denom
    }
}

pub struct LikelihoodRatioBigram;
impl BigramScorer for LikelihoodRatioBigram {
    fn score(&self, n_ii: u64, n_iw: u64, n_wi: u64, n_total: u64) -> f64 {
        // -2 ln λ for the bigram contingency table. Standard NLTK formula.
        if n_total == 0 {
            return 0.0;
        }
        let n = n_total as f64;
        let n_ii = n_ii as f64;
        let n_io = (n_iw as f64) - n_ii;
        let n_oi = (n_wi as f64) - n_ii;
        let n_oo = n - n_ii - n_io - n_oi;
        let row_sums = [n_ii + n_io, n_oi + n_oo];
        let col_sums = [n_ii + n_oi, n_io + n_oo];
        let observed = [[n_ii, n_io], [n_oi, n_oo]];
        let mut g = 0.0;
        for r in 0..2 {
            for c in 0..2 {
                let o = observed[r][c];
                if o <= 0.0 {
                    continue;
                }
                let e = row_sums[r] * col_sums[c] / n;
                if e <= 0.0 {
                    continue;
                }
                g += o * (o / e).ln();
            }
        }
        2.0 * g
    }
}

// ---------------------------------------------------------------------------
// Trigram finder
// ---------------------------------------------------------------------------

/// Trigram + bigram + unigram counts. Mirrors NLTK
/// `TrigramCollocationFinder`.
#[derive(Debug, Clone)]
pub struct TrigramCollocationFinder {
    word_fd: FreqDist<String>,
    /// Counts of `(w1, w2)` for every adjacent pair.
    bigram_fd: FreqDist<(String, String)>,
    /// Counts of skip-pair `(w1, w3)` over a 3-window — needed by
    /// the trigram likelihood-ratio family but kept simple here.
    wildcard_fd: FreqDist<(String, String)>,
    trigram_fd: FreqDist<(String, String, String)>,
    n_all: u64,
}

impl TrigramCollocationFinder {
    pub fn from_words<I, S>(tokens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let words: Vec<String> = tokens.into_iter().map(Into::into).collect();
        let mut word_fd = FreqDist::new();
        let mut bigram_fd = FreqDist::new();
        let mut wildcard_fd = FreqDist::new();
        let mut trigram_fd = FreqDist::new();
        for w in &words {
            word_fd.inc(w.clone(), 1);
        }
        for w in words.windows(2) {
            bigram_fd.inc((w[0].clone(), w[1].clone()), 1);
        }
        for w in words.windows(3) {
            wildcard_fd.inc((w[0].clone(), w[2].clone()), 1);
            trigram_fd.inc((w[0].clone(), w[1].clone(), w[2].clone()), 1);
        }
        let n_all = trigram_fd.total();
        Self {
            word_fd,
            bigram_fd,
            wildcard_fd,
            trigram_fd,
            n_all,
        }
    }

    pub fn word_fd(&self) -> &FreqDist<String> {
        &self.word_fd
    }

    pub fn bigram_fd(&self) -> &FreqDist<(String, String)> {
        &self.bigram_fd
    }

    pub fn wildcard_fd(&self) -> &FreqDist<(String, String)> {
        &self.wildcard_fd
    }

    pub fn ngram_fd(&self) -> &FreqDist<(String, String, String)> {
        &self.trigram_fd
    }

    pub fn n_all(&self) -> u64 {
        self.n_all
    }

    pub fn apply_freq_filter(&mut self, min_freq: u64) {
        let mut new_fd = FreqDist::new();
        for (k, c) in self.trigram_fd.iter() {
            if c >= min_freq {
                new_fd.inc(k.clone(), c);
            }
        }
        self.trigram_fd = new_fd;
    }

    pub fn score_ngrams<S: TrigramScorer>(
        &self,
        scorer: &S,
    ) -> Vec<((String, String, String), f64)> {
        let mut out: Vec<((String, String, String), f64)> = Vec::new();
        for (k, c) in self.trigram_fd.iter() {
            let n1 = self.word_fd.get(&k.0);
            let n2 = self.word_fd.get(&k.1);
            let n3 = self.word_fd.get(&k.2);
            let s = scorer.score(c, n1, n2, n3, self.n_all);
            out.push((k.clone(), s));
        }
        out.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.0.cmp(&b.0))
        });
        out
    }

    pub fn nbest<S: TrigramScorer>(&self, scorer: &S, n: usize) -> Vec<(String, String, String)> {
        self.score_ngrams(scorer)
            .into_iter()
            .take(n)
            .map(|(k, _)| k)
            .collect()
    }
}

pub trait TrigramScorer {
    fn score(&self, n_iii: u64, n1: u64, n2: u64, n3: u64, n_total: u64) -> f64;
}

pub struct TrigramAssocMeasures;

impl TrigramAssocMeasures {
    pub fn raw_freq() -> RawFreqTrigram {
        RawFreqTrigram
    }
    pub fn pmi() -> PmiTrigram {
        PmiTrigram
    }
    pub fn student_t() -> StudentTTrigram {
        StudentTTrigram
    }
}

pub struct RawFreqTrigram;
impl TrigramScorer for RawFreqTrigram {
    fn score(&self, n_iii: u64, _: u64, _: u64, _: u64, n_total: u64) -> f64 {
        if n_total == 0 {
            0.0
        } else {
            n_iii as f64 / n_total as f64
        }
    }
}

pub struct PmiTrigram;
impl TrigramScorer for PmiTrigram {
    fn score(&self, n_iii: u64, n1: u64, n2: u64, n3: u64, n_total: u64) -> f64 {
        if n_iii == 0 || n1 == 0 || n2 == 0 || n3 == 0 || n_total == 0 {
            f64::NEG_INFINITY
        } else {
            let n = n_total as f64;
            ((n.powi(2)) * (n_iii as f64) / ((n1 as f64) * (n2 as f64) * (n3 as f64))).log2()
        }
    }
}

pub struct StudentTTrigram;
impl TrigramScorer for StudentTTrigram {
    fn score(&self, n_iii: u64, n1: u64, n2: u64, n3: u64, n_total: u64) -> f64 {
        if n_total == 0 || n_iii == 0 {
            return 0.0;
        }
        let n = n_total as f64;
        let expected = (n1 as f64) * (n2 as f64) * (n3 as f64) / n.powi(2);
        (n_iii as f64 - expected) / (n_iii as f64).sqrt()
    }
}

// ---------------------------------------------------------------------------
// Helpers (kept private — exported through the scorers)
// ---------------------------------------------------------------------------

#[allow(dead_code)]
fn marginal_table(bigram_fd: &FreqDist<(String, String)>) -> BTreeMap<String, (u64, u64)> {
    // Returns {word: (count_as_left, count_as_right)} — unused by the
    // measures above but useful for diagnostics.
    let mut t: BTreeMap<String, (u64, u64)> = BTreeMap::new();
    for ((a, b), c) in bigram_fd.iter() {
        t.entry(a.clone()).or_default().0 += c;
        t.entry(b.clone()).or_default().1 += c;
    }
    t
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn toy() -> Vec<&'static str> {
        // Six tokens; bigram counts (window=2):
        // (a,b)=2, (b,c)=1, (c,a)=1, (a,b)=already, (b,a)=1
        vec!["a", "b", "c", "a", "b", "a"]
    }

    #[test]
    fn bigram_finder_basic_counts() {
        let bcf = BigramCollocationFinder::from_words(toy());
        // word counts: a=3, b=2, c=1
        assert_eq!(bcf.word_fd().get(&"a".to_string()), 3);
        assert_eq!(bcf.word_fd().get(&"b".to_string()), 2);
        assert_eq!(bcf.word_fd().get(&"c".to_string()), 1);
        // bigrams: (a,b),(b,c),(c,a),(a,b),(b,a)  → 5 total
        assert_eq!(bcf.n_all(), 5);
        assert_eq!(bcf.ngram_fd().get(&("a".into(), "b".into())), 2);
    }

    #[test]
    fn bigram_window_3_counts_skip_pairs() {
        let bcf = BigramCollocationFinder::from_words_window(["a", "b", "c"], 3);
        // pairs within distance < 3: (a,b),(a,c),(b,c)
        assert_eq!(bcf.n_all(), 3);
        assert_eq!(bcf.ngram_fd().get(&("a".into(), "c".into())), 1);
    }

    #[test]
    fn bigram_freq_filter_removes_low() {
        let mut bcf = BigramCollocationFinder::from_words(toy());
        bcf.apply_freq_filter(2);
        assert_eq!(bcf.ngram_fd().b(), 1); // only (a,b)=2 survives
    }

    #[test]
    fn bigram_ngram_filter_removes_match() {
        let mut bcf = BigramCollocationFinder::from_words(toy());
        bcf.apply_ngram_filter(|w1, _| w1 == "c");
        // (c,a) removed
        assert_eq!(bcf.ngram_fd().get(&("c".into(), "a".into())), 0);
    }

    #[test]
    fn pmi_high_for_pair_that_only_co_occurs() {
        let bcf = BigramCollocationFinder::from_words(["x", "y", "z", "z", "z"]);
        let pmi = BigramAssocMeasures::pmi();
        // (x,y) appears 1 time, x and y each occur once, n_total=4
        let s = bcf.score_ngrams(&pmi);
        let xy = s
            .iter()
            .find(|(k, _)| k == &("x".into(), "y".into()))
            .unwrap();
        assert!(xy.1 > 0.0);
    }

    #[test]
    fn raw_freq_matches_definition() {
        let bcf = BigramCollocationFinder::from_words(toy());
        let s = bcf.score_ngrams(&BigramAssocMeasures::raw_freq());
        let ab = s
            .iter()
            .find(|(k, _)| k == &("a".into(), "b".into()))
            .unwrap();
        assert!((ab.1 - 2.0 / 5.0).abs() < 1e-12);
    }

    #[test]
    fn nbest_returns_top_scores() {
        let bcf = BigramCollocationFinder::from_words(toy());
        let top = bcf.nbest(&BigramAssocMeasures::raw_freq(), 1);
        assert_eq!(top, vec![("a".into(), "b".into())]);
    }

    #[test]
    fn likelihood_ratio_nonneg_on_real_data() {
        let bcf = BigramCollocationFinder::from_words(toy());
        let s = bcf.score_ngrams(&BigramAssocMeasures::likelihood_ratio());
        for (_, v) in &s {
            assert!(*v >= -1e-9, "G^2 should be >= 0, got {}", v);
        }
    }

    #[test]
    fn chi_sq_nonneg() {
        let bcf = BigramCollocationFinder::from_words(toy());
        let s = bcf.score_ngrams(&BigramAssocMeasures::chi_sq());
        for (_, v) in &s {
            assert!(*v >= -1e-9);
        }
    }

    // ---- trigrams ----

    #[test]
    fn trigram_finder_counts_windows() {
        let tcf = TrigramCollocationFinder::from_words(["a", "b", "c", "a", "b", "c"]);
        // trigrams: (a,b,c),(b,c,a),(c,a,b),(a,b,c)
        assert_eq!(tcf.n_all(), 4);
        assert_eq!(tcf.ngram_fd().get(&("a".into(), "b".into(), "c".into())), 2);
        assert_eq!(tcf.bigram_fd().get(&("a".into(), "b".into())), 2);
        // skip pair (a, c) appears twice
        assert_eq!(tcf.wildcard_fd().get(&("a".into(), "c".into())), 2);
    }

    #[test]
    fn trigram_pmi_finite_for_observed() {
        let tcf = TrigramCollocationFinder::from_words(["a", "b", "c", "a", "b", "c"]);
        let s = tcf.score_ngrams(&TrigramAssocMeasures::pmi());
        for (_, v) in &s {
            assert!(v.is_finite());
        }
    }

    #[test]
    fn trigram_freq_filter() {
        let mut tcf = TrigramCollocationFinder::from_words(["a", "b", "c", "a", "b", "c"]);
        tcf.apply_freq_filter(2);
        assert_eq!(tcf.ngram_fd().b(), 1);
    }
}
