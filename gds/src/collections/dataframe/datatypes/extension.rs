//! Extension type registry (seed pass), inspired by polars.datatypes.extension.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionType {
    Storage,
    Class(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionRegistryError {
    AlreadyRegistered(String),
    NotRegistered(String),
    InvalidArguments(String),
}

fn registry() -> &'static RwLock<HashMap<String, ExtensionType>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, ExtensionType>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register the extension type for the given extension name.
pub fn register_extension_type(
    ext_name: &str,
    ext_class: Option<&str>,
    as_storage: bool,
) -> Result<(), ExtensionRegistryError> {
    let mut map = registry().write().expect("extension registry poisoned");
    if map.contains_key(ext_name) {
        return Err(ExtensionRegistryError::AlreadyRegistered(
            ext_name.to_string(),
        ));
    }

    if as_storage {
        if ext_class.is_some() {
            return Err(ExtensionRegistryError::InvalidArguments(
                "cannot specify ext_class when as_storage is true".to_string(),
            ));
        }
        map.insert(ext_name.to_string(), ExtensionType::Storage);
    } else {
        let class_name = ext_class.ok_or_else(|| {
            ExtensionRegistryError::InvalidArguments(
                "ext_class is required when as_storage is false".to_string(),
            )
        })?;
        map.insert(
            ext_name.to_string(),
            ExtensionType::Class(class_name.to_string()),
        );
    }

    Ok(())
}

/// Unregister the extension type for the given extension name.
pub fn unregister_extension_type(ext_name: &str) -> Result<(), ExtensionRegistryError> {
    let mut map = registry().write().expect("extension registry poisoned");
    if map.remove(ext_name).is_none() {
        return Err(ExtensionRegistryError::NotRegistered(ext_name.to_string()));
    }
    Ok(())
}

/// Get the extension type for the given extension name.
pub fn get_extension_type(ext_name: &str) -> Option<ExtensionType> {
    let map = registry().read().expect("extension registry poisoned");
    map.get(ext_name).cloned()
}
