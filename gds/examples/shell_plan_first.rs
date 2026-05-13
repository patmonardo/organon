//! Shell Plan First: execution ordering moment in Model:Feature::Plan.
//!
//! Run with:
//!   cargo run -p gds --example shell_plan_first

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Plan First ==");
    println!("Plan turns Principle/Concept declarations into executable order.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame Entry",
        "Shell receives a framed body and orients to Plan as execution ordering.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["pipeline", "plan", "dop"]),
        (role: ["order", "coordination", "operation"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "Plan Moment",
        "Read the Shell Plan report: plan topics and runtime readiness surfaces.",
    );
    let program = program_features(
        "rustscript.shell.plan_first",
        ["ShellPlanFirst"],
        [
            program_source(
                "frame",
                "fixtures/collections/shell/shell_plan_first/00-frame.csv",
            ),
            program_reflection("plan-as-execution-order"),
            program_principle("plan-coordinates-feature-execution"),
            program_concept("ShellPlanFirst"),
            program_procedure("descend-to-dataset-model-feature-plan"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-plan-first")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:shell-plan-first")
        .facet("plan-moment")
        .source_io(io_path(
            "fixtures/collections/shell/shell_plan_first/00-frame.csv",
        ))
        .into_shell_with_program_features(program);

    let plan = shell.plan_moment_knowledge();
    let plan_path = fixture_root.join("01-shell-plan-moment.txt");
    fs::write(
        &plan_path,
        format!(
            "resource: {}\naddress: {:?}\nfold: {}\nmoment: {}\nalgebra: {:?}\nplan_topics: {:?}\nruntime_surfaces: {:?}\npipeline_axis: {:?}\ndataset_pipeline_active: {}\nmetapipeline_active: {}\nai_assistance_presupposed: {}\n",
            plan.doctrine_resource(),
            plan.address(),
            plan.fold(),
            plan.moment(),
            plan.algebra(),
            plan.plan_topics(),
            plan.plan_runtime_surfaces(),
            plan.pipeline_axis(),
            plan.has_dataset_pipeline(),
            plan.has_metapipeline(),
            plan.ai_assistance_presupposed(),
        ),
    )?;
    println!("plan topics: {:?}", plan.plan_topics());
    println!("runtime surfaces: {:?}", plan.plan_runtime_surfaces());
    println!("persisted: {}", fixture_path(&plan_path));
    println!();

    stage(
        2,
        "Execution Readiness",
        "Plan readiness is visible through progress, memory estimation, and capability map.",
    );
    let readiness_path = fixture_root.join("02-execution-readiness.txt");
    fs::write(
        &readiness_path,
        format!(
            "pipeline_axis: {:?}\ndataset_pipeline_active: {}\nmetapipeline_active: {}\nruntime_surfaces: {}\n",
            plan.pipeline_axis(),
            plan.has_dataset_pipeline(),
            plan.has_metapipeline(),
            plan.plan_runtime_surfaces().join(", "),
        ),
    )?;
    println!("persisted: {}", fixture_path(&readiness_path));
    println!();

    stage(
        3,
        "Descent",
        "Continue into dataset_model_feature_plan.rs for material plan execution.",
    );
    let descent_path = fixture_root.join("03-descent-dataset-model-feature-plan.txt");
    fs::write(
        &descent_path,
        format!(
            "next_example: {}\nmethod: Plan orders Model and Feature surfaces into executable middle operations.\n",
            plan.descends_to_example(),
        ),
    )?;
    println!("next example: {}", plan.descends_to_example());
    println!("persisted: {}", fixture_path(&descent_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &plan_path, &readiness_path, &descent_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/shell/shell_plan_first")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_plan_first/{file_name}")
}

fn manifest(
    frame_path: &Path,
    plan_path: &Path,
    readiness_path: &Path,
    descent_path: &Path,
) -> String {
    format!(
        "Shell Plan First Fixture\n\n\
         Namespace: shell::plan\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: framed body supplied to Shell.\n\n\
         01 Shell Plan Moment\n\
         artifact: {}\n\
         meaning: Shell-readable Plan moment report.\n\n\
         02 Execution Readiness\n\
         artifact: {}\n\
         meaning: runtime readiness surfaces (progress/memory/capability).\n\n\
         03 Dataset Descent\n\
         artifact: {}\n\
         meaning: Doctrinal Method handoff to dataset_model_feature_plan.rs.\n",
        fixture_path(frame_path),
        fixture_path(plan_path),
        fixture_path(readiness_path),
        fixture_path(descent_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
