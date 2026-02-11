//! Seed dataset resources (download/unzip/catalog helpers).

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use zip::ZipArchive;

use crate::collections::catalog::CatalogError;
use crate::collections::catalog::CollectionsCatalogDiskEntry;
use crate::collections::dataset::catalog::DatasetCatalog;
use crate::collections::dataset::download::{copy_local, download_url, DownloadReport};
use crate::collections::dataset::error::DatasetIoError;
use crate::collections::dataset::io::detect_format_from_path;

const DATA_HOME_ENV: &str = "GDS_DATA_DIRECTORY";

const FILE_PROTOCOL: &str = "file";
const NLTK_PROTOCOL: &str = "nltk";

#[derive(Debug, Clone, Copy)]
pub struct DatasetResource {
    pub name: &'static str,
    pub urls: &'static [&'static str],
    pub sha256: &'static str,
}

#[derive(Debug, Clone)]
pub struct DatasetResourceReport {
    pub name: String,
    pub archive_path: PathBuf,
    pub data_dir: PathBuf,
    pub bytes: u64,
    pub source: String,
}

pub fn split_resource_url(resource_url: &str) -> Result<(String, String), DatasetIoError> {
    let (protocol, path) = resource_url
        .split_once(':')
        .ok_or_else(|| DatasetIoError::Unsupported("resource url missing protocol".to_string()))?;

    if protocol.len() == 1
        && protocol.chars().all(|c| c.is_ascii_alphabetic())
        && (path.starts_with('/') || path.starts_with('\\'))
    {
        let path = format!("/{protocol}:{}", path.trim_start_matches(['/', '\\']));
        return Ok((FILE_PROTOCOL.to_string(), path));
    }

    let path = if protocol == NLTK_PROTOCOL {
        path.to_string()
    } else if protocol == FILE_PROTOCOL {
        if path.starts_with('/') {
            format!("/{}", path.trim_start_matches('/'))
        } else {
            path.to_string()
        }
    } else {
        path.trim_start_matches('/').to_string()
    };

    Ok((protocol.to_string(), path))
}

pub fn normalize_resource_url(resource_url: &str) -> Result<String, DatasetIoError> {
    let (mut protocol, mut name) = match split_resource_url(resource_url) {
        Ok((protocol, name)) => (protocol, name),
        Err(_) => {
            reject_unsafe_no_protocol(resource_url)?;
            (NLTK_PROTOCOL.to_string(), resource_url.to_string())
        }
    };

    if protocol == FILE_PROTOCOL && !resource_url.to_ascii_lowercase().starts_with("file:") {
        reject_unsafe_no_protocol(resource_url)?;
    }

    if protocol == NLTK_PROTOCOL {
        if is_windows_drive_path(&name) {
            return Err(DatasetIoError::Unsupported(format!(
                "unsafe resource path: {resource_url:?}"
            )));
        }
        if Path::new(&name).is_absolute() {
            protocol = "file://".to_string();
            name = normalize_resource_name(&name, false, None);
        } else {
            protocol = "nltk:".to_string();
            name = normalize_resource_name(&name, true, None);
        }
    } else if protocol == FILE_PROTOCOL {
        protocol = "file://".to_string();
        name = normalize_resource_name(&name, false, None);
    } else {
        protocol = format!("{}://", protocol);
    }

    Ok(format!("{protocol}{name}"))
}

pub fn normalize_resource_name(
    resource_name: &str,
    allow_relative: bool,
    relative_path: Option<&Path>,
) -> String {
    let mut name = resource_name.replace('\\', "/");
    let is_dir = name.ends_with('/') || name.ends_with(std::path::MAIN_SEPARATOR);

    if cfg!(windows) {
        name = name.trim_start_matches('/').to_string();
    } else {
        let trimmed = name.trim_start_matches('/');
        name = if resource_name.starts_with('/') {
            format!("/{trimmed}")
        } else {
            trimmed.to_string()
        };
    }

    if allow_relative {
        name = normalize_posix_path(&name);
    } else {
        let base = relative_path.unwrap_or_else(|| Path::new("."));
        let joined = base.join(&name);
        name = normalize_posix_path(&joined.to_string_lossy());
    }

    if cfg!(windows) && Path::new(&name).is_absolute() && !name.starts_with('/') {
        name = format!("/{name}");
    }

    if is_dir && !name.ends_with('/') {
        name.push('/');
    }

    name
}

pub fn list_resources() -> &'static [DatasetResource] {
    DATASET_RESOURCES
}

pub fn data_home() -> Result<PathBuf, DatasetIoError> {
    if let Some(env_path) = env::var_os(DATA_HOME_ENV) {
        let path = PathBuf::from(env_path);
        return ensure_dir(path);
    }
    if let Some(home) = env::var_os("HOME") {
        return ensure_dir(PathBuf::from(home).join("gds_data"));
    }
    ensure_dir(PathBuf::from("gds_data"))
}

pub fn data_home_with(path: impl AsRef<Path>) -> Result<PathBuf, DatasetIoError> {
    ensure_dir(path.as_ref().to_path_buf())
}

pub fn resource_dir(resource: &DatasetResource, data_home: &Path) -> PathBuf {
    data_home.join(resource.name)
}

pub fn fetch_resource(
    name: &str,
    data_home_override: Option<impl AsRef<Path>>,
) -> Result<DatasetResourceReport, DatasetIoError> {
    let resource = DATASET_RESOURCES
        .iter()
        .find(|r| r.name == name)
        .ok_or_else(|| DatasetIoError::Unsupported(format!("unknown resource: {name}")))?;

    let data_home = match data_home_override {
        Some(path) => data_home_with(path)?,
        None => data_home()?,
    };

    let dataset_dir = resource_dir(resource, &data_home);
    ensure_dir(dataset_dir.clone())?;

    let archive_path = dataset_dir.join(format!("{}.zip", resource.name));
    let data_dir = dataset_dir.join(resource.name);

    let mut report = if archive_path.exists() {
        let bytes = archive_path
            .metadata()
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            .len();
        DatasetResourceReport {
            name: resource.name.to_string(),
            archive_path: archive_path.clone(),
            data_dir: data_dir.clone(),
            bytes,
            source: "local-cache".to_string(),
        }
    } else {
        download_first(resource.urls, &archive_path)?
    };

    if !resource.sha256.is_empty() {
        let actual = sha256_file(&archive_path)?;
        if actual != resource.sha256 {
            return Err(DatasetIoError::Archive(format!(
                "sha256 mismatch for {} (got {}, expected {})",
                resource.name, actual, resource.sha256
            )));
        }
    }

    if !data_dir.exists() {
        unzip_archive(&archive_path, &dataset_dir)?;
    }

    report.data_dir = data_dir;
    Ok(report)
}

pub fn catalog_resource_tables(
    catalog: &mut DatasetCatalog,
    resource: &DatasetResource,
    data_dir: impl AsRef<Path>,
) -> Result<Vec<CollectionsCatalogDiskEntry>, CatalogError> {
    let mut entries = Vec::new();
    let data_dir = data_dir.as_ref();
    let files = list_data_files(data_dir, 8)?;
    for file in files {
        let name = resource_table_name(resource, &file);
        let format = Some(detect_format_from_path(&file));
        let entry = catalog.register_table_path(&name, &file, format, None)?;
        entries.push(entry);
    }
    Ok(entries)
}

fn download_first(
    urls: &[&str],
    destination: &Path,
) -> Result<DatasetResourceReport, DatasetIoError> {
    let mut last_error = None;
    for url in urls {
        match download_resource_url(url, destination) {
            Ok(DownloadReport {
                source,
                destination,
                bytes,
            }) => {
                return Ok(DatasetResourceReport {
                    name: destination
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("dataset")
                        .to_string(),
                    archive_path: destination.clone(),
                    data_dir: destination.parent().unwrap_or(Path::new(".")).to_path_buf(),
                    bytes,
                    source,
                });
            }
            Err(err) => last_error = Some(err),
        }
    }
    Err(last_error.unwrap_or_else(|| DatasetIoError::Http("no urls provided".to_string())))
}

fn download_resource_url(url: &str, destination: &Path) -> Result<DownloadReport, DatasetIoError> {
    let normalized = normalize_resource_url(url)?;
    if normalized.starts_with("file://") {
        let path = normalized.trim_start_matches("file://");
        let path = Path::new(path);
        return copy_local(path, destination);
    }
    if normalized.starts_with("http://") || normalized.starts_with("https://") {
        return download_url(&normalized, destination);
    }
    if normalized.starts_with("nltk:") {
        return Err(DatasetIoError::Unsupported(format!(
            "unsupported resource protocol: {normalized}"
        )));
    }
    Err(DatasetIoError::Unsupported(format!(
        "unsupported resource protocol: {normalized}"
    )))
}

fn unzip_archive(archive_path: &Path, destination: &Path) -> Result<(), DatasetIoError> {
    let file = File::open(archive_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut archive = ZipArchive::new(file).map_err(|e| DatasetIoError::Archive(e.to_string()))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| DatasetIoError::Archive(e.to_string()))?;
        let out_path = destination.join(file.mangled_name());

        if file.name().ends_with('/') {
            fs::create_dir_all(&out_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
            continue;
        }

        if let Some(parent) = out_path.parent() {
            fs::create_dir_all(parent).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        }

        let mut outfile = File::create(&out_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| DatasetIoError::Archive(e.to_string()))?;
        outfile
            .write_all(&buffer)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
    }

    Ok(())
}

fn sha256_file(path: &Path) -> Result<String, DatasetIoError> {
    let mut file = File::open(path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|e| DatasetIoError::Io(e.to_string()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(hex_lower(&hasher.finalize()))
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(&format!("{:02x}", b));
    }
    out
}

fn ensure_dir(path: PathBuf) -> Result<PathBuf, DatasetIoError> {
    fs::create_dir_all(&path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    Ok(path)
}

fn resource_table_name(resource: &DatasetResource, file: &Path) -> String {
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("table")
        .replace('-', "_");
    format!("{}_{}", resource.name, stem)
}

fn list_data_files(path: &Path, depth: usize) -> Result<Vec<PathBuf>, CatalogError> {
    let mut out = Vec::new();
    if depth == 0 {
        return Ok(out);
    }
    for entry in fs::read_dir(path).map_err(|e| CatalogError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| CatalogError::Io(e.to_string()))?;
        let meta = entry
            .metadata()
            .map_err(|e| CatalogError::Io(e.to_string()))?;
        let entry_path = entry.path();
        if meta.is_dir() {
            out.extend(list_data_files(&entry_path, depth - 1)?);
            continue;
        }
        let ext = entry_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let ext = ext.to_ascii_lowercase();
        if matches!(
            ext.as_str(),
            "csv" | "parquet" | "json" | "ndjson" | "ipc" | "arrow"
        ) {
            out.push(entry_path);
        }
    }
    Ok(out)
}

fn reject_unsafe_no_protocol(resource_url: &str) -> Result<(), DatasetIoError> {
    if is_unsafe_no_protocol(resource_url) {
        return Err(DatasetIoError::Unsupported(format!(
            "unsafe resource path: {resource_url:?}"
        )));
    }
    Ok(())
}

fn is_unsafe_no_protocol(resource_url: &str) -> bool {
    resource_url.contains("../")
        || resource_url.ends_with("..")
        || resource_url.starts_with('/')
        || resource_url.contains('\\')
        || is_windows_drive_path(resource_url)
}

fn is_windows_drive_path(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    let Some(second) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() || second != ':' {
        return false;
    }
    let third = chars.next();
    matches!(third, Some('/') | Some('\\'))
}

fn normalize_posix_path(path: &str) -> String {
    let mut parts = Vec::new();
    let mut absolute = false;

    if path.starts_with('/') {
        absolute = true;
    }

    for part in path.split('/') {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            if !parts.is_empty() && parts.last() != Some(&"..") {
                parts.pop();
            } else if !absolute {
                parts.push("..");
            }
            continue;
        }
        parts.push(part);
    }

    let mut out = if absolute {
        String::from("/")
    } else {
        String::new()
    };
    out.push_str(&parts.join("/"));
    out
}

const DATASET_RESOURCES: &[DatasetResource] = &[
    DatasetResource {
        name: "bike_sharing",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/bike_sharing.zip",
            "https://osf.io/download/3z4qc",
        ],
        sha256: "33745414801712034cf1d8615d7f086bba411ea8e44bfffefc0c6f23cb8afb83",
    },
    DatasetResource {
        name: "california_housing",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/california_housing.zip",
            "https://osf.io/download/dxew5",
        ],
        sha256: "86545bc4250a0c5d1762c52934f3760bc50b5d618f4aeb3a67bb66416909e919",
    },
    DatasetResource {
        name: "country_happiness",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/country_happiness.zip",
            "https://osf.io/download/8a6wm",
        ],
        sha256: "10b35da781a13a94dedcfeb43b291d16677b06a781e9b88a780f04ad173b422d",
    },
    DatasetResource {
        name: "credit_fraud",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/credit_fraud.zip",
            "https://osf.io/download/y8qg5",
        ],
        sha256: "ec40d370a275d4bd2637d4c617120e91e2e7946d23c93b1a1ea7df824ee1e514",
    },
    DatasetResource {
        name: "drug_directory",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/drug_directory.zip",
            "https://osf.io/download/rtgk5",
        ],
        sha256: "0c3885894baf02fc787109801ec2c34cc25cd4a31e0066a16941b74157474887",
    },
    DatasetResource {
        name: "employee_salaries",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/employee_salaries.zip",
            "https://osf.io/download/c592f",
        ],
        sha256: "4b4919f38d921014cb1fd24ad302f44bccc55d1eeeeb8482902b09d9b43576cb",
    },
    DatasetResource {
        name: "flight_delays",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/flight_delays.zip",
            "https://osf.io/download/45xu3",
        ],
        sha256: "f26ed72db5792dba3c6f0c32bdd83438e49b1e6e007a6e4e467f805207b2e4ab",
    },
    DatasetResource {
        name: "medical_charge",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/medical_charge.zip",
            "https://osf.io/download/g8cvw",
        ],
        sha256: "4850651103b7c7580587aafaccc05ca7a31125767d4da662e87890346f984b93",
    },
    DatasetResource {
        name: "midwest_survey",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/midwest_survey.zip",
            "https://osf.io/download/aedqu",
        ],
        sha256: "94d5005402e5e72c2d5ce62f4d3115742dd12190db85920159b2ed8f44df7fc2",
    },
    DatasetResource {
        name: "movielens",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/movielens.zip",
            "https://osf.io/download/z5yqv",
        ],
        sha256: "d6b22c707f9a1605da5616ac1a601f4090467c1a02fa663195a42cf80f32fd57",
    },
    DatasetResource {
        name: "open_payments",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/open_payments.zip",
            "https://osf.io/download/a7p9w",
        ],
        sha256: "ead65dcb8d45ec16ab30dd71025c3cfc5730128f85eeb19ce6f56670923f04ba",
    },
    DatasetResource {
        name: "road_safety",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/road_safety.zip",
            "https://osf.io/download/bae6d",
        ],
        sha256: "035df2a644ba2be52022aa9ca5f41790a24cbd9c76434c3e5224c8c218cf6f87",
    },
    DatasetResource {
        name: "toxicity",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/toxicity.zip",
            "https://osf.io/download/zebm7",
        ],
        sha256: "ee187c119925ea4cdb9abd7f0f3758159f042e71b172cafe5b784d79c7590ce3",
    },
    DatasetResource {
        name: "traffic_violations",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/traffic_violations.zip",
            "https://osf.io/download/av4cw",
        ],
        sha256: "b52a145a34b1866b6deee7cbfd1c0b8d2af3bbd53fb5658b155f752ac7d85ce0",
    },
    DatasetResource {
        name: "videogame_sales",
        urls: &[
            "https://github.com/skrub-data/skrub-data-files/raw/refs/heads/main/videogame_sales.zip",
            "https://osf.io/download/g2fw4",
        ],
        sha256: "3e6d995af025b8a3a1dc64983aa9d53c3c6e72150644d42c58c8b86888c3dacd",
    },
];
