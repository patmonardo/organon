//! Pathfinding procedure facade exemplar.
//!
//! Run with:
//!   cargo run -p gds --example proc_pathfinding_procedure

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gds::config::GraphStoreConfig;
use gds::procedures::GraphFacade;
use gds::projection::RelationshipType;
use gds::types::graph::{RelationshipTopology, SimpleIdMap};
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
};
use gds::types::prelude::GraphStore;
use gds::types::schema::{Direction, MutableGraphSchema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Pathfinding Procedure Facade ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Procedure Boundary",
        "Application code enters Pathfinding through GraphFacade procedures.",
    );
    let store = Arc::new(store_from_directed_edges(
        5,
        &[(0, 1), (0, 2), (1, 3), (2, 4), (3, 4)],
    ));
    let graph = GraphFacade::new(Arc::clone(&store));
    let graph_path = fixture_root.join("00-graph.txt");
    fs::write(
        &graph_path,
        format!(
            "nodes: {}\nrelationships: {}\nentrypoint: GraphFacade::bfs / GraphFacade::dfs\n",
            store.node_count(),
            store.relationship_count(),
        ),
    )?;
    println!("nodes: {}", store.node_count());
    println!("relationships: {}", store.relationship_count());
    println!("persisted: {}", fixture_path(&graph_path));
    println!();

    stage(
        1,
        "BFS Stream Mode",
        "Breadth-first search emits the level-order procedure view.",
    );
    let bfs_rows: Vec<_> = graph
        .bfs()
        .source(0)
        .max_depth(3)
        .track_paths(true)
        .stream()?
        .collect();
    let bfs_path = fixture_root.join("01-bfs-stream.txt");
    fs::write(&bfs_path, format!("bfs_rows: {:?}\n", bfs_rows))?;
    println!("bfs row count: {}", bfs_rows.len());
    println!("persisted: {}", fixture_path(&bfs_path));
    println!();

    stage(
        2,
        "DFS Stream Mode",
        "Depth-first search emits the branch-first procedure view.",
    );
    let dfs_rows: Vec<_> = graph
        .dfs()
        .source(0)
        .max_depth(4)
        .track_paths(true)
        .stream()?
        .collect();
    let dfs_path = fixture_root.join("02-dfs-stream.txt");
    fs::write(&dfs_path, format!("dfs_rows: {:?}\n", dfs_rows))?;
    println!("dfs row count: {}", dfs_rows.len());
    println!("persisted: {}", fixture_path(&dfs_path));
    println!();

    stage(
        3,
        "Stats Mode",
        "Stats mode exposes traversal aggregates from the same procedure surface.",
    );
    let bfs_stats = graph.bfs().source(0).max_depth(3).stats()?;
    let dfs_stats = graph.dfs().source(0).max_depth(4).stats()?;
    let stats_path = fixture_root.join("03-stats.txt");
    fs::write(
        &stats_path,
        format!(
            "bfs_nodes_visited: {}\nbfs_max_depth_reached: {}\ndfs_nodes_visited: {}\ndfs_max_depth_reached: {}\n",
            bfs_stats.nodes_visited,
            bfs_stats.max_depth_reached,
            dfs_stats.nodes_visited,
            dfs_stats.max_depth_reached,
        ),
    )?;
    println!("bfs nodes visited: {}", bfs_stats.nodes_visited);
    println!("dfs nodes visited: {}", dfs_stats.nodes_visited);
    println!("persisted: {}", fixture_path(&stats_path));
    println!();

    stage(
        4,
        "Estimate Mode",
        "Memory estimation belongs to the procedure contract and uses traversal graph size.",
    );
    let bfs_memory = graph.bfs().source(0).track_paths(true).estimate_memory();
    let dfs_memory = graph.dfs().source(0).track_paths(true).estimate_memory();
    let memory_path = fixture_root.join("04-memory.txt");
    fs::write(
        &memory_path,
        format!(
            "bfs_min_bytes: {}\nbfs_max_bytes: {}\ndfs_min_bytes: {}\ndfs_max_bytes: {}\n",
            bfs_memory.min(),
            bfs_memory.max(),
            dfs_memory.min(),
            dfs_memory.max(),
        ),
    )?;
    println!(
        "bfs memory: {}..{} bytes",
        bfs_memory.min(),
        bfs_memory.max()
    );
    println!(
        "dfs memory: {}..{} bytes",
        dfs_memory.min(),
        dfs_memory.max()
    );
    println!("persisted: {}", fixture_path(&memory_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&graph_path, &bfs_path, &dfs_path, &stats_path, &memory_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
    let mut outgoing: Vec<Vec<i64>> = vec![Vec::new(); node_count];
    let mut incoming: Vec<Vec<i64>> = vec![Vec::new(); node_count];

    for &(source, target) in edges {
        outgoing[source].push(target as i64);
        incoming[target].push(source as i64);
    }

    let rel_type = RelationshipType::of("REL");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder
        .relationship_schema_mut()
        .add_relationship_type(rel_type.clone(), Direction::Directed);
    let schema = schema_builder.build();

    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(
        rel_type,
        RelationshipTopology::new(outgoing, Some(incoming)),
    );

    let original_ids: Vec<i64> = (0..node_count as i64).collect();
    let id_map = SimpleIdMap::from_original_ids(original_ids);

    DefaultGraphStore::new(
        GraphStoreConfig::default(),
        GraphName::new("proc-pathfinding"),
        DatabaseInfo::new(
            DatabaseId::new("proc-pathfinding-db"),
            DatabaseLocation::remote("localhost", 7687, None, None),
        ),
        schema,
        Capabilities::default(),
        id_map,
        relationship_topologies,
    )
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/procedures/032-pathfinding-procedure-facade")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/procedures/032-pathfinding-procedure-facade/{file_name}")
}

fn manifest(
    graph_path: &Path,
    bfs_path: &Path,
    dfs_path: &Path,
    stats_path: &Path,
    memory_path: &Path,
) -> String {
    format!(
        "Pathfinding Procedure Facade Fixture\n\n\
         Namespace: procedures::pathfinding\n\n\
         00 Graph\n\
         artifact: {}\n\
         meaning: in-memory graph store wrapped by GraphFacade.\n\n\
         01 BFS Stream\n\
         artifact: {}\n\
         meaning: breadth-first rows produced by the procedure facade.\n\n\
         02 DFS Stream\n\
         artifact: {}\n\
         meaning: depth-first rows produced by the procedure facade.\n\n\
         03 Stats\n\
         artifact: {}\n\
         meaning: aggregate traversal results returned by stats mode.\n\n\
         04 Memory\n\
         artifact: {}\n\
         meaning: procedure-level BFS/DFS memory estimates using graph size.\n",
        fixture_path(graph_path),
        fixture_path(bfs_path),
        fixture_path(dfs_path),
        fixture_path(stats_path),
        fixture_path(memory_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
