//! Shell-defined pipeline language.

use std::collections::HashMap;

use crate::form::{Context, Morph, PureFormPrinciple, Shape};

use super::{GdsShell, ShellAddress, ShellProgram, ShellSchema};

/// Current Shell pipeline kind.
///
/// The Shell is intentionally modeled as one data pipeline here. Future
/// multipipeline work can add variants without changing the PureForm return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellPipelineKind {
    DataPipeline,
}

/// Named return from Shell pipeline knowledge into PureForm.
///
/// Speculative status: this is the Shell's Knowledge return. The Shell gathers
/// register, pipeline, schema, and program-feature determinations into a
/// PureForm principle; an Agent does not produce that return so much as trap it
/// as the knowledge available for its next act.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPureFormReturn {
    pipeline_kind: ShellPipelineKind,
    address: ShellAddress,
    principle: PureFormPrinciple,
}

impl ShellPureFormReturn {
    pub fn new(
        pipeline_kind: ShellPipelineKind,
        address: ShellAddress,
        principle: PureFormPrinciple,
    ) -> Self {
        Self {
            pipeline_kind,
            address,
            principle,
        }
    }

    pub fn pipeline_kind(&self) -> ShellPipelineKind {
        self.pipeline_kind
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn principle(&self) -> &PureFormPrinciple {
        &self.principle
    }

    pub fn into_principle(self) -> PureFormPrinciple {
        self.principle
    }
}

/// Shell-level facade for pipeline/procedure execution intent.
///
/// This is a thin snapshot over the shell descriptor and its PureForm return,
/// so callers can keep the orchestration boundary at Shell instead of reaching
/// directly into program or evaluator internals. In the current speculative
/// reading, this is also the place where an Agent can trap the Shell's
/// PureForm/Knowledge return without claiming to define PureForm itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPipelineFacade {
    descriptor: ShellPipelineDescriptor,
}

impl ShellPipelineFacade {
    pub fn new(descriptor: ShellPipelineDescriptor) -> Self {
        Self { descriptor }
    }

    pub fn from_shell(shell: &GdsShell) -> Self {
        Self::new(shell.descriptor())
    }

    pub fn descriptor(&self) -> &ShellPipelineDescriptor {
        &self.descriptor
    }

    pub fn pure_form_return(&self) -> ShellPureFormReturn {
        self.descriptor.to_pure_form_return()
    }

    pub fn into_descriptor(self) -> ShellPipelineDescriptor {
        self.descriptor
    }
}

/// Shell-owned view of a pipeline and its metapipeline account.
///
/// DataFrame and Dataset are the workhorse registers that implement this form.
/// The shell descriptor keeps their shared address, schema, and optional
/// mediated program declaration together without collapsing either register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPipelineDescriptor {
    address: ShellAddress,
    schema: Option<ShellSchema>,
    program: Option<ShellProgram>,
    has_immediate_body: bool,
    has_mediated_body: bool,
}

impl ShellPipelineDescriptor {
    pub fn new(address: ShellAddress) -> Self {
        Self {
            address,
            schema: None,
            program: None,
            has_immediate_body: false,
            has_mediated_body: false,
        }
    }

    pub fn with_schema(mut self, schema: ShellSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    pub fn with_program(mut self, program: ShellProgram) -> Self {
        self.program = Some(program);
        self
    }

    pub fn with_immediate_body(mut self) -> Self {
        self.has_immediate_body = true;
        self
    }

    pub fn with_mediated_body(mut self) -> Self {
        self.has_mediated_body = true;
        self
    }

    pub fn address(&self) -> ShellAddress {
        self.address
    }

    pub fn schema(&self) -> Option<&ShellSchema> {
        self.schema.as_ref()
    }

    pub fn program(&self) -> Option<&ShellProgram> {
        self.program.as_ref()
    }

    pub fn has_immediate_body(&self) -> bool {
        self.has_immediate_body
    }

    pub fn has_mediated_body(&self) -> bool {
        self.has_mediated_body
    }

    pub fn has_metapipeline(&self) -> bool {
        self.schema.is_some() || self.program.is_some() || self.has_mediated_body
    }

    pub fn to_pure_form_principle(&self) -> PureFormPrinciple {
        self.to_pure_form_return().into_principle()
    }

    pub fn to_pure_form_return(&self) -> ShellPureFormReturn {
        let shape = Shape::new(
            self.schema
                .as_ref()
                .map(ShellSchema::column_names)
                .unwrap_or_default(),
            Vec::new(),
            self.schema
                .as_ref()
                .map(ShellSchema::type_constraints)
                .unwrap_or_default(),
            self.validation_rules(),
        );

        let context = Context::new(
            self.dependencies(),
            self.execution_order(),
            "gds-shell.compute-graph".to_string(),
            self.conditions(),
        );

        let morph = Morph::new(self.morph_patterns());

        ShellPureFormReturn::new(
            ShellPipelineKind::DataPipeline,
            self.address,
            PureFormPrinciple::new(shape, context, morph),
        )
    }

    fn validation_rules(&self) -> HashMap<String, String> {
        let mut rules = HashMap::new();
        rules.insert(
            "shell.register".to_string(),
            format!("{:?}", self.address.register),
        );
        rules.insert(
            "shell.pipeline".to_string(),
            format!("{:?}", self.address.pipeline),
        );
        rules.insert(
            "shell.algebra".to_string(),
            format!("{:?}", self.address.algebra),
        );
        rules
    }

    fn dependencies(&self) -> Vec<String> {
        let mut dependencies = Vec::new();
        if self.has_immediate_body {
            dependencies.push("dataframe".to_string());
        }
        if self.has_mediated_body {
            dependencies.push("dataset".to_string());
        }
        if let Some(program) = &self.program {
            dependencies.push(format!("program::{}", program.program_name()));
        }
        dependencies
    }

    fn execution_order(&self) -> Vec<String> {
        let mut order = Vec::new();
        if self.has_immediate_body {
            order.push("dataframe.seed".to_string());
        }
        if self.has_mediated_body {
            order.push("dataset.mediate".to_string());
        }
        if self.schema.is_some() {
            order.push("shell.schema".to_string());
        }
        if self.program.is_some() {
            order.push("shell.program".to_string());
        }
        order.push("pureform.principle".to_string());
        order
    }

    fn conditions(&self) -> Vec<String> {
        vec![
            format!("has_immediate_body={}", self.has_immediate_body),
            format!("has_mediated_body={}", self.has_mediated_body),
            format!("has_metapipeline={}", self.has_metapipeline()),
        ]
    }

    fn morph_patterns(&self) -> Vec<String> {
        if let Some(program) = &self.program {
            let opcodes = program.features().opcodes();
            if !opcodes.is_empty() {
                return opcodes;
            }
        }

        vec!["shell.compute_graph".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::form::{ProgramFeature, ProgramFeatureKind, ProgramFeatures};

    fn sample_program_features() -> ProgramFeatures {
        ProgramFeatures::new(
            "gdsl.analytics".to_string(),
            vec!["centrality".to_string()],
            vec![ProgramFeature::new(
                ProgramFeatureKind::OperatorPattern,
                "algo.pagerank".to_string(),
                "operator_pattern::algo.pagerank".to_string(),
            )],
        )
    }

    #[test]
    fn shell_pipeline_facade_preserves_descriptor_and_return() {
        let shell = GdsShell::new().with_program_features(sample_program_features());
        let facade = shell.pipeline_facade();

        assert_eq!(
            facade.descriptor().program().unwrap().program_name(),
            "gdsl.analytics"
        );
        assert_eq!(
            facade.descriptor().address(),
            facade.pure_form_return().address()
        );
        assert_eq!(
            facade.pure_form_return().pipeline_kind(),
            ShellPipelineKind::DataPipeline
        );
        assert_eq!(
            facade.pure_form_return().principle().morph.patterns,
            vec!["algo.pagerank".to_string()]
        );
    }
}
