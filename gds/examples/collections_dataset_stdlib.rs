use gds::collections::dataset::prelude::*;

fn main() {
    let mut args = std::env::args().skip(1);
    let resource = args.next();

    let resources = list_resources();
    let chosen = resource
        .as_deref()
        .or_else(|| resources.first().map(|r| r.name))
        .expect("no stdlib resources configured");

    println!(
        "available: {}",
        resources
            .iter()
            .map(|r| r.name)
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!("fetching: {}", chosen);

    match fetch_resource(chosen, None::<std::path::PathBuf>) {
        Ok(report) => {
            println!("resource: {}", report.name);
            println!("archive: {}", report.archive_path.display());
            println!("data_dir: {}", report.data_dir.display());
            println!("bytes: {}", report.bytes);
        }
        Err(err) => {
            eprintln!("fetch failed: {err}");
        }
    }
}
