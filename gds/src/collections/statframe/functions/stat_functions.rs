//! Statistical modeling function traits.
//!
//! These traits mirror sklearn-style contracts (fit/transform/predict) but are
//! container-agnostic by design.

pub trait StatTransformer<Input, Output> {
    fn transform(&self, input: Input) -> Output;
}

pub trait StatEstimator<Input, Target, Model> {
    fn fit(&self, input: &[Input], target: &[Target]) -> Model;
}

pub trait StatPredictor<Input, Output> {
    fn predict(&self, input: &[Input]) -> Vec<Output>;
}

pub trait StatFitTransform<Input, Output> {
    fn fit_transform(&self, input: &[Input]) -> Vec<Output>;
}

pub trait StatPipelineStep {
    fn name(&self) -> &str;
}
