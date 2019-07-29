#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations
)]

//! A framework for Live2D's cubism sdk
pub use cubism_core as core;

pub mod error;
pub mod json;
