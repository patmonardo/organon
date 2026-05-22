//! GDS Shell protocol: the internal language of the GDS kernel.
//!
//! The shell is the protocol layer over the curated DSL vocabulary and the
//! language in which the kernel coordinates pipeline execution. It keeps the
//! immediate DataFrame register and mediated Dataset register visible at the
//! same time: DataFrame supplies the Polars body, while Dataset expands the same
//! moments through framework metadata and ProgramFeatures.

mod core;
mod help;
mod moments;
mod pipeline;
mod program;
mod schema;

pub use crate::dsl::*;

pub use core::{
    GdsShell, ShellCapabilityBand, ShellCapabilityMap, ShellCapabilityState, ShellCorpusError,
    ShellCorpusReport, ShellDataFrameKnowledge, ShellFeatureMomentKnowledge, ShellLearningReport,
    ShellMemoryEstimate, ShellModelFeaturePlanKnowledge, ShellModelGenesisKnowledge,
    ShellModelGenesisMoment, ShellModelMomentKnowledge, ShellPlanMomentKnowledge,
    ShellPlatformCapability, ShellProjectionTraceValidation, ShellSeed, ShellSemanticCapability,
    ShellSemanticPipelineKnowledge,
};
pub use help::{ShellFold, ShellHelp, ShellHelpEntry, ShellMomentKind, ShellVocabKind};
pub use moments::{ShellAddress, ShellAlgebra, ShellMoment, ShellPipeline, ShellRegister};
pub use pipeline::{ShellPipelineDescriptor, ShellPipelineFacade, ShellPipelineKind, ShellPureFormReturn};
pub use program::ShellProgram;
pub use schema::ShellSchema;
