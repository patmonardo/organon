//! N-gram language model utilities.

use rand::distributions::WeightedIndex;
use rand::prelude::{Distribution, SeedableRng, StdRng};
use std::collections::HashMap;

#[derive(Debug, thiserror::Error)]
pub enum LmError {
    #[error("cannot fit without vocabulary text or pre-seeded vocabulary")]
    MissingVocabulary,
    #[error("discount must be between 0 and 1 inclusive (got {0})")]
    InvalidDiscount(f64),
}

#[derive(Debug, Clone)]
pub struct Vocabulary {
    counts: HashMap<String, usize>,
    cutoff: usize,
    unk_label: String,
    len_cache: usize,
}

impl Vocabulary {
    pub fn new() -> Self {
        Self::with_cutoff(1, "<UNK>")
    }

    pub fn with_cutoff(unk_cutoff: usize, unk_label: impl Into<String>) -> Self {
        let cutoff = unk_cutoff.max(1);
        Self {
            counts: HashMap::new(),
            cutoff,
            unk_label: unk_label.into(),
            len_cache: 0,
        }
    }

    pub fn update<I>(&mut self, words: I)
    where
        I: IntoIterator<Item = String>,
    {
        for word in words {
            *self.counts.entry(word).or_insert(0) += 1;
        }
        self.refresh_len();
    }

    pub fn lookup_word<'a>(&'a self, word: &'a str) -> String {
        if self.contains(word) {
            word.to_string()
        } else {
            self.unk_label.clone()
        }
    }

    pub fn lookup_tokens(&self, tokens: &[String]) -> Vec<String> {
        tokens.iter().map(|t| self.lookup_word(t)).collect()
    }

    pub fn cutoff(&self) -> usize {
        self.cutoff
    }

    pub fn unk_label(&self) -> &str {
        &self.unk_label
    }

    pub fn contains(&self, word: &str) -> bool {
        if word == self.unk_label {
            return !self.counts.is_empty();
        }
        self.counts.get(word).copied().unwrap_or(0) >= self.cutoff
    }

    pub fn is_empty(&self) -> bool {
        self.len_cache == 0
    }

    pub fn len(&self) -> usize {
        self.len_cache
    }

    fn refresh_len(&mut self) {
        let mut count = 0;
        for (_word, freq) in &self.counts {
            if *freq >= self.cutoff {
                count += 1;
            }
        }
        if !self.counts.is_empty() {
            count += 1; // include unk label
        }
        self.len_cache = count;
    }
}

#[derive(Debug, Clone)]
pub struct NgramCounter {
    orders: HashMap<usize, HashMap<Vec<String>, HashMap<String, usize>>>,
    unigrams: HashMap<String, usize>,
}

impl NgramCounter {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
            unigrams: HashMap::new(),
        }
    }

    pub fn update<I, S>(&mut self, ngram_text: I)
    where
        I: IntoIterator<Item = S>,
        S: IntoIterator<Item = Vec<String>>,
    {
        for sentence in ngram_text {
            for ngram in sentence {
                self.update_ngram(&ngram);
            }
        }
    }

    pub fn update_ngram(&mut self, ngram: &[String]) {
        if ngram.is_empty() {
            return;
        }
        if ngram.len() == 1 {
            let word = &ngram[0];
            *self.unigrams.entry(word.clone()).or_insert(0) += 1;
            return;
        }

        let order = ngram.len();
        let context = ngram[..order - 1].to_vec();
        let word = ngram[order - 1].clone();
        let contexts = self.orders.entry(order).or_insert_with(HashMap::new);
        let counts = contexts.entry(context).or_insert_with(HashMap::new);
        *counts.entry(word).or_insert(0) += 1;
    }

    pub fn context_counts<'a>(&'a self, context: &[String]) -> ContextCounts<'a> {
        if context.is_empty() {
            return ContextCounts::new(Some(&self.unigrams));
        }
        let order = context.len() + 1;
        let counts = self
            .orders
            .get(&order)
            .and_then(|contexts| contexts.get(context));
        ContextCounts::new(counts)
    }

    pub fn order_counts(
        &self,
        order: usize,
    ) -> Option<&HashMap<Vec<String>, HashMap<String, usize>>> {
        self.orders.get(&order)
    }

    pub fn total_ngrams(&self) -> usize {
        let mut total = 0;
        total += self.unigrams.values().sum::<usize>();
        for contexts in self.orders.values() {
            for counts in contexts.values() {
                total += counts.values().sum::<usize>();
            }
        }
        total
    }

    pub fn unigrams(&self) -> &HashMap<String, usize> {
        &self.unigrams
    }
}

#[derive(Debug, Clone)]
pub struct ContextCounts<'a> {
    counts: Option<&'a HashMap<String, usize>>,
    total: usize,
}

impl<'a> ContextCounts<'a> {
    fn new(counts: Option<&'a HashMap<String, usize>>) -> Self {
        let total = counts
            .map(|counts| counts.values().sum::<usize>())
            .unwrap_or(0);
        Self { counts, total }
    }

    pub fn count(&self, word: &str) -> usize {
        self.counts
            .and_then(|counts| counts.get(word).copied())
            .unwrap_or(0)
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn freq(&self, word: &str) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.count(word) as f64 / self.total as f64
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }

    pub fn words(&self) -> Vec<String> {
        match self.counts {
            Some(counts) => counts.keys().cloned().collect(),
            None => Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LmBase {
    order: usize,
    vocab: Vocabulary,
    counts: NgramCounter,
}

impl LmBase {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            vocab: Vocabulary::new(),
            counts: NgramCounter::new(),
        }
    }

    pub fn with_vocab(order: usize, vocab: Vocabulary) -> Self {
        Self {
            order,
            vocab,
            counts: NgramCounter::new(),
        }
    }

    pub fn order(&self) -> usize {
        self.order
    }

    pub fn vocab(&self) -> &Vocabulary {
        &self.vocab
    }

    pub fn vocab_mut(&mut self) -> &mut Vocabulary {
        &mut self.vocab
    }

    pub fn counts(&self) -> &NgramCounter {
        &self.counts
    }

    pub fn counts_mut(&mut self) -> &mut NgramCounter {
        &mut self.counts
    }

    pub fn fit<I, S, V>(&mut self, text: I, vocab_text: Option<V>) -> Result<(), LmError>
    where
        I: IntoIterator<Item = S>,
        S: IntoIterator<Item = Vec<String>>,
        V: IntoIterator<Item = String>,
    {
        if self.vocab.is_empty() {
            if let Some(vocab_text) = vocab_text {
                self.vocab.update(vocab_text);
            } else {
                return Err(LmError::MissingVocabulary);
            }
        }
        for sentence in text {
            let mut masked = Vec::new();
            for ngram in sentence {
                masked.push(self.vocab.lookup_tokens(&ngram));
            }
            self.counts.update(std::iter::once(masked));
        }
        Ok(())
    }
}

pub trait LanguageModel {
    fn base(&self) -> &LmBase;
    fn base_mut(&mut self) -> &mut LmBase;
    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64;

    fn order(&self) -> usize {
        self.base().order()
    }

    fn vocab(&self) -> &Vocabulary {
        self.base().vocab()
    }

    fn counts(&self) -> &NgramCounter {
        self.base().counts()
    }

    fn fit<I, S, V>(&mut self, text: I, vocab_text: Option<V>) -> Result<(), LmError>
    where
        I: IntoIterator<Item = S>,
        S: IntoIterator<Item = Vec<String>>,
        V: IntoIterator<Item = String>,
    {
        self.base_mut().fit(text, vocab_text)
    }

    fn score(&self, word: &str, context: Option<&[String]>) -> f64 {
        let masked_word = self.vocab().lookup_word(word);
        let masked_context = context.map(|ctx| self.vocab().lookup_tokens(ctx));
        self.unmasked_score(&masked_word, masked_context.as_deref())
    }

    fn logscore(&self, word: &str, context: Option<&[String]>) -> f64 {
        let score = self.score(word, context);
        if score == 0.0 {
            f64::NEG_INFINITY
        } else {
            score.log2()
        }
    }

    fn context_counts(&self, context: &[String]) -> ContextCounts<'_> {
        self.counts().context_counts(context)
    }

    fn entropy<I>(&self, text_ngrams: I) -> f64
    where
        I: IntoIterator<Item = Vec<String>>,
    {
        let mut sum = 0.0;
        let mut count = 0;
        for ngram in text_ngrams {
            if ngram.is_empty() {
                continue;
            }
            let (context, word) = ngram.split_at(ngram.len() - 1);
            let logscore = self.logscore(&word[0], Some(context));
            sum += logscore;
            count += 1;
        }
        if count == 0 {
            return 0.0;
        }
        -sum / count as f64
    }

    fn perplexity<I>(&self, text_ngrams: I) -> f64
    where
        I: IntoIterator<Item = Vec<String>>,
    {
        (2.0_f64).powf(self.entropy(text_ngrams))
    }

    fn generate(&self, num_words: usize, seed: &[String], random_seed: Option<u64>) -> Vec<String> {
        let mut rng = match random_seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };
        let mut generated = Vec::new();
        for _ in 0..num_words {
            let mut context = seed.to_vec();
            context.extend(generated.iter().cloned());
            let context = if context.len() >= self.order() {
                context[context.len() - (self.order() - 1)..].to_vec()
            } else {
                context
            };
            let counts = self.context_counts(&context);
            if counts.is_empty() {
                break;
            }
            let mut samples = counts.words();
            samples.sort();
            let weights: Vec<f64> = samples
                .iter()
                .map(|w| self.score(w, Some(&context)))
                .collect();
            if weights.iter().all(|w| *w == 0.0) {
                break;
            }
            if let Ok(dist) = WeightedIndex::new(&weights) {
                let idx = dist.sample(&mut rng);
                generated.push(samples[idx].clone());
            } else {
                break;
            }
        }
        generated
    }
}

pub trait Smoothing {
    fn unigram_score(&self, vocab: &Vocabulary, counts: &NgramCounter, word: &str) -> f64;
    fn alpha_gamma(
        &self,
        vocab: &Vocabulary,
        counts: &NgramCounter,
        word: &str,
        context: &[String],
    ) -> (f64, f64);
}

#[derive(Debug, Clone, Copy)]
pub struct WittenBell;

impl Smoothing for WittenBell {
    fn unigram_score(&self, _vocab: &Vocabulary, counts: &NgramCounter, word: &str) -> f64 {
        counts.context_counts(&[]).freq(word)
    }

    fn alpha_gamma(
        &self,
        _vocab: &Vocabulary,
        counts: &NgramCounter,
        word: &str,
        context: &[String],
    ) -> (f64, f64) {
        let ctx_counts = counts.context_counts(context);
        let alpha = ctx_counts.freq(word);
        let gamma = gamma_from_counts(&ctx_counts);
        ((1.0 - gamma) * alpha, gamma)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AbsoluteDiscounting {
    pub discount: f64,
}

impl Smoothing for AbsoluteDiscounting {
    fn unigram_score(&self, _vocab: &Vocabulary, counts: &NgramCounter, word: &str) -> f64 {
        counts.context_counts(&[]).freq(word)
    }

    fn alpha_gamma(
        &self,
        _vocab: &Vocabulary,
        counts: &NgramCounter,
        word: &str,
        context: &[String],
    ) -> (f64, f64) {
        let ctx_counts = counts.context_counts(context);
        let total = ctx_counts.total() as f64;
        if total == 0.0 {
            return (0.0, 1.0);
        }
        let count = ctx_counts.count(word) as f64;
        let alpha = (count - self.discount).max(0.0) / total;
        let gamma = (self.discount * values_gt_zero(&ctx_counts) as f64) / total;
        (alpha, gamma)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KneserNey {
    pub discount: f64,
    pub order: usize,
}

impl Smoothing for KneserNey {
    fn unigram_score(&self, _vocab: &Vocabulary, counts: &NgramCounter, word: &str) -> f64 {
        let (continuation, total) = continuation_counts(counts, word, &[]);
        if total == 0 {
            0.0
        } else {
            continuation as f64 / total as f64
        }
    }

    fn alpha_gamma(
        &self,
        _vocab: &Vocabulary,
        counts: &NgramCounter,
        word: &str,
        context: &[String],
    ) -> (f64, f64) {
        let ctx_counts = counts.context_counts(context);
        let total = ctx_counts.total() as f64;
        if total == 0.0 {
            return (0.0, 1.0);
        }
        let (continuation, cont_total) = if context.len() + 1 == self.order {
            (ctx_counts.count(word), ctx_counts.total())
        } else {
            continuation_counts(counts, word, context)
        };
        let alpha = (continuation as f64 - self.discount).max(0.0) / cont_total as f64;
        let gamma = (self.discount * values_gt_zero(&ctx_counts) as f64) / total;
        (alpha, gamma)
    }
}

fn gamma_from_counts(counts: &ContextCounts<'_>) -> f64 {
    let n_plus = values_gt_zero(counts) as f64;
    let total = counts.total() as f64;
    if total == 0.0 {
        1.0
    } else {
        n_plus / (n_plus + total)
    }
}

fn values_gt_zero(counts: &ContextCounts<'_>) -> usize {
    match counts.counts {
        Some(map) => map.values().filter(|v| **v > 0).count(),
        None => 0,
    }
}

fn continuation_counts(counts: &NgramCounter, word: &str, context: &[String]) -> (usize, usize) {
    let order = context.len() + 2;
    let Some(order_map) = counts.order_counts(order) else {
        return (0, 0);
    };
    let mut continuation = 0;
    let mut total = 0;
    for (prefix, next_counts) in order_map {
        if prefix.len() < 1 {
            continue;
        }
        if prefix[1..] != *context {
            continue;
        }
        if next_counts.get(word).copied().unwrap_or(0) > 0 {
            continuation += 1;
        }
        total += next_counts.values().filter(|v| **v > 0).count();
    }
    (continuation, total)
}

#[derive(Debug, Clone)]
pub struct MLE {
    base: LmBase,
}

impl MLE {
    pub fn new(order: usize) -> Self {
        Self {
            base: LmBase::new(order),
        }
    }

    pub fn with_vocab(order: usize, vocab: Vocabulary) -> Self {
        Self {
            base: LmBase::with_vocab(order, vocab),
        }
    }
}

impl LanguageModel for MLE {
    fn base(&self) -> &LmBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LmBase {
        &mut self.base
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        match context {
            Some(ctx) => self.context_counts(ctx).freq(word),
            None => self.counts().context_counts(&[]).freq(word),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lidstone {
    base: LmBase,
    gamma: f64,
}

impl Lidstone {
    pub fn new(order: usize, gamma: f64) -> Self {
        Self {
            base: LmBase::new(order),
            gamma,
        }
    }

    pub fn with_vocab(order: usize, gamma: f64, vocab: Vocabulary) -> Self {
        Self {
            base: LmBase::with_vocab(order, vocab),
            gamma,
        }
    }
}

impl LanguageModel for Lidstone {
    fn base(&self) -> &LmBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LmBase {
        &mut self.base
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        let ctx_counts = match context {
            Some(ctx) => self.context_counts(ctx),
            None => self.counts().context_counts(&[]),
        };
        let total = ctx_counts.total() as f64;
        let count = ctx_counts.count(word) as f64;
        (count + self.gamma) / (total + self.vocab().len() as f64 * self.gamma)
    }
}

#[derive(Debug, Clone)]
pub struct Laplace {
    inner: Lidstone,
}

impl Laplace {
    pub fn new(order: usize) -> Self {
        Self {
            inner: Lidstone::new(order, 1.0),
        }
    }

    pub fn with_vocab(order: usize, vocab: Vocabulary) -> Self {
        Self {
            inner: Lidstone::with_vocab(order, 1.0, vocab),
        }
    }
}

impl LanguageModel for Laplace {
    fn base(&self) -> &LmBase {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut LmBase {
        self.inner.base_mut()
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        self.inner.unmasked_score(word, context)
    }
}

#[derive(Debug, Clone)]
pub struct StupidBackoff {
    base: LmBase,
    alpha: f64,
}

impl StupidBackoff {
    pub fn new(order: usize, alpha: f64) -> Self {
        Self {
            base: LmBase::new(order),
            alpha,
        }
    }

    pub fn with_vocab(order: usize, alpha: f64, vocab: Vocabulary) -> Self {
        Self {
            base: LmBase::with_vocab(order, vocab),
            alpha,
        }
    }
}

impl LanguageModel for StupidBackoff {
    fn base(&self) -> &LmBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LmBase {
        &mut self.base
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        let Some(ctx) = context else {
            return self.counts().context_counts(&[]).freq(word);
        };
        if ctx.is_empty() {
            return self.counts().context_counts(&[]).freq(word);
        }
        let ctx_counts = self.context_counts(ctx);
        if ctx_counts.count(word) > 0 {
            ctx_counts.freq(word)
        } else {
            let next_ctx = &ctx[1..];
            self.alpha * self.unmasked_score(word, Some(next_ctx))
        }
    }
}

#[derive(Debug, Clone)]
pub struct InterpolatedLanguageModel<S: Smoothing> {
    base: LmBase,
    smoothing: S,
}

impl<S: Smoothing> InterpolatedLanguageModel<S> {
    pub fn new(order: usize, smoothing: S) -> Self {
        Self {
            base: LmBase::new(order),
            smoothing,
        }
    }

    pub fn with_vocab(order: usize, smoothing: S, vocab: Vocabulary) -> Self {
        Self {
            base: LmBase::with_vocab(order, vocab),
            smoothing,
        }
    }
}

impl<S: Smoothing> LanguageModel for InterpolatedLanguageModel<S> {
    fn base(&self) -> &LmBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut LmBase {
        &mut self.base
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        let Some(ctx) = context else {
            return self
                .smoothing
                .unigram_score(self.vocab(), self.counts(), word);
        };
        if ctx.is_empty() {
            return self
                .smoothing
                .unigram_score(self.vocab(), self.counts(), word);
        }
        let ctx_counts = self.context_counts(ctx);
        if ctx_counts.is_empty() {
            let (alpha, gamma) = (0.0, 1.0);
            return alpha + gamma * self.unmasked_score(word, Some(&ctx[1..]));
        }
        let (alpha, gamma) = self
            .smoothing
            .alpha_gamma(self.vocab(), self.counts(), word, ctx);
        alpha + gamma * self.unmasked_score(word, Some(&ctx[1..]))
    }
}

#[derive(Debug, Clone)]
pub struct WittenBellInterpolated {
    inner: InterpolatedLanguageModel<WittenBell>,
}

impl WittenBellInterpolated {
    pub fn new(order: usize) -> Self {
        Self {
            inner: InterpolatedLanguageModel::new(order, WittenBell),
        }
    }

    pub fn with_vocab(order: usize, vocab: Vocabulary) -> Self {
        Self {
            inner: InterpolatedLanguageModel::with_vocab(order, WittenBell, vocab),
        }
    }
}

impl LanguageModel for WittenBellInterpolated {
    fn base(&self) -> &LmBase {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut LmBase {
        self.inner.base_mut()
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        self.inner.unmasked_score(word, context)
    }
}

#[derive(Debug, Clone)]
pub struct AbsoluteDiscountingInterpolated {
    inner: InterpolatedLanguageModel<AbsoluteDiscounting>,
}

impl AbsoluteDiscountingInterpolated {
    pub fn new(order: usize, discount: f64) -> Self {
        Self {
            inner: InterpolatedLanguageModel::new(order, AbsoluteDiscounting { discount }),
        }
    }

    pub fn with_vocab(order: usize, discount: f64, vocab: Vocabulary) -> Self {
        Self {
            inner: InterpolatedLanguageModel::with_vocab(
                order,
                AbsoluteDiscounting { discount },
                vocab,
            ),
        }
    }
}

impl LanguageModel for AbsoluteDiscountingInterpolated {
    fn base(&self) -> &LmBase {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut LmBase {
        self.inner.base_mut()
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        self.inner.unmasked_score(word, context)
    }
}

#[derive(Debug, Clone)]
pub struct KneserNeyInterpolated {
    inner: InterpolatedLanguageModel<KneserNey>,
}

impl KneserNeyInterpolated {
    pub fn new(order: usize, discount: f64) -> Result<Self, LmError> {
        if !(0.0..=1.0).contains(&discount) {
            return Err(LmError::InvalidDiscount(discount));
        }
        Ok(Self {
            inner: InterpolatedLanguageModel::new(order, KneserNey { discount, order }),
        })
    }

    pub fn with_vocab(order: usize, discount: f64, vocab: Vocabulary) -> Result<Self, LmError> {
        if !(0.0..=1.0).contains(&discount) {
            return Err(LmError::InvalidDiscount(discount));
        }
        Ok(Self {
            inner: InterpolatedLanguageModel::with_vocab(
                order,
                KneserNey { discount, order },
                vocab,
            ),
        })
    }
}

impl LanguageModel for KneserNeyInterpolated {
    fn base(&self) -> &LmBase {
        self.inner.base()
    }

    fn base_mut(&mut self) -> &mut LmBase {
        self.inner.base_mut()
    }

    fn unmasked_score(&self, word: &str, context: Option<&[String]>) -> f64 {
        self.inner.unmasked_score(word, context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collections::dataset::functions::model::preprocessing::padded_everygram_pipeline;

    fn to_text(input: &[&[&str]]) -> Vec<Vec<String>> {
        input
            .iter()
            .map(|sent| sent.iter().map(|w| (*w).to_string()).collect())
            .collect()
    }

    #[test]
    fn mle_fit_score_generate() {
        let text = to_text(&[&["a", "b", "c"], &["a", "c", "d", "c", "e", "f"]]);
        let (train, vocab) = padded_everygram_pipeline(2, &text);

        let mut lm = MLE::new(2);
        lm.fit(train, Some(vocab)).unwrap();

        let score = lm.score("a", None);
        assert!(score > 0.0);

        let seed = vec!["a".to_string()];
        let generated = lm.generate(3, &seed, Some(3));
        assert!(!generated.is_empty());
        assert!(generated.len() <= 3);
        for token in generated {
            assert!(lm.vocab().contains(&token) || token == lm.vocab().unk_label());
        }
    }
}
