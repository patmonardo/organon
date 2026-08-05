//! GraphFrame: Polars-backed graph table DSL.
//!
//! This module follows the RustScript 2×2 entrypoint matrix:
//! - `expr.rs` / `lazy.rs`
//! - `series.rs` / `frame.rs`
//!
//! Additional surfaces like `pgql/` live alongside the entrypoints.

pub mod agent_plan;
pub mod expr;
pub mod feature_grammar;
pub mod frame;
pub mod functions;
pub mod graph_form;
pub mod lazy;
pub mod r#macro;
pub mod model;
// PGQL is work-in-progress and currently depends on crates not in the default build.
// Keep it behind an explicit feature gate so GraphFrame can compile cleanly.
pub mod daemon;
#[cfg(feature = "graphframe_pgql")]
pub mod pgql;
pub mod plan;
pub mod plugins;
pub mod prelude;
pub mod rational_language;
pub mod series;

pub use agent_plan::{GraphAgentPlanAction, GraphAgentPlanInterface, GraphProjectionMoment};
pub use daemon::{
    GraphTaskDaemon, GraphTaskDaemonRunError, GraphTaskDaemonRuntimeBundle,
    GraphTaskDependencyInjector, GraphTaskFacadeRuntimeInjector, GraphTaskGraphFacadeProvider,
    GraphTaskLocalPipelinesFacadeProvider, GraphTaskPipelinesFacadeProvider,
    GraphTaskRouteEvaluators, GraphTaskRuntimeDependencyAdapter, GraphTaskRuntimeInjector,
    GraphTaskRuntimeProfile, GraphTaskStoreGraphFacadeProvider,
};
pub use expr::{
    GraphFeatureGrammarExpr, GraphFrameExpr, GraphModelExpr, GraphPlanExpr, GraphProcedureExpr,
    GraphViewExpr,
};
pub use feature_grammar::*;
pub use frame::{GraphFrame, GraphFrameError, SharedGraphStore};
pub use graph_form::{
    build_gdsl_transmission_spec, build_initial_gml_dataset_pipeline, GraphGdslTransmissionSpec,
    GraphTransmissionTarget,
};
pub use graph_form::{compile_graph_form, GraphFormCompilation, GraphFormError, GraphFormKind};
pub use graph_form::{
    derive_dataset_framings, derive_empirical_outflows, derive_execution_surface_mode,
    derive_execution_surfaces, derive_graph_store_components, derive_plan_logics,
    derive_primary_dataset_framing, derive_primary_execution_surface, graph_forms,
    synthetic_moments, GraphDatasetFraming, GraphEmpiricalOutflowAspect, GraphExecutionSurface,
    GraphExecutionSurfaceMode, GraphOutflowChannel, GraphPlanLogic, GraphSyntheticMoment,
};
pub use graph_form::{validate_automation_shell_plan, GraphAutomationProfile};
pub use lazy::{
    GraphAgentProcessingContract, GraphExecutionIntent, GraphFramePlan,
    GraphFramePureFormReciprocity, GraphTaskDaemonRoute, GraphTaskDaemonSubmission,
};
pub use model::{GraphFrameModelExt, GraphModelNameSpace};
pub use plan::{GraphFramePlanExt, GraphPlanNameSpace};
pub use plugins::{
    GraphFeatureGrammarPlugin, GRAPH_FEATURE_GRAMMAR_LANGUAGE_ID, GRAPH_FEATURE_GRAMMAR_PLUGIN_ID,
};
pub use rational_language::{
    lower_graph_semantic_program, lower_graph_semantics, observe_graph_density,
    validate_graph_density_plan, GraphRationalLanguageError, GraphSemanticLowering,
    GraphSemanticProgram, GraphSharedPlanTemplate,
};
pub use series::{GraphFrameSeriesNameSpace, SeriesGraphFrameExt};
