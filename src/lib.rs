#![deny(missing_docs)]
//! # Spatium
//!
//! Spatium is a library for calcuting distances beetween sequences.
//! The library allows you to run different algorithm implementations with various parameters

//! ## Examples
//! ```
//! use spatium::edit_based::levenshtein;
//! use spatium::prelude::*;
//!
//! // Get default algorithm for calc levenshtein distance.
//! let alg = levenshtein::Default::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0);
//!
//! // With normalization (normalized distance = distance / x.len())
//! let alg = levenshtein::Default::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.normalized_distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0 / 3.0);
//!
//! // Use obviously algorithm (for example recursive version)
//!
//! let alg = levenshtein::Recursive::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0);
//! ```
//!
//! ## Algorithms:
//!
//! ### Edit based:
//! - [Hamming](edit_based::hamming)
//! - [Levenshtein](edit_based::levenshtein)
//! - [Damerau-Levenshtein](edit_based::damerau_levenshtein)

/// # Distance trait
pub mod distance;

/// # Edit Based algorithms
pub mod edit_based;

/// # Library error class
pub mod error;

/// # Prelude
pub mod prelude {
    pub use crate::distance::Distance;
}
