use crate::algo::similarity::knn::KnnSamplerType;
use crate::algo::similarity::knn::SimilarityMetric;
use crate::algo::similarity::node_similarity::NodeSimilarityMetric;
use crate::procedures::centrality::ArticulationPointsFacade;
use crate::procedures::centrality::BetweennessCentralityFacade;
use crate::procedures::centrality::BridgesFacade;
use crate::procedures::centrality::CELFFacade;
use crate::procedures::centrality::ClosenessCentralityFacade;
use crate::procedures::centrality::DegreeCentralityFacade;
use crate::procedures::centrality::HarmonicCentralityFacade;
use crate::procedures::centrality::HitsCentralityFacade;
use crate::procedures::centrality::Orientation as DegreeOrientation;
use crate::procedures::centrality::PageRankFacade;
use crate::procedures::community::ApproxMaxKCutFacade;
use crate::procedures::community::ConductanceFacade;
use crate::procedures::community::K1ColoringFacade;
use crate::procedures::community::KCoreFacade;
use crate::procedures::community::KMeansFacade;
use crate::procedures::community::KMeansSamplerType;
use crate::procedures::community::LabelPropagationFacade;
use crate::procedures::community::LeidenFacade;
use crate::procedures::community::LouvainFacade;
use crate::procedures::community::ModularityFacade;
use crate::procedures::community::SccFacade;
use crate::procedures::community::TriangleFacade;
use crate::procedures::community::WccFacade;
use crate::procedures::pathfinding::AStarBuilder;
use crate::procedures::pathfinding::AllShortestPathsBuilder;
use crate::procedures::pathfinding::BellmanFordBuilder;
use crate::procedures::pathfinding::BfsBuilder;
use crate::procedures::pathfinding::DagLongestPathBuilder;
use crate::procedures::pathfinding::DeltaSteppingBuilder;
use crate::procedures::pathfinding::DfsBuilder;
use crate::procedures::pathfinding::DijkstraBuilder;
use crate::procedures::pathfinding::Heuristic;
use crate::procedures::pathfinding::KSpanningTreeBuilder;
use crate::procedures::pathfinding::RandomWalkBuilder;
use crate::procedures::pathfinding::SpanningTreeBuilder;
use crate::procedures::pathfinding::SteinerTreeBuilder;
use crate::procedures::pathfinding::TopologicalSortBuilder;
use crate::procedures::pathfinding::YensBuilder;
use crate::procedures::similarity::FilteredKnnFacade;
use crate::procedures::similarity::FilteredNodeSimilarityFacade;
use crate::procedures::similarity::KnnFacade;
use crate::procedures::similarity::NodeSimilarityFacade;
use crate::projection::NodeLabel;
use crate::shell::builtin_component;
use crate::shell::ShellComponentCall;
use crate::shell::ShellComponentMode;

use super::inputs::optional_bool;
use super::inputs::optional_f64;
use super::inputs::optional_f64_matrix;
use super::inputs::optional_str;
use super::inputs::optional_string_array;
use super::inputs::optional_string_or_array;
use super::inputs::optional_u64;
use super::inputs::optional_u64_array;
use super::inputs::optional_usize;
use super::inputs::output_property;
use super::inputs::required_output_property;
use super::inputs::required_property_metrics;
use super::inputs::required_string;
use super::inputs::required_u64;
use super::GraphFacade;
use super::ShellProcedureBinding;
use super::ShellProcedureError;
use super::ShellProcedureResult;

pub(super) fn bind_algorithm(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ShellProcedureBinding, ShellProcedureError> {
    let component = builtin_component(call.component.as_str())
        .ok_or(ShellProcedureError::UnknownComponent(call.component))?
        .descriptor();

    if !component.supports(call.mode) {
        return Err(ShellProcedureError::UnsupportedMode {
            component: component.id,
            mode: call.mode,
        });
    }

    match component.alias {
        "approx_max_kcut" => Ok(ShellProcedureBinding::ApproxMaxKCut {
            component: component.id,
            mode: call.mode,
            procedure: bind_approx_max_kcut(graph, call)?,
            output_property: output_property(call)?,
        }),
        "articulation_points" => Ok(ShellProcedureBinding::ArticulationPoints {
            component: component.id,
            mode: call.mode,
            procedure: bind_articulation_points(graph, call)?,
            output_property: output_property(call)?,
        }),
        "all_shortest_paths" => Ok(ShellProcedureBinding::AllShortestPaths {
            component: component.id,
            mode: call.mode,
            procedure: bind_all_shortest_paths(graph, call)?,
            output_property: output_property(call)?,
        }),
        "astar" => Ok(ShellProcedureBinding::AStar {
            component: component.id,
            mode: call.mode,
            procedure: bind_astar(graph, call)?,
            output_property: output_property(call)?,
        }),
        "bellman_ford" => Ok(ShellProcedureBinding::BellmanFord {
            component: component.id,
            mode: call.mode,
            procedure: bind_bellman_ford(graph, call)?,
            output_property: output_property(call)?,
        }),
        "betweenness" => Ok(ShellProcedureBinding::Betweenness {
            component: component.id,
            mode: call.mode,
            procedure: bind_betweenness(graph, call)?,
            output_property: output_property(call)?,
        }),
        "bfs" => Ok(ShellProcedureBinding::Bfs {
            component: component.id,
            mode: call.mode,
            procedure: bind_bfs(graph, call)?,
            output_property: output_property(call)?,
        }),
        "bridges" => Ok(ShellProcedureBinding::Bridges {
            component: component.id,
            mode: call.mode,
            procedure: bind_bridges(graph, call)?,
            output_property: output_property(call)?,
        }),
        "celf" => Ok(ShellProcedureBinding::Celf {
            component: component.id,
            mode: call.mode,
            procedure: bind_celf(graph, call)?,
            output_property: output_property(call)?,
        }),
        "closeness" => Ok(ShellProcedureBinding::Closeness {
            component: component.id,
            mode: call.mode,
            procedure: bind_closeness(graph, call)?,
            output_property: output_property(call)?,
        }),
        "conductance" => Ok(ShellProcedureBinding::Conductance {
            component: component.id,
            mode: call.mode,
            procedure: bind_conductance(graph, call)?,
            output_property: output_property(call)?,
        }),
        "degree_centrality" => Ok(ShellProcedureBinding::DegreeCentrality {
            component: component.id,
            mode: call.mode,
            procedure: bind_degree_centrality(graph, call)?,
            output_property: output_property(call)?,
        }),
        "dfs" => Ok(ShellProcedureBinding::Dfs {
            component: component.id,
            mode: call.mode,
            procedure: bind_dfs(graph, call)?,
            output_property: output_property(call)?,
        }),
        "delta_stepping" => Ok(ShellProcedureBinding::DeltaStepping {
            component: component.id,
            mode: call.mode,
            procedure: bind_delta_stepping(graph, call)?,
            output_property: output_property(call)?,
        }),
        "dag_longest_path" => Ok(ShellProcedureBinding::DagLongestPath {
            component: component.id,
            mode: call.mode,
            procedure: bind_dag_longest_path(graph, call)?,
            output_property: output_property(call)?,
        }),
        "dijkstra" => Ok(ShellProcedureBinding::Dijkstra {
            component: component.id,
            mode: call.mode,
            procedure: bind_dijkstra(graph, call)?,
            output_property: output_property(call)?,
        }),
        "harmonic" => Ok(ShellProcedureBinding::Harmonic {
            component: component.id,
            mode: call.mode,
            procedure: bind_harmonic(graph, call)?,
            output_property: output_property(call)?,
        }),
        "hits" => Ok(ShellProcedureBinding::Hits {
            component: component.id,
            mode: call.mode,
            procedure: bind_hits(graph, call)?,
            output_property: output_property(call)?,
        }),
        "k1coloring" => Ok(ShellProcedureBinding::K1Coloring {
            component: component.id,
            mode: call.mode,
            procedure: bind_k1coloring(graph, call)?,
            output_property: output_property(call)?,
        }),
        "kcore" => Ok(ShellProcedureBinding::KCore {
            component: component.id,
            mode: call.mode,
            procedure: bind_kcore(graph, call)?,
            output_property: output_property(call)?,
        }),
        "kmeans" => Ok(ShellProcedureBinding::KMeans {
            component: component.id,
            mode: call.mode,
            procedure: bind_kmeans(graph, call)?,
            output_property: output_property(call)?,
        }),
        "label_propagation" => Ok(ShellProcedureBinding::LabelPropagation {
            component: component.id,
            mode: call.mode,
            procedure: bind_label_propagation(graph, call)?,
            output_property: output_property(call)?,
        }),
        "leiden" => Ok(ShellProcedureBinding::Leiden {
            component: component.id,
            mode: call.mode,
            procedure: bind_leiden(graph, call)?,
            output_property: output_property(call)?,
        }),
        "louvain" => Ok(ShellProcedureBinding::Louvain {
            component: component.id,
            mode: call.mode,
            procedure: bind_louvain(graph, call)?,
            output_property: output_property(call)?,
        }),
        "modularity" => Ok(ShellProcedureBinding::Modularity {
            component: component.id,
            mode: call.mode,
            procedure: bind_modularity(graph, call)?,
            output_property: output_property(call)?,
        }),
        "scc" => Ok(ShellProcedureBinding::Scc {
            component: component.id,
            mode: call.mode,
            procedure: bind_scc(graph, call)?,
            output_property: output_property(call)?,
        }),
        "triangle" => Ok(ShellProcedureBinding::Triangle {
            component: component.id,
            mode: call.mode,
            procedure: bind_triangle(graph, call)?,
            output_property: output_property(call)?,
        }),
        "wcc" => Ok(ShellProcedureBinding::Wcc {
            component: component.id,
            mode: call.mode,
            procedure: bind_wcc(graph, call)?,
            output_property: output_property(call)?,
        }),
        "knn" => Ok(ShellProcedureBinding::Knn {
            component: component.id,
            mode: call.mode,
            procedure: bind_knn(graph, call)?,
            output_property: output_property(call)?,
        }),
        "filtered_knn" => Ok(ShellProcedureBinding::FilteredKnn {
            component: component.id,
            mode: call.mode,
            procedure: bind_filtered_knn(graph, call)?,
            output_property: output_property(call)?,
        }),
        "node_similarity" => Ok(ShellProcedureBinding::NodeSimilarity {
            component: component.id,
            mode: call.mode,
            procedure: bind_node_similarity(graph, call)?,
            output_property: output_property(call)?,
        }),
        "filtered_node_similarity" => Ok(ShellProcedureBinding::FilteredNodeSimilarity {
            component: component.id,
            mode: call.mode,
            procedure: bind_filtered_node_similarity(graph, call)?,
            output_property: output_property(call)?,
        }),
        "kspanningtree" => Ok(ShellProcedureBinding::KSpanningTree {
            component: component.id,
            mode: call.mode,
            procedure: bind_kspanning_tree(graph, call)?,
            output_property: output_property(call)?,
        }),
        "pagerank" => Ok(ShellProcedureBinding::PageRank {
            component: component.id,
            mode: call.mode,
            procedure: bind_pagerank(graph, call)?,
            output_property: output_property(call)?,
        }),
        "random_walk" => Ok(ShellProcedureBinding::RandomWalk {
            component: component.id,
            mode: call.mode,
            procedure: bind_random_walk(graph, call)?,
            output_property: output_property(call)?,
        }),
        "spanning_tree" => Ok(ShellProcedureBinding::SpanningTree {
            component: component.id,
            mode: call.mode,
            procedure: bind_spanning_tree(graph, call)?,
            output_property: output_property(call)?,
        }),
        "steiner_tree" => Ok(ShellProcedureBinding::SteinerTree {
            component: component.id,
            mode: call.mode,
            procedure: bind_steiner_tree(graph, call)?,
            output_property: output_property(call)?,
        }),
        "topological_sort" => Ok(ShellProcedureBinding::TopologicalSort {
            component: component.id,
            mode: call.mode,
            procedure: bind_topological_sort(graph, call)?,
            output_property: output_property(call)?,
        }),
        "yens" => Ok(ShellProcedureBinding::Yens {
            component: component.id,
            mode: call.mode,
            procedure: bind_yens(graph, call)?,
            output_property: output_property(call)?,
        }),
        _ => Err(ShellProcedureError::UnboundComponent(component.id)),
    }
}

pub(super) fn invoke_approx_max_kcut(
    mode: ShellComponentMode,
    procedure: ApproxMaxKCutFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::ApproxMaxKCutStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::ApproxMaxKCutStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::ApproxMaxKCutEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::ApproxMaxKCutMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::ApproxMaxKCutWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("ApproxMaxKCut mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_conductance(
    mode: ShellComponentMode,
    procedure: ConductanceFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::ConductanceStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::ConductanceStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::ConductanceEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::ConductanceMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::ConductanceWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Conductance mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_astar(
    mode: ShellComponentMode,
    procedure: AStarBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::AStarStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::AStarStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::AStarEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::AStarMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::AStarWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => unreachable!("A* mode is validated before invocation"),
    })
}

pub(super) fn invoke_articulation_points(
    mode: ShellComponentMode,
    procedure: ArticulationPointsFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::ArticulationPointsStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => {
            ShellProcedureResult::ArticulationPointsStats(procedure.stats()?)
        }
        ShellComponentMode::Estimate => {
            ShellProcedureResult::ArticulationPointsEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::ArticulationPointsMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::ArticulationPointsWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Articulation Points mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_all_shortest_paths(
    mode: ShellComponentMode,
    procedure: AllShortestPathsBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::AllShortestPathsStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => {
            ShellProcedureResult::AllShortestPathsStats(procedure.stats()?)
        }
        ShellComponentMode::Estimate => {
            ShellProcedureResult::AllShortestPathsEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::AllShortestPathsMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::AllShortestPathsWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("All Shortest Paths mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_bfs(
    mode: ShellComponentMode,
    procedure: BfsBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::BfsStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::BfsStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::BfsEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::BfsMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::BfsWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => unreachable!("BFS mode is validated before invocation"),
    })
}

pub(super) fn invoke_bridges(
    mode: ShellComponentMode,
    procedure: BridgesFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::BridgesStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::BridgesStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::BridgesEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::BridgesMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::BridgesWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Bridges mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_celf(
    mode: ShellComponentMode,
    procedure: CELFFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::CelfStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::CelfStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::CelfEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::CelfMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::CelfWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("CELF mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_closeness(
    mode: ShellComponentMode,
    procedure: ClosenessCentralityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::ClosenessStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::ClosenessStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::ClosenessEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::ClosenessMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::ClosenessWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Closeness mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_degree_centrality(
    mode: ShellComponentMode,
    procedure: DegreeCentralityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::DegreeCentralityStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => {
            ShellProcedureResult::DegreeCentralityStats(procedure.stats()?)
        }
        ShellComponentMode::Estimate => {
            ShellProcedureResult::DegreeCentralityEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::DegreeCentralityMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::DegreeCentralityWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Degree Centrality mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_bellman_ford(
    mode: ShellComponentMode,
    procedure: BellmanFordBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::BellmanFordStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::BellmanFordStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::BellmanFordEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::BellmanFordMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::BellmanFordWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Bellman-Ford mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_betweenness(
    mode: ShellComponentMode,
    procedure: BetweennessCentralityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::BetweennessStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::BetweennessStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::BetweennessEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::BetweennessMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::BetweennessWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Betweenness mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_dfs(
    mode: ShellComponentMode,
    procedure: DfsBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::DfsStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::DfsStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::DfsEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::DfsMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::DfsWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => unreachable!("DFS mode is validated before invocation"),
    })
}

pub(super) fn invoke_dijkstra(
    mode: ShellComponentMode,
    procedure: DijkstraBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::DijkstraStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::DijkstraStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::DijkstraEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::DijkstraMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::DijkstraWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Dijkstra mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_harmonic(
    mode: ShellComponentMode,
    procedure: HarmonicCentralityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::HarmonicStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::HarmonicStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::HarmonicEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::HarmonicMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::HarmonicWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Harmonic mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_hits(
    mode: ShellComponentMode,
    procedure: HitsCentralityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::HitsStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::HitsStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::HitsEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::HitsMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::HitsWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("HITS mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_k1coloring(
    mode: ShellComponentMode,
    procedure: K1ColoringFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::K1ColoringStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::K1ColoringStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::K1ColoringEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::K1ColoringMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::K1ColoringWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("K1Coloring mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_kcore(
    mode: ShellComponentMode,
    procedure: KCoreFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::KCoreStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::KCoreStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::KCoreEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::KCoreMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::KCoreWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("K-Core mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_kmeans(
    mode: ShellComponentMode,
    procedure: KMeansFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::KMeansStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::KMeansStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::KMeansEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::KMeansMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::KMeansWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("K-Means mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_label_propagation(
    mode: ShellComponentMode,
    procedure: LabelPropagationFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::LabelPropagationStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => {
            ShellProcedureResult::LabelPropagationStats(procedure.stats()?)
        }
        ShellComponentMode::Estimate => {
            ShellProcedureResult::LabelPropagationEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::LabelPropagationMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::LabelPropagationWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Label Propagation mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_leiden(
    mode: ShellComponentMode,
    procedure: LeidenFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::LeidenStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::LeidenStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::LeidenEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::LeidenMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::LeidenWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Leiden mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_louvain(
    mode: ShellComponentMode,
    procedure: LouvainFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::LouvainStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::LouvainStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::LouvainEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::LouvainMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::LouvainWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Louvain mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_modularity(
    mode: ShellComponentMode,
    procedure: ModularityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::ModularityStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::ModularityStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::ModularityEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::ModularityMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::ModularityWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Modularity mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_scc(
    mode: ShellComponentMode,
    procedure: SccFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::SccStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::SccStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::SccEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::SccMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::SccWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("SCC mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_triangle(
    mode: ShellComponentMode,
    procedure: TriangleFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::TriangleStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::TriangleStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::TriangleEstimate(procedure.estimate_memory()?)
        }
        ShellComponentMode::Mutate => ShellProcedureResult::TriangleMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::TriangleWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Triangle mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_wcc(
    mode: ShellComponentMode,
    procedure: WccFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::WccStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::WccStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::WccEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::WccMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::WccWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("WCC mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_knn(
    mode: ShellComponentMode,
    procedure: KnnFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::KnnStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::KnnStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::KnnEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::KnnMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::KnnWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("KNN mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_filtered_knn(
    mode: ShellComponentMode,
    procedure: FilteredKnnFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::FilteredKnnStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::FilteredKnnStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::FilteredKnnEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::FilteredKnnMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::FilteredKnnWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Filtered KNN mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_node_similarity(
    mode: ShellComponentMode,
    procedure: NodeSimilarityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::NodeSimilarityStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::NodeSimilarityStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::NodeSimilarityEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::NodeSimilarityMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::NodeSimilarityWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Node Similarity mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_filtered_node_similarity(
    mode: ShellComponentMode,
    procedure: FilteredNodeSimilarityFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::FilteredNodeSimilarityStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => {
            ShellProcedureResult::FilteredNodeSimilarityStats(procedure.stats()?)
        }
        ShellComponentMode::Estimate => {
            ShellProcedureResult::FilteredNodeSimilarityEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::FilteredNodeSimilarityMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::FilteredNodeSimilarityWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Filtered Node Similarity mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_delta_stepping(
    mode: ShellComponentMode,
    procedure: DeltaSteppingBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::DeltaSteppingStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::DeltaSteppingStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::DeltaSteppingEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::DeltaSteppingMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::DeltaSteppingWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Delta-Stepping mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_kspanning_tree(
    mode: ShellComponentMode,
    procedure: KSpanningTreeBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::KSpanningTreeStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::KSpanningTreeStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::KSpanningTreeEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::KSpanningTreeMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::KSpanningTreeWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("K-Spanning Tree mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_pagerank(
    mode: ShellComponentMode,
    procedure: PageRankFacade,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::PageRankStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::PageRankStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::PageRankEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::PageRankMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::PageRankWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("PageRank mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_random_walk(
    mode: ShellComponentMode,
    procedure: RandomWalkBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::RandomWalkStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::RandomWalkStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::RandomWalkEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::RandomWalkMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::RandomWalkWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Random Walk mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_dag_longest_path(
    mode: ShellComponentMode,
    procedure: DagLongestPathBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::DagLongestPathStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::DagLongestPathStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::DagLongestPathEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::DagLongestPathMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::DagLongestPathWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("DAG Longest Path mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_spanning_tree(
    mode: ShellComponentMode,
    procedure: SpanningTreeBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::SpanningTreeStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::SpanningTreeStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::SpanningTreeEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::SpanningTreeMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::SpanningTreeWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Spanning Tree mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_steiner_tree(
    mode: ShellComponentMode,
    procedure: SteinerTreeBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::SteinerTreeStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::SteinerTreeStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::SteinerTreeEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::SteinerTreeMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::SteinerTreeWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Steiner Tree mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_topological_sort(
    mode: ShellComponentMode,
    procedure: TopologicalSortBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::TopologicalSortStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::TopologicalSortStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::TopologicalSortEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::TopologicalSortMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::TopologicalSortWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Topological Sort mode is validated before invocation")
        }
    })
}

pub(super) fn invoke_yens(
    mode: ShellComponentMode,
    procedure: YensBuilder,
    output_property: Option<String>,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match mode {
        ShellComponentMode::Stream => {
            ShellProcedureResult::YensStream(procedure.stream()?.collect())
        }
        ShellComponentMode::Stats => ShellProcedureResult::YensStats(procedure.stats()?),
        ShellComponentMode::Estimate => {
            ShellProcedureResult::YensEstimate(procedure.estimate_memory())
        }
        ShellComponentMode::Mutate => ShellProcedureResult::YensMutate(
            procedure.mutate(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Write => ShellProcedureResult::YensWrite(
            procedure.write(required_output_property(output_property.as_deref())?)?,
        ),
        ShellComponentMode::Invoke => {
            unreachable!("Yen's mode is validated before invocation")
        }
    })
}

fn bind_astar(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<AStarBuilder, ShellProcedureError> {
    let mut procedure = graph
        .astar()
        .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(target) = optional_u64(call, "target", &["targetNode"])? {
        procedure = procedure.target(target);
    } else if let Some(targets) = optional_u64_array(call, "targets", &[])? {
        if targets.is_empty() {
            return Err(ShellProcedureError::MissingInput("target"));
        }
        procedure = procedure.targets(targets);
    } else {
        return Err(ShellProcedureError::MissingInput("target"));
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(latitude_property) = optional_str(call, "latitudeProperty", &["latitude_property"])?
    {
        procedure = procedure.latitude_property(latitude_property);
    }
    if let Some(longitude_property) =
        optional_str(call, "longitudeProperty", &["longitude_property"])?
    {
        procedure = procedure.longitude_property(longitude_property);
    }
    if let Some(heuristic) = optional_str(call, "heuristic", &[])? {
        let heuristic = match heuristic {
            "manhattan" => Heuristic::Manhattan,
            "euclidean" => Heuristic::Euclidean,
            "haversine" => Heuristic::Haversine,
            _ => {
                return Err(ShellProcedureError::InvalidInput {
                    input: "heuristic",
                    expected: "`manhattan`, `euclidean`, or `haversine`",
                })
            }
        };
        procedure = procedure.heuristic(heuristic);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_articulation_points(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ArticulationPointsFacade, ShellProcedureError> {
    let mut procedure = graph.articulation_points();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_all_shortest_paths(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<AllShortestPathsBuilder, ShellProcedureError> {
    let mut procedure = graph.all_shortest_paths();

    if let Some(weighted) = optional_bool(call, "weighted", &[])? {
        procedure = procedure.weighted(weighted);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(max_results) = optional_usize(call, "maxResults", &["max_results"])? {
        procedure = procedure.max_results(max_results);
    }

    Ok(procedure)
}

fn bind_bfs(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<BfsBuilder, ShellProcedureError> {
    let mut procedure = graph
        .bfs()
        .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(target) = optional_u64(call, "target", &["targetNode"])? {
        procedure = procedure.target(target);
    }
    if let Some(targets) = optional_u64_array(call, "targets", &[])? {
        procedure = procedure.targets(targets);
    }
    if let Some(max_depth) = optional_u64(call, "maxDepth", &["max_depth"])? {
        let max_depth =
            u32::try_from(max_depth).map_err(|_| ShellProcedureError::InvalidInput {
                input: "maxDepth",
                expected: "a 32-bit unsigned integer",
            })?;
        procedure = procedure.max_depth(max_depth);
    }
    if let Some(track_paths) = optional_bool(call, "trackPaths", &["track_paths"])? {
        procedure = procedure.track_paths(track_paths);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(delta) = optional_usize(call, "delta", &[])? {
        procedure = procedure.delta(delta);
    }

    Ok(procedure)
}

fn bind_bridges(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<BridgesFacade, ShellProcedureError> {
    let mut procedure = graph.bridges();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_celf(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<CELFFacade, ShellProcedureError> {
    let mut procedure = graph.celf();

    if let Some(seed_set_size) = optional_usize(call, "seedSetSize", &["seed_set_size"])? {
        procedure = procedure.seed_set_size(seed_set_size);
    }
    if let Some(simulations) =
        optional_usize(call, "monteCarloSimulations", &["monte_carlo_simulations"])?
    {
        procedure = procedure.monte_carlo_simulations(simulations);
    }
    if let Some(probability) =
        optional_f64(call, "propagationProbability", &["propagation_probability"])?
    {
        procedure = procedure.propagation_probability(probability);
    }
    if let Some(batch_size) = optional_usize(call, "batchSize", &["batch_size"])? {
        procedure = procedure.batch_size(batch_size);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed", "seed"])? {
        procedure = procedure.random_seed(random_seed);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_closeness(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ClosenessCentralityFacade, ShellProcedureError> {
    let mut procedure = graph.closeness();

    if let Some(wasserman_faust) = optional_bool(call, "wassermanFaust", &["wasserman_faust"])? {
        procedure = procedure.wasserman_faust(wasserman_faust);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_degree_centrality(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<DegreeCentralityFacade, ShellProcedureError> {
    let mut procedure = graph.degree_centrality();

    if let Some(orientation) = optional_str(call, "orientation", &["direction"])? {
        let orientation = match orientation {
            "natural" | "outgoing" => DegreeOrientation::Natural,
            "reverse" | "incoming" => DegreeOrientation::Reverse,
            "undirected" | "both" => DegreeOrientation::Undirected,
            _ => {
                return Err(ShellProcedureError::InvalidInput {
                    input: "orientation",
                    expected: "`natural`, `reverse`, or `undirected`",
                })
            }
        };
        procedure = procedure.orientation(orientation);
    }
    if let Some(normalize) = optional_bool(call, "normalize", &[])? {
        procedure = procedure.normalize(normalize);
    }
    if let Some(weighted) = optional_bool(call, "weighted", &[])? {
        procedure = procedure.weighted(weighted);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.relationship_weight_property(weight_property);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_bellman_ford(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<BellmanFordBuilder, ShellProcedureError> {
    let mut procedure = graph
        .bellman_ford()
        .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(track_negative_cycles) =
        optional_bool(call, "trackNegativeCycles", &["track_negative_cycles"])?
    {
        procedure = procedure.track_negative_cycles(track_negative_cycles);
    }
    if let Some(track_paths) = optional_bool(call, "trackPaths", &["track_paths"])? {
        procedure = procedure.track_paths(track_paths);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_betweenness(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<BetweennessCentralityFacade, ShellProcedureError> {
    let mut procedure = graph.betweenness();

    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.relationship_weight_property(Some(weight_property.to_string()));
    }
    if let Some(sampling_strategy) = optional_str(call, "samplingStrategy", &["sampling_strategy"])?
    {
        procedure = procedure.sampling_strategy(sampling_strategy);
    }
    if let Some(sampling_size) = optional_usize(call, "samplingSize", &["sampling_size"])? {
        procedure = procedure.sampling_size(Some(sampling_size));
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed", "seed"])? {
        procedure = procedure.random_seed(random_seed);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_dfs(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<DfsBuilder, ShellProcedureError> {
    let mut procedure = graph
        .dfs()
        .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(target) = optional_u64(call, "target", &["targetNode"])? {
        procedure = procedure.target(target);
    }
    if let Some(targets) = optional_u64_array(call, "targets", &[])? {
        procedure = procedure.targets(targets);
    }
    if let Some(max_depth) = optional_u64(call, "maxDepth", &["max_depth"])? {
        let max_depth =
            u32::try_from(max_depth).map_err(|_| ShellProcedureError::InvalidInput {
                input: "maxDepth",
                expected: "a 32-bit unsigned integer",
            })?;
        procedure = procedure.max_depth(max_depth);
    }
    if let Some(track_paths) = optional_bool(call, "trackPaths", &["track_paths"])? {
        procedure = procedure.track_paths(track_paths);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_dijkstra(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<DijkstraBuilder, ShellProcedureError> {
    let mut procedure = graph
        .dijkstra()
        .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(target) = optional_u64(call, "target", &["targetNode"])? {
        procedure = procedure.target(target);
    }
    if let Some(targets) = optional_u64_array(call, "targets", &[])? {
        procedure = procedure.targets(targets);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(track_relationships) = optional_bool(
        call,
        "trackRelationships",
        &["track_relationships", "trackPaths"],
    )? {
        procedure = procedure.track_relationships(track_relationships);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_harmonic(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<HarmonicCentralityFacade, ShellProcedureError> {
    let mut procedure = graph.harmonic();

    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_approx_max_kcut(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ApproxMaxKCutFacade, ShellProcedureError> {
    let mut procedure = graph.approx_max_kcut();

    if let Some(k) = optional_u64(call, "k", &[])? {
        procedure =
            procedure.k(
                u8::try_from(k).map_err(|_| ShellProcedureError::InvalidInput {
                    input: "k",
                    expected: "an unsigned 8-bit integer",
                })?,
            );
    }
    if let Some(iterations) = optional_usize(call, "iterations", &[])? {
        procedure = procedure.iterations(iterations);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed"])? {
        procedure = procedure.random_seed(random_seed);
    }
    if let Some(minimize) = optional_bool(call, "minimize", &[])? {
        procedure = procedure.minimize(minimize);
    }
    if let Some(use_weights) = optional_bool(
        call,
        "relationshipWeightProperty",
        &["relationship_weight_property", "useWeights"],
    )? {
        procedure = procedure.relationship_weight_property(use_weights);
    }
    if let Some(sizes) = optional_u64_array(call, "minCommunitySizes", &["min_community_sizes"])? {
        let sizes = sizes
            .into_iter()
            .map(|size| {
                usize::try_from(size).map_err(|_| ShellProcedureError::InvalidInput {
                    input: "minCommunitySizes",
                    expected: "an array of platform-sized unsigned integers",
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        procedure = procedure.min_community_sizes(sizes);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(min_batch_size) = optional_usize(call, "minBatchSize", &["min_batch_size"])? {
        procedure = procedure.min_batch_size(min_batch_size);
    }
    if let Some(order) = optional_usize(
        call,
        "vnsMaxNeighborhoodOrder",
        &["vns_max_neighborhood_order"],
    )? {
        procedure = procedure.vns_max_neighborhood_order(order);
    }

    Ok(procedure)
}

fn bind_conductance(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ConductanceFacade, ShellProcedureError> {
    let community_property = required_string(call, "communityProperty", &["community_property"])?;
    let mut procedure = graph.conductance(community_property);

    if let Some(use_weights) = optional_bool(
        call,
        "relationshipWeightProperty",
        &["relationship_weight_property", "useWeights"],
    )? {
        procedure = procedure.relationship_weight_property(use_weights);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(min_batch_size) = optional_usize(call, "minBatchSize", &["min_batch_size"])? {
        procedure = procedure.min_batch_size(min_batch_size);
    }

    Ok(procedure)
}

fn bind_k1coloring(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<K1ColoringFacade, ShellProcedureError> {
    let mut procedure = graph.k1coloring();

    if let Some(max_iterations) = optional_u64(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(batch_size) = optional_usize(
        call,
        "batchSize",
        &["batch_size", "minBatchSize", "min_batch_size"],
    )? {
        procedure = procedure.batch_size(batch_size);
    }

    Ok(procedure)
}

fn bind_kcore(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<KCoreFacade, ShellProcedureError> {
    let mut procedure = graph.kcore();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_kmeans(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<KMeansFacade, ShellProcedureError> {
    let mut procedure = graph.kmeans();

    if let Some(k) = optional_usize(call, "k", &[])? {
        procedure = procedure.k(k);
    }
    if let Some(max_iterations) = optional_u64(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(u32::try_from(max_iterations).map_err(|_| {
            ShellProcedureError::InvalidInput {
                input: "maxIterations",
                expected: "an unsigned 32-bit integer",
            }
        })?);
    }
    if let Some(delta_threshold) = optional_f64(call, "deltaThreshold", &["delta_threshold"])? {
        procedure = procedure.delta_threshold(delta_threshold);
    }
    if let Some(restarts) = optional_u64(call, "numberOfRestarts", &["number_of_restarts"])? {
        procedure = procedure.number_of_restarts(u32::try_from(restarts).map_err(|_| {
            ShellProcedureError::InvalidInput {
                input: "numberOfRestarts",
                expected: "an unsigned 32-bit integer",
            }
        })?);
    }
    if let Some(compute_silhouette) =
        optional_bool(call, "computeSilhouette", &["compute_silhouette"])?
    {
        procedure = procedure.compute_silhouette(compute_silhouette);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    procedure =
        procedure.node_property(&required_string(call, "nodeProperty", &["node_property"])?);
    if let Some(sampler_type) = optional_str(call, "samplerType", &["sampler_type"])? {
        let sampler_type = match sampler_type.to_ascii_uppercase().as_str() {
            "UNIFORM" => KMeansSamplerType::Uniform,
            "KMEANSPP" => KMeansSamplerType::KmeansPlusPlus,
            _ => {
                return Err(ShellProcedureError::InvalidInput {
                    input: "samplerType",
                    expected: "UNIFORM or KMEANSPP",
                })
            }
        };
        procedure = procedure.sampler_type(sampler_type);
    }
    if let Some(seed_centroids) = optional_f64_matrix(call, "seedCentroids", &["seed_centroids"])? {
        procedure = procedure.seed_centroids(seed_centroids);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed"])? {
        procedure = procedure.random_seed(random_seed);
    }

    Ok(procedure)
}

fn bind_label_propagation(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<LabelPropagationFacade, ShellProcedureError> {
    let mut procedure = graph.label_propagation();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(max_iterations) = optional_u64(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(property) = optional_str(call, "nodeWeightProperty", &["node_weight_property"])? {
        procedure = procedure.node_weight_property(property);
    }
    if let Some(property) = optional_str(call, "seedProperty", &["seed_property"])? {
        procedure = procedure.seed_property(property);
    }

    Ok(procedure)
}

fn bind_leiden(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<LeidenFacade, ShellProcedureError> {
    let mut procedure = graph.leiden();

    if let Some(gamma) = optional_f64(call, "gamma", &[])? {
        procedure = procedure.gamma(gamma);
    }
    if let Some(theta) = optional_f64(call, "theta", &[])? {
        procedure = procedure.theta(theta);
    }
    if let Some(tolerance) = optional_f64(call, "tolerance", &[])? {
        procedure = procedure.tolerance(tolerance);
    }
    if let Some(max_iterations) = optional_usize(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(include) = optional_bool(
        call,
        "includeIntermediateCommunities",
        &["include_intermediate_communities"],
    )? {
        procedure = procedure.include_intermediate_communities(include);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed"])? {
        procedure = procedure.random_seed(random_seed);
    }
    if let Some(seed_communities) =
        optional_u64_array(call, "seedCommunities", &["seed_communities"])?
    {
        procedure = procedure.seed_communities(seed_communities);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_louvain(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<LouvainFacade, ShellProcedureError> {
    let mut procedure = graph.louvain();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(max_iterations) = optional_usize(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(max_levels) = optional_usize(call, "maxLevels", &["max_levels"])? {
        procedure = procedure.max_levels(max_levels);
    }
    if let Some(tolerance) = optional_f64(call, "tolerance", &[])? {
        procedure = procedure.tolerance(tolerance);
    }
    if let Some(gamma) = optional_f64(call, "gamma", &[])? {
        procedure = procedure.gamma(gamma);
    }
    if let Some(theta) = optional_f64(call, "theta", &[])? {
        procedure = procedure.theta(theta);
    }
    if let Some(include) = optional_bool(
        call,
        "includeIntermediateCommunities",
        &["include_intermediate_communities"],
    )? {
        procedure = procedure.include_intermediate_communities(include);
    }
    if let Some(seed_property) = optional_str(call, "seedProperty", &["seed_property"])? {
        procedure = procedure.seed_property(seed_property);
    }

    Ok(procedure)
}

fn bind_modularity(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<ModularityFacade, ShellProcedureError> {
    let community_property = required_string(call, "communityProperty", &["community_property"])?;
    let mut procedure = graph.modularity(community_property);

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_scc(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<SccFacade, ShellProcedureError> {
    let mut procedure = graph.scc();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_triangle(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<TriangleFacade, ShellProcedureError> {
    let mut procedure = graph.triangle();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(max_degree) = optional_u64(call, "maxDegree", &["max_degree"])? {
        procedure = procedure.max_degree(max_degree);
    }

    Ok(procedure)
}

fn bind_wcc(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<WccFacade, ShellProcedureError> {
    let mut procedure = graph.wcc();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(min_batch_size) = optional_usize(call, "minBatchSize", &["min_batch_size"])? {
        procedure = procedure.min_batch_size(min_batch_size);
    }
    if let Some(threshold) = optional_f64(call, "threshold", &[])? {
        procedure = procedure.threshold(threshold);
    }
    if let Some(seed_property) = optional_str(call, "seedProperty", &["seed_property"])? {
        procedure = procedure.seed_property(seed_property);
    }

    Ok(procedure)
}

fn bind_knn(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<KnnFacade, ShellProcedureError> {
    let properties = required_property_metrics(call, "nodeProperties", &["node_properties"])?;
    let default_metric = optional_str(call, "similarityMetric", &["similarity_metric"])?
        .map(|value| parse_knn_metric(value, "similarityMetric"))
        .transpose()?
        .unwrap_or_default();
    let mut properties = properties.into_iter();
    let primary = properties.next().expect("nodeProperties is non-empty");
    let primary_metric = primary
        .metric
        .as_deref()
        .map(|value| parse_knn_metric(value, "nodeProperties"))
        .transpose()?
        .unwrap_or(default_metric);
    let mut procedure = graph.knn(primary.name).metric(primary_metric);
    for property in properties {
        let metric = property
            .metric
            .as_deref()
            .map(|value| parse_knn_metric(value, "nodeProperties"))
            .transpose()?
            .unwrap_or(default_metric);
        procedure = procedure.add_property(property.name, metric);
    }
    configure_knn(procedure, call)
}

fn bind_filtered_knn(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<FilteredKnnFacade, ShellProcedureError> {
    let properties = required_property_metrics(call, "nodeProperties", &["node_properties"])?;
    let default_metric = optional_str(call, "similarityMetric", &["similarity_metric"])?
        .map(|value| parse_knn_metric(value, "similarityMetric"))
        .transpose()?
        .unwrap_or_default();
    let mut properties = properties.into_iter();
    let primary = properties.next().expect("nodeProperties is non-empty");
    let primary_metric = primary
        .metric
        .as_deref()
        .map(|value| parse_knn_metric(value, "nodeProperties"))
        .transpose()?
        .unwrap_or(default_metric);
    let mut procedure = graph.filtered_knn(primary.name).metric(primary_metric);
    for property in properties {
        let metric = property
            .metric
            .as_deref()
            .map(|value| parse_knn_metric(value, "nodeProperties"))
            .transpose()?
            .unwrap_or(default_metric);
        procedure = procedure.add_property(property.name, metric);
    }
    if let Some(k) = optional_usize(call, "topK", &["top_k"])? {
        procedure = procedure.k(k);
    }
    if let Some(sampled_k) = optional_usize(call, "sampledK", &["sampled_k"])? {
        procedure = procedure.sampled_k(sampled_k);
    }
    if let Some(max_iterations) = optional_usize(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(initial_sampler) = optional_str(call, "initialSampler", &["initial_sampler"])? {
        procedure = procedure.initial_sampler(parse_knn_sampler(initial_sampler)?);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed"])? {
        procedure = procedure.random_seed(Some(random_seed));
    }
    if let Some(rate) = optional_f64(call, "perturbationRate", &["perturbation_rate"])? {
        procedure = procedure.perturbation_rate(rate);
    }
    if let Some(random_joins) = optional_usize(call, "randomJoins", &["random_joins"])? {
        procedure = procedure.random_joins(random_joins);
    }
    if let Some(threshold) = optional_u64(call, "updateThreshold", &["update_threshold"])? {
        procedure = procedure.update_threshold(threshold);
    }
    if let Some(cutoff) = optional_f64(call, "similarityCutoff", &["similarity_cutoff"])? {
        procedure = procedure.similarity_cutoff(cutoff);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(labels) = optional_string_or_array(
        call,
        "sourceNodeLabels",
        &["sourceNodeLabel", "source_node_labels", "source_node_label"],
    )? {
        procedure = procedure.source_labels(labels.into_iter().map(NodeLabel::of).collect());
    }
    if let Some(labels) = optional_string_or_array(
        call,
        "targetNodeLabels",
        &["targetNodeLabel", "target_node_labels", "target_node_label"],
    )? {
        procedure = procedure.target_labels(labels.into_iter().map(NodeLabel::of).collect());
    }
    Ok(procedure)
}

fn configure_knn(
    mut procedure: KnnFacade,
    call: &ShellComponentCall,
) -> Result<KnnFacade, ShellProcedureError> {
    if let Some(k) = optional_usize(call, "topK", &["top_k"])? {
        procedure = procedure.k(k);
    }
    if let Some(sampled_k) = optional_usize(call, "sampledK", &["sampled_k"])? {
        procedure = procedure.sampled_k(sampled_k);
    }
    if let Some(max_iterations) = optional_usize(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(initial_sampler) = optional_str(call, "initialSampler", &["initial_sampler"])? {
        procedure = procedure.initial_sampler(parse_knn_sampler(initial_sampler)?);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed"])? {
        procedure = procedure.random_seed(Some(random_seed));
    }
    if let Some(rate) = optional_f64(call, "perturbationRate", &["perturbation_rate"])? {
        procedure = procedure.perturbation_rate(rate);
    }
    if let Some(random_joins) = optional_usize(call, "randomJoins", &["random_joins"])? {
        procedure = procedure.random_joins(random_joins);
    }
    if let Some(threshold) = optional_u64(call, "updateThreshold", &["update_threshold"])? {
        procedure = procedure.update_threshold(threshold);
    }
    if let Some(cutoff) = optional_f64(call, "similarityCutoff", &["similarity_cutoff"])? {
        procedure = procedure.similarity_cutoff(cutoff);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    Ok(procedure)
}

fn bind_node_similarity(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<NodeSimilarityFacade, ShellProcedureError> {
    let mut procedure = graph.node_similarity();
    if let Some(metric) = optional_str(call, "similarityMetric", &["similarity_metric"])? {
        procedure = procedure.metric(parse_node_similarity_metric(metric)?);
    }
    procedure = configure_node_similarity(procedure, call)?;
    Ok(procedure)
}

fn bind_filtered_node_similarity(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<FilteredNodeSimilarityFacade, ShellProcedureError> {
    let mut procedure = graph.filtered_node_similarity();
    if let Some(metric) = optional_str(call, "similarityMetric", &["similarity_metric"])? {
        procedure = procedure.metric(parse_node_similarity_metric(metric)?);
    }
    if let Some(cutoff) = optional_f64(call, "similarityCutoff", &["similarity_cutoff"])? {
        procedure = procedure.similarity_cutoff(cutoff);
    }
    if let Some(top_k) = optional_usize(call, "topK", &["top_k"])? {
        procedure = procedure.top_k(top_k);
    }
    if let Some(top_n) = optional_usize(call, "topN", &["top_n"])? {
        procedure = procedure.top_n(top_n);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(property) = optional_str(call, "weightProperty", &["weight_property"])? {
        procedure = procedure.weight_property(property.to_string());
    }
    if let Some(labels) = optional_string_or_array(
        call,
        "sourceNodeLabels",
        &["sourceNodeLabel", "source_node_labels", "source_node_label"],
    )? {
        procedure = procedure.source_labels(labels.into_iter().map(NodeLabel::of).collect());
    }
    if let Some(labels) = optional_string_or_array(
        call,
        "targetNodeLabels",
        &["targetNodeLabel", "target_node_labels", "target_node_label"],
    )? {
        procedure = procedure.target_labels(labels.into_iter().map(NodeLabel::of).collect());
    }
    Ok(procedure)
}

fn configure_node_similarity(
    mut procedure: NodeSimilarityFacade,
    call: &ShellComponentCall,
) -> Result<NodeSimilarityFacade, ShellProcedureError> {
    if let Some(cutoff) = optional_f64(call, "similarityCutoff", &["similarity_cutoff"])? {
        procedure = procedure.similarity_cutoff(cutoff);
    }
    if let Some(top_k) = optional_usize(call, "topK", &["top_k"])? {
        procedure = procedure.top_k(top_k);
    }
    if let Some(top_n) = optional_usize(call, "topN", &["top_n"])? {
        procedure = procedure.top_n(top_n);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }
    if let Some(property) = optional_str(call, "weightProperty", &["weight_property"])? {
        procedure = procedure.weight_property(property.to_string());
    }
    Ok(procedure)
}

fn parse_knn_metric(
    value: &str,
    input: &'static str,
) -> Result<SimilarityMetric, ShellProcedureError> {
    match value.to_ascii_uppercase().as_str() {
        "DEFAULT" => Ok(SimilarityMetric::Default),
        "COSINE" => Ok(SimilarityMetric::Cosine),
        "EUCLIDEAN" => Ok(SimilarityMetric::Euclidean),
        "PEARSON" => Ok(SimilarityMetric::Pearson),
        "JACCARD" => Ok(SimilarityMetric::Jaccard),
        "OVERLAP" => Ok(SimilarityMetric::Overlap),
        _ => Err(ShellProcedureError::InvalidInput {
            input,
            expected: "DEFAULT, COSINE, EUCLIDEAN, PEARSON, JACCARD, or OVERLAP",
        }),
    }
}

fn parse_knn_sampler(value: &str) -> Result<KnnSamplerType, ShellProcedureError> {
    match value.to_ascii_uppercase().as_str() {
        "UNIFORM" => Ok(KnnSamplerType::Uniform),
        "RANDOMWALK" => Ok(KnnSamplerType::RandomWalk),
        _ => Err(ShellProcedureError::InvalidInput {
            input: "initialSampler",
            expected: "UNIFORM or RANDOMWALK",
        }),
    }
}

fn parse_node_similarity_metric(value: &str) -> Result<NodeSimilarityMetric, ShellProcedureError> {
    match value.to_ascii_uppercase().as_str() {
        "JACCARD" => Ok(NodeSimilarityMetric::Jaccard),
        "COSINE" => Ok(NodeSimilarityMetric::Cosine),
        "OVERLAP" => Ok(NodeSimilarityMetric::Overlap),
        _ => Err(ShellProcedureError::InvalidInput {
            input: "similarityMetric",
            expected: "JACCARD, COSINE, or OVERLAP",
        }),
    }
}

fn bind_hits(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<HitsCentralityFacade, ShellProcedureError> {
    let mut procedure = graph.hits();

    if let Some(max_iterations) = optional_usize(call, "maxIterations", &["max_iterations"])? {
        procedure = procedure.max_iterations(max_iterations);
    }
    if let Some(tolerance) = optional_f64(call, "tolerance", &[])? {
        procedure = procedure.tolerance(tolerance);
    }
    if let Some(hub_property) = optional_str(call, "hubProperty", &["hub_property"])? {
        procedure = procedure.hub_property(hub_property);
    }
    if let Some(auth_property) = optional_str(call, "authProperty", &["auth_property"])? {
        procedure = procedure.auth_property(auth_property);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_delta_stepping(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<DeltaSteppingBuilder, ShellProcedureError> {
    let mut procedure =
        graph
            .delta_stepping()
            .source(required_u64(call, "source", &["sourceNode"])?);

    if let Some(delta) = optional_f64(call, "delta", &[])? {
        procedure = procedure.delta(delta);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(store_predecessors) =
        optional_bool(call, "storePredecessors", &["store_predecessors"])?
    {
        procedure = procedure.store_predecessors(store_predecessors);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_dag_longest_path(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<DagLongestPathBuilder, ShellProcedureError> {
    let mut procedure = graph.dag_longest_path();

    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_kspanning_tree(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<KSpanningTreeBuilder, ShellProcedureError> {
    let mut procedure = graph.kspanning_tree().source_node(required_u64(
        call,
        "source",
        &["sourceNode", "sourceNodeId"],
    )?);

    if let Some(k) = optional_u64(call, "k", &[])? {
        procedure = procedure.k(k);
    }
    if let Some(objective) = optional_str(call, "objective", &[])? {
        procedure = procedure.objective(objective);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }

    Ok(procedure)
}

fn bind_pagerank(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<PageRankFacade, ShellProcedureError> {
    let mut procedure = graph.pagerank();

    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(source_nodes) = optional_u64_array(call, "sourceNodes", &["source_nodes"])? {
        procedure = procedure.source_nodes(source_nodes);
    }
    if let Some(variant) = optional_str(call, "variant", &["pageRankVariant"])? {
        procedure = match variant {
            "pagerank" | "page_rank" => procedure.page_rank(),
            "articlerank" | "article_rank" => procedure.article_rank(),
            "eigenvector" => procedure.eigenvector(),
            _ => {
                return Err(ShellProcedureError::InvalidInput {
                    input: "variant",
                    expected: "`pagerank`, `article_rank`, or `eigenvector`",
                })
            }
        };
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.relationship_weight_property(weight_property);
    }
    if let Some(max_iterations) = optional_u64(call, "maxIterations", &["max_iterations"])? {
        let max_iterations =
            u32::try_from(max_iterations).map_err(|_| ShellProcedureError::InvalidInput {
                input: "maxIterations",
                expected: "a 32-bit unsigned integer",
            })?;
        procedure = procedure.iterations(max_iterations);
    }
    if let Some(damping_factor) = optional_f64(call, "dampingFactor", &["damping_factor"])? {
        procedure = procedure.damping_factor(damping_factor);
    }
    if let Some(tolerance) = optional_f64(call, "tolerance", &[])? {
        procedure = procedure.tolerance(tolerance);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_random_walk(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<RandomWalkBuilder, ShellProcedureError> {
    let mut procedure = graph.random_walk();

    if let Some(source_nodes) =
        optional_u64_array(call, "sourceNodes", &["source_nodes", "sources"])?
    {
        procedure = procedure.source_nodes(source_nodes);
    }
    if let Some(walks_per_node) = optional_usize(call, "walksPerNode", &["walks_per_node"])? {
        procedure = procedure.walks_per_node(walks_per_node);
    }
    if let Some(walk_length) = optional_usize(call, "walkLength", &["walk_length"])? {
        procedure = procedure.walk_length(walk_length);
    }
    if let Some(return_factor) = optional_f64(call, "returnFactor", &["return_factor"])? {
        procedure = procedure.return_factor(return_factor);
    }
    if let Some(in_out_factor) = optional_f64(call, "inOutFactor", &["in_out_factor"])? {
        procedure = procedure.in_out_factor(in_out_factor);
    }
    if let Some(random_seed) = optional_u64(call, "randomSeed", &["random_seed", "seed"])? {
        procedure = procedure.random_seed(random_seed);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_spanning_tree(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<SpanningTreeBuilder, ShellProcedureError> {
    let mut procedure = graph.spanning_tree().start_node(required_u64(
        call,
        "startNode",
        &["start_node", "source", "sourceNode"],
    )?);

    if let Some(compute_minimum) = optional_bool(call, "computeMinimum", &["compute_minimum"])? {
        procedure = procedure.compute_minimum(compute_minimum);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_steiner_tree(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<SteinerTreeBuilder, ShellProcedureError> {
    let target_nodes = optional_u64_array(call, "targetNodes", &["target_nodes", "targets"])?
        .ok_or(ShellProcedureError::MissingInput("targetNodes"))?;
    if target_nodes.is_empty() {
        return Err(ShellProcedureError::InvalidInput {
            input: "targetNodes",
            expected: "a non-empty array of unsigned integers",
        });
    }

    let mut procedure = graph
        .steiner_tree()
        .source_node(required_u64(call, "source", &["sourceNode"])?)
        .target_nodes(target_nodes);

    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.relationship_weight_property(weight_property);
    }
    if let Some(delta) = optional_f64(call, "delta", &[])? {
        procedure = procedure.delta(delta);
    }
    if let Some(apply_rerouting) = optional_bool(call, "applyRerouting", &["apply_rerouting"])? {
        procedure = procedure.apply_rerouting(apply_rerouting);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_topological_sort(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<TopologicalSortBuilder, ShellProcedureError> {
    let mut procedure = graph.topological_sort();

    if let Some(compute_max_distance) = optional_bool(
        call,
        "computeMaxDistanceFromSource",
        &["compute_max_distance_from_source", "computeMaxDistance"],
    )? {
        procedure = procedure.compute_max_distance(compute_max_distance);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}

fn bind_yens(
    graph: &GraphFacade,
    call: &ShellComponentCall,
) -> Result<YensBuilder, ShellProcedureError> {
    let mut procedure = graph
        .yens()
        .source(required_u64(call, "source", &["sourceNode"])?)
        .target(required_u64(call, "target", &["targetNode"])?);

    if let Some(k) = optional_usize(call, "k", &[])? {
        procedure = procedure.k(k);
    }
    if let Some(weight_property) = optional_str(
        call,
        "weightProperty",
        &["weight_property", "relationshipWeightProperty"],
    )? {
        procedure = procedure.weight_property(weight_property);
    }
    if let Some(relationship_types) =
        optional_string_array(call, "relationshipTypes", &["relationship_types"])?
    {
        procedure = procedure.relationship_types(relationship_types);
    }
    if let Some(direction) = optional_str(call, "direction", &["traversalDirection"])? {
        procedure = procedure.direction(direction);
    }
    if let Some(track_relationships) =
        optional_bool(call, "trackRelationships", &["track_relationships"])?
    {
        procedure = procedure.track_relationships(track_relationships);
    }
    if let Some(concurrency) = optional_usize(call, "concurrency", &[])? {
        procedure = procedure.concurrency(concurrency);
    }

    Ok(procedure)
}
