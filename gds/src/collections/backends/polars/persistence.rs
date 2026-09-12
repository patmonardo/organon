//! Durable, immutable Polars frame snapshots.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use polars::prelude::SchemaExt;
use polars_utils::pl_path::PlRefPath;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::io::parquet;

use super::PolarsFrameBackend;

const MANIFEST_FILE: &str = "manifest.json";
const DATA_FILE: &str = "data.parquet";
const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistentFrameFormat {
    Parquet,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistentFrameManifest {
    pub format_version: u32,
    pub artifact_id: String,
    pub snapshot_version: u64,
    pub storage_format: PersistentFrameFormat,
    pub data_file: String,
    pub row_count: usize,
    pub column_count: usize,
    pub schema_fingerprint: String,
    pub data_sha256: String,
    pub created_at: DateTime<Utc>,
    pub provenance: BTreeMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum PersistentFrameError {
    #[error("invalid persistent frame identity: {0}")]
    InvalidIdentity(String),
    #[error("persistent frame snapshot already exists: {0}")]
    AlreadyExists(String),
    #[error("persistent frame IO failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("persistent frame serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("persistent frame Polars operation failed: {0}")]
    Polars(#[from] polars::error::PolarsError),
    #[error("unsupported persistent frame format version: {0}")]
    UnsupportedFormatVersion(u32),
    #[error("persistent frame manifest contains an unsafe data path: {0}")]
    UnsafeDataPath(String),
    #[error("persistent frame checksum mismatch: expected {expected}, found {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("persistent frame schema mismatch: expected {expected}, found {actual}")]
    SchemaMismatch { expected: String, actual: String },
    #[error("persistent frame shape mismatch: expected {expected:?}, found {actual:?}")]
    ShapeMismatch {
        expected: (usize, usize),
        actual: (usize, usize),
    },
}

/// Rooted storage service for immutable Polars frame versions.
#[derive(Debug, Clone)]
pub struct PersistentPolarsStore {
    root: PathBuf,
}

impl PersistentPolarsStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn publish(
        &self,
        artifact_id: impl Into<String>,
        snapshot_version: u64,
        backend: PolarsFrameBackend,
    ) -> Result<PersistentPolarsFrame, PersistentFrameError> {
        self.publish_with_provenance(artifact_id, snapshot_version, backend, BTreeMap::new())
    }

    pub fn publish_with_provenance(
        &self,
        artifact_id: impl Into<String>,
        snapshot_version: u64,
        backend: PolarsFrameBackend,
        provenance: BTreeMap<String, String>,
    ) -> Result<PersistentPolarsFrame, PersistentFrameError> {
        let artifact_id = artifact_id.into();
        validate_artifact_id(&artifact_id)?;
        let artifact_dir = self.artifact_dir(&artifact_id);
        fs::create_dir_all(&artifact_dir)?;
        let version_name = version_directory(snapshot_version);
        let final_dir = artifact_dir.join(&version_name);
        if final_dir.exists() {
            return Err(PersistentFrameError::AlreadyExists(
                final_dir.display().to_string(),
            ));
        }

        let staging_dir = artifact_dir.join(format!(".{version_name}.{}.staging", Uuid::new_v4()));
        fs::create_dir(&staging_dir)?;
        let result = self.stage_snapshot(
            &staging_dir,
            artifact_id,
            snapshot_version,
            backend,
            provenance,
        );
        let manifest = match result {
            Ok(manifest) => manifest,
            Err(error) => {
                let _ = fs::remove_dir_all(&staging_dir);
                return Err(error);
            }
        };

        fs::rename(&staging_dir, &final_dir)?;
        sync_directory(&artifact_dir)?;
        Ok(PersistentPolarsFrame {
            snapshot_dir: final_dir,
            manifest,
        })
    }

    pub fn open(
        &self,
        artifact_id: &str,
        snapshot_version: u64,
    ) -> Result<PersistentPolarsFrame, PersistentFrameError> {
        validate_artifact_id(artifact_id)?;
        PersistentPolarsFrame::open(
            self.artifact_dir(artifact_id)
                .join(version_directory(snapshot_version)),
        )
    }

    fn artifact_dir(&self, artifact_id: &str) -> PathBuf {
        self.root.join(sha256_bytes(artifact_id.as_bytes()))
    }

    fn stage_snapshot(
        &self,
        staging_dir: &Path,
        artifact_id: String,
        snapshot_version: u64,
        backend: PolarsFrameBackend,
        provenance: BTreeMap<String, String>,
    ) -> Result<PersistentFrameManifest, PersistentFrameError> {
        let frame = backend.collect()?;
        let data_path = staging_dir.join(DATA_FILE);
        parquet::write_table(&data_path, &frame)?;
        File::open(&data_path)?.sync_all()?;

        let manifest = PersistentFrameManifest {
            format_version: FORMAT_VERSION,
            artifact_id,
            snapshot_version,
            storage_format: PersistentFrameFormat::Parquet,
            data_file: DATA_FILE.to_string(),
            row_count: frame.height(),
            column_count: frame.width(),
            schema_fingerprint: schema_fingerprint(&frame),
            data_sha256: sha256_file(&data_path)?,
            created_at: Utc::now(),
            provenance,
        };
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        let mut manifest_file = File::create(staging_dir.join(MANIFEST_FILE))?;
        manifest_file.write_all(&manifest_bytes)?;
        manifest_file.sync_all()?;
        sync_directory(staging_dir)?;
        Ok(manifest)
    }
}

/// Verified handle to one published Polars frame version.
#[derive(Debug, Clone)]
pub struct PersistentPolarsFrame {
    snapshot_dir: PathBuf,
    manifest: PersistentFrameManifest,
}

impl PersistentPolarsFrame {
    pub fn open(snapshot_dir: impl Into<PathBuf>) -> Result<Self, PersistentFrameError> {
        let snapshot_dir = snapshot_dir.into();
        let manifest: PersistentFrameManifest =
            serde_json::from_reader(File::open(snapshot_dir.join(MANIFEST_FILE))?)?;
        let frame = Self {
            snapshot_dir,
            manifest,
        };
        frame.verify()?;
        Ok(frame)
    }

    pub fn manifest(&self) -> &PersistentFrameManifest {
        &self.manifest
    }

    pub fn snapshot_dir(&self) -> &Path {
        &self.snapshot_dir
    }

    pub fn data_path(&self) -> Result<PathBuf, PersistentFrameError> {
        if self.manifest.data_file != DATA_FILE {
            return Err(PersistentFrameError::UnsafeDataPath(
                self.manifest.data_file.clone(),
            ));
        }
        Ok(self.snapshot_dir.join(DATA_FILE))
    }

    pub fn verify(&self) -> Result<(), PersistentFrameError> {
        if self.manifest.format_version != FORMAT_VERSION {
            return Err(PersistentFrameError::UnsupportedFormatVersion(
                self.manifest.format_version,
            ));
        }
        let data_path = self.data_path()?;
        let actual = sha256_file(&data_path)?;
        if actual != self.manifest.data_sha256 {
            return Err(PersistentFrameError::ChecksumMismatch {
                expected: self.manifest.data_sha256.clone(),
                actual,
            });
        }
        let frame = parquet::read_table(&data_path)?;
        let actual_shape = frame.shape();
        let expected_shape = (self.manifest.row_count, self.manifest.column_count);
        if actual_shape != expected_shape {
            return Err(PersistentFrameError::ShapeMismatch {
                expected: expected_shape,
                actual: actual_shape,
            });
        }
        let actual_schema = schema_fingerprint(&frame);
        if actual_schema != self.manifest.schema_fingerprint {
            return Err(PersistentFrameError::SchemaMismatch {
                expected: self.manifest.schema_fingerprint.clone(),
                actual: actual_schema,
            });
        }
        Ok(())
    }

    pub fn lazy_backend(&self) -> Result<PolarsFrameBackend, PersistentFrameError> {
        let path = self.data_path()?;
        let lazy = parquet::scan_table(PlRefPath::new(path.to_string_lossy()))?;
        Ok(PolarsFrameBackend::from_lazy(
            crate::collections::dataframe::GDSLazyFrame::new(lazy),
        ))
    }

    pub fn collect(&self) -> Result<GDSDataFrame, PersistentFrameError> {
        Ok(parquet::read_table(&self.data_path()?)?)
    }
}

fn validate_artifact_id(artifact_id: &str) -> Result<(), PersistentFrameError> {
    if artifact_id.trim().is_empty() {
        return Err(PersistentFrameError::InvalidIdentity(
            "artifact ID must not be empty".to_string(),
        ));
    }
    Ok(())
}

fn version_directory(version: u64) -> String {
    format!("v{version:020}")
}

fn schema_fingerprint(frame: &GDSDataFrame) -> String {
    let schema = frame
        .schema()
        .iter_fields()
        .map(|field| format!("{}:{:?}", field.name(), field.dtype()))
        .collect::<Vec<_>>()
        .join("\n");
    sha256_bytes(schema.as_bytes())
}

fn sha256_file(path: &Path) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex_lower(&hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex_lower(&hasher.finalize())
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        write!(&mut output, "{byte:02x}").expect("writing to String cannot fail");
    }
    output
}

fn sync_directory(path: &Path) -> Result<(), std::io::Error> {
    File::open(path)?.sync_all()
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;

    fn temporary_store() -> (PathBuf, PersistentPolarsStore) {
        let root = std::env::temp_dir().join(format!("gds-polars-store-{}", Uuid::new_v4()));
        (root.clone(), PersistentPolarsStore::new(root))
    }

    #[test]
    fn publishes_verifies_and_lazily_reopens_snapshot() {
        let (root, store) = temporary_store();
        let frame = GDSDataFrame::new(df!("id" => [1_i64, 2], "name" => ["sat", "cit"]).unwrap());
        let snapshot = store
            .publish(
                "dataset:meaning",
                3,
                PolarsFrameBackend::from_dataframe(frame.clone()),
            )
            .unwrap();

        assert_eq!(snapshot.manifest().artifact_id, "dataset:meaning");
        assert_eq!(snapshot.manifest().snapshot_version, 3);
        assert_eq!(snapshot.manifest().row_count, 2);
        assert_eq!(snapshot.collect().unwrap().shape(), (2, 2));
        assert_eq!(
            snapshot.lazy_backend().unwrap().collect().unwrap().shape(),
            (2, 2)
        );
        assert!(snapshot.snapshot_dir().join(MANIFEST_FILE).is_file());
        assert!(snapshot.snapshot_dir().join(DATA_FILE).is_file());

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn immutable_version_refuses_overwrite() {
        let (root, store) = temporary_store();
        let frame = GDSDataFrame::new(df!("id" => [1_i64]).unwrap());
        store
            .publish(
                "graph:nodes",
                1,
                PolarsFrameBackend::from_dataframe(frame.clone()),
            )
            .unwrap();
        let error = store
            .publish("graph:nodes", 1, PolarsFrameBackend::from_dataframe(frame))
            .unwrap_err();
        assert!(matches!(error, PersistentFrameError::AlreadyExists(_)));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn open_detects_data_corruption() {
        let (root, store) = temporary_store();
        let frame = GDSDataFrame::new(df!("id" => [1_i64]).unwrap());
        let snapshot = store
            .publish("graph:nodes", 2, PolarsFrameBackend::from_dataframe(frame))
            .unwrap();
        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(snapshot.data_path().unwrap())
            .unwrap();
        file.write_all(b"corrupt").unwrap();
        file.sync_all().unwrap();

        let error = store.open("graph:nodes", 2).unwrap_err();
        assert!(matches!(
            error,
            PersistentFrameError::ChecksumMismatch { .. }
        ));
        fs::remove_dir_all(root).unwrap();
    }
}
