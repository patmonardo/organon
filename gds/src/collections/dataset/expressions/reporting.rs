//! Dataset reporting expressions.
//!
//! Declarative reporting requests that can be executed by reporting backends.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatasetReportKind {
    Summary,
    Profile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetReportExpr {
    kind: DatasetReportKind,
    title: Option<String>,
}

impl DatasetReportExpr {
    pub fn summary() -> Self {
        Self {
            kind: DatasetReportKind::Summary,
            title: None,
        }
    }

    pub fn profile() -> Self {
        Self {
            kind: DatasetReportKind::Profile,
            title: None,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn kind(&self) -> &DatasetReportKind {
        &self.kind
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }
}
