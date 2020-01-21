//! # Damerau-Levenshtein distance
//!
//! The [Damerau-Levenshtein distance](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance) distance is the minimum number of edit
//! operations necessary for transforming one sequence into the other.
//! The edit operations allowed are:
//!
//! 1. deletion:      ABC -> BC, AC, AB
//! 2. insertion:     ABC -> ABCD, EABC, AEBC..
//! 3. substitution:  ABC -> ABE, ADC, FBC..
//! 4. transposition: ABC -> ACB, BAC
//!
//! ## Examples
//! ```
//! use spatium::edit_based::damerau_levenshtein;
//!
//! let alg = damerau_levenshtein::Default::default();
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
//! let alg = damerau_levenshtein::Default::default().normalize_result(true);
//! let x = [1, 2, 3];
//! let y = [1, 2, 4];
//! let distance = alg.distance(&x, &y).unwrap();
//! assert_eq!(distance, 1.0 / 3.0);
//! ```
//!
//! # References:
//! - [Wikipedia](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
//!
//! ## Some implementation
//! - [python](https://github.com/chrislit/abydos/blob/master/abydos/distance/_damerau_levenshtein.py)
//! - [C#](http://blog.softwx.net/2015/01/optimizing-damerau-levenshtein_15.html)
//! - [js](https://github.com/Yomguithereal/talisman/blob/master/src/metrics/distance/damerau-levenshtein.js)
//! - [Java](https://github.com/KevinStern/software-and-algorithms/blob/master/src/main/java/blogspot/software_and_algorithms/stern_library/string/DamerauLevenshteinAlgorithm.java)

/// Dameraulevenshtein1 algorithm
pub mod damerau_levenshtein1;
pub use damerau_levenshtein1::DamerauLevenshtein1;

/// Default algorithm
pub type Default = DamerauLevenshtein1;
