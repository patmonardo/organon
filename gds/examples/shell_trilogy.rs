//! Shell Trilogy: Model:Feature::Plan as one chained Shell narrative.
//!
//! The three Essence moments are not separate programs — they are one arc:
//!   Model names the body → Feature structures it → Plan orders it for execution.
//!
//! Run with:
//!   cargo run -p gds --example shell_trilogy

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Trilogy: Model:Feature::Plan ==");
    println!("One Shell, three Essence moments, one arc.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // -------------------------------------------------------------------
    // Stage 0: Frame Entry
    // The framed body is the same body all three moments read.
    // -------------------------------------------------------------------
    stage(
        0,
        "Frame Entry",
        "Shell receives one framed body. All three moments read the same shell.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["model", "feature", "plan"]),
        (role: ["name", "structure", "order"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    // Rich program features spanning all three moments.
    let program = program_features(
        "rustscript.shell.trilogy",
        ["ShellTrilogy"],
        [
            program_source(
                "frame",
                "fixtures/collections/shell/shell_trilogy/00-frame.csv",
            ),
            program_reflection("essence-trilogy-model-feature-plan"),
            program_principle("model-names-feature-structures-plan-orders"),
            program_concept("ShellTrilogy"),
            program_judgment("model-feature-plan-form-one-arc"),
            program_procedure("descend-to-dataset-model-feature-plan"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-trilogy")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:shell-trilogy")
        .facet("model-feature-plan")
        .source_io(io_path(
            "fixtures/collections/shell/shell_trilogy/00-frame.csv",
        ))
        .into_shell_with_program_features(program);

    // -------------------------------------------------------------------
    // Stage 1: Model Moment — Shell names the body
    // -------------------------------------------------------------------
    stage(
        1,
        "Model Moment",
        "Model is the naming moment: Shell assigns an identity to the mediated body.",
    );
    let model = shell.model_moment_knowledge();
    println!("fold:    {}", model.fold());
    println!("moment:  {}", model.moment());
    println!("topics:  {:?}", model.model_topics());
    let model_path = fixture_root.join("01-model-moment.txt");
    fs::write(
        &model_path,
        format!(
            "resource: {}\nfold: {}\nmoment: {}\nalgebra: {:?}\ndataset: {:?}\nprogram: {:?}\nselected_forms: {:?}\nmodel_topics: {:?}\nai_assistance_presupposed: {}\n",
            model.doctrine_resource(),
            model.fold(),
            model.moment(),
            model.algebra(),
            model.dataset_name(),
            model.program_name(),
            model.selected_forms(),
            model.model_topics(),
            model.ai_assistance_presupposed(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&model_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 2: Feature Moment — Shell structures what Model named
    // -------------------------------------------------------------------
    stage(
        2,
        "Feature Moment",
        "Feature is Model's companion: predicates and structures the named body can carry.",
    );
    let feature = shell.feature_moment_knowledge();
    println!("fold:    {}", feature.fold());
    println!("moment:  {}", feature.moment());
    println!("companion: {}", feature.companion_moment());
    println!("topics:  {:?}", feature.feature_topics());
    println!("law:     {}", feature.program_feature_law().join(" -> "));
    let feature_path = fixture_root.join("02-feature-moment.txt");
    fs::write(
        &feature_path,
        format!(
            "resource: {}\nfold: {}\nmoment: {}\ncompanion: {}\nalgebra: {:?}\nfeature_topics: {:?}\nprogram_feature_law: {}\nfeatstruct_primitives: {:?}\ndragon_note: {}\nai_assistance_presupposed: {}\n",
            feature.doctrine_resource(),
            feature.fold(),
            feature.moment(),
            feature.companion_moment(),
            feature.algebra(),
            feature.feature_topics(),
            feature.program_feature_law().join(" -> "),
            feature.featstruct_primitives(),
            feature.dragon_zone_note(),
            feature.ai_assistance_presupposed(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&feature_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 3: Plan Moment — Shell orders what Feature structured
    // -------------------------------------------------------------------
    stage(
        3,
        "Plan Moment",
        "Plan is the execution ordering: Principle/Concept declarations become pipeline steps.",
    );
    let plan = shell.plan_moment_knowledge();
    println!("fold:    {}", plan.fold());
    println!("moment:  {}", plan.moment());
    println!("topics:  {:?}", plan.plan_topics());
    println!("surfaces: {:?}", plan.plan_runtime_surfaces());
    let plan_path = fixture_root.join("03-plan-moment.txt");
    fs::write(
        &plan_path,
        format!(
            "resource: {}\nfold: {}\nmoment: {}\nalgebra: {:?}\nplan_topics: {:?}\nruntime_surfaces: {:?}\npipeline_axis: {:?}\ndataset_pipeline_active: {}\nmetapipeline_active: {}\nai_assistance_presupposed: {}\n",
            plan.doctrine_resource(),
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
    println!("persisted: {}", fixture_path(&plan_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 4: Trilogy Arc — one summary showing the three moments as a whole
    // -------------------------------------------------------------------
    stage(
        4,
        "Trilogy Arc",
        "Model names → Feature structures → Plan orders. One body, three moments, one arc.",
    );
    let arc_path = fixture_root.join("04-trilogy-arc.txt");
    fs::write(
        &arc_path,
        format!(
            "arc: {} | {} | {}\n\
             model_fold: {} | feature_fold: {} | plan_fold: {}\n\
             model_moment: {} | feature_moment: {} | plan_moment: {}\n\
             \n\
             Model names the mediated body.\n\
             Feature provides the predicates and structures Model can evaluate.\n\
             Plan coordinates execution: pipeline -> plan -> dop.\n\
             \n\
             feature_law: {}\n\
             plan_surfaces: {}\n\
             \n\
             All three moments descend to: {}\n",
            model.fold(),
            feature.fold(),
            plan.fold(),
            model.fold(),
            feature.fold(),
            plan.fold(),
            model.moment(),
            feature.moment(),
            plan.moment(),
            feature.program_feature_law().join(" -> "),
            plan.plan_runtime_surfaces().join(", "),
            plan.descends_to_example(),
        ),
    )?;
    println!("model:   {} / {}", model.fold(), model.moment());
    println!("feature: {} / {}", feature.fold(), feature.moment());
    println!("plan:    {} / {}", plan.fold(), plan.moment());
    println!("arc persisted: {}", fixture_path(&arc_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 5: Descent
    // -------------------------------------------------------------------
    stage(
        5,
        "Descent",
        "Continue into dataset_model_feature_plan.rs for material trilogy execution.",
    );
    let descent_path = fixture_root.join("05-descent.txt");
    fs::write(
        &descent_path,
        format!(
            "next_example: {}\n\
             method: Model:Feature::Plan form one arc; dataset_model_feature_plan materializes it.\n",
            plan.descends_to_example(),
        ),
    )?;
    println!("next: {}", plan.descends_to_example());
    println!("persisted: {}", fixture_path(&descent_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Shell Trilogy Fixture\n\n\
             Namespace: shell::essence (Model:Feature::Plan)\n\n\
             Files:\n\
             00-frame.csv                         — shared framed body\n\
             01-model-moment.txt                  — Model names the body\n\
             02-feature-moment.txt                — Feature structures what Model named\n\
             03-plan-moment.txt                   — Plan orders execution\n\
             04-trilogy-arc.txt                   — three moments as one arc\n\
             05-descent.txt                       — pointer to dataset example\n\
             README.txt                           — this manifest\n\n\
             Stage flow:\n\
               0 Frame Entry  -> one shell receives all three moments\n\
               1 Model Moment -> names the mediated body\n\
               2 Feature Moment -> predicates and structures\n\
               3 Plan Moment -> execution ordering\n\
               4 Trilogy Arc -> unified summary\n\
               5 Descent -> dataset_model_feature_plan.rs\n",
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/shell/shell_trilogy")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_trilogy/{file_name}")
}
