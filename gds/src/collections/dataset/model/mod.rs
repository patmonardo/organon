//! Model module wiring.
//!
//! Keep `mod.rs` structural; implementation lives in `core.rs`.

pub mod core;
pub mod exec;
mod frames;
pub mod image;
mod mediator;
pub mod prep;

pub use core::*;
pub use exec::{
    execute_essence, execute_feature, execute_marked, ExecutedFeature, Execution, ExecutionAction,
};
pub use frames::*;
pub use image::{realize_from_essence, realize_image, ImageOptions};
pub use mediator::*;
pub use prep::{
    prepare_model, prepare_model_with_provenance, FeatureMark, MarkRequirement, MarkedFeature,
    Modality, ModelEssence, ModelPrepExt, PreparationError, PreparationReport, PreparationStep,
};
