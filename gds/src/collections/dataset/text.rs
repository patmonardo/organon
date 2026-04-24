//! Text — port of NLTK `nltk/text.py`.
//!
//! ## Scope
//!
//! Ported:
//! - [`ConcordanceIndex`] — offsets per word and a word-window
//!   formatter ([`ConcordanceLine`]).
//! - [`ContextIndex`] — left/right context map for `similar` and
//!   `common_contexts`.
//! - [`Text`] — the user-facing object: indexing on demand,
//!   `concordance`, `similar`, `common_contexts`, `count`, `index`,
//!   `vocab`, `findall`, plus a `collocations` shortcut that drives
//!   [`super::collocations::BigramCollocationFinder`].
//! - [`TextCollection`] — a small bag of `Text`s with `tf`, `idf`,
//!   `tf_idf`.
//!
//! Deferred (not part of "back to basics"):
//! - `dispersion_plot`, `plot` (matplotlib),
//! - `Text.generate` (depends on a language model — see
//!   [`super::lm`] when revisited),
//! - `TokenSearcher` regex glue (the `findall` here uses simple
//!   regex backtracking on space-joined tokens).
//!
//! Sits at the dataset top level beside [`super::tree`] and
//! [`super::grammar`], completing the "text + tree + feature-grammar"
//! triple.

use std::collections::{BTreeMap, BTreeSet};

use crate::collections::dataset::collocations::{BigramAssocMeasures, BigramCollocationFinder};
use crate::collections::dataset::probability::FreqDist;

// ---------------------------------------------------------------------------
// Concordance
// ---------------------------------------------------------------------------

/// One concordance hit: the word position plus left/right context
/// strings. Mirrors NLTK `ConcordanceLine`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcordanceLine {
    pub offset: usize,
    pub left: String,
    pub query: String,
    pub right: String,
}

/// Offsets-per-word index supporting concordance lookups.
#[derive(Debug, Clone)]
pub struct ConcordanceIndex {
    tokens: Vec<String>,
    /// Word → ordered list of token offsets (case-folded).
    by_word: BTreeMap<String, Vec<usize>>,
    case_sensitive: bool,
}

impl ConcordanceIndex {
    pub fn new<I, S>(tokens: I, case_sensitive: bool) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tokens: Vec<String> = tokens.into_iter().map(Into::into).collect();
        let mut by_word: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, w) in tokens.iter().enumerate() {
            let key = if case_sensitive {
                w.clone()
            } else {
                w.to_lowercase()
            };
            by_word.entry(key).or_default().push(i);
        }
        Self {
            tokens,
            by_word,
            case_sensitive,
        }
    }

    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }

    /// Token offsets where `word` occurs. Empty if not present.
    pub fn offsets(&self, word: &str) -> Vec<usize> {
        let key = if self.case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };
        self.by_word.get(&key).cloned().unwrap_or_default()
    }

    /// Format up to `lines` concordance hits with `width` characters of
    /// surrounding context per side.
    pub fn find_concordance(&self, word: &str, width: usize, lines: usize) -> Vec<ConcordanceLine> {
        let offsets = self.offsets(word);
        let mut out = Vec::new();
        for off in offsets.into_iter().take(lines) {
            let left = self.left_context(off, width);
            let right = self.right_context(off, width);
            out.push(ConcordanceLine {
                offset: off,
                left,
                query: self.tokens[off].clone(),
                right,
            });
        }
        out
    }

    fn left_context(&self, off: usize, width: usize) -> String {
        let mut acc: Vec<&str> = Vec::new();
        let mut size = 0usize;
        for i in (0..off).rev() {
            let w = self.tokens[i].as_str();
            if size + w.len() + 1 > width && !acc.is_empty() {
                break;
            }
            size += w.len() + 1;
            acc.push(w);
        }
        acc.reverse();
        acc.join(" ")
    }

    fn right_context(&self, off: usize, width: usize) -> String {
        let mut acc: Vec<&str> = Vec::new();
        let mut size = 0usize;
        for i in (off + 1)..self.tokens.len() {
            let w = self.tokens[i].as_str();
            if size + w.len() + 1 > width && !acc.is_empty() {
                break;
            }
            size += w.len() + 1;
            acc.push(w);
        }
        acc.join(" ")
    }
}

// ---------------------------------------------------------------------------
// Context index
// ---------------------------------------------------------------------------

/// `(left, right)` neighbours per word, used by `similar` and
/// `common_contexts`. Mirrors NLTK `ContextIndex` with the default
/// "(prev_word, next_word)" context function.
#[derive(Debug, Clone)]
pub struct ContextIndex {
    /// word → multiset of contexts, with counts.
    word_to_contexts: BTreeMap<String, BTreeMap<(String, String), u64>>,
    /// context → multiset of words filling it.
    context_to_words: BTreeMap<(String, String), BTreeMap<String, u64>>,
    case_sensitive: bool,
}

const SENT_START: &str = "*START*";
const SENT_END: &str = "*END*";

impl ContextIndex {
    pub fn new<I, S>(tokens: I, case_sensitive: bool) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let tokens: Vec<String> = tokens.into_iter().map(Into::into).collect();
        let fold = |s: &str| -> String {
            if case_sensitive {
                s.to_string()
            } else {
                s.to_lowercase()
            }
        };
        let mut word_to_contexts: BTreeMap<String, BTreeMap<(String, String), u64>> =
            BTreeMap::new();
        let mut context_to_words: BTreeMap<(String, String), BTreeMap<String, u64>> =
            BTreeMap::new();
        for i in 0..tokens.len() {
            let w = fold(&tokens[i]);
            let left = if i == 0 {
                SENT_START.to_string()
            } else {
                fold(&tokens[i - 1])
            };
            let right = if i + 1 == tokens.len() {
                SENT_END.to_string()
            } else {
                fold(&tokens[i + 1])
            };
            let ctx = (left, right);
            *word_to_contexts
                .entry(w.clone())
                .or_default()
                .entry(ctx.clone())
                .or_insert(0) += 1;
            *context_to_words
                .entry(ctx)
                .or_default()
                .entry(w)
                .or_insert(0) += 1;
        }
        Self {
            word_to_contexts,
            context_to_words,
            case_sensitive,
        }
    }

    fn fold(&self, s: &str) -> String {
        if self.case_sensitive {
            s.to_string()
        } else {
            s.to_lowercase()
        }
    }

    /// Words that share the most contexts with `word`. Sorted by
    /// shared-context count, ties broken by word order.
    pub fn similar_words(&self, word: &str, num: usize) -> Vec<String> {
        let key = self.fold(word);
        let Some(contexts) = self.word_to_contexts.get(&key) else {
            return Vec::new();
        };
        let mut scores: BTreeMap<String, u64> = BTreeMap::new();
        for (ctx, cnt_for_word) in contexts {
            if let Some(words_in_ctx) = self.context_to_words.get(ctx) {
                for (w, cnt_for_w) in words_in_ctx {
                    if w == &key {
                        continue;
                    }
                    *scores.entry(w.clone()).or_insert(0) += (*cnt_for_word) * (*cnt_for_w);
                }
            }
        }
        let mut ranked: Vec<(String, u64)> = scores.into_iter().collect();
        ranked.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        ranked.into_iter().take(num).map(|(w, _)| w).collect()
    }

    /// Contexts shared by all words in `words` (intersection).
    /// Returns `[(context, total_count)]` sorted by descending count.
    pub fn common_contexts(&self, words: &[&str]) -> Vec<((String, String), u64)> {
        if words.is_empty() {
            return Vec::new();
        }
        let mut iter = words.iter().map(|w| {
            self.word_to_contexts
                .get(&self.fold(w))
                .map(|m| m.keys().cloned().collect::<BTreeSet<_>>())
                .unwrap_or_default()
        });
        let first = iter.next().unwrap();
        let intersection: BTreeSet<(String, String)> =
            iter.fold(first, |acc, s| acc.intersection(&s).cloned().collect());
        let mut out: Vec<((String, String), u64)> = intersection
            .into_iter()
            .map(|ctx| {
                let total: u64 = words
                    .iter()
                    .map(|w| {
                        self.word_to_contexts
                            .get(&self.fold(w))
                            .and_then(|m| m.get(&ctx))
                            .copied()
                            .unwrap_or(0)
                    })
                    .sum();
                (ctx, total)
            })
            .collect();
        out.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        out
    }
}

// ---------------------------------------------------------------------------
// Text
// ---------------------------------------------------------------------------

/// The high-level NLTK-style text object. Lazily builds concordance
/// and context indices on first access.
#[derive(Debug, Clone)]
pub struct Text {
    name: Option<String>,
    tokens: Vec<String>,
    concordance: std::cell::OnceCell<ConcordanceIndex>,
    context: std::cell::OnceCell<ContextIndex>,
    vocab: std::cell::OnceCell<FreqDist<String>>,
    case_sensitive: bool,
}

impl Text {
    pub fn new<I, S>(tokens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            name: None,
            tokens: tokens.into_iter().map(Into::into).collect(),
            concordance: std::cell::OnceCell::new(),
            context: std::cell::OnceCell::new(),
            vocab: std::cell::OnceCell::new(),
            case_sensitive: false,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_case_sensitive(mut self, on: bool) -> Self {
        self.case_sensitive = on;
        // Invalidate caches that depend on case folding.
        self.concordance = std::cell::OnceCell::new();
        self.context = std::cell::OnceCell::new();
        self.vocab = std::cell::OnceCell::new();
        self
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn tokens(&self) -> &[String] {
        &self.tokens
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn concordance_index(&self) -> &ConcordanceIndex {
        self.concordance
            .get_or_init(|| ConcordanceIndex::new(self.tokens.iter().cloned(), self.case_sensitive))
    }

    fn context_index(&self) -> &ContextIndex {
        self.context
            .get_or_init(|| ContextIndex::new(self.tokens.iter().cloned(), self.case_sensitive))
    }

    /// Up to `lines` concordance hits for `word`, each with up to
    /// `width` characters of context per side.
    pub fn concordance(&self, word: &str, width: usize, lines: usize) -> Vec<ConcordanceLine> {
        self.concordance_index()
            .find_concordance(word, width, lines)
    }

    /// Words that appear in similar contexts to `word`.
    pub fn similar(&self, word: &str, num: usize) -> Vec<String> {
        self.context_index().similar_words(word, num)
    }

    /// Contexts shared by all `words`, ordered by descending count.
    pub fn common_contexts(&self, words: &[&str]) -> Vec<((String, String), u64)> {
        self.context_index().common_contexts(words)
    }

    /// Number of times `word` appears (case folded).
    pub fn count(&self, word: &str) -> usize {
        let key = if self.case_sensitive {
            word.to_string()
        } else {
            word.to_lowercase()
        };
        self.tokens
            .iter()
            .filter(|t| {
                let t = if self.case_sensitive {
                    t.as_str().to_string()
                } else {
                    t.to_lowercase()
                };
                t == key
            })
            .count()
    }

    /// First offset of `word`, or `None` if absent.
    pub fn index(&self, word: &str) -> Option<usize> {
        self.concordance_index().offsets(word).into_iter().next()
    }

    /// Word frequency distribution (case folded when not
    /// `case_sensitive`).
    pub fn vocab(&self) -> &FreqDist<String> {
        self.vocab.get_or_init(|| {
            let mut fd = FreqDist::new();
            for t in &self.tokens {
                let k = if self.case_sensitive {
                    t.clone()
                } else {
                    t.to_lowercase()
                };
                fd.inc(k, 1);
            }
            fd
        })
    }

    /// Top bigram collocations by likelihood ratio, mirroring NLTK's
    /// default. Returns the bigrams as space-joined strings for easy
    /// printing, plus the ordered word pair.
    pub fn collocations(&self, num: usize, window_size: usize) -> Vec<(String, String)> {
        let bcf =
            BigramCollocationFinder::from_words_window(self.tokens.iter().cloned(), window_size);
        bcf.nbest(&BigramAssocMeasures::likelihood_ratio(), num)
    }

    /// Token-level regex search joined by single spaces. Returns the
    /// matched substrings (each a contiguous word run).
    pub fn findall(&self, pattern: &str) -> Result<Vec<String>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let joined = self.tokens.join(" ");
        Ok(re
            .find_iter(&joined)
            .map(|m| m.as_str().to_string())
            .collect())
    }
}

// ---------------------------------------------------------------------------
// TextCollection — tf / idf / tf-idf
// ---------------------------------------------------------------------------

/// A small bag of [`Text`]s used for tf–idf scoring. Mirrors NLTK
/// `TextCollection`.
#[derive(Debug, Clone)]
pub struct TextCollection {
    texts: Vec<Text>,
    /// Concatenation of all token streams — used for collection-wide
    /// term frequency.
    all_tokens: Vec<String>,
}

impl TextCollection {
    pub fn new(texts: Vec<Text>) -> Self {
        let all_tokens: Vec<String> = texts
            .iter()
            .flat_map(|t| t.tokens().iter().cloned())
            .collect();
        Self { texts, all_tokens }
    }

    pub fn texts(&self) -> &[Text] {
        &self.texts
    }

    /// Term frequency of `term` *within* a single text: count / |text|.
    pub fn tf(&self, term: &str, text: &Text) -> f64 {
        if text.is_empty() {
            return 0.0;
        }
        text.count(term) as f64 / text.len() as f64
    }

    /// Inverse document frequency: `ln(N / df)`. NLTK uses natural log.
    /// Returns `0.0` if no text contains the term (rather than panicking
    /// on division by zero) — caller should treat that as "unseen".
    pub fn idf(&self, term: &str) -> f64 {
        let df = self.texts.iter().filter(|t| t.count(term) > 0).count();
        if df == 0 {
            0.0
        } else {
            (self.texts.len() as f64 / df as f64).ln()
        }
    }

    pub fn tf_idf(&self, term: &str, text: &Text) -> f64 {
        self.tf(term, text) * self.idf(term)
    }

    /// Collection-wide term count (all texts concatenated).
    pub fn count(&self, term: &str) -> usize {
        let needle = term.to_lowercase();
        self.all_tokens
            .iter()
            .filter(|t| t.to_lowercase() == needle)
            .count()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn brown_like() -> Vec<&'static str> {
        // Tiny synthetic corpus with deliberate context overlaps.
        // "the dog barked" / "the cat barked" / "a dog ran"
        vec![
            "the", "dog", "barked", "the", "cat", "barked", "a", "dog", "ran",
        ]
    }

    // ---- ConcordanceIndex ----

    #[test]
    fn concordance_offsets_case_insensitive_by_default() {
        let ci = ConcordanceIndex::new(["The", "dog", "the", "cat"], false);
        assert_eq!(ci.offsets("the"), vec![0, 2]);
    }

    #[test]
    fn concordance_offsets_case_sensitive() {
        let ci = ConcordanceIndex::new(["The", "dog", "the", "cat"], true);
        assert_eq!(ci.offsets("the"), vec![2]);
        assert_eq!(ci.offsets("The"), vec![0]);
    }

    #[test]
    fn concordance_lines_have_left_query_right() {
        let ci = ConcordanceIndex::new(brown_like(), false);
        let lines = ci.find_concordance("dog", 80, 5);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].query, "dog");
        assert_eq!(lines[0].left, "the");
        assert_eq!(lines[0].right, "barked the cat barked a dog ran");
    }

    // ---- ContextIndex ----

    #[test]
    fn similar_finds_words_in_shared_contexts() {
        // "dog" and "cat" both appear in (the, barked).
        let ctx = ContextIndex::new(brown_like(), false);
        let sim = ctx.similar_words("dog", 5);
        assert!(sim.contains(&"cat".to_string()));
    }

    #[test]
    fn common_contexts_intersects() {
        let ctx = ContextIndex::new(brown_like(), false);
        let cc = ctx.common_contexts(&["dog", "cat"]);
        // (the, barked) is shared
        assert!(cc.iter().any(|((l, r), _)| l == "the" && r == "barked"));
    }

    // ---- Text ----

    #[test]
    fn text_count_index_vocab() {
        let t = Text::new(brown_like());
        assert_eq!(t.count("dog"), 2);
        assert_eq!(t.count("DOG"), 2); // case-folded
        assert_eq!(t.index("cat"), Some(4));
        assert_eq!(t.vocab().get(&"the".to_string()), 2);
    }

    #[test]
    fn text_concordance_via_text() {
        let t = Text::new(brown_like());
        let lines = t.concordance("dog", 80, 5);
        assert_eq!(lines.len(), 2);
    }

    #[test]
    fn text_similar_via_text() {
        let t = Text::new(brown_like());
        let s = t.similar("dog", 5);
        assert!(s.contains(&"cat".to_string()));
    }

    #[test]
    fn text_collocations_returns_pairs() {
        let t = Text::new(brown_like());
        let c = t.collocations(3, 2);
        assert!(!c.is_empty());
        // every returned pair must consist of tokens from the corpus
        let vocab: BTreeSet<&str> = brown_like().into_iter().collect();
        for (a, b) in &c {
            assert!(vocab.contains(a.as_str()));
            assert!(vocab.contains(b.as_str()));
        }
    }

    #[test]
    fn text_findall_uses_token_joined_regex() {
        let t = Text::new(brown_like());
        let m = t.findall(r"the \w+").unwrap();
        // two matches: "the dog", "the cat"
        assert_eq!(m, vec!["the dog".to_string(), "the cat".to_string()]);
    }

    #[test]
    fn text_with_name_round_trip() {
        let t = Text::new(["a"]).with_name("toy");
        assert_eq!(t.name(), Some("toy"));
    }

    // ---- TextCollection ----

    #[test]
    fn tf_idf_basic() {
        let t1 = Text::new(["a", "b", "c"]);
        let t2 = Text::new(["a", "a", "d"]);
        let t3 = Text::new(["e", "f", "g"]);
        let coll = TextCollection::new(vec![t1.clone(), t2.clone(), t3.clone()]);

        // tf("a", t2) = 2/3
        assert!((coll.tf("a", &t2) - 2.0 / 3.0).abs() < 1e-12);
        // df("a") = 2 ⟹ idf = ln(3/2)
        assert!((coll.idf("a") - (3.0_f64 / 2.0).ln()).abs() < 1e-12);
        // tf_idf("a", t2)
        let expected = (2.0 / 3.0) * (3.0_f64 / 2.0).ln();
        assert!((coll.tf_idf("a", &t2) - expected).abs() < 1e-12);
    }

    #[test]
    fn idf_zero_for_unseen() {
        let coll = TextCollection::new(vec![Text::new(["a"])]);
        assert_eq!(coll.idf("z"), 0.0);
    }
}
