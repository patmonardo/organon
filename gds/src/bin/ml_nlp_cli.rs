use gds::ml::workbench::nlp::available_experiments;
use gds::ml::workbench::nlp::run_classify_preview;
use gds::ml::workbench::nlp::run_featstruct_preview;
use gds::ml::workbench::nlp::run_inference_preview;
use gds::ml::workbench::nlp::run_logic_preview;
use gds::ml::workbench::nlp::run_logic_processor_preview;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "list" => print_json(available_experiments()),
        "logic-preview" => print_json(run_logic_preview()),
        "inference-preview" => print_json(run_inference_preview()),
        "featstruct-preview" => print_json(run_featstruct_preview()),
        "classify-preview" => print_json(run_classify_preview()),
        "logic-processor-preview" => print_json(run_logic_processor_preview()),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_json<T>(value: T)
where
    T: serde::Serialize,
{
    let json_value = serde_json::to_value(value).expect("nlp workbench output should serialize");
    let json =
        serde_json::to_string_pretty(&json_value).expect("nlp workbench output should serialize");
    println!("{json}");
}

fn print_help() {
    println!("NLP Semantic Workbench");
    println!();
    println!("Usage:");
    println!("  cargo run -p gds --bin ml_nlp_cli -- <command>");
    println!();
    println!("Commands:");
    println!("  list              Show available NLP workbench experiments");
    println!("  logic-preview     Parse, evaluate and skolemize first-order logic");
    println!("  inference-preview Resolution proof of classic syllogisms");
    println!("  featstruct-preview Feature structure unification and subsumption algebra");
    println!("  classify-preview  Naive Bayes + Decision Tree over NLP FeatureSets");
    println!("  logic-processor-preview FeatStruct semantics into entailment-style classify");
}
