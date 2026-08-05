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
const STORE_INVOKE_MODE: &[ShellComponentMode] = &[ShellComponentMode::Invoke];
const STORE_STREAM_MODE: &[ShellComponentMode] = &[ShellComponentMode::Stream];
const STORE_ESTIMATE_MODE: &[ShellComponentMode] = &[ShellComponentMode::Estimate];
const STORE_MUTATE_MODE: &[ShellComponentMode] = &[ShellComponentMode::Mutate];
const STORE_WRITE_MODE: &[ShellComponentMode] = &[ShellComponentMode::Write];

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

pub static STORE_API_BUILTINS: &[ShellComponentDescriptor] = &[
    ShellComponentDescriptor::new(
        "gds.store.graph.put",
        "put_graph_store",
        ShellComponentCategory::StoreApi,
        STORE_WRITE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.catalog.exists",
        "graph_exists",
        ShellComponentCategory::StoreApi,
        STORE_INVOKE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.catalog.list",
        "list_graphs",
        ShellComponentCategory::StoreApi,
        STORE_STREAM_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.catalog.memory_usage",
        "graph_memory_usage",
        ShellComponentCategory::StoreApi,
        STORE_ESTIMATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.catalog.drop",
        "drop_graph",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.catalog.drop_many",
        "drop_graphs",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.node_properties.drop",
        "drop_node_properties",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.relationships.drop",
        "drop_relationships",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.graph_property.drop",
        "drop_graph_property",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.node_properties.write",
        "write_node_properties",
        ShellComponentCategory::StoreApi,
        STORE_WRITE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.relationship_properties.write",
        "write_relationship_properties",
        ShellComponentCategory::StoreApi,
        STORE_WRITE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.relationships.write",
        "write_relationships",
        ShellComponentCategory::StoreApi,
        STORE_WRITE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.node_label.write",
        "write_node_label",
        ShellComponentCategory::StoreApi,
        STORE_WRITE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.node_label.mutate",
        "mutate_label",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.graph_property.stream",
        "stream_graph_property",
        ShellComponentCategory::StoreApi,
        STORE_STREAM_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.node_properties.stream",
        "stream_node_properties",
        ShellComponentCategory::StoreApi,
        STORE_STREAM_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.relationship_properties.stream",
        "stream_relationship_properties",
        ShellComponentCategory::StoreApi,
        STORE_STREAM_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.relationships.stream",
        "stream_relationships",
        ShellComponentCategory::StoreApi,
        STORE_STREAM_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.graph.generate",
        "generate_graph",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.graph.sample",
        "sample_graph",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.subgraph.project",
        "subgraph_project",
        ShellComponentCategory::StoreApi,
        STORE_MUTATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.native_project.estimate",
        "estimate_native_project",
        ShellComponentCategory::StoreApi,
        STORE_ESTIMATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
    ShellComponentDescriptor::new(
        "gds.store.common_neighbour_aware_random_walk.estimate",
        "estimate_common_neighbour_aware_random_walk",
        ShellComponentCategory::StoreApi,
        STORE_ESTIMATE_MODE,
    )
    .with_execution_kind(crate::shell::ShellComponentExecutionKind::StoreApi),
];
