//! DataFrame type extension walkthrough.
//!
//! Run with:
//!   cargo run -p gds --example dataframe_type_extensions

use std::fs;
use std::path::{Path, PathBuf};

use gds::collections::dataframe::datatypes::{
    get_extension_type, register_extension_type, BaseExtension, Extension, ExtensionType,
    Field as GDSField, GDSDataType, Struct as GDSStruct,
};
use gds::collections::dataframe::expressions::ext::ExprExt;
use gds::collections::dataframe::functions::datatype::struct_with_fields;
use gds::collections::dataframe::{col, Schema};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("== DataFrame Type Extensions ==");

    let fixture_root = fixture_root();
    fs::create_dir_all(&fixture_root)?;
    println!("fixture root: {}", fixture_root.display());
    println!();

    stage(
        0,
        "Register Extension Name",
        "The local registry records semantic extension names before full Polars dtype wiring.",
    );
    register_extension_type("gds.tree", Some("TreeClass"), false)?;
    let registered = get_extension_type("gds.tree").expect("registered tree extension");
    println!("registered: {registered:?}");
    println!();

    stage(
        1,
        "Describe Extension DType",
        "Extension wraps semantic name, storage dtype, and metadata.",
    );
    let storage_dtype = tree_storage_dtype();
    let base = BaseExtension::new(
        "gds.tree",
        storage_dtype.clone(),
        Some("class=Tree;version=seed".to_string()),
    );
    let extension = Extension::new(
        base.ext_name(),
        base.ext_storage().clone(),
        base.ext_metadata().map(str::to_string),
    );
    println!("extension name:     {}", extension.base().ext_name());
    println!("extension storage:  {:?}", extension.base().ext_storage());
    println!("extension metadata: {:?}", extension.base().ext_metadata());
    println!();

    stage(
        2,
        "Resolve Storage Contract",
        "Until true Polars extension dtype integration lands, collect_dtype resolves the storage contract.",
    );
    let storage_contract = struct_with_fields(&[
        ("label", GDSDataType::String.into()),
        (
            "children",
            GDSDataType::List(Box::new(storage_dtype.clone())).into(),
        ),
        ("span_start", GDSDataType::UInt32.into()),
        ("span_end", GDSDataType::UInt32.into()),
    ])
    .collect_dtype(Schema::default().as_polars())?;
    println!("storage contract: {storage_contract:?}");
    println!();

    stage(
        3,
        "Expression Namespace Seed",
        "ExprExt currently builds storage-level expression nodes; ext.storage is identity in this pass.",
    );
    let ext_ns = ExprExt::new(col("tree"));
    let to_storage_expr = ext_ns.to(storage_dtype.clone());
    let storage_expr = ext_ns.storage();
    println!("ext.to(storage) expr: {to_storage_expr:?}");
    println!("ext.storage() expr:   {storage_expr:?}");

    let manifest_path = fixture_root.join("README.txt");
    fs::write(
        &manifest_path,
        manifest(&registered, extension.base(), &storage_contract),
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
        .join("fixtures/collections/dataframe/dataframe_type_extensions")
}

fn fixture_path(path: &Path) -> String {
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());
    format!("fixtures/collections/dataframe/dataframe_type_extensions/{file_name}")
}

fn manifest(
    registered: &ExtensionType,
    base: &BaseExtension,
    storage_contract: &GDSDataType,
) -> String {
    format!(
        "DataFrame Type Extensions Fixture\n\n\
         Namespace: dataframe::datatypes::extension + dataframe::expressions::ext\n\n\
         registered gds.tree: {registered:?}\n\
         extension name: {}\n\
         extension storage: {:?}\n\
         extension metadata: {:?}\n\
         storage contract: {storage_contract:?}\n\n\
         Current gap: this local registry does not yet materialize a Polars DataType::Extension.\n\
         The expression namespace currently lowers to storage-level cast/identity behavior.\n",
        base.ext_name(),
        base.ext_storage(),
        base.ext_metadata(),
    )
}

fn stage(number: u8, name: &str, doctrine: &str) {
    println!("-- Stage {number}: {name} --");
    println!("doctrine: {doctrine}");
}
