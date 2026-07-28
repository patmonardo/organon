use super::spec::{LouvainConfig, LouvainResult};
use crate::algo::modularity_optimization::{
    ModularityOptimizationComputationRuntime, ModularityOptimizationConfig,
    ModularityOptimizationInput,
};
use crate::task::concurrency::TerminationFlag;
use std::collections::HashMap;
use std::time::Duration;

pub struct LouvainComputationRuntime {
    max_levels: usize,
    tolerance: f64,
    gamma: f64,
}

impl LouvainComputationRuntime {
    pub fn new() -> Self {
        Self {
            max_levels: 10,
            tolerance: 1e-6,
            gamma: 1.0,
        }
    }

    pub fn with_config(mut self, config: &LouvainConfig) -> Self {
        self.max_levels = config.max_iterations;
        if config.max_levels > 0 {
            self.max_levels = config.max_levels;
        }
        self.tolerance = config.tolerance;
        self.gamma = config.gamma;
        self
    }

    pub fn compute(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &LouvainConfig,
    ) -> LouvainResult {
        self.compute_with_seed_mapping(input, config, None, None)
            .expect("singleton Louvain initialization is always valid")
    }

    pub(crate) fn compute_with_seed_mapping(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &LouvainConfig,
        initial_communities: Option<&[u64]>,
        initial_external_ids: Option<&[u64]>,
    ) -> Result<LouvainResult, String> {
        self.compute_with_controls(
            input,
            config,
            initial_communities,
            initial_external_ids,
            &TerminationFlag::running_true(),
            || {},
        )
    }

    pub(crate) fn compute_with_controls(
        &mut self,
        input: &ModularityOptimizationInput,
        config: &LouvainConfig,
        initial_communities: Option<&[u64]>,
        initial_external_ids: Option<&[u64]>,
        termination_flag: &TerminationFlag,
        mut on_level_complete: impl FnMut(),
    ) -> Result<LouvainResult, String> {
        if input.node_count == 0 {
            return Ok(LouvainResult {
                data: Vec::new(),
                ran_levels: 0,
                modularities: Vec::new(),
                modularity: 0.0,
                intermediate_communities: config.include_intermediate_communities.then(Vec::new),
                node_count: 0,
                execution_time: Duration::default(),
            });
        }

        let original_node_count = input.node_count;
        let mut original_to_working: Vec<usize> = (0..original_node_count).collect();

        let mut working_graph = input.clone();
        let mut working_initial_communities = initial_communities
            .map(<[u64]>::to_vec)
            .unwrap_or_else(|| (0..original_node_count as u64).collect());
        let mut working_external_ids = initial_external_ids
            .map(<[u64]>::to_vec)
            .unwrap_or_else(|| (0..original_node_count as u64).collect());
        if working_initial_communities.len() != original_node_count {
            return Err("initial community count must match node count".to_string());
        }
        if working_initial_communities
            .iter()
            .any(|community| *community as usize >= working_external_ids.len())
        {
            return Err("initial communities must reference external community ids".to_string());
        }

        let mut output_communities: Vec<u64> = working_initial_communities
            .iter()
            .map(|community| working_external_ids[*community as usize])
            .collect();
        let mut last_modularity = f64::NEG_INFINITY;
        let mut modularities = Vec::new();
        let mut ran_levels = 0usize;
        let mut dendrogram = config.include_intermediate_communities.then(Vec::new);

        let mut modopt = ModularityOptimizationComputationRuntime::new();
        let modopt_config = ModularityOptimizationConfig {
            concurrency: config.concurrency,
            max_iterations: config.max_iterations,
            tolerance: self.tolerance,
            gamma: self.gamma,
            ..ModularityOptimizationConfig::default()
        };

        for _level in 0..self.max_levels {
            termination_flag.assert_running();
            let level_result = modopt.compute_with_controls(
                &working_graph,
                &modopt_config,
                Some(&working_initial_communities),
                termination_flag,
                |_| {},
            )?;
            let level_assignments: Vec<usize> = level_result
                .communities
                .iter()
                .map(|&c| c as usize)
                .collect();

            let modularity_now = level_result.modularity;
            let improvement = modularity_now - last_modularity;
            last_modularity = modularity_now;
            modularities.push(modularity_now);
            ran_levels += 1;
            on_level_complete();

            for (original, working_node) in original_to_working.iter().copied().enumerate() {
                let internal = level_assignments[working_node];
                output_communities[original] = working_external_ids[internal];
            }
            if let Some(levels) = &mut dendrogram {
                levels.push(output_communities.clone());
            }

            let (next_graph, next_mapping, next_external_ids) =
                aggregate(&working_graph, &level_assignments, &working_external_ids);
            if next_graph.node_count == working_graph.node_count
                || next_graph.node_count == 1
                || (ran_levels > 1 && improvement <= self.tolerance)
            {
                break;
            }

            for working_node in &mut original_to_working {
                *working_node = next_mapping[*working_node];
            }
            working_graph = next_graph;
            working_external_ids = next_external_ids;
            working_initial_communities = (0..working_graph.node_count as u64).collect();
        }

        Ok(LouvainResult {
            data: output_communities,
            ran_levels,
            modularity: modularities.last().copied().unwrap_or(0.0),
            modularities,
            intermediate_communities: dendrogram,
            node_count: original_node_count,
            execution_time: Duration::default(),
        })
    }
}

fn aggregate(
    graph: &ModularityOptimizationInput,
    assignment: &[usize],
    external_ids: &[u64],
) -> (ModularityOptimizationInput, Vec<usize>, Vec<u64>) {
    let mut community_ids = assignment.to_vec();
    community_ids.sort_unstable();
    community_ids.dedup();

    let dense_by_community: HashMap<usize, usize> = community_ids
        .iter()
        .copied()
        .enumerate()
        .map(|(dense, community)| (community, dense))
        .collect();
    let node_mapping: Vec<usize> = assignment
        .iter()
        .map(|community| dense_by_community[community])
        .collect();
    let next_external_ids: Vec<u64> = community_ids
        .iter()
        .map(|community| external_ids[*community])
        .collect();

    let mut edge_weights: HashMap<(usize, usize), f64> = HashMap::new();

    for u in 0..graph.node_count {
        let cu = node_mapping[u];
        for &(v, w) in &graph.adj[u] {
            let cv = node_mapping[v];
            *edge_weights.entry((cu, cv)).or_insert(0.0) += w;
        }
    }

    let new_node_count = community_ids.len();
    let mut new_adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); new_node_count];
    for ((source, target), weight) in edge_weights {
        new_adj[source].push((target, weight));
    }

    (
        ModularityOptimizationInput::new(new_node_count, new_adj),
        node_mapping,
        next_external_ids,
    )
}

impl Default for LouvainComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contraction_preserves_weight_and_densifies_communities() {
        let graph = ModularityOptimizationInput::new(
            3,
            vec![
                vec![(0, 2.0), (1, 1.0)],
                vec![(0, 1.0), (2, 3.0)],
                vec![(1, 3.0), (2, 4.0)],
            ],
        );
        let original_weight: f64 = graph.adj.iter().flatten().map(|(_, weight)| weight).sum();

        let (contracted, node_mapping, external_ids) =
            aggregate(&graph, &[2, 2, 5], &[10, 11, 20, 21, 22, 50]);
        let contracted_weight: f64 = contracted
            .adj
            .iter()
            .flatten()
            .map(|(_, weight)| weight)
            .sum();

        assert_eq!(contracted.node_count, 2);
        assert_eq!(node_mapping, vec![0, 0, 1]);
        assert_eq!(external_ids, vec![20, 50]);
        assert_eq!(contracted_weight, original_weight);
    }
}
