use gds::ml::workbench::nlp::available_experiments;
use gds::ml::workbench::nlp::run_classify_preview;
use gds::ml::workbench::nlp::run_cluster_preview;
use gds::ml::workbench::nlp::run_decision_tree_experiment;
use gds::ml::workbench::nlp::run_featstruct_preview;
use gds::ml::workbench::nlp::run_inference_preview;
use gds::ml::workbench::nlp::run_logic_preview;
use gds::ml::workbench::nlp::run_logic_processor_preview;
use gds::ml::workbench::nlp::NlpWorkbenchExperiment;
use std::collections::BTreeSet;
use std::io::{self, Write};

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "menu".to_string());

    match command.as_str() {
        "menu" => run_interactive_menu(),
        "list" => print_json(available_experiments()),
        "logic-preview" => print_json(run_logic_preview()),
        "inference-preview" => print_json(run_inference_preview()),
        "featstruct-preview" => print_json(run_featstruct_preview()),
        "classify-preview" => print_json(run_classify_preview()),
        "cluster-preview" => print_json(run_cluster_preview()),
        "decision-tree-experiment" => print_json(run_decision_tree_experiment()),
        "logic-processor-preview" => print_json(run_logic_processor_preview()),
        "run" => {
            if let Some(selection) = args.next() {
                run_selection_expr(&selection);
            } else {
                eprintln!("missing selection");
                print_help();
                std::process::exit(2);
            }
        }
        "help" | "--help" | "-h" => print_help(),
        other => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn run_interactive_menu() {
    loop {
        println!("NLP Semantic Workbench");
        println!();
        print_experiment_menu();
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

        run_selection_expr(input);
    }
}

fn print_experiment_menu() {
    println!("Experiments:");
    for (index, exp) in available_experiments().iter().enumerate() {
        println!("  {}. {} [{}]", index + 1, exp.name, exp.id);
        println!("     focus: {}", exp.focus);
    }
}

fn run_selection_expr(selection: &str) {
    let experiments = available_experiments();
    if let Some(indices) = parse_index_selection(selection, experiments.len()) {
        for index in indices {
            run_experiment(&experiments[index - 1]);
        }
        return;
    }

    for token in split_tokens(selection) {
        if let Some(exp) = experiments.iter().find(|exp| exp.id == token) {
            run_experiment(exp);
        } else {
            eprintln!("unknown experiment id: {token}");
        }
    }
}

fn run_experiment(exp: &NlpWorkbenchExperiment) {
    println!();
    println!("running: {}", exp.id);
    println!("name: {}", exp.name);
    println!("focus: {}", exp.focus);
    if !dispatch_nlp(exp.id) {
        eprintln!("unsupported experiment id: {}", exp.id);
    }
    println!();
}

fn dispatch_nlp(command: &str) -> bool {
    match command {
        "logic-preview" => print_json(run_logic_preview()),
        "inference-preview" => print_json(run_inference_preview()),
        "featstruct-preview" => print_json(run_featstruct_preview()),
        "classify-preview" => print_json(run_classify_preview()),
        "cluster-preview" => print_json(run_cluster_preview()),
        "decision-tree-experiment" => print_json(run_decision_tree_experiment()),
        "logic-processor-preview" => print_json(run_logic_processor_preview()),
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
            let (lo, hi) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
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
    let json_value = serde_json::to_value(value).expect("nlp workbench output should serialize");
    let json =
        serde_json::to_string_pretty(&json_value).expect("nlp workbench output should serialize");
    println!("{json}");
}

fn print_help() {
    println!("NLP Semantic Workbench");
    println!();
    println!("Usage:");
    println!("  cargo run -p gds --bin ml_nlp_cli -- [menu|list|run <selection>|<command>]");
    println!();
    println!("Commands:");
    println!("  menu              Interactive experiment selection");
    println!("  list              Show available NLP workbench experiments");
    println!("  run <selection>   Run by number/id/list/range (e.g., 1,3-4)");
    println!("  logic-preview     Parse, evaluate and skolemize first-order logic");
    println!("  inference-preview Resolution proof of classic syllogisms");
    println!("  featstruct-preview Feature structure unification and subsumption algebra");
    println!("  classify-preview  Naive Bayes + Decision Tree over NLP FeatureSets");
    println!("  cluster-preview   Token-overlap text clustering preview");
    println!("  decision-tree-experiment Compare NLP classify methods with ml decision-tree fixture shape");
    println!("  logic-processor-preview FeatStruct semantics into entailment-style classify");
}
