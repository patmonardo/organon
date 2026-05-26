use gds::ml::workbench::models::*;
use serde_json::Value;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "help".to_string());
    let verbose = args.any(|arg| arg == "--verbose" || arg == "-v");

    match command.as_str() {
        "list" => print_json(available_experiments()),
        "fixtures" => print_json(fixture_catalog()),
        "logistic-demo" => print_json(run_logistic_demo()),
        "logistic-three-class-demo" => print_json(run_logistic_three_class_demo()),
        "logistic-three-class-overlap-demo" => print_json(run_logistic_three_class_overlap_demo()),
        "logistic-three-class-large-demo" => print_json(run_logistic_three_class_large_demo()),
        "mlp-demo" => print_json(run_mlp_demo()),
        "mlp-three-class-demo" => print_json(run_mlp_three_class_demo()),
        "mlp-three-class-overlap-demo" => print_json(run_mlp_three_class_overlap_demo()),
        "mlp-three-class-large-demo" => print_json(run_mlp_three_class_large_demo()),
        "logistic-vs-mlp-overlap-comparison" => {
            print_json(run_logistic_vs_mlp_overlap_comparison())
        }
        "mlp-three-class-sweep" => print_json(run_mlp_three_class_sweep()),
        "linear-demo" => print_json(run_linear_demo()),
        "logistic-sweep" => print_json(run_logistic_sweep()),
        "linear-sweep" => print_json(run_linear_sweep()),
        "node-classification-preview" => print_json(run_node_classification_preview()),
        "node-classification-metrics" => print_json(run_node_classification_metrics()),
        "node-prediction-split-preview" => print_json(run_node_prediction_split_preview()),
        "node-regression-preview" => print_json(run_node_regression_preview()),
        "node-regression-metrics" => print_json(run_node_regression_metrics()),
        "large-logistic-benchmark" => {
            print_json(run_large_logistic_benchmark_with_verbose(verbose))
        }
        "large-linear-benchmark" => print_json(run_large_linear_benchmark_with_verbose(verbose)),
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
    let json_value = serde_json::to_value(value).expect("workbench output should serialize");
    let mut out = String::new();
    write_pretty_json_no_scientific(&json_value, 0, &mut out);
    println!("{out}");
}

fn write_pretty_json_no_scientific(value: &Value, indent: usize, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(boolean) => out.push_str(if *boolean { "true" } else { "false" }),
        Value::Number(number) => out.push_str(&format_json_number(number)),
        Value::String(string) => {
            let encoded =
                serde_json::to_string(string).expect("string encoding for JSON should succeed");
            out.push_str(&encoded);
        }
        Value::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }

            out.push('[');
            out.push('\n');
            for (index, item) in items.iter().enumerate() {
                out.push_str(&"  ".repeat(indent + 1));
                write_pretty_json_no_scientific(item, indent + 1, out);
                if index + 1 != items.len() {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&"  ".repeat(indent));
            out.push(']');
        }
        Value::Object(map) => {
            if map.is_empty() {
                out.push_str("{}");
                return;
            }

            out.push('{');
            out.push('\n');
            let len = map.len();
            for (index, (key, item)) in map.iter().enumerate() {
                out.push_str(&"  ".repeat(indent + 1));
                let encoded_key = serde_json::to_string(key)
                    .expect("object key encoding for JSON should succeed");
                out.push_str(&encoded_key);
                out.push_str(": ");
                write_pretty_json_no_scientific(item, indent + 1, out);
                if index + 1 != len {
                    out.push(',');
                }
                out.push('\n');
            }
            out.push_str(&"  ".repeat(indent));
            out.push('}');
        }
    }
}

fn format_json_number(number: &serde_json::Number) -> String {
    if number.is_i64() || number.is_u64() {
        return number.to_string();
    }

    let value = number
        .as_f64()
        .expect("JSON float should always be representable as f64");
    if !value.is_finite() {
        return "null".to_string();
    }

    let mut fixed = format!("{value:.12}");
    while fixed.ends_with('0') {
        fixed.pop();
    }
    if fixed.ends_with('.') {
        fixed.push('0');
    }
    if fixed == "-0.0" || fixed == "-0" {
        return "0.0".to_string();
    }
    if fixed.is_empty() {
        return "0.0".to_string();
    }

    fixed
}

fn print_help() {
    println!("ML Models Workbench");
    println!();
    println!("Usage:");
    println!("  cargo run -p gds --bin ml_models_cli -- <command> [--verbose|-v]");
    println!();
    println!("Commands:");
    println!("  list           Show available experiments");
    println!("  fixtures       Show shared ML model fixtures");
    println!("  logistic-demo  Train/evaluate a dense logistic regression fixture");
    println!("  logistic-three-class-demo Train/evaluate a 3-class logistic fixture");
    println!(
        "  logistic-three-class-overlap-demo Train/evaluate a harder 3-class logistic fixture"
    );
    println!(
        "  logistic-three-class-large-demo Train/evaluate a larger noisy 3-class logistic fixture"
    );
    println!("  mlp-demo       Train/evaluate an MLP classifier on binary fixture");
    println!("  mlp-three-class-demo Train/evaluate an MLP classifier on 3-class fixture");
    println!(
        "  mlp-three-class-overlap-demo Train/evaluate an MLP classifier on harder 3-class fixture"
    );
    println!(
        "  mlp-three-class-large-demo Train/evaluate an MLP classifier on larger noisy 3-class fixture"
    );
    println!(
        "  logistic-vs-mlp-overlap-comparison Compare logistic and MLP side-by-side on harder 3-class fixture"
    );
    println!("  mlp-three-class-sweep Compare MLP learning settings on 3-class fixture");
    println!("  linear-demo    Train/evaluate a dense linear regression fixture");
    println!("  logistic-sweep Compare logistic regression learning settings");
    println!("  linear-sweep   Compare linear regression learning settings");
    println!("  node-classification-preview  Run node classification preview over model fixtures");
    println!("  node-classification-metrics  Run node classification metric suite");
    println!("  node-prediction-split-preview Run node prediction split preview over fixture ids");
    println!("  node-regression-preview      Run node regression predictor over model fixtures");
    println!("  node-regression-metrics      Run node regression metric suite");
    println!("  large-logistic-benchmark     Benchmark logistic regression on a 1k fixture (supports --verbose)");
    println!("  large-linear-benchmark       Benchmark linear regression on a 1k fixture (supports --verbose)");
}
