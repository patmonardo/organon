use crate::applications::services::algorithms_dispatch;
use crate::applications::services::graph_store_catalog_dispatch;
use crate::core::User;
use crate::shell::builtin_component;
use crate::shell::ShellComponentId;
use crate::types::catalog::GraphCatalog;
use crate::types::graph_store::DatabaseId;
use serde_json::{json, Value};
use std::sync::Arc;

type AlgorithmHandler = fn(&Value, Arc<dyn GraphCatalog>) -> Value;

#[derive(Clone, Copy)]
struct AlgorithmBinding {
    component: ShellComponentId,
    handler: AlgorithmHandler,
}

impl AlgorithmBinding {
    const fn new(component: &'static str, handler: AlgorithmHandler) -> Self {
        Self {
            component: ShellComponentId::new(component),
            handler,
        }
    }
}

macro_rules! algorithm_binding {
    ($category:literal, $name:literal, $handler:ident) => {
        AlgorithmBinding::new(
            concat!("gds.algorithms.", $category, ".", $name),
            algorithms_dispatch::$handler,
        )
    };
}

static ALGORITHM_BINDINGS: &[AlgorithmBinding] = &[
    algorithm_binding!("pathfinding", "bfs", handle_bfs),
    algorithm_binding!("pathfinding", "dfs", handle_dfs),
    algorithm_binding!("pathfinding", "dijkstra", handle_dijkstra),
    algorithm_binding!("pathfinding", "bellman_ford", handle_bellman_ford),
    algorithm_binding!("pathfinding", "astar", handle_astar),
    algorithm_binding!("pathfinding", "delta_stepping", handle_delta_stepping),
    algorithm_binding!("pathfinding", "dag_longest_path", handle_dag_longest_path),
    algorithm_binding!("pathfinding", "kspanningtree", handle_kspanningtree),
    algorithm_binding!("pathfinding", "yens", handle_yens),
    algorithm_binding!(
        "pathfinding",
        "all_shortest_paths",
        handle_all_shortest_paths
    ),
    algorithm_binding!("pathfinding", "spanning_tree", handle_spanning_tree),
    algorithm_binding!("pathfinding", "steiner_tree", handle_steiner_tree),
    algorithm_binding!("pathfinding", "topological_sort", handle_topological_sort),
    algorithm_binding!("pathfinding", "random_walk", handle_random_walk),
    algorithm_binding!("centrality", "pagerank", handle_pagerank),
    algorithm_binding!(
        "centrality",
        "articulation_points",
        handle_articulation_points
    ),
    algorithm_binding!("centrality", "betweenness", handle_betweenness),
    algorithm_binding!("centrality", "bridges", handle_bridges),
    algorithm_binding!("centrality", "celf", handle_celf),
    algorithm_binding!("centrality", "closeness", handle_closeness),
    algorithm_binding!("centrality", "degree_centrality", handle_degree_centrality),
    algorithm_binding!("centrality", "harmonic", handle_harmonic),
    algorithm_binding!("centrality", "hits", handle_hits),
    algorithm_binding!("community", "approx_max_kcut", handle_approx_max_kcut),
    algorithm_binding!("community", "conductance", handle_conductance),
    algorithm_binding!("community", "k1coloring", handle_k1coloring),
    algorithm_binding!("community", "kcore", handle_kcore),
    algorithm_binding!("community", "kmeans", handle_kmeans),
    algorithm_binding!("community", "label_propagation", handle_label_propagation),
    algorithm_binding!("community", "leiden", handle_leiden),
    algorithm_binding!("community", "louvain", handle_louvain),
    algorithm_binding!("community", "modularity", handle_modularity),
    algorithm_binding!("community", "scc", handle_scc),
    algorithm_binding!("community", "triangle", handle_triangle),
    algorithm_binding!("community", "wcc", handle_wcc),
    algorithm_binding!("similarity", "knn", handle_knn),
    algorithm_binding!("similarity", "node_similarity", handle_node_similarity),
    algorithm_binding!("similarity", "filtered_knn", handle_filtered_knn),
    algorithm_binding!(
        "similarity",
        "filtered_node_similarity",
        handle_filtered_node_similarity
    ),
    algorithm_binding!("embeddings", "fast_rp", handle_fast_rp),
    algorithm_binding!("embeddings", "hash_gnn", handle_hash_gnn),
    algorithm_binding!("embeddings", "graphsage", handle_graphsage),
    algorithm_binding!("embeddings", "node2vec", handle_node2vec),
    algorithm_binding!("miscellaneous", "to_undirected", handle_to_undirected),
    algorithm_binding!("miscellaneous", "scale_properties", handle_scale_properties),
    algorithm_binding!("miscellaneous", "index_inverse", handle_index_inverse),
    algorithm_binding!("miscellaneous", "collapse_path", handle_collapse_path),
];

fn algorithm_binding(name: &str) -> Option<AlgorithmBinding> {
    let component = builtin_component(name)?.descriptor();
    ALGORITHM_BINDINGS
        .iter()
        .find(|binding| binding.component == component.id)
        .copied()
}

fn err(op: &str, code: &str, message: &str) -> Value {
    json!({
        "ok": false,
        "op": op,
        "error": { "code": code, "message": message }
    })
}

/// Dispatches algorithm requests by op name.
///
/// This keeps the TS-JSON boundary (`tsjson_napi.rs`) thin and makes the routing
/// shape stable for schema generation.
pub fn handle_algorithms(request: &Value, catalog: Arc<dyn GraphCatalog>) -> Value {
    let op = request.get("op").and_then(|v| v.as_str()).unwrap_or("");

    match algorithm_binding(op) {
        Some(binding) => (binding.handler)(request, catalog),
        None => err(op, "UNSUPPORTED_OP", "Unsupported algorithms operation."),
    }
}

/// Dispatches graph-store-catalog requests.
///
/// Routing is delegated to the graph-store-catalog dispatcher.
pub fn handle_graph_store_catalog(
    request: &Value,
    user: &dyn User,
    db: &DatabaseId,
    catalog: Arc<dyn GraphCatalog>,
) -> Value {
    graph_store_catalog_dispatch::handle_graph_store_catalog(request, user, db, catalog)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::BuiltinComponentSuite;
    use crate::types::catalog::InMemoryGraphCatalog;

    fn dispatch_without_graph(op: &str) -> Value {
        let request = json!({ "op": op });
        handle_algorithms(&request, Arc::new(InMemoryGraphCatalog::new()))
    }

    #[test]
    fn every_builtin_algorithm_has_an_application_binding() {
        let suite = BuiltinComponentSuite::algorithms();

        assert_eq!(ALGORITHM_BINDINGS.len(), suite.all().len());
        for component in suite.all() {
            assert!(algorithm_binding(component.id.as_str()).is_some());
            assert!(algorithm_binding(component.alias).is_some());
        }
    }

    #[test]
    fn unsupported_algorithm_has_no_application_binding() {
        assert!(algorithm_binding("gds.algorithms.unknown").is_none());
    }

    #[test]
    fn bfs_canonical_id_and_alias_reach_the_same_handler() {
        let alias_response = dispatch_without_graph("bfs");
        let canonical_response = dispatch_without_graph("gds.algorithms.pathfinding.bfs");

        assert_eq!(canonical_response, alias_response);
        assert_eq!(canonical_response["op"], "bfs");
        assert_eq!(canonical_response["error"]["code"], "INVALID_REQUEST");
    }

    #[test]
    fn dijkstra_canonical_id_and_alias_reach_the_same_handler() {
        let alias_response = dispatch_without_graph("dijkstra");
        let canonical_response = dispatch_without_graph("gds.algorithms.pathfinding.dijkstra");

        assert_eq!(canonical_response, alias_response);
        assert_eq!(canonical_response["op"], "dijkstra");
        assert_eq!(canonical_response["error"]["code"], "INVALID_REQUEST");
    }
}
