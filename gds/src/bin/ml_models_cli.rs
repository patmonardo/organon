use gds::ml::models::workbench;

fn main() {
    let command = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "help".to_string());

    match command.as_str() {
        "list" => print_json(workbench::available_experiments()),
        "logistic-demo" => print_json(&workbench::run_logistic_demo()),
        "linear-demo" => print_json(&workbench::run_linear_demo()),
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
    println!("  logistic-demo  Train/evaluate a dense logistic regression fixture");
    println!("  linear-demo    Train/evaluate a dense linear regression fixture");
}
