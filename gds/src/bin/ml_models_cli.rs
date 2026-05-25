use gds::ml::workbench;

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "list" => print_json(workbench::available_experiments()),
        "fixtures" => print_json(workbench::fixture_catalog()),
        "logistic-demo" => print_json(&workbench::run_logistic_demo()),
        "linear-demo" => print_json(&workbench::run_linear_demo()),
        "logistic-sweep" => print_json(&workbench::run_logistic_sweep()),
        "linear-sweep" => print_json(&workbench::run_linear_sweep()),
        "node-classification-preview" => print_json(&workbench::run_node_classification_preview()),
        "node-prediction-split-preview" => {
            print_json(&workbench::run_node_prediction_split_preview())
        }
        "node-regression-preview" => print_json(&workbench::run_node_regression_preview()),
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn print_json<T>(value: &T)
where
    T: serde::Serialize + ?Sized,
{
    let json = serde_json::to_string_pretty(value).expect("workbench output should serialize");
    println!("{json}");
}

fn print_help() {
    println!("ML Models Workbench");
    println!();
    println!("Usage:");
    println!("  cargo run -p gds --bin ml_models_cli -- <command>");
    println!();
    println!("Commands:");
    println!("  list           Show available experiments");
    println!("  fixtures       Show shared ML model fixtures");
    println!("  logistic-demo  Train/evaluate a dense logistic regression fixture");
    println!("  linear-demo    Train/evaluate a dense linear regression fixture");
    println!("  logistic-sweep Compare logistic regression learning settings");
    println!("  linear-sweep   Compare linear regression learning settings");
    println!("  node-classification-preview  Run node classification preview over model fixtures");
    println!("  node-prediction-split-preview Run node prediction split preview over fixture ids");
    println!("  node-regression-preview      Run node regression predictor over model fixtures");
}
