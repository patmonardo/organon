//! Shell Compute Protocol walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example shell_compute_protocol

use std::fs;
use std::path::{Path, PathBuf};

use gds::form::ProgramFeatureKind;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Compute Protocol ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame Entry",
        "Shell begins from a concrete DataFrame register body.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["being", "essence", "concept"]),
        (moment: ["immediate", "mediated", "return"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!(
        "shape: {} rows x {} cols",
        table.row_count(),
        table.column_count()
    );
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "Program Features",
        "Shell protocol is configured through internal Rust DSL features.",
    );
    let program = program_features(
        "rustscript.shell.compute_protocol",
        ["ShellComputeProtocol"],
        [
            program_source(
                "dataframe",
                "fixtures/collections/shell/shell_compute_protocol/00-frame.csv",
            ),
            program_reflection("dataset-mediated-register"),
            program_principle("frame-middle-return-trace"),
            program_concept("ShellComputeProtocol"),
            program_feature(
                ProgramFeatureKind::Judgment,
                "all x. (human(x) -> mortal(x))",
                "doctrine://shell/compute-protocol/judgment",
            ),
            program_feature(
                ProgramFeatureKind::Inference,
                "human(socrates) -> mortal(socrates)",
                "doctrine://shell/compute-protocol/inference",
            ),
            program_procedure("persist-shell-compute-protocol"),
        ],
    );
    let features_path = fixture_root.join("01-program-features.txt");
    fs::write(
        &features_path,
        format!(
            "program: {}\nselected_forms: {:?}\nfeature_count: {}\n",
            program.program_name,
            program.selected_forms,
            program.features.len(),
        ),
    )?;
    println!("program: {}", program.program_name);
    println!("feature count: {}", program.features.len());
    println!("persisted: {}", fixture_path(&features_path));
    println!();

    stage(
        2,
        "Shell Materialization",
        "Shell unifies immediate and mediated registers at one address.",
    );
    let shell = ds_frame(table)
        .named("shell-compute-protocol")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:029-shell-compute-protocol")
        .source_io(io_path(
            "fixtures/collections/shell/shell_compute_protocol/00-frame.csv",
        ))
        .into_shell_with_program_features(program)
        .materialize_semdataset_from_texts(&[
            "Socrates is human.",
            "All humans are mortal.",
            "Socrates is mortal.",
        ])?;

    let semantic = shell.semantic_pipeline_knowledge();
    let learning = shell.learning_report();
    let trace = shell.validate_projection_trace();
    let descriptor = shell.descriptor();
    let principle = descriptor.to_pure_form_principle();

    let shell_path = fixture_root.join("02-shell-return.txt");
    fs::write(
        &shell_path,
        format!(
            "address: {:?}\nregister: {:?}\npipeline: {:?}\ntrace_valid: {}\nsemdataset_ready: {}\npureform_return_ready: {}\nreadiness_score: {}\n",
            descriptor.address(),
            shell.register_kind(),
            shell.pipeline_axis(),
            trace.is_valid(),
            semantic.semdataset_ready(),
            semantic.pureform_return_ready(),
            learning.readiness_score(),
        ),
    )?;
    println!("trace valid: {}", trace.is_valid());
    println!("semdataset ready: {}", semantic.semdataset_ready());
    println!(
        "pureform return ready: {}",
        semantic.pureform_return_ready()
    );
    println!("persisted: {}", fixture_path(&shell_path));
    println!();

    stage(
        3,
        "Protocol Witness",
        "The descriptor projects Shell execution toward PureForm principle.",
    );
    let principle_path = fixture_root.join("03-pureform-principle.txt");
    fs::write(
        &principle_path,
        format!(
            "required_fields: {:?}\nruntime_strategy: {}\nexecution_order: {:?}\npatterns: {:?}\n",
            principle.shape.required_fields,
            principle.context.runtime_strategy,
            principle.context.execution_order,
            principle.morph.patterns,
        ),
    )?;
    println!("persisted: {}", fixture_path(&principle_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &features_path, &shell_path, &principle_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/shell/shell_compute_protocol")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_compute_protocol/{file_name}")
}

fn manifest(
    frame_path: &Path,
    features_path: &Path,
    shell_path: &Path,
    principle_path: &Path,
) -> String {
    format!(
        "Shell Compute Protocol Fixture\n\n\
         Namespace: shell::compute\n\n\
         00 Frame Entry\n\
         artifact: {}\n\
         meaning: immediate register supplied to Shell.\n\n\
         01 Program Features\n\
         artifact: {}\n\
         meaning: internal Rust DSL commitments for Shell protocol.\n\n\
         02 Shell Return\n\
         artifact: {}\n\
         meaning: semantic readiness and projection trace state.\n\n\
         03 PureForm Principle\n\
         artifact: {}\n\
         meaning: principle projection witness from the shell descriptor.\n",
        fixture_path(frame_path),
        fixture_path(features_path),
        fixture_path(shell_path),
        fixture_path(principle_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
