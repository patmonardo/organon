//! Plan namespace for dataset-level DSL authoring.

use crate::collections::dataframe::GDSExpr as Expr;

use crate::collections::dataset::lab::protocol::dataop::DatasetDataOpExpr;
use crate::collections::dataset::plan::EvalMode;
use crate::collections::dataset::plan::Plan;
use crate::collections::dataset::plan::PlanEnv;
use crate::collections::dataset::Dataset;
use crate::collections::dataset::DatasetSplit;

#[derive(Debug, Clone, Default)]
pub struct PlanNs;

impl PlanNs {
    pub fn from_var(name: impl Into<String>) -> Plan {
        Plan::from_var(name)
    }

    pub fn from_dataset(dataset: Dataset) -> Plan {
        Plan::from_dataset(dataset)
    }

    pub fn named(plan: Plan, name: impl Into<String>) -> Plan {
        plan.named(name)
    }

    pub fn env() -> PlanEnv {
        PlanEnv::new()
    }

    pub fn env_preview_rows(n: usize) -> PlanEnv {
        PlanEnv::new().with_preview_rows(n)
    }

    pub fn bind_dataset(env: PlanEnv, name: impl Into<String>, dataset: Dataset) -> PlanEnv {
        env.bind_dataset(name, dataset)
    }

    pub fn preview() -> EvalMode {
        EvalMode::Preview
    }

    pub fn fit() -> EvalMode {
        EvalMode::Fit
    }

    pub fn filter(plan: Plan, predicate: Expr) -> Plan {
        plan.filter(predicate)
    }

    pub fn select<I>(plan: Plan, exprs: I) -> Plan
    where
        I: IntoIterator<Item = Expr>,
    {
        plan.select(exprs)
    }

    pub fn with_columns<I>(plan: Plan, exprs: I) -> Plan
    where
        I: IntoIterator<Item = Expr>,
    {
        plan.with_columns(exprs)
    }

    pub fn batch(plan: Plan, n: usize) -> Plan {
        plan.batch(n)
    }

    pub fn split(plan: Plan, split: DatasetSplit) -> Plan {
        plan.split(split)
    }

    pub fn dataop(plan: Plan, op: DatasetDataOpExpr) -> Plan {
        plan.dataop(op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::collections::dataframe::col;
    use crate::collections::dataframe::lit;

    #[test]
    fn test_plan_ns_builds_lazy_expr_pipeline() {
        let plan = PlanNs::from_var("ds")
            .named("text-flow")
            .with_columns([col("text").str().to_lowercase().alias("text_lower")])
            .filter(col("id").gt(lit(1i64)))
            .select([col("id"), col("text_lower")]);

        assert_eq!(plan.name(), Some("text-flow"));
        assert_eq!(plan.steps().len(), 3);
    }

    #[test]
    fn test_plan_ns_env_builders() {
        let env = PlanNs::env_preview_rows(12);
        assert_eq!(env.preview_rows(), 12);
        assert!(matches!(PlanNs::preview(), EvalMode::Preview));
        assert!(matches!(PlanNs::fit(), EvalMode::Fit));
    }
}
