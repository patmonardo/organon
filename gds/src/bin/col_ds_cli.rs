use gds::collections::dataset::workbench::dataset_workbench_track;
use gds::collections::dataset::workbench::dataset_workbench_tracks;
use gds::collections::dataset::workbench::DatasetWorkbenchTrack;
use std::collections::BTreeSet;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next();

    match command.as_deref() {
        None | Some("menu") => run_interactive_menu(),
        Some("list") => print_tracks(),
        Some("run") => {
            if let Some(id) = args.next() {
                run_selection_expr(&id);
            } else {
                eprintln!("missing track id");
                print_help();
                std::process::exit(2);
            }
        }
        Some("help") | Some("--help") | Some("-h") => print_help(),
        Some(other) => {
            eprintln!("unknown command: {other}");
            print_help();
            std::process::exit(2);
        }
    }
}

fn run_interactive_menu() {
    loop {
        println!("Dataset Collections Workbench");
        println!();
        print_tracks();
        println!();
        println!(
            "Select track number/id, batch (1,3-5), or ids (ds-a,ds-b). Commands: l=list, q=quit"
        );
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

fn run_selection_expr(selection: &str) {
    let tracks = dataset_workbench_tracks();
    if let Some(indices) = parse_index_selection(selection, tracks.len()) {
        for index in indices {
            run_track(&tracks[index - 1]);
        }
        return;
    }

    for token in split_tokens(selection) {
        run_track_by_id(token);
    }
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

fn run_track_by_id(id: &str) {
    match dataset_workbench_track(id) {
        Some(track) => run_track(track),
        None => {
            eprintln!("unknown track id: {id}");
            print_tracks();
        }
    }
}

fn run_track(track: &DatasetWorkbenchTrack) {
    let Some(example) = example_name(track.exemplar) else {
        eprintln!("invalid exemplar path: {}", track.exemplar);
        return;
    };

    println!();
    println!("running: {} ({})", track.id, track.title);
    println!("focus: {}", track.focus);
    println!("example: {example}");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(example)
        .current_dir(manifest_dir)
        .status();

    match status {
        Ok(status) if status.success() => {
            println!("completed: {}", track.id);
        }
        Ok(status) => {
            eprintln!("example exited with status: {status}");
        }
        Err(err) => {
            eprintln!("failed to launch cargo: {err}");
        }
    }
    println!();
}

fn example_name(exemplar: &str) -> Option<&str> {
    exemplar
        .strip_prefix("examples/")
        .and_then(|s| s.strip_suffix(".rs"))
}

fn print_tracks() {
    let tracks = dataset_workbench_tracks();
    println!("Tracks:");
    for (idx, track) in tracks.iter().enumerate() {
        println!(
            "  {}. {} [{}] - {}",
            idx + 1,
            track.title,
            track.id,
            track.status
        );
        println!("     focus: {}", track.focus);
    }
}

fn print_help() {
    println!("Collections Dataset Workbench CLI");
    println!();
    println!("Usage:");
    println!("  cargo run -p gds --bin col_ds_cli -- [menu|list|run <selection>]");
    println!();
    println!("Selection supports: single id, number, comma list, or ranges (e.g., 1,3-5).");
    println!("Defaults to interactive menu when no command is provided.");
}
