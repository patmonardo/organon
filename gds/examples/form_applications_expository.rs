//! Form Applications Expository fixture.
//!
//! Exposes ProgramSpec application-form compilation and evaluator pre-eval
//! given-form traces.
//!
//! Run with:
//!   cargo run -p gds --example form_applications_expository

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use gds::form::{ApplicationForm, Context, FormShape, Morph, ProgramSpec, Shape, Specification};
use gds::projection::eval::{FormEvalRequest, FormEvaluator};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Form Applications Expository ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Program Spec",
        "Assemble a program with two application forms and one selected form.",
    );

    let program = sample_program();
    let spec_path = fixture_root.join("00-program-spec.txt");
    fs::write(
        &spec_path,
        format!(
            "gdsl={}\napplication_forms={:?}\nselected_forms={:?}\n",
            program.gdsl.name,
            program
                .application_forms
                .iter()
                .map(|f| f.name.clone())
                .collect::<Vec<_>>(),
            program.selected_forms,
        ),
    )?;
    println!("persisted: {}", fixture_path(&spec_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Execution Plan",
        "Compile selected application forms into ordered operator patterns.",
    );

    let plan = program.compile_execution_plan()?;
    let features = program.define_features()?;

    let plan_path = fixture_root.join("01-plan.txt");
    fs::write(
        &plan_path,
        format!(
            "selected_forms={:?}\npatterns={:?}\nfeature_count={}\n",
            plan.selected_forms,
            plan.patterns,
            features.features.len(),
        ),
    )?;
    println!("patterns: {:?}", plan.patterns);
    println!("persisted: {}", fixture_path(&plan_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "Given Form",
        "Build Given-Form decomposition from selected application forms.",
    );

    let given_forms = program.given_forms(Some("graph=organon".to_string()))?;
    let given = &given_forms[0];
    let perfected = given.clone().into_perfected();

    let given_path = fixture_root.join("02-given-form.txt");
    fs::write(
        &given_path,
        format!(
            "stage_order={:?}\nentity={}\nproperty={}\naspect={}\n",
            gds::form::GivenFormEnvelope::stage_order(),
            perfected.entity,
            perfected.property,
            perfected.aspect,
        ),
    )?;
    println!("given forms: {}", given_forms.len());
    println!("persisted: {}", fixture_path(&given_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Evaluator Trace",
        "Run FormEvaluator and persist selected forms, patterns, and pre-eval count.",
    );

    let evaluator = FormEvaluator::new();
    let eval = evaluator.evaluate(FormEvalRequest::new(program))?;

    let eval_path = fixture_root.join("03-eval.txt");
    fs::write(
        &eval_path,
        format!(
            "selected_forms={:?}\npatterns={:?}\npre_eval_given_forms={}\n",
            eval.plan.selected_forms,
            eval.plan.patterns,
            eval.pre_eval.given_forms.len(),
        ),
    )?;
    println!("pre-eval given forms: {}", eval.pre_eval.given_forms.len());
    println!("persisted: {}", fixture_path(&eval_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&spec_path, &plan_path, &given_path, &eval_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn sample_program() -> ProgramSpec {
    ProgramSpec {
        form: FormShape::new(
            Shape::default(),
            Context::new(vec![], vec![], "kernel".to_string(), vec![]),
            Morph::new(vec!["base.normalize".to_string()]),
        ),
        gdsl: Specification {
            name: "gdsl.applications.expository".to_string(),
            version: Some("0.1.0".to_string()),
            attributes: HashMap::new(),
        },
        sdsl: vec![],
        application_forms: vec![
            ApplicationForm {
                name: "centrality".to_string(),
                domain: "graph-ml".to_string(),
                features: vec!["feature.centrality".to_string()],
                patterns: vec!["algo.pagerank".to_string()],
                specifications: HashMap::new(),
            },
            ApplicationForm {
                name: "community".to_string(),
                domain: "graph-ml".to_string(),
                features: vec!["feature.community".to_string()],
                patterns: vec!["algo.louvain".to_string()],
                specifications: HashMap::new(),
            },
        ],
        selected_forms: vec!["centrality".to_string()],
    }
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/form/form_applications_expository")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/form/form_applications_expository/{file_name}")
}

fn manifest(spec_path: &Path, plan_path: &Path, given_path: &Path, eval_path: &Path) -> String {
    format!(
        "Form Applications Expository Fixture\n\n\
         Namespace: form::applications\n\n\
         00 Program Spec\n\
         artifact: {}\n\
         meaning: declared application forms and selected form set.\n\n\
         01 Plan\n\
         artifact: {}\n\
         meaning: selected forms compiled into operator-pattern chain.\n\n\
         02 Given Form\n\
         artifact: {}\n\
         meaning: principled-effect decomposition witness.\n\n\
         03 Eval\n\
         artifact: {}\n\
         meaning: evaluator pre-eval trace over selected forms.\n",
        fixture_path(spec_path),
        fixture_path(plan_path),
        fixture_path(given_path),
        fixture_path(eval_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
