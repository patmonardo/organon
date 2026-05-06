//! Dataset IO Catalog fixture.
//!
//! Exercises DatasetCatalog table registration/loading while explicitly
//! disabling GraphFrame projection work in this walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataset_io_catalog_graphframe

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::catalog::DatasetCatalog;
use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset IO Catalog (GraphFrame Disabled) ==");

    let fixture_root = fixture_root();
    let catalog_root = fixture_root.join("catalog");
    let data_root = catalog_root.join("data");
    if catalog_root.exists() {
        fs::remove_dir_all(&catalog_root)?;
    }
    fs::create_dir_all(&data_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Graph Tables",
        "Create node/edge source tables for catalog registration.",
    );

    let nodes = gds::tbl_def!(
        (node_id: i64 => [1, 2, 3, 4]),
        (label: ["Person", "Person", "Topic", "Topic"]),
        (name: ["Ada", "Blaise", "Logic", "Math"]),
    )?;
    let edges = gds::tbl_def!(
        (src: i64 => [1, 2, 1]),
        (dst: i64 => [3, 4, 4]),
        (rel: ["INTERESTED_IN", "INTERESTED_IN", "STUDIES"]),
    )?;

    let nodes_path = data_root.join("nodes.csv");
    let edges_path = data_root.join("edges.csv");
    nodes.write_csv(&path_string(&nodes_path))?;
    edges.write_csv(&path_string(&edges_path))?;

    let source_path = fixture_root.join("00-source-paths.txt");
    fs::write(
        &source_path,
        format!(
            "nodes={}\nedges={}\n",
            nodes_path.display(),
            edges_path.display(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&source_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Dataset Catalog",
        "Register node/edge tables and persist catalog manifest.",
    );

    let mut catalog = DatasetCatalog::load(&catalog_root)?;
    let nodes_entry = catalog.register_table_path("graph_nodes", &nodes_path, None, None)?;
    let edges_entry = catalog.register_table_path("graph_edges", &edges_path, None, None)?;
    catalog.save()?;

    let schema_nodes = catalog.resolve_table_schema("graph_nodes")?;
    let schema_edges = catalog.resolve_table_schema("graph_edges")?;

    let catalog_path = fixture_root.join("01-catalog.txt");
    fs::write(
        &catalog_path,
        format!(
            "nodes_entry={} format={:?} schema_source={}\n\
             edges_entry={} format={:?} schema_source={}\n",
            nodes_entry.name,
            nodes_entry.io_policy.format,
            schema_nodes.source.as_str(),
            edges_entry.name,
            edges_entry.io_policy.format,
            schema_edges.source.as_str(),
        ),
    )?;
    println!("registered entries: graph_nodes, graph_edges");
    println!("persisted: {}", fixture_path(&catalog_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Catalog Load",
        "Load datasets by name and verify table dimensions.",
    );

    let reloaded = DatasetCatalog::load(&catalog_root)?;
    let nodes_ds = reloaded.load_table("graph_nodes")?;
    let edges_ds = reloaded.load_table("graph_edges")?;

    let load_path = fixture_root.join("02-load.txt");
    fs::write(
        &load_path,
        format!(
            "nodes_rows={} nodes_cols={}\nedges_rows={} edges_cols={}\n",
            nodes_ds.row_count(),
            nodes_ds.column_count(),
            edges_ds.row_count(),
            edges_ds.column_count(),
        ),
    )?;
    println!(
        "nodes shape: ({}, {})",
        nodes_ds.row_count(),
        nodes_ds.column_count()
    );
    println!(
        "edges shape: ({}, {})",
        edges_ds.row_count(),
        edges_ds.column_count()
    );
    println!("persisted: {}", fixture_path(&load_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "GraphFrame Disabled",
        "Explicitly disable GraphFrame projection and persist a no-graphframe witness.",
    );

    let disabled_path = fixture_root.join("03-graphframe-disabled.txt");
    let disabled_report = disabled_report(&nodes_ds, &edges_ds);
    fs::write(&disabled_path, disabled_report)?;
    println!("persisted: {}", fixture_path(&disabled_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&source_path, &catalog_path, &load_path, &disabled_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn disabled_report(nodes_ds: &Dataset, edges_ds: &Dataset) -> String {
    format!(
        "graphframe_enabled=false\nstatus=disabled-by-user-request\nnodes_rows={}\nedges_rows={}\n",
        nodes_ds.row_count(),
        edges_ds.row_count(),
    )
}
fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_io_catalog_graphframe")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_io_catalog_graphframe/{file_name}")
}

fn manifest(
    source_path: &Path,
    catalog_path: &Path,
    load_path: &Path,
    disabled_path: &Path,
) -> String {
    format!(
        "Dataset IO Catalog Fixture\n\n\
         Namespace: dataset::io\n\n\
         00 Sources\n\
         artifact: {}\n\
         meaning: node/edge source tables written to catalog-local files.\n\n\
         01 Catalog\n\
         artifact: {}\n\
         meaning: DatasetCatalog registration and schema resolution.\n\n\
         02 Load\n\
         artifact: {}\n\
         meaning: named table loading through catalog entries.\n\n\
         03 GraphFrame Disabled\n\
         artifact: {}\n\
         meaning: explicit no-graphframe witness for this example.\n",
        fixture_path(source_path),
        fixture_path(catalog_path),
        fixture_path(load_path),
        fixture_path(disabled_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
