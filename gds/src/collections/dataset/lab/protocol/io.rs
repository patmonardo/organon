//! Dataset IO expressions.
//!
//! These are declarative descriptors of dataset sources and formats.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetSource {
    Path(String),
    Url(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetIoExpr {
    source: DatasetSource,
    checksum: Option<String>,
    format: Option<String>,
}

impl DatasetIoExpr {
    pub fn from_path(path: impl Into<String>) -> Self {
        Self {
            source: DatasetSource::Path(path.into()),
            checksum: None,
            format: None,
        }
    }

    pub fn from_url(url: impl Into<String>) -> Self {
        Self {
            source: DatasetSource::Url(url.into()),
            checksum: None,
            format: None,
        }
    }

    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        self.checksum = Some(checksum.into());
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = Some(format.into());
        self
    }

    pub fn source(&self) -> &DatasetSource {
        &self.source
    }

    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }

    pub fn format(&self) -> Option<&str> {
        self.format.as_deref()
    }
}
