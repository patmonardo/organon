//! MetaFrame: the explicit `Model : Feature : Plan` mediation surface.
//!
//! A MetaFrame does not replace [`Dataset`](crate::collections::dataset::Dataset)
//! or wrap an already-materialized result with descriptive metadata. It binds
//! the rational anchors of a [`Plan`](super::Plan) and executes that plan into
//! a DataFrame-backed Dataset while preserving the determining plan beside the
//! appearance it produced.

use crate::collections::dataset::Dataset;

use super::{EvalMode, Plan, PlanEnv, PlanError};

#[derive(Debug, thiserror::Error)]
pub enum MetaFrameError {
    #[error("MetaFrame requires a Model anchor")]
    MissingModel,

    #[error("MetaFrame requires at least one Feature anchor")]
    MissingFeature,

    #[error(transparent)]
    Plan(#[from] PlanError),
}

/// The rational middle of the Dataset pipeline: `Model : Feature : Plan`.
#[derive(Debug, Clone)]
pub struct MetaFrame {
    plan: Plan,
}

impl MetaFrame {
    /// Admit a Plan as a MetaFrame only when its Model and Feature moments are
    /// explicit. The Plan itself remains the third, executable moment.
    pub fn new(plan: Plan) -> Result<Self, MetaFrameError> {
        if plan.synthesis().model_anchor.is_none() {
            return Err(MetaFrameError::MissingModel);
        }
        if plan.synthesis().feature_anchors.is_empty() {
            return Err(MetaFrameError::MissingFeature);
        }
        Ok(Self { plan })
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn model_anchor(&self) -> &str {
        self.plan
            .synthesis()
            .model_anchor
            .as_deref()
            .expect("MetaFrame construction validates its Model anchor")
    }

    pub fn feature_anchors(&self) -> &[String] {
        &self.plan.synthesis().feature_anchors
    }

    /// Execute the mediation without losing the Plan that determined the
    /// resulting DataFrame appearance.
    pub fn materialize(
        &self,
        env: &PlanEnv,
        mode: EvalMode,
    ) -> Result<MetaFrameMaterialization, MetaFrameError> {
        let appearance = self.plan.eval_dataset(env, mode)?;
        Ok(MetaFrameMaterialization {
            meta_frame: self.clone(),
            appearance,
            mode,
        })
    }
}

/// The identity-in-difference required for LogicFrame return: an empirical
/// DataFrame appearance together with the MetaFrame that determined it.
#[derive(Debug, Clone)]
pub struct MetaFrameMaterialization {
    meta_frame: MetaFrame,
    appearance: Dataset,
    mode: EvalMode,
}

impl MetaFrameMaterialization {
    pub fn meta_frame(&self) -> &MetaFrame {
        &self.meta_frame
    }

    pub fn appearance(&self) -> &Dataset {
        &self.appearance
    }

    pub fn mode(&self) -> EvalMode {
        self.mode
    }

    pub fn into_parts(self) -> (Dataset, MetaFrame) {
        (self.appearance, self.meta_frame)
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::dataframe::{col, lit};
    use crate::collections::dataset::Dataset;
    use crate::tbl_def;

    use super::*;

    #[test]
    fn metaframe_requires_model_and_feature_determinations() {
        let dataset = Dataset::new(tbl_def!((value: i64 => [1, 2])).expect("table"));
        let bare_plan = Plan::from_dataset(dataset);

        assert!(matches!(
            MetaFrame::new(bare_plan.clone()),
            Err(MetaFrameError::MissingModel)
        ));
        assert!(matches!(
            MetaFrame::new(bare_plan.with_model_anchor("model:test")),
            Err(MetaFrameError::MissingFeature)
        ));
    }

    #[test]
    fn materialization_preserves_plan_beside_dataframe_appearance() {
        let dataset = Dataset::new(tbl_def!((value: i64 => [1, 2, 3])).expect("table"));
        let plan = Plan::from_dataset(dataset)
            .named("positive-values")
            .with_model_anchor("model:numeric")
            .with_feature_anchor("feature:positive")
            .filter(col("value").gt(lit(1_i64)));
        let meta_frame = MetaFrame::new(plan).expect("admitted MetaFrame");

        let materialized = meta_frame
            .materialize(&PlanEnv::new(), EvalMode::Fit)
            .expect("materialized MetaFrame");

        assert_eq!(materialized.appearance().row_count(), 2);
        assert_eq!(materialized.meta_frame().model_anchor(), "model:numeric");
        assert_eq!(
            materialized.meta_frame().feature_anchors(),
            &["feature:positive".to_string()]
        );
        assert_eq!(materialized.mode(), EvalMode::Fit);
    }
}
