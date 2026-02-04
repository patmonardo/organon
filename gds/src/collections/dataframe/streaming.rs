//! Streaming adapter for Polars DataFrame (LazyFrame-first pipelines).

use polars::prelude::{DataFrame, IntoLazy, LazyFrame};

use crate::collections::extensions::streaming::{
    StreamingConfig, StreamingError, StreamingSupport,
};

/// Streaming adapter over a Polars DataFrame.
#[derive(Debug, Clone)]
pub struct GDSStreamingFrame {
    df: DataFrame,
    streaming_config: Option<StreamingConfig>,
    is_streaming_enabled: bool,
}

impl GDSStreamingFrame {
    pub fn new(df: DataFrame) -> Self {
        Self {
            df,
            streaming_config: None,
            is_streaming_enabled: false,
        }
    }

    pub fn dataframe(&self) -> &DataFrame {
        &self.df
    }

    pub fn into_inner(self) -> DataFrame {
        self.df
    }
}

impl From<DataFrame> for GDSStreamingFrame {
    fn from(df: DataFrame) -> Self {
        Self::new(df)
    }
}

impl StreamingSupport for GDSStreamingFrame {
    fn enable_streaming(&mut self, config: StreamingConfig) -> Result<(), StreamingError> {
        self.streaming_config = Some(config);
        self.is_streaming_enabled = true;
        Ok(())
    }

    fn disable_streaming(&mut self) {
        self.streaming_config = None;
        self.is_streaming_enabled = false;
    }

    fn is_streaming_enabled(&self) -> bool {
        self.is_streaming_enabled
    }

    fn streaming_config(&self) -> Option<&StreamingConfig> {
        self.streaming_config.as_ref()
    }

    fn stream_lazy(&self) -> LazyFrame {
        self.df.clone().lazy()
    }

    fn collect_streaming(&self) -> Result<DataFrame, StreamingError> {
        self.collect_streaming_lazy(self.stream_lazy())
    }

    fn collect_streaming_lazy(&self, lazy: LazyFrame) -> Result<DataFrame, StreamingError> {
        let _ = self.is_streaming_enabled;
        let _ = &self.streaming_config;
        Ok(lazy.collect()?)
    }
}
