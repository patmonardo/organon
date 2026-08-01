use crate::types::graph::MappedNodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PCSTreeParent {
    Pruned,
    Root,
    Parent(MappedNodeId),
}

/// Computation runtime for Prize-Collecting Steiner Tree.
///
/// Pure state only: maintains the current tree (parent pointers + parent costs),
/// node inclusion flags, and pruning operations. Graph access and the main loop
/// live in the storage runtime.
pub struct PCSTreeComputationRuntime {
    node_count: usize,
    prizes: Vec<f64>,

    parent: Vec<PCSTreeParent>,
    parent_cost: Vec<f64>,
    in_tree: Vec<bool>,
}

impl PCSTreeComputationRuntime {
    pub fn new(prizes: Vec<f64>, node_count: usize) -> Self {
        Self {
            node_count,
            prizes,
            parent: vec![PCSTreeParent::Pruned; node_count],
            parent_cost: vec![0.0; node_count],
            in_tree: vec![false; node_count],
        }
    }

    pub fn reset(&mut self) {
        self.parent.fill(PCSTreeParent::Pruned);
        self.parent_cost.fill(0.0);
        self.in_tree.fill(false);
    }

    pub fn node_count(&self) -> usize {
        self.node_count
    }

    pub fn prizes(&self) -> &[f64] {
        &self.prizes
    }

    pub fn parent_array(&self) -> &[PCSTreeParent] {
        &self.parent
    }

    pub fn parent_cost_array(&self) -> &[f64] {
        &self.parent_cost
    }

    pub fn is_in_tree(&self, node: MappedNodeId) -> bool {
        node.to_usize()
            .and_then(|index| self.in_tree.get(index))
            .copied()
            .unwrap_or(false)
    }

    pub fn prize(&self, node: MappedNodeId) -> f64 {
        node.to_usize()
            .and_then(|index| self.prizes.get(index))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn max_prize_node(&self) -> Option<MappedNodeId> {
        self.prizes
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|(index, _)| MappedNodeId::try_from(index).ok())
    }

    pub fn include_root(&mut self, root: MappedNodeId) {
        let Some(idx) = root.to_usize() else {
            return;
        };
        if idx >= self.node_count {
            return;
        }
        self.parent[idx] = PCSTreeParent::Root;
        self.parent_cost[idx] = 0.0;
        self.in_tree[idx] = true;
    }

    pub fn include_edge(&mut self, parent: MappedNodeId, node: MappedNodeId, weight: f64) {
        let Some(node_idx) = node.to_usize() else {
            return;
        };
        if node_idx >= self.node_count {
            return;
        }
        self.parent[node_idx] = PCSTreeParent::Parent(parent);
        self.parent_cost[node_idx] = weight;
        self.in_tree[node_idx] = true;
    }

    pub fn prune_negative_subtrees(&mut self) {
        // Identify root.
        let Some(root) = self
            .parent
            .iter()
            .enumerate()
            .find(|(_, &parent)| parent == PCSTreeParent::Root)
            .map(|(idx, _)| idx)
        else {
            return;
        };

        // Build children lists for current tree.
        let mut children: Vec<Vec<usize>> = vec![Vec::new(); self.node_count];
        for (node, &parent) in self.parent.iter().enumerate() {
            if let PCSTreeParent::Parent(parent) = parent {
                if let Some(parent_index) = parent.to_usize() {
                    children[parent_index].push(node);
                }
            }
        }

        // Post-order traversal.
        let mut stack: Vec<usize> = vec![root];
        let mut order: Vec<usize> = Vec::new();
        while let Some(n) = stack.pop() {
            order.push(n);
            for &c in &children[n] {
                stack.push(c);
            }
        }

        let mut subtree_value = vec![0.0_f64; self.node_count];
        for &n in order.iter().rev() {
            if self.parent[n] == PCSTreeParent::Pruned {
                subtree_value[n] = 0.0_f64;
                continue;
            }

            let mut value = self.prizes[n];
            if matches!(self.parent[n], PCSTreeParent::Parent(_)) {
                value -= self.parent_cost[n];
            }

            for &c in &children[n] {
                value += subtree_value[c].max(0.0_f64);
            }
            subtree_value[n] = value;
        }

        // Prune any child-subtree with non-positive value.
        let mut prune_stack: Vec<usize> = Vec::new();
        let mut walk: Vec<usize> = vec![root];
        while let Some(n) = walk.pop() {
            for &c in &children[n] {
                if subtree_value[c] <= 0.0 {
                    prune_stack.push(c);
                } else {
                    walk.push(c);
                }
            }
        }

        while let Some(n) = prune_stack.pop() {
            if self.parent[n] == PCSTreeParent::Pruned {
                continue;
            }
            self.parent[n] = PCSTreeParent::Pruned;
            self.parent_cost[n] = 0.0;
            self.in_tree[n] = false;
            for &c in &children[n] {
                prune_stack.push(c);
            }
        }
    }
}
