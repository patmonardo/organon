//! Shell Feature First: companion moment to Model in Model:Feature::Plan.
//!
//! Run with:
//!   cargo run -p gds --example shell_feature_first

use std::fs;
use std::path::{Path, PathBuf};

use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Shell Feature First ==");
    println!("Feature is Model's companion: predicates and structures the model can evaluate.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Frame Entry",
        "Shell receives a framed body and orients to Feature as Model companion.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3]),
        (term: ["fspec", "unify", "subsumes"]),
        (zone: ["declaration", "dragon", "resolution"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    stage(
        1,
        "Feature Moment",
        "Read the Shell Feature report: topics, ProgramFeature law, and FeatStruct primitives.",
    );
    let program = program_features(
        "rustscript.shell.feature_first",
        ["ShellFeatureFirst"],
        [
            program_source(
                "frame",
                "fixtures/collections/shell/shell_feature_first/00-frame.csv",
            ),
            program_reflection("feature-companion-of-model"),
            program_principle("feature-structures-mediate-model"),
            program_concept("ShellFeatureFirst"),
            program_procedure("descend-to-dataset-model-feature-plan"),
        ],
    );

    let shell = ds_frame(table)
        .named("shell-feature-first")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:shell-feature-first")
        .facet("feature-moment")
        .source_io(io_path(
            "fixtures/collections/shell/shell_feature_first/00-frame.csv",
        ))
        .into_shell_with_program_features(program);

    let feature = shell.feature_moment_knowledge();
    let feature_path = fixture_root.join("01-shell-feature-moment.txt");
    fs::write(
        &feature_path,
        format!(
            "resource: {}\naddress: {:?}\nfold: {}\nmoment: {}\ncompanion: {}\nalgebra: {:?}\nfeature_topics: {:?}\nprogram_feature_law: {}\nfeatstruct_primitives: {:?}\ndragon_note: {}\nai_assistance_presupposed: {}\n",
            feature.doctrine_resource(),
            feature.address(),
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
    println!("feature topics: {:?}", feature.feature_topics());
    println!(
        "featstruct primitives: {:?}",
        feature.featstruct_primitives()
    );
    println!("persisted: {}", fixture_path(&feature_path));
    println!();

    stage(
        2,
        "Dragon Zone",
        "FeatStruct is where unification, subsumption, and reentrance checks operate.",
    );
    let dragon_path = fixture_root.join("02-dragon-zone.txt");
    fs::write(
        &dragon_path,
        format!(
            "note: {}\nprimitives: {}\n",
            feature.dragon_zone_note(),
            feature.featstruct_primitives().join(", "),
        ),
    )?;
    println!("dragon note: {}", feature.dragon_zone_note());
    println!("persisted: {}", fixture_path(&dragon_path));
    println!();

    stage(
        3,
        "Descent",
        "Continue into dataset_model_feature_plan.rs for material feature execution.",
    );
    let descent_path = fixture_root.join("03-descent-dataset-model-feature-plan.txt");
    fs::write(
        &descent_path,
        format!(
            "next_example: {}\nmethod: Feature companions Model; FeatStruct provides structural operations before Plan execution.\n",
            feature.descends_to_example(),
        ),
    )?;
    println!("next example: {}", feature.descends_to_example());
    println!("persisted: {}", fixture_path(&descent_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&frame_path, &feature_path, &dragon_path, &descent_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/shell/shell_feature_first")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/shell/shell_feature_first/{file_name}")
}

fn manifest(
    frame_path: &Path,
    feature_path: &Path,
    dragon_path: &Path,
    descent_path: &Path,
) -> String {
    format!(
        "Shell Feature First Fixture\n\n\
         Namespace: shell::feature\n\n\
         00 Frame\n\
         artifact: {}\n\
         meaning: framed body supplied to Shell.\n\n\
         01 Shell Feature Moment\n\
         artifact: {}\n\
         meaning: Shell-readable Feature companion report.\n\n\
         02 Dragon Zone\n\
         artifact: {}\n\
         meaning: FeatStruct primitive operations and cautions.\n\n\
         03 Dataset Descent\n\
         artifact: {}\n\
         meaning: Doctrinal Method handoff to dataset_model_feature_plan.rs.\n",
        fixture_path(frame_path),
        fixture_path(feature_path),
        fixture_path(dragon_path),
        fixture_path(descent_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
