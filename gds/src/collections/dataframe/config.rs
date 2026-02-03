//! DataFrame configuration helpers (py-polars inspired).

use std::collections::BTreeMap;
use std::env;
use std::path::Path;

use serde::{Deserialize, Serialize};

const POLARS_CFG_ENV_VARS: &[&str] = &[
    "POLARS_WARN_UNSTABLE",
    "POLARS_FMT_MAX_COLS",
    "POLARS_FMT_MAX_ROWS",
    "POLARS_FMT_NUM_DECIMAL",
    "POLARS_FMT_NUM_GROUP_SEPARATOR",
    "POLARS_FMT_NUM_LEN",
    "POLARS_FMT_STR_LEN",
    "POLARS_FMT_TABLE_CELL_ALIGNMENT",
    "POLARS_FMT_TABLE_CELL_LIST_LEN",
    "POLARS_FMT_TABLE_CELL_NUMERIC_ALIGNMENT",
    "POLARS_FMT_TABLE_DATAFRAME_SHAPE_BELOW",
    "POLARS_FMT_TABLE_FORMATTING",
    "POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES",
    "POLARS_FMT_TABLE_HIDE_COLUMN_NAMES",
    "POLARS_FMT_TABLE_HIDE_COLUMN_SEPARATOR",
    "POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION",
    "POLARS_FMT_TABLE_INLINE_COLUMN_DATA_TYPE",
    "POLARS_FMT_TABLE_ROUNDED_CORNERS",
    "POLARS_STREAMING_CHUNK_SIZE",
    "POLARS_TABLE_WIDTH",
    "POLARS_VERBOSE",
    "POLARS_MAX_EXPR_DEPTH",
    "POLARS_ENGINE_AFFINITY",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConfigSnapshot {
    environment: BTreeMap<String, Option<String>>,
}

/// Minimal config facade backed by environment variables.
#[derive(Debug, Clone, Default)]
pub struct Config;

impl Config {
    /// Save current config state as JSON.
    pub fn save() -> Result<String, serde_json::Error> {
        let mut environment = BTreeMap::new();
        for key in POLARS_CFG_ENV_VARS {
            environment.insert((*key).to_string(), env::var(key).ok());
        }
        let snapshot = ConfigSnapshot { environment };
        serde_json::to_string(&snapshot)
    }

    /// Save current config state to a file.
    pub fn save_to_file(path: impl AsRef<Path>) -> std::io::Result<()> {
        let json =
            Self::save().map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        std::fs::write(path, json)
    }

    /// Load config from a JSON string, applying environment variables.
    pub fn load(cfg: &str) -> Result<(), serde_json::Error> {
        let snapshot: ConfigSnapshot = serde_json::from_str(cfg)?;
        for (key, value) in snapshot.environment {
            match value {
                Some(val) => env::set_var(key, val),
                None => env::remove_var(key),
            }
        }
        Ok(())
    }

    /// Load config from a JSON file.
    pub fn load_from_file(path: impl AsRef<Path>) -> std::io::Result<()> {
        let cfg = std::fs::read_to_string(path)?;
        Self::load(&cfg).map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))
    }

    /// Clear all known config env vars.
    pub fn restore_defaults() {
        for key in POLARS_CFG_ENV_VARS {
            env::remove_var(key);
        }
    }

    /// Return current config env values.
    pub fn state() -> BTreeMap<String, Option<String>> {
        let mut state = BTreeMap::new();
        for key in POLARS_CFG_ENV_VARS {
            state.insert((*key).to_string(), env::var(key).ok());
        }
        state
    }
}
