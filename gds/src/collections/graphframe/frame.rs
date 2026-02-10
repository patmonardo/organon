//! GraphFrame DataFrame facade.
//!
//! Eager table entrypoint for graphframe surfaces.

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};

use crate::collections::graphframe::lazy::GraphFrameLazyFrameNameSpace;

#[derive(Debug, Clone)]
pub struct GraphFrameDataFrameNameSpace {
    df: GDSDataFrame,
}

impl GraphFrameDataFrameNameSpace {
    pub fn new(df: GDSDataFrame) -> Self {
        Self { df }
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn lazy(&self) -> GraphFrameLazyFrameNameSpace {
        let lf = GDSLazyFrame::from(self.df.dataframe().clone());
        GraphFrameLazyFrameNameSpace::new(lf)
    }
}

pub trait DataFrameGraphFrameExt {
    fn gf(self) -> GraphFrameDataFrameNameSpace;
}

impl DataFrameGraphFrameExt for GDSDataFrame {
    fn gf(self) -> GraphFrameDataFrameNameSpace {
        GraphFrameDataFrameNameSpace::new(self)
    }
}
