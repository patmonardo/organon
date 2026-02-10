//! StatFrame DataFrame facade.
//!
//! Eager table entrypoint for statframe surfaces.

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame};

use crate::collections::statframe::lazy::StatFrameLazyFrameNameSpace;

#[derive(Debug, Clone)]
pub struct StatFrameDataFrameNameSpace {
    df: GDSDataFrame,
}

impl StatFrameDataFrameNameSpace {
    pub fn new(df: GDSDataFrame) -> Self {
        Self { df }
    }

    pub fn dataframe(&self) -> &GDSDataFrame {
        &self.df
    }

    pub fn into_dataframe(self) -> GDSDataFrame {
        self.df
    }

    pub fn lazy(&self) -> StatFrameLazyFrameNameSpace {
        let lf = GDSLazyFrame::from(self.df.dataframe().clone());
        StatFrameLazyFrameNameSpace::new(lf)
    }
}

pub trait DataFrameStatFrameExt {
    fn sf(self) -> StatFrameDataFrameNameSpace;
}

impl DataFrameStatFrameExt for GDSDataFrame {
    fn sf(self) -> StatFrameDataFrameNameSpace {
        StatFrameDataFrameNameSpace::new(self)
    }
}
