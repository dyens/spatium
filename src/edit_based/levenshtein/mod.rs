//! # Levenshtein distance
//!
//! The [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two strings
//! In simple case number of operations for transforming one sequence to another.
//! The edit operations allowed are:
//!
//! 1. deletion:      ABC -> BC, AC, AB
//! 2. insertion:     ABC -> ABCD, EABC, AEBC..
//! 3. substitution:  ABC -> ABE, ADC, FBC..
//!
//! ## Examples
//! ```
//! use spatium::edit_based::levenshtein;
//!
//! // Get default algorithm for calc levenshtein distance.
//! let alg = levenshtein::Default::default();
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0);
//!
//! // On &str.
//! let x = "Hello-МИР";
//! let y = "Hello-ПИР";
//! let xc = x.chars().collect::<Vec<char>>();
//! let yc = y.chars().collect::<Vec<char>>();
//! let distance = alg.distance(&xc, &yc).unwrap();
//! assert_eq!(distance, 1.0);
//!
//! // With normaliztion (normalized distance = distance / x.len())
//! let alg = levenshtein::Default::default().normalize_result(true);
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
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
//! # References:
//! - [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance)
//!
//! ## Some implementation
//! - [python](https://github.com/chrislit/abydos/blob/master/abydos/distance/_levenshtein.py)

/// Recursive algorithm
pub mod recursive;
pub use recursive::Recursive;

/// Wagner-Fisher algorithm
pub mod wagner_fisher;
pub use wagner_fisher::WagnerFisher;

/// Default algorithm
pub type Default = WagnerFisher;
