use crate::error::SpatiumError;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Algorithmtype
pub enum AlgorithmType {
    /// Recursive algorith
    Recursive,
    /// Wagner-Fisher algorithm
    WagnerFisher,
}

impl Default for AlgorithmType {
    fn default() -> Self {
        AlgorithmType::WagnerFisher
    }
}

/// # Levenshtein algorithm
#[derive(Default)]
pub struct Levenshtein {
    /// Type of algorithm
    algorithm_type: AlgorithmType,
}

impl Levenshtein {
    /// # New object for calc distance
    pub fn new(alg_type: AlgorithmType) -> Self {
        Self {
            algorithm_type: alg_type,
        }
    }
    /// # Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq,
    {
        let alg_function = match self.algorithm_type {
            AlgorithmType::Recursive => levenshtein_recursive,
            AlgorithmType::WagnerFisher => levenshtein_wagner_fisher,
        };
        alg_function(x, y)
    }
}

/// # Levenshtein recursive algorithm
///
/// Dont use it in real live!
/// This is a straightforward, but inefficient, recursive
/// implementation of a Levenshtein distance.
pub fn levenshtein_recursive<T>(x: &[T], y: &[T]) -> Result<f64>
where
    T: Eq,
{
    let x_len = x.len();
    let y_len = y.len();
    if x_len == 0 {
        return Ok(y_len as f64);
    }
    if y_len == 0 {
        return Ok(x_len as f64);
    }

    let mut cost = 1;
    if x[x_len - 1] == y[y_len - 1] {
        cost = 0;
    }

    Ok(min(
        min(
            levenshtein_recursive(&x[..x_len - 1], &y).unwrap() as u32 + 1,
            levenshtein_recursive(&x, &y[..y_len - 1]).unwrap() as u32 + 1,
        ),
        levenshtein_recursive(&x[..x_len - 1], &y[..y_len - 1]).unwrap() as u32 + cost,
    ) as f64)
}

/// # Levenshtein Wagner-Fisher algorithm
///
/// [Wagner-Fisher] (https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm)
pub fn levenshtein_wagner_fisher<T>(x: &[T], y: &[T]) -> Result<f64>
where
    T: Eq,
{
    let x_len = x.len();
    let y_len = y.len();

    // for all i and j, matrix[i,j] will hold the Levenshtein distance between
    // the first i characters of s and the first j characters of t
    // note that matrix has (x_len + 1) * (y_len + 1) values
    let mut matrix = vec![vec![0; x_len + 1]; y_len + 1];

    // source prefixes can be transformed into empty sequence by
    // dropping all elements
    for i in 1..(y_len + 1) {
        matrix[i][0] = i;
    }

    // target prefixes can be reached from empty source prefix
    // by inserting every element
    for j in 1..(x_len + 1) {
        matrix[0][j] = j;
    }

    for i in 1..(x_len + 1) {
        for j in 1..(y_len + 1) {
            let cost = if x[i - 1] == y[j - 1] { 0 } else { 1 };
            matrix[j][i] = min(
                min(
                    matrix[j - 1][i] + 1, // deletion
                    matrix[j][i - 1] + 1, // insertion
                ),
                matrix[j - 1][i - 1] + cost, //substitution
            );
        }
    }
    Ok(matrix[y_len][x_len] as f64)
}

#[cfg(test)]
mod tests {

    use super::levenshtein_recursive;
    use super::levenshtein_wagner_fisher;
    use super::AlgorithmType;
    use super::Levenshtein;

    macro_rules! levenshtein {
	($($name:ident: $value:expr,)*) => {
	$(
	    #[test]
	    fn $name() {
		let (function, x, y, distance) = $value;
		assert_eq!(distance, function(x, y).unwrap());
	    }
	)*
	}
    }

    levenshtein! {
    recursive_eq: (levenshtein_recursive, &[1], &[1], 0.0),
    recursive_deletion: (levenshtein_recursive, &[1], &[], 1.0),
    recursive_insertion: (levenshtein_recursive, &[0], &[1], 1.0),
    recursive_substitution: (levenshtein_recursive, &[1], &[2], 1.0),
    recursive_case1: (levenshtein_recursive, &[1, 5, 3], &[4, 5, 6, 7], 3.0),

    wagner_fisher_eq: (levenshtein_wagner_fisher, &[1], &[1], 0.0),
    wagner_fisher_deletion: (levenshtein_wagner_fisher, &[1], &[], 1.0),
    wagner_fisher_insertion: (levenshtein_wagner_fisher, &[0], &[1], 1.0),
    wagner_fisher_substitution: (levenshtein_wagner_fisher, &[1], &[2], 1.0),
    wagner_fisher_case1: (levenshtein_wagner_fisher, &[1, 5, 3], &[4, 5, 6, 7], 3.0),
    }

    #[test]
    fn default() {
        let alg = Levenshtein::default();
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3.0);
    }

    #[test]
    fn recusive() {
        let alg = Levenshtein::new(AlgorithmType::Recursive);
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3.0);
    }

    #[test]
    fn wagner_fisher() {
        let alg = Levenshtein::new(AlgorithmType::WagnerFisher);
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3.0);
    }
}
