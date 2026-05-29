//! DataFrame type expression walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_type_expressions

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::datatypes::{Field as GDSField, GDSDataType, Struct as GDSStruct};
use gds::collections::dataframe::functions::datatype::{dtype_of, struct_with_fields};
use gds::collections::dataframe::{col, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Type Expressions ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    let tree_storage = tree_storage_dtype();
    let schema = Schema::from_pairs(
        [
            ("tree", tree_storage.clone()),
            ("token_ids", GDSDataType::List(Box::new(GDSDataType::Int64))),
            (
                "embedding",
                GDSDataType::Array(Box::new(GDSDataType::Float32), 3),
            ),
            ("user_id", GDSDataType::UInt64),
        ],
        true,
    )?;

    stage(
        0,
        "Tree Storage DType",
        "A Tree can begin as a nested Struct/List storage dtype.",
    );
    let tree_dtype = dtype_of(col("tree")).collect_dtype(schema.as_polars())?;
    let label_dtype = dtype_of(col("tree"))
        .structure()
        .field_dtype("label")
        .collect_dtype(schema.as_polars())?;
    let child_label_dtype = dtype_of(col("tree"))
        .structure()
        .field_dtype("children")
        .list()
        .inner_dtype()
        .structure()
        .field_dtype("label")
        .collect_dtype(schema.as_polars())?;

    println!("tree dtype:        {tree_dtype:?}");
    println!("label dtype:       {label_dtype:?}");
    println!("child label dtype: {child_label_dtype:?}");
    println!();

    stage(
        1,
        "Collection DTypes",
        "List and Array namespaces expose inner dtype relations.",
    );
    let token_inner_dtype = dtype_of(col("token_ids"))
        .list()
        .inner_dtype()
        .collect_dtype(schema.as_polars())?;
    let embedding_inner_dtype = dtype_of(col("embedding"))
        .arr()
        .inner_dtype()
        .collect_dtype(schema.as_polars())?;

    println!("token inner dtype:     {token_inner_dtype:?}");
    println!("embedding inner dtype: {embedding_inner_dtype:?}");
    println!();

    stage(
        2,
        "Derived DTypes",
        "DataTypeExpr can express output dtype contracts without materializing data.",
    );
    let signed_user_id_dtype = dtype_of(col("user_id"))
        .to_signed_integer()
        .collect_dtype(schema.as_polars())?;
    let tree_contract_dtype = struct_with_fields(&[
        ("label", GDSDataType::String.into()),
        (
            "children",
            GDSDataType::List(Box::new(tree_storage_dtype())).into(),
        ),
        ("span_start", GDSDataType::UInt32.into()),
        ("span_end", GDSDataType::UInt32.into()),
    ])
    .collect_dtype(Schema::default().as_polars())?;

    println!("signed user id dtype: {signed_user_id_dtype:?}");
    println!("tree contract dtype: {tree_contract_dtype:?}");

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(
            &tree_dtype,
            &label_dtype,
            &child_label_dtype,
            &token_inner_dtype,
            &embedding_inner_dtype,
            &signed_user_id_dtype,
            &tree_contract_dtype,
        ),
    )?;
    println!("\nmanifest: {}", fixture_path(&manifest_path));

    Ok(())
}

fn tree_storage_dtype() -> GDSDataType {
    GDSStruct::new(vec![
        GDSField::new("label", GDSDataType::String),
        GDSField::new(
            "children",
            GDSDataType::List(Box::new(
                GDSStruct::new(vec![
                    GDSField::new("label", GDSDataType::String),
                    GDSField::new("span_start", GDSDataType::UInt32),
                    GDSField::new("span_end", GDSDataType::UInt32),
                ])
                .to_dtype(),
            )),
        ),
        GDSField::new("span_start", GDSDataType::UInt32),
        GDSField::new("span_end", GDSDataType::UInt32),
    ])
    .to_dtype()
}

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/collections/dataframe/dataframe_type_expressions")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_type_expressions/{file_name}")
}

fn manifest(
    tree_dtype: &GDSDataType,
    label_dtype: &GDSDataType,
    child_label_dtype: &GDSDataType,
    token_inner_dtype: &GDSDataType,
    embedding_inner_dtype: &GDSDataType,
    signed_user_id_dtype: &GDSDataType,
    tree_contract_dtype: &GDSDataType,
) -> String {
    format!(
        "DataFrame Type Expressions Fixture\n\n\
         Namespace: dataframe::datatype_expr\n\n\
         tree dtype: {tree_dtype:?}\n\
         label dtype: {label_dtype:?}\n\
         child label dtype: {child_label_dtype:?}\n\
         token inner dtype: {token_inner_dtype:?}\n\
         embedding inner dtype: {embedding_inner_dtype:?}\n\
         signed user id dtype: {signed_user_id_dtype:?}\n\
         tree contract dtype: {tree_contract_dtype:?}\n\n\
         This fixture exercises collect_dtype over nested Struct/List/Array storage.\n"
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
