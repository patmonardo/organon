//! Form Framing and Chunking walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example form_framing_chunking

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::backends::vec::VecInt;
use gds::collections::extensions::chunking::{ChunkedCollection, ChunkingConfig, ChunkingSupport};
use gds::collections::extensions::framing::{
    FrameCollection, FrameOrder, FrameShape, FramingConfig, FramingSupport,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== Form Framing and Chunking ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Framing Configuration",
        "Form starts as a shaped enclosure over an immediate sequence.",
    );
    let framing_values: Vec<i32> = (1..=9).collect();
    let framing_base = VecInt::from(framing_values);
    let mut framed = FrameCollection::new(framing_base);
    let framing_config = FramingConfig {
        shape: FrameShape {
            rows: 3,
            cols: 3,
            order: FrameOrder::RowMajor,
        },
        strict_bounds: true,
    };
    framed.enable_framing(framing_config)?;
    let framing_path = fixture_root.join("00-framing.txt");
    let center_cell = framed.get_cell(1, 1)?;
    let row = framed.row_values(1)?;
    let col = framed.col_values(2)?;
    fs::write(
        &framing_path,
        format!(
            "shape: 3x3 row-major\ncell_1_1: {}\nrow_1: {:?}\ncol_2: {:?}\n",
            center_cell, row, col
        ),
    )?;
    println!("cell(1,1): {}", center_cell);
    println!("persisted: {}", fixture_path(&framing_path));
    println!();

    stage(
        1,
        "Chunking Configuration",
        "Form can also appear as bounded segmental partitioning.",
    );
    let chunking_values: Vec<i32> = (1..=12).collect();
    let chunking_base = VecInt::from(chunking_values);
    let mut chunked = ChunkedCollection::new(chunking_base);
    let chunking_config = ChunkingConfig {
        chunk_size: 4,
        max_chunks: None,
        prefer_power_of_two: false,
        allow_tail: true,
    };
    chunked.enable_chunking(chunking_config)?;

    let mut chunk_lines = Vec::new();
    chunked.for_each_chunk(|range, values| {
        chunk_lines.push(format!("{:?} => {:?}", range, values));
    })?;

    let chunking_path = fixture_root.join("01-chunking.txt");
    fs::write(
        &chunking_path,
        format!("chunk_size: 4\nchunks:\n{}\n", chunk_lines.join("\n")),
    )?;
    println!("chunks: {}", chunk_lines.len());
    println!("persisted: {}", fixture_path(&chunking_path));
    println!();

    stage(
        2,
        "Form Witness",
        "Framing and chunking are complementary form operations over one substrate.",
    );
    let witness_path = fixture_root.join("02-form-witness.txt");
    fs::write(
        &witness_path,
        "framing: enabled\nchunking: enabled\nprinciple: shape-and-partition-are-form-level-operations\n",
    )?;
    println!("persisted: {}", fixture_path(&witness_path));

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&framing_path, &chunking_path, &witness_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/form/form_framing_chunking")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/form/form_framing_chunking/{file_name}")
}

fn manifest(framing_path: &Path, chunking_path: &Path, witness_path: &Path) -> String {
    format!(
        "Form Framing and Chunking Fixture\n\n\
         Namespace: form::framing_chunking\n\n\
         00 Framing\n\
         artifact: {}\n\
         meaning: frame shape establishes row/column enclosure.\n\n\
         01 Chunking\n\
         artifact: {}\n\
         meaning: sequence is partitioned into durable chunk ranges.\n\n\
         02 Witness\n\
         artifact: {}\n\
         meaning: both operations are documented as form-level structure work.\n",
        fixture_path(framing_path),
        fixture_path(chunking_path),
        fixture_path(witness_path),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
