//! KGE Predict Facade
//!
//! Procedure-facing facade for KGE link prediction.
//!
//! Layering:
//! - Applications layer parses JSON and calls this facade.
//! - This facade owns procedure semantics (defaults, id-mapping, timings later).
//! - The compute kernel lives in `crate::algo::algorithms::machine_learning::kge`.

pub use crate::algo::algorithms::machine_learning::kge::ScoreFunction;
use crate::algo::algorithms::machine_learning::kge::{
    compute_kge_predict, KgePredictParameters, KgePredictResult,
};
use crate::algo::algorithms::Result;
use crate::task::concurrency::TerminationFlag;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::types::graph::id_map::IdMap;
use crate::types::graph::MappedNodeId;
use crate::types::graph::OriginalNodeId;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

/// Stream row for `gds.*.kge.stream`.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct KgeStreamResult {
    pub source_node_id: i64,
    pub target_node_id: i64,
    pub score: f64,
}

/// Stats payload for `gds.*.kge.stats`.
#[derive(Debug, Clone, serde::Serialize)]
pub struct KgePredictStats {
    pub relationships_written: u64,
    pub links_considered: u64,
}

/// Procedure facade for KGE predict.
#[derive(Clone)]
pub struct KgePredictFacade {
    graph_store: Arc<DefaultGraphStore>,
    node_embedding_property: String,
    relationship_type_embedding: Vec<f64>,
    scoring_function: ScoreFunction,
    top_k: usize,
    source_nodes: Option<Vec<i64>>,
    target_nodes: Option<Vec<i64>>,
}

impl KgePredictFacade {
    pub fn new(graph_store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_store,
            node_embedding_property: "embedding".to_string(),
            relationship_type_embedding: Vec::new(),
            scoring_function: ScoreFunction::Distmult,
            top_k: 10,
            source_nodes: None,
            target_nodes: None,
        }
    }

    pub fn node_embedding_property(mut self, key: impl Into<String>) -> Self {
        self.node_embedding_property = key.into();
        self
    }

    pub fn relationship_type_embedding(mut self, embedding: Vec<f64>) -> Self {
        self.relationship_type_embedding = embedding;
        self
    }

    pub fn scoring_function(mut self, scoring_function: ScoreFunction) -> Self {
        self.scoring_function = scoring_function;
        self
    }

    pub fn top_k(mut self, top_k: usize) -> Self {
        self.top_k = top_k;
        self
    }

    pub fn source_nodes(mut self, source_nodes: Vec<i64>) -> Self {
        self.source_nodes = Some(source_nodes);
        self
    }

    pub fn target_nodes(mut self, target_nodes: Vec<i64>) -> Self {
        self.target_nodes = Some(target_nodes);
        self
    }

    fn compute(&self) -> Result<KgePredictResult> {
        let graph = self.graph_store.graph();

        let map_nodes = |node_ids: &Option<Vec<i64>>, role: &str| -> Result<Option<Vec<i64>>> {
            node_ids
                .as_ref()
                .map(|ids| {
                    ids.iter()
                        .map(|&node_id| {
                            let original = OriginalNodeId::new(node_id);
                            let mapped = graph.safe_to_mapped_node_id(original).ok_or_else(|| {
                                AlgorithmError::Execution(format!(
                                    "KGE {role} node {original} is not present in the graph"
                                ))
                            })?;
                            i64::try_from(u64::from(mapped)).map_err(|_| {
                                AlgorithmError::Execution(format!(
                                    "KGE {role} node {original} exceeds the kernel ID domain"
                                ))
                            })
                        })
                        .collect()
                })
                .transpose()
        };

        let params = KgePredictParameters {
            node_embedding_property: self.node_embedding_property.clone(),
            relationship_type_embedding: self.relationship_type_embedding.clone(),
            scoring_function: self.scoring_function,
            top_k: self.top_k,
            source_nodes: map_nodes(&self.source_nodes, "source")?,
            target_nodes: map_nodes(&self.target_nodes, "target")?,
        };

        let termination_flag = TerminationFlag::running_true();

        compute_kge_predict(graph.as_ref(), &params, &termination_flag)
            .map_err(|e| AlgorithmError::Execution(format!("KGE failed: {e}")))
    }

    /// Stream mode: per-(source,target) predicted link score.
    pub fn stream(self) -> Result<Box<dyn Iterator<Item = KgeStreamResult>>> {
        let graph = self.graph_store.graph();
        let result = self.compute()?;

        let rows: Vec<KgeStreamResult> = result
            .iter()
            .map(|row| -> Result<KgeStreamResult> {
                let source = MappedNodeId::try_from(row.source_node_id).map_err(|_| {
                    AlgorithmError::Execution(format!(
                        "KGE returned invalid mapped source node {}",
                        row.source_node_id
                    ))
                })?;
                let target = MappedNodeId::try_from(row.target_node_id).map_err(|_| {
                    AlgorithmError::Execution(format!(
                        "KGE returned invalid mapped target node {}",
                        row.target_node_id
                    ))
                })?;
                let source = graph.to_original_node_id(source).ok_or_else(|| {
                    AlgorithmError::Execution(format!("KGE returned unknown source node {source}"))
                })?;
                let target = graph.to_original_node_id(target).ok_or_else(|| {
                    AlgorithmError::Execution(format!("KGE returned unknown target node {target}"))
                })?;

                Ok(KgeStreamResult {
                    source_node_id: source.get(),
                    target_node_id: target.get(),
                    score: row.score,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Box::new(rows.into_iter()))
    }

    /// Stats mode: aggregate counts.
    pub fn stats(self) -> Result<KgePredictStats> {
        let result = self.compute()?;
        Ok(KgePredictStats {
            relationships_written: result.relationship_count(),
            links_considered: result.links_considered(),
        })
    }
}
