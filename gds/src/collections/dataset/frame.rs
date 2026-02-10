//! Dataset-level DataFrame facade.
//!
//! This module completes the dataset-side 2×2 matrix by providing a small
//! eager `DataFrame` entrypoint.

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};
use crate::collections::dataset::lazy::DatasetLazyFrameNameSpace;

#[derive(Debug, Clone)]
pub struct DatasetDataFrameNameSpace {
	df: GDSDataFrame,
}

impl DatasetDataFrameNameSpace {
	pub fn new(df: GDSDataFrame) -> Self {
		Self { df }
	}

	pub fn dataframe(&self) -> &GDSDataFrame {
		&self.df
	}

	pub fn into_dataframe(self) -> GDSDataFrame {
		self.df
	}

	/// Convert this DataFrame into a LazyFrame and enter the dataset lazy namespace.
	pub fn lazy(&self) -> DatasetLazyFrameNameSpace {
		let lf = GDSLazyFrame::from(self.df.dataframe().clone());
		DatasetLazyFrameNameSpace::new(lf)
	}
}

pub trait DataFrameDatasetExt {
	fn ds(self) -> DatasetDataFrameNameSpace;
}

impl DataFrameDatasetExt for GDSDataFrame {
	fn ds(self) -> DatasetDataFrameNameSpace {
		DatasetDataFrameNameSpace::new(self)
	}
}

