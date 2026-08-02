//! Typed local Procedure bindings for Shell Component calls.

mod algorithms;
mod inputs;
mod pipelines;

use crate::algo::algorithms::pathfinding::PathResult;
use crate::algo::algorithms::CentralityScore;
use crate::algo::algorithms::WriteResult;
use crate::algo::all_shortest_paths::AllShortestPathsMutateResult;
use crate::algo::all_shortest_paths::AllShortestPathsStats;
use crate::algo::approx_max_kcut::ApproxMaxKCutMutateResult;
use crate::algo::approx_max_kcut::ApproxMaxKCutStats;
use crate::algo::articulation_points::ArticulationPointRow;
use crate::algo::articulation_points::ArticulationPointsMutateResult;
use crate::algo::articulation_points::ArticulationPointsStats;
use crate::algo::astar::AStarMutateResult;
use crate::algo::astar::AStarStats;
use crate::algo::astar::AStarWriteSummary;
use crate::algo::bellman_ford::BellmanFordMutateResult;
use crate::algo::bellman_ford::BellmanFordStats;
use crate::algo::bellman_ford::BellmanFordWriteSummary;
use crate::algo::betweenness::BetweennessCentralityMutateResult;
use crate::algo::betweenness::BetweennessCentralityStats;
use crate::algo::bfs::BfsMutateResult;
use crate::algo::bfs::BfsStats;
use crate::algo::bfs::BfsWriteSummary;
use crate::algo::bridges::BridgesMutateResult;
use crate::algo::bridges::BridgesRow;
use crate::algo::bridges::BridgesStats;
use crate::algo::celf::CELFMutateResult;
use crate::algo::celf::CELFRow;
use crate::algo::celf::CELFStats;
use crate::algo::closeness::ClosenessCentralityMutateResult;
use crate::algo::closeness::ClosenessCentralityStats;
use crate::algo::conductance::ConductanceMutateResult;
use crate::algo::conductance::ConductanceStats;
use crate::algo::dag_longest_path::DagLongestPathMutateResult;
use crate::algo::dag_longest_path::DagLongestPathRow;
use crate::algo::dag_longest_path::DagLongestPathStats;
use crate::algo::degree_centrality::DegreeCentralityMutateResult;
use crate::algo::degree_centrality::DegreeCentralityStats;
use crate::algo::delta_stepping::DeltaSteppingMutateResult;
use crate::algo::delta_stepping::DeltaSteppingStats;
use crate::algo::delta_stepping::DeltaSteppingWriteSummary;
use crate::algo::dfs::DfsMutateResult;
use crate::algo::dfs::DfsStats;
use crate::algo::dfs::DfsWriteSummary;
use crate::algo::dijkstra::DijkstraMutateResult;
use crate::algo::dijkstra::DijkstraStats;
use crate::algo::dijkstra::DijkstraWriteSummary;
use crate::algo::harmonic::HarmonicCentralityMutateResult;
use crate::algo::harmonic::HarmonicCentralityStats;
use crate::algo::hits::HitsCentralityMutateResult;
use crate::algo::hits::HitsCentralityStats;
use crate::algo::k1coloring::K1ColoringMutateResult;
use crate::algo::k1coloring::K1ColoringStats;
use crate::algo::kspanningtree::KSpanningTreeMutateResult;
use crate::algo::kspanningtree::KSpanningTreeRow;
use crate::algo::kspanningtree::KSpanningTreeStats;
use crate::algo::pagerank::PageRankMutateResult;
use crate::algo::pagerank::PageRankStats;
use crate::algo::random_walk::RandomWalkMutateResult;
use crate::algo::random_walk::RandomWalkRow;
use crate::algo::random_walk::RandomWalkStats;
use crate::algo::random_walk::RandomWalkWriteSummary;
use crate::algo::spanning_tree::SpanningTreeMutateResult;
use crate::algo::spanning_tree::SpanningTreeRow;
use crate::algo::spanning_tree::SpanningTreeStats;
use crate::algo::spanning_tree::SpanningTreeWriteSummary;
use crate::algo::steiner_tree::SteinerTreeMutateResult;
use crate::algo::steiner_tree::SteinerTreeRow;
use crate::algo::steiner_tree::SteinerTreeStats;
use crate::algo::topological_sort::TopologicalSortMutateResult;
use crate::algo::topological_sort::TopologicalSortRow;
use crate::algo::topological_sort::TopologicalSortStats;
use crate::algo::topological_sort::TopologicalSortWriteSummary;
use crate::algo::yens::YensMutateResult;
use crate::algo::yens::YensStats;
use crate::algo::yens::YensWriteSummary;
use crate::procedures::centrality::ArticulationPointsFacade;
use crate::procedures::centrality::BetweennessCentralityFacade;
use crate::procedures::centrality::BridgesFacade;
use crate::procedures::centrality::CELFFacade;
use crate::procedures::centrality::ClosenessCentralityFacade;
use crate::procedures::centrality::DegreeCentralityFacade;
use crate::procedures::centrality::HarmonicCentralityFacade;
use crate::procedures::centrality::HitsCentralityFacade;
use crate::procedures::centrality::PageRankFacade;
use crate::procedures::community::ApproxMaxKCutFacade;
use crate::procedures::community::ApproxMaxKCutRow;
use crate::procedures::community::ConductanceFacade;
use crate::procedures::community::ConductanceRow;
use crate::procedures::community::K1ColoringFacade;
use crate::procedures::community::K1ColoringRow;
use crate::procedures::pathfinding::AStarBuilder;
use crate::procedures::pathfinding::AllShortestPathsBuilder;
use crate::procedures::pathfinding::AllShortestPathsRow;
use crate::procedures::pathfinding::BellmanFordBuilder;
use crate::procedures::pathfinding::BfsBuilder;
use crate::procedures::pathfinding::DagLongestPathBuilder;
use crate::procedures::pathfinding::DeltaSteppingBuilder;
use crate::procedures::pathfinding::DfsBuilder;
use crate::procedures::pathfinding::DijkstraBuilder;
use crate::procedures::pathfinding::KSpanningTreeBuilder;
use crate::procedures::pathfinding::RandomWalkBuilder;
use crate::procedures::pathfinding::SpanningTreeBuilder;
use crate::procedures::pathfinding::SteinerTreeBuilder;
use crate::procedures::pathfinding::TopologicalSortBuilder;
use crate::procedures::pathfinding::YensBuilder;
use crate::procedures::pipelines::LocalPipelinesProcedureFacade;
use crate::procedures::pipelines::NodePipelineInfoResult;
use crate::procedures::pipelines::PipelineCatalogResult;
use crate::procedures::pipelines::PipelineExistsResult;
use crate::procedures::pipelines::PipelineInfoResult;
use crate::projection::eval::algorithm::AlgorithmError;
use crate::shell::builtin_component;
use crate::shell::ShellComponentCall;
use crate::shell::ShellComponentCategory;
use crate::shell::ShellComponentId;
use crate::shell::ShellComponentMode;
use crate::task::memory::MemoryRange;
use std::sync::Arc;

use super::GraphFacade;

pub enum ShellProcedureBinding {
    ApproxMaxKCut {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: ApproxMaxKCutFacade,
        output_property: Option<String>,
    },
    ArticulationPoints {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: ArticulationPointsFacade,
        output_property: Option<String>,
    },
    AllShortestPaths {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: AllShortestPathsBuilder,
        output_property: Option<String>,
    },
    AStar {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: AStarBuilder,
        output_property: Option<String>,
    },
    BellmanFord {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: BellmanFordBuilder,
        output_property: Option<String>,
    },
    Betweenness {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: BetweennessCentralityFacade,
        output_property: Option<String>,
    },
    Bfs {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: BfsBuilder,
        output_property: Option<String>,
    },
    Bridges {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: BridgesFacade,
        output_property: Option<String>,
    },
    Celf {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: CELFFacade,
        output_property: Option<String>,
    },
    Closeness {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: ClosenessCentralityFacade,
        output_property: Option<String>,
    },
    Conductance {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: ConductanceFacade,
        output_property: Option<String>,
    },
    DegreeCentrality {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: DegreeCentralityFacade,
        output_property: Option<String>,
    },
    Dfs {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: DfsBuilder,
        output_property: Option<String>,
    },
    DeltaStepping {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: DeltaSteppingBuilder,
        output_property: Option<String>,
    },
    DagLongestPath {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: DagLongestPathBuilder,
        output_property: Option<String>,
    },
    Dijkstra {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: DijkstraBuilder,
        output_property: Option<String>,
    },
    Harmonic {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: HarmonicCentralityFacade,
        output_property: Option<String>,
    },
    Hits {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: HitsCentralityFacade,
        output_property: Option<String>,
    },
    K1Coloring {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: K1ColoringFacade,
        output_property: Option<String>,
    },
    KSpanningTree {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: KSpanningTreeBuilder,
        output_property: Option<String>,
    },
    PageRank {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: PageRankFacade,
        output_property: Option<String>,
    },
    RandomWalk {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: RandomWalkBuilder,
        output_property: Option<String>,
    },
    SpanningTree {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: SpanningTreeBuilder,
        output_property: Option<String>,
    },
    SteinerTree {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: SteinerTreeBuilder,
        output_property: Option<String>,
    },
    TopologicalSort {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: TopologicalSortBuilder,
        output_property: Option<String>,
    },
    Yens {
        component: ShellComponentId,
        mode: ShellComponentMode,
        procedure: YensBuilder,
        output_property: Option<String>,
    },
    Pipeline {
        component: ShellComponentId,
        mode: ShellComponentMode,
        facade: Arc<LocalPipelinesProcedureFacade>,
        procedure: ShellPipelineProcedure,
    },
}

impl ShellProcedureBinding {
    pub fn component(&self) -> ShellComponentId {
        match self {
            Self::ApproxMaxKCut { component, .. }
            | Self::ArticulationPoints { component, .. }
            | Self::AllShortestPaths { component, .. }
            | Self::AStar { component, .. }
            | Self::BellmanFord { component, .. }
            | Self::Betweenness { component, .. }
            | Self::Bfs { component, .. }
            | Self::Bridges { component, .. }
            | Self::Celf { component, .. }
            | Self::Closeness { component, .. }
            | Self::Conductance { component, .. }
            | Self::DegreeCentrality { component, .. }
            | Self::Dfs { component, .. }
            | Self::DeltaStepping { component, .. }
            | Self::DagLongestPath { component, .. }
            | Self::Dijkstra { component, .. }
            | Self::Harmonic { component, .. }
            | Self::Hits { component, .. }
            | Self::K1Coloring { component, .. }
            | Self::KSpanningTree { component, .. }
            | Self::PageRank { component, .. }
            | Self::RandomWalk { component, .. }
            | Self::SpanningTree { component, .. }
            | Self::SteinerTree { component, .. }
            | Self::TopologicalSort { component, .. }
            | Self::Yens { component, .. }
            | Self::Pipeline { component, .. } => *component,
        }
    }

    pub fn mode(&self) -> ShellComponentMode {
        match self {
            Self::ApproxMaxKCut { mode, .. }
            | Self::ArticulationPoints { mode, .. }
            | Self::AllShortestPaths { mode, .. }
            | Self::AStar { mode, .. }
            | Self::BellmanFord { mode, .. }
            | Self::Betweenness { mode, .. }
            | Self::Bfs { mode, .. }
            | Self::Bridges { mode, .. }
            | Self::Celf { mode, .. }
            | Self::Closeness { mode, .. }
            | Self::Conductance { mode, .. }
            | Self::DegreeCentrality { mode, .. }
            | Self::Dfs { mode, .. }
            | Self::DeltaStepping { mode, .. }
            | Self::DagLongestPath { mode, .. }
            | Self::Dijkstra { mode, .. }
            | Self::Harmonic { mode, .. }
            | Self::Hits { mode, .. }
            | Self::K1Coloring { mode, .. }
            | Self::KSpanningTree { mode, .. }
            | Self::PageRank { mode, .. }
            | Self::RandomWalk { mode, .. }
            | Self::SpanningTree { mode, .. }
            | Self::SteinerTree { mode, .. }
            | Self::TopologicalSort { mode, .. }
            | Self::Yens { mode, .. }
            | Self::Pipeline { mode, .. } => *mode,
        }
    }

    pub fn invoke(self) -> Result<ShellProcedureResult, ShellProcedureError> {
        match self {
            Self::ApproxMaxKCut {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_approx_max_kcut(mode, procedure, output_property),
            Self::ArticulationPoints {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_articulation_points(mode, procedure, output_property),
            Self::AllShortestPaths {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_all_shortest_paths(mode, procedure, output_property),
            Self::AStar {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_astar(mode, procedure, output_property),
            Self::BellmanFord {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_bellman_ford(mode, procedure, output_property),
            Self::Betweenness {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_betweenness(mode, procedure, output_property),
            Self::Bfs {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_bfs(mode, procedure, output_property),
            Self::Bridges {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_bridges(mode, procedure, output_property),
            Self::Celf {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_celf(mode, procedure, output_property),
            Self::Closeness {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_closeness(mode, procedure, output_property),
            Self::Conductance {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_conductance(mode, procedure, output_property),
            Self::DegreeCentrality {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_degree_centrality(mode, procedure, output_property),
            Self::Dfs {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_dfs(mode, procedure, output_property),
            Self::DeltaStepping {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_delta_stepping(mode, procedure, output_property),
            Self::DagLongestPath {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_dag_longest_path(mode, procedure, output_property),
            Self::Dijkstra {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_dijkstra(mode, procedure, output_property),
            Self::Harmonic {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_harmonic(mode, procedure, output_property),
            Self::Hits {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_hits(mode, procedure, output_property),
            Self::K1Coloring {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_k1coloring(mode, procedure, output_property),
            Self::KSpanningTree {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_kspanning_tree(mode, procedure, output_property),
            Self::PageRank {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_pagerank(mode, procedure, output_property),
            Self::RandomWalk {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_random_walk(mode, procedure, output_property),
            Self::SpanningTree {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_spanning_tree(mode, procedure, output_property),
            Self::SteinerTree {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_steiner_tree(mode, procedure, output_property),
            Self::TopologicalSort {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_topological_sort(mode, procedure, output_property),
            Self::Yens {
                mode,
                procedure,
                output_property,
                ..
            } => algorithms::invoke_yens(mode, procedure, output_property),
            Self::Pipeline {
                facade, procedure, ..
            } => pipelines::invoke_pipeline(facade.as_ref(), procedure),
        }
    }
}

pub enum ShellPipelineProcedure {
    CreateLinkPrediction {
        pipeline_name: String,
    },
    CreateNodeClassification {
        pipeline_name: String,
    },
    CreateNodeRegression {
        pipeline_name: String,
    },
    List {
        pipeline_name: Option<String>,
    },
    Exists {
        pipeline_name: String,
    },
    Drop {
        pipeline_name: String,
        fail_if_missing: bool,
    },
}

pub enum ShellProcedureResult {
    ApproxMaxKCutStream(Vec<ApproxMaxKCutRow>),
    ApproxMaxKCutStats(ApproxMaxKCutStats),
    ApproxMaxKCutEstimate(MemoryRange),
    ApproxMaxKCutMutate(ApproxMaxKCutMutateResult),
    ApproxMaxKCutWrite(WriteResult),
    ArticulationPointsStream(Vec<ArticulationPointRow>),
    ArticulationPointsStats(ArticulationPointsStats),
    ArticulationPointsEstimate(MemoryRange),
    ArticulationPointsMutate(ArticulationPointsMutateResult),
    ArticulationPointsWrite(WriteResult),
    AllShortestPathsStream(Vec<AllShortestPathsRow>),
    AllShortestPathsStats(AllShortestPathsStats),
    AllShortestPathsEstimate(MemoryRange),
    AllShortestPathsMutate(AllShortestPathsMutateResult),
    AllShortestPathsWrite(WriteResult),
    AStarStream(Vec<PathResult>),
    AStarStats(AStarStats),
    AStarEstimate(MemoryRange),
    AStarMutate(AStarMutateResult),
    AStarWrite(AStarWriteSummary),
    BellmanFordStream(Vec<PathResult>),
    BellmanFordStats(BellmanFordStats),
    BellmanFordEstimate(MemoryRange),
    BellmanFordMutate(BellmanFordMutateResult),
    BellmanFordWrite(BellmanFordWriteSummary),
    BetweennessStream(Vec<CentralityScore>),
    BetweennessStats(BetweennessCentralityStats),
    BetweennessEstimate(MemoryRange),
    BetweennessMutate(BetweennessCentralityMutateResult),
    BetweennessWrite(WriteResult),
    BfsStream(Vec<PathResult>),
    BfsStats(BfsStats),
    BfsEstimate(MemoryRange),
    BfsMutate(BfsMutateResult),
    BfsWrite(BfsWriteSummary),
    BridgesStream(Vec<BridgesRow>),
    BridgesStats(BridgesStats),
    BridgesEstimate(MemoryRange),
    BridgesMutate(BridgesMutateResult),
    BridgesWrite(WriteResult),
    CelfStream(Vec<CELFRow>),
    CelfStats(CELFStats),
    CelfEstimate(MemoryRange),
    CelfMutate(CELFMutateResult),
    CelfWrite(WriteResult),
    ClosenessStream(Vec<CentralityScore>),
    ClosenessStats(ClosenessCentralityStats),
    ClosenessEstimate(MemoryRange),
    ClosenessMutate(ClosenessCentralityMutateResult),
    ClosenessWrite(WriteResult),
    ConductanceStream(Vec<ConductanceRow>),
    ConductanceStats(ConductanceStats),
    ConductanceEstimate(MemoryRange),
    ConductanceMutate(ConductanceMutateResult),
    ConductanceWrite(WriteResult),
    DegreeCentralityStream(Vec<CentralityScore>),
    DegreeCentralityStats(DegreeCentralityStats),
    DegreeCentralityEstimate(MemoryRange),
    DegreeCentralityMutate(DegreeCentralityMutateResult),
    DegreeCentralityWrite(WriteResult),
    DfsStream(Vec<PathResult>),
    DfsStats(DfsStats),
    DfsEstimate(MemoryRange),
    DfsMutate(DfsMutateResult),
    DfsWrite(DfsWriteSummary),
    DeltaSteppingStream(Vec<PathResult>),
    DeltaSteppingStats(DeltaSteppingStats),
    DeltaSteppingEstimate(MemoryRange),
    DeltaSteppingMutate(DeltaSteppingMutateResult),
    DeltaSteppingWrite(DeltaSteppingWriteSummary),
    DagLongestPathStream(Vec<DagLongestPathRow>),
    DagLongestPathStats(DagLongestPathStats),
    DagLongestPathEstimate(MemoryRange),
    DagLongestPathMutate(DagLongestPathMutateResult),
    DagLongestPathWrite(WriteResult),
    DijkstraStream(Vec<PathResult>),
    DijkstraStats(DijkstraStats),
    DijkstraEstimate(MemoryRange),
    DijkstraMutate(DijkstraMutateResult),
    DijkstraWrite(DijkstraWriteSummary),
    HarmonicStream(Vec<CentralityScore>),
    HarmonicStats(HarmonicCentralityStats),
    HarmonicEstimate(MemoryRange),
    HarmonicMutate(HarmonicCentralityMutateResult),
    HarmonicWrite(WriteResult),
    HitsStream(Vec<CentralityScore>),
    HitsStats(HitsCentralityStats),
    HitsEstimate(MemoryRange),
    HitsMutate(HitsCentralityMutateResult),
    HitsWrite(WriteResult),
    K1ColoringStream(Vec<K1ColoringRow>),
    K1ColoringStats(K1ColoringStats),
    K1ColoringEstimate(MemoryRange),
    K1ColoringMutate(K1ColoringMutateResult),
    K1ColoringWrite(WriteResult),
    KSpanningTreeStream(Vec<KSpanningTreeRow>),
    KSpanningTreeStats(KSpanningTreeStats),
    KSpanningTreeEstimate(MemoryRange),
    KSpanningTreeMutate(KSpanningTreeMutateResult),
    KSpanningTreeWrite(WriteResult),
    PageRankStream(Vec<CentralityScore>),
    PageRankStats(PageRankStats),
    PageRankEstimate(MemoryRange),
    PageRankMutate(PageRankMutateResult),
    PageRankWrite(WriteResult),
    RandomWalkStream(Vec<RandomWalkRow>),
    RandomWalkStats(RandomWalkStats),
    RandomWalkEstimate(MemoryRange),
    RandomWalkMutate(RandomWalkMutateResult),
    RandomWalkWrite(RandomWalkWriteSummary),
    SpanningTreeStream(Vec<SpanningTreeRow>),
    SpanningTreeStats(SpanningTreeStats),
    SpanningTreeEstimate(MemoryRange),
    SpanningTreeMutate(SpanningTreeMutateResult),
    SpanningTreeWrite(SpanningTreeWriteSummary),
    SteinerTreeStream(Vec<SteinerTreeRow>),
    SteinerTreeStats(SteinerTreeStats),
    SteinerTreeEstimate(MemoryRange),
    SteinerTreeMutate(SteinerTreeMutateResult),
    SteinerTreeWrite(WriteResult),
    TopologicalSortStream(Vec<TopologicalSortRow>),
    TopologicalSortStats(TopologicalSortStats),
    TopologicalSortEstimate(MemoryRange),
    TopologicalSortMutate(TopologicalSortMutateResult),
    TopologicalSortWrite(TopologicalSortWriteSummary),
    YensStream(Vec<PathResult>),
    YensStats(YensStats),
    YensEstimate(MemoryRange),
    YensMutate(YensMutateResult),
    YensWrite(YensWriteSummary),
    LinkPredictionPipeline(Vec<PipelineInfoResult>),
    NodePipeline(Vec<NodePipelineInfoResult>),
    PipelineCatalog(Vec<PipelineCatalogResult>),
    PipelineExists(Vec<PipelineExistsResult>),
}

pub struct ShellProcedureRuntime {
    graph: GraphFacade,
    pipelines: Arc<LocalPipelinesProcedureFacade>,
}

impl ShellProcedureRuntime {
    pub fn new(graph: GraphFacade, pipelines: LocalPipelinesProcedureFacade) -> Self {
        Self {
            graph,
            pipelines: Arc::new(pipelines),
        }
    }

    pub fn graph(&self) -> &GraphFacade {
        &self.graph
    }

    pub fn pipelines(&self) -> &LocalPipelinesProcedureFacade {
        self.pipelines.as_ref()
    }

    pub fn bind(
        &self,
        call: &ShellComponentCall,
    ) -> Result<ShellProcedureBinding, ShellProcedureError> {
        let component = builtin_component(call.component.as_str())
            .ok_or(ShellProcedureError::UnknownComponent(call.component))?
            .descriptor();

        if component.category == ShellComponentCategory::Pipeline {
            pipelines::bind_pipeline(Arc::clone(&self.pipelines), call)
        } else {
            self.graph.bind_shell_component(call)
        }
    }

    pub fn invoke(
        &self,
        call: &ShellComponentCall,
    ) -> Result<ShellProcedureResult, ShellProcedureError> {
        self.bind(call)?.invoke()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ShellProcedureError {
    #[error("unknown Shell Component `{0}`")]
    UnknownComponent(ShellComponentId),

    #[error("Shell Component `{0}` has no local Procedure binding")]
    UnboundComponent(ShellComponentId),

    #[error("Shell Component `{component}` does not support mode `{mode:?}`")]
    UnsupportedMode {
        component: ShellComponentId,
        mode: ShellComponentMode,
    },

    #[error("Shell Component input `{0}` is required")]
    MissingInput(&'static str),

    #[error("Shell Component input `{input}` must be {expected}")]
    InvalidInput {
        input: &'static str,
        expected: &'static str,
    },

    #[error(transparent)]
    Algorithm(#[from] AlgorithmError),
}

impl GraphFacade {
    pub fn bind_shell_component(
        &self,
        call: &ShellComponentCall,
    ) -> Result<ShellProcedureBinding, ShellProcedureError> {
        algorithms::bind_algorithm(self, call)
    }

    pub fn invoke_shell_component(
        &self,
        call: &ShellComponentCall,
    ) -> Result<ShellProcedureResult, ShellProcedureError> {
        self.bind_shell_component(call)?.invoke()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::procedures::pipelines::PipelineModelStore;
    use crate::procedures::pipelines::RequestScopedDependencies;
    use crate::projection::eval::pipeline::PipelineCatalog;
    use crate::shell::builtin_component;
    use crate::types::catalog::InMemoryGraphCatalog;
    use crate::types::prelude::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;
    use crate::types::random::RandomRelationshipConfig;
    use std::sync::Arc;

    fn graph() -> GraphFacade {
        let config = RandomGraphConfig {
            seed: Some(7),
            node_count: 8,
            relationships: vec![RandomRelationshipConfig::new("REL", 1.0)],
            ..RandomGraphConfig::default()
        };
        GraphFacade::new(Arc::new(DefaultGraphStore::random(&config).unwrap()))
    }

    fn runtime() -> ShellProcedureRuntime {
        let graph_catalog = Arc::new(InMemoryGraphCatalog::new());
        let dependencies = RequestScopedDependencies::with_runtime_dependencies(
            crate::types::user::User::from("shell-test"),
            graph_catalog,
            Arc::new(PipelineModelStore::new()),
        );
        let pipelines =
            LocalPipelinesProcedureFacade::new(dependencies, Arc::new(PipelineCatalog::new()));
        ShellProcedureRuntime::new(graph(), pipelines)
    }

    #[test]
    fn binds_and_invokes_bfs_estimate_as_a_typed_procedure() {
        let call = builtin_component("bfs")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("trackPaths", true);

        let binding = graph().bind_shell_component(&call).unwrap();
        assert_eq!(binding.component(), call.component);
        assert_eq!(binding.mode(), ShellComponentMode::Estimate);
        assert!(matches!(
            binding.invoke().unwrap(),
            ShellProcedureResult::BfsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_dijkstra_estimate_as_a_typed_procedure() {
        let call = builtin_component("gds.algorithms.pathfinding.dijkstra")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("sourceNode", 0_u64)
            .with_input("trackRelationships", true);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::DijkstraEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn rejects_modes_not_supported_by_the_component() {
        let call = builtin_component("bfs")
            .unwrap()
            .call(ShellComponentMode::Invoke)
            .with_input("source", 0_u64);

        assert!(matches!(
            graph().bind_shell_component(&call),
            Err(ShellProcedureError::UnsupportedMode { .. })
        ));
    }

    #[test]
    fn reports_algorithms_without_a_local_binding() {
        let call = builtin_component("kcore")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64);

        assert!(matches!(
            graph().bind_shell_component(&call),
            Err(ShellProcedureError::UnboundComponent(component))
                if component == call.component
        ));
    }

    #[test]
    fn binds_and_invokes_k1coloring_estimate_as_a_typed_procedure() {
        let call = builtin_component("k1coloring")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("maxIterations", 20_u64)
            .with_input("concurrency", 2_u64)
            .with_input("batchSize", 64_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::K1ColoringEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_conductance_estimate_as_a_typed_procedure() {
        let call = builtin_component("conductance")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("communityProperty", "community")
            .with_input("relationshipWeightProperty", true)
            .with_input("concurrency", 2_u64)
            .with_input("minBatchSize", 64_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::ConductanceEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_approx_max_kcut_estimate_as_a_typed_procedure() {
        let call = builtin_component("approx_max_kcut")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("k", 2_u64)
            .with_input("iterations", 5_u64)
            .with_input("randomSeed", 17_u64)
            .with_input("minCommunitySizes", vec![0_u64, 0_u64])
            .with_input("vnsMaxNeighborhoodOrder", 1_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::ApproxMaxKCutEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_hits_estimate_as_a_typed_procedure() {
        let call = builtin_component("hits")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("maxIterations", 30_u64)
            .with_input("tolerance", 0.00001_f64)
            .with_input("hubProperty", "hub_score")
            .with_input("authProperty", "authority_score");

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::HitsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_harmonic_estimate_as_a_typed_procedure() {
        let call = builtin_component("harmonic")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("direction", "both")
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::HarmonicEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_degree_centrality_estimate_as_a_typed_procedure() {
        let call = builtin_component("degree_centrality")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("orientation", "undirected")
            .with_input("normalize", true)
            .with_input("weightProperty", "cost");

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::DegreeCentralityEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_closeness_estimate_as_a_typed_procedure() {
        let call = builtin_component("closeness")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("wassermanFaust", true)
            .with_input("direction", "incoming")
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::ClosenessEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_celf_estimate_as_a_typed_procedure() {
        let call = builtin_component("celf")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("seedSetSize", 3_u64)
            .with_input("monteCarloSimulations", 100_u64)
            .with_input("propagationProbability", 0.25_f64)
            .with_input("batchSize", 8_u64)
            .with_input("randomSeed", 42_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::CelfEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_bridges_estimate_as_a_typed_procedure() {
        let call = builtin_component("bridges")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::BridgesEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_betweenness_estimate_as_a_typed_procedure() {
        let call = builtin_component("betweenness")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("direction", "both")
            .with_input("samplingStrategy", "random_degree")
            .with_input("samplingSize", 4_u64)
            .with_input("randomSeed", 42_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::BetweennessEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_articulation_points_estimate_as_a_typed_procedure() {
        let call = builtin_component("articulation_points")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::ArticulationPointsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_pagerank_estimate_as_a_typed_procedure() {
        let call = builtin_component("pagerank")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("variant", "article_rank")
            .with_input("sourceNodes", vec![0_u64, 1_u64])
            .with_input("maxIterations", 30_u64)
            .with_input("dampingFactor", 0.9_f64)
            .with_input("tolerance", 0.00001_f64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::PageRankEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_random_walk_estimate_as_a_typed_procedure() {
        let call = builtin_component("random_walk")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("sourceNodes", vec![0_u64, 1_u64])
            .with_input("walksPerNode", 4_u64)
            .with_input("walkLength", 8_u64)
            .with_input("returnFactor", 0.5_f64)
            .with_input("inOutFactor", 2.0_f64)
            .with_input("randomSeed", 42_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::RandomWalkEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_topological_sort_estimate_as_a_typed_procedure() {
        let call = builtin_component("topological_sort")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("computeMaxDistanceFromSource", true)
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::TopologicalSortEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_steiner_tree_estimate_as_a_typed_procedure() {
        let call = builtin_component("steiner_tree")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("sourceNode", 0_u64)
            .with_input("targetNodes", vec![1_u64, 2_u64])
            .with_input("delta", 2.0_f64)
            .with_input("applyRerouting", false);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::SteinerTreeEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_spanning_tree_estimate_as_a_typed_procedure() {
        let call = builtin_component("spanning_tree")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("startNode", 0_u64)
            .with_input("computeMinimum", false)
            .with_input("direction", "undirected");

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::SpanningTreeEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_all_shortest_paths_estimate_as_a_typed_procedure() {
        let call = builtin_component("all_shortest_paths")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("weighted", true)
            .with_input("weightProperty", "cost")
            .with_input("maxResults", 10_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::AllShortestPathsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_yens_estimate_as_a_typed_procedure() {
        let call = builtin_component("yens")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("sourceNode", 0_u64)
            .with_input("targetNode", 1_u64)
            .with_input("k", 2_u64)
            .with_input("trackRelationships", true);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::YensEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_kspanning_tree_estimate_as_a_typed_procedure() {
        let call = builtin_component("kspanningtree")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("sourceNode", 0_u64)
            .with_input("k", 2_u64)
            .with_input("objective", "max");

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::KSpanningTreeEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_dag_longest_path_estimate_as_a_typed_procedure() {
        let call = builtin_component("dag_longest_path")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("concurrency", 2_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::DagLongestPathEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_delta_stepping_estimate_as_a_typed_procedure() {
        let call = builtin_component("delta_stepping")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("delta", 2.5_f64)
            .with_input("storePredecessors", true);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::DeltaSteppingEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_bellman_ford_estimate_as_a_typed_procedure() {
        let call = builtin_component("bellman_ford")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("trackNegativeCycles", true)
            .with_input("trackPaths", true);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::BellmanFordEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn binds_and_invokes_astar_estimate_as_a_typed_procedure() {
        let call = builtin_component("astar")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("target", 4_u64)
            .with_input("heuristic", "euclidean");

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::AStarEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn rejects_unknown_astar_heuristics() {
        let call = builtin_component("astar")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("target", 4_u64)
            .with_input("heuristic", "guess");

        assert!(matches!(
            graph().bind_shell_component(&call),
            Err(ShellProcedureError::InvalidInput {
                input: "heuristic",
                ..
            })
        ));
    }

    #[test]
    fn binds_and_invokes_dfs_estimate_as_a_typed_procedure() {
        let call = builtin_component("dfs")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64)
            .with_input("maxDepth", 4_u64);

        assert!(matches!(
            graph().invoke_shell_component(&call).unwrap(),
            ShellProcedureResult::DfsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn runtime_invokes_algorithm_procedures() {
        let call = builtin_component("bfs")
            .unwrap()
            .call(ShellComponentMode::Estimate)
            .with_input("source", 0_u64);

        assert!(matches!(
            runtime().invoke(&call).unwrap(),
            ShellProcedureResult::BfsEstimate(memory) if !memory.is_empty()
        ));
    }

    #[test]
    fn runtime_invokes_pipeline_create_and_exists_procedures() {
        let runtime = runtime();
        let create = builtin_component("create_node_classification_pipeline")
            .unwrap()
            .call(ShellComponentMode::Invoke)
            .with_input("pipelineName", "shell-pipeline");

        assert!(matches!(
            runtime.invoke(&create).unwrap(),
            ShellProcedureResult::NodePipeline(results) if results.len() == 1
        ));

        let exists = builtin_component("pipeline_exists")
            .unwrap()
            .call(ShellComponentMode::Invoke)
            .with_input("pipelineName", "shell-pipeline");
        assert!(matches!(
            runtime.invoke(&exists).unwrap(),
            ShellProcedureResult::PipelineExists(results) if results.len() == 1
        ));
    }
}
