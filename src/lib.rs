//! MCM algorithm implementation in Rust.
//!
//! This crate provides cloud masking capabilities for landsat 8-9 images.
//! There is also an experimental attempt at providing cloud masking for sentinel 2 images.

pub mod classifiers;
pub mod comparison;
pub mod persistence;
