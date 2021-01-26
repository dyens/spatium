#![feature(min_const_generics)]
//! # Spatium
//!
//! Spatium is a library for calcuting distances beetween sequences.
//! The library allows you to run different algorithm implementations with various parameters

/// # Distance trait
pub mod distance;

/// # Library error class
pub mod error;

/// # Library types
pub mod types;

/// # Edit Based algorithms
pub mod edit_based;

/// # Prelude

pub mod prelude {
    pub use crate::distance::IteratorsDistance;
    pub use crate::distance::ExactSizeIteratorsDistance;
}
