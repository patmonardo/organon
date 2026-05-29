use gds::ml::workbench::models::*;
use serde_json::Value;
use std::collections::BTreeSet;
use std::io::{self, Write};

#[derive(Clone, Copy)]
struct ModelWorkbenchCommand {
    id: &'static str,
    group: &'static str,
    title: &'static str,
}

const MODEL_COMMANDS: &[ModelWorkbenchCommand] = &[
    ModelWorkbenchCommand {
        id: "fixtures",
        group: "Catalog",
        title: "Show shared fixtures",
    },
    ModelWorkbenchCommand {
        id: "logistic-demo",
        group: "Classification",
        title: "Binary logistic baseline",
    },
    ModelWorkbenchCommand {
        id: "logistic-three-class-demo",
        group: "Classification",
        title: "Three-class logistic",
    },
    ModelWorkbenchCommand {
        id: "logistic-three-class-overlap-demo",
        group: "Classification",
        title: "Three-class overlap logistic",
    },
    ModelWorkbenchCommand {
        id: "logistic-three-class-large-demo",
        group: "Classification",
        title: "Large noisy three-class logistic",
    },
    ModelWorkbenchCommand {
        id: "mlp-demo",
        group: "Classification",
        title: "MLP binary classifier",
    },
    ModelWorkbenchCommand {
        id: "mlp-three-class-demo",
        group: "Classification",
        title: "MLP three-class",
    },
    ModelWorkbenchCommand {
        id: "mlp-three-class-overlap-demo",
        group: "Classification",
        title: "MLP three-class overlap",
    },
    ModelWorkbenchCommand {
        id: "mlp-three-class-large-demo",
        group: "Classification",
        title: "MLP three-class large",
    },
    ModelWorkbenchCommand {
        id: "logistic-vs-mlp-overlap-comparison",
        group: "Comparisons",
        title: "Logistic vs MLP overlap",
    },
    ModelWorkbenchCommand {
        id: "tree-vs-random-forest-demo",
        group: "Comparisons",
        title: "Tree vs Random Forest",
    },
    ModelWorkbenchCommand {
        id: "svm-demo",
        group: "SVM",
        title: "SVM demo",
    },
    ModelWorkbenchCommand {
        id: "svm-linear-vs-rbf-comparison",
        group: "SVM",
        title: "Linear vs RBF SVM",
    },
    ModelWorkbenchCommand {
        id: "svm-nonlinear-comparison",
        group: "SVM",
        title: "Nonlinear XOR SVM comparison",
    },
    ModelWorkbenchCommand {
        id: "svm-rbf-sweep",
        group: "SVM",
        title: "RBF sweep",
    },
    ModelWorkbenchCommand {
        id: "mlp-three-class-sweep",
        group: "Sweeps",
        title: "MLP three-class sweep",
    },
    ModelWorkbenchCommand {
        id: "logistic-sweep",
        group: "Sweeps",
        title: "Logistic sweep",
    },
    ModelWorkbenchCommand {
        id: "linear-sweep",
        group: "Sweeps",
        title: "Linear sweep",
    },
    ModelWorkbenchCommand {
        id: "linear-demo",
        group: "Regression",
        title: "Linear regression demo",
    },
    ModelWorkbenchCommand {
        id: "node-classification-preview",
        group: "Node Tasks",
        title: "Node classification preview",
    },
    ModelWorkbenchCommand {
        id: "node-classification-metrics",
        group: "Node Tasks",
        title: "Node classification metrics",
    },
    ModelWorkbenchCommand {
        id: "node-prediction-split-preview",
        group: "Node Tasks",
        title: "Node prediction split preview",
    },
    ModelWorkbenchCommand {
        id: "node-regression-preview",
        group: "Node Tasks",
        title: "Node regression preview",
    },
    ModelWorkbenchCommand {
        id: "node-regression-metrics",
        group: "Node Tasks",
        title: "Node regression metrics",
    },
    ModelWorkbenchCommand {
        id: "large-logistic-benchmark",
        group: "Benchmarks",
        title: "Large logistic benchmark",
    },
    ModelWorkbenchCommand {
        id: "large-linear-benchmark",
        group: "Benchmarks",
        title: "Large linear benchmark",
    },
];

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "menu".to_string());
    let verbose = args.any(|arg| arg == "--verbose" || arg == "-v");

    match command.as_str() {
        "menu" => run_interactive_menu(verbose),
        "run" => {
            if let Some(selection) = std::env::args().nth(2) {
                run_selection_expr(&selection, verbose);
            } else {
                eprintln!("missing selection");
                print_help();
                std::process::exit(2);
            }
        }
        "help" | "--help" | "-h" => print_help(),
        other => {
            if !dispatch_command(other, verbose) {
                eprintln!("unknown command: {other}");
                print_help();
                std::process::exit(2);
            }
        }
    }
}

fn run_interactive_menu(verbose: bool) {
    loop {
        println!("ML Models Workbench");
        println!();
        print_command_menu();
        println!();
        println!("Select number/id, batch (1,3-5), or ids (a,b). Commands: l=list, q=quit");
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("failed to read selection");
            std::process::exit(2);
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("q") || input.eq_ignore_ascii_case("quit") {
            println!("bye");
            break;
        }

        if input.eq_ignore_ascii_case("l") || input.eq_ignore_ascii_case("list") {
            continue;
        }

        run_selection_expr(input, verbose);
    }
}

fn print_command_menu() {
    println!("Commands:");
    for (index, cmd) in MODEL_COMMANDS.iter().enumerate() {
        println!("  {}. {} [{}]", index + 1, cmd.title, cmd.id);
        println!("     group: {}", cmd.group);
    }
}

fn run_selection_expr(selection: &str, verbose: bool) {
    if let Some(indices) = parse_index_selection(selection, MODEL_COMMANDS.len()) {
        for index in indices {
            run_command(MODEL_COMMANDS[index - 1].id, verbose);
        }
        return;
    }

    for token in split_tokens(selection) {
        run_command(token, verbose);
    }
}

fn run_command(command: &str, verbose: bool) {
    println!();
    println!("running: {command}");
    if !dispatch_command(command, verbose) {
        eprintln!("unknown command: {command}");
    }
    println!();
}

fn dispatch_command(command: &str, verbose: bool) -> bool {
    match command {
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
        "tree-vs-random-forest-demo" => print_json(run_tree_vs_random_forest_demo()),
        "svm-demo" => print_json(run_svm_demo()),
        "svm-linear-vs-rbf-comparison" => print_json(run_svm_linear_vs_rbf_comparison()),
        "svm-nonlinear-comparison" => print_json(run_svm_nonlinear_comparison()),
        "svm-rbf-sweep" => print_json(run_svm_rbf_sweep()),
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
        _ => return false,
    }

    true
}

fn split_tokens(input: &str) -> Vec<&str> {
    input
        .split(',')
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .collect()
}

fn parse_index_selection(input: &str, max: usize) -> Option<Vec<usize>> {
    let tokens = split_tokens(input);
    if tokens.is_empty() {
        return None;
    }

    let mut selected = BTreeSet::new();
    for token in tokens {
        if let Some((left, right)) = token.split_once('-') {
            let start = left.trim().parse::<usize>().ok()?;
            let end = right.trim().parse::<usize>().ok()?;
            if start == 0 || end == 0 || start > max || end > max {
                return None;
            }
            let (lo, hi) = if start <= end { (start, end) } else { (end, start) };
            for value in lo..=hi {
                selected.insert(value);
            }
        } else {
            let index = token.parse::<usize>().ok()?;
            if index == 0 || index > max {
                return None;
            }
            selected.insert(index);
        }
    }
    Some(selected.into_iter().collect())
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
    println!("  cargo run -p gds --bin ml_models_cli -- [menu|run <selection>|<command>] [--verbose|-v]");
    println!();
    println!("Commands:");
    println!("  menu           Interactive command selection");
    println!("  run <selection> Run by number/id/list/range (e.g., 1,3-4)");
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
    println!(
        "  tree-vs-random-forest-demo Compare single-tree baseline and Random Forest on binary fixture"
    );
    println!("  svm-demo       Train/evaluate an SVM classifier on harder 3-class fixture");
    println!(
        "  svm-linear-vs-rbf-comparison Compare linear and RBF SVM kernels on a held-out split"
    );
    println!(
        "  svm-nonlinear-comparison Compare linear and RBF SVM kernels on nonlinear XOR fixture"
    );
    println!("  svm-rbf-sweep  Sweep RBF gamma/C settings on a held-out split");
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
