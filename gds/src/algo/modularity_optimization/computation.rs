use super::spec::{ModularityOptimizationConfig, ModularityOptimizationResult};
use crate::algo::k1coloring::K1ColoringComputationRuntime;
use crate::task::concurrency::virtual_threads::Executor;
use crate::task::concurrency::Concurrency;
use crate::task::concurrency::TerminationFlag;
use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

const K1_COLORING_MAX_ITERATIONS: u64 = 5;

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

    pub fn total_relationship_count(&self) -> usize {
        self.adj.iter().map(Vec::len).sum()
    }
}

pub struct ModularityOptimizationComputationRuntime {}

#[derive(Debug, Clone, Copy)]
struct MoveProposal {
    node: usize,
    current: usize,
    next: usize,
    node_weight: f64,
}

#[derive(Debug)]
struct ModularityColorLayout {
    groups: Vec<Vec<usize>>,
}

impl ModularityColorLayout {
    fn compute(
        input: &ModularityOptimizationInput,
        _concurrency: usize,
        termination_flag: &TerminationFlag,
    ) -> Result<Self, String> {
        // Keep the color partition stable across worker counts; request concurrency
        // applies to proposal evaluation within each resulting color superstep.
        let mut coloring =
            K1ColoringComputationRuntime::new(input.node_count, K1_COLORING_MAX_ITERATIONS)
                .concurrency(1);
        let result = coloring.compute(
            input.node_count,
            |node| input.adj[node].iter().map(|(target, _)| *target).collect(),
            termination_flag,
            |_| {},
        );
        termination_flag.assert_running();

        for source in 0..input.node_count {
            for &(target, _) in &input.adj[source] {
                if target >= input.node_count {
                    return Err(format!(
                        "relationship target {target} is outside node count {}",
                        input.node_count
                    ));
                }
                if source != target && result.colors[source] == result.colors[target] {
                    return Err(format!(
                        "K1 coloring did not resolve conflict on edge {source}->{target}"
                    ));
                }
            }
        }

        let mut nodes_by_color: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
        for (node, color) in result.colors.into_iter().enumerate() {
            nodes_by_color.entry(color).or_default().push(node);
        }

        Ok(Self {
            groups: nodes_by_color.into_values().collect(),
        })
    }
}

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
        self.compute_with_initial_communities(input, config, None)
            .expect("singleton modularity initialization is always valid")
    }

    pub fn compute_with_initial_communities(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &ModularityOptimizationConfig,
        initial_communities: Option<&[u64]>,
    ) -> Result<ModularityOptimizationResult, String> {
        self.compute_with_controls(
            input,
            config,
            initial_communities,
            &TerminationFlag::running_true(),
            |_| {},
        )
    }

    pub fn compute_with_controls(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &ModularityOptimizationConfig,
        initial_communities: Option<&[u64]>,
        termination_flag: &TerminationFlag,
        mut on_iteration_complete: impl FnMut(usize),
    ) -> Result<ModularityOptimizationResult, String> {
        if let Some(communities) = initial_communities {
            if communities.len() != input.node_count {
                return Err(format!(
                    "initial community count ({}) must match node count ({})",
                    communities.len(),
                    input.node_count
                ));
            }
            if communities
                .iter()
                .any(|community| *community as usize >= input.node_count)
            {
                return Err("initial community ids must be dense node indices".to_string());
            }
        }

        if input.node_count == 0 {
            return Ok(ModularityOptimizationResult {
                communities: Vec::new(),
                modularity: 0.0,
                ran_iterations: 0,
                did_converge: true,
                node_count: 0,
                execution_time: Duration::default(),
            });
        }

        let degrees = input.degrees();
        let two_m = input.total_relationship_weight(&degrees);
        if two_m == 0.0 {
            return Ok(ModularityOptimizationResult {
                communities: initial_communities
                    .map(<[u64]>::to_vec)
                    .unwrap_or_else(|| (0..input.node_count as u64).collect()),
                modularity: 0.0,
                ran_iterations: 0,
                did_converge: true,
                node_count: input.node_count,
                execution_time: Duration::default(),
            });
        }

        let color_layout =
            ModularityColorLayout::compute(input, config.concurrency, termination_flag)?;
        let executor = Executor::new(Concurrency::of(config.concurrency));

        let mut assignment: Vec<usize> = initial_communities
            .map(|communities| {
                communities
                    .iter()
                    .map(|community| *community as usize)
                    .collect()
            })
            .unwrap_or_else(|| (0..input.node_count).collect());
        let mut tot = vec![0.0; input.node_count];
        for (node, community) in assignment.iter().copied().enumerate() {
            tot[community] += degrees[node];
        }

        let mut ran_iterations = 0;
        let mut did_converge = false;
        let mut previous_modularity = f64::NEG_INFINITY;

        for iter in 0..config.max_iterations {
            termination_flag.assert_running();
            ran_iterations = iter + 1;
            let mut moved_any = false;

            for nodes in &color_layout.groups {
                termination_flag.assert_running();
                let proposals = executor
                    .parallel_map(0, nodes.len(), termination_flag, |index| {
                        move_proposal(
                            input,
                            nodes[index],
                            &assignment,
                            &tot,
                            &degrees,
                            two_m,
                            config.gamma,
                        )
                    })
                    .map_err(|_| "modularity optimization terminated".to_string())?;

                termination_flag.assert_running();
                let mut community_weight_deltas = vec![0.0; input.node_count];
                for proposal in proposals {
                    if proposal.next == proposal.current {
                        continue;
                    }
                    assignment[proposal.node] = proposal.next;
                    community_weight_deltas[proposal.current] -= proposal.node_weight;
                    community_weight_deltas[proposal.next] += proposal.node_weight;
                    moved_any = true;
                }
                for (community, delta) in community_weight_deltas.into_iter().enumerate() {
                    if delta != 0.0 {
                        tot[community] += delta;
                    }
                }
            }

            let modularity =
                modularity_from_assignment(input, &assignment, &degrees, two_m, config.gamma);
            on_iteration_complete(input.total_relationship_count());

            if !moved_any {
                did_converge = true;
                break;
            }

            if iter > 0
                && !(modularity > previous_modularity
                    && (modularity - previous_modularity).abs() > config.tolerance)
            {
                did_converge = true;
                break;
            }

            previous_modularity = modularity;
        }

        let modularity =
            modularity_from_assignment(input, &assignment, &degrees, two_m, config.gamma);

        Ok(ModularityOptimizationResult {
            communities: assignment.into_iter().map(|c| c as u64).collect(),
            modularity,
            ran_iterations,
            did_converge,
            node_count: input.node_count,
            execution_time: Duration::default(),
        })
    }
}

fn move_proposal(
    input: &ModularityOptimizationInput,
    node: usize,
    assignment: &[usize],
    community_weights: &[f64],
    degrees: &[f64],
    two_m: f64,
    gamma: f64,
) -> MoveProposal {
    let current = assignment[node];
    let node_weight = degrees[node];
    let mut community_in: HashMap<usize, f64> = HashMap::new();
    let mut self_weight = 0.0;
    for &(neighbor, weight) in &input.adj[node] {
        if neighbor == node {
            self_weight += weight;
        }
        *community_in.entry(assignment[neighbor]).or_insert(0.0) += weight;
    }

    let mut next = current;
    let mut best_gain = 0.0;
    let current_influence = community_in.get(&current).copied().unwrap_or(0.0) - self_weight;
    let current_weight_without_node = community_weights[current] - node_weight;

    for (&candidate, &candidate_influence) in &community_in {
        if candidate == current {
            continue;
        }
        let gain = 2.0 * (candidate_influence - current_influence) / two_m
            + 2.0
                * gamma
                * node_weight
                * (current_weight_without_node - community_weights[candidate])
                / (two_m * two_m);
        if gain > best_gain || (gain == best_gain && gain != 0.0 && candidate < next) {
            best_gain = gain;
            next = candidate;
        }
    }

    MoveProposal {
        node,
        current,
        next,
        node_weight,
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

#[cfg(test)]
mod certification_tests {
    use super::*;

    #[test]
    fn move_proposal_excludes_self_loop_from_transferable_influence() {
        let input =
            ModularityOptimizationInput::new(2, vec![vec![(0, 100.0), (1, 1.0)], vec![(0, 1.0)]]);
        let degrees = input.degrees();
        let two_m = input.total_relationship_weight(&degrees);
        let assignment = vec![0, 1];

        let proposal = move_proposal(&input, 0, &assignment, &degrees, &degrees, two_m, 1.0);

        assert_eq!(proposal.current, 0);
        assert_eq!(proposal.next, 1);
    }
}
