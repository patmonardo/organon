//! Dataset Feature Structures fixture.
//!
//! Exercises the Essence feature fold: FeatStruct construction,
//! parse_featstruct, format_featstruct, unify_featstruct, and subsumes.
//!
//! Run with:
//!   cargo run -p gds --example dataset_feature_structures

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Feature Structures ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Hand-Built FeatStruct",
        "FeatStruct::dict constructs a typed attribute-value matrix.",
    );

    let mut verb_dict: BTreeMap<String, FeatValue> = BTreeMap::new();
    verb_dict.insert("cat".to_string(), FeatValue::text("V"));
    verb_dict.insert("num".to_string(), FeatValue::text("sg"));
    verb_dict.insert("tense".to_string(), FeatValue::text("past"));
    let verb_fs = FeatStruct::dict(verb_dict);

    let formatted = format_featstruct(&verb_fs);
    println!("verb fs: {}", formatted);

    let built_path = fixture_root.join("00-built.txt");
    fs::write(&built_path, format!("verb: {formatted}\n"))?;
    println!("persisted: {}", fixture_path(&built_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Parse FeatStruct",
        "parse_featstruct decodes the NLTK-style feature structure notation.",
    );

    let input = "[cat='NP', num='pl', agr=[num='pl', per=3]]";
    let parsed = parse_featstruct(input).map_err(|e| e.message().to_string())?;
    let roundtrip = format_featstruct(&parsed);
    println!("input:     {}", input);
    println!("roundtrip: {}", roundtrip);

    let parse_path = fixture_root.join("01-parse.txt");
    fs::write(
        &parse_path,
        format!("input: {input}\nroundtrip: {roundtrip}\n"),
    )?;
    println!("persisted: {}", fixture_path(&parse_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Unification",
        "unify_featstruct merges two feature structures by value unification.",
    );

    // partial: cat=NP + num=?X agrees with full: cat=NP, num=pl
    let left_input = "[cat='NP']";
    let right_input = "[cat='NP', num='pl']";
    let left_fs = parse_featstruct(left_input).map_err(|e| e.message().to_string())?;
    let right_fs = parse_featstruct(right_input).map_err(|e| e.message().to_string())?;

    let unified = unify_featstruct(&left_fs, &right_fs, None);
    let unified_str = unified
        .as_ref()
        .map(format_featstruct)
        .unwrap_or_else(|| "FAIL".to_string());
    println!("left:  {}", format_featstruct(&left_fs));
    println!("right: {}", format_featstruct(&right_fs));
    println!("unified: {}", unified_str);

    // conflicting: cat=NP vs cat=VP → None
    let conflict_input = "[cat='VP']";
    let conflict_fs = parse_featstruct(conflict_input).map_err(|e| e.message().to_string())?;
    let conflict_unified = unify_featstruct(&left_fs, &conflict_fs, None);
    println!(
        "conflict: {}",
        conflict_unified
            .as_ref()
            .map(format_featstruct)
            .unwrap_or_else(|| "FAIL (expected)".to_string())
    );

    let unify_path = fixture_root.join("02-unify.txt");
    fs::write(
        &unify_path,
        format!(
            "left: {}\nright: {}\nunified: {}\nconflict NP/VP: {}\n",
            format_featstruct(&left_fs),
            format_featstruct(&right_fs),
            unified_str,
            conflict_unified
                .map(|fs| format_featstruct(&fs))
                .unwrap_or_else(|| "FAIL".to_string()),
        ),
    )?;
    println!("persisted: {}", fixture_path(&unify_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Subsumption",
        "subsumes_featstruct checks whether one structure is more general.",
    );

    let general_input = "[cat='NP']";
    let specific_input = "[cat='NP', num='sg']";
    let general_fs = parse_featstruct(general_input).map_err(|e| e.message().to_string())?;
    let specific_fs = parse_featstruct(specific_input).map_err(|e| e.message().to_string())?;

    let gen_subsumes_spec = subsumes_featstruct(&general_fs, &specific_fs);
    let spec_subsumes_gen = subsumes_featstruct(&specific_fs, &general_fs);
    println!(
        "general subsumes specific: {}  (expected: true)",
        gen_subsumes_spec
    );
    println!(
        "specific subsumes general: {}  (expected: false)",
        spec_subsumes_gen
    );

    let subs_path = fixture_root.join("03-subsumes.txt");
    fs::write(
        &subs_path,
        format!(
            "general: {}\nspecific: {}\ngeneral subsumes specific: {}\nspecific subsumes general: {}\n",
            format_featstruct(&general_fs),
            format_featstruct(&specific_fs),
            gen_subsumes_spec,
            spec_subsumes_gen,
        ),
    )?;
    println!("persisted: {}", fixture_path(&subs_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&built_path, &parse_path, &unify_path, &subs_path),
    )?;
    println!("\nmanifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_feature_structures")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_feature_structures/{file_name}")
}

fn manifest(built_path: &Path, parse_path: &Path, unify_path: &Path, subs_path: &Path) -> String {
    format!(
        "Dataset Feature Structures Fixture\n\n\
         Namespace: dataset::feature::featstruct\n\n\
         00 Hand-Built\n\
         artifact: {}\n\
         meaning: FeatStruct::dict construction + format_featstruct.\n\n\
         01 Parse\n\
         artifact: {}\n\
         meaning: parse_featstruct roundtrip over nested attribute-value matrix.\n\n\
         02 Unify\n\
         artifact: {}\n\
         meaning: unify_featstruct merges compatible structures, fails on conflict.\n\n\
         03 Subsumes\n\
         artifact: {}\n\
         meaning: subsumes_featstruct checks generality ordering.\n",
        fixture_path(built_path),
        fixture_path(parse_path),
        fixture_path(unify_path),
        fixture_path(subs_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
