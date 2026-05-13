//! Shell Model Genesis: the three-fold inner arc of the Model moment.
//!
//! The Model moment of Model:Feature::Plan is not atomic — it unfolds
//! as three genesis boxes following the Essence logic triad:
//!
//!   Box 1 — Preparation  (Identity)  prepare_model
//!   Box 2 — Execution    (Difference) execute_essence
//!   Box 3 — Image        (Ground)    realize_from_essence
//!
//! The ProgramFeatureKind arc is the "Law" of the ProgramFeature:
//! the compulsory sequence any scientific language program must declare
//! to be evaluable by the Shell.
//!
//! This example exhibits the genesis knowledge from the Shell side.
//! Descent into the Dataset boxes happens in:
//!   gds/examples/dataset_model_feature_plan.rs
//!
//! Run with:
//!   cargo run -p gds --example shell_model_genesis

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Model Genesis ==");
    println!("Three-fold: Preparation (Identity) → Execution (Difference) → Image (Ground)");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ── Stage 0: Frame entry ─────────────────────────────────────────────────

    stage(0, "Frame Entry", "Shell receives the framed body.");
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["preparation", "execution", "image"]),
        (essence_role: ["identity", "difference", "ground"]),
        (entry_fn: ["prepare_model", "execute_essence", "realize_from_essence"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    // ── Stage 1: ProgramFeature Law ──────────────────────────────────────────

    stage(
        1,
        "ProgramFeature Law",
        "The compulsory arc: every scientific language program must declare its feature kinds in this sequence.",
    );

    let program = program_features(
        "rustscript.shell.model_genesis",
        ["ModelGenesis"],
        [
            program_source(
                "frame",
                "fixtures/collections/shell/shell_model_genesis/00-frame.csv",
            ),
            program_observation("three-box-model-genesis"),
            program_reflection("essence-logic-identity-difference-ground"),
            program_principle("model-is-triadic"),
            program_concept("ModelGenesis"),
            program_judgment("each-box-is-a-moment-not-a-step"),
            program_procedure("descend-to-dataset-model-feature-plan"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-model-genesis")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:shell-model-genesis")
        .facet("model-genesis")
        .source_io(io_path(
            "fixtures/collections/shell/shell_model_genesis/00-frame.csv",
        ))
        .into_shell_with_program_features(program);

    let genesis = shell.model_genesis_knowledge();

    println!("doctrine resource: {}", genesis.doctrine_resource());
    println!("fold:   {}", genesis.fold());
    println!("moment: {:?}", genesis.moment());
    println!();

    println!(
        "ProgramFeature Law ({} kinds):",
        genesis.program_feature_law().len()
    );
    println!("  {}", genesis.program_feature_law().join(" → "));
    println!();

    let law_path = fixture_root.join("01-program-feature-law.txt");
    fs::write(
        &law_path,
        format!(
            "fold: {}\nmoment: {:?}\nlaw ({} kinds):\n  {}\n",
            genesis.fold(),
            genesis.moment(),
            genesis.program_feature_law().len(),
            genesis.program_feature_law().join(" → "),
        ),
    )?;
    println!("persisted: {}", fixture_path(&law_path));
    println!();

    // ── Stage 2: Three Genesis Moments ──────────────────────────────────────

    stage(
        2,
        "Three Genesis Moments",
        "Preparation (Identity) → Execution (Difference) → Image (Ground).",
    );

    let mut genesis_text = String::new();
    for moment in genesis.genesis_moments() {
        println!(
            "  Box {} — {} ({})",
            moment.box_number, moment.name, moment.essence_role
        );
        println!("    input:  {}", moment.primary_input);
        println!("    output: {}", moment.primary_output);
        println!("    fn:     {}", moment.entry_fn);
        println!("    note:   {}", moment.doctrine_note);
        println!();
        genesis_text.push_str(&format!(
            "Box {} — {} ({})\n  input:  {}\n  output: {}\n  fn:     {}\n  note:   {}\n\n",
            moment.box_number,
            moment.name,
            moment.essence_role,
            moment.primary_input,
            moment.primary_output,
            moment.entry_fn,
            moment.doctrine_note,
        ));
    }

    let genesis_path = fixture_root.join("02-genesis-moments.txt");
    fs::write(&genesis_path, &genesis_text)?;
    println!("persisted: {}", fixture_path(&genesis_path));
    println!();

    // ── Stage 3: Genesis Prompt (AI-ready) ──────────────────────────────────

    stage(
        3,
        "Genesis Prompt",
        "AI-ready summary of the genesis arc (ai_assistance_presupposed = true).",
    );
    let prompt = shell.model_genesis_prompt();
    println!("{}", prompt);
    println!();

    let prompt_path = fixture_root.join("03-genesis-prompt.txt");
    fs::write(&prompt_path, &prompt)?;
    println!("persisted: {}", fixture_path(&prompt_path));
    println!();

    // ── Stage 4: Descent pointer ─────────────────────────────────────────────

    stage(
        4,
        "Descent",
        "The three boxes are executable in the Dataset substrate.",
    );
    println!("descends to: {}", genesis.descends_to_example());
    println!("  → dataset::model::prep::prepare_model   (Box 1 — Identity)");
    println!("  → dataset::model::exec::execute_essence (Box 2 — Difference)");
    println!("  → dataset::model::image::realize_from_essence (Box 3 — Ground)");
    println!();

    let descent_text = format!(
        "descends to: {}\n  Box 1 — Identity:    dataset::model::prep::prepare_model\n  Box 2 — Difference:  dataset::model::exec::execute_essence\n  Box 3 — Ground:      dataset::model::image::realize_from_essence\n",
        genesis.descends_to_example()
    );
    let descent_path = fixture_root.join("04-descent.txt");
    fs::write(&descent_path, &descent_text)?;
    println!("persisted: {}", fixture_path(&descent_path));
    println!();

    write_readme(&fixture_root)?;
    println!("== done ==");
    Ok(())
}

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/shell/shell_model_genesis")
}

fn fixture_path(path: &Path) -> String {
    path.strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")))
        .unwrap_or(path)
        .display()
        .to_string()
}

fn path_string(path: &Path) -> String {
    path.to_str().unwrap().to_string()
}

fn stage(n: u8, name: &str, desc: &str) {
    println!("── Stage {} — {} ──", n, name);
    println!("   {}", desc);
    println!();
}

fn write_readme(root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let readme = "\
Shell Model Genesis — fixture root
===================================
Generated by: cargo run -p gds --example shell_model_genesis

Files:
  00-frame.csv               Frame body (three genesis moments as rows)
  01-program-feature-law.txt ProgramFeatureKind compulsory arc (12 kinds)
  02-genesis-moments.txt     Three-fold genesis: Preparation / Execution / Image
  03-genesis-prompt.txt      AI-ready genesis summary
  04-descent.txt             Descent pointers to dataset::model sub-modules

Doctrine thread:
  Shell sees the Model moment as triadic (Essence: Identity/Difference/Ground).
  The ProgramFeature Law is the compulsory sequence any scientific language
  program must declare to be evaluable by the Shell.

  Descent: gds/examples/dataset_model_feature_plan.rs
";
    fs::write(root.join("README.txt"), readme)?;
    Ok(())
}
