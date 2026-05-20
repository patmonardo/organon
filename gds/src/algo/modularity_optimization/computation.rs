use super::spec::{ModularityOptimizationConfig, ModularityOptimizationResult};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ModularityOptimizationInput {
    pub node_count: usize,
    /// Weighted adjacency list.
    pub adj: Vec<Vec<(usize, f64)>>,
}

impl ModularityOptimizationInput {
    pub fn new(node_count: usize, adj: Vec<Vec<(usize, f64)>>) -> Self {
        Self { node_count, adj }
    }

    pub fn degrees(&self) -> Vec<f64> {
        let mut degrees = vec![0.0; self.node_count];
        for (i, nbrs) in self.adj.iter().enumerate() {
            degrees[i] = nbrs.iter().map(|&(_, w)| w).sum();
        }
        degrees
    }

    pub fn total_relationship_weight(&self, degrees: &[f64]) -> f64 {
        degrees.iter().copied().sum::<f64>()
    }
}

pub struct ModularityOptimizationComputationRuntime {}

impl ModularityOptimizationComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    /// Single-level Louvain-style local moving.
    ///
    /// Returns a community assignment over nodes in `input`.
    pub fn compute(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &ModularityOptimizationConfig,
    ) -> ModularityOptimizationResult {
        if input.node_count == 0 {
            return ModularityOptimizationResult {
                communities: Vec::new(),
                modularity: 0.0,
                ran_iterations: 0,
                did_converge: true,
                node_count: 0,
                execution_time: Duration::default(),
            };
        }

        let degrees = input.degrees();
        let two_m = input.total_relationship_weight(&degrees);
        if two_m == 0.0 {
            return ModularityOptimizationResult {
                communities: (0..input.node_count as u64).collect(),
                modularity: 0.0,
                ran_iterations: 0,
                did_converge: true,
                node_count: input.node_count,
                execution_time: Duration::default(),
            };
        }

        // Start with each node in its own community.
        let mut assignment: Vec<usize> = (0..input.node_count).collect();
        let mut tot: Vec<f64> = degrees.clone();

        let mut ran_iterations = 0;
        let mut did_converge = false;

        for iter in 0..config.max_iterations {
            ran_iterations = iter + 1;
            let mut moved_any = false;

            for node in 0..input.node_count {
                let current = assignment[node];
                let k_i = degrees[node];

                // Weight from node to each neighboring community.
                let mut community_in: HashMap<usize, f64> = HashMap::new();
                for &(nbr, w) in &input.adj[node] {
                    let c = assignment[nbr];
                    *community_in.entry(c).or_insert(0.0) += w;
                }

                // Remove node from current community.
                tot[current] -= k_i;

                let mut best_comm = current;
                let mut best_gain = 0.0;

                for (&candidate, &k_i_in) in community_in.iter() {
                    let gain = k_i_in - (config.gamma * k_i * tot[candidate] / two_m);
                    if gain > best_gain
                        || (gain == best_gain && candidate < best_comm && best_gain > 0.0)
                    {
                        best_gain = gain;
                        best_comm = candidate;
                    }
                }

                // Reinsert.
                tot[best_comm] += k_i;
                assignment[node] = best_comm;
                if best_comm != current {
                    moved_any = true;
                }
            }

            if !moved_any {
                did_converge = true;
                break;
            }
        }

        let modularity =
            modularity_from_assignment(input, &assignment, &degrees, two_m, config.gamma);

        ModularityOptimizationResult {
            communities: assignment.into_iter().map(|c| c as u64).collect(),
            modularity,
            ran_iterations,
            did_converge,
            node_count: input.node_count,
            execution_time: Duration::default(),
        }
    }
}

fn modularity_from_assignment(
    graph: &ModularityOptimizationInput,
    assignment: &[usize],
    degrees: &[f64],
    two_m: f64,
    gamma: f64,
) -> f64 {
    let mut community_degree: HashMap<usize, f64> = HashMap::new();
    let mut inside: HashMap<usize, f64> = HashMap::new();

    for node in 0..graph.node_count {
        let c = assignment[node];
        *community_degree.entry(c).or_insert(0.0) += degrees[node];
        for &(nbr, w) in &graph.adj[node] {
            if assignment[nbr] == c {
                *inside.entry(c).or_insert(0.0) += w;
            }
        }
    }

    let mut q = 0.0;
    for (&c, &kc) in community_degree.iter() {
        let ec = inside.get(&c).copied().unwrap_or(0.0) / two_m;
        let ac = kc / two_m;
        q += ec - gamma * (ac * ac);
    }
    q
}

impl Default for ModularityOptimizationComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}
