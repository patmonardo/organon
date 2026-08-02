use crate::algo::steiner_tree::spec::SteinerTreeParent;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::task::concurrency::TerminationFlag;
use crate::types::graph::MappedNodeId;
use std::collections::VecDeque;

/// Pure state runtime for Steiner Tree.
///
/// Storage owns graph access and drives the algorithm loop.
pub struct SteinerTreeComputationRuntime {
    node_count: usize,
    delta: f64,

    // Final output state (tree)
    parent: Vec<SteinerTreeParent>,
    parent_cost: Vec<f64>,
    in_tree: Vec<bool>,

    // Search state (per-iteration)
    distances: Vec<f64>,
    predecessor: Vec<Option<MappedNodeId>>,
    predecessor_edge_weight: Vec<f64>,
    bins: Vec<VecDeque<MappedNodeId>>,
}

impl SteinerTreeComputationRuntime {
    pub fn new(delta: f64, node_count: usize) -> Self {
        Self {
            node_count,
            delta,
            parent: vec![SteinerTreeParent::Pruned; node_count],
            parent_cost: vec![0.0; node_count],
            in_tree: vec![false; node_count],
            distances: vec![f64::INFINITY; node_count],
            predecessor: vec![None; node_count],
            predecessor_edge_weight: vec![0.0; node_count],
            bins: Vec::new(),
        }
    }

    pub fn initialize_tree(&mut self, source: MappedNodeId) {
        self.parent.fill(SteinerTreeParent::Pruned);
        self.parent_cost.fill(0.0);
        self.in_tree.fill(false);

        if let Some(source_idx) = source.to_usize().filter(|&index| index < self.node_count) {
            self.parent[source_idx] = SteinerTreeParent::Root;
            self.in_tree[source_idx] = true;
        }
    }

    pub fn parent_array(&self) -> &[SteinerTreeParent] {
        &self.parent
    }

    pub fn parent_cost_array(&self) -> &[f64] {
        &self.parent_cost
    }

    pub fn reset_search(&mut self, merged_to_source: &[bool]) -> VecDeque<MappedNodeId> {
        self.reset_search_with_context(merged_to_source, &TerminationFlag::running_true())
            .expect("default Steiner search reset should not terminate")
    }

    pub fn reset_search_with_context(
        &mut self,
        merged_to_source: &[bool],
        termination: &TerminationFlag,
    ) -> Result<VecDeque<MappedNodeId>, AlgorithmError> {
        self.distances.fill(f64::INFINITY);
        self.predecessor.fill(None);
        self.predecessor_edge_weight.fill(0.0);
        self.bins.clear();

        let mut frontier = VecDeque::new();
        for (idx, merged) in merged_to_source.iter().enumerate() {
            Self::ensure_running(termination)?;
            if *merged {
                self.distances[idx] = 0.0;
                frontier.push_back(
                    MappedNodeId::try_from(idx).expect("node count must fit mapped ID space"),
                );
            }
        }
        Ok(frontier)
    }

    pub fn distance(&self, node: MappedNodeId) -> f64 {
        self.distances
            .get(physical_node_index(node))
            .copied()
            .unwrap_or(f64::INFINITY)
    }

    pub fn predecessor(&self, node: MappedNodeId) -> Option<MappedNodeId> {
        self.predecessor
            .get(physical_node_index(node))
            .copied()
            .flatten()
    }

    pub fn predecessor_edge_weight(&self, node: MappedNodeId) -> f64 {
        self.predecessor_edge_weight
            .get(physical_node_index(node))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn try_relax(&mut self, source: MappedNodeId, target: MappedNodeId, weight: f64) -> bool {
        let source_idx = physical_node_index(source);
        let target_idx = physical_node_index(target);
        if source_idx >= self.node_count || target_idx >= self.node_count {
            return false;
        }

        let new_distance = self.distances[source_idx] + weight;
        if new_distance < self.distances[target_idx] {
            self.distances[target_idx] = new_distance;
            self.predecessor[target_idx] = Some(source);
            self.predecessor_edge_weight[target_idx] = weight;
            true
        } else {
            false
        }
    }

    pub fn add_to_bin(&mut self, node: MappedNodeId, bin_index: usize) {
        while self.bins.len() <= bin_index {
            self.bins.push(VecDeque::new());
        }
        self.bins[bin_index].push_back(node);
    }

    pub fn find_next_non_empty_bin(&self, start_index: usize) -> Option<usize> {
        for i in start_index..self.bins.len() {
            if !self.bins[i].is_empty() {
                return Some(i);
            }
        }
        None
    }

    pub fn drain_bin(&mut self, bin_index: usize) -> Vec<MappedNodeId> {
        if bin_index < self.bins.len() {
            self.bins[bin_index].drain(..).collect()
        } else {
            Vec::new()
        }
    }

    /// Merge a predecessor-chain path into the output tree.
    pub fn merge_path_into_tree(
        &mut self,
        terminal: MappedNodeId,
        merged_to_source: &mut [bool],
    ) -> bool {
        self.merge_path_into_tree_with_context(
            terminal,
            merged_to_source,
            &TerminationFlag::running_true(),
        )
        .expect("default Steiner path merge should not terminate")
    }

    pub fn merge_path_into_tree_with_context(
        &mut self,
        terminal: MappedNodeId,
        merged_to_source: &mut [bool],
        termination: &TerminationFlag,
    ) -> Result<bool, AlgorithmError> {
        let mut current = terminal;
        let mut merged_any = false;

        while physical_node_index(current) < self.node_count
            && !merged_to_source[physical_node_index(current)]
        {
            Self::ensure_running(termination)?;
            let pred = match self.predecessor(current) {
                Some(p) => p,
                None => break,
            };

            let current_idx = physical_node_index(current);
            merged_to_source[current_idx] = true;
            merged_any = true;

            if !self.in_tree[current_idx] {
                self.parent[current_idx] = SteinerTreeParent::Parent(pred);
                self.parent_cost[current_idx] = self.predecessor_edge_weight(current);
                self.in_tree[current_idx] = true;
            }

            current = pred;
        }

        Ok(merged_any)
    }

    pub fn prune_non_terminal_leaves(&mut self, is_terminal: &[bool], source: MappedNodeId) {
        self.prune_non_terminal_leaves_with_context(
            is_terminal,
            source,
            &TerminationFlag::running_true(),
        )
        .expect("default Steiner pruning should not terminate");
    }

    pub fn prune_non_terminal_leaves_with_context(
        &mut self,
        is_terminal: &[bool],
        source: MappedNodeId,
        termination: &TerminationFlag,
    ) -> Result<(), AlgorithmError> {
        let node_count = self.node_count;
        let mut child_count = vec![0u32; node_count];

        for node_id in 0..node_count {
            Self::ensure_running(termination)?;
            if let SteinerTreeParent::Parent(parent) = self.parent[node_id] {
                child_count[physical_node_index(parent)] += 1;
            }
        }

        let mut queue = VecDeque::new();
        for node_id in 0..node_count {
            Self::ensure_running(termination)?;
            if !matches!(self.parent[node_id], SteinerTreeParent::Parent(_)) {
                continue;
            }
            if node_id == physical_node_index(source) {
                continue;
            }
            if child_count[node_id] == 0 && !is_terminal[node_id] {
                queue.push_back(
                    MappedNodeId::try_from(node_id).expect("node count must fit mapped ID space"),
                );
            }
        }

        while let Some(node) = queue.pop_front() {
            Self::ensure_running(termination)?;
            let node_idx = physical_node_index(node);
            let SteinerTreeParent::Parent(parent) = self.parent[node_idx] else {
                continue;
            };

            self.parent[node_idx] = SteinerTreeParent::Pruned;
            self.parent_cost[node_idx] = 0.0;
            self.in_tree[node_idx] = false;

            let parent_idx = physical_node_index(parent);
            child_count[parent_idx] = child_count[parent_idx].saturating_sub(1);

            if parent_idx != physical_node_index(source)
                && matches!(self.parent[parent_idx], SteinerTreeParent::Parent(_))
                && child_count[parent_idx] == 0
                && !is_terminal[parent_idx]
            {
                queue.push_back(
                    MappedNodeId::try_from(parent_idx)
                        .expect("node count must fit mapped ID space"),
                );
            }
        }
        Ok(())
    }

    fn ensure_running(termination: &TerminationFlag) -> Result<(), AlgorithmError> {
        if !termination.running() {
            return Err(AlgorithmError::Execution(
                "Steiner tree computation terminated".to_string(),
            ));
        }
        Ok(())
    }

    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.node_count
    }

    #[allow(dead_code)]
    pub fn delta(&self) -> f64 {
        self.delta
    }
}

fn physical_node_index(node_id: MappedNodeId) -> usize {
    node_id
        .to_usize()
        .expect("mapped graph node must fit physical index space")
}
