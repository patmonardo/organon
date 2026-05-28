//! Dataset catalog integration with CollectionsCatalogDisk.
//!
//! Dataset catalog integration with CollectionsCatalogDisk.
//!
//! This module owns Dataset-facing catalog and registry helpers: the
//! disk-backed catalog of concrete tables, plus a small legacy in-memory
//! family/split registry used by examples and planning code.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::collections::catalog::schema::CollectionsSchema;
use crate::collections::catalog::{
    CatalogError, CollectionsCatalogDisk, CollectionsCatalogDiskEntry, CollectionsIoFormat,
    CollectionsIoPolicy,
};
use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::core::artifact::DatasetArtifactProfile;
use crate::collections::dataset::core::dataset::Dataset;
use crate::collections::dataset::core::io::detect_format_from_path;
use crate::config::CollectionsBackend;
use crate::types::ValueType;

/// Indicates whether a schema came directly from the persisted catalog entry or
/// had to be derived from the underlying table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatasetCatalogSchemaSource {
    Catalog,
    Derived,
}

impl DatasetCatalogSchemaSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Catalog => "catalog",
            Self::Derived => "dataset",
        }
    }
}

/// Unified schema view for a Dataset catalog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedDatasetSchema {
    pub schema: CollectionsSchema,
    pub source: DatasetCatalogSchemaSource,
}

/// Dataset-first wrapper for the disk catalog.
#[derive(Debug, Clone)]
pub struct DatasetCatalog {
    catalog: CollectionsCatalogDisk,
}

impl DatasetCatalog {
    pub fn load(root: impl Into<PathBuf>) -> Result<Self, CatalogError> {
        let catalog = CollectionsCatalogDisk::load(root)?;
        Ok(Self { catalog })
    }

    pub fn catalog(&self) -> &CollectionsCatalogDisk {
        &self.catalog
    }

    pub fn catalog_mut(&mut self) -> &mut CollectionsCatalogDisk {
        &mut self.catalog
    }

    pub fn save(&self) -> Result<(), CatalogError> {
        self.catalog.save()
    }

    pub fn register_table_path(
        &mut self,
        name: &str,
        data_path: impl AsRef<Path>,
        format: Option<CollectionsIoFormat>,
        schema: Option<CollectionsSchema>,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let data_path = data_path.as_ref();
        let format = format.unwrap_or_else(|| detect_format_from_path(data_path));
        let relative_path = self.ensure_relative_path(data_path)?;
        let schema = match schema {
            Some(schema) => Some(schema),
            None => Some(self.infer_table_schema(data_path, format)?),
        };
        let entry = self.build_table_entry(name, format, schema, relative_path);
        self.catalog.register(entry.clone())?;
        Ok(entry)
    }

    pub fn ingest_table(
        &mut self,
        name: &str,
        table: &GDSDataFrame,
        format: CollectionsIoFormat,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        let schema = Some(CollectionsSchema::from_polars(table.dataframe()));
        let data_path = self.catalog.default_data_path(name, format);
        let entry = self.build_table_entry(name, format, schema, data_path);
        self.catalog.register(entry.clone())?;
        self.catalog.write_table(&entry, table)?;
        Ok(entry)
    }

    pub fn load_table(&self, name: &str) -> Result<Dataset, CatalogError> {
        let entry = self.entry(name)?;
        let table = self.catalog.read_table(entry)?;
        Ok(Dataset::named(name.to_string(), table))
    }

    pub fn entry(&self, name: &str) -> Result<&CollectionsCatalogDiskEntry, CatalogError> {
        self.catalog
            .get(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))
    }

    pub fn resolve_table_schema(&self, name: &str) -> Result<ResolvedDatasetSchema, CatalogError> {
        let entry = self.entry(name)?;
        if let Some(schema) = entry.schema.clone() {
            return Ok(ResolvedDatasetSchema {
                schema,
                source: DatasetCatalogSchemaSource::Catalog,
            });
        }

        let table = self.catalog.read_table(entry)?;
        Ok(ResolvedDatasetSchema {
            schema: CollectionsSchema::from_polars(table.dataframe()),
            source: DatasetCatalogSchemaSource::Derived,
        })
    }

    pub fn remove_entry(
        &mut self,
        name: &str,
    ) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
        self.catalog
            .remove(name)
            .ok_or_else(|| CatalogError::NotFound(name.to_string()))
    }

    fn ensure_relative_path(&self, path: &Path) -> Result<String, CatalogError> {
        if path.is_absolute() {
            let root = self.catalog.root();
            if !path.starts_with(root) {
                return Err(CatalogError::Io(format!(
                    "Path {:?} is outside catalog root {:?}",
                    path, root
                )));
            }
            let relative = path
                .strip_prefix(root)
                .map_err(|e| CatalogError::Io(e.to_string()))?;
            return Ok(relative.to_string_lossy().to_string());
        }
        Ok(path.to_string_lossy().to_string())
    }

    fn infer_table_schema(
        &self,
        data_path: &Path,
        format: CollectionsIoFormat,
    ) -> Result<CollectionsSchema, CatalogError> {
        let relative_path = self.ensure_relative_path(data_path)?;
        let entry = self.build_table_entry("__schema_probe__", format, None, relative_path);
        let table = self.catalog.read_table(&entry)?;
        Ok(CollectionsSchema::from_polars(table.dataframe()))
    }

    fn build_table_entry(
        &self,
        name: &str,
        format: CollectionsIoFormat,
        schema: Option<CollectionsSchema>,
        data_path: impl Into<String>,
    ) -> CollectionsCatalogDiskEntry {
        let io_policy = CollectionsIoPolicy {
            format,
            ..Default::default()
        };
        CollectionsCatalogDiskEntry {
            name: name.to_string(),
            value_type: ValueType::Unknown,
            schema,
            backend: CollectionsBackend::Arrow,
            extensions: Vec::new(),
            io_policy,
            data_path: data_path.into(),
        }
    }
}

/// Common dataset splits for family-level registry planning.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatasetSplit {
    Train,
    Validation,
    Test,
    All,
    Custom(String),
}

/// Minimal metadata for a dataset family entry.
#[derive(Debug, Clone, Default)]
pub struct DatasetMetadata {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub tags: Vec<String>,
    pub artifact_profile: DatasetArtifactProfile,
}

impl DatasetMetadata {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn with_artifact_profile(mut self, artifact_profile: DatasetArtifactProfile) -> Self {
        self.artifact_profile = artifact_profile;
        self
    }

    pub fn from_dataset(dataset: &Dataset) -> Option<Self> {
        let name = dataset.name()?.to_string();
        Some(Self::new(name).with_artifact_profile(dataset.artifact_profile().clone()))
    }
}

/// Resolved dataset artifact path for a family entry and split.
#[derive(Debug, Clone)]
pub struct DatasetArtifact {
    pub split: DatasetSplit,
    pub path: PathBuf,
    pub format_hint: Option<String>,
    pub artifact_profile: DatasetArtifactProfile,
}

/// Legacy in-memory registry for dataset family metadata and roots.
#[derive(Debug, Clone)]
pub struct DatasetRegistry {
    root: PathBuf,
    entries: HashMap<String, DatasetMetadata>,
}

impl DatasetRegistry {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: HashMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn register(&mut self, metadata: DatasetMetadata) {
        self.entries.insert(metadata.name.clone(), metadata);
    }

    pub fn register_dataset(&mut self, dataset: &Dataset) -> Result<(), String> {
        let metadata = DatasetMetadata::from_dataset(dataset)
            .ok_or_else(|| "cannot register unnamed dataset".to_string())?;
        self.register(metadata);
        Ok(())
    }

    pub fn register_names(&mut self, names: &[&str]) {
        for name in names {
            self.register(DatasetMetadata {
                name: name.to_string(),
                ..Default::default()
            });
        }
    }

    pub fn list(&self) -> Vec<&DatasetMetadata> {
        let mut values: Vec<&DatasetMetadata> = self.entries.values().collect();
        values.sort_by(|a, b| a.name.cmp(&b.name));
        values
    }

    pub fn get(&self, name: &str) -> Option<&DatasetMetadata> {
        self.entries.get(name)
    }

    pub fn dataset_root(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    pub fn dataset_artifact(&self, name: &str, split: DatasetSplit) -> DatasetArtifact {
        let artifact_profile = self
            .get(name)
            .map(|metadata| metadata.artifact_profile.clone())
            .unwrap_or_default();
        DatasetArtifact {
            split,
            path: self.dataset_root(name),
            format_hint: None,
            artifact_profile,
        }
    }
}
