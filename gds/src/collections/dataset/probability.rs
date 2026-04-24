//! Probability distributions — port of NLTK `nltk/probability.py`.
//!
//! Faithfully translates the kernel-applicable surface of NLTK's
//! `probability` module. Plotting, tabulation, pickling, and Python
//! `__repr__` quirks are skipped; everything load-bearing for
//! probability and frequency estimation is preserved.
//!
//! Ported types:
//!
//! * [`FreqDist`], [`ConditionalFreqDist`]
//! * trait [`ProbDist`] with default `logprob`/`discount`/`generate`
//! * [`UniformProbDist`], [`RandomProbDist`], [`DictionaryProbDist`]
//! * [`MLEProbDist`], [`LidstoneProbDist`], [`LaplaceProbDist`],
//!   [`ELEProbDist`]
//! * [`HeldoutProbDist`], [`CrossValidationProbDist`]
//! * [`WittenBellProbDist`], [`SimpleGoodTuringProbDist`]
//! * [`MutableProbDist`], [`KneserNeyProbDist`]
//! * [`ConditionalProbDist`], [`DictionaryConditionalProbDist`]
//! * free functions [`add_logs`], [`sum_logs`], [`log_likelihood`],
//!   [`entropy`]
//!
//! Backed by `BTreeMap` for deterministic iteration.

use std::collections::BTreeMap;
use std::hash::Hash;

use rand::Rng;

/// Sentinel used by NLTK in place of true `-inf` when log-probabilities
/// would otherwise be undefined. NLTK literally uses `float("-1e300")`.
pub const NINF: f64 = -1e300;

// ===========================================================================
// FreqDist
// ===========================================================================

/// A counter over arbitrary keys. Mirrors NLTK `FreqDist`.
#[derive(Debug, Clone)]
pub struct FreqDist<K: Ord + Eq + Hash + Clone> {
    counts: BTreeMap<K, u64>,
    total: u64,
}

impl<K: Ord + Eq + Hash + Clone> Default for FreqDist<K> {
    fn default() -> Self {
        Self {
            counts: BTreeMap::new(),
            total: 0,
        }
    }
}

impl<K: Ord + Eq + Hash + Clone> FreqDist<K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_samples<I: IntoIterator<Item = K>>(samples: I) -> Self {
        let mut fd = Self::default();
        for s in samples {
            fd.inc(s, 1);
        }
        fd
    }

    /// Add `count` to the count of `key`.
    pub fn inc(&mut self, key: K, count: u64) {
        *self.counts.entry(key).or_insert(0) += count;
        self.total += count;
    }

    pub fn get(&self, key: &K) -> u64 {
        self.counts.get(key).copied().unwrap_or(0)
    }

    /// Sum of all counts (`N` in NLTK).
    pub fn total(&self) -> u64 {
        self.total
    }

    /// Alias for [`total`](Self::total) matching NLTK's `N()`.
    pub fn n(&self) -> u64 {
        self.total
    }

    /// Number of distinct samples (`B` in NLTK).
    pub fn b(&self) -> usize {
        self.counts.len()
    }

    pub fn freq(&self, key: &K) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.get(key) as f64 / self.total as f64
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, u64)> {
        self.counts.iter().map(|(k, &v)| (k, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.counts.keys()
    }

    /// Samples sorted by descending count, ties broken by key order.
    pub fn most_common(&self, n: Option<usize>) -> Vec<(K, u64)> {
        let mut v: Vec<(K, u64)> = self.counts.iter().map(|(k, &c)| (k.clone(), c)).collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        if let Some(n) = n {
            v.truncate(n);
        }
        v
    }

    /// Sample with the highest count; ties broken by key order.
    pub fn max(&self) -> Option<K> {
        self.most_common(Some(1)).into_iter().next().map(|(k, _)| k)
    }

    /// Samples that occur exactly once.
    pub fn hapaxes(&self) -> Vec<K> {
        self.counts
            .iter()
            .filter(|(_, &c)| c == 1)
            .map(|(k, _)| k.clone())
            .collect()
    }

    /// Number of sample values that occur exactly `r` times. Mirrors
    /// NLTK `FreqDist.Nr`.
    pub fn nr(&self, r: u64) -> u64 {
        if r == 0 {
            // Without a known bin count we cannot report unseen events.
            return 0;
        }
        self.counts.values().filter(|&&c| c == r).count() as u64
    }

    /// Map *r → Nr* (frequency-of-frequency). When `bins` is given,
    /// `Nr[0] = max(0, bins - B)` is set, otherwise the entry is absent
    /// (matching NLTK's behaviour where unseen-bin counts are unknown
    /// without external information).
    pub fn r_nr(&self, bins: Option<usize>) -> BTreeMap<u64, u64> {
        let mut out: BTreeMap<u64, u64> = BTreeMap::new();
        for &c in self.counts.values() {
            *out.entry(c).or_insert(0) += 1;
        }
        if let Some(bins) = bins {
            let b = self.b();
            if bins >= b {
                out.insert(0, (bins - b) as u64);
            }
        }
        out
    }

    /// Cumulative counts of `samples` in iteration order. Used for
    /// cumulative-frequency reporting, à la NLTK's
    /// `_cumulative_frequencies`.
    pub fn cumulative_frequencies(&self, samples: &[K]) -> Vec<u64> {
        let mut acc: u64 = 0;
        let mut out = Vec::with_capacity(samples.len());
        for s in samples {
            acc += self.get(s);
            out.push(acc);
        }
        out
    }
}

// ===========================================================================
// ProbDist trait
// ===========================================================================

/// Common interface for probability distributions over discrete
/// samples. Mirrors NLTK `ProbDistI`.
pub trait ProbDist<K: Clone + Eq + Hash + Ord> {
    /// Whether this distribution sums to one.
    fn sum_to_one(&self) -> bool {
        true
    }

    /// Probability of a sample.
    fn prob(&self, sample: &K) -> f64;

    /// Sample with the highest probability.
    fn max(&self) -> Option<K>;

    /// Samples for which `prob` may be non-zero.
    fn samples(&self) -> Vec<K>;

    /// Log₂-probability. Returns [`NINF`] when `prob == 0.0`.
    fn logprob(&self, sample: &K) -> f64 {
        let p = self.prob(sample);
        if p > 0.0 {
            p.log2()
        } else {
            NINF
        }
    }

    /// Mass reserved for events not observed in training.
    fn discount(&self) -> f64 {
        0.0
    }

    /// Roulette-wheel sampling.
    fn generate(&self) -> Option<K> {
        let samples = self.samples();
        if samples.is_empty() {
            return None;
        }
        let mut rng = rand::thread_rng();
        let p: f64 = rng.gen_range(0.0..1.0);
        let mut cum = 0.0;
        for s in &samples {
            cum += self.prob(s);
            if cum >= p {
                return Some(s.clone());
            }
        }
        samples.into_iter().last()
    }
}

// ===========================================================================
// UniformProbDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct UniformProbDist<K: Ord + Eq + Hash + Clone> {
    samples: Vec<K>,
}

impl<K: Ord + Eq + Hash + Clone> UniformProbDist<K> {
    pub fn new<I: IntoIterator<Item = K>>(samples: I) -> Self {
        let mut v: Vec<K> = samples.into_iter().collect();
        v.sort();
        v.dedup();
        Self { samples: v }
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for UniformProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        if self.samples.binary_search(sample).is_ok() {
            1.0 / self.samples.len() as f64
        } else {
            0.0
        }
    }

    fn max(&self) -> Option<K> {
        self.samples.first().cloned()
    }

    fn samples(&self) -> Vec<K> {
        self.samples.clone()
    }
}

// ===========================================================================
// RandomProbDist — assigns uniformly random masses that sum to 1.
// ===========================================================================

#[derive(Debug, Clone)]
pub struct RandomProbDist<K: Ord + Eq + Hash + Clone> {
    probs: BTreeMap<K, f64>,
}

impl<K: Ord + Eq + Hash + Clone> RandomProbDist<K> {
    pub fn new<I: IntoIterator<Item = K>>(samples: I) -> Self {
        let mut keys: Vec<K> = samples.into_iter().collect();
        keys.sort();
        keys.dedup();
        let mut rng = rand::thread_rng();
        let mut raw: Vec<f64> = (0..keys.len()).map(|_| rng.gen_range(0.0..1.0)).collect();
        let s: f64 = raw.iter().sum();
        if s > 0.0 {
            for v in &mut raw {
                *v /= s;
            }
        }
        let probs = keys.into_iter().zip(raw).collect();
        Self { probs }
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for RandomProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        self.probs.get(sample).copied().unwrap_or(0.0)
    }

    fn max(&self) -> Option<K> {
        self.probs
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, _)| k.clone())
    }

    fn samples(&self) -> Vec<K> {
        self.probs.keys().cloned().collect()
    }
}

// ===========================================================================
// DictionaryProbDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct DictionaryProbDist<K: Ord + Eq + Hash + Clone> {
    probs: BTreeMap<K, f64>,
    log: bool,
}

impl<K: Ord + Eq + Hash + Clone> DictionaryProbDist<K> {
    /// Create from explicit probabilities. When `log` is true the
    /// supplied values are interpreted as log₂-probabilities.
    pub fn new(probs: BTreeMap<K, f64>, log: bool) -> Self {
        Self { probs, log }
    }

    /// Renormalize so that the masses (in their respective space)
    /// sum to 1. Mirrors NLTK's `normalize=True` constructor option.
    pub fn normalized(mut self) -> Self {
        if self.log {
            let logs: Vec<f64> = self.probs.values().copied().collect();
            let lz = sum_logs(&logs);
            for v in self.probs.values_mut() {
                *v -= lz;
            }
        } else {
            let s: f64 = self.probs.values().sum();
            if s > 0.0 {
                for v in self.probs.values_mut() {
                    *v /= s;
                }
            }
        }
        self
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for DictionaryProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        match self.probs.get(sample) {
            Some(&v) => {
                if self.log {
                    2f64.powf(v)
                } else {
                    v
                }
            }
            None => 0.0,
        }
    }

    fn logprob(&self, sample: &K) -> f64 {
        match self.probs.get(sample) {
            Some(&v) => {
                if self.log {
                    v
                } else if v > 0.0 {
                    v.log2()
                } else {
                    NINF
                }
            }
            None => NINF,
        }
    }

    fn max(&self) -> Option<K> {
        self.probs
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(k, _)| k.clone())
    }

    fn samples(&self) -> Vec<K> {
        self.probs.keys().cloned().collect()
    }
}

// ===========================================================================
// MLEProbDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct MLEProbDist<K: Ord + Eq + Hash + Clone> {
    fd: FreqDist<K>,
}

impl<K: Ord + Eq + Hash + Clone> MLEProbDist<K> {
    pub fn new(fd: FreqDist<K>) -> Self {
        Self { fd }
    }

    pub fn freqdist(&self) -> &FreqDist<K> {
        &self.fd
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for MLEProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        self.fd.freq(sample)
    }

    fn max(&self) -> Option<K> {
        self.fd.max()
    }

    fn samples(&self) -> Vec<K> {
        self.fd.keys().cloned().collect()
    }
}

// ===========================================================================
// LidstoneProbDist (and Laplace / ELE specializations)
// ===========================================================================

#[derive(Debug, Clone)]
pub struct LidstoneProbDist<K: Ord + Eq + Hash + Clone> {
    fd: FreqDist<K>,
    gamma: f64,
    bins: usize,
    n: u64,
}

impl<K: Ord + Eq + Hash + Clone> LidstoneProbDist<K> {
    /// `bins` defaults to `fd.B()` when `None`. NLTK panics when
    /// `bins < B`; we mirror that with an `assert!`.
    pub fn new(fd: FreqDist<K>, gamma: f64, bins: Option<usize>) -> Self {
        let b = fd.b();
        let bins = bins.unwrap_or(b);
        assert!(
            bins >= b,
            "bins ({bins}) must not be less than B ({b}) of the freqdist"
        );
        let n = fd.total();
        Self { fd, gamma, bins, n }
    }

    pub fn freqdist(&self) -> &FreqDist<K> {
        &self.fd
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for LidstoneProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }

    fn prob(&self, sample: &K) -> f64 {
        let c = self.fd.get(sample) as f64;
        let denom = self.n as f64 + self.bins as f64 * self.gamma;
        if denom == 0.0 {
            0.0
        } else {
            (c + self.gamma) / denom
        }
    }

    fn max(&self) -> Option<K> {
        self.fd.max()
    }

    fn samples(&self) -> Vec<K> {
        self.fd.keys().cloned().collect()
    }

    fn discount(&self) -> f64 {
        let gb = self.gamma * self.bins as f64;
        let denom = self.n as f64 + gb;
        if denom == 0.0 {
            0.0
        } else {
            gb / denom
        }
    }
}

#[derive(Debug, Clone)]
pub struct LaplaceProbDist<K: Ord + Eq + Hash + Clone>(LidstoneProbDist<K>);

impl<K: Ord + Eq + Hash + Clone> LaplaceProbDist<K> {
    pub fn new(fd: FreqDist<K>, bins: Option<usize>) -> Self {
        Self(LidstoneProbDist::new(fd, 1.0, bins))
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for LaplaceProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }
    fn prob(&self, s: &K) -> f64 {
        self.0.prob(s)
    }
    fn max(&self) -> Option<K> {
        self.0.max()
    }
    fn samples(&self) -> Vec<K> {
        self.0.samples()
    }
    fn discount(&self) -> f64 {
        self.0.discount()
    }
}

#[derive(Debug, Clone)]
pub struct ELEProbDist<K: Ord + Eq + Hash + Clone>(LidstoneProbDist<K>);

impl<K: Ord + Eq + Hash + Clone> ELEProbDist<K> {
    pub fn new(fd: FreqDist<K>, bins: Option<usize>) -> Self {
        Self(LidstoneProbDist::new(fd, 0.5, bins))
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for ELEProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }
    fn prob(&self, s: &K) -> f64 {
        self.0.prob(s)
    }
    fn max(&self) -> Option<K> {
        self.0.max()
    }
    fn samples(&self) -> Vec<K> {
        self.0.samples()
    }
    fn discount(&self) -> f64 {
        self.0.discount()
    }
}

// ===========================================================================
// HeldoutProbDist
// ===========================================================================

/// `HeldoutProbDist` derives probabilities from two parallel
/// frequency distributions: a base and a heldout sample. Implements
/// the NLTK formula `_estimate[r] = Tr[r] / (Nr[r] * N)` where
/// `Tr[r]` is the total heldout count of items with base-count `r`.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HeldoutProbDist<K: Ord + Eq + Hash + Clone> {
    base: FreqDist<K>,
    heldout: FreqDist<K>,
    bins: usize,
    estimate: Vec<f64>,
    base_n: u64,
    base_b: usize,
}

impl<K: Ord + Eq + Hash + Clone> HeldoutProbDist<K> {
    pub fn new(base: FreqDist<K>, heldout: FreqDist<K>, bins: Option<usize>) -> Self {
        let base_b = base.b();
        let bins = bins.unwrap_or(base_b);
        assert!(bins >= base_b, "bins must be at least base.B()");

        let base_n = base.total();
        let heldout_n = heldout.total();

        // Maximum base count, so we can size `estimate` as a vector
        // indexed by integer count r.
        let max_r = base.iter().map(|(_, c)| c).max().unwrap_or(0) as usize;

        // r_nr[r] = number of base samples with count r.
        let mut r_nr = vec![0u64; max_r + 1];
        for (_, c) in base.iter() {
            r_nr[c as usize] += 1;
        }
        // r=0 bucket: items that appear in the base universe of size
        // `bins` but not in `base`.
        if bins >= base_b {
            r_nr[0] = (bins - base_b) as u64;
        }

        // Tr[r] = sum of heldout counts of items with base-count r.
        let mut tr = vec![0u64; max_r + 1];
        for (k, hc) in heldout.iter() {
            let r = base.get(k) as usize;
            if r < tr.len() {
                tr[r] += hc;
            }
        }
        // Heldout mass for items unseen in base also includes any
        // heldout-only samples (already covered above when r==0).
        let _ = heldout_n;

        // _estimate[r] = Tr[r] / (Nr[r] * N)
        let mut estimate = vec![0.0; max_r + 1];
        for r in 0..=max_r {
            let nr = r_nr[r] as f64;
            if nr > 0.0 && base_n > 0 {
                estimate[r] = tr[r] as f64 / (nr * base_n as f64);
            }
        }

        Self {
            base,
            heldout,
            bins,
            estimate,
            base_n,
            base_b,
        }
    }

    pub fn base_freqdist(&self) -> &FreqDist<K> {
        &self.base
    }

    pub fn heldout_freqdist(&self) -> &FreqDist<K> {
        &self.heldout
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for HeldoutProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }

    fn prob(&self, sample: &K) -> f64 {
        let r = self.base.get(sample) as usize;
        if r >= self.estimate.len() {
            // Out of range: treat as never-seen heldout estimate.
            0.0
        } else {
            self.estimate[r]
        }
    }

    fn max(&self) -> Option<K> {
        self.base.max()
    }

    fn samples(&self) -> Vec<K> {
        self.base.keys().cloned().collect()
    }

    fn discount(&self) -> f64 {
        // Mass given to unseen events: estimate at r = 0 multiplied by
        // the number of unseen bins.
        if self.estimate.is_empty() {
            return 0.0;
        }
        let unseen_bins = self.bins.saturating_sub(self.base_b) as f64;
        self.estimate[0] * unseen_bins
    }
}

impl<K: Ord + Eq + Hash + Clone> HeldoutProbDist<K> {
    /// Total heldout count attributed to base-count bucket `r`.
    pub fn tr(&self, r: usize) -> u64 {
        if r >= self.estimate.len() {
            0
        } else {
            // Recompute from heldout for transparency.
            self.heldout
                .iter()
                .filter(|(k, _)| self.base.get(k) as usize == r)
                .map(|(_, c)| c)
                .sum()
        }
    }
}

// ===========================================================================
// CrossValidationProbDist
// ===========================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CrossValidationProbDist<K: Ord + Eq + Hash + Clone> {
    fdists: Vec<FreqDist<K>>,
    bins: usize,
    heldouts: Vec<HeldoutProbDist<K>>,
    samples_set: Vec<K>,
}

impl<K: Ord + Eq + Hash + Clone> CrossValidationProbDist<K> {
    pub fn new(fdists: Vec<FreqDist<K>>, bins: Option<usize>) -> Self {
        // Mirror NLTK: build one HeldoutProbDist per ordered pair where
        // base = sum of others and heldout = fdists[i].
        let n = fdists.len();
        let mut samples_set: BTreeMap<K, ()> = BTreeMap::new();
        for fd in &fdists {
            for k in fd.keys() {
                samples_set.insert(k.clone(), ());
            }
        }
        let samples_vec: Vec<K> = samples_set.into_keys().collect();
        let bins = bins.unwrap_or(samples_vec.len());

        let mut heldouts = Vec::with_capacity(n.saturating_sub(1) * n);
        for i in 0..n {
            // Base = union of all fdists except i.
            let mut base = FreqDist::<K>::new();
            for (j, fd) in fdists.iter().enumerate() {
                if i == j {
                    continue;
                }
                for (k, c) in fd.iter() {
                    base.inc(k.clone(), c);
                }
            }
            heldouts.push(HeldoutProbDist::new(base, fdists[i].clone(), Some(bins)));
        }

        Self {
            fdists,
            bins,
            heldouts,
            samples_set: samples_vec,
        }
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for CrossValidationProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }

    fn prob(&self, sample: &K) -> f64 {
        if self.heldouts.is_empty() {
            return 0.0;
        }
        let s: f64 = self.heldouts.iter().map(|h| h.prob(sample)).sum();
        s / self.heldouts.len() as f64
    }

    fn max(&self) -> Option<K> {
        // Pick the most-probable sample under this distribution.
        self.samples_set.iter().cloned().max_by(|a, b| {
            self.prob(a)
                .partial_cmp(&self.prob(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    fn samples(&self) -> Vec<K> {
        self.samples_set.clone()
    }
}

// ===========================================================================
// WittenBellProbDist
// ===========================================================================

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WittenBellProbDist<K: Ord + Eq + Hash + Clone> {
    fd: FreqDist<K>,
    t: usize,
    z: usize,
    n: u64,
    p0: f64,
}

impl<K: Ord + Eq + Hash + Clone> WittenBellProbDist<K> {
    pub fn new(fd: FreqDist<K>, bins: Option<usize>) -> Self {
        let b = fd.b();
        let bins = bins.unwrap_or(b);
        assert!(bins >= b, "bins must be at least freqdist.B()");
        let t = b;
        let z = bins.saturating_sub(b);
        let n = fd.total();
        let p0 = if n == 0 {
            if z == 0 {
                0.0
            } else {
                1.0 / z as f64
            }
        } else if z == 0 {
            0.0
        } else {
            t as f64 / (z as f64 * (n as f64 + t as f64))
        };
        Self { fd, t, z, n, p0 }
    }

    pub fn freqdist(&self) -> &FreqDist<K> {
        &self.fd
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for WittenBellProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        let c = self.fd.get(sample);
        if c == 0 {
            self.p0
        } else {
            c as f64 / (self.n as f64 + self.t as f64)
        }
    }

    fn max(&self) -> Option<K> {
        self.fd.max()
    }

    fn samples(&self) -> Vec<K> {
        self.fd.keys().cloned().collect()
    }
}

// ===========================================================================
// SimpleGoodTuringProbDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct SimpleGoodTuringProbDist<K: Ord + Eq + Hash + Clone> {
    fd: FreqDist<K>,
    bins: usize,
    slope: f64,
    intercept: f64,
    switch_at: u64,
    renormal: f64,
}

impl<K: Ord + Eq + Hash + Clone> SimpleGoodTuringProbDist<K> {
    pub fn new(fd: FreqDist<K>, bins: Option<usize>) -> Self {
        let b = fd.b();
        let bins = bins.unwrap_or(b + 1);
        assert!(bins > b, "bins must be greater than freqdist.B()");

        let mut me = Self {
            fd,
            bins,
            slope: 0.0,
            intercept: 0.0,
            switch_at: 0,
            renormal: 1.0,
        };

        let (r, nr) = me.r_nr_pair();
        me.find_best_fit(&r, &nr);
        me.compute_switch(&r, &nr);
        me.renormalize(&r, &nr);
        me
    }

    fn r_nr_pair(&self) -> (Vec<u64>, Vec<u64>) {
        let r_nr = self.fd.r_nr(None); // Without bins: no Nr[0] entry.
        let mut r = Vec::new();
        let mut nr = Vec::new();
        for (k, v) in r_nr {
            if k == 0 {
                continue;
            }
            r.push(k);
            nr.push(v);
        }
        (r, nr)
    }

    fn find_best_fit(&mut self, r: &[u64], nr: &[u64]) {
        if r.is_empty() || nr.is_empty() {
            return;
        }
        // zr[j] = 2 * Nr[j] / (k - i)
        let mut zr = Vec::with_capacity(r.len());
        for j in 0..r.len() {
            let i = if j > 0 { r[j - 1] as f64 } else { 0.0 };
            let k = if j == r.len() - 1 {
                2.0 * r[j] as f64 - i
            } else {
                r[j + 1] as f64
            };
            zr.push(2.0 * nr[j] as f64 / (k - i));
        }

        let log_r: Vec<f64> = r.iter().map(|&v| (v as f64).ln()).collect();
        let log_zr: Vec<f64> = zr.iter().map(|&v| v.ln()).collect();

        let x_mean = log_r.iter().sum::<f64>() / log_r.len() as f64;
        let y_mean = log_zr.iter().sum::<f64>() / log_zr.len() as f64;

        let mut xy_cov = 0.0;
        let mut x_var = 0.0;
        for (x, y) in log_r.iter().zip(log_zr.iter()) {
            xy_cov += (x - x_mean) * (y - y_mean);
            x_var += (x - x_mean).powi(2);
        }
        self.slope = if x_var != 0.0 { xy_cov / x_var } else { 0.0 };
        self.intercept = y_mean - self.slope * x_mean;
    }

    fn compute_switch(&mut self, r: &[u64], nr: &[u64]) {
        for (i, &r_) in r.iter().enumerate() {
            if i + 1 == r.len() || r[i + 1] != r_ + 1 {
                self.switch_at = r_;
                return;
            }
            let smooth_r_star = (r_ as f64 + 1.0) * self.smoothed_nr(r_ + 1) / self.smoothed_nr(r_);
            let unsmooth_r_star = (r_ as f64 + 1.0) * nr[i + 1] as f64 / nr[i] as f64;
            let std = Self::variance(r_, nr[i], nr[i + 1]).sqrt();
            if (unsmooth_r_star - smooth_r_star).abs() <= 1.96 * std {
                self.switch_at = r_;
                return;
            }
        }
        if let Some(&last) = r.last() {
            self.switch_at = last;
        }
    }

    fn variance(r: u64, nr: u64, nr_1: u64) -> f64 {
        let r = r as f64;
        let nr = nr as f64;
        let nr_1 = nr_1 as f64;
        if nr == 0.0 {
            return 0.0;
        }
        (r + 1.0).powi(2) * (nr_1 / nr.powi(2)) * (1.0 + nr_1 / nr)
    }

    fn renormalize(&mut self, r: &[u64], nr: &[u64]) {
        let mut prob_cov = 0.0;
        for (&r_, &nr_) in r.iter().zip(nr.iter()) {
            prob_cov += nr_ as f64 * self.prob_measure(r_);
        }
        if prob_cov > 0.0 {
            self.renormal = (1.0 - self.prob_measure(0)) / prob_cov;
        } else {
            self.renormal = 1.0;
        }
    }

    /// Smoothed Nr from the regression line.
    pub fn smoothed_nr(&self, r: u64) -> f64 {
        (self.intercept + self.slope * (r as f64).ln()).exp()
    }

    fn prob_measure(&self, count: u64) -> f64 {
        let n = self.fd.total();
        if count == 0 && n == 0 {
            return 1.0;
        }
        if count == 0 && n != 0 {
            return self.fd.nr(1) as f64 / n as f64;
        }
        let (er_1, er) = if self.switch_at > count {
            (self.fd.nr(count + 1) as f64, self.fd.nr(count) as f64)
        } else {
            (self.smoothed_nr(count + 1), self.smoothed_nr(count))
        };
        if er == 0.0 {
            return 0.0;
        }
        let r_star = (count as f64 + 1.0) * er_1 / er;
        r_star / n as f64
    }

    pub fn freqdist(&self) -> &FreqDist<K> {
        &self.fd
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for SimpleGoodTuringProbDist<K> {
    fn sum_to_one(&self) -> bool {
        false
    }

    fn prob(&self, sample: &K) -> f64 {
        let count = self.fd.get(sample);
        let p = self.prob_measure(count);
        if count == 0 {
            if self.bins == self.fd.b() {
                0.0
            } else {
                p / (self.bins - self.fd.b()) as f64
            }
        } else {
            p * self.renormal
        }
    }

    fn max(&self) -> Option<K> {
        self.fd.max()
    }

    fn samples(&self) -> Vec<K> {
        self.fd.keys().cloned().collect()
    }

    fn discount(&self) -> f64 {
        let n = self.fd.total();
        if n == 0 {
            0.0
        } else {
            self.smoothed_nr(1) / n as f64
        }
    }
}

// ===========================================================================
// MutableProbDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct MutableProbDist<K: Ord + Eq + Hash + Clone> {
    samples: Vec<K>,
    index: BTreeMap<K, usize>,
    data: Vec<f64>,
    logs: bool,
}

impl<K: Ord + Eq + Hash + Clone> MutableProbDist<K> {
    pub fn new<P: ProbDist<K>>(prob_dist: &P, samples: Vec<K>, store_logs: bool) -> Self {
        let mut index = BTreeMap::new();
        let mut data = Vec::with_capacity(samples.len());
        for (i, s) in samples.iter().enumerate() {
            index.insert(s.clone(), i);
            data.push(if store_logs {
                prob_dist.logprob(s)
            } else {
                prob_dist.prob(s)
            });
        }
        Self {
            samples,
            index,
            data,
            logs: store_logs,
        }
    }

    /// Update the value for `sample`. If `log` is true, `value` is
    /// already a log₂-probability.
    pub fn update(&mut self, sample: &K, value: f64, log: bool) {
        let i = self
            .index
            .get(sample)
            .copied()
            .expect("sample not in MutableProbDist universe");
        self.data[i] = if self.logs {
            if log {
                value
            } else {
                if value > 0.0 {
                    value.log2()
                } else {
                    NINF
                }
            }
        } else if log {
            2f64.powf(value)
        } else {
            value
        };
    }
}

impl<K: Ord + Eq + Hash + Clone> ProbDist<K> for MutableProbDist<K> {
    fn prob(&self, sample: &K) -> f64 {
        match self.index.get(sample) {
            Some(&i) => {
                if self.logs {
                    2f64.powf(self.data[i])
                } else {
                    self.data[i]
                }
            }
            None => 0.0,
        }
    }

    fn logprob(&self, sample: &K) -> f64 {
        match self.index.get(sample) {
            Some(&i) => {
                if self.logs {
                    self.data[i]
                } else if self.data[i] > 0.0 {
                    self.data[i].log2()
                } else {
                    NINF
                }
            }
            None => NINF,
        }
    }

    fn max(&self) -> Option<K> {
        self.index
            .iter()
            .max_by(|a, b| {
                self.data[*a.1]
                    .partial_cmp(&self.data[*b.1])
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(k, _)| k.clone())
    }

    fn samples(&self) -> Vec<K> {
        self.samples.clone()
    }
}

// ===========================================================================
// KneserNeyProbDist
// ===========================================================================

/// Kneser-Ney smoothing over trigrams. The sample type is a triple.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct KneserNeyProbDist<W: Ord + Eq + Hash + Clone> {
    bins: usize,
    discount: f64,
    trigrams: FreqDist<(W, W, W)>,
    bigrams: BTreeMap<(W, W), u64>,
    wordtypes_after: BTreeMap<(W, W), u64>,
    trigrams_contain: BTreeMap<W, u64>,
    wordtypes_before: BTreeMap<(W, W), u64>,
}

impl<W: Ord + Eq + Hash + Clone> KneserNeyProbDist<W> {
    pub fn new(trigrams: FreqDist<(W, W, W)>, bins: Option<usize>, discount: f64) -> Self {
        let bins = bins.unwrap_or_else(|| trigrams.b());
        let mut bigrams: BTreeMap<(W, W), u64> = BTreeMap::new();
        let mut wa: BTreeMap<(W, W), u64> = BTreeMap::new();
        let mut tc: BTreeMap<W, u64> = BTreeMap::new();
        let mut wb: BTreeMap<(W, W), u64> = BTreeMap::new();
        for ((w0, w1, w2), c) in trigrams.iter() {
            *bigrams.entry((w0.clone(), w1.clone())).or_insert(0) += c;
            *wa.entry((w0.clone(), w1.clone())).or_insert(0) += 1;
            *tc.entry(w1.clone()).or_insert(0) += 1;
            *wb.entry((w1.clone(), w2.clone())).or_insert(0) += 1;
        }
        Self {
            bins,
            discount,
            trigrams,
            bigrams,
            wordtypes_after: wa,
            trigrams_contain: tc,
            wordtypes_before: wb,
        }
    }

    pub fn set_discount(&mut self, d: f64) {
        self.discount = d;
    }

    pub fn discount_value(&self) -> f64 {
        self.discount
    }
}

impl<W: Ord + Eq + Hash + Clone> ProbDist<(W, W, W)> for KneserNeyProbDist<W> {
    fn prob(&self, trigram: &(W, W, W)) -> f64 {
        let (w0, w1, w2) = trigram;
        let bigram = (w0.clone(), w1.clone());

        if self.trigrams.get(trigram) > 0 {
            let c = self.trigrams.get(trigram) as f64;
            let b = *self.bigrams.get(&bigram).unwrap_or(&0) as f64;
            if b == 0.0 {
                return 0.0;
            }
            return (c - self.discount) / b;
        }

        let after_key = (w1.clone(), w2.clone());
        if self.bigrams.contains_key(&bigram) && self.wordtypes_before.contains_key(&after_key) {
            let aftr = *self.wordtypes_after.get(&bigram).unwrap_or(&0) as f64;
            let bfr = *self.wordtypes_before.get(&after_key).unwrap_or(&0) as f64;
            let bcount = *self.bigrams.get(&bigram).unwrap_or(&0) as f64;
            let tcount = *self.trigrams_contain.get(w1).unwrap_or(&0) as f64;
            if bcount == 0.0 || (tcount - aftr) == 0.0 {
                return 0.0;
            }
            let leftover = (aftr * self.discount) / bcount;
            let beta = bfr / (tcount - aftr);
            return leftover * beta;
        }

        0.0
    }

    fn discount(&self) -> f64 {
        self.discount
    }

    fn samples(&self) -> Vec<(W, W, W)> {
        self.trigrams.keys().cloned().collect()
    }

    fn max(&self) -> Option<(W, W, W)> {
        self.trigrams.max()
    }
}

// ===========================================================================
// ConditionalFreqDist
// ===========================================================================

#[derive(Debug, Clone)]
pub struct ConditionalFreqDist<C: Ord + Eq + Hash + Clone, K: Ord + Eq + Hash + Clone> {
    by_condition: BTreeMap<C, FreqDist<K>>,
}

impl<C: Ord + Eq + Hash + Clone, K: Ord + Eq + Hash + Clone> Default for ConditionalFreqDist<C, K> {
    fn default() -> Self {
        Self {
            by_condition: BTreeMap::new(),
        }
    }
}

impl<C: Ord + Eq + Hash + Clone, K: Ord + Eq + Hash + Clone> ConditionalFreqDist<C, K> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pairs<I: IntoIterator<Item = (C, K)>>(pairs: I) -> Self {
        let mut cfd = Self::default();
        for (c, k) in pairs {
            cfd.inc(c, k, 1);
        }
        cfd
    }

    pub fn inc(&mut self, condition: C, key: K, count: u64) {
        self.by_condition
            .entry(condition)
            .or_default()
            .inc(key, count);
    }

    pub fn get(&self, condition: &C) -> Option<&FreqDist<K>> {
        self.by_condition.get(condition)
    }

    pub fn get_or_empty(&self, condition: &C) -> FreqDist<K> {
        self.by_condition
            .get(condition)
            .cloned()
            .unwrap_or_default()
    }

    pub fn conditions(&self) -> Vec<&C> {
        self.by_condition.keys().collect()
    }

    /// Total of all conditional counts.
    pub fn n(&self) -> u64 {
        self.by_condition.values().map(|fd| fd.total()).sum()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&C, &FreqDist<K>)> {
        self.by_condition.iter()
    }

    pub fn len(&self) -> usize {
        self.by_condition.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_condition.is_empty()
    }
}

// ===========================================================================
// ConditionalProbDist
// ===========================================================================

/// Builds a probability distribution per condition by applying a
/// `factory` closure to each condition's `FreqDist`.
pub struct ConditionalProbDist<C, K, P>
where
    C: Ord + Eq + Hash + Clone,
    K: Ord + Eq + Hash + Clone,
    P: ProbDist<K>,
{
    by_condition: BTreeMap<C, P>,
    _phantom: std::marker::PhantomData<K>,
}

impl<C, K, P> ConditionalProbDist<C, K, P>
where
    C: Ord + Eq + Hash + Clone,
    K: Ord + Eq + Hash + Clone,
    P: ProbDist<K>,
{
    pub fn new<F>(cfd: &ConditionalFreqDist<C, K>, factory: F) -> Self
    where
        F: Fn(FreqDist<K>) -> P,
    {
        let mut by = BTreeMap::new();
        for (cond, fd) in cfd.iter() {
            by.insert(cond.clone(), factory(fd.clone()));
        }
        Self {
            by_condition: by,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, cond: &C) -> Option<&P> {
        self.by_condition.get(cond)
    }

    pub fn conditions(&self) -> Vec<&C> {
        self.by_condition.keys().collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&C, &P)> {
        self.by_condition.iter()
    }

    pub fn len(&self) -> usize {
        self.by_condition.len()
    }
}

/// Wraps a pre-built dictionary of `ProbDist`s by condition.
pub struct DictionaryConditionalProbDist<C, K, P>
where
    C: Ord + Eq + Hash + Clone,
    K: Ord + Eq + Hash + Clone,
    P: ProbDist<K>,
{
    by_condition: BTreeMap<C, P>,
    _phantom: std::marker::PhantomData<K>,
}

impl<C, K, P> DictionaryConditionalProbDist<C, K, P>
where
    C: Ord + Eq + Hash + Clone,
    K: Ord + Eq + Hash + Clone,
    P: ProbDist<K>,
{
    pub fn new(probdists: BTreeMap<C, P>) -> Self {
        Self {
            by_condition: probdists,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, cond: &C) -> Option<&P> {
        self.by_condition.get(cond)
    }

    pub fn conditions(&self) -> Vec<&C> {
        self.by_condition.keys().collect()
    }
}

// ===========================================================================
// log-space arithmetic & info-theoretic free functions
// ===========================================================================

/// Threshold below which log-space addition collapses to the maximum,
/// equal to NLTK's `_ADD_LOGS_MAX_DIFF = log_2(1e-30)`.
fn add_logs_max_diff() -> f64 {
    1e-30_f64.log2()
}

/// log₂(2^logx + 2^logy) computed without overflow.
pub fn add_logs(logx: f64, logy: f64) -> f64 {
    let m = add_logs_max_diff();
    if logx < logy + m {
        return logy;
    }
    if logy < logx + m {
        return logx;
    }
    let base = logx.min(logy);
    base + (2f64.powf(logx - base) + 2f64.powf(logy - base)).log2()
}

/// log₂ of the sum of `2^l` for each `l` in `logs`. Empty input
/// yields [`NINF`].
pub fn sum_logs(logs: &[f64]) -> f64 {
    if logs.is_empty() {
        return NINF;
    }
    let mut acc = logs[0];
    for &l in &logs[1..] {
        acc = add_logs(acc, l);
    }
    acc
}

/// Sum over the union of samples of `actual.prob(s) * log_2(test.prob(s))`.
pub fn log_likelihood<P, Q, K>(test: &P, actual: &Q) -> f64
where
    P: ProbDist<K>,
    Q: ProbDist<K>,
    K: Clone + Eq + Hash + Ord,
{
    let mut total = 0.0;
    for s in actual.samples() {
        let p_test = test.prob(&s);
        let p_actual = actual.prob(&s);
        if p_test > 0.0 {
            total += p_actual * p_test.log2();
        } else {
            total += p_actual * NINF;
        }
    }
    total
}

/// Shannon entropy in bits.
pub fn entropy<P, K>(pdist: &P) -> f64
where
    P: ProbDist<K>,
    K: Clone + Eq + Hash + Ord,
{
    let mut h = 0.0;
    for s in pdist.samples() {
        let p = pdist.prob(&s);
        if p > 0.0 {
            h -= p * p.log2();
        }
    }
    h
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- FreqDist ----

    #[test]
    fn freqdist_basic_counts() {
        let fd = FreqDist::from_samples(["a", "b", "a", "c", "a", "b"]);
        assert_eq!(fd.get(&"a"), 3);
        assert_eq!(fd.get(&"b"), 2);
        assert_eq!(fd.get(&"c"), 1);
        assert_eq!(fd.get(&"z"), 0);
        assert_eq!(fd.total(), 6);
        assert_eq!(fd.n(), 6);
        assert_eq!(fd.b(), 3);
    }

    #[test]
    fn freqdist_freq_and_max() {
        let fd = FreqDist::from_samples(["a", "a", "b"]);
        assert!((fd.freq(&"a") - 2.0 / 3.0).abs() < 1e-12);
        assert_eq!(fd.max(), Some("a"));
    }

    #[test]
    fn freqdist_most_common_ordering() {
        let fd = FreqDist::from_samples(["b", "a", "a", "b", "c"]);
        let mc = fd.most_common(Some(2));
        assert_eq!(mc, vec![("a", 2), ("b", 2)]);
    }

    #[test]
    fn freqdist_hapaxes() {
        let fd = FreqDist::from_samples(["a", "a", "b", "c"]);
        assert_eq!(fd.hapaxes(), vec!["b", "c"]);
    }

    #[test]
    fn freqdist_nr_and_r_nr() {
        let fd = FreqDist::from_samples(["a", "a", "b", "b", "c"]);
        // counts: a=2, b=2, c=1 → Nr[1]=1, Nr[2]=2
        assert_eq!(fd.nr(1), 1);
        assert_eq!(fd.nr(2), 2);
        let r_nr = fd.r_nr(Some(5));
        assert_eq!(r_nr.get(&0), Some(&2)); // 5 - 3 = 2 unseen
        assert_eq!(r_nr.get(&1), Some(&1));
        assert_eq!(r_nr.get(&2), Some(&2));
    }

    #[test]
    fn freqdist_cumulative_frequencies() {
        let fd = FreqDist::from_samples(["a", "a", "b", "c"]);
        let cf = fd.cumulative_frequencies(&["a", "b", "c"]);
        assert_eq!(cf, vec![2, 3, 4]);
    }

    // ---- ConditionalFreqDist ----

    #[test]
    fn cfd_per_condition() {
        let cfd = ConditionalFreqDist::from_pairs([
            ("noun", "dog"),
            ("noun", "cat"),
            ("noun", "dog"),
            ("verb", "ran"),
        ]);
        assert_eq!(cfd.get(&"noun").unwrap().get(&"dog"), 2);
        assert_eq!(cfd.get(&"verb").unwrap().get(&"ran"), 1);
        assert_eq!(cfd.n(), 4);
        assert_eq!(cfd.conditions(), vec![&"noun", &"verb"]);
        assert_eq!(cfd.len(), 2);
    }

    // ---- UniformProbDist ----

    #[test]
    fn uniform_probdist_basic() {
        let u = UniformProbDist::new(["a", "b", "c"]);
        assert!((u.prob(&"a") - 1.0 / 3.0).abs() < 1e-12);
        assert_eq!(u.prob(&"z"), 0.0);
        assert_eq!(u.samples().len(), 3);
    }

    // ---- DictionaryProbDist ----

    #[test]
    fn dictionary_probdist_normalize() {
        let mut m = BTreeMap::new();
        m.insert("a", 1.0);
        m.insert("b", 3.0);
        let pd = DictionaryProbDist::new(m, false).normalized();
        assert!((pd.prob(&"a") - 0.25).abs() < 1e-12);
        assert!((pd.prob(&"b") - 0.75).abs() < 1e-12);
        assert_eq!(pd.max(), Some("b"));
    }

    // ---- MLE / Lidstone family ----

    #[test]
    fn mle_probdist_matches_freq() {
        let fd = FreqDist::from_samples(["a", "a", "b", "c"]);
        let pd = MLEProbDist::new(fd);
        assert!((pd.prob(&"a") - 0.5).abs() < 1e-12);
        assert!((pd.prob(&"b") - 0.25).abs() < 1e-12);
        assert_eq!(pd.prob(&"z"), 0.0);
    }

    #[test]
    fn lidstone_smoothing() {
        let fd = FreqDist::from_samples(["a", "a", "b"]);
        // gamma=1, bins=4 → denom = 3 + 4 = 7, prob(a) = (2+1)/7
        let pd = LidstoneProbDist::new(fd, 1.0, Some(4));
        assert!((pd.prob(&"a") - 3.0 / 7.0).abs() < 1e-12);
        assert!((pd.prob(&"z") - 1.0 / 7.0).abs() < 1e-12);
        assert!(!pd.sum_to_one());
    }

    #[test]
    fn laplace_is_lidstone_one() {
        let fd = FreqDist::from_samples(["a", "a", "b"]);
        let lp = LaplaceProbDist::new(fd.clone(), Some(3));
        let li = LidstoneProbDist::new(fd, 1.0, Some(3));
        assert!((lp.prob(&"a") - li.prob(&"a")).abs() < 1e-12);
    }

    #[test]
    fn ele_is_lidstone_half() {
        let fd = FreqDist::from_samples(["a", "a", "b"]);
        let ele = ELEProbDist::new(fd.clone(), Some(3));
        let li = LidstoneProbDist::new(fd, 0.5, Some(3));
        assert!((ele.prob(&"a") - li.prob(&"a")).abs() < 1e-12);
    }

    // ---- Heldout ----

    #[test]
    fn heldout_basic_estimate() {
        let base = FreqDist::from_samples(["a", "a", "b", "c"]); // a=2,b=1,c=1
        let held = FreqDist::from_samples(["a", "b", "b", "c"]); // a=1,b=2,c=1
        let pd = HeldoutProbDist::new(base, held, Some(4));
        // Some samples produce non-negative probabilities; verify non-NaN.
        assert!(pd.prob(&"a").is_finite());
        assert!(pd.prob(&"z").is_finite());
        assert!(!pd.sum_to_one());
    }

    // ---- WittenBell ----

    #[test]
    fn witten_bell_unseen_gets_mass() {
        let fd = FreqDist::from_samples(["a", "a", "b", "c"]); // T=3, N=4
        let pd = WittenBellProbDist::new(fd, Some(10));
        let pa = pd.prob(&"a");
        let pz = pd.prob(&"z");
        assert!(pa > 0.0);
        assert!(pz > 0.0);
        assert!(pa > pz);
    }

    // ---- SimpleGoodTuring ----

    #[test]
    fn good_turing_runs_to_completion() {
        let mut samples: Vec<&'static str> = Vec::new();
        for s in ["a", "b", "c", "d", "e"] {
            samples.push(s);
        }
        for s in ["a", "b", "c"] {
            samples.push(s);
        }
        for s in ["a", "b"] {
            samples.push(s);
        }
        for _ in 0..3 {
            samples.push("a");
        }
        let fd = FreqDist::from_samples(samples);
        let pd = SimpleGoodTuringProbDist::new(fd, Some(20));
        assert!(pd.prob(&"a").is_finite());
        assert!(pd.discount() >= 0.0);
        assert!(!pd.sum_to_one());
    }

    // ---- MutableProbDist ----

    #[test]
    fn mutable_probdist_update() {
        let fd = FreqDist::from_samples(["a", "a", "b"]);
        let base = MLEProbDist::new(fd);
        let mut mp = MutableProbDist::new(&base, vec!["a", "b"], false);
        assert!((mp.prob(&"a") - 2.0 / 3.0).abs() < 1e-12);
        mp.update(&"a", 0.9, false);
        assert!((mp.prob(&"a") - 0.9).abs() < 1e-12);
    }

    // ---- KneserNey ----

    #[test]
    fn kneser_ney_seen_trigram() {
        let mut fd = FreqDist::<(&'static str, &'static str, &'static str)>::new();
        fd.inc(("a", "b", "c"), 3);
        fd.inc(("a", "b", "d"), 1);
        fd.inc(("x", "y", "z"), 1);
        let kn = KneserNeyProbDist::new(fd, None, 0.75);
        // For seen trigram (a,b,c): (3 - 0.75) / bigram(a,b) = 2.25 / 4
        assert!((kn.prob(&("a", "b", "c")) - 2.25 / 4.0).abs() < 1e-12);
        // Unseen trigram: probability is 0.0 (no backoff hit either).
        assert_eq!(kn.prob(&("p", "q", "r")), 0.0);
    }

    // ---- ConditionalProbDist ----

    #[test]
    fn conditional_probdist_factory() {
        let cfd = ConditionalFreqDist::from_pairs([
            ("n", "dog"),
            ("n", "dog"),
            ("n", "cat"),
            ("v", "ran"),
        ]);
        let cpd: ConditionalProbDist<_, _, MLEProbDist<&'static str>> =
            ConditionalProbDist::new(&cfd, MLEProbDist::new);
        assert!((cpd.get(&"n").unwrap().prob(&"dog") - 2.0 / 3.0).abs() < 1e-12);
        assert!((cpd.get(&"v").unwrap().prob(&"ran") - 1.0).abs() < 1e-12);
    }

    // ---- log-space helpers ----

    #[test]
    fn add_logs_basic() {
        // 2^0 + 2^0 = 2 → log2 = 1
        let s = add_logs(0.0, 0.0);
        assert!((s - 1.0).abs() < 1e-12);
    }

    #[test]
    fn add_logs_collapses_when_far_apart() {
        let s = add_logs(0.0, -1000.0);
        assert!((s - 0.0).abs() < 1e-12);
    }

    #[test]
    fn sum_logs_empty_returns_ninf() {
        assert_eq!(sum_logs(&[]), NINF);
    }

    #[test]
    fn entropy_uniform_two() {
        let u = UniformProbDist::new(["a", "b"]);
        // -2 * (0.5 * log2 0.5) = 1.0
        assert!((entropy(&u) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn log_likelihood_runs() {
        let test = UniformProbDist::new(["a", "b"]);
        let actual = MLEProbDist::new(FreqDist::from_samples(["a", "a", "b"]));
        let v = log_likelihood(&test, &actual);
        // log2(0.5) for each non-zero sample, weighted by actual probs.
        // = 1.0 * (-1.0) = -1.0
        assert!((v - (-1.0)).abs() < 1e-12);
    }
}
