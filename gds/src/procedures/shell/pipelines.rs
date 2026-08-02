use crate::procedures::pipelines::LocalPipelinesProcedureFacade;
use crate::procedures::pipelines::PipelinesProcedureFacade;
use crate::shell::builtin_component;
use crate::shell::ShellComponentCall;
use std::sync::Arc;

use super::inputs::optional_bool;
use super::inputs::optional_str;
use super::inputs::required_string;
use super::ShellPipelineProcedure;
use super::ShellProcedureBinding;
use super::ShellProcedureError;
use super::ShellProcedureResult;

pub(super) fn bind_pipeline(
    facade: Arc<LocalPipelinesProcedureFacade>,
    call: &ShellComponentCall,
) -> Result<ShellProcedureBinding, ShellProcedureError> {
    let component = builtin_component(call.component.as_str())
        .ok_or(ShellProcedureError::UnknownComponent(call.component))?
        .descriptor();

    if !component.supports(call.mode) {
        return Err(ShellProcedureError::UnsupportedMode {
            component: component.id,
            mode: call.mode,
        });
    }

    let procedure = match component.alias {
        "create_link_prediction_pipeline" => ShellPipelineProcedure::CreateLinkPrediction {
            pipeline_name: required_string(call, "pipelineName", &["pipeline_name"])?,
        },
        "create_node_classification_pipeline" => ShellPipelineProcedure::CreateNodeClassification {
            pipeline_name: required_string(call, "pipelineName", &["pipeline_name"])?,
        },
        "create_node_regression_pipeline" => ShellPipelineProcedure::CreateNodeRegression {
            pipeline_name: required_string(call, "pipelineName", &["pipeline_name"])?,
        },
        "list_pipelines" => ShellPipelineProcedure::List {
            pipeline_name: optional_str(call, "pipelineName", &["pipeline_name"])?
                .map(str::to_string),
        },
        "pipeline_exists" => ShellPipelineProcedure::Exists {
            pipeline_name: required_string(call, "pipelineName", &["pipeline_name"])?,
        },
        "drop_pipeline" => ShellPipelineProcedure::Drop {
            pipeline_name: required_string(call, "pipelineName", &["pipeline_name"])?,
            fail_if_missing: optional_bool(call, "failIfMissing", &["fail_if_missing"])?
                .unwrap_or(true),
        },
        _ => return Err(ShellProcedureError::UnboundComponent(component.id)),
    };

    Ok(ShellProcedureBinding::Pipeline {
        component: component.id,
        mode: call.mode,
        facade,
        procedure,
    })
}

pub(super) fn invoke_pipeline(
    facade: &LocalPipelinesProcedureFacade,
    procedure: ShellPipelineProcedure,
) -> Result<ShellProcedureResult, ShellProcedureError> {
    Ok(match procedure {
        ShellPipelineProcedure::CreateLinkPrediction { pipeline_name } => {
            ShellProcedureResult::LinkPredictionPipeline(
                facade.link_prediction().create_pipeline(&pipeline_name),
            )
        }
        ShellPipelineProcedure::CreateNodeClassification { pipeline_name } => {
            ShellProcedureResult::NodePipeline(
                facade.node_classification().create_pipeline(&pipeline_name),
            )
        }
        ShellPipelineProcedure::CreateNodeRegression { pipeline_name } => {
            ShellProcedureResult::NodePipeline(
                facade.node_regression().create_pipeline(&pipeline_name),
            )
        }
        ShellPipelineProcedure::List { pipeline_name } => ShellProcedureResult::PipelineCatalog(
            facade.list(
                pipeline_name
                    .as_deref()
                    .unwrap_or(LocalPipelinesProcedureFacade::NO_VALUE),
            ),
        ),
        ShellPipelineProcedure::Exists { pipeline_name } => {
            ShellProcedureResult::PipelineExists(facade.exists(&pipeline_name))
        }
        ShellPipelineProcedure::Drop {
            pipeline_name,
            fail_if_missing,
        } => ShellProcedureResult::PipelineCatalog(facade.drop(&pipeline_name, fail_if_missing)),
    })
}
