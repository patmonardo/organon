//! Dataset Plan Moment: deferred Polars recipes in depth.
//!
//! Plan (R6) is the deferred form of Frame:Series — a lazily described transformation
//! staying in the Polars world. It is NOT FeaturePlan; it is the execution-ordering
//! that Model and Feature consume.
//!
//! Plan exposes:
//!   - Steps: Filter / Select / WithColumns / Item / Split / Batch
//!   - describe_steps() — human-readable step description
//!   - attention_report() — structured PlanAttentionReport with PlanStepReports
//!   - eval_dataset() — materialize the plan to a Dataset result
//!
//! Run with:
//!   cargo run -p gds --example dataset_plan_moment

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Plan Moment ==");
    println!("Plan (R6): deferred Polars recipe. Steps → describe → report → eval.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // -------------------------------------------------------------------
    // Stage 0: Frame Entry
    // -------------------------------------------------------------------
    stage(
        0,
        "Frame Entry",
        "Immediate body: terms with pos, role, score.",
    );
    let table = tbl_def!(
        (term_id: i64 => [1, 2, 3, 4, 5]),
        (token: ["concept", "judgment", "syllogism", "inference", "procedure"]),
        (pos: ["noun", "noun", "noun", "noun", "noun"]),
        (role: ["subject", "predicate", "ground", "consequence", "operation"]),
        (score: i64 => [10, 20, 30, 40, 50]),
    )?;
    let ds = Dataset::named("plan-moment-frame", table.clone());
    let frame_path = fixture_root.join("00-frame.csv");
    table.write_csv(&path_string(&frame_path))?;
    println!("shape: {} x {}", table.row_count(), table.column_count());
    println!("columns: {:?}", table.column_names());
    println!("persisted: {}", fixture_path(&frame_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 1: Plan construction — steps in order
    // Filter: keep rows where score >= 20
    // Select: choose term_id, token, role, score
    // WithColumns: add a label column
    // Item: project the canonical item artifact
    // -------------------------------------------------------------------
    stage(
        1,
        "Plan construction",
        "Plan::new(source) + step-by-step: Filter, Select, WithColumns, Item.",
    );
    let plan = Plan::new(PlanSource::Value(ds.clone()))
        .named("doctrine-plan")
        .push_step(Step::Filter(
            polars::prelude::col("score").gt_eq(polars::prelude::lit(20i64)),
        ))
        .push_step(Step::Select(vec![
            polars::prelude::col("term_id"),
            polars::prelude::col("token"),
            polars::prelude::col("role"),
            polars::prelude::col("score"),
        ]))
        .push_step(Step::WithColumns(vec![
            polars::prelude::lit("doctrine").alias("label")
        ]))
        .push_step(Step::Batch(100))
        .project_item(polars::prelude::col("token").alias("item"));

    println!("plan name:  {:?}", plan.name());
    println!("step count: {}", plan.steps().len());
    println!("has_item:   {}", plan.has_item());
    println!("ends_item:  {}", plan.ends_with_item());
    println!();

    // -------------------------------------------------------------------
    // Stage 2: describe_steps
    // Human-readable description of the plan's source and each step.
    // -------------------------------------------------------------------
    stage(
        2,
        "describe_steps",
        "Plan.describe_steps() gives a step-by-step textual trace.",
    );
    let description = plan.describe_steps();
    println!("{}", description);
    let desc_path = fixture_root.join("02-describe-steps.txt");
    fs::write(&desc_path, format!("{description}\n"))?;
    println!("persisted: {}", fixture_path(&desc_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 3: attention_report
    // PlanAttentionReport: plan name, mode, source, PlanStepReport per step,
    // planned_columns, batch_hint.
    // -------------------------------------------------------------------
    stage(
        3,
        "attention_report",
        "PlanAttentionReport: structured report of steps, planned columns, batch hint.",
    );
    let env = PlanEnv::new().with_preview_rows(10);
    let report = plan.attention_report(Some(&env), EvalMode::Preview);

    println!(
        "plan name:      {:?}",
        report.plan_name.as_deref().unwrap_or("<unnamed>")
    );
    println!("mode:           {}", report.mode);
    println!("source:         {}", report.source);
    println!("step count:     {}", report.steps.len());
    println!(
        "planned cols:   {:?}",
        report.planned_columns.as_deref().unwrap_or(&[])
    );
    println!("batch hint:     {:?}", report.batch_hint);
    println!();
    println!("steps:");
    for step in &report.steps {
        println!(
            "  [{}] {}  detail: {}",
            step.index,
            step.op,
            step.detail
                .as_ref()
                .map(|d| d.to_string())
                .unwrap_or_default()
        );
    }
    println!();

    let report_text = format!(
        "plan: {:?}\nmode: {}\nsource: {}\nsteps: {}\nplanned_cols: {:?}\nbatch_hint: {:?}\n\nsteps:\n{}",
        report.plan_name.as_deref().unwrap_or("<unnamed>"),
        report.mode,
        report.source,
        report.steps.len(),
        report.planned_columns.as_deref().unwrap_or(&[]),
        report.batch_hint,
        report
            .steps
            .iter()
            .map(|s| format!(
                "  [{}] {}  {}",
                s.index,
                s.op,
                s.detail.as_ref().map(|d| d.to_string()).unwrap_or_default()
            ))
            .collect::<Vec<_>>()
            .join("\n"),
    );
    let report_path = fixture_root.join("03-attention-report.txt");
    fs::write(&report_path, format!("{report_text}\n"))?;
    println!("persisted: {}", fixture_path(&report_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 4: eval_dataset — Preview mode
    // Execute the plan and materialize to a Dataset.
    // -------------------------------------------------------------------
    stage(
        4,
        "eval_dataset (Preview)",
        "Plan.eval_dataset(&env, EvalMode::Preview) executes steps and returns a Dataset.",
    );
    let result = plan.eval_dataset(&env, EvalMode::Preview)?;
    println!(
        "result shape: {} x {}",
        result.row_count(),
        result.column_count()
    );
    println!("result cols:  {:?}", result.column_names());
    let result_path = fixture_root.join("04-eval-preview.csv");
    result.to_csv(&result_path)?;
    println!("persisted: {}", fixture_path(&result_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 5: Fit mode comparison
    // Fit mode processes the full dataset (no head limit).
    // -------------------------------------------------------------------
    stage(
        5,
        "eval_dataset (Fit)",
        "EvalMode::Fit processes the full dataset without a row limit.",
    );
    let env_fit = PlanEnv::new();
    let result_fit = plan.eval_dataset(&env_fit, EvalMode::Fit)?;
    println!(
        "fit shape: {} x {}",
        result_fit.row_count(),
        result_fit.column_count()
    );
    println!("fit cols:  {:?}", result_fit.column_names());
    let fit_path = fixture_root.join("05-eval-fit.csv");
    result_fit.to_csv(&fit_path)?;
    println!("persisted: {}", fixture_path(&fit_path));
    println!();

    // -------------------------------------------------------------------
    // Stage 6: Minimal plan — PlanSource::Var with PlanEnv binding
    // Plans can also take a Var source resolved at eval time via PlanEnv.
    // -------------------------------------------------------------------
    stage(
        6,
        "Source::Var + PlanEnv binding",
        "Plan(PlanSource::Var) resolves at eval time; PlanEnv binds the dataset by name.",
    );
    let plan_var = Plan::new(PlanSource::Var("terms".to_string()))
        .named("var-plan")
        .push_step(Step::Select(vec![
            polars::prelude::col("token"),
            polars::prelude::col("score"),
        ]));

    let env_bound = PlanEnv::new().bind_dataset("terms", ds.clone());
    let result_var = plan_var.eval_dataset(&env_bound, EvalMode::Preview)?;
    println!(
        "var result: {} x {}",
        result_var.row_count(),
        result_var.column_count()
    );
    println!("var cols:   {:?}", result_var.column_names());

    let var_report = plan_var.attention_report(Some(&env_bound), EvalMode::Preview);
    println!("source:     {}", var_report.source);
    let var_path = fixture_root.join("06-var-plan.txt");
    fs::write(
        &var_path,
        format!(
            "plan: {:?}\nsource: {}\nrows: {}\ncols: {:?}\n",
            plan_var.name(),
            var_report.source,
            result_var.row_count(),
            result_var.column_names(),
        ),
    )?;
    println!("persisted: {}", fixture_path(&var_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        format!(
            "Dataset Plan Moment Fixture\n\n\
             Namespace: dataset::plan (R6)\n\n\
             Files:\n\
             00-frame.csv                  — immediate DataFrame body\n\
             02-describe-steps.txt         — Plan.describe_steps() trace\n\
             03-attention-report.txt       — PlanAttentionReport (steps + planned cols)\n\
             04-eval-preview.csv           — eval_dataset(Preview): filtered + enriched rows\n\
             05-eval-fit.csv               — eval_dataset(Fit): full dataset\n\
             06-var-plan.txt               — Source::Var + PlanEnv binding\n\
             README.txt                    — this manifest\n\n\
             Steps demonstrated:\n\
               Filter        — keep rows where score >= 20\n\
               Select        — choose columns\n\
               WithColumns   — add label column\n\
               Batch         — hint for streaming evaluation\n\
               Item          — canonical item projection (Struct)\n",
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
        .join("fixtures/collections/dataset/dataset_plan_moment")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset/dataset_plan_moment/{file_name}")
}
