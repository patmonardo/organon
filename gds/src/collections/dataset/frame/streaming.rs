//! Streaming Dataset: DataFrame-first batching over a Dataset.

use polars::prelude::{DataFrame, LazyFrame};

use crate::collections::dataframe::GDSStreamingFrame;
use crate::collections::dataset::feature::Feature;
use crate::collections::dataset::plan::{Plan, Step};
use crate::collections::dataset::Dataset;
use crate::collections::extensions::streaming::{
    StreamingConfig, StreamingError, StreamingSupport,
};

/// Streaming Dataset wrapper over a Polars-backed Dataset.
pub struct StreamingDataset {
    dataset: Dataset,
    batch_size: usize,
    streaming_config: StreamingConfig,
    transform: Option<Box<dyn Fn(LazyFrame) -> LazyFrame + Send + Sync>>,
}

impl StreamingDataset {
    pub fn new(dataset: Dataset, batch_size: usize) -> Self {
        Self {
            dataset,
            batch_size: batch_size.max(1),
            streaming_config: StreamingConfig::default(),
            transform: None,
        }
    }

    pub fn with_config(dataset: Dataset, batch_size: usize, config: StreamingConfig) -> Self {
        Self {
            dataset,
            batch_size: batch_size.max(1),
            streaming_config: config,
            transform: None,
        }
    }

    pub fn with_transform<F>(mut self, transform: F) -> Self
    where
        F: Fn(LazyFrame) -> LazyFrame + Send + Sync + 'static,
    {
        self.transform = Some(Box::new(transform));
        self
    }

    /// Treat a `Plan` as a streaming "model": apply it to each batch.
    ///
    /// Notes:
    /// - Only tabular steps are applied (`filter`, `select`, `with_columns`, `item`).
    /// - If the plan contains a `batch(n)` hint, this sets the batch size.
    pub fn with_plan(mut self, plan: &Plan) -> Self {
        if let Some(n) = plan.steps().iter().rev().find_map(|s| match s {
            Step::Batch(n) => Some(*n),
            _ => None,
        }) {
            self.batch_size = n.max(1);
        }

        self.transform = Some(plan.to_streaming_transform());
        self
    }

    /// Treat a `Feature` as a streaming feature processor.
    pub fn with_feature(self, feature: &Feature) -> Self {
        self.with_plan(feature.plan())
    }

    pub fn transform(&self) -> Option<&(dyn Fn(LazyFrame) -> LazyFrame + Send + Sync)> {
        self.transform.as_deref()
    }

    pub fn dataset(&self) -> &Dataset {
        &self.dataset
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn set_batch_size(&mut self, batch_size: usize) {
        self.batch_size = batch_size.max(1);
    }

    pub fn row_count(&self) -> usize {
        self.dataset.row_count()
    }

    pub fn is_empty(&self) -> bool {
        self.dataset.is_empty()
    }

    pub fn streaming_config(&self) -> &StreamingConfig {
        &self.streaming_config
    }

    pub fn set_streaming_config(&mut self, config: StreamingConfig) {
        self.streaming_config = config;
    }

    pub fn iter(&self) -> StreamingBatchIter<'_> {
        StreamingBatchIter::new(self)
    }

    fn build_batch_frame(&self, offset: usize) -> Result<DataFrame, StreamingError> {
        let batch = self.dataset.slice(offset as i64, self.batch_size);
        let mut streaming = GDSStreamingFrame::from(batch.table().dataframe().clone());
        streaming.enable_streaming(self.streaming_config.clone())?;

        let mut lazy = streaming.stream_lazy();
        if let Some(transform) = self.transform.as_ref() {
            lazy = (transform)(lazy);
        }

        streaming.collect_streaming_lazy(lazy)
    }
}

pub struct StreamingBatchIter<'a> {
    dataset: &'a StreamingDataset,
    offset: usize,
}

impl<'a> StreamingBatchIter<'a> {
    fn new(dataset: &'a StreamingDataset) -> Self {
        Self { dataset, offset: 0 }
    }
}

impl<'a> Iterator for StreamingBatchIter<'a> {
    type Item = Result<DataFrame, StreamingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.dataset.row_count() {
            return None;
        }
        let current = self.offset;
        self.offset += self.dataset.batch_size;
        Some(self.dataset.build_batch_frame(current))
    }
}
