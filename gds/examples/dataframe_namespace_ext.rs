//! DataFrame Extension Namespace walkthrough.
//!
//! Exercises extension namespace surfaces on both GDSSeries and Expr:
//! - extension registry (register/get/unregister)
//! - GDSSeries.ext().to / storage
//! - Expr.ext().to / storage in DataFrame pipelines
//!
//! Run with:
//!   cargo run -p gds --example dataframe_namespace_ext

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::datatypes::{
    get_extension_type, register_extension_type, unregister_extension_type, GDSDataType,
};
use gds::collections::dataframe::expressions::expr::ExprNamespace;
use gds::collections::dataframe::GDSSeries;

const NODE_IDS: &[i64] = &[1, 2, 3, 4, 5, 6];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Extension Namespace ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    // ------------------------------------------------------------------ Stage 0
    stage(
        0,
        "Source",
        "A compact tree-node table to exercise extension namespace behavior.",
    );

    let source = build_node_table()?;
    println!("{}", source.fmt_table());
    let src_path = persist_csv(&source, &fixture_root, "00-source")?;
    println!("persisted: {}", fixture_path(&src_path));
    println!();

    // ------------------------------------------------------------------ Stage 1
    stage(
        1,
        "Registry",
        "Register semantic extension names (Class and Storage variants).",
    );

    let _ = unregister_extension_type("gds.tree");
    let _ = unregister_extension_type("gds.tree.storage");

    register_extension_type("gds.tree", Some("TreeClass"), false)?;
    register_extension_type("gds.tree.storage", None, true)?;

    let class_reg = get_extension_type("gds.tree");
    let storage_reg = get_extension_type("gds.tree.storage");
    println!("gds.tree => {:?}", class_reg);
    println!("gds.tree.storage => {:?}", storage_reg);

    let reg_path = fixture_root.join("01-registry.txt");
    fs::write(
        &reg_path,
        format!(
            "gds.tree => {:?}\ngds.tree.storage => {:?}\n",
            class_reg, storage_reg,
        ),
    )?;
    println!("persisted: {}", fixture_path(&reg_path));
    println!();

    // ------------------------------------------------------------------ Stage 2
    stage(
        2,
        "GDSSeries.ext",
        "Seed behavior: ext.to casts to storage dtype and ext.storage is identity.",
    );

    let node_ids = GDSSeries::from("node_id", NODE_IDS);
    let ext_ns = node_ids.ext();
    let casted = ext_ns.to(&GDSDataType::Float64)?;
    let storage = ext_ns.storage();

    println!("source dtype:  {:?}", node_ids.dtype());
    println!("casted dtype:  {:?}", casted.dtype());
    println!("storage dtype: {:?}", storage.dtype());
    println!("casted values: {:?}", f64_opt_vec(&casted));
    println!("storage values:{:?}", i64_opt_vec(&storage));

    let series_path = fixture_root.join("02-series-ext.txt");
    fs::write(
        &series_path,
        format!(
            "source dtype:  {:?}\ncasted dtype:  {:?}\nstorage dtype: {:?}\ncasted values: {:?}\nstorage values:{:?}\n",
            node_ids.dtype(),
            casted.dtype(),
            storage.dtype(),
            f64_opt_vec(&casted),
            i64_opt_vec(&storage),
        ),
    )?;
    println!("persisted: {}", fixture_path(&series_path));
    println!();

    // ------------------------------------------------------------------ Stage 3
    stage(
        3,
        "Expr.ext",
        "Extension Expr namespace in a DataFrame pipeline (cast + storage projection).",
    );

    let ext_table = build_node_table()?.with_columns(&[
        ExprNamespace::new(gds::col!("node_id"))
            .ext()
            .to(GDSDataType::Float64)
            .alias("node_id_ext_f64"),
        ExprNamespace::new(gds::col!("node_id"))
            .ext()
            .storage()
            .alias("node_id_storage"),
    ])?;
    println!("{}", ext_table.fmt_table());

    let ext_path = persist_csv(&ext_table, &fixture_root, "03-expr-ext")?;
    println!("persisted: {}", fixture_path(&ext_path));
    println!();

    // ------------------------------------------------------------------ README
    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&src_path, &reg_path, &series_path, &ext_path),
    )?;
    println!("manifest: {}", fixture_path(&manifest_path));

    Ok(())
}

// ── display helpers ───────────────────────────────────────────────────────────

fn i64_opt_vec(series: &polars::prelude::Series) -> Vec<Option<i64>> {
    series
        .cast(&polars::prelude::DataType::Int64)
        .unwrap()
        .i64()
        .unwrap()
        .into_iter()
        .collect()
}

fn f64_opt_vec(series: &polars::prelude::Series) -> Vec<Option<f64>> {
    series
        .cast(&polars::prelude::DataType::Float64)
        .unwrap()
        .f64()
        .unwrap()
        .into_iter()
        .collect()
}

// ── fixture helpers ───────────────────────────────────────────────────────────

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_namespace_ext")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_namespace_ext/{file_name}")
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn persist_csv(
    frame: &gds::collections::dataframe::GDSDataFrame,
    root: &Path,
    stem: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = root.join(format!("{stem}.csv"));
    frame.write_csv(&path_string(&path))?;
    Ok(path)
}

fn build_node_table(
) -> Result<gds::collections::dataframe::GDSDataFrame, Box<dyn std::error::Error>> {
    Ok(gds::tbl_def!(
        (node_id: [1, 2, 3, 4, 5, 6]),
        (label: ["S", "NP", "VP", "PP", "NN", "VBZ"])
    )?)
}

fn stage(n: u32, name: &str, doctrine: &str) {
    println!("-- Stage {n}: {name} --");
    println!("doctrine: {doctrine}");
    println!();
}

fn manifest(src: &Path, reg: &Path, ser: &Path, expr: &Path) -> String {
    format!(
        "DataFrame Extension Namespace Fixture\n\n\
         Namespace: dataframe::namespaces::ext + dataframe::expressions::ext\n\n\
         00 Source\n\
         artifact: {}\n\
         meaning: tree-node source frame used for extension namespace walkthrough.\n\n\
         01 Registry\n\
         artifact: {}\n\
         meaning: extension registration state for Class and Storage variants.\n\n\
         02 GDSSeries.ext\n\
         artifact: {}\n\
         meaning: ext.to cast and ext.storage identity on a series.\n\n\
         03 Expr.ext\n\
         artifact: {}\n\
         meaning: extension expression namespace in a DataFrame projection pipeline.\n",
        fixture_path(src),
        fixture_path(reg),
        fixture_path(ser),
        fixture_path(expr),
    )
}
