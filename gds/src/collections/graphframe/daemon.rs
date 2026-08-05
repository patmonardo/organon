//! GraphFrame TaskDaemon adaptor.
//!
//! This module turns GraphFrame agent contracts into route-aware TaskDaemon
//! execution without bypassing the unified TaskJob lifecycle.

use std::fmt;

use crate::collections::graphframe::lazy::GraphExecutionIntent;
use crate::collections::graphframe::lazy::GraphTaskDaemonRoute;
use crate::collections::graphframe::lazy::GraphTaskDaemonSubmission;
use crate::procedures::pipelines::LocalPipelinesProcedureFacade;
use crate::procedures::GraphFacade;
use crate::shell::ShellComponentPlan;
use crate::shell::ShellProcedureEvaluator;
use crate::shell::ShellProcedurePlanResult;
use crate::task::concurrency::TerminationFlag;
use crate::task::daemon::TaskDaemon;
use crate::task::evaluator::TaskEvaluator;
use crate::task::evaluator::TaskExecutionContext;
use crate::task::job::TaskJob;
use crate::task::job::TaskJobReceipt;
use crate::task::spec::TaskMonitoringLevel;
use crate::task::spec::TaskSpecError;
use crate::types::prelude::DefaultGraphStore;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Default)]
pub struct GraphTaskDaemon {
    daemon: TaskDaemon,
}

pub struct GraphTaskRouteEvaluators<
    'a,
    AlgorithmEvaluator,
    PipelineEvaluator,
    StoreApiEvaluator,
    HybridEvaluator,
> where
    AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
    PipelineEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    StoreApiEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    HybridEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
{
    algorithm: &'a AlgorithmEvaluator,
    pipeline: &'a PipelineEvaluator,
    store_api: &'a StoreApiEvaluator,
    hybrid: &'a HybridEvaluator,
}

pub struct GraphTaskDaemonRuntimeBundle {
    algorithm: ShellProcedureEvaluator,
    pipeline: ShellProcedureEvaluator,
    store_api: ShellProcedureEvaluator,
    hybrid: ShellProcedureEvaluator,
}

pub trait GraphTaskGraphFacadeProvider {
    fn provide_graph_facade(
        &self,
        route: GraphTaskDaemonRoute,
        profile: GraphTaskRuntimeProfile,
    ) -> GraphFacade;
}

pub trait GraphTaskPipelinesFacadeProvider {
    fn provide_pipelines_facade(
        &self,
        route: GraphTaskDaemonRoute,
        profile: GraphTaskRuntimeProfile,
    ) -> LocalPipelinesProcedureFacade;
}

pub trait GraphTaskRuntimeInjector {
    fn inject(
        &self,
        route: GraphTaskDaemonRoute,
        profile: GraphTaskRuntimeProfile,
    ) -> ShellProcedureEvaluator;
}

pub struct GraphTaskStoreGraphFacadeProvider {
    store: Arc<DefaultGraphStore>,
}

impl GraphTaskStoreGraphFacadeProvider {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self { store }
    }
}

impl GraphTaskGraphFacadeProvider for GraphTaskStoreGraphFacadeProvider {
    fn provide_graph_facade(
        &self,
        _route: GraphTaskDaemonRoute,
        _profile: GraphTaskRuntimeProfile,
    ) -> GraphFacade {
        GraphFacade::new(Arc::clone(&self.store))
    }
}

pub struct GraphTaskLocalPipelinesFacadeProvider;

impl GraphTaskPipelinesFacadeProvider for GraphTaskLocalPipelinesFacadeProvider {
    fn provide_pipelines_facade(
        &self,
        _route: GraphTaskDaemonRoute,
        _profile: GraphTaskRuntimeProfile,
    ) -> LocalPipelinesProcedureFacade {
        LocalPipelinesProcedureFacade::default()
    }
}

pub struct GraphTaskDependencyInjector<'a, GraphProvider, PipelinesProvider>
where
    GraphProvider: GraphTaskGraphFacadeProvider + ?Sized,
    PipelinesProvider: GraphTaskPipelinesFacadeProvider + ?Sized,
{
    graph_provider: &'a GraphProvider,
    pipelines_provider: &'a PipelinesProvider,
}

pub struct GraphTaskRuntimeDependencyAdapter<'a> {
    graph_provider: &'a dyn GraphTaskGraphFacadeProvider,
    pipelines_provider: &'a dyn GraphTaskPipelinesFacadeProvider,
}

impl<'a, GraphProvider, PipelinesProvider>
    GraphTaskDependencyInjector<'a, GraphProvider, PipelinesProvider>
where
    GraphProvider: GraphTaskGraphFacadeProvider + ?Sized,
    PipelinesProvider: GraphTaskPipelinesFacadeProvider + ?Sized,
{
    pub fn new(
        graph_provider: &'a GraphProvider,
        pipelines_provider: &'a PipelinesProvider,
    ) -> Self {
        Self {
            graph_provider,
            pipelines_provider,
        }
    }
}

impl<'a> GraphTaskRuntimeDependencyAdapter<'a> {
    pub fn new(
        graph_provider: &'a dyn GraphTaskGraphFacadeProvider,
        pipelines_provider: &'a dyn GraphTaskPipelinesFacadeProvider,
    ) -> Self {
        Self {
            graph_provider,
            pipelines_provider,
        }
    }

    pub fn runtime_bundle(&self, profile: GraphTaskRuntimeProfile) -> GraphTaskDaemonRuntimeBundle {
        let injector =
            GraphTaskDependencyInjector::new(self.graph_provider, self.pipelines_provider);
        GraphTaskDaemonRuntimeBundle::from_injector(profile, &injector)
    }
}

impl<'a, GraphProvider, PipelinesProvider> GraphTaskRuntimeInjector
    for GraphTaskDependencyInjector<'a, GraphProvider, PipelinesProvider>
where
    GraphProvider: GraphTaskGraphFacadeProvider + ?Sized,
    PipelinesProvider: GraphTaskPipelinesFacadeProvider + ?Sized,
{
    fn inject(
        &self,
        route: GraphTaskDaemonRoute,
        profile: GraphTaskRuntimeProfile,
    ) -> ShellProcedureEvaluator {
        let graph = self.graph_provider.provide_graph_facade(route, profile);
        let pipelines = self
            .pipelines_provider
            .provide_pipelines_facade(route, profile);
        ShellProcedureEvaluator::new(graph, pipelines)
    }
}

pub struct GraphTaskFacadeRuntimeInjector {
    graph_provider: GraphTaskStoreGraphFacadeProvider,
    pipelines_provider: GraphTaskLocalPipelinesFacadeProvider,
}

impl GraphTaskFacadeRuntimeInjector {
    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self {
            graph_provider: GraphTaskStoreGraphFacadeProvider::new(store),
            pipelines_provider: GraphTaskLocalPipelinesFacadeProvider,
        }
    }
}

impl GraphTaskRuntimeInjector for GraphTaskFacadeRuntimeInjector {
    fn inject(
        &self,
        route: GraphTaskDaemonRoute,
        profile: GraphTaskRuntimeProfile,
    ) -> ShellProcedureEvaluator {
        GraphTaskDependencyInjector::new(&self.graph_provider, &self.pipelines_provider)
            .inject(route, profile)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GraphTaskRuntimeProfile {
    Analytics,
    Pipelines,
    StoreManagement,
    #[default]
    HybridBalanced,
    EngineeringStrict,
}

impl GraphTaskRuntimeProfile {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Analytics => "analytics",
            Self::Pipelines => "pipelines",
            Self::StoreManagement => "store_management",
            Self::HybridBalanced => "hybrid_balanced",
            Self::EngineeringStrict => "engineering_strict",
        }
    }

    pub fn allows_route(self, route: GraphTaskDaemonRoute) -> bool {
        match self {
            Self::Analytics => matches!(route, GraphTaskDaemonRoute::Algorithm),
            Self::Pipelines => matches!(route, GraphTaskDaemonRoute::Pipeline),
            Self::StoreManagement => {
                matches!(
                    route,
                    GraphTaskDaemonRoute::StoreApi | GraphTaskDaemonRoute::Hybrid
                )
            }
            Self::HybridBalanced | Self::EngineeringStrict => {
                !matches!(route, GraphTaskDaemonRoute::None)
            }
        }
    }

    pub fn monitoring_level_for(self, route: GraphTaskDaemonRoute) -> TaskMonitoringLevel {
        match self {
            Self::Analytics => TaskMonitoringLevel::Detailed,
            Self::Pipelines => TaskMonitoringLevel::Basic,
            Self::StoreManagement => TaskMonitoringLevel::Detailed,
            Self::HybridBalanced => match route {
                GraphTaskDaemonRoute::StoreApi | GraphTaskDaemonRoute::Hybrid => {
                    TaskMonitoringLevel::Detailed
                }
                GraphTaskDaemonRoute::Algorithm | GraphTaskDaemonRoute::Pipeline => {
                    TaskMonitoringLevel::Basic
                }
                GraphTaskDaemonRoute::None => TaskMonitoringLevel::Off,
            },
            Self::EngineeringStrict => TaskMonitoringLevel::Detailed,
        }
    }

    pub fn runtime_bundle(self, store: Arc<DefaultGraphStore>) -> GraphTaskDaemonRuntimeBundle {
        GraphTaskDaemonRuntimeBundle::for_profile(store, self)
    }

    pub fn runtime_bundle_with_injector(
        self,
        injector: &impl GraphTaskRuntimeInjector,
    ) -> GraphTaskDaemonRuntimeBundle {
        GraphTaskDaemonRuntimeBundle::from_injector(self, injector)
    }

    pub fn runtime_bundle_with_providers<GraphProvider, PipelinesProvider>(
        self,
        graph_provider: &GraphProvider,
        pipelines_provider: &PipelinesProvider,
    ) -> GraphTaskDaemonRuntimeBundle
    where
        GraphProvider: GraphTaskGraphFacadeProvider,
        PipelinesProvider: GraphTaskPipelinesFacadeProvider,
    {
        GraphTaskDaemonRuntimeBundle::from_providers(self, graph_provider, pipelines_provider)
    }
}

impl GraphTaskDaemonRuntimeBundle {
    pub fn for_profile(store: Arc<DefaultGraphStore>, profile: GraphTaskRuntimeProfile) -> Self {
        // Profile-specific runtime specialization is centralized here so
        // enterprise CoreGraphStore wiring can evolve without daemon API churn.
        match profile {
            GraphTaskRuntimeProfile::Analytics
            | GraphTaskRuntimeProfile::Pipelines
            | GraphTaskRuntimeProfile::StoreManagement
            | GraphTaskRuntimeProfile::HybridBalanced
            | GraphTaskRuntimeProfile::EngineeringStrict => {
                let injector = GraphTaskFacadeRuntimeInjector::new(store);
                Self::from_injector(profile, &injector)
            }
        }
    }

    pub fn from_injector(
        profile: GraphTaskRuntimeProfile,
        injector: &impl GraphTaskRuntimeInjector,
    ) -> Self {
        Self {
            algorithm: injector.inject(GraphTaskDaemonRoute::Algorithm, profile),
            pipeline: injector.inject(GraphTaskDaemonRoute::Pipeline, profile),
            store_api: injector.inject(GraphTaskDaemonRoute::StoreApi, profile),
            hybrid: injector.inject(GraphTaskDaemonRoute::Hybrid, profile),
        }
    }

    pub fn from_providers<GraphProvider, PipelinesProvider>(
        profile: GraphTaskRuntimeProfile,
        graph_provider: &GraphProvider,
        pipelines_provider: &PipelinesProvider,
    ) -> Self
    where
        GraphProvider: GraphTaskGraphFacadeProvider,
        PipelinesProvider: GraphTaskPipelinesFacadeProvider,
    {
        let injector = GraphTaskDependencyInjector::new(graph_provider, pipelines_provider);
        Self::from_injector(profile, &injector)
    }

    pub fn new(store: Arc<DefaultGraphStore>) -> Self {
        Self {
            algorithm: Self::runtime_for_store(Arc::clone(&store)),
            pipeline: Self::runtime_for_store(Arc::clone(&store)),
            store_api: Self::runtime_for_store(Arc::clone(&store)),
            hybrid: Self::runtime_for_store(store),
        }
    }

    pub fn from_route_runtimes(
        algorithm: ShellProcedureEvaluator,
        pipeline: ShellProcedureEvaluator,
        store_api: ShellProcedureEvaluator,
        hybrid: ShellProcedureEvaluator,
    ) -> Self {
        Self {
            algorithm,
            pipeline,
            store_api,
            hybrid,
        }
    }

    fn runtime_for_store(store: Arc<DefaultGraphStore>) -> ShellProcedureEvaluator {
        let graph = GraphFacade::new(store);
        let pipelines = LocalPipelinesProcedureFacade::default();
        ShellProcedureEvaluator::new(graph, pipelines)
    }

    pub fn route_evaluators(
        &self,
    ) -> GraphTaskRouteEvaluators<
        '_,
        ShellProcedureEvaluator,
        ShellProcedureEvaluator,
        ShellProcedureEvaluator,
        ShellProcedureEvaluator,
    > {
        GraphTaskRouteEvaluators::new(
            &self.algorithm,
            &self.pipeline,
            &self.store_api,
            &self.hybrid,
        )
    }
}

impl<'a, AlgorithmEvaluator, PipelineEvaluator, StoreApiEvaluator, HybridEvaluator>
    GraphTaskRouteEvaluators<
        'a,
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >
where
    AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
    PipelineEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    StoreApiEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    HybridEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
{
    pub fn new(
        algorithm: &'a AlgorithmEvaluator,
        pipeline: &'a PipelineEvaluator,
        store_api: &'a StoreApiEvaluator,
        hybrid: &'a HybridEvaluator,
    ) -> Self {
        Self {
            algorithm,
            pipeline,
            store_api,
            hybrid,
        }
    }
}

#[derive(Debug)]
pub enum GraphTaskDaemonRunError<E: fmt::Display> {
    UnsupportedRoute(GraphTaskDaemonRoute),
    RoutePlanMismatch {
        route: GraphTaskDaemonRoute,
        surfaces: String,
    },
    DisallowedComponent {
        profile: GraphTaskRuntimeProfile,
        route: GraphTaskDaemonRoute,
        component: String,
    },
    UnsupportedProfileRoute {
        profile: GraphTaskRuntimeProfile,
        route: GraphTaskDaemonRoute,
    },
    Delegate(E),
}

impl<E: fmt::Display> fmt::Display for GraphTaskDaemonRunError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedRoute(route) => {
                write!(
                    f,
                    "unsupported graph task daemon route `{}`",
                    route.as_str()
                )
            }
            Self::RoutePlanMismatch { route, surfaces } => {
                write!(
                    f,
                    "graph route `{}` is incompatible with execution surfaces `{}`",
                    route.as_str(),
                    surfaces
                )
            }
            Self::DisallowedComponent {
                profile,
                route,
                component,
            } => {
                write!(
                    f,
                    "runtime profile `{}` disallows component `{}` on route `{}`",
                    profile.as_str(),
                    component,
                    route.as_str()
                )
            }
            Self::UnsupportedProfileRoute { profile, route } => {
                write!(
                    f,
                    "runtime profile `{}` disallows graph route `{}`",
                    profile.as_str(),
                    route.as_str()
                )
            }
            Self::Delegate(error) => write!(f, "{error}"),
        }
    }
}

impl GraphTaskDaemon {
    pub fn new() -> Self {
        Self {
            daemon: TaskDaemon::new(),
        }
    }

    pub fn task_daemon(&self) -> TaskDaemon {
        self.daemon
    }

    pub fn run_with_evaluator<Evaluator>(
        &self,
        submission: GraphTaskDaemonSubmission,
        evaluator: &Evaluator,
        termination: TerminationFlag,
    ) -> Result<TaskJobReceipt<Evaluator::Output>, TaskSpecError>
    where
        Evaluator: TaskEvaluator<GraphExecutionIntent>,
    {
        let route = submission.route();
        let job = submission.into_job()?;
        let routed = GraphTaskRoutedEvaluator { route, evaluator };
        Ok(self.daemon.run(job, &routed, termination))
    }

    pub fn run_with_route_evaluators<
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >(
        &self,
        submission: GraphTaskDaemonSubmission,
        evaluators: GraphTaskRouteEvaluators<
            '_,
            AlgorithmEvaluator,
            PipelineEvaluator,
            StoreApiEvaluator,
            HybridEvaluator,
        >,
        termination: TerminationFlag,
    ) -> Result<TaskJobReceipt<AlgorithmEvaluator::Output>, TaskSpecError>
    where
        AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
        PipelineEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
        StoreApiEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
        HybridEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
    {
        let route = submission.route();
        let job = submission.into_job()?;
        Ok(self.run_job_with_route_evaluators(job, route, None, evaluators, termination))
    }

    pub fn run_with_runtime_bundle(
        &self,
        submission: GraphTaskDaemonSubmission,
        bundle: &GraphTaskDaemonRuntimeBundle,
        termination: TerminationFlag,
    ) -> Result<TaskJobReceipt<ShellProcedurePlanResult>, TaskSpecError> {
        self.run_with_runtime_profile(
            submission,
            bundle,
            GraphTaskRuntimeProfile::default(),
            termination,
        )
    }

    pub fn run_with_runtime_profile(
        &self,
        submission: GraphTaskDaemonSubmission,
        bundle: &GraphTaskDaemonRuntimeBundle,
        profile: GraphTaskRuntimeProfile,
        termination: TerminationFlag,
    ) -> Result<TaskJobReceipt<ShellProcedurePlanResult>, TaskSpecError> {
        let route = submission.route();
        let monitoring = profile.monitoring_level_for(route);
        let job = submission
            .task_frame()
            .clone()
            .with_monitoring_level(monitoring)
            .into_job(submission.owner().to_string())?;
        Ok(self.run_job_with_route_evaluators(
            job,
            route,
            Some(profile),
            bundle.route_evaluators(),
            termination,
        ))
    }

    fn run_job_with_route_evaluators<
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >(
        &self,
        job: TaskJob<GraphExecutionIntent>,
        route: GraphTaskDaemonRoute,
        profile: Option<GraphTaskRuntimeProfile>,
        evaluators: GraphTaskRouteEvaluators<
            '_,
            AlgorithmEvaluator,
            PipelineEvaluator,
            StoreApiEvaluator,
            HybridEvaluator,
        >,
        termination: TerminationFlag,
    ) -> TaskJobReceipt<AlgorithmEvaluator::Output>
    where
        AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
        PipelineEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
        StoreApiEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
        HybridEvaluator: TaskEvaluator<
            GraphExecutionIntent,
            Output = AlgorithmEvaluator::Output,
            Error = AlgorithmEvaluator::Error,
        >,
    {
        let routed = GraphTaskRouteRegistryEvaluator {
            route,
            profile,
            evaluators,
        };
        self.daemon.run(job, &routed, termination)
    }
}

fn route_plan_surface_flags(plan: &ShellComponentPlan) -> (bool, bool, bool) {
    (
        plan.has_algorithm_components(),
        plan.has_pipeline_components(),
        plan.has_store_api_components(),
    )
}

fn route_plan_surfaces_label(plan: &ShellComponentPlan) -> String {
    let (algorithm, pipeline, store_api) = route_plan_surface_flags(plan);
    let mut surfaces = Vec::new();
    if algorithm {
        surfaces.push("algorithm");
    }
    if pipeline {
        surfaces.push("pipeline");
    }
    if store_api {
        surfaces.push("store_api");
    }
    if surfaces.is_empty() {
        return "none".to_string();
    }
    surfaces.join("+")
}

fn route_matches_plan(route: GraphTaskDaemonRoute, plan: &ShellComponentPlan) -> bool {
    let (algorithm, pipeline, store_api) = route_plan_surface_flags(plan);
    let count = [algorithm, pipeline, store_api]
        .iter()
        .filter(|enabled| **enabled)
        .count();

    match route {
        GraphTaskDaemonRoute::None => false,
        GraphTaskDaemonRoute::Algorithm => algorithm && !pipeline && !store_api,
        GraphTaskDaemonRoute::Pipeline => pipeline && !algorithm && !store_api,
        GraphTaskDaemonRoute::StoreApi => store_api && !algorithm && !pipeline,
        GraphTaskDaemonRoute::Hybrid => count >= 2,
    }
}

fn component_allowed_for_profile(
    profile: GraphTaskRuntimeProfile,
    route: GraphTaskDaemonRoute,
    component: &str,
) -> bool {
    if !matches!(profile, GraphTaskRuntimeProfile::EngineeringStrict) {
        return true;
    }

    match route {
        GraphTaskDaemonRoute::Algorithm => component == "gds.algorithms.centrality.pagerank",
        GraphTaskDaemonRoute::Pipeline => {
            component == "gds.beta.pipeline.nodeClassification.train"
                || component == "gds.beta.pipeline.nodeClassification.predict.stream"
        }
        GraphTaskDaemonRoute::StoreApi => {
            component.starts_with("gds.store.catalog.")
                || component.starts_with("gds.store.memory.")
                || component.starts_with("gds.store.graph.")
        }
        GraphTaskDaemonRoute::Hybrid => {
            component == "gds.algorithms.centrality.pagerank"
                || component.starts_with("gds.store.catalog.")
                || component.starts_with("gds.store.memory.")
                || component.starts_with("gds.store.graph.")
        }
        GraphTaskDaemonRoute::None => false,
    }
}

fn validate_profile_components(
    profile: GraphTaskRuntimeProfile,
    route: GraphTaskDaemonRoute,
    plan: &ShellComponentPlan,
) -> Result<(), String> {
    for call in plan.calls() {
        let component = call.component.as_str();
        if !component_allowed_for_profile(profile, route, component) {
            return Err(component.to_string());
        }
    }
    Ok(())
}

struct GraphTaskRoutedEvaluator<'a, Evaluator> {
    route: GraphTaskDaemonRoute,
    evaluator: &'a Evaluator,
}

impl<'a, Evaluator> GraphTaskRoutedEvaluator<'a, Evaluator>
where
    Evaluator: TaskEvaluator<GraphExecutionIntent>,
{
    fn route_stage<'b>(&self, context: &'b TaskExecutionContext<'_>) -> &'b str {
        context
            .spec()
            .workflow()
            .frames()
            .iter()
            .find(|frame| frame.pipeline() == "pipeline::GraphFrameCompute")
            .map(|frame| frame.pipeline())
            .or_else(|| {
                context
                    .spec()
                    .workflow()
                    .frames()
                    .first()
                    .map(|frame| frame.pipeline())
            })
            .unwrap_or("pipeline::GraphFrameCompute")
    }
}

struct GraphTaskRouteRegistryEvaluator<
    'a,
    AlgorithmEvaluator,
    PipelineEvaluator,
    StoreApiEvaluator,
    HybridEvaluator,
> where
    AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
    PipelineEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    StoreApiEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    HybridEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
{
    route: GraphTaskDaemonRoute,
    profile: Option<GraphTaskRuntimeProfile>,
    evaluators: GraphTaskRouteEvaluators<
        'a,
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >,
}

impl<'a, AlgorithmEvaluator, PipelineEvaluator, StoreApiEvaluator, HybridEvaluator>
    GraphTaskRouteRegistryEvaluator<
        'a,
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >
where
    AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
    PipelineEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    StoreApiEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    HybridEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
{
    fn route_stage<'b>(&self, context: &'b TaskExecutionContext<'_>) -> &'b str {
        context
            .spec()
            .workflow()
            .frames()
            .iter()
            .find(|frame| frame.pipeline() == "pipeline::GraphFrameCompute")
            .map(|frame| frame.pipeline())
            .or_else(|| {
                context
                    .spec()
                    .workflow()
                    .frames()
                    .first()
                    .map(|frame| frame.pipeline())
            })
            .unwrap_or("pipeline::GraphFrameCompute")
    }
}

impl<'a, AlgorithmEvaluator, PipelineEvaluator, StoreApiEvaluator, HybridEvaluator>
    TaskEvaluator<GraphExecutionIntent>
    for GraphTaskRouteRegistryEvaluator<
        'a,
        AlgorithmEvaluator,
        PipelineEvaluator,
        StoreApiEvaluator,
        HybridEvaluator,
    >
where
    AlgorithmEvaluator: TaskEvaluator<GraphExecutionIntent>,
    PipelineEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    StoreApiEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
    HybridEvaluator: TaskEvaluator<
        GraphExecutionIntent,
        Output = AlgorithmEvaluator::Output,
        Error = AlgorithmEvaluator::Error,
    >,
{
    type Output = AlgorithmEvaluator::Output;
    type Error = GraphTaskDaemonRunError<AlgorithmEvaluator::Error>;

    fn error_classification(&self, error: &Self::Error) -> &'static str {
        match error {
            GraphTaskDaemonRunError::UnsupportedRoute(_) => "route",
            GraphTaskDaemonRunError::RoutePlanMismatch { .. } => "route",
            GraphTaskDaemonRunError::DisallowedComponent { .. } => "policy",
            GraphTaskDaemonRunError::UnsupportedProfileRoute { .. } => "policy",
            GraphTaskDaemonRunError::Delegate(source) => match self.route {
                GraphTaskDaemonRoute::None => "route",
                GraphTaskDaemonRoute::Algorithm => {
                    self.evaluators.algorithm.error_classification(source)
                }
                GraphTaskDaemonRoute::Pipeline => {
                    self.evaluators.pipeline.error_classification(source)
                }
                GraphTaskDaemonRoute::StoreApi => {
                    self.evaluators.store_api.error_classification(source)
                }
                GraphTaskDaemonRoute::Hybrid => self.evaluators.hybrid.error_classification(source),
            },
        }
    }

    fn evaluate(
        &self,
        program: &GraphExecutionIntent,
        context: &TaskExecutionContext<'_>,
    ) -> Result<Self::Output, Self::Error> {
        let stage = self.route_stage(context);
        let profile = self
            .profile
            .map(|profile| profile.as_str().to_string())
            .unwrap_or_else(|| "none".to_string());
        context.push_stage_trace_at(
            TaskMonitoringLevel::Basic,
            stage,
            format!(
                "graphframe.daemon.route_registry.begin route={} profile={}",
                self.route.as_str(),
                profile
            ),
        );

        if let Some(profile) = self.profile {
            if !profile.allows_route(self.route) {
                context.push_stage_trace_at(
                    TaskMonitoringLevel::Basic,
                    stage,
                    format!(
                        "graphframe.daemon.route_registry.policy_block profile={} route={}",
                        profile.as_str(),
                        self.route.as_str()
                    ),
                );
                return Err(GraphTaskDaemonRunError::UnsupportedProfileRoute {
                    profile,
                    route: self.route,
                });
            }

            if let Err(component) =
                validate_profile_components(profile, self.route, program.program())
            {
                context.push_stage_trace_at(
                    TaskMonitoringLevel::Basic,
                    stage,
                    format!(
                        "graphframe.daemon.route_registry.component_block profile={} route={} component={}",
                        profile.as_str(),
                        self.route.as_str(),
                        component
                    ),
                );
                return Err(GraphTaskDaemonRunError::DisallowedComponent {
                    profile,
                    route: self.route,
                    component,
                });
            }
        }

        if !route_matches_plan(self.route, program.program()) {
            let surfaces = route_plan_surfaces_label(program.program());
            context.push_stage_trace_at(
                TaskMonitoringLevel::Basic,
                stage,
                format!(
                    "graphframe.daemon.route_registry.surface_mismatch route={} surfaces={}",
                    self.route.as_str(),
                    surfaces
                ),
            );
            return Err(GraphTaskDaemonRunError::RoutePlanMismatch {
                route: self.route,
                surfaces,
            });
        }

        let result = match self.route {
            GraphTaskDaemonRoute::None => Err(GraphTaskDaemonRunError::UnsupportedRoute(
                GraphTaskDaemonRoute::None,
            )),
            GraphTaskDaemonRoute::Algorithm => self
                .evaluators
                .algorithm
                .evaluate(program, context)
                .map_err(GraphTaskDaemonRunError::Delegate),
            GraphTaskDaemonRoute::Pipeline => self
                .evaluators
                .pipeline
                .evaluate(program, context)
                .map_err(GraphTaskDaemonRunError::Delegate),
            GraphTaskDaemonRoute::StoreApi => self
                .evaluators
                .store_api
                .evaluate(program, context)
                .map_err(GraphTaskDaemonRunError::Delegate),
            GraphTaskDaemonRoute::Hybrid => self
                .evaluators
                .hybrid
                .evaluate(program, context)
                .map_err(GraphTaskDaemonRunError::Delegate),
        };

        context.push_stage_trace_at(
            TaskMonitoringLevel::Basic,
            stage,
            format!(
                "graphframe.daemon.route_registry.end route={} profile={}",
                self.route.as_str(),
                profile
            ),
        );

        result
    }
}

impl<'a, Evaluator> TaskEvaluator<GraphExecutionIntent> for GraphTaskRoutedEvaluator<'a, Evaluator>
where
    Evaluator: TaskEvaluator<GraphExecutionIntent>,
{
    type Output = Evaluator::Output;
    type Error = GraphTaskDaemonRunError<Evaluator::Error>;

    fn error_classification(&self, error: &Self::Error) -> &'static str {
        match error {
            GraphTaskDaemonRunError::UnsupportedRoute(_) => "route",
            GraphTaskDaemonRunError::RoutePlanMismatch { .. } => "route",
            GraphTaskDaemonRunError::DisallowedComponent { .. } => "policy",
            GraphTaskDaemonRunError::UnsupportedProfileRoute { .. } => "policy",
            GraphTaskDaemonRunError::Delegate(source) => {
                self.evaluator.error_classification(source)
            }
        }
    }

    fn evaluate(
        &self,
        program: &GraphExecutionIntent,
        context: &TaskExecutionContext<'_>,
    ) -> Result<Self::Output, Self::Error> {
        let stage = self.route_stage(context);
        context.push_stage_trace_at(
            TaskMonitoringLevel::Basic,
            stage,
            format!(
                "graphframe.daemon.route.begin route={}",
                self.route.as_str()
            ),
        );

        let result = match self.route {
            GraphTaskDaemonRoute::None => Err(GraphTaskDaemonRunError::UnsupportedRoute(
                GraphTaskDaemonRoute::None,
            )),
            GraphTaskDaemonRoute::Algorithm
            | GraphTaskDaemonRoute::Pipeline
            | GraphTaskDaemonRoute::StoreApi
            | GraphTaskDaemonRoute::Hybrid => {
                if !route_matches_plan(self.route, program.program()) {
                    let surfaces = route_plan_surfaces_label(program.program());
                    Err(GraphTaskDaemonRunError::RoutePlanMismatch {
                        route: self.route,
                        surfaces,
                    })
                } else {
                    self.evaluator
                        .evaluate(program, context)
                        .map_err(GraphTaskDaemonRunError::Delegate)
                }
            }
        };

        context.push_stage_trace_at(
            TaskMonitoringLevel::Basic,
            stage,
            format!("graphframe.daemon.route.end route={}", self.route.as_str()),
        );

        result
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::Mutex;

    use crate::collections::graphframe::feature_grammar::validate_graph_feature_grammar;
    use crate::collections::graphframe::feature_grammar::GraphFeatureCardinality;
    use crate::collections::graphframe::feature_grammar::GraphFeatureGrammarForm;
    use crate::collections::graphframe::feature_grammar::GraphFeatureRule;
    use crate::collections::graphframe::feature_grammar::GraphFeatureStratum;
    use crate::collections::graphframe::feature_grammar::GraphFeatureValueType;
    use crate::collections::graphframe::frame::GraphFrame;
    use crate::collections::graphframe::GraphAutomationProfile;
    use crate::collections::graphframe::GraphFrameModelExt;
    use crate::collections::graphframe::GraphFramePlanExt;
    use crate::procedures::pipelines::LocalPipelinesProcedureFacade;
    use crate::procedures::GraphFacade;
    use crate::shell::ShellComponentMode;
    use crate::shell::ShellProcedureEvaluator;
    use crate::task::concurrency::TerminationFlag;
    use crate::task::evaluator::TaskEvaluator;
    use crate::task::evaluator::TaskExecutionContext;
    use crate::task::job::TaskJobState;
    use crate::task::spec::TaskMonitoringLevel;
    use crate::types::graph_store::DefaultGraphStore;
    use crate::types::random::RandomGraphConfig;

    use super::route_matches_plan;
    use super::route_plan_surfaces_label;
    use super::validate_profile_components;
    use super::GraphTaskDaemon;
    use super::GraphTaskDaemonRoute;
    use super::GraphTaskDaemonRuntimeBundle;
    use super::GraphTaskGraphFacadeProvider;
    use super::GraphTaskLocalPipelinesFacadeProvider;
    use super::GraphTaskPipelinesFacadeProvider;
    use super::GraphTaskRouteEvaluators;
    use super::GraphTaskRuntimeDependencyAdapter;
    use super::GraphTaskRuntimeInjector;
    use super::GraphTaskRuntimeProfile;
    use super::GraphTaskStoreGraphFacadeProvider;

    struct TestEvaluator;

    struct LabeledEvaluator {
        label: &'static str,
    }

    struct RecordingInjector {
        store: Arc<DefaultGraphStore>,
        seen: Arc<Mutex<Vec<(GraphTaskDaemonRoute, GraphTaskRuntimeProfile)>>>,
    }

    struct RecordingGraphProvider {
        store: Arc<DefaultGraphStore>,
        seen: Arc<Mutex<Vec<(GraphTaskDaemonRoute, GraphTaskRuntimeProfile)>>>,
    }

    struct RecordingPipelinesProvider {
        seen: Arc<Mutex<Vec<(GraphTaskDaemonRoute, GraphTaskRuntimeProfile)>>>,
    }

    impl TaskEvaluator<crate::collections::graphframe::GraphExecutionIntent> for TestEvaluator {
        type Output = &'static str;
        type Error = &'static str;

        fn evaluate(
            &self,
            _program: &crate::collections::graphframe::GraphExecutionIntent,
            _context: &TaskExecutionContext<'_>,
        ) -> Result<Self::Output, Self::Error> {
            Ok("ok")
        }
    }

    impl TaskEvaluator<crate::collections::graphframe::GraphExecutionIntent> for LabeledEvaluator {
        type Output = &'static str;
        type Error = &'static str;

        fn evaluate(
            &self,
            _program: &crate::collections::graphframe::GraphExecutionIntent,
            _context: &TaskExecutionContext<'_>,
        ) -> Result<Self::Output, Self::Error> {
            Ok(self.label)
        }
    }

    impl GraphTaskRuntimeInjector for RecordingInjector {
        fn inject(
            &self,
            route: GraphTaskDaemonRoute,
            profile: GraphTaskRuntimeProfile,
        ) -> ShellProcedureEvaluator {
            self.seen
                .lock()
                .expect("recording injector lock should succeed")
                .push((route, profile));
            GraphTaskDaemonRuntimeBundle::runtime_for_store(Arc::clone(&self.store))
        }
    }

    impl GraphTaskGraphFacadeProvider for RecordingGraphProvider {
        fn provide_graph_facade(
            &self,
            route: GraphTaskDaemonRoute,
            profile: GraphTaskRuntimeProfile,
        ) -> GraphFacade {
            self.seen
                .lock()
                .expect("recording graph provider lock should succeed")
                .push((route, profile));
            GraphFacade::new(Arc::clone(&self.store))
        }
    }

    impl GraphTaskPipelinesFacadeProvider for RecordingPipelinesProvider {
        fn provide_pipelines_facade(
            &self,
            route: GraphTaskDaemonRoute,
            profile: GraphTaskRuntimeProfile,
        ) -> LocalPipelinesProcedureFacade {
            self.seen
                .lock()
                .expect("recording pipelines provider lock should succeed")
                .push((route, profile));
            LocalPipelinesProcedureFacade::default()
        }
    }

    fn deterministic_frame() -> GraphFrame {
        let (_, frame) = deterministic_store_and_frame();
        frame
    }

    fn deterministic_store_and_frame() -> (Arc<DefaultGraphStore>, GraphFrame) {
        let store = Arc::new(
            DefaultGraphStore::random(&RandomGraphConfig::seeded(21))
                .expect("seeded graph store should build"),
        );
        let frame = GraphFrame::from_store(Arc::clone(&store)).expect("graph frame should build");
        (store, frame)
    }

    fn density_grammar(
    ) -> crate::collections::graphframe::feature_grammar::GraphFeatureGrammarChecked {
        validate_graph_feature_grammar(
            GraphFeatureGrammarForm::new("graph_theory", "v1").with_feature_rule(
                GraphFeatureRule::new(
                    GraphFeatureStratum::Graph,
                    "density",
                    GraphFeatureValueType::Scalar,
                    true,
                    GraphFeatureCardinality::One,
                ),
            ),
        )
        .expect("density grammar should be valid")
    }

    #[test]
    fn graph_task_daemon_runs_algorithm_submission_with_route_trace() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                2,
            )
            .expect("agent processing contract should compile");

        assert_eq!(contract.daemon_route(), GraphTaskDaemonRoute::Algorithm);

        let submission = contract.into_task_daemon_submission("organon");
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_evaluator(submission, &TestEvaluator, TerminationFlag::running_true())
            .expect("submission should become a runnable job");

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert_eq!(receipt.output().copied(), Some("ok"));
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route.begin route=algorithm",
            )
        }));
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route.end route=algorithm",
            )
        }));
    }

    #[test]
    fn graph_task_daemon_runs_hybrid_submission_with_route_trace() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats)
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                2,
            )
            .expect("hybrid processing contract should compile");

        assert_eq!(contract.daemon_route(), GraphTaskDaemonRoute::Hybrid);

        let submission = contract.into_task_daemon_submission("organon");
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_evaluator(submission, &TestEvaluator, TerminationFlag::running_true())
            .expect("submission should become a runnable job");

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route.begin route=hybrid",
            )
        }));
    }

    #[test]
    fn graph_task_daemon_route_registry_dispatches_store_api_submission() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                1,
            )
            .expect("store api processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let algorithm = LabeledEvaluator { label: "algorithm" };
        let pipeline = LabeledEvaluator { label: "pipeline" };
        let store_api = LabeledEvaluator { label: "store_api" };
        let hybrid = LabeledEvaluator { label: "hybrid" };
        let evaluators = GraphTaskRouteEvaluators::new(&algorithm, &pipeline, &store_api, &hybrid);

        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_route_evaluators(submission, evaluators, TerminationFlag::running_true())
            .expect("submission should run through route registry");

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert_eq!(receipt.output().copied(), Some("store_api"));
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=store_api",
            )
        }));
    }

    #[test]
    fn graph_task_daemon_route_registry_dispatches_hybrid_submission() {
        let plan = deterministic_frame()
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats)
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                2,
            )
            .expect("hybrid processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let algorithm = LabeledEvaluator { label: "algorithm" };
        let pipeline = LabeledEvaluator { label: "pipeline" };
        let store_api = LabeledEvaluator { label: "store_api" };
        let hybrid = LabeledEvaluator { label: "hybrid" };
        let evaluators = GraphTaskRouteEvaluators::new(&algorithm, &pipeline, &store_api, &hybrid);

        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_route_evaluators(submission, evaluators, TerminationFlag::running_true())
            .expect("submission should run through route registry");

        assert_eq!(receipt.state(), TaskJobState::Succeeded);
        assert_eq!(receipt.output().copied(), Some("hybrid"));
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=hybrid",
            )
        }));
    }

    #[test]
    fn graph_task_daemon_runtime_bundle_routes_algorithm_submission() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let bundle = GraphTaskDaemonRuntimeBundle::new(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_bundle(submission, &bundle, TerminationFlag::running_true())
            .expect("submission should run with default runtime bundle");

        assert!(receipt.state().is_terminal());
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=algorithm profile=hybrid_balanced",
            )
        }));
    }

    #[test]
    fn runtime_profile_blocks_disallowed_route() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                1,
            )
            .expect("store api processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let bundle = GraphTaskDaemonRuntimeBundle::new(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                GraphTaskRuntimeProfile::Analytics,
                TerminationFlag::running_true(),
            )
            .expect("submission should become task receipt even when blocked");

        assert_eq!(receipt.state(), TaskJobState::Failed);
        assert!(receipt
            .error()
            .map(|error| error
                .contains("runtime profile `analytics` disallows graph route `store_api`"))
            .unwrap_or(false));
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.policy_block profile=analytics route=store_api",
            )
        }));
    }

    #[test]
    fn runtime_profile_overrides_monitoring_level() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let bundle = GraphTaskDaemonRuntimeBundle::new(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                GraphTaskRuntimeProfile::EngineeringStrict,
                TerminationFlag::running_true(),
            )
            .expect("submission should run under strict profile");

        assert_eq!(
            receipt.spec().monitoring_level(),
            TaskMonitoringLevel::Detailed
        );
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=algorithm profile=engineering_strict",
            )
        }));
    }

    #[test]
    fn profile_runtime_bundle_constructor_supports_store_management_policy() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("drop_graph", ShellComponentMode::Mutate);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentHybrid,
                1,
            )
            .expect("store api processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let profile = GraphTaskRuntimeProfile::StoreManagement;
        let bundle = profile.runtime_bundle(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                profile,
                TerminationFlag::running_true(),
            )
            .expect("submission should run with store management profile bundle");

        assert!(receipt.state().is_terminal());
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=store_api profile=store_management",
            )
        }));
        assert!(!receipt
            .error()
            .map(|error| error.contains("disallows graph route"))
            .unwrap_or(false));
    }

    #[test]
    fn explicit_route_runtime_injection_constructor_is_usable() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let algorithm = GraphTaskDaemonRuntimeBundle::new(Arc::clone(&store));
        let pipeline = GraphTaskDaemonRuntimeBundle::new(Arc::clone(&store));
        let store_api = GraphTaskDaemonRuntimeBundle::new(Arc::clone(&store));
        let hybrid = GraphTaskDaemonRuntimeBundle::new(store);
        let bundle = GraphTaskDaemonRuntimeBundle::from_route_runtimes(
            algorithm.algorithm,
            pipeline.pipeline,
            store_api.store_api,
            hybrid.hybrid,
        );
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_bundle(submission, &bundle, TerminationFlag::running_true())
            .expect("submission should run with injected route runtimes");

        assert!(receipt.state().is_terminal());
    }

    #[test]
    fn runtime_bundle_from_injector_records_route_specific_injection() {
        let (store, _) = deterministic_store_and_frame();
        let seen = Arc::new(Mutex::new(Vec::new()));
        let injector = RecordingInjector {
            store,
            seen: Arc::clone(&seen),
        };

        let _bundle = GraphTaskDaemonRuntimeBundle::from_injector(
            GraphTaskRuntimeProfile::EngineeringStrict,
            &injector,
        );

        let calls = seen
            .lock()
            .expect("recording injector lock should succeed")
            .clone();
        assert_eq!(calls.len(), 4);
        assert!(calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::Algorithm
                && *profile == GraphTaskRuntimeProfile::EngineeringStrict
        }));
        assert!(calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::Pipeline
                && *profile == GraphTaskRuntimeProfile::EngineeringStrict
        }));
        assert!(calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::StoreApi
                && *profile == GraphTaskRuntimeProfile::EngineeringStrict
        }));
        assert!(calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::Hybrid
                && *profile == GraphTaskRuntimeProfile::EngineeringStrict
        }));
    }

    #[test]
    fn profile_runtime_bundle_with_injector_routes_algorithm_submission() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let seen = Arc::new(Mutex::new(Vec::new()));
        let injector = RecordingInjector {
            store,
            seen: Arc::clone(&seen),
        };
        let profile = GraphTaskRuntimeProfile::EngineeringStrict;
        let bundle = profile.runtime_bundle_with_injector(&injector);

        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                profile,
                TerminationFlag::running_true(),
            )
            .expect("submission should run under injected runtime bundle");

        assert!(receipt.state().is_terminal());
        assert_eq!(
            receipt.spec().monitoring_level(),
            TaskMonitoringLevel::Detailed
        );
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.begin route=algorithm profile=engineering_strict",
            )
        }));

        let calls = seen
            .lock()
            .expect("recording injector lock should succeed")
            .clone();
        assert_eq!(calls.len(), 4);
    }

    #[test]
    fn runtime_bundle_from_providers_records_provider_calls() {
        let (store, _) = deterministic_store_and_frame();
        let graph_seen = Arc::new(Mutex::new(Vec::new()));
        let pipelines_seen = Arc::new(Mutex::new(Vec::new()));
        let graph_provider = RecordingGraphProvider {
            store,
            seen: Arc::clone(&graph_seen),
        };
        let pipelines_provider = RecordingPipelinesProvider {
            seen: Arc::clone(&pipelines_seen),
        };

        let _bundle = GraphTaskDaemonRuntimeBundle::from_providers(
            GraphTaskRuntimeProfile::StoreManagement,
            &graph_provider,
            &pipelines_provider,
        );

        let graph_calls = graph_seen
            .lock()
            .expect("recording graph provider lock should succeed")
            .clone();
        let pipeline_calls = pipelines_seen
            .lock()
            .expect("recording pipelines provider lock should succeed")
            .clone();
        assert_eq!(graph_calls.len(), 4);
        assert_eq!(pipeline_calls.len(), 4);
        assert!(graph_calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::StoreApi
                && *profile == GraphTaskRuntimeProfile::StoreManagement
        }));
        assert!(pipeline_calls.iter().any(|(route, profile)| {
            *route == GraphTaskDaemonRoute::Hybrid
                && *profile == GraphTaskRuntimeProfile::StoreManagement
        }));
    }

    #[test]
    fn profile_runtime_bundle_with_providers_routes_algorithm_submission() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let graph_seen = Arc::new(Mutex::new(Vec::new()));
        let pipelines_seen = Arc::new(Mutex::new(Vec::new()));
        let graph_provider = RecordingGraphProvider {
            store,
            seen: Arc::clone(&graph_seen),
        };
        let pipelines_provider = RecordingPipelinesProvider {
            seen: Arc::clone(&pipelines_seen),
        };

        let profile = GraphTaskRuntimeProfile::EngineeringStrict;
        let bundle = profile.runtime_bundle_with_providers(&graph_provider, &pipelines_provider);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                profile,
                TerminationFlag::running_true(),
            )
            .expect("submission should run under provider-built runtime bundle");

        assert!(receipt.state().is_terminal());
        assert_eq!(
            receipt.spec().monitoring_level(),
            TaskMonitoringLevel::Detailed
        );

        let graph_calls = graph_seen
            .lock()
            .expect("recording graph provider lock should succeed")
            .clone();
        let pipeline_calls = pipelines_seen
            .lock()
            .expect("recording pipelines provider lock should succeed")
            .clone();
        assert_eq!(graph_calls.len(), 4);
        assert_eq!(pipeline_calls.len(), 4);
    }

    #[test]
    fn runtime_dependency_adapter_builds_profile_bundle() {
        let (store, _) = deterministic_store_and_frame();
        let graph_seen = Arc::new(Mutex::new(Vec::new()));
        let pipelines_seen = Arc::new(Mutex::new(Vec::new()));
        let graph_provider = RecordingGraphProvider {
            store,
            seen: Arc::clone(&graph_seen),
        };
        let pipelines_provider = RecordingPipelinesProvider {
            seen: Arc::clone(&pipelines_seen),
        };

        let adapter = GraphTaskRuntimeDependencyAdapter::new(&graph_provider, &pipelines_provider);
        let _bundle = adapter.runtime_bundle(GraphTaskRuntimeProfile::StoreManagement);

        let graph_calls = graph_seen
            .lock()
            .expect("recording graph provider lock should succeed")
            .clone();
        let pipeline_calls = pipelines_seen
            .lock()
            .expect("recording pipelines provider lock should succeed")
            .clone();
        assert_eq!(graph_calls.len(), 4);
        assert_eq!(pipeline_calls.len(), 4);
    }

    #[test]
    fn runtime_dependency_adapter_bundle_runs_algorithm_submission() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let graph_provider = GraphTaskStoreGraphFacadeProvider::new(store);
        let pipelines_provider = GraphTaskLocalPipelinesFacadeProvider;
        let adapter = GraphTaskRuntimeDependencyAdapter::new(&graph_provider, &pipelines_provider);
        let profile = GraphTaskRuntimeProfile::EngineeringStrict;
        let bundle = adapter.runtime_bundle(profile);

        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                profile,
                TerminationFlag::running_true(),
            )
            .expect("submission should run under adapter-built bundle");

        assert!(receipt.state().is_terminal());
        assert_eq!(
            receipt.spec().monitoring_level(),
            TaskMonitoringLevel::Detailed
        );
    }

    #[test]
    fn route_surface_alignment_follows_execution_kind_contract() {
        use crate::shell::GdsShell;

        let algorithm = GdsShell::new()
            .component_plan()
            .component("pagerank", ShellComponentMode::Stats)
            .expect("pagerank component should build")
            .finish();
        assert!(route_matches_plan(
            GraphTaskDaemonRoute::Algorithm,
            &algorithm
        ));
        assert_eq!(route_plan_surfaces_label(&algorithm), "algorithm");

        let store_api = GdsShell::new()
            .component_plan()
            .component("drop_graph", ShellComponentMode::Mutate)
            .expect("drop_graph component should build")
            .finish();
        assert!(route_matches_plan(
            GraphTaskDaemonRoute::StoreApi,
            &store_api
        ));
        assert!(!route_matches_plan(
            GraphTaskDaemonRoute::Algorithm,
            &store_api
        ));
        assert_eq!(route_plan_surfaces_label(&store_api), "store_api");

        let hybrid = GdsShell::new()
            .component_plan()
            .component("pagerank", ShellComponentMode::Stats)
            .expect("pagerank component should build")
            .finish()
            .component("drop_graph", ShellComponentMode::Mutate)
            .expect("drop_graph component should build")
            .finish();
        assert!(route_matches_plan(GraphTaskDaemonRoute::Hybrid, &hybrid));
        assert_eq!(route_plan_surfaces_label(&hybrid), "algorithm+store_api");
    }

    #[test]
    fn route_registry_reports_surface_mismatch_as_failure() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("pagerank", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let bundle = GraphTaskDaemonRuntimeBundle::new(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                GraphTaskRuntimeProfile::StoreManagement,
                TerminationFlag::running_true(),
            )
            .expect("submission should return a task receipt");

        assert_eq!(receipt.state(), TaskJobState::Failed);
        assert!(receipt
            .error()
            .map(|message| message.contains("disallows graph route `algorithm`"))
            .unwrap_or(false));
    }

    #[test]
    fn engineering_strict_allows_curated_component_for_algorithm_route() {
        use crate::shell::GdsShell;

        let allowed = GdsShell::new()
            .component_plan()
            .component("pagerank", ShellComponentMode::Stats)
            .expect("pagerank component should build")
            .finish();

        let result = validate_profile_components(
            GraphTaskRuntimeProfile::EngineeringStrict,
            GraphTaskDaemonRoute::Algorithm,
            &allowed,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn engineering_strict_blocks_non_allowlisted_algorithm_component() {
        let (store, frame) = deterministic_store_and_frame();
        let plan = frame
            .gm()
            .model("graph-theory.density-model.v1")
            .grammar_with_version("graph_theory", "v1")
            .into_plan()
            .gp()
            .id("graph-theory.observe-density.v1")
            .into_plan()
            .call("leiden", ShellComponentMode::Stats);

        let contract = plan
            .compile_agent_processing_contract(
                &density_grammar(),
                GraphAutomationProfile::AgentAnalytics,
                1,
            )
            .expect("algorithm processing contract should compile");
        let submission = contract.into_task_daemon_submission("organon");

        let bundle = GraphTaskDaemonRuntimeBundle::new(store);
        let daemon = GraphTaskDaemon::new();
        let receipt = daemon
            .run_with_runtime_profile(
                submission,
                &bundle,
                GraphTaskRuntimeProfile::EngineeringStrict,
                TerminationFlag::running_true(),
            )
            .expect("submission should return a receipt under strict policy");

        assert_eq!(receipt.state(), TaskJobState::Failed);
        assert!(
            receipt
                .error()
                .map(|message| message
                    .contains("disallows component `gds.algorithms.community.leiden`"))
                .unwrap_or(false)
        );
        assert!(receipt.trace().iter().any(|entry| {
            entry.contains(
                "stage=pipeline::GraphFrameCompute event=graphframe.daemon.route_registry.component_block profile=engineering_strict route=algorithm component=gds.algorithms.community.leiden",
            )
        }));
    }
}
