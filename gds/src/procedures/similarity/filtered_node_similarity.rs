use crate::algo::algorithms::similarity::build_similarity_relationship_store;
use crate::algo::algorithms::Result;
use crate::algo::algorithms::{ConfigValidator, WriteResult};
use crate::algo::similarity::filtered_node_similarity::compute_filtered_node_similarity;
use crate::algo::similarity::filtered_node_similarity::{
    FilteredNodeSimilarityMutateResult, FilteredNodeSimilarityResultBuilder,
    FilteredNodeSimilarityStats,
};
use crate::algo::similarity::node_similarity::{
    NodeSimilarityConfig, NodeSimilarityMetric, NodeSimilarityResult,
};
use crate::concurrency::TerminationFlag;
use crate::core::utils::progress::{ProgressTracker, Tasks};
use crate::mem::MemoryRange;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::projection::Orientation;
use crate::projection::{NodeLabel, RelationshipType};
use crate::types::prelude::{DefaultGraphStore, GraphStore};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

// Additional imports for progress tracking and node ID mapping
use crate::core::utils::progress::TaskProgressTracker;
use crate::types::graph::id_map::MappedNodeId;

pub struct FilteredNodeSimilarityFacade {
    graph_store: Arc<DefaultGraphStore>,
    metric: NodeSimilarityMetric,
    similarity_cutoff: f64,
    top_k: usize,
    top_n: usize,
    concurrency: usize,
    weight_property: Option<String>,
    source_node_labels: Vec<NodeLabel>,
    target_node_labels: Vec<NodeLabel>,
}

impl FilteredNodeSimilarityFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            metric: NodeSimilarityMetric::Jaccard,
            similarity_cutoff: 0.1,
            top_k: 10,
            top_n: 0,
            concurrency: 4,
            weight_property: None,
            source_node_labels: Vec::new(),
            target_node_labels: Vec::new(),
        }
    }

    pub fn metric(mut self, metric: NodeSimilarityMetric) -> Self {
        self.metric = metric;
        self
    }

    pub fn similarity_cutoff(mut self, cutoff: f64) -> Self {
        self.similarity_cutoff = cutoff;
        self
    }

    pub fn top_k(mut self, k: usize) -> Self {
        self.top_k = k;
        self
    }

    pub fn top_n(mut self, n: usize) -> Self {
        self.top_n = n;
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub fn weight_property(mut self, property: String) -> Self {
        self.weight_property = Some(property);
        self
    }

    pub fn source_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.source_node_labels = labels;
        self
    }

    pub fn target_labels(mut self, labels: Vec<NodeLabel>) -> Self {
        self.target_node_labels = labels;
        self
    }

    fn validate(&self) -> Result<()> {
        ConfigValidator::in_range(self.similarity_cutoff, 0.0, 1.0, "similarity_cutoff")?;
        ConfigValidator::in_range(self.top_k as f64, 1.0, 1_000_000.0, "top_k")?;
        ConfigValidator::in_range(self.top_n as f64, 0.0, 1_000_000.0, "top_n")?;
        ConfigValidator::in_range(self.concurrency as f64, 1.0, 1_000_000.0, "concurrency")?;
        if let Some(prop) = &self.weight_property {
            ConfigValidator::non_empty_string(prop, "weight_property")?;
        }

        for label in &self.source_node_labels {
            if !GraphStore::has_node_label(self.graph_store.as_ref(), label) {
                return Err(AlgorithmError::Execution(format!(
                    "Unknown sourceLabels entry '{}'",
                    label.name()
                )));
            }
        }

        for label in &self.target_node_labels {
            if !GraphStore::has_node_label(self.graph_store.as_ref(), label) {
                return Err(AlgorithmError::Execution(format!(
                    "Unknown targetLabels entry '{}'",
                    label.name()
                )));
            }
        }

        Ok(())
    }

    fn build_config(&self) -> NodeSimilarityConfig {
        NodeSimilarityConfig {
            similarity_metric: self.metric,
            similarity_cutoff: self.similarity_cutoff,
            top_k: self.top_k,
            top_n: self.top_n,
            concurrency: self.concurrency,
            weight_property: self.weight_property.clone(),
        }
    }

    fn compute_results(&self) -> Result<Vec<NodeSimilarityResult>> {
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "filtered_node_similarity".to_string(),
                self.graph_store.node_count(),
            ),
            self.concurrency,
        );
        let termination = TerminationFlag::running_true();
        Ok(self
            .compute_results_with_context(&mut progress_tracker, &termination)?
            .0)
    }

    fn compute_results_with_context(
        &self,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<(Vec<NodeSimilarityResult>, u64)> {
        self.validate()?;

        let rel_types: HashSet<RelationshipType> = self.graph_store.relationship_types();

        let graph = if let Some(prop) = self.weight_property.as_ref() {
            let selectors = rel_types
                .iter()
                .cloned()
                .map(|t| (t, prop.clone()))
                .collect::<HashMap<_, _>>();
            self.graph_store
                .get_graph_with_types_selectors_and_orientation(
                    &rel_types,
                    &selectors,
                    Orientation::Natural,
                )
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?
        } else {
            self.graph_store
                .get_graph_with_types_and_orientation(&rel_types, Orientation::Natural)
                .map_err(|e| AlgorithmError::InvalidGraph(e.to_string()))?
        };

        progress_tracker.begin_subtask_with_volume(graph.node_count());

        let id_map = GraphStore::nodes(self.graph_store.as_ref());

        let mut source_nodes: Option<HashSet<MappedNodeId>> = if self.source_node_labels.is_empty()
        {
            None
        } else {
            let labels: HashSet<NodeLabel> = self.source_node_labels.iter().cloned().collect();
            Some(id_map.iter_with_labels(&labels).collect())
        };

        let mut target_nodes: Option<HashSet<MappedNodeId>> = if self.target_node_labels.is_empty()
        {
            None
        } else {
            let labels: HashSet<NodeLabel> = self.target_node_labels.iter().cloned().collect();
            Some(id_map.iter_with_labels(&labels).collect())
        };

        if source_nodes.as_ref().is_some_and(|nodes| nodes.is_empty()) {
            return Err(AlgorithmError::Execution(
                "sourceLabels selection is empty".to_string(),
            ));
        }
        if target_nodes.as_ref().is_some_and(|nodes| nodes.is_empty()) {
            return Err(AlgorithmError::Execution(
                "targetLabels selection is empty".to_string(),
            ));
        }

        if source_nodes.is_none() {
            // If only target label is provided, treat it as both source+target filter.
            source_nodes = target_nodes.clone();
        }
        if target_nodes.is_none() {
            // If only source label is provided, treat it as both source+target filter.
            target_nodes = source_nodes.clone();
        }

        let config = self.build_config();
        let on_sources_done = Arc::new(|_n: usize| {});

        let report = match compute_filtered_node_similarity(
            graph.as_ref(),
            &config,
            source_nodes.as_ref(),
            target_nodes.as_ref(),
            termination,
            on_sources_done,
        ) {
            Ok(report) => report,
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                return Err(AlgorithmError::Execution(format!(
                    "Filtered node similarity terminated: {e}"
                )));
            }
        };

        progress_tracker.log_progress(report.completed_sources);

        progress_tracker.end_subtask();

        Ok((report.results, report.compared_nodes))
    }

    pub fn stream(self) -> Result<Box<dyn Iterator<Item = NodeSimilarityResult>>> {
        let results = self.compute_results()?;
        Ok(Box::new(results.into_iter()))
    }

    pub fn stream_with_context(
        self,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<Vec<NodeSimilarityResult>> {
        Ok(self
            .compute_results_with_context(progress_tracker, termination)?
            .0)
    }

    pub fn stats(self) -> Result<FilteredNodeSimilarityStats> {
        let (results, compared_nodes) = self.compute_results_with_internal_context()?;
        Ok(FilteredNodeSimilarityResultBuilder::new(&results)
            .stats_with_nodes_compared(compared_nodes))
    }

    fn compute_results_with_internal_context(&self) -> Result<(Vec<NodeSimilarityResult>, u64)> {
        let mut progress_tracker = TaskProgressTracker::with_concurrency(
            Tasks::leaf_with_volume(
                "filtered_node_similarity".to_string(),
                self.graph_store.node_count(),
            ),
            self.concurrency,
        );
        let termination = TerminationFlag::running_true();
        self.compute_results_with_context(&mut progress_tracker, &termination)
    }

    pub fn stats_with_context(
        self,
        progress_tracker: &mut dyn ProgressTracker,
        termination: &TerminationFlag,
    ) -> Result<FilteredNodeSimilarityStats> {
        let (results, compared_nodes) =
            self.compute_results_with_context(progress_tracker, termination)?;
        Ok(FilteredNodeSimilarityResultBuilder::new(&results)
            .stats_with_nodes_compared(compared_nodes))
    }

    pub fn mutate(self, property: &str) -> Result<FilteredNodeSimilarityMutateResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        let graph_store = Arc::clone(&self.graph_store);
        let start = std::time::Instant::now();
        let (results, compared_nodes) = self.compute_results_with_internal_context()?;
        let builder = FilteredNodeSimilarityResultBuilder::new(&results);
        let stats = builder.stats_with_nodes_compared(compared_nodes);

        let pairs: Vec<(u64, u64, f64)> = results
            .iter()
            .map(|r| (r.source, r.target, r.similarity))
            .collect();

        let updated_store =
            build_similarity_relationship_store(graph_store.as_ref(), property, &pairs)?;

        let relationships_updated = pairs.len() as u64;
        let summary = builder.mutation_summary(property, relationships_updated, start.elapsed());

        Ok(FilteredNodeSimilarityMutateResult {
            summary,
            stats,
            updated_store,
        })
    }

    pub fn write(self, property: &str) -> Result<WriteResult> {
        self.validate()?;
        ConfigValidator::non_empty_string(property, "property_name")?;

        let res = self.mutate(property)?;
        Ok(WriteResult::new(
            res.summary.nodes_updated,
            res.summary.property_name,
            std::time::Duration::from_millis(res.summary.execution_time_ms),
        ))
    }

    pub fn estimate_memory(&self) -> MemoryRange {
        let node_count = self.graph_store.node_count();

        // Worst-case is still bounded by top_k per node.
        let pair_count = node_count.saturating_mul(self.top_k);

        let results_memory = pair_count * 32;
        let per_node_scratch = node_count * 32;
        let weight_memory = if self.weight_property.is_some() {
            node_count * 8
        } else {
            0
        };

        // Label filtering can require temporary ID sets.
        let label_filter_memory = node_count * 8;

        let total = results_memory + per_node_scratch + weight_memory + label_filter_memory;
        let overhead = total / 5;
        MemoryRange::of_range(total, total + overhead)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algo::similarity::node_similarity::NodeSimilarityResult;
    use crate::config::GraphStoreConfig;
    use crate::types::graph::id_map::IdMap;
    use crate::types::graph::id_map::SimpleIdMap;
    use crate::types::graph::RelationshipTopology;
    use crate::types::graph_store::GraphName;
    use crate::types::graph_store::{Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation};
    use crate::types::prelude::GraphStore;
    use crate::types::random::RandomGraphConfig;
    use crate::types::schema::{Direction, GraphSchema, MutableGraphSchema};
    use std::cmp::Ordering;

    fn sort_results(mut rows: Vec<NodeSimilarityResult>) -> Vec<(u64, u64, f64)> {
        rows.sort_by(|a, b| {
            a.source
                .cmp(&b.source)
                .then_with(|| a.target.cmp(&b.target))
                .then_with(|| {
                    a.similarity
                        .partial_cmp(&b.similarity)
                        .unwrap_or(Ordering::Equal)
                })
        });
        rows.into_iter()
            .map(|r| (r.source, r.target, r.similarity))
            .collect()
    }

    #[test]
    fn filtered_node_similarity_filters_to_label_pair_space() {
        let cfg = GraphStoreConfig::default();
        let graph_name = GraphName::new("g");
        let db_info = DatabaseInfo::new(
            DatabaseId::new("test-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        );
        let capabilities = Capabilities::default();

        let label_a = NodeLabel::of("A");
        let label_b = NodeLabel::of("B");
        let rel_type = RelationshipType::of("R");

        let mut schema = MutableGraphSchema::empty();
        schema.node_schema_mut().add_label(label_a.clone());
        schema.node_schema_mut().add_label(label_b.clone());
        schema
            .relationship_schema_mut()
            .add_relationship_type(rel_type.clone(), Direction::Directed);
        let schema: GraphSchema = schema.build();

        let mut id_map = SimpleIdMap::from_original_ids([0, 1, 2, 3]);
        id_map.add_node_label(label_a.clone());
        id_map.add_node_label(label_b.clone());
        id_map.add_node_id_to_label(0, label_a.clone());
        id_map.add_node_id_to_label(1, label_a.clone());
        id_map.add_node_id_to_label(2, label_b.clone());
        id_map.add_node_id_to_label(3, label_b.clone());

        // Bipartite-ish: 0->2 and 1->2 gives 0 and 1 identical neighborhoods.
        let outgoing = vec![vec![2], vec![2], vec![], vec![]];
        let incoming = vec![vec![], vec![], vec![0, 1], vec![]];
        let topo = RelationshipTopology::new(outgoing, Some(incoming));
        let mut topologies = std::collections::HashMap::new();
        topologies.insert(rel_type, topo);

        let store = Arc::new(DefaultGraphStore::new(
            cfg,
            graph_name,
            db_info,
            schema,
            capabilities,
            id_map,
            topologies,
        ));

        let results: Vec<_> = FilteredNodeSimilarityFacade::new(Arc::clone(&store))
            .source_labels(vec![label_a.clone()])
            .target_labels(vec![label_a])
            .similarity_cutoff(0.0)
            .top_k(10)
            .concurrency(1)
            .stream()
            .unwrap()
            .collect();

        assert!(!results.is_empty());
        assert!(results.iter().all(|r| r.source < 2 && r.target < 2));
        assert!(results
            .iter()
            .any(|r| (r.source, r.target) == (0, 1) || (r.source, r.target) == (1, 0)));
    }

    #[test]
    fn mutate_adds_relationship_property() {
        let config = RandomGraphConfig {
            node_count: 10,
            seed: Some(9),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let result = FilteredNodeSimilarityFacade::new(Arc::clone(&store))
            .similarity_cutoff(0.0)
            .top_k(3)
            .concurrency(1)
            .mutate("sim_score")
            .unwrap();

        let rel_type = result
            .updated_store
            .relationship_types()
            .into_iter()
            .next()
            .expect("expected at least one relationship type");
        let _ = result
            .updated_store
            .relationship_property_values(&rel_type, "sim_score")
            .unwrap();
    }

    #[test]
    fn filtered_stream_parallel_matches_single_worker() {
        let config = RandomGraphConfig {
            node_count: 14,
            seed: Some(19),
            ..RandomGraphConfig::default()
        };
        let store = Arc::new(DefaultGraphStore::random(&config).unwrap());

        let single = FilteredNodeSimilarityFacade::new(Arc::clone(&store))
            .similarity_cutoff(0.0)
            .top_k(4)
            .concurrency(1)
            .stream()
            .unwrap()
            .collect::<Vec<_>>();

        let parallel = FilteredNodeSimilarityFacade::new(store)
            .similarity_cutoff(0.0)
            .top_k(4)
            .concurrency(4)
            .stream()
            .unwrap()
            .collect::<Vec<_>>();

        assert_eq!(sort_results(single), sort_results(parallel));
    }
}
