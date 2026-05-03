//! Shared axes for the GDS Shell language.

/// Full shell address in the provisional 3^3 language cube.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellAddress {
    pub register: ShellRegister,
    pub pipeline: ShellPipeline,
    pub algebra: ShellAlgebra,
}

impl ShellAddress {
    pub fn new(register: ShellRegister, pipeline: ShellPipeline, algebra: ShellAlgebra) -> Self {
        Self {
            register,
            pipeline,
            algebra,
        }
    }
}

/// Seed moments shared by immediate DataFrame and mediated Dataset registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellMoment {
    /// Immediate frame/body: the data is present as a tabular object.
    Frame,
    /// Middle seed: schema, dtype, UDT seed, feature schematism, and plan.
    ModelFeaturePlan,
    /// Realized table/image: data or metadata made available for exchange.
    Table,
}

/// Pipeline moment governed by the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellPipeline {
    /// Immediate framed body.
    Frame,
    /// Middle where schema, typing, feature, model, and plan converge.
    ModelFeaturePlan,
    /// Realized table, image, or exchange form.
    TableImage,
}

impl From<ShellMoment> for ShellPipeline {
    fn from(moment: ShellMoment) -> Self {
        match moment {
            ShellMoment::Frame => ShellPipeline::Frame,
            ShellMoment::ModelFeaturePlan => ShellPipeline::ModelFeaturePlan,
            ShellMoment::Table => ShellPipeline::TableImage,
        }
    }
}

/// Algebraic keeper active at a shell address.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellAlgebra {
    /// Immediate schema/dtype surface.
    Schema,
    /// Feature-structure compatibility algebra.
    FeatStruct,
    /// Mediated declaration/program-feature surface.
    ProgramFeature,
}

/// Degree of development for a shell value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShellRegister {
    /// Immediate Polars/DataFrame definition.
    ImmediateDataFrame,
    /// Mediated Dataset/ProgramFeature expansion.
    MediatedDataset,
    /// Both registers are present and aligned.
    Unified,
}
