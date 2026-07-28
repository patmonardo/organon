//! Label Propagation computation runtime
//!
//! Java parity references:
//! - `org.neo4j.gds.labelpropagation.LabelPropagation`
//! - `InitStep` / `ComputeStep` / `ComputeStepConsumer`
//!
//! Notes:
//! - Uses Java-shaped asynchronous in-place updates over parallel node batches.
//! - Voting is weighted by relationship weight * target-node weight.
//! - Tie-breaker matches Java: smallest label ID wins when weights equal.

use crate::collections::HugeAtomicLongArray;
use crate::task::concurrency::virtual_threads::Executor;
use crate::task::concurrency::virtual_threads::WorkerContext;
use crate::task::concurrency::Concurrency;
use crate::task::concurrency::TerminationFlag;
use crate::task::progress::{NoopProgressTracker, ProgressTracker};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LabelPropResult {
    pub labels: Vec<u64>,
    pub did_converge: bool,
    pub ran_iterations: u64,
}

#[derive(Debug, Clone)]
pub struct LabelPropComputationRuntime {
    node_count: usize,
    max_iterations: u64,
    concurrency: usize,
    node_weights: Vec<f64>,
    initial_labels: Option<Vec<u64>>, // must match node_count when present
}

impl LabelPropComputationRuntime {
    pub fn new(node_count: usize, max_iterations: u64) -> Self {
        Self {
            node_count,
            max_iterations,
            concurrency: 1,
            node_weights: vec![1.0; node_count],
            initial_labels: None,
        }
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency.max(1);
        self
    }

    pub fn with_weights(mut self, weights: Vec<f64>) -> Self {
        self.node_weights = weights;
        self
    }

    /// Sets the initial labels for all nodes.
    ///
    /// This corresponds to Java's `InitStep` output.
    pub fn with_seeds(mut self, labels: Vec<u64>) -> Self {
        self.initial_labels = Some(labels);
        self
    }

    pub fn compute<F>(&mut self, node_count: u64, neighbors: F) -> LabelPropResult
    where
        F: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let mut progress_tracker = NoopProgressTracker;
        let termination_flag = TerminationFlag::default();
        self.compute_with_controls(
            node_count,
            neighbors,
            &mut progress_tracker,
            &termination_flag,
        )
        .expect("default label propagation computation should not terminate")
    }

    pub fn compute_with_controls<F>(
        &mut self,
        node_count: u64,
        neighbors: F,
        progress_tracker: &mut dyn ProgressTracker,
        termination_flag: &TerminationFlag,
    ) -> Result<LabelPropResult, String>
    where
        F: Fn(usize) -> Vec<(usize, f64)> + Sync,
    {
        let node_count = node_count as usize;
        if node_count == 0 {
            return Ok(LabelPropResult {
                labels: Vec::new(),
                did_converge: true,
                ran_iterations: 0,
            });
        }
        if node_count != self.node_count {
            return Err(format!(
                "runtime node count ({}) must match input node count ({node_count})",
                self.node_count
            ));
        }
        if self.node_weights.len() != node_count {
            return Err(format!(
                "node weight count ({}) must match node count ({node_count})",
                self.node_weights.len()
            ));
        }

        let initial_labels: Vec<u64> = if let Some(init) = self.initial_labels.take() {
            if init.len() != node_count {
                return Err(format!(
                    "initial label count ({}) must match node count ({node_count})",
                    init.len()
                ));
            }
            init
        } else {
            (0..node_count as u64).collect()
        };

        let labels = HugeAtomicLongArray::new(node_count);
        for (node, label) in initial_labels.into_iter().enumerate() {
            let label = i64::try_from(label)
                .map_err(|_| format!("initial label {label} exceeds signed label range"))?;
            labels.set(node, label);
        }

        let executor = Executor::new(Concurrency::of(self.concurrency));
        let vote_tallies = WorkerContext::new(VoteTally::new);

        let mut ran_iterations = 0u64;
        let mut did_converge = false;

        while ran_iterations < self.max_iterations {
            termination_flag.assert_running();
            let any_changed = AtomicBool::new(false);
            let relationships_processed = AtomicUsize::new(0);

            executor
                .parallel_for(0, node_count, termination_flag, |node_id| {
                    let neighbors = neighbors(node_id);
                    relationships_processed.fetch_add(neighbors.len(), Ordering::Relaxed);
                    vote_tallies.with(|tally| {
                        tally.clear();

                        let current_label = labels.get(node_id) as u64;
                        let mut best_label = current_label;
                        let mut best_weight = f64::NEG_INFINITY;

                        for (target, rel_weight) in neighbors {
                            let node_weight = *self.node_weights.get(target).unwrap_or(&1.0);
                            let vote_weight = rel_weight * node_weight;
                            let candidate_label = labels.get(target) as u64;
                            tally.add_vote(candidate_label, vote_weight);
                        }

                        for (&label, &weight) in tally.votes.iter() {
                            if weight > best_weight || (weight == best_weight && label < best_label)
                            {
                                best_weight = weight;
                                best_label = label;
                            }
                        }

                        if best_label != current_label {
                            labels.set(node_id, best_label as i64);
                            any_changed.store(true, Ordering::Relaxed);
                        }
                    });
                })
                .map_err(|_| "label propagation terminated during iteration".to_string())?;

            ran_iterations += 1;
            progress_tracker.log_progress(relationships_processed.load(Ordering::Relaxed));
            if !any_changed.load(Ordering::Relaxed) {
                did_converge = true;
                break;
            }
        }

        let labels = (0..node_count)
            .map(|node| labels.get(node) as u64)
            .collect();

        Ok(LabelPropResult {
            labels,
            did_converge,
            ran_iterations,
        })
    }
}

struct VoteTally {
    votes: HashMap<u64, f64>,
}

impl VoteTally {
    fn new() -> Self {
        Self {
            votes: HashMap::with_capacity(64),
        }
    }

    fn clear(&mut self) {
        self.votes.clear();
    }

    fn add_vote(&mut self, label: u64, weight: f64) {
        *self.votes.entry(label).or_insert(0.0) += weight;
    }
}
