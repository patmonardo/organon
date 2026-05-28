//! Dataset Feature Moment: Feature as projection, binding, and chaining.
//!
//! Feature (R5) is Model's companion: it provides named, typed addresses into Models.
//! Four roles:
//!   1. Projection  — addresses a Model, codomain is a value or another Model (Plan-backed)
//!   2. Binder      — addresses a Model with an unbound, scope-local codomain
//!   3. Reentrancy  — addresses two paths and binds them to the same cell
//!   4. Annotation  — addresses a Document with a provenance tuple
//!
//! This example focuses on Role 1 (Projection) since it is the fully typed path.
//! It builds Features from Plans, constructs a FeatureSpace, chains two features,
//! reads attention_report, and evaluates to a Dataset.
//!
//! Run with:
//!   cargo run -p gds --example dataset_feature_moment

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Feature Moment ==");
    println!(
        "Feature (R5): named, typed addresses into Models. Role 1 — Projection (Plan-backed)."
    );
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // -------------------------------------------------------------------
    // Stage 0: Frame Entry
    // Dataset body: POS-tagged terms with role and score columns.
    // -------------------------------------------------------------------
    stage(
        0,
        "Frame Entry",
        "Immediate body: tokens with pos, role, score.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3, 4, 5]),
        (token: ["concept", "judgment", "syllogism", "inference", "procedure"]),
        (pos: ["noun", "noun", "noun", "noun", "noun"]),
        (role: ["subject", "predicate", "ground", "consequence", "operation"]),
    )?;
    let ds = Dataset::named("feature-moment-frame", table.clone());
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("columns: {:?}", table.column_names());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 1: Feature construction (Projection role)
    // A Feature is a Plan-backed address. We build two: one for the token
    // column, one for the role column. Then we chain them.
    // -------------------------------------------------------------------
    stage(
        1,
        "Feature construction",
        "Feature::new(plan).named(...) — projection role: Plan-backed address into the Model.",
    );

    // Feature 1: select the token column
    let f_token = Feature::new(
        Plan::new(PlanSource::Value(ds.clone()))
            .named("token-projection")
            .push_step(Step::Select(vec![
                polars::prelude::col("term_id"),
                polars::prelude::col("token"),
            ])),
    )
    .named("token");

    // Feature 2: select the role column
    let f_role = Feature::new(
        Plan::new(PlanSource::Value(ds.clone()))
            .named("role-projection")
            .push_step(Step::Select(vec![
                polars::prelude::col("term_id"),
                polars::prelude::col("role"),
            ])),
    )
    .named("role");

    println!("f_token name: {:?}", f_token.name());
    println!("f_role  name: {:?}", f_role.name());
    let feat_path = fixture_root.join("01-features.txt");
    fs::write(
        &feat_path,
        format!(
            "f_token: {:?}\nf_role: {:?}\nrole: Projection (Plan-backed)\n",
            f_token.name(),
            f_role.name(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&feat_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 2: FeatureSpace
    // FeatureSpace is a named registry of Features sharing a schema.
    // Insert, iterate, and describe the space.
    // -------------------------------------------------------------------
    stage(
        2,
        "FeatureSpace",
        "FeatureSpace collects named features; supports insert, union, merge, keep.",
    );
    let space = FeatureSpace::new()
        .insert("token", f_token.clone())
        .insert("role", f_role.clone());
    println!("feature count: {}", space.len());
    println!(
        "features: {:?}",
        space.iter().map(|(n, _)| n).collect::<Vec<_>>()
    );
    let space_path = fixture_root.join("02-feature-space.txt");
    let space_lines = space
        .iter()
        .map(|(name, _feat)| format!("  {name}: Projection"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(
        &space_path,
        format!("FeatureSpace (count: {})\n{}\n", space.len(), space_lines),
    )?;
    println!("persisted: {}", fixture_path(&space_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 3: Feature chaining
    // chain(f1, f2) composes two feature pipelines by chaining their Plans.
    // The chained feature runs f_token's Plan steps then f_role's Plan steps.
    // This is the "feature-chain" idea: text → tokens → embeddings → …
    // -------------------------------------------------------------------
    stage(
        3,
        "Feature chaining",
        "f_token.chain(&f_role) composes two projections: the second reads the first's output.",
    );

    // To chain meaningfully, the second feature must read from a var that
    // binds the first feature's output. We demonstrate the shape of chaining here.
    let f_pos = Feature::new(
        Plan::new(Source::Value(ds.clone()))
            .named("pos-projection")
            .push_step(Step::Select(vec![
                polars::prelude::col("term_id"),
                polars::prelude::col("token"),
                polars::prelude::col("pos"),
            ]))
            .push_step(Step::WithColumns(vec![
                (polars::prelude::col("pos").alias("pos_tag")),
            ])),
    )
    .named("pos");

    // Chain: f_pos selects and enriches, then f_token narrows to token+id
    let f_chain = f_pos.chained(&f_token);
    println!("chained name: {:?}", f_chain.name());
    println!("chained plan steps: {}", f_chain.plan().steps().len());
    let chain_path = fixture_root.join("03-chain.txt");
    fs::write(
        &chain_path,
        format!(
            "chained: {:?}\nsteps: {}\nmethod: pos enriches → token narrows (chain composes)\n",
            f_chain.name(),
            f_chain.plan().steps().len(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&chain_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 4: attention_report
    // attention_report describes the plan structure: source, steps, planned columns.
    // This is the Feature's self-description before execution.
    // -------------------------------------------------------------------
    stage(
        4,
        "attention_report",
        "Feature.attention_report() describes plan steps and planned output columns.",
    );
    let env = PlanEnv::new().with_preview_rows(5);
    let report_token = f_token.attention_report(Some(&env), EvalMode::Preview);
    let report_role = f_role.attention_report(Some(&env), EvalMode::Preview);
    let report_chain = f_chain.attention_report(Some(&env), EvalMode::Preview);

    println!("token report steps: {}", report_token.steps.len());
    println!(
        "token planned cols: {:?}",
        report_token.planned_columns.as_deref().unwrap_or(&[])
    );
    println!("role  report steps: {}", report_role.steps.len());
    println!("chain report steps: {}", report_chain.steps.len());

    let report_path = fixture_root.join("04-attention-reports.txt");
    fs::write(
        &report_path,
        format!(
            "f_token report:\n  mode: {}\n  steps: {}\n  planned_cols: {:?}\n\n\
             f_role report:\n  mode: {}\n  steps: {}\n  planned_cols: {:?}\n\n\
             f_chain report:\n  mode: {}\n  steps: {}\n  planned_cols: {:?}\n",
            report_token.mode,
            report_token.steps.len(),
            report_token.planned_columns.as_deref().unwrap_or(&[]),
            report_role.mode,
            report_role.steps.len(),
            report_role.planned_columns.as_deref().unwrap_or(&[]),
            report_chain.mode,
            report_chain.steps.len(),
            report_chain.planned_columns.as_deref().unwrap_or(&[]),
        ),
    )?;
    println!("persisted: {}", fixture_path(&report_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 5: eval_dataset
    // Execute the feature pipeline and materialize a Dataset result.
    // -------------------------------------------------------------------
    stage(
        5,
        "eval_dataset",
        "Feature.eval_dataset(&env, EvalMode::Preview) materializes the projection.",
    );
    let result_token = f_token.eval_dataset(&env, EvalMode::Preview)?;
    let result_role = f_role.eval_dataset(&env, EvalMode::Preview)?;

    println!(
        "token result: {} x {}",
        result_token.row_count(),
        result_token.column_count()
    );
    println!("token cols:   {:?}", result_token.column_names());
    println!(
        "role  result: {} x {}",
        result_role.row_count(),
        result_role.column_count()
    );
    println!("role  cols:   {:?}", result_role.column_names());

    let token_result_path = fixture_root.join("05-eval-token.csv");
    result_token.to_csv(&token_result_path)?;
    let role_result_path = fixture_root.join("05-eval-role.csv");
    result_role.to_csv(&role_result_path)?;
    println!("persisted: {}", fixture_path(&token_result_path));
    println!("persisted: {}", fixture_path(&role_result_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 6: FeatureView
    // FeatureView is a schema selector: named access into a FeatureSpace
    // with an explicit view label.
    // -------------------------------------------------------------------
    stage(
        6,
        "FeatureView",
        "FeatureView(feature, view) — named schema view into the feature space.",
    );
    let token_view = FeatureView::new("token", "projection");
    let role_view = FeatureView::new("role", "projection");
    println!(
        "token view: feature={:?} view={:?}",
        token_view.feature(),
        token_view.view()
    );
    println!(
        "role  view: feature={:?} view={:?}",
        role_view.feature(),
        role_view.view()
    );
    let view_path = fixture_root.join("06-feature-views.txt");
    fs::write(
        &view_path,
        format!(
            "token_view: feature={} view={}\nrole_view:  feature={} view={}\n\
             note: FeatureView resolves via space.eval_view(feature, schema, view, env, mode)\n",
            token_view.feature(),
            token_view.view(),
            role_view.feature(),
            role_view.view(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&view_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Dataset Feature Moment Fixture\n\n\
             Namespace: dataset::feature (R5)\n\n\
             Files:\n\
             00-frame.csv                  — immediate DataFrame body\n\
             01-features.txt               — Feature construction (Projection role)\n\
             02-feature-space.txt          — FeatureSpace: named feature registry\n\
             03-chain.txt                  — Feature chaining (plan composition)\n\
             04-attention-reports.txt      — attention_report for token/role/chain\n\
             05-eval-token.csv             — token feature evaluated to Dataset\n\
             05-eval-role.csv              — role feature evaluated to Dataset\n\
             06-feature-views.txt          — FeatureView: named schema selectors\n\
             README.txt                    — this manifest\n\n\
             Feature roles demonstrated:\n\
               Projection — Plan-backed address; .new(plan), .named(), .chain(), .eval_dataset()\n\
               (Binder, Reentrancy, Annotation are doctrine commitments, not yet typed)\n",
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
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataset/dataset_feature_moment")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_feature_moment/{file_name}")
}
