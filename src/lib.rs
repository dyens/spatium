#![deny(missing_docs)]
//! # SPATIUM
//!
//! SPATIUM is a library for calcuting distances beetween sequences.
//! ## Algorithms:
//!
//! ### Edit based:
//! - [Hamming](edit_based::hamming::Hamming)
//! - [Levenshtein](edit_based::levenshtein::Levenshtein)
//! - [Damerau-Levenshtein](edit_based::damerau_levenshtein::DamerauLevenshtein)

/// # Edit Based algorithms
pub mod edit_based;

/// # Library error class
pub mod error;
