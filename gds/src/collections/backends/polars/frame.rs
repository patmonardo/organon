//! Deferred Polars frame backend used by higher Collections products.

use std::fmt;

use polars::error::PolarsResult;
use polars::prelude::Schema;

use crate::collections::dataframe::{GDSDataFrame, GDSLazyFrame, QueryOptFlags};
use crate::config::CollectionsBackend;

use super::PolarsBackendDescriptor;

/// A backend-owned relational plan.
///
/// Keeping the lazy plan here lets Dataset and a future CoreGraphStore share
/// physical execution without depending on one another.
#[derive(Clone)]
pub struct PolarsFrameBackend {
    descriptor: PolarsBackendDescriptor,
    plan: GDSLazyFrame,
}

impl fmt::Debug for PolarsFrameBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PolarsFrameBackend")
            .field("descriptor", &self.descriptor)
            .finish_non_exhaustive()
    }
}

impl PolarsFrameBackend {
    pub fn from_dataframe(frame: GDSDataFrame) -> Self {
        Self::from_lazy(frame.lazy())
    }

    pub fn from_lazy(plan: GDSLazyFrame) -> Self {
        Self {
            descriptor: PolarsBackendDescriptor::default(),
            plan,
        }
    }

    pub fn with_descriptor(mut self, descriptor: PolarsBackendDescriptor) -> Self {
        self.descriptor = descriptor;
        self
    }

    pub fn descriptor(&self) -> &PolarsBackendDescriptor {
        &self.descriptor
    }

    pub const fn backend(&self) -> CollectionsBackend {
        CollectionsBackend::Polars
    }

    pub fn plan(&self) -> &GDSLazyFrame {
        &self.plan
    }

    pub fn schema(&self) -> PolarsResult<Schema> {
        self.plan.clone().schema()
    }

    pub fn explain(&self, optimized: bool) -> PolarsResult<String> {
        self.plan.explain(optimized)
    }

    pub fn with_optimizations(mut self, optimizations: QueryOptFlags) -> Self {
        self.plan = self.plan.with_optimizations(optimizations);
        self
    }

    pub fn collect(self) -> PolarsResult<GDSDataFrame> {
        self.plan.collect().map(GDSDataFrame::new)
    }

    pub fn collect_with_optimizations(
        self,
        optimizations: QueryOptFlags,
    ) -> PolarsResult<GDSDataFrame> {
        self.plan
            .collect_with_optimizations(optimizations)
            .map(GDSDataFrame::new)
    }
}

#[cfg(test)]
mod tests {
    use polars::df;

    use super::*;

    fn test_frame() -> GDSDataFrame {
        GDSDataFrame::new(df!("id" => [1_i64, 2, 3], "score" => [0.5_f64, 1.5, 2.5]).unwrap())
    }

    #[test]
    fn descriptor_exposes_relational_capabilities() {
        let backend = PolarsFrameBackend::from_dataframe(test_frame());
        let capabilities = backend.descriptor().capabilities();

        assert_eq!(backend.backend(), CollectionsBackend::Polars);
        assert!(capabilities.eager_frames);
        assert!(capabilities.lazy_plans);
        assert!(capabilities.optimizer_control);
        assert!(capabilities.plan_explain);
    }

    #[test]
    fn lazy_plan_is_explainable_and_materializes() {
        let backend = PolarsFrameBackend::from_dataframe(test_frame());
        let explanation = backend.explain(true).unwrap();
        assert!(!explanation.is_empty());

        let materialized = backend.collect().unwrap();
        assert_eq!(materialized.shape(), (3, 2));
    }

    #[test]
    fn optimizer_flags_are_applied_at_backend_boundary() {
        let mut flags = QueryOptFlags::none();
        flags
            .set_predicate_pushdown(true)
            .set_projection_pushdown(true)
            .set_simplify_expression(true);

        let materialized = PolarsFrameBackend::from_dataframe(test_frame())
            .collect_with_optimizations(flags)
            .unwrap();

        assert_eq!(materialized.column_names(), vec!["id", "score"]);
    }
}
