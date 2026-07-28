//! Leiden computation runtime.
//!
//! This is a phase-structured implementation inspired by the Java GDS Leiden pipeline:
//! - init volumes
//! - local move (queue-based)
//! - refinement (ensure connected communities)
//! - aggregation (contract graph and iterate levels)

use super::spec::{LeidenConfig, LeidenResult};
use crate::task::concurrency::TerminationFlag;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

#[derive(Clone, Debug)]
pub(crate) struct AdjacencyGraph {
    pub node_count: usize,
    /// Undirected adjacency list.
    pub adj: Vec<Vec<(usize, f64)>>,
}

impl AdjacencyGraph {
    pub fn new(node_count: usize, adj: Vec<Vec<(usize, f64)>>) -> Self {
        Self { node_count, adj }
    }

    pub fn degree(&self, node: usize) -> f64 {
        self.adj[node].iter().map(|(_, w)| *w).sum()
    }

    pub fn total_edge_weight(&self) -> f64 {
        let total_degree: f64 = (0..self.node_count).map(|i| self.degree(i)).sum();
        total_degree / 2.0
    }
}

#[derive(Clone, Debug)]
pub struct LeidenComputationResult {
    pub communities: Vec<u64>,
    pub modularity: f64,
    pub levels: usize,
    pub converged: bool,
    pub modularities: Vec<f64>,
    pub intermediate_communities: Option<Vec<Vec<u64>>>,
}

impl LeidenComputationResult {
    pub fn intermediate_communities(&self, node_id: usize) -> Vec<u64> {
        match &self.intermediate_communities {
            Some(levels) => levels
                .iter()
                .filter_map(|level| level.get(node_id).copied())
                .collect(),
            None => self.communities.get(node_id).copied().into_iter().collect(),
        }
    }
}

pub struct LeidenComputationRuntime {}

impl LeidenComputationRuntime {
    pub fn new() -> Self {
        Self {}
    }

    pub(crate) fn compute(
        &mut self,
        graph: &AdjacencyGraph,
        config: &LeidenConfig,
        termination_flag: &TerminationFlag,
    ) -> Result<LeidenComputationResult, String> {
        let n = graph.node_count;
        if n == 0 {
            return Ok(LeidenComputationResult {
                communities: Vec::new(),
                modularity: 0.0,
                levels: 0,
                converged: true,
                modularities: Vec::new(),
                intermediate_communities: config.include_intermediate_communities.then(Vec::new),
            });
        }

        let seed_mapper = SeedCommunityMapper::new(n, config);

        // === Init volumes ===
        // Total edge weight `m` changes after aggregation, so we treat it as a per-level value.
        let mut m = graph.total_edge_weight();
        if m <= 0.0 {
            let communities = seed_mapper.map_assignments(&starting_communities(n, config));
            return Ok(LeidenComputationResult {
                intermediate_communities: config
                    .include_intermediate_communities
                    .then(|| vec![communities.clone()]),
                communities,
                modularity: 0.0,
                levels: 0,
                converged: true,
                modularities: vec![0.0],
            });
        }

        // Dendrogram-lift: for each original node, track the current (working-graph) node id.
        // At level 0, working nodes match original nodes.
        let mut original_to_working: Vec<usize> = (0..n).collect();

        // Working graph + assignments at current level.
        let mut working_graph = graph.clone();
        let mut working_communities: Vec<u64> = starting_communities(n, config);

        // Output communities over original nodes; updated after each level.
        let mut output_communities = seed_mapper.map_assignments(&working_communities);

        let mut last_modularity = modularity(&working_graph, &working_communities, m, config.gamma);
        let mut modularities = vec![last_modularity];
        let mut dendrogram = config
            .include_intermediate_communities
            .then(|| vec![output_communities.clone()]);
        let mut levels = 0usize;
        let mut converged = false;

        for _level in 0..config.max_iterations {
            termination_flag.assert_running();
            levels += 1;

            // Recompute total edge weight for this working graph level.
            m = working_graph.total_edge_weight();
            if m <= 0.0 {
                converged = true;
                break;
            }

            let node_volumes = node_volumes_for(&working_graph);

            // Local move determines the output partition for this level.
            let swaps = local_move_phase(
                &working_graph,
                &mut working_communities,
                &node_volumes,
                m,
                config.gamma,
                termination_flag,
            )?;

            working_communities = renumber_communities(working_communities);

            let new_modularity = modularity(&working_graph, &working_communities, m, config.gamma);
            let improvement = new_modularity - last_modularity;

            // Lift community ids back to original nodes.
            let previous_output = output_communities.clone();
            let mut next_output_communities = vec![0u64; n];
            for (original, &working_node) in original_to_working.iter().enumerate() {
                next_output_communities[original] = working_communities[working_node];
            }
            next_output_communities = seed_mapper.map_assignments(&next_output_communities);

            if improvement < 0.0 {
                output_communities = previous_output;
                break;
            }

            last_modularity = new_modularity;
            modularities.push(new_modularity);
            output_communities = next_output_communities;
            if let Some(levels) = &mut dendrogram {
                levels.push(output_communities.clone());
            }

            if swaps == 0 || improvement <= config.tolerance {
                converged = true;
                break;
            }

            // Refinement starts from singleton subsets of the local-move partition and only
            // controls contraction; the local-move partition remains the algorithm partition.
            let mut refined_communities = working_communities.clone();
            let mut rng = StdRng::seed_from_u64(config.random_seed);
            refinement_phase(
                &working_graph,
                &mut refined_communities,
                &node_volumes,
                m,
                config.gamma,
                config.theta,
                &mut rng,
                termination_flag,
            )?;
            refined_communities = renumber_communities(refined_communities);

            let (next_graph, next_mapping) = aggregate_graph(&working_graph, &refined_communities);
            if next_graph.node_count == working_graph.node_count {
                // No change in number of nodes => no further meaningful aggregation.
                converged = true;
                break;
            }

            // Update the original->working mapping by translating each node's working id
            // through its community-to-aggregated-node mapping.
            for w in &mut original_to_working {
                *w = next_mapping[*w];
            }

            let mut next_partition = vec![u64::MAX; next_graph.node_count];
            for node in 0..working_graph.node_count {
                let aggregated_node = next_mapping[node];
                let local_community = working_communities[node];
                let assigned = &mut next_partition[aggregated_node];
                if *assigned == u64::MAX {
                    *assigned = local_community;
                } else {
                    debug_assert_eq!(*assigned, local_community);
                }
            }

            working_graph = next_graph;
            working_communities = renumber_communities(next_partition);
        }

        let final_modularity = last_modularity;

        Ok(LeidenComputationResult {
            communities: output_communities,
            modularity: final_modularity,
            levels,
            converged,
            modularities,
            intermediate_communities: dendrogram,
        })
    }

    pub fn into_result(result: LeidenComputationResult) -> LeidenResult {
        let community_count = unique_count(&result.communities) as u64;
        let node_count = result.communities.len();
        LeidenResult {
            communities: result.communities,
            community_count,
            modularity: result.modularity,
            levels: result.levels,
            ran_levels: result.levels,
            converged: result.converged,
            did_converge: result.converged,
            modularities: result.modularities,
            intermediate_communities: result.intermediate_communities,
            node_count,
            execution_time: Duration::default(),
        }
    }
}

impl Default for LeidenComputationRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn node_volumes_for(graph: &AdjacencyGraph) -> Vec<f64> {
    (0..graph.node_count).map(|i| graph.degree(i)).collect()
}

fn starting_communities(node_count: usize, config: &LeidenConfig) -> Vec<u64> {
    if let Some(seeds) = &config.seed_communities {
        if seeds.len() == node_count {
            let mut ids = seeds.clone();
            ids.sort_unstable();
            ids.dedup();

            let mut map: HashMap<u64, u64> = HashMap::with_capacity(ids.len());
            for (new, old) in ids.into_iter().enumerate() {
                map.insert(old, new as u64);
            }

            return seeds.iter().map(|seed| map[seed]).collect();
        }
        if seeds.is_empty() {
            // Treat empty as "no seeds".
        }
    }
    (0..node_count as u64).collect()
}

struct SeedCommunityMapper {
    reverse: HashMap<u64, u64>,
}

impl SeedCommunityMapper {
    fn new(node_count: usize, config: &LeidenConfig) -> Self {
        let Some(seeds) = &config.seed_communities else {
            return Self {
                reverse: HashMap::new(),
            };
        };
        if seeds.len() != node_count {
            return Self {
                reverse: HashMap::new(),
            };
        }

        let mut ids = seeds.clone();
        ids.sort_unstable();
        ids.dedup();

        let reverse = ids
            .into_iter()
            .enumerate()
            .map(|(internal, seed)| (internal as u64, seed))
            .collect();

        Self { reverse }
    }

    fn map_assignments(&self, communities: &[u64]) -> Vec<u64> {
        if self.reverse.is_empty() {
            return communities.to_vec();
        }
        communities
            .iter()
            .map(|community| self.reverse.get(community).copied().unwrap_or(*community))
            .collect()
    }
}

fn local_move_phase(
    graph: &AdjacencyGraph,
    communities: &mut [u64],
    node_volumes: &[f64],
    m: f64,
    gamma: f64,
    termination_flag: &TerminationFlag,
) -> Result<usize, String> {
    let n = graph.node_count;

    // Community total volume (sum of node volumes).
    let mut community_totals: HashMap<u64, f64> = HashMap::new();
    for i in 0..n {
        *community_totals.entry(communities[i]).or_insert(0.0) += node_volumes[i];
    }

    let mut in_queue = vec![true; n];
    let mut queue: VecDeque<usize> = (0..n).collect();
    let mut swaps = 0usize;

    while let Some(node) = queue.pop_front() {
        termination_flag.assert_running();
        in_queue[node] = false;

        let current = communities[node];
        let k_i = node_volumes[node];
        if k_i == 0.0 {
            continue;
        }

        // Sum weights to each neighboring community.
        let mut w_to_comm: HashMap<u64, f64> = HashMap::new();
        let mut self_weight = 0.0;
        for (nbr, w) in &graph.adj[node] {
            if *nbr == node {
                self_weight += *w;
            }
            let c = communities[*nbr];
            *w_to_comm.entry(c).or_insert(0.0) += *w;
        }

        let w_to_current = *w_to_comm.get(&current).unwrap_or(&0.0) - self_weight;
        let d_a = *community_totals.get(&current).unwrap_or(&0.0);

        let mut best = current;
        let mut best_delta = 0.0;

        for (&candidate, &w_to_candidate) in &w_to_comm {
            if candidate == current {
                continue;
            }

            let d_b = *community_totals.get(&candidate).unwrap_or(&0.0);

            // ΔQ derived from community contribution changes (see module docs).
            let delta = (w_to_candidate - w_to_current) / m
                - gamma * (k_i * (d_b - d_a) + k_i * k_i) / (2.0 * m * m);

            if delta > best_delta {
                best_delta = delta;
                best = candidate;
            }
        }

        if best != current && best_delta > 1e-12 {
            communities[node] = best;

            *community_totals.entry(current).or_insert(0.0) -= k_i;
            *community_totals.entry(best).or_insert(0.0) += k_i;
            swaps += 1;

            // Add neighbors to queue.
            for (nbr, _) in &graph.adj[node] {
                if !in_queue[*nbr] {
                    in_queue[*nbr] = true;
                    queue.push_back(*nbr);
                }
            }
        }
    }

    Ok(swaps)
}

fn refinement_phase(
    graph: &AdjacencyGraph,
    communities: &mut [u64],
    node_volumes: &[f64],
    m: f64,
    gamma: f64,
    theta: f64,
    rng: &mut StdRng,
    termination_flag: &TerminationFlag,
) -> Result<(), String> {
    let n = graph.node_count;
    let original_communities = communities.to_vec();
    let mut original_community_volumes: HashMap<u64, f64> = HashMap::new();
    for node in 0..n {
        *original_community_volumes
            .entry(original_communities[node])
            .or_insert(0.0) += node_volumes[node];
    }

    let mut refined_communities: Vec<u64> = (0..n as u64).collect();
    let mut refined_community_volumes = node_volumes.to_vec();
    let mut relationships_within_original = vec![0.0; n];
    for node in 0..n {
        let original = original_communities[node];
        relationships_within_original[node] = graph.adj[node]
            .iter()
            .filter(|(target, _)| original_communities[*target] == original)
            .map(|(_, weight)| *weight)
            .sum();
    }

    let normalized_gamma = gamma / (2.0 * m);
    let mut singleton = vec![true; n];

    for node in 0..n {
        termination_flag.assert_running();
        if !singleton[node]
            || !is_well_connected_refined_community(
                node,
                &original_communities,
                &original_community_volumes,
                &refined_community_volumes,
                &relationships_within_original,
                normalized_gamma,
            )
        {
            continue;
        }

        let original = original_communities[node];
        let mut candidates: Vec<(usize, f64)> = Vec::new();
        for (target, weight) in &graph.adj[node] {
            if original_communities[*target] != original {
                continue;
            }
            let candidate = refined_communities[*target] as usize;
            if !is_well_connected_refined_community(
                candidate,
                &original_communities,
                &original_community_volumes,
                &refined_community_volumes,
                &relationships_within_original,
                normalized_gamma,
            ) {
                continue;
            }
            if let Some((_, accumulated_weight)) = candidates
                .iter_mut()
                .find(|(community, _)| *community == candidate)
            {
                *accumulated_weight += *weight;
            } else {
                candidates.push((candidate, *weight));
            }
        }
        if candidates.is_empty() {
            continue;
        }

        let node_volume = node_volumes[node];
        let mut probabilities = Vec::with_capacity(candidates.len());
        let mut probability_sum = 0.0;
        let mut best_gain = 0.0;
        let mut best_community = node;
        let mut total_relationship_weight = 0.0;
        for &(candidate, relationship_weight) in &candidates {
            total_relationship_weight += relationship_weight;
            let gain = relationship_weight
                - node_volume * refined_community_volumes[candidate] * normalized_gamma;
            if gain > best_gain {
                best_gain = gain;
                best_community = candidate;
            }
            let probability = if gain >= 0.0 && theta > 0.0 {
                (gain / theta).exp()
            } else if gain > 0.0 {
                f64::INFINITY
            } else {
                0.0
            };
            probabilities.push(probability);
            probability_sum += probability;
        }

        let mut selected = node;
        if !probability_sum.is_finite() || probability_sum <= 0.0 {
            if best_gain > 0.0 {
                selected = best_community;
            }
        } else {
            let draw = probability_sum * rng.gen::<f64>();
            let mut cumulative = 0.0;
            for ((candidate, _), probability) in candidates.iter().zip(probabilities) {
                cumulative += probability;
                if draw <= cumulative {
                    selected = *candidate;
                    break;
                }
            }
        }

        if selected != node {
            let selected_relationship_weight = candidates
                .iter()
                .find(|(candidate, _)| *candidate == selected)
                .map(|(_, weight)| *weight)
                .unwrap_or(0.0);
            refined_communities[node] = selected as u64;
            singleton[selected] = false;
            refined_community_volumes[selected] += node_volume;
            refined_community_volumes[node] -= node_volume;
            relationships_within_original[selected] +=
                total_relationship_weight - selected_relationship_weight;
        }
    }

    communities.copy_from_slice(&refined_communities);
    Ok(())
}

fn is_well_connected_refined_community(
    community: usize,
    original_communities: &[u64],
    original_community_volumes: &HashMap<u64, f64>,
    refined_community_volumes: &[f64],
    relationships_within_original: &[f64],
    normalized_gamma: f64,
) -> bool {
    let original_volume = original_community_volumes[&original_communities[community]];
    let refined_volume = refined_community_volumes[community];
    let threshold = normalized_gamma * refined_volume * (original_volume - refined_volume);
    relationships_within_original[community] >= threshold
}

fn aggregate_graph(graph: &AdjacencyGraph, communities: &[u64]) -> (AdjacencyGraph, Vec<usize>) {
    let n = graph.node_count;
    if n == 0 {
        return (AdjacencyGraph::new(0, Vec::new()), Vec::new());
    }

    // Map community ids to compact [0..k) ids.
    let mut comm_ids = communities.to_vec();
    comm_ids.sort_unstable();
    comm_ids.dedup();

    let mut comm_to_new: HashMap<u64, usize> = HashMap::with_capacity(comm_ids.len());
    for (idx, c) in comm_ids.into_iter().enumerate() {
        comm_to_new.insert(c, idx);
    }

    let k = comm_to_new.len();

    // Node->aggregated node mapping (via its community).
    let mut node_to_agg: Vec<usize> = vec![0; n];
    for node in 0..n {
        node_to_agg[node] = *comm_to_new.get(&communities[node]).unwrap();
    }

    // Preserve the directed adjacency-weight convention, including internal edges as loops.
    let mut edge_weights: HashMap<(usize, usize), f64> = HashMap::new();
    for u in 0..n {
        let cu = node_to_agg[u];
        for (v, w) in &graph.adj[u] {
            let cv = node_to_agg[*v];
            *edge_weights.entry((cu, cv)).or_insert(0.0) += *w;
        }
    }

    let mut adj = vec![Vec::new(); k];
    for ((source, target), weight) in edge_weights {
        adj[source].push((target, weight));
    }

    (AdjacencyGraph::new(k, adj), node_to_agg)
}

fn modularity(graph: &AdjacencyGraph, communities: &[u64], m: f64, gamma: f64) -> f64 {
    if m <= 0.0 {
        return 0.0;
    }

    let n = graph.node_count;

    let mut tot: HashMap<u64, f64> = HashMap::new();
    for i in 0..n {
        let k_i = graph.degree(i);
        *tot.entry(communities[i]).or_insert(0.0) += k_i;
    }

    let two_m = 2.0 * m;

    // Internal directed adjacency weight per community. Contracted internal edges are loops.
    let mut internal: HashMap<u64, f64> = HashMap::new();
    for u in 0..n {
        let cu = communities[u];
        for (v, w) in &graph.adj[u] {
            if cu == communities[*v] {
                *internal.entry(cu).or_insert(0.0) += *w;
            }
        }
    }

    let mut q = 0.0;
    for (&c, &d_c) in &tot {
        let l_c = *internal.get(&c).unwrap_or(&0.0);
        q += l_c / two_m - gamma * (d_c / two_m).powi(2);
    }

    q
}

fn renumber_communities(mut communities: Vec<u64>) -> Vec<u64> {
    if communities.is_empty() {
        return communities;
    }

    let mut ids = communities.clone();
    ids.sort_unstable();
    ids.dedup();

    let mut map: HashMap<u64, u64> = HashMap::with_capacity(ids.len());
    for (new, old) in ids.into_iter().enumerate() {
        map.insert(old, new as u64);
    }

    for c in &mut communities {
        *c = *map.get(c).unwrap_or(c);
    }

    communities
}

fn unique_count(communities: &[u64]) -> usize {
    let mut ids = communities.to_vec();
    ids.sort_unstable();
    ids.dedup();
    ids.len()
}

#[cfg(test)]
mod certification_tests {
    use super::*;

    fn undirected_graph(node_count: usize, edges: &[(usize, usize, f64)]) -> AdjacencyGraph {
        let mut adjacency = vec![Vec::new(); node_count];
        for &(source, target, weight) in edges {
            adjacency[source].push((target, weight));
            adjacency[target].push((source, weight));
        }
        AdjacencyGraph::new(node_count, adjacency)
    }

    #[test]
    fn aggregation_preserves_internal_and_total_weight() {
        let graph = AdjacencyGraph::new(
            3,
            vec![
                vec![(0, 2.0), (1, 1.0)],
                vec![(0, 1.0), (2, 3.0)],
                vec![(1, 3.0), (2, 4.0)],
            ],
        );
        let original_weight: f64 = graph.adj.iter().flatten().map(|(_, weight)| weight).sum();

        let (contracted, mapping) = aggregate_graph(&graph, &[7, 7, 9]);
        let contracted_weight: f64 = contracted
            .adj
            .iter()
            .flatten()
            .map(|(_, weight)| weight)
            .sum();

        assert_eq!(contracted.node_count, 2);
        assert_eq!(mapping, vec![0, 0, 1]);
        assert_eq!(contracted_weight, original_weight);
        let mut first = contracted.adj[0].clone();
        let mut second = contracted.adj[1].clone();
        first.sort_by_key(|(target, _)| *target);
        second.sort_by_key(|(target, _)| *target);
        assert_eq!(first, vec![(0, 4.0), (1, 3.0)]);
        assert_eq!(second, vec![(0, 3.0), (1, 4.0)]);
    }

    #[test]
    fn modularity_is_invariant_under_contraction() {
        let graph = AdjacencyGraph::new(
            3,
            vec![vec![(1, 2.0)], vec![(0, 2.0), (2, 1.0)], vec![(1, 1.0)]],
        );
        let communities = vec![0, 0, 1];
        let before = modularity(&graph, &communities, graph.total_edge_weight(), 1.0);

        let (contracted, _) = aggregate_graph(&graph, &communities);
        let after = modularity(&contracted, &[0, 1], contracted.total_edge_weight(), 1.0);

        assert!((before - after).abs() < 1e-12);
    }

    #[test]
    fn refinement_merges_only_connected_subsets_of_original_partition() {
        let graph = undirected_graph(4, &[(0, 1, 1.0), (2, 3, 1.0)]);
        let volumes = node_volumes_for(&graph);
        let mut communities = vec![0, 0, 0, 0];
        let mut rng = StdRng::seed_from_u64(42);

        refinement_phase(
            &graph,
            &mut communities,
            &volumes,
            graph.total_edge_weight(),
            1.0,
            0.01,
            &mut rng,
            &TerminationFlag::default(),
        )
        .unwrap();

        assert_eq!(communities[0], communities[1]);
        assert_eq!(communities[2], communities[3]);
        assert_ne!(communities[0], communities[2]);
    }

    #[test]
    fn refinement_is_reproducible_for_fixed_seed() {
        let graph = undirected_graph(4, &[(0, 1, 1.0), (0, 2, 1.0), (1, 3, 1.0), (2, 3, 1.0)]);
        let volumes = node_volumes_for(&graph);
        let mut first = vec![0, 0, 0, 0];
        let mut second = first.clone();

        for communities in [&mut first, &mut second] {
            let mut rng = StdRng::seed_from_u64(73);
            refinement_phase(
                &graph,
                communities,
                &volumes,
                graph.total_edge_weight(),
                1.0,
                1.0,
                &mut rng,
                &TerminationFlag::default(),
            )
            .unwrap();
        }

        assert_eq!(first, second);
    }

    #[test]
    fn refinement_rejects_subsets_below_well_connectedness_threshold() {
        let graph = undirected_graph(2, &[(0, 1, 1.0)]);
        let volumes = node_volumes_for(&graph);
        let mut communities = vec![0, 0];
        let mut rng = StdRng::seed_from_u64(42);

        refinement_phase(
            &graph,
            &mut communities,
            &volumes,
            graph.total_edge_weight(),
            10.0,
            0.01,
            &mut rng,
            &TerminationFlag::default(),
        )
        .unwrap();

        assert_eq!(communities, vec![0, 1]);
    }
}
