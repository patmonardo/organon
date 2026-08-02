use std::sync::Arc;

use crate::types::graph_store::GraphStoreRead;
use crate::types::prelude::{DefaultGraphStore, GraphStore, ShellStoreControl};

use super::pathfinding::{
    AStarBuilder, AllShortestPathsBuilder, BellmanFordBuilder, BfsBuilder, DagLongestPathBuilder,
    DeltaSteppingBuilder, DfsBuilder, DijkstraBuilder, KSpanningTreeBuilder, RandomWalkBuilder,
    SpanningTreeBuilder, SteinerTreeBuilder, TopologicalSortBuilder, YensBuilder,
};

use super::centrality::{
    ArticulationPointsFacade, BetweennessCentralityFacade, BridgesFacade, CELFFacade,
    ClosenessCentralityFacade, DegreeCentralityFacade, HarmonicCentralityFacade,
    HitsCentralityFacade, PageRankFacade,
};

use super::community::{
    ApproxMaxKCutFacade, ConductanceFacade, K1ColoringFacade, KCoreFacade, KMeansFacade,
    LabelPropagationFacade, LeidenFacade, LouvainFacade, ModularityFacade, SccFacade,
    TriangleFacade, WccFacade,
};

use super::embeddings::{FastRPBuilder, GraphSageBuilder, HashGNNBuilder, Node2VecBuilder};

use super::miscellaneous::{
    CollapsePathFacade, IndexInverseFacade, ScalePropertiesFacade, ToUndirectedFacade,
};

use super::similarity::FilteredKnnFacade;
use super::similarity::FilteredNodeSimilarityFacade;
use super::similarity::KnnFacade;
use super::similarity::NodeSimilarityFacade;

/// User-facing graph handle for running algorithms against a live graph store.
///
/// This is the main entrypoint for the facade layer.
#[derive(Clone)]
pub struct GraphFacade<Store: GraphStoreRead + ?Sized = DefaultGraphStore> {
    store: Arc<Store>,
}

impl<Store: GraphStoreRead + ?Sized> GraphFacade<Store> {
    /// Create a facade graph handle from a readable graph store.
    pub fn new(store: Arc<Store>) -> Self {
        Self { store }
    }

    /// Access the underlying graph store.
    pub fn store(&self) -> &Arc<Store> {
        &self.store
    }

    /// Dijkstra shortest-paths.
    pub fn dijkstra(&self) -> DijkstraBuilder<Store> {
        DijkstraBuilder::new(Arc::clone(&self.store))
    }

    /// Breadth-first search traversal.
    pub fn bfs(&self) -> BfsBuilder<Store> {
        BfsBuilder::new(Arc::clone(&self.store))
    }

    /// Depth-first search traversal.
    pub fn dfs(&self) -> DfsBuilder<Store> {
        DfsBuilder::new(Arc::clone(&self.store))
    }

    /// A* shortest-path (heuristic-guided).
    pub fn astar(&self) -> AStarBuilder<Store> {
        AStarBuilder::new(Arc::clone(&self.store))
    }

    /// Bellman-Ford shortest-paths (supports negative weights; detects negative cycles).
    pub fn bellman_ford(&self) -> BellmanFordBuilder<Store> {
        BellmanFordBuilder::new(Arc::clone(&self.store))
    }

    /// Delta Stepping shortest-paths (binning strategy).
    pub fn delta_stepping(&self) -> DeltaSteppingBuilder<Store> {
        DeltaSteppingBuilder::new(Arc::clone(&self.store))
    }

    /// Yen's K-shortest simple paths (single-pair).
    pub fn yens(&self) -> YensBuilder<Store> {
        YensBuilder::new(Arc::clone(&self.store))
    }

    /// All-pairs shortest path distances.
    pub fn all_shortest_paths(&self) -> AllShortestPathsBuilder<Store> {
        AllShortestPathsBuilder::new(Arc::clone(&self.store))
    }

    /// Spanning tree via Prim's algorithm.
    pub fn spanning_tree(&self) -> SpanningTreeBuilder<Store> {
        SpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    /// K-spanning tree (prune MST to exactly k nodes).
    pub fn kspanning_tree(&self) -> KSpanningTreeBuilder<Store> {
        KSpanningTreeBuilder::new(Arc::clone(&self.store))
    }

    /// Steiner tree (minimum tree connecting source to terminals).
    pub fn steiner_tree(&self) -> SteinerTreeBuilder<Store> {
        SteinerTreeBuilder::new(Arc::clone(&self.store))
    }

    /// Topological sort for directed acyclic graphs (DAG).
    pub fn topological_sort(&self) -> TopologicalSortBuilder<Store> {
        TopologicalSortBuilder::new(Arc::clone(&self.store))
    }

    /// Longest path in directed acyclic graphs (DAG).
    pub fn dag_longest_path(&self) -> DagLongestPathBuilder<Store> {
        DagLongestPathBuilder::new(Arc::clone(&self.store))
    }

    pub fn random_walk(&self) -> RandomWalkBuilder<Store> {
        RandomWalkBuilder::new(Arc::clone(&self.store))
    }
}

impl<Store: GraphStore> GraphFacade<Store> {
    pub fn node2vec(&self) -> Node2VecBuilder<Store> {
        Node2VecBuilder::new(Arc::clone(&self.store))
    }

    /// FastRP node embeddings.
    pub fn fast_rp(&self) -> FastRPBuilder<Store> {
        FastRPBuilder::new(Arc::clone(&self.store))
    }

    /// HashGNN node embeddings.
    pub fn hash_gnn(&self) -> HashGNNBuilder<Store> {
        HashGNNBuilder::new(Arc::clone(&self.store))
    }

    /// GraphSAGE node embeddings (inductive representation learning).
    pub fn graphsage(&self) -> GraphSageBuilder<Store> {
        GraphSageBuilder::new(Arc::clone(&self.store))
    }

    /// Degree centrality (counts connections per node).
    pub fn degree_centrality(&self) -> DegreeCentralityFacade<Store> {
        DegreeCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Closeness centrality (distance-based centrality).
    pub fn closeness(&self) -> ClosenessCentralityFacade<Store> {
        ClosenessCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Harmonic centrality (reciprocal distances).
    pub fn harmonic(&self) -> HarmonicCentralityFacade<Store> {
        HarmonicCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Betweenness centrality (Brandes shortest-path dependency).
    pub fn betweenness(&self) -> BetweennessCentralityFacade<Store> {
        BetweennessCentralityFacade::new(Arc::clone(&self.store))
    }

    /// PageRank (delta-based, Java GDS aligned).
    pub fn pagerank(&self) -> PageRankFacade<Store> {
        PageRankFacade::new(Arc::clone(&self.store))
    }

    /// HITS (bidirectional authority/hub scoring).
    pub fn hits(&self) -> HitsCentralityFacade<Store> {
        HitsCentralityFacade::new(Arc::clone(&self.store))
    }

    /// Articulation Points (cut vertices) for undirected connectivity.
    pub fn articulation_points(&self) -> ArticulationPointsFacade<Store> {
        ArticulationPointsFacade::new(Arc::clone(&self.store))
    }

    /// Bridges (cut edges) for undirected graphs.
    pub fn bridges(&self) -> BridgesFacade<Store> {
        BridgesFacade::new(Arc::clone(&self.store))
    }

    /// CELF (Cost-Effective Lazy Forward) influence maximization.
    pub fn celf(&self) -> CELFFacade<Store> {
        CELFFacade::new(Arc::clone(&self.store))
    }

    /// Triangle Count (per-node triangles + global triangle count).
    pub fn triangle(&self) -> TriangleFacade<Store> {
        TriangleFacade::new(Arc::clone(&self.store))
    }

    /// Strongly Connected Components (directed graph SCCs).
    pub fn scc(&self) -> SccFacade<Store> {
        SccFacade::new(Arc::clone(&self.store))
    }

    /// Weakly Connected Components (undirected connectivity).
    pub fn wcc(&self) -> WccFacade<Store> {
        WccFacade::new(Arc::clone(&self.store))
    }

    /// K1-Coloring (greedy graph coloring).
    pub fn k1coloring(&self) -> K1ColoringFacade<Store> {
        K1ColoringFacade::new(Arc::clone(&self.store))
    }

    /// K-Core Decomposition (core values).
    pub fn kcore(&self) -> KCoreFacade<Store> {
        KCoreFacade::new(Arc::clone(&self.store))
    }

    /// Label Propagation (community detection via label voting).
    pub fn label_propagation(&self) -> LabelPropagationFacade<Store> {
        LabelPropagationFacade::new(Arc::clone(&self.store))
    }

    /// Louvain community detection (modularity optimization).
    pub fn louvain(&self) -> LouvainFacade<Store> {
        LouvainFacade::new(Arc::clone(&self.store))
    }

    /// K-Means clustering (community detection on feature vectors).
    pub fn kmeans(&self) -> KMeansFacade<Store> {
        KMeansFacade::new(Arc::clone(&self.store))
    }

    /// Leiden community detection (modularity refinement with connected components).
    pub fn leiden(&self) -> LeidenFacade<Store> {
        LeidenFacade::new(Arc::clone(&self.store))
    }

    /// Conductance community quality metric.
    pub fn conductance(&self, community_property: String) -> ConductanceFacade<Store> {
        ConductanceFacade::new(Arc::clone(&self.store), community_property)
    }

    /// Approximate maximum k-cut partitioning.
    pub fn approx_max_kcut(&self) -> ApproxMaxKCutFacade<Store> {
        ApproxMaxKCutFacade::new(Arc::clone(&self.store))
    }

    /// Modularity community quality metric.
    pub fn modularity(&self, community_property: String) -> ModularityFacade<Store> {
        ModularityFacade::new(Arc::clone(&self.store), community_property)
    }

    /// Node Similarity (Jaccard, Cosine, Overlap).
    pub fn node_similarity(&self) -> NodeSimilarityFacade<Store> {
        NodeSimilarityFacade::new(Arc::clone(&self.store))
    }

    /// K-nearest neighbors over one or more node properties.
    pub fn knn(&self, node_property: impl Into<String>) -> KnnFacade<Store> {
        KnnFacade::new(Arc::clone(&self.store), node_property)
    }

    /// Label-filtered K-nearest neighbors over one or more node properties.
    pub fn filtered_knn(&self, node_property: impl Into<String>) -> FilteredKnnFacade<Store> {
        FilteredKnnFacade::new(Arc::clone(&self.store), node_property)
    }

    /// Label-filtered Node Similarity.
    pub fn filtered_node_similarity(&self) -> FilteredNodeSimilarityFacade<Store> {
        FilteredNodeSimilarityFacade::new(Arc::clone(&self.store))
    }

    /// Scales a numeric node property (utility).
    pub fn scale_properties(&self) -> ScalePropertiesFacade<Store> {
        ScalePropertiesFacade::new(Arc::clone(&self.store))
    }
}

impl<Store: GraphStore + ShellStoreControl> GraphFacade<Store> {
    /// Converts the graph to an undirected projection (utility).
    pub fn to_undirected(&self) -> ToUndirectedFacade<Store> {
        ToUndirectedFacade::new(Arc::clone(&self.store))
    }

    /// Builds inverse indices for relationships (placeholder utility).
    pub fn index_inverse(&self) -> IndexInverseFacade<Store> {
        IndexInverseFacade::new(Arc::clone(&self.store))
    }

    /// Collapses paths into relationships (placeholder utility).
    pub fn collapse_path(&self) -> CollapsePathFacade<Store> {
        CollapsePathFacade::new(Arc::clone(&self.store))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::random::RandomGraphConfig;

    #[test]
    fn facade_accepts_an_erased_readable_store() {
        let store: Arc<dyn GraphStoreRead> = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::default()).expect("random graph store"),
        );
        let facade = GraphFacade::new(store);

        assert_eq!(
            facade.store().node_count(),
            facade.store().get_graph().node_count()
        );
        let memory = facade.dijkstra().source(0).estimate_memory();
        assert!(memory.max() >= memory.min());
    }
}
