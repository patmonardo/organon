//! Dataset Corpus:LM::SemDataset doctrinal fixture.
//!
//! Run with:
//!   cargo run -p gds --example dataset_sem_meta_pipeline

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataset::prelude::*;
use gds::form::ProgramFeatureKind;
use gds::shell::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Dataset Corpus:LM::SemDataset ==");
    println!("Doctrine exemplar 026: SemDataset as the Concept return end-view.");
    println!("This example shows the Concept fold that answers Model:Feature::Plan.");
    println!();

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Corpus",
        "Corpus is extensional evidence: documents, sources, and observed text.",
    );
    let corpus_frame = tbl_def!(
        (doc_id: i64 => [1, 2, 3]),
        (text: [
            "Socrates is human.",
            "All humans are mortal.",
            "Socrates is mortal."
        ]),
    )?;
    let corpus_path = fixture_root.join("00-corpus.csv");
    corpus_frame.write_csv(&path_string(&corpus_path))?;
    let texts = [
        "Socrates is human.",
        "All humans are mortal.",
        "Socrates is mortal.",
    ];
    let corpus = Corpus::from_texts(&texts)?;
    println!("artifact: Corpus");
    println!("documents: {}", corpus.document_count());
    println!("persisted: {}", fixture_path(&corpus_path));
    println!();

    stage(
        1,
        "LM",
        "LM is the fitted intensional application over Corpus evidence.",
    );
    let program = program_features(
        "rustscript.dataset.sem_meta_pipeline",
        ["CorpusLmSemDataset"],
        [
            program_source(
                "corpus",
                "fixtures/collections/dataset_sem_meta_pipeline/00-corpus.csv",
            ),
            program_subfeature("tokenizer"),
            program_subfeature("language-model"),
            program_concept("CorpusLmSemDataset"),
            program_principle("concept-return-requires-corpus-and-lm"),
            program_feature(
                ProgramFeatureKind::Judgment,
                "all x. (human(x) -> mortal(x))",
                "doctrine://dataset-sem-meta-pipeline/judgment",
            ),
            program_feature(
                ProgramFeatureKind::Inference,
                "human(socrates) -> mortal(socrates)",
                "doctrine://dataset-sem-meta-pipeline/inference",
            ),
            program_procedure("persist-semdataset-artifacts"),
        ],
    );
    let mut sem = SemDataset::fit(corpus, MLE::new(2), &WhitespaceTokenizer)?;
    sem.ingest_forms(program.features.clone());
    let parsed = sem.parse_forms();
    let lm_path = fixture_root.join("01-lm-report.txt");
    fs::write(
        &lm_path,
        format!(
            "lm_order: {}\nvocab_size: {}\nforms: {}\nparsed: {}\n",
            sem.lm().order(),
            sem.lm().vocab().len(),
            sem.forms().len(),
            parsed,
        ),
    )?;
    println!("artifact: MLE + WhitespaceTokenizer");
    println!("lm order: {}", sem.lm().order());
    println!("vocab size: {}", sem.lm().vocab().len());
    println!("persisted: {}", fixture_path(&lm_path));
    println!();

    stage(
        2,
        "SemDataset",
        "SemDataset carries Corpus, LM application, and SemForm parse state together.",
    );
    let forms_path = fixture_root.join("02-semforms.txt");
    fs::write(&forms_path, semform_report(&sem))?;
    println!("artifact: SemDataset");
    println!("forms: {}", sem.forms().len());
    println!("parsed forms: {}", parsed);
    println!("persisted: {}", fixture_path(&forms_path));
    println!();

    stage(
        3,
        "Shell Return",
        "Shell verifies that the Concept fold can be carried as a pipeline address.",
    );
    let shell_frame = tbl_def!(
        (moment_id: i64 => [1, 2, 3]),
        (moment: ["corpus", "lm", "semdataset"]),
        (role: ["evidence", "application", "concept-return"]),
    )?;
    let shell = ds_frame(shell_frame)
        .named("dataset-sem-meta-pipeline")
        .artifact_kind(DatasetArtifactKind::ProgramImage)
        .facet("doctrine:026-semantic-meta-pipeline")
        .source_io(io_path(
            "fixtures/collections/dataset_sem_meta_pipeline/00-corpus.csv",
        ))
        .into_shell_with_program_features(program)
        .materialize_semdataset_from_texts(&texts)?;
    let semantic = shell.semantic_pipeline_knowledge();
    let learning = shell.learning_report();
    let trace = shell.validate_projection_trace();
    let shell_path = fixture_root.join("03-shell-return.txt");
    fs::write(
        &shell_path,
        format!(
            "semdataset_ready: {}\npureform_return_ready: {}\nreadiness: {}\ntrace_valid: {}\n",
            semantic.semdataset_ready(),
            semantic.pureform_return_ready(),
            learning.readiness_score(),
            trace.is_valid(),
        ),
    )?;
    println!("semdataset ready: {}", semantic.semdataset_ready());
    println!(
        "pureform return ready: {}",
        semantic.pureform_return_ready()
    );
    println!("trace valid: {}", trace.is_valid());
    println!("persisted: {}", fixture_path(&shell_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&corpus_path, &lm_path, &forms_path, &shell_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures/collections/dataset_sem_meta_pipeline")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataset_sem_meta_pipeline/{file_name}")
}

fn semform_report(sem: &SemDataset<MLE>) -> String {
    let mut report = String::new();
    for form in sem.forms() {
        let status = if form.parsed() { "parsed" } else { "error" };
        report.push_str(&format!(
            "{} {} [{}]\n",
            form.kind.as_str(),
            status,
            form.source
        ));
        if let Some(expr) = &form.expr {
            report.push_str(&format!("  expr: {expr}\n"));
        }
        if let Some(error) = &form.error {
            report.push_str(&format!("  error: {error}\n"));
        }
    }
    report
}

fn manifest(corpus_path: &Path, lm_path: &Path, forms_path: &Path, shell_path: &Path) -> String {
    format!(
        "Dataset Corpus:LM::SemDataset Fixture\n\n\
         Doctrine: EXEMPLARS/026-semantic-meta-pipeline.md\n\n\
         00 Corpus\n\
         artifact: {}\n\
         meaning: extensional evidence.\n\n\
         01 LM\n\
         artifact: {}\n\
         meaning: fitted intensional application over Corpus.\n\n\
         02 SemDataset\n\
         artifact: {}\n\
         meaning: Corpus, LM, and SemForm state held together.\n\n\
         03 Shell Return\n\
         artifact: {}\n\
         meaning: Shell-readable Concept return witness.\n",
        fixture_path(corpus_path),
        fixture_path(lm_path),
        fixture_path(forms_path),
        fixture_path(shell_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
