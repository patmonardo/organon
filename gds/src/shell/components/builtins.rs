use super::ShellComponentCategory;
use super::ShellComponentDescriptor;
use super::ShellComponentMode;

const ALGORITHM_MODES: &[ShellComponentMode] = &[
    ShellComponentMode::Stream,
    ShellComponentMode::Stats,
    ShellComponentMode::Estimate,
    ShellComponentMode::Mutate,
    ShellComponentMode::Write,
];

const INVOKE_MODE: &[ShellComponentMode] = &[ShellComponentMode::Invoke];

macro_rules! category_name {
    (Pathfinding) => {
        "pathfinding"
    };
    (Centrality) => {
        "centrality"
    };
    (Community) => {
        "community"
    };
    (Similarity) => {
        "similarity"
    };
    (Embeddings) => {
        "embeddings"
    };
    (Miscellaneous) => {
        "miscellaneous"
    };
}

macro_rules! component {
    ($category:ident, $name:literal) => {
        ShellComponentDescriptor::new(
            concat!("gds.algorithms.", category_name!($category), ".", $name),
            $name,
            ShellComponentCategory::$category,
            ALGORITHM_MODES,
        )
    };
    (invoke $category:ident, $name:literal) => {
        ShellComponentDescriptor::new(
            concat!("gds.algorithms.", category_name!($category), ".", $name),
            $name,
            ShellComponentCategory::$category,
            INVOKE_MODE,
        )
    };
}

pub static ALGORITHM_BUILTINS: &[ShellComponentDescriptor] = &[
    component!(Pathfinding, "bfs"),
    component!(Pathfinding, "dfs"),
    component!(Pathfinding, "dijkstra"),
    component!(Pathfinding, "bellman_ford"),
    component!(Pathfinding, "astar"),
    component!(Pathfinding, "delta_stepping"),
    component!(Pathfinding, "dag_longest_path"),
    component!(Pathfinding, "kspanningtree"),
    component!(Pathfinding, "yens"),
    component!(Pathfinding, "all_shortest_paths"),
    component!(Pathfinding, "spanning_tree"),
    component!(Pathfinding, "steiner_tree"),
    component!(Pathfinding, "topological_sort"),
    component!(Pathfinding, "random_walk"),
    component!(Centrality, "pagerank"),
    component!(Centrality, "articulation_points"),
    component!(Centrality, "betweenness"),
    component!(Centrality, "bridges"),
    component!(Centrality, "celf"),
    component!(Centrality, "closeness"),
    component!(Centrality, "degree_centrality"),
    component!(Centrality, "harmonic"),
    component!(Centrality, "hits"),
    component!(Community, "approx_max_kcut"),
    component!(Community, "conductance"),
    component!(Community, "k1coloring"),
    component!(Community, "kcore"),
    component!(Community, "kmeans"),
    component!(Community, "label_propagation"),
    component!(Community, "leiden"),
    component!(Community, "louvain"),
    component!(Community, "modularity"),
    component!(Community, "scc"),
    component!(Community, "triangle"),
    component!(Community, "wcc"),
    component!(Similarity, "knn"),
    component!(Similarity, "node_similarity"),
    component!(Similarity, "filtered_knn"),
    component!(Similarity, "filtered_node_similarity"),
    component!(Embeddings, "fast_rp"),
    component!(Embeddings, "hash_gnn"),
    component!(Embeddings, "graphsage"),
    component!(Embeddings, "node2vec"),
    component!(invoke Miscellaneous, "to_undirected"),
    component!(invoke Miscellaneous, "scale_properties"),
    component!(invoke Miscellaneous, "index_inverse"),
    component!(invoke Miscellaneous, "collapse_path"),
];
