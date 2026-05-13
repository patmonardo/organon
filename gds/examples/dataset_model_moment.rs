//! Dataset Model Moment: Box 1 preparation in depth.
//!
//! Model (R4) is the naming moment of the Essence fold. This example drills into
//! `prepare_model`: how ModelSpec + FeatStruct seed + FeatureMark entries pass through
//! Box 1 essence resolution, producing a ModelEssence with per-feature Modality stamps.
//!
//! Three-box pipeline:
//!   Box 1 — Model preparation (this example)
//!   Box 2 — Feature execution (lowers MarkedFeatures to Polars Expr)
//!   Box 3 — Image realization (OntologyDataFrameImage + provenance)
//!
//! Run with:
//!   cargo run -p gds --example dataset_model_moment

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Model Moment ==");
    println!("Box 1: prepare_model — ModelSpec + FeatStruct seed + FeatureMark → ModelEssence.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // -------------------------------------------------------------------
    // Stage 0: Frame Entry
    // The DataFrame body. Each row is a lexical token with a POS tag and role.
    // -------------------------------------------------------------------
    stage(
        0,
        "Frame Entry",
        "Immediate DataFrame body: tokens with POS and role columns.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3, 4, 5]),
        (token: ["concept", "judgment", "syllogism", "inference", "procedure"]),
        (pos: ["noun", "noun", "noun", "noun", "noun"]),
        (role: ["subject", "predicate", "ground", "consequence", "operation"]),
    )?;
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("columns: {:?}", table.column_names());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 1: ModelSpec
    // The spec declares what kind of model this is, its input and output views.
    // -------------------------------------------------------------------
    stage(
        1,
        "ModelSpec",
        "Model names itself: id, kind, input view, output view.",
    );
    let spec = ModelSpec {
        id: ModelId("doctrine.dataset.model_moment.tagger".to_string()),
        kind: ModelKind::Tagger,
        input: ModelView::Tokens,
        output: ModelView::Tags,
        description: Some("POS tagger model: tokens → tags with role marks.".to_string()),
    };
    println!("id:          {:?}", spec.id.0);
    println!("kind:        {:?}", spec.kind);
    println!("input view:  {:?}", spec.input);
    println!("output view: {:?}", spec.output);
    let spec_path = fixture_root.join("01-model-spec.txt");
    fs::write(
        &spec_path,
        format!(
            "id: {}\nkind: {:?}\ninput: {:?}\noutput: {:?}\ndescription: {}\n",
            spec.id.0,
            spec.kind,
            spec.input,
            spec.output,
            spec.description.as_deref().unwrap_or(""),
        ),
    )?;
    println!("persisted: {}", fixture_path(&spec_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 2: FeatStruct seed constraints
    // The seed is the model's prior essence: all features will unify against it.
    // Here we seed with {pos: "noun"} — the model is a noun-tagger.
    // -------------------------------------------------------------------
    stage(
        2,
        "Seed Constraints",
        "FeatStruct seed: the model's prior essential commitments.",
    );
    let seed = parse_featstruct("[pos='noun']").expect("valid featstruct");
    println!("seed: {}", format_featstruct(&seed));
    let seed_path = fixture_root.join("02-seed-constraints.txt");
    fs::write(
        &seed_path,
        format!(
            "seed: {}\nnote: all feature marks unify against this.\n",
            format_featstruct(&seed)
        ),
    )?;
    println!("persisted: {}", fixture_path(&seed_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 3: FeatureMark entries
    // Each mark declares: feature (carrying a Plan), optional FeatStruct mark,
    // and whether the model requires the mark (Required) or treats it as
    // optional (Optional/contingent).
    // -------------------------------------------------------------------
    stage(
        3,
        "FeatureMark entries",
        "Each feature is stamped before Box 2 execution. Three cases: contingent, optional, required.",
    );

    let ds = table_to_dataset(&table, "model-moment-frame");

    // Feature 1: no mark → always Contingent (predicate-free)
    let f_term = Feature::new(
        Plan::new(Source::Value(ds.clone()))
            .named("term-id")
            .push_step(Step::Select(vec![polars::prelude::col("term_id")])),
    )
    .named("term_id");

    // Feature 2: optional mark {pos: noun} → unifies with seed, Contingent (optional)
    let f_token = Feature::new(
        Plan::new(Source::Value(ds.clone()))
            .named("token")
            .push_step(Step::Select(vec![polars::prelude::col("token")])),
    )
    .named("token");

    // Feature 3: required mark {pos: noun} → unifies with seed, Necessary
    let f_pos = Feature::new(
        Plan::new(Source::Value(ds.clone()))
            .named("pos")
            .push_step(Step::Select(vec![polars::prelude::col("pos")])),
    )
    .named("pos");

    // Feature 4: required mark {pos: verb} → FAILS to unify with {pos: noun} seed → Impossible
    let f_role = Feature::new(
        Plan::new(Source::Value(ds.clone()))
            .named("role")
            .push_step(Step::Select(vec![polars::prelude::col("role")])),
    )
    .named("role");

    let mark_pos_noun = parse_featstruct("[pos='noun']").expect("valid featstruct");
    let mark_pos_verb = parse_featstruct("[pos='verb']").expect("valid featstruct");

    let feature_marks = vec![
        FeatureMark::contingent(f_term),
        FeatureMark::optional(f_token, mark_pos_noun.clone()),
        FeatureMark::required(f_pos, mark_pos_noun.clone()),
        FeatureMark::required(f_role, mark_pos_verb), // will be Impossible
    ];

    println!("feature marks: {} entries", feature_marks.len());
    println!("  [0] term_id   — contingent (no mark)            → expected: Contingent");
    println!(
        "  [1] token     — optional [pos='noun']           → expected: Contingent (extends seed)"
    );
    println!("  [2] pos       — required [pos='noun']           → expected: Necessary (entailed)");
    println!(
        "  [3] role      — required [pos='verb']           → expected: Impossible (contradicts)"
    );
    println!();

    // -------------------------------------------------------------------
    // Stage 4: prepare_model — Box 1 execution
    // -------------------------------------------------------------------
    stage(
        4,
        "prepare_model",
        "Box 1 walks features in order, unifying each mark against the accumulated essence.",
    );
    // prepare_model only returns Err on StrictContradiction (strict mode), which we don't trigger;
    // contradictions appear as Modality::Impossible in the report instead.
    let essence = prepare_model(spec, Some(seed), feature_marks)
        .unwrap_or_else(|e| panic!("unexpected strict contradiction: {e:?}"));

    println!("executable features: {}", essence.executable().count());
    println!("impossible features: {}", essence.impossibilities().count());
    println!("any_impossible: {}", essence.report.any_impossible());
    if let Some(acc) = &essence.accumulated {
        println!("accumulated essence: {}", format_featstruct(acc));
    }
    println!();
    println!("preparation steps:");
    for step in &essence.report.steps {
        println!(
            "  {:?}  {:?}  — {}",
            step.feature_name.as_deref().unwrap_or("<unnamed>"),
            step.modality,
            step.note,
        );
    }
    println!();

    let essence_path = fixture_root.join("04-model-essence.txt");
    let mut essence_lines = vec![
        format!("executable: {}", essence.executable().count()),
        format!("impossible: {}", essence.impossibilities().count()),
        format!(
            "accumulated: {}",
            essence
                .accumulated
                .as_ref()
                .map(|fs| format_featstruct(fs))
                .unwrap_or_default()
        ),
        String::new(),
        "steps:".to_string(),
    ];
    for step in &essence.report.steps {
        essence_lines.push(format!(
            "  {} | {:?} | {}",
            step.feature_name.as_deref().unwrap_or("<unnamed>"),
            step.modality,
            step.note,
        ));
    }
    let modality_counts = {
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for step in &essence.report.steps {
            *counts.entry(format!("{:?}", step.modality)).or_default() += 1;
        }
        counts
    };
    essence_lines.push(String::new());
    essence_lines.push("modality summary:".to_string());
    for (k, v) in &modality_counts {
        essence_lines.push(format!("  {k}: {v}"));
    }
    fs::write(&essence_path, essence_lines.join("\n") + "\n")?;
    println!("persisted: {}", fixture_path(&essence_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 5: Box 2 preview — executable loop
    // In a real Box 2 this would lower each MarkedFeature to a Polars Expr.
    // Here we describe which features are in the executable path.
    // -------------------------------------------------------------------
    stage(
        5,
        "Box 2 preview",
        "Executable features are those Box 2 will lower to Polars Expr / LazyFrame.",
    );
    let box2_path = fixture_root.join("05-box2-executable.txt");
    let mut box2_lines = vec!["Executable features (Necessary + Contingent):".to_string()];
    for mf in essence.executable() {
        box2_lines.push(format!(
            "  {:?}  {:?}",
            mf.feature.name().unwrap_or("<unnamed>"),
            mf.modality,
        ));
    }
    box2_lines.push(String::new());
    box2_lines.push("Impossible features (Box 3 records as contradictions):".to_string());
    for mf in essence.impossibilities() {
        box2_lines.push(format!(
            "  {:?}  {:?}  mark: {}",
            mf.feature.name().unwrap_or("<unnamed>"),
            mf.modality,
            mf.mark
                .as_ref()
                .map(|fs| format_featstruct(fs))
                .unwrap_or_default(),
        ));
    }
    println!("{}", box2_lines.join("\n"));
    fs::write(&box2_path, box2_lines.join("\n") + "\n")?;
    println!("persisted: {}", fixture_path(&box2_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Dataset Model Moment Fixture\n\n\
             Namespace: dataset::model (R4)\n\n\
             Files:\n\
             00-frame.csv                  — immediate DataFrame body\n\
             01-model-spec.txt             — ModelSpec: id, kind, input/output views\n\
             02-seed-constraints.txt       — FeatStruct seed for Box 1\n\
             04-model-essence.txt          — ModelEssence: preparation steps + modalities\n\
             05-box2-executable.txt        — executable vs impossible features\n\
             README.txt                    — this manifest\n\n\
             Modality cases demonstrated:\n\
               Contingent  — no mark or optional mark that extends\n\
               Necessary   — required mark, unifies + entailed by seed\n\
               Impossible  — required mark contradicts accumulated essence\n",
        ),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn table_to_dataset(table: &gds::collections::dataframe::GDSDataFrame, name: &str) -> Dataset {
    Dataset::named(name, table.clone())
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_model_moment")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_model_moment/{file_name}")
}
