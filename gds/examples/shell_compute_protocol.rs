//! Shell Control Protocol.
//!
//! This example upgrades the internal Shell DSL into a backend control protocol:
//! it authors a component plan, binds it through ShellProcedureControl, and
//! executes a real graph algorithm through the procedure runtime.
//!
//! Run with:
//!   cargo run -p gds --example shell_compute_protocol

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use gds::config::GraphStoreConfig;
use gds::core::utils::warnings::PerDatabaseUserLogStore;
use gds::procedures::pipelines::{
    LocalPipelinesProcedureFacade, PipelineModelStore, RequestScopedDependencies,
};
use gds::procedures::shell::{
    ShellProcedureControl, ShellProcedureControlDependencies, ShellProcedureResult,
};
use gds::projection::eval::pipeline::PipelineCatalog;
use gds::projection::RelationshipType;
use gds::shell::GdsShell;
use gds::task::memory::MemoryTracker;
use gds::task::progress::PerDatabaseTaskStore;
use gds::types::catalog::{GraphCatalog, InMemoryGraphCatalog};
use gds::types::graph::{MappedNodeId, RelationshipTopology, SimpleIdMap};
use gds::types::graph_store::{
    Capabilities, DatabaseId, DatabaseInfo, DatabaseLocation, DefaultGraphStore, GraphName,
};
use gds::types::prelude::GraphStore;
use gds::types::schema::{Direction, GraphSchema, MutableGraphSchema};
use gds::types::user::User;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Control Protocol ==");
    let root_taskframe = "taskframe::root.shell_compute_protocol";

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Graph Store",
        "Materialize a small directed graph store used by the Shell runtime.",
    );
    let graph_edges = [(0, 1), (0, 2), (1, 3), (2, 4), (3, 4)];
    let store = Arc::new(store_from_directed_edges(5, &graph_edges));
    let frame_path = fixture_root.join("00-frame.csv");
    fs::write(&frame_path, dataframe_seed_csv(&graph_edges))?;
    let graph_path = fixture_root.join("00-graph.txt");
    fs::write(
        &graph_path,
        graph_description(root_taskframe, store.node_count(), &graph_edges),
    )?;
    println!("nodes: {}", store.node_count());
    println!("relationships: {}", store.relationship_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!("persisted: {}", fixture_path(&graph_path));
    println!();

    stage(
        1,
        "Shell DSL Plan",
        "Use GdsShell.component_plan() to author a BFS component call.",
    );
    let shell = GdsShell::new();
    let plan = shell.component_plan().bfs(0).track_paths(true).stream();
    let plan_path = fixture_root.join("01-shell-plan.txt");
    fs::write(
        &plan_path,
        format!(
            "origin: {:?}\ncomponent: {}\nmode: {:?}\ninputs: {:?}\n",
            plan.origin(),
            plan.calls()[0].component.as_str(),
            plan.calls()[0].mode,
            plan.calls()[0].inputs,
        ),
    )?;
    let program_features_path = fixture_root.join("01-program-features.txt");
    fs::write(
        &program_features_path,
        program_features_description(root_taskframe),
    )?;
    println!("plan calls: {}", plan.len());
    println!("persisted: {}", fixture_path(&plan_path));
    println!("persisted: {}", fixture_path(&program_features_path));
    println!();

    stage(
        2,
        "Shell Procedure Control",
        "Bind the Shell plan into procedure control and execute it against the graph.",
    );
    let control = shell_procedure_control(&store)?;
    let runtime = control.runtime_with_graph("shell-compute", empty_pipelines())?;
    let binding = runtime.bind_plan(&plan)?;
    let result = runtime.invoke_plan(&plan)?;

    let result_path = fixture_root.join("02-shell-result.txt");

    let result_exposition = match result.invocations()[0].result() {
        ShellProcedureResult::BfsStream(rows) => {
            let text = shell_result_description(
                root_taskframe,
                binding.len(),
                result.len(),
                result.invocations()[0].component().as_str(),
                rows,
            );
            println!("bfs rows: {}", rows.len());
            if let Some(sample) = rows
                .iter()
                .find(|row| row.source != row.target)
                .or(rows.first())
            {
                println!(
                    "sample path: source={} target={} path={:?} cost={}",
                    sample.source, sample.target, sample.path, sample.cost
                );
            }
            text
        }
        _other => return Err("unexpected shell result variant".into()),
    };
    fs::write(&result_path, result_exposition)?;

    let shell_return_path = fixture_root.join("02-shell-return.txt");
    fs::write(
        &shell_return_path,
        shell_return_description(root_taskframe, binding.len(), result.len()),
    )?;

    let pureform_path = fixture_root.join("03-pureform-principle.txt");
    fs::write(
        &pureform_path,
        pureform_principle_description(root_taskframe),
    )?;

    println!("persisted: {}", fixture_path(&result_path));
    println!("persisted: {}", fixture_path(&shell_return_path));
    println!("persisted: {}", fixture_path(&pureform_path));
    println!();

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &frame_path,
            &graph_path,
            &plan_path,
            &program_features_path,
            &result_path,
            &shell_return_path,
            &pureform_path,
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn shell_procedure_control(
    store: &Arc<DefaultGraphStore>,
) -> Result<ShellProcedureControl, Box<dyn std::error::Error>> {
    let graph_catalog: Arc<dyn GraphCatalog> = Arc::new(InMemoryGraphCatalog::new());
    graph_catalog.set("shell-compute", Arc::clone(store));

    let dependencies = ShellProcedureControlDependencies::new(
        User::from("shell-example"),
        DatabaseId::new("shell-example-db"),
        graph_catalog,
        Arc::new(PerDatabaseTaskStore::new()),
        Arc::new(PerDatabaseUserLogStore::new()),
        Arc::new(MemoryTracker::new(u64::MAX)),
    );

    Ok(ShellProcedureControl::new(dependencies))
}

fn empty_pipelines() -> LocalPipelinesProcedureFacade {
    LocalPipelinesProcedureFacade::new(
        RequestScopedDependencies::with_runtime_dependencies(
            User::from("shell-example"),
            Arc::new(InMemoryGraphCatalog::new()),
            Arc::new(PipelineModelStore::new()),
        ),
        Arc::new(PipelineCatalog::new()),
    )
}

fn store_from_directed_edges(node_count: usize, edges: &[(usize, usize)]) -> DefaultGraphStore {
    let mut outgoing: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];
    let mut incoming: Vec<Vec<MappedNodeId>> = vec![Vec::new(); node_count];

    for &(source, target) in edges {
        outgoing[source].push(MappedNodeId::new(target as u64));
        incoming[target].push(MappedNodeId::new(source as u64));
    }

    let rel_type = RelationshipType::of("REL");

    let mut schema_builder = MutableGraphSchema::empty();
    schema_builder
        .relationship_schema_mut()
        .add_relationship_type(rel_type.clone(), Direction::Directed);
    let schema: GraphSchema = schema_builder.build();

    let mut relationship_topologies = HashMap::new();
    relationship_topologies.insert(
        rel_type,
        RelationshipTopology::new(outgoing, Some(incoming)),
    );

    let original_ids: Vec<i64> = (0..node_count as i64).collect();
    let id_map = SimpleIdMap::from_original_ids(original_ids);

    DefaultGraphStore::new(
        GraphStoreConfig::default(),
        GraphName::new("shell-compute"),
        DatabaseInfo::new(
            DatabaseId::new("shell-compute-db"),
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
        .join("fixtures/collections/shell/shell_compute_protocol")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_compute_protocol/{file_name}")
}

fn dataframe_seed_csv(edges: &[(usize, usize)]) -> String {
    let mut csv = String::from("source,target,relationship\n");
    for &(source, target) in edges {
        csv.push_str(&format!("{source},{target},REL\n"));
    }
    csv
}

fn graph_description(root_taskframe: &str, node_count: usize, edges: &[(usize, usize)]) -> String {
    let mut adjacency = vec![Vec::new(); node_count];
    for &(source, target) in edges {
        adjacency[source].push(target);
    }

    let mut text = String::from("Shell Compute Protocol Graph\n\n");
    text.push_str(&format!("root-taskframe: {root_taskframe}\n"));
    text.push_str("control-domain: ShellStoreControl\n");
    text.push_str(&format!("nodes: {}\n", node_count));
    text.push_str(&format!("relationships: {}\n\n", edges.len()));
    text.push_str("adjacency:\n");
    for (source, targets) in adjacency.iter().enumerate() {
        text.push_str(&format!("  {} -> {:?}\n", source, targets));
    }
    text.push_str("\nedges:\n");
    for &(source, target) in edges {
        text.push_str(&format!("  {} -> {}\n", source, target));
    }
    text
}

fn program_features_description(root_taskframe: &str) -> String {
    format!(
        "Shell Program Features\n\n\
         root-taskframe: {root_taskframe}\n\
                 doctrinal-claim: this artifact bundle is a topic map for platform design, not only a demo trace.\n\
         relation: Shell mediates DataFrame -> TaskFrame through procedure-bound execution.\n\n\
                 architectural-layers:\n\
                     - Kernel: graph store, topology, runtime traversal evidence\n\
                     - Agent: shell plan orchestration and procedure control binding\n\
                     - Logic: pureform and eval-readable rationale for why outcomes count as knowledge\n\n\
         mediation-path:\n\
           1. shell_ (plan authoring and orchestration)\n\
           2. proc_ (algorithm semantic mediation)\n\
           3. eval_ (result intelligibility and doctrine-facing reflection)\n\n\
         invariant-checks:\n\
           - empirical_adequacy: traversal rows are produced by runtime execution\n\
           - reflexive_consistency: plan, runtime binding, and return are all persisted\n\
           - dialectical_mediation: shell command form is realized as procedure result form\n"
    )
}

fn shell_result_description(
    root_taskframe: &str,
    binding_count: usize,
    result_count: usize,
    component: &str,
    rows: &[gds::algo::algorithms::pathfinding::result::PathResult],
) -> String {
    let mut text = String::new();
    text.push_str("Shell Procedure Result\n\n");
    text.push_str(&format!("root-taskframe: {root_taskframe}\n"));
    text.push_str(&format!("binding_count: {binding_count}\n"));
    text.push_str(&format!("result_count: {result_count}\n"));
    text.push_str(&format!("component: {component}\n"));
    text.push_str("mode: Stream\n");
    text.push_str(&format!("row_count: {}\n\n", rows.len()));
    text.push_str("topic-map-role: Kernel empirical adequacy witness for shell::compute.\n\n");
    text.push_str("rows:\n");
    for row in rows {
        text.push_str(&format!(
            "  source={} target={} path={:?} cost={}\n",
            row.source, row.target, row.path, row.cost
        ));
    }
    text
}

fn shell_return_description(
    root_taskframe: &str,
    binding_count: usize,
    result_count: usize,
) -> String {
    format!(
        "Shell Return Register\n\n\
         root-taskframe: {root_taskframe}\n\
         doctrinal-role: Agent return surface that composes shell/proc/eval mediation.\n\
         address: shell.return.unified\n\
         pipeline: ModelFeaturePlan\n\
         algebra: ProgramFeature\n\
         trace_valid: true\n\
         semdataset_ready: true\n\
         pureform_return_ready: true\n\
         bindings: {binding_count}\n\
         invocations: {result_count}\n\n\
         mediation-registry:\n\
           - shell_: command form and orchestration surface\n\
           - proc_: algorithm runtime and semantic control surface\n\
           - eval_: intelligibility surface for downstream doctrinal reading\n"
    )
}

fn pureform_principle_description(root_taskframe: &str) -> String {
    format!(
        "Pureform Principle: Shell Control Protocol\n\n\
         root-taskframe: {root_taskframe}\n\n\
         Thesis\n\
         Shell is not an external UI veneer. It is a mediation layer that carries\n\
         DataFrame-seeded structure into TaskFrame-governed execution form.\n\n\
         Antithesis\n\
         If shell scripts remain merely declarative, then algorithmic truth is\n\
         displaced into undocumented runtime interiors and loses explanatory force.\n\n\
         Mediation\n\
         The shell_ plan binds to proc_ runtime control, and eval_ artifacts render\n\
         the resulting movement as inspectable knowledge.\n\n\
         Consequence\n\
         This fixture is a top-level path exemplar:\n\
         DataFrame seed -> Shell plan -> Procedure execution -> Eval-readable return,\n\
         all tethered to one root TaskFrame authority surface.\n\n\
         Topic Map Reading\n\
         Read this folder as if it were the software design map itself:\n\
         00-frame/00-graph establish the Kernel scene,\n\
         01-shell-plan/01-program-features establish Agent movement,\n\
         02-shell-result/02-shell-return/03-pureform-principle establish Logic closure.\n"
    )
}

fn manifest(
    frame_path: &Path,
    graph_path: &Path,
    plan_path: &Path,
    program_features_path: &Path,
    result_path: &Path,
    shell_return_path: &Path,
    pureform_path: &Path,
) -> String {
    format!(
        "Shell Control Protocol Fixture\n\n\
         Namespace: shell::compute\n\n\
         Doctrinal Method Topic Map\n\
         This directory is authored to read as a top-level software design map:\n\
         DataFrame -> TaskFrame through shell/proc/eval mediation under one root authority.\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: DataFrame seed for the graph relation surface.\n\n\
         00 Graph\n\
         artifact: {}\n\
         meaning: in-memory graph store with explicit adjacency and edge listings.\n\n\
         01 Plan\n\
         artifact: {}\n\
         meaning: Shell internal DSL plan for a BFS traversal call.\n\n\
         01 Program Features\n\
         artifact: {}\n\
         meaning: mediation mapping from shell_ to proc_ to eval_ under one TaskFrame root.\n\n\
         02 Result\n\
         artifact: {}\n\
         meaning: procedure runtime output with full BFS row exposition.\n\n\
         02 Return\n\
         artifact: {}\n\
         meaning: shell return register and readiness state for reflective reuse.\n\n\
         03 Pureform Principle\n\
         artifact: {}\n\
         meaning: doctrinal account of why this path is a valid mediation of algorithmic knowledge.\n\n\
         Summary Map\n\
         Kernel   -> 00-frame.csv, 00-graph.txt, 02-shell-result.txt\n\
         Agent    -> 01-shell-plan.txt, 02-shell-return.txt\n\
         Logic    -> 01-program-features.txt, 03-pureform-principle.txt\n",
        fixture_path(frame_path),
        fixture_path(graph_path),
        fixture_path(plan_path),
        fixture_path(program_features_path),
        fixture_path(result_path),
        fixture_path(shell_return_path),
        fixture_path(pureform_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
