//! Shell-defined pipeline language.

use std::collections::HashMap;

use crate::form::{Context, Morph, PureFormPrinciple, Shape};

use super::{ShellAddress, ShellProgram, ShellSchema};

/// Current Shell pipeline kind.
///
/// The Shell is intentionally modeled as one data pipeline here. Future
/// multipipeline work can add variants without changing the PureForm return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellPipelineKind {
    DataPipeline,
}

/// Named return from Shell pipeline knowledge into PureForm.
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
