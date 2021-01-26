//! # Hamming distance
//!
//! The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at
//! which two strings are different.
//!
//! ## Examples
//! ```
//! use spatium::prelude::*;
//! use spatium::edit_based::hamming;
//!
//! let alg = hamming::Default::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0);
//!
//! // On &str.
//! let x = "Hello";
//! let y = "Hella";
//! let distance = alg.distance(x, y).unwrap();
//! assert_eq!(distance, 1.0);
//!
//! // With normalization (normalized distance = distance / x.len())
//! let alg = hamming::Default::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.normalized_distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0 / 3.0);
//! ```
//! # References:
//! - [Wikipedia](https://en.wikipedia.org/wiki/Hamming_distance)
//!
//! ## Some implementation
//! - [python](https://github.com/chrislit/abydos/blob/master/abydos/distance/_hamming.py)

/// Hamming algorithm on iteartors
pub mod on_iterators;
/// Hamming algorithm on const size iteartors
pub mod on_exact_size_iterators;

pub use on_exact_size_iterators::Algorithm as DefaultHammingAlgorithm;
/// Default algorithm
pub type Default = DefaultHammingAlgorithm;
