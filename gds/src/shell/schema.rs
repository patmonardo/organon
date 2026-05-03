//! Shell schema keeper built on feature structures.

use std::collections::{BTreeMap, HashMap};

use crate::collections::dataframe::GDSDataFrame;
use crate::collections::dataset::feature::featstruct::{
    subsumes_featstruct, unify_featstruct, FeatDict, FeatStruct, FeatValue,
};

/// Root schema carried by the GDS Shell.
///
/// This is the immediate DataFrame schema lifted into a feature-structure
/// algebra. ProgramFeatures can declare mediated program intent, but this
/// structure is where compatibility, unification, and schema keeping begin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellSchema {
    feature_structure: FeatStruct,
}

impl ShellSchema {
    pub fn new(feature_structure: FeatStruct) -> Self {
        Self { feature_structure }
    }

    pub fn from_dataframe(dataframe: &GDSDataFrame) -> Self {
        let mut columns = BTreeMap::new();
        for (index, (name, dtype)) in dataframe
            .column_names()
            .into_iter()
            .zip(dataframe.dtypes())
            .enumerate()
        {
            let mut column = BTreeMap::new();
            column.insert("role".to_string(), FeatValue::text("column"));
            column.insert("dtype".to_string(), FeatValue::text(format!("{dtype:?}")));
            column.insert("index".to_string(), FeatValue::Number(index as i64));
            columns.insert(name, FeatValue::Struct(FeatStruct::Dict(column)));
        }

        let mut shape = BTreeMap::new();
        shape.insert(
            "rows".to_string(),
            FeatValue::Number(dataframe.row_count() as i64),
        );
        shape.insert(
            "columns".to_string(),
            FeatValue::Number(dataframe.column_count() as i64),
        );

        let mut root = FeatDict::new();
        root.insert(
            "register".to_string(),
            FeatValue::text("immediate-dataframe"),
        );
        root.insert(
            "columns".to_string(),
            FeatValue::Struct(FeatStruct::Dict(columns)),
        );
        root.insert(
            "shape".to_string(),
            FeatValue::Struct(FeatStruct::Dict(shape)),
        );

        Self::new(FeatStruct::Dict(root))
    }

    pub fn feature_structure(&self) -> &FeatStruct {
        &self.feature_structure
    }

    pub fn column_names(&self) -> Vec<String> {
        self.columns_dict()
            .map(|columns| columns.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn type_constraints(&self) -> HashMap<String, String> {
        let Some(columns) = self.columns_dict() else {
            return HashMap::new();
        };

        columns
            .iter()
            .filter_map(|(name, value)| match value {
                FeatValue::Struct(FeatStruct::Dict(column)) => match column.get("dtype") {
                    Some(FeatValue::Text(dtype)) => Some((name.clone(), dtype.clone())),
                    _ => None,
                },
                _ => None,
            })
            .collect()
    }

    pub fn unify(&self, other: &ShellSchema) -> Option<ShellSchema> {
        unify_featstruct(&self.feature_structure, &other.feature_structure, None).map(Self::new)
    }

    pub fn subsumes(&self, other: &ShellSchema) -> bool {
        subsumes_featstruct(&self.feature_structure, &other.feature_structure)
    }

    fn columns_dict(&self) -> Option<&FeatDict> {
        match &self.feature_structure {
            FeatStruct::Dict(root) => match root.get("columns") {
                Some(FeatValue::Struct(FeatStruct::Dict(columns))) => Some(columns),
                _ => None,
            },
            _ => None,
        }
    }
}
