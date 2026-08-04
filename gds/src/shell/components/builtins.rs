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

const STREAM_STATS_MODES: &[ShellComponentMode] =
    &[ShellComponentMode::Stream, ShellComponentMode::Stats];

const STATS_MODE: &[ShellComponentMode] = &[ShellComponentMode::Stats];

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
    (Pipeline) => {
        "pipeline"
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
    (stream_stats $category:ident, $name:literal) => {
        ShellComponentDescriptor::new(
            concat!("gds.algorithms.", category_name!($category), ".", $name),
            $name,
            ShellComponentCategory::$category,
            STREAM_STATS_MODES,
        )
    };
    (stats $category:ident, $name:literal) => {
        ShellComponentDescriptor::new(
            concat!("gds.algorithms.", category_name!($category), ".", $name),
            $name,
            ShellComponentCategory::$category,
            STATS_MODE,
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
    component!(stream_stats Embeddings, "fast_rp"),
    component!(stats Embeddings, "hash_gnn"),
    component!(stats Embeddings, "graphsage"),
    component!(stream_stats Embeddings, "node2vec"),
    component!(invoke Miscellaneous, "to_undirected"),
    component!(invoke Miscellaneous, "scale_properties"),
    component!(invoke Miscellaneous, "index_inverse"),
    component!(invoke Miscellaneous, "collapse_path"),
];

pub static PIPELINE_BUILTINS: &[ShellComponentDescriptor] = &[
    ShellComponentDescriptor::new(
        "gds.pipelines.link_prediction.create",
        "create_link_prediction_pipeline",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
    ShellComponentDescriptor::new(
        "gds.pipelines.node_classification.create",
        "create_node_classification_pipeline",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
    ShellComponentDescriptor::new(
        "gds.pipelines.node_regression.create",
        "create_node_regression_pipeline",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
    ShellComponentDescriptor::new(
        "gds.pipelines.list",
        "list_pipelines",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
    ShellComponentDescriptor::new(
        "gds.pipelines.exists",
        "pipeline_exists",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
    ShellComponentDescriptor::new(
        "gds.pipelines.drop",
        "drop_pipeline",
        ShellComponentCategory::Pipeline,
        INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::Pipeline),
];
