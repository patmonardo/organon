//! Feature role taxonomy and descriptor access objects.
//!
//! Companion to [`crate::collections::dataset::feature`]. This module names
//! the **four roles** a Feature plays per the doctrine
//! (`gds/doc/SEMANTIC-DATASET-FIVE-FOLD.md` §"The four roles of a Feature")
//! as plain data, plus a [`FeatureDescriptor`] row struct and a
//! [`FeatureFrame`] access object over `GDSDataFrame`.
//!
//! This module is **purely descriptive**. It does not execute features and
//! does not touch the Plan-backed `Feature` runtime in `feature.rs`. The
//! intent is that the existing `Feature` is "Feature in its Projection
//! role"; Binder / Reentrancy / Annotation roles get typed here first as
//! data, then wired into the runtime in later phases.
//!
//! Status: **Phase 2**. Role enum + descriptor + descriptor-frame only.
//! No lattice ops, no execution, no Polars-dtype mapping for `FeatureDType`.

use polars::prelude::{DataFrame, NamedFrom, PolarsError, Series};

use crate::collections::dataframe::GDSDataFrame;

/// A feature's name. Newtype so that name-as-data and name-as-Polars-column
/// can diverge later without a rename.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FeatureName(pub String);

impl FeatureName {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for FeatureName {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl From<String> for FeatureName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Codomain type of a Feature. Conservative enum: explicit cases for the
/// types the doctrine names, plus a `Custom(String)` escape hatch. Mapping
/// these to concrete Polars dtypes is deferred — that is part of the
/// computation-graph terminology question still open in the doctrine.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureDType {
    String,
    Int64,
    Float64,
    Bool,
    /// Typed offset range into a `Source` (see `document::Span`).
    Span,
    /// A reference to a `Source` (carried as `ContentHash`).
    Source,
    /// A reference to a `Document` row.
    Document,
    /// A nested `Model`.
    Model,
    /// Anything else, named opaquely.
    Custom(String),
}

impl FeatureDType {
    pub fn name(&self) -> &str {
        match self {
            Self::String => "string",
            Self::Int64 => "int64",
            Self::Float64 => "float64",
            Self::Bool => "bool",
            Self::Span => "span",
            Self::Source => "source",
            Self::Document => "document",
            Self::Model => "model",
            Self::Custom(s) => s.as_str(),
        }
    }
}

/// Scope identifier for a Binder-role Feature (Ch10 discourse referent).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScopeId(pub String);

impl ScopeId {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Mandatory provenance tuple carried by an Annotation-role Feature.
///
/// Provenance is part of identity. Two Annotations with different
/// `annotator` or `guideline_version` are different Features even if they
/// address the same path with the same codomain.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Provenance {
    pub layer: String,
    pub annotator: String,
    pub guideline_version: String,
    pub derivation: String,
}

impl Provenance {
    pub fn new(
        layer: impl Into<String>,
        annotator: impl Into<String>,
        guideline_version: impl Into<String>,
        derivation: impl Into<String>,
    ) -> Self {
        Self {
            layer: layer.into(),
            annotator: annotator.into(),
            guideline_version: guideline_version.into(),
            derivation: derivation.into(),
        }
    }
}

/// One Feature, four roles. The roles are *projections of one definition*,
/// not four parallel types — see doctrine §"The four roles of a Feature".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRole {
    /// Addresses a Model; codomain is a value or another Model.
    Projection,
    /// Addresses a Model with an unbound, scope-local codomain
    /// (Ch10 discourse referent).
    Binder { scope: ScopeId },
    /// Two paths bound to the same cell (anaphora; AVM coreference).
    Reentrancy { peer: FeatureName },
    /// Addresses a Document; codomain is a value plus mandatory provenance.
    Annotation { provenance: Provenance },
}

impl FeatureRole {
    /// Stable string tag for the role kind (`"projection"`, `"binder"`,
    /// `"reentrancy"`, `"annotation"`). Used by [`FeatureFrame`] for the
    /// `role` column.
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Projection => "projection",
            Self::Binder { .. } => "binder",
            Self::Reentrancy { .. } => "reentrancy",
            Self::Annotation { .. } => "annotation",
        }
    }
}

/// Row-shaped descriptor of a Feature: name + codomain + role.
///
/// This is the data form of "what is this Feature." Constructing one does
/// **not** make the Feature evaluable; that is the Plan-backed
/// [`crate::collections::dataset::feature::Feature`]'s job. A descriptor is
/// what a `Model` schema, a `LanguageModel` binding context, or a `Corpus`
/// annotation index needs to talk about Features without executing them.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FeatureDescriptor {
    pub name: FeatureName,
    pub dtype: FeatureDType,
    pub role: FeatureRole,
}

impl FeatureDescriptor {
    pub fn projection(name: impl Into<FeatureName>, dtype: FeatureDType) -> Self {
        Self {
            name: name.into(),
            dtype,
            role: FeatureRole::Projection,
        }
    }

    pub fn binder(name: impl Into<FeatureName>, dtype: FeatureDType, scope: ScopeId) -> Self {
        Self {
            name: name.into(),
            dtype,
            role: FeatureRole::Binder { scope },
        }
    }

    pub fn reentrancy(
        name: impl Into<FeatureName>,
        dtype: FeatureDType,
        peer: FeatureName,
    ) -> Self {
        Self {
            name: name.into(),
            dtype,
            role: FeatureRole::Reentrancy { peer },
        }
    }

    pub fn annotation(
        name: impl Into<FeatureName>,
        dtype: FeatureDType,
        provenance: Provenance,
    ) -> Self {
        Self {
            name: name.into(),
            dtype,
            role: FeatureRole::Annotation { provenance },
        }
    }
}

/// Canonical column names for a [`FeatureFrame`].
pub mod columns {
    pub const NAME: &str = "name";
    pub const DTYPE: &str = "dtype";
    pub const ROLE: &str = "role";
    /// Populated only for `Binder` rows; empty string otherwise.
    pub const SCOPE: &str = "scope";
    /// Populated only for `Reentrancy` rows; empty string otherwise.
    pub const PEER: &str = "peer";
    pub const PROV_LAYER: &str = "prov_layer";
    pub const PROV_ANNOTATOR: &str = "prov_annotator";
    pub const PROV_GUIDELINE_VERSION: &str = "prov_guideline_version";
    pub const PROV_DERIVATION: &str = "prov_derivation";
}

/// Frame-level access object over a table of [`FeatureDescriptor`] rows.
///
/// One row per descriptor. Role-specific columns (`scope`, `peer`,
/// `prov_*`) are populated only for the rows whose role uses them; the
/// other rows carry empty strings. This is the storage shape; richer
/// schema-with-nullability is a later concern.
#[derive(Debug, Clone)]
pub struct FeatureFrame {
    df: GDSDataFrame,
}

impl FeatureFrame {
    /// Wrap an existing `GDSDataFrame`, validating that the canonical
    /// columns are present.
    pub fn from_dataframe(df: GDSDataFrame) -> Result<Self, PolarsError> {
        let names: std::collections::HashSet<String> = df.column_names().into_iter().collect();
        for required in [
            columns::NAME,
            columns::DTYPE,
            columns::ROLE,
            columns::SCOPE,
            columns::PEER,
            columns::PROV_LAYER,
            columns::PROV_ANNOTATOR,
            columns::PROV_GUIDELINE_VERSION,
            columns::PROV_DERIVATION,
        ] {
            if !names.contains(required) {
                return Err(PolarsError::ColumnNotFound(required.into()));
            }
        }
        Ok(Self { df })
    }

    /// Build a `FeatureFrame` from an iterator of descriptors.
    pub fn from_descriptors<I>(descriptors: I) -> Result<Self, PolarsError>
    where
        I: IntoIterator<Item = FeatureDescriptor>,
    {
        let mut name = Vec::<String>::new();
        let mut dtype = Vec::<String>::new();
        let mut role = Vec::<String>::new();
        let mut scope = Vec::<String>::new();
        let mut peer = Vec::<String>::new();
        let mut p_layer = Vec::<String>::new();
        let mut p_annot = Vec::<String>::new();
        let mut p_guide = Vec::<String>::new();
        let mut p_deriv = Vec::<String>::new();

        for d in descriptors {
            name.push(d.name.0);
            dtype.push(d.dtype.name().to_owned());
            role.push(d.role.kind().to_owned());
            match d.role {
                FeatureRole::Projection => {
                    scope.push(String::new());
                    peer.push(String::new());
                    p_layer.push(String::new());
                    p_annot.push(String::new());
                    p_guide.push(String::new());
                    p_deriv.push(String::new());
                }
                FeatureRole::Binder { scope: s } => {
                    scope.push(s.0);
                    peer.push(String::new());
                    p_layer.push(String::new());
                    p_annot.push(String::new());
                    p_guide.push(String::new());
                    p_deriv.push(String::new());
                }
                FeatureRole::Reentrancy { peer: pn } => {
                    scope.push(String::new());
                    peer.push(pn.0);
                    p_layer.push(String::new());
                    p_annot.push(String::new());
                    p_guide.push(String::new());
                    p_deriv.push(String::new());
                }
                FeatureRole::Annotation { provenance } => {
                    scope.push(String::new());
                    peer.push(String::new());
                    p_layer.push(provenance.layer);
                    p_annot.push(provenance.annotator);
                    p_guide.push(provenance.guideline_version);
                    p_deriv.push(provenance.derivation);
                }
            }
        }

        let df = DataFrame::new_infer_height(vec![
            Series::new(columns::NAME.into(), name).into(),
            Series::new(columns::DTYPE.into(), dtype).into(),
            Series::new(columns::ROLE.into(), role).into(),
            Series::new(columns::SCOPE.into(), scope).into(),
            Series::new(columns::PEER.into(), peer).into(),
            Series::new(columns::PROV_LAYER.into(), p_layer).into(),
            Series::new(columns::PROV_ANNOTATOR.into(), p_annot).into(),
            Series::new(columns::PROV_GUIDELINE_VERSION.into(), p_guide).into(),
            Series::new(columns::PROV_DERIVATION.into(), p_deriv).into(),
        ])?;

        Ok(Self {
            df: GDSDataFrame::new(df),
        })
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn len(&self) -> usize {
        self.df.height()
    }

    pub fn is_empty(&self) -> bool {
        self.df.height() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p() -> Provenance {
        Provenance::new("ner", "alice", "v1.2", "manual")
    }

    #[test]
    fn role_kinds_are_stable() {
        assert_eq!(FeatureRole::Projection.kind(), "projection");
        assert_eq!(
            FeatureRole::Binder {
                scope: ScopeId::new("s1")
            }
            .kind(),
            "binder"
        );
        assert_eq!(
            FeatureRole::Reentrancy {
                peer: FeatureName::new("x")
            }
            .kind(),
            "reentrancy"
        );
        assert_eq!(
            FeatureRole::Annotation { provenance: p() }.kind(),
            "annotation"
        );
    }

    #[test]
    fn descriptor_constructors_carry_role() {
        let d1 = FeatureDescriptor::projection("token", FeatureDType::String);
        let d2 = FeatureDescriptor::binder(
            "ref",
            FeatureDType::Custom("dref".into()),
            ScopeId::new("s1"),
        );
        let d3 =
            FeatureDescriptor::reentrancy("anaphor", FeatureDType::Model, FeatureName::new("ref"));
        let d4 = FeatureDescriptor::annotation("ner_tag", FeatureDType::String, p());

        assert!(matches!(d1.role, FeatureRole::Projection));
        assert!(matches!(d2.role, FeatureRole::Binder { .. }));
        assert!(matches!(d3.role, FeatureRole::Reentrancy { .. }));
        assert!(matches!(d4.role, FeatureRole::Annotation { .. }));
    }

    #[test]
    fn dtype_names_are_stable() {
        assert_eq!(FeatureDType::String.name(), "string");
        assert_eq!(FeatureDType::Span.name(), "span");
        assert_eq!(FeatureDType::Custom("avm".into()).name(), "avm");
    }

    #[test]
    fn feature_frame_round_trips_all_four_roles() {
        let frame = FeatureFrame::from_descriptors(vec![
            FeatureDescriptor::projection("token", FeatureDType::String),
            FeatureDescriptor::binder(
                "ref",
                FeatureDType::Custom("dref".into()),
                ScopeId::new("s1"),
            ),
            FeatureDescriptor::reentrancy("anaphor", FeatureDType::Model, FeatureName::new("ref")),
            FeatureDescriptor::annotation("ner_tag", FeatureDType::String, p()),
        ])
        .expect("construction");

        assert_eq!(frame.len(), 4);
        let cols: std::collections::HashSet<String> =
            frame.dataframe().column_names().into_iter().collect();
        for c in [
            columns::NAME,
            columns::DTYPE,
            columns::ROLE,
            columns::SCOPE,
            columns::PEER,
            columns::PROV_LAYER,
            columns::PROV_ANNOTATOR,
            columns::PROV_GUIDELINE_VERSION,
            columns::PROV_DERIVATION,
        ] {
            assert!(cols.contains(c), "missing column {c}");
        }
    }

    #[test]
    fn feature_frame_rejects_missing_columns() {
        let df = GDSDataFrame::new(
            DataFrame::new_infer_height(vec![Series::new("name".into(), vec!["x"]).into()])
                .unwrap(),
        );
        let err = FeatureFrame::from_dataframe(df).unwrap_err();
        assert!(matches!(err, PolarsError::ColumnNotFound(_)));
    }
}
