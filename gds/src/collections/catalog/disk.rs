//! Collections Catalog (Disk)
//!
//! Provides a simple on-disk manifest and IO helpers for Collections.

use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::collections::catalog::polars_io::{
    read_collection_csv, read_collection_ipc, read_collection_parquet, write_collection_csv,
    write_collection_ipc, write_collection_parquet, PolarsCollectionType,
};
use crate::collections::catalog::schema::CollectionsSchema;
use crate::collections::catalog::types::{
    CatalogError, CollectionsCatalogDiskEntry, CollectionsCatalogManifest, CollectionsIoFormat,
};
use crate::collections::catalog::unity::{ColumnInfo, DataSourceFormat, TableInfo, TableType};
use crate::collections::dataframe::{read_table_csv, read_table_ipc, read_table_parquet, Selector};
use crate::collections::dataframe::{
    write_table_csv, write_table_ipc, write_table_parquet, PolarsDataFrameCollection,
};
use crate::collections::Collections;
use polars::prelude::{
    col, DataFrame, LazyCsvReader, LazyFileListReader, LazyFrame, PlPath, ScanArgsIpc,
    ScanArgsParquet,
};

pub const CATALOG_MANIFEST_FILE: &str = "catalog.json";

/// Disk-backed catalog with a JSON manifest.
#[derive(Debug, Clone)]
pub struct CollectionsCatalogDisk {
    root: PathBuf,
    manifest: CollectionsCatalogManifest,
}

impl CollectionsCatalogDisk {
    /// Load or initialize a catalog at the given root path.
    pub fn load(root: impl Into<PathBuf>) -> Result<Self, CatalogError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|e| CatalogError::Io(e.to_string()))?;
        let manifest_path = root.join(CATALOG_MANIFEST_FILE);
        let manifest = if manifest_path.exists() {
            let raw =
                fs::read_to_string(&manifest_path).map_err(|e| CatalogError::Io(e.to_string()))?;
            serde_json::from_str::<CollectionsCatalogManifest>(&raw)
                .map_err(|e| CatalogError::Parse(e.to_string()))?
        } else {
            CollectionsCatalogManifest {
                version: 1,
                created_at: Utc::now().to_rfc3339(),
                entries: Vec::new(),
            }
        };

        Ok(Self { root, manifest })
    }

    /// Persist the manifest to disk.
    pub fn save(&self) -> Result<(), CatalogError> {
        let manifest_path = self.root.join(CATALOG_MANIFEST_FILE);
        let data = serde_json::to_string_pretty(&self.manifest)
            .map_err(|e| CatalogError::Parse(e.to_string()))?;
        fs::write(manifest_path, data).map_err(|e| CatalogError::Io(e.to_string()))?;
        Ok(())
    }

    /// Catalog root path.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Register a new entry in the catalog.
    pub fn register(&mut self, entry: CollectionsCatalogDiskEntry) -> Result<(), CatalogError> {
        if self
            .manifest
            .entries
            .iter()
            .any(|existing| existing.name == entry.name)
        {
            return Err(CatalogError::AlreadyExists(entry.name));
        }
        self.manifest.entries.push(entry);
        Ok(())
    }

    /// Get an entry by name.
    pub fn get(&self, name: &str) -> Option<&CollectionsCatalogDiskEntry> {
        self.manifest
            .entries
            .iter()
            .find(|entry| entry.name == name)
    }

    /// List all entries.
    pub fn list(&self) -> &[CollectionsCatalogDiskEntry] {
        &self.manifest.entries
    }

    /// Remove an entry by name.
    pub fn remove(&mut self, name: &str) -> Option<CollectionsCatalogDiskEntry> {
        let index = self
            .manifest
            .entries
            .iter()
            .position(|entry| entry.name == name)?;
        Some(self.manifest.entries.remove(index))
    }

    /// List Unity-style table metadata for all catalog entries.
    pub fn list_table_info(&self) -> Vec<TableInfo> {
        self.manifest
            .entries
            .iter()
            .map(|entry| self.entry_to_table_info(entry))
            .collect()
    }

    /// Get Unity-style table metadata for a specific entry.
    pub fn table_info(&self, name: &str) -> Result<TableInfo, CatalogError> {
        let entry = self
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))?;
        Ok(self.entry_to_table_info(entry))
    }

    /// Refresh schema metadata for an entry by reading the on-disk data.
    pub fn refresh_schema(&mut self, name: &str) -> Result<(), CatalogError> {
        let root = self.root.clone();
        let mut entries = std::mem::take(&mut self.manifest.entries);
        let result = (|| {
            let mut found = false;
            for entry in entries.iter_mut() {
                if entry.name == name {
                    let data_path = root.join(&entry.data_path);
                    let schema = Self::infer_schema_from_path(entry.io_policy.format, &data_path)?;
                    entry.schema = Some(schema);
                    found = true;
                    break;
                }
            }
            if !found {
                return Err(CatalogError::NotFound(name.to_string()));
            }
            Ok(())
        })();
        self.manifest.entries = entries;
        result
    }

    /// Refresh schema metadata for all entries.
    pub fn refresh_all_schemas(&mut self) -> Result<(), CatalogError> {
        let root = self.root.clone();
        let mut entries = std::mem::take(&mut self.manifest.entries);
        let result = (|| {
            for entry in entries.iter_mut() {
                let data_path = root.join(&entry.data_path);
                let schema = Self::infer_schema_from_path(entry.io_policy.format, &data_path)?;
                entry.schema = Some(schema);
            }
            Ok(())
        })();
        self.manifest.entries = entries;
        result
    }

    /// Return the on-disk data path for an entry.
    pub fn data_path(&self, entry: &CollectionsCatalogDiskEntry) -> PathBuf {
        self.root.join(&entry.data_path)
    }

    /// Default relative path for an entry (collections/<name>/data.<ext>)
    pub fn default_data_path(&self, name: &str, format: CollectionsIoFormat) -> String {
        format!("collections/{name}/data.{}", format.file_extension())
    }

    /// Write a collection to disk using the entry's IO policy.
    pub fn write_collection<T, C>(
        &self,
        entry: &CollectionsCatalogDiskEntry,
        collection: &C,
    ) -> Result<(), CatalogError>
    where
        T: PolarsCollectionType + Clone,
        C: Collections<T>,
    {
        let data_path = self.data_path(entry);
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| CatalogError::Io(e.to_string()))?;
        }

        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                write_collection_parquet(&data_path, &entry.name, collection)
            }
            CollectionsIoFormat::ArrowIpc => {
                write_collection_ipc(&data_path, &entry.name, collection)
            }
            CollectionsIoFormat::Csv => write_collection_csv(&data_path, &entry.name, collection),
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON write not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database write not implemented for Collections yet".to_string(),
            )),
        }
    }

    /// Write a DataFrame-backed table using the entry's IO policy.
    pub fn write_table(
        &self,
        entry: &CollectionsCatalogDiskEntry,
        table: &PolarsDataFrameCollection,
    ) -> Result<(), CatalogError> {
        let data_path = self.data_path(entry);
        if let Some(parent) = data_path.parent() {
            fs::create_dir_all(parent).map_err(|e| CatalogError::Io(e.to_string()))?;
        }

        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                write_table_parquet(&data_path, table)
                    .map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::ArrowIpc => {
                write_table_ipc(&data_path, table).map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::Csv => {
                write_table_csv(&data_path, table).map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON write not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database write not implemented for Collections yet".to_string(),
            )),
        }
    }

    /// Read a collection from disk using the entry's IO policy.
    pub fn read_collection<T>(
        &self,
        entry: &CollectionsCatalogDiskEntry,
    ) -> Result<Vec<T>, CatalogError>
    where
        T: PolarsCollectionType,
    {
        let data_path = self.data_path(entry);
        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                read_collection_parquet(&data_path, &entry.name)
            }
            CollectionsIoFormat::ArrowIpc => read_collection_ipc(&data_path, &entry.name),
            CollectionsIoFormat::Csv => read_collection_csv(&data_path, &entry.name),
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON read not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database read not implemented for Collections yet".to_string(),
            )),
        }
    }

    /// Read a DataFrame-backed table using the entry's IO policy.
    pub fn read_table(
        &self,
        entry: &CollectionsCatalogDiskEntry,
    ) -> Result<PolarsDataFrameCollection, CatalogError> {
        let data_path = self.data_path(entry);
        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                read_table_parquet(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::ArrowIpc => {
                read_table_ipc(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::Csv => {
                read_table_csv(&data_path).map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON read not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database read not implemented for Collections yet".to_string(),
            )),
        }
    }

    /// Read a DataFrame-backed table and apply a selector projection.
    pub fn read_table_select(
        &self,
        entry: &CollectionsCatalogDiskEntry,
        selector: &Selector,
    ) -> Result<PolarsDataFrameCollection, CatalogError> {
        let table = self.read_table(entry)?;
        table
            .select_selector(selector)
            .map_err(|e| CatalogError::Polars(e.to_string()))
    }

    /// Scan a DataFrame-backed table into a LazyFrame.
    pub fn scan_table(
        &self,
        entry: &CollectionsCatalogDiskEntry,
    ) -> Result<LazyFrame, CatalogError> {
        let data_path = self.data_path(entry);
        let path_str = data_path.to_string_lossy();
        let path = PlPath::new(path_str.as_ref());
        match entry.io_policy.format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                LazyFrame::scan_parquet(path, ScanArgsParquet::default())
                    .map_err(|e| CatalogError::Polars(e.to_string()))
            }
            CollectionsIoFormat::ArrowIpc => LazyFrame::scan_ipc(path, ScanArgsIpc::default())
                .map_err(|e| CatalogError::Polars(e.to_string())),
            CollectionsIoFormat::Csv => LazyCsvReader::new(path)
                .finish()
                .map_err(|e: polars::error::PolarsError| CatalogError::Polars(e.to_string())),
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON scan not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database scan not implemented for Collections yet".to_string(),
            )),
        }
    }

    /// Scan a table into a LazyFrame and apply selector projection.
    pub fn scan_table_select(
        &self,
        entry: &CollectionsCatalogDiskEntry,
        selector: &Selector,
    ) -> Result<LazyFrame, CatalogError> {
        let mut lazy = self.scan_table(entry)?;
        let schema = lazy
            .collect_schema()
            .map_err(|e| CatalogError::Polars(e.to_string()))?;
        let df = DataFrame::empty_with_schema(schema.as_ref());
        let names = crate::collections::dataframe::selectors::expand_selector(&df, selector);
        let exprs = names.into_iter().map(|name| col(&name)).collect::<Vec<_>>();
        Ok(lazy.select(exprs))
    }

    /// Validate a path is inside the catalog root.
    pub fn validate_entry_path(&self, path: &Path) -> Result<(), CatalogError> {
        if !path.starts_with(&self.root) {
            return Err(CatalogError::Io(format!(
                "Path {path:?} is outside catalog root {:?}",
                self.root
            )));
        }
        Ok(())
    }

    fn infer_schema_from_path(
        format: CollectionsIoFormat,
        data_path: &Path,
    ) -> Result<CollectionsSchema, CatalogError> {
        match format {
            CollectionsIoFormat::Auto | CollectionsIoFormat::Parquet => {
                let table = read_table_parquet(data_path)
                    .map_err(|e| CatalogError::Polars(e.to_string()))?;
                Ok(CollectionsSchema::from_polars(table.dataframe()))
            }
            CollectionsIoFormat::ArrowIpc => {
                let table =
                    read_table_ipc(data_path).map_err(|e| CatalogError::Polars(e.to_string()))?;
                Ok(CollectionsSchema::from_polars(table.dataframe()))
            }
            CollectionsIoFormat::Csv => {
                let table =
                    read_table_csv(data_path).map_err(|e| CatalogError::Polars(e.to_string()))?;
                Ok(CollectionsSchema::from_polars(table.dataframe()))
            }
            CollectionsIoFormat::Json => Err(CatalogError::Polars(
                "JSON schema inference not implemented for Collections yet".to_string(),
            )),
            CollectionsIoFormat::Database => Err(CatalogError::Polars(
                "Database schema inference not implemented for Collections yet".to_string(),
            )),
        }
    }

    fn entry_to_table_info(&self, entry: &CollectionsCatalogDiskEntry) -> TableInfo {
        let columns = entry
            .schema
            .as_ref()
            .map(|schema| schema_to_columns(schema))
            .unwrap_or_default();

        TableInfo {
            name: entry.name.clone(),
            comment: None,
            table_id: entry.name.clone(),
            table_type: TableType::Managed,
            storage_location: Some(self.data_path(entry).to_string_lossy().to_string()),
            data_source_format: Some(io_format_to_data_source(entry.io_policy.format)),
            columns,
            properties: Vec::new(),
            created_at: None,
            created_by: None,
            updated_at: None,
            updated_by: None,
        }
    }
}

fn schema_to_columns(schema: &CollectionsSchema) -> Vec<ColumnInfo> {
    schema
        .fields
        .iter()
        .enumerate()
        .map(|(index, field)| ColumnInfo {
            name: field.name.clone(),
            type_name: field.value_type.name().to_string(),
            type_text: field.value_type.name().to_string(),
            type_json: field.value_type.name().to_string(),
            position: Some(index),
            comment: None,
            partition_index: None,
        })
        .collect()
}

fn io_format_to_data_source(format: CollectionsIoFormat) -> DataSourceFormat {
    match format {
        CollectionsIoFormat::Parquet | CollectionsIoFormat::Auto => DataSourceFormat::Parquet,
        CollectionsIoFormat::Csv => DataSourceFormat::Csv,
        CollectionsIoFormat::Json => DataSourceFormat::Json,
        CollectionsIoFormat::ArrowIpc => DataSourceFormat::UnityCatalog,
        CollectionsIoFormat::Database => DataSourceFormat::Unknown,
    }
}
