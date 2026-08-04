use super::builtin_component;
use super::ShellComponentCall;
use super::ShellComponentMode;
use super::ShellComponentPlan;
use crate::shell::components::plan::ShellPlanStepRole;

#[derive(Debug, Clone)]
pub struct ShellBfsCallBuilder {
    plan: ShellComponentPlan,
    call: ShellComponentCall,
}

impl ShellBfsCallBuilder {
    pub(crate) fn new(plan: ShellComponentPlan, source: u64) -> Self {
        Self {
            plan,
            call: algorithm_call("bfs").with_input("source", source),
        }
    }

    pub fn target(mut self, target: u64) -> Self {
        self.call = self.call.with_input("target", target);
        self
    }

    pub fn targets(mut self, targets: impl IntoIterator<Item = u64>) -> Self {
        self.call = self
            .call
            .with_input("targets", targets.into_iter().collect::<Vec<_>>());
        self
    }

    pub fn max_depth(mut self, max_depth: u32) -> Self {
        self.call = self.call.with_input("maxDepth", u64::from(max_depth));
        self
    }

    pub fn track_paths(mut self, track_paths: bool) -> Self {
        self.call = self.call.with_input("trackPaths", track_paths);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.call = self.call.with_input("concurrency", concurrency as u64);
        self
    }

    pub fn delta(mut self, delta: usize) -> Self {
        self.call = self.call.with_input("delta", delta as u64);
        self
    }

    pub fn stream(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Stream)
    }

    pub fn stats(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Stats)
    }

    pub fn estimate(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Estimate)
    }

    pub fn mutate(self, output_property: impl Into<String>) -> ShellComponentPlan {
        self.with_output_property(output_property)
            .finish(ShellComponentMode::Mutate)
    }

    pub fn write(self, output_property: impl Into<String>) -> ShellComponentPlan {
        self.with_output_property(output_property)
            .finish(ShellComponentMode::Write)
    }

    fn with_output_property(mut self, output_property: impl Into<String>) -> Self {
        self.call = self
            .call
            .with_input("outputProperty", output_property.into());
        self
    }

    fn finish(mut self, mode: ShellComponentMode) -> ShellComponentPlan {
        self.call.mode = mode;
        self.plan
            .push_with_role(self.call, ShellPlanStepRole::Other)
    }
}

#[derive(Debug, Clone)]
pub struct ShellDijkstraCallBuilder {
    plan: ShellComponentPlan,
    call: ShellComponentCall,
}

impl ShellDijkstraCallBuilder {
    pub(crate) fn new(plan: ShellComponentPlan, source: u64) -> Self {
        Self {
            plan,
            call: algorithm_call("dijkstra").with_input("source", source),
        }
    }

    pub fn target(mut self, target: u64) -> Self {
        self.call = self.call.with_input("target", target);
        self
    }

    pub fn targets(mut self, targets: impl IntoIterator<Item = u64>) -> Self {
        self.call = self
            .call
            .with_input("targets", targets.into_iter().collect::<Vec<_>>());
        self
    }

    pub fn weight_property(mut self, weight_property: impl Into<String>) -> Self {
        self.call = self
            .call
            .with_input("weightProperty", weight_property.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.call = self.call.with_input("direction", direction.into());
        self
    }

    pub fn track_relationships(mut self, track_relationships: bool) -> Self {
        self.call = self
            .call
            .with_input("trackRelationships", track_relationships);
        self
    }

    pub fn concurrency(mut self, concurrency: usize) -> Self {
        self.call = self.call.with_input("concurrency", concurrency as u64);
        self
    }

    pub fn stream(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Stream)
    }

    pub fn stats(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Stats)
    }

    pub fn estimate(self) -> ShellComponentPlan {
        self.finish(ShellComponentMode::Estimate)
    }

    pub fn mutate(self, output_property: impl Into<String>) -> ShellComponentPlan {
        self.with_output_property(output_property)
            .finish(ShellComponentMode::Mutate)
    }

    pub fn write(self, output_property: impl Into<String>) -> ShellComponentPlan {
        self.with_output_property(output_property)
            .finish(ShellComponentMode::Write)
    }

    fn with_output_property(mut self, output_property: impl Into<String>) -> Self {
        self.call = self
            .call
            .with_input("outputProperty", output_property.into());
        self
    }

    fn finish(mut self, mode: ShellComponentMode) -> ShellComponentPlan {
        self.call.mode = mode;
        self.plan
            .push_with_role(self.call, ShellPlanStepRole::Other)
    }
}

fn algorithm_call(name: &str) -> ShellComponentCall {
    builtin_component(name)
        .expect("typed Shell algorithm must have a builtin descriptor")
        .call(ShellComponentMode::Estimate)
}
