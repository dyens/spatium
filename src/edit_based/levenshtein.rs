use crate::error::SpatiumError;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

/// #Algorithmtype
pub enum AlgorithmType {
    /// #Recursive algorith
    RECURSIVE,
    /// #Matrix algorith
    MATRIX,
}

impl Default for AlgorithmType {
    fn default() -> Self {
        AlgorithmType::MATRIX
    }
}

/// #Levenshtein algorithm
#[derive(Default)]
pub struct Levenshtein {
    /// Type of algorithm
    pub algorithm_type: AlgorithmType,
}

impl Levenshtein {
    /// #New object for calc distance
    pub fn new(alg_type: AlgorithmType) -> Self {
        Self {
            algorithm_type: alg_type,
        }
    }
    /// #Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<u64>
    where
        T: Eq,
    {
        let alg_function = match self.algorithm_type {
            AlgorithmType::RECURSIVE => levenshtein_recursive,
            AlgorithmType::MATRIX => levenshtein_matrix,
        };
        alg_function(x, y)
    }
}

/// #Levenshtein recursive algorithm
///
/// Dont use it in real live!
/// This is a straightforward, but inefficient, recursive
/// implementation of a Levenshtein distance.
pub fn levenshtein_recursive<T>(x: &[T], y: &[T]) -> Result<u64>
where
    T: Eq,
{
    let x_len = x.len();
    let y_len = y.len();
    if x_len == 0 {
        return Ok(y_len as u64);
    }
    if y_len == 0 {
        return Ok(x_len as u64);
    }

    let mut cost = 1;
    if x[x_len - 1] == y[y_len - 1] {
        cost = 0;
    }

    Ok(min(
        min(
            levenshtein_recursive(&x[..x_len - 1], &y).unwrap() + 1,
            levenshtein_recursive(&x, &y[..y_len - 1]).unwrap() + 1,
        ),
        levenshtein_recursive(&x[..x_len - 1], &y[..y_len - 1]).unwrap() + cost,
    ))
}

/// #Levenshtein matrix algorithm
///
pub fn levenshtein_matrix<T>(x: &[T], y: &[T]) -> Result<u64>
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
    Ok(matrix[y_len][x_len] as u64)
}

#[cfg(test)]
mod tests {

    use super::levenshtein_matrix;
    use super::levenshtein_recursive;
    use super::AlgorithmType;
    use super::Levenshtein;

    macro_rules! levenshtein_recursive {
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

    levenshtein_recursive! {
    recursive_eq: (levenshtein_recursive, &[1], &[1], 0),
    recursive_deletion: (levenshtein_recursive, &[1], &[], 1),
    recursive_insertion: (levenshtein_recursive, &[0], &[1], 1),
    recursive_substitution: (levenshtein_recursive, &[1], &[2], 1),
    recursive_case1: (levenshtein_recursive, &[1, 5, 3], &[4, 5,6,7], 3),

    matrix_eq: (levenshtein_matrix, &[1], &[1], 0),
    matrix_deletion: (levenshtein_matrix, &[1], &[], 1),
    matrix_insertion: (levenshtein_matrix, &[0], &[1], 1),
    matrix_substitution: (levenshtein_matrix, &[1], &[2], 1),
    matrix_case1: (levenshtein_matrix, &[1, 5, 3], &[4, 5, 6, 7], 3),
    }

    #[test]
    fn default() {
        let alg = Levenshtein::default();
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3);
    }

    #[test]
    fn recusive() {
        let alg = Levenshtein::new(AlgorithmType::RECURSIVE);
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3);
    }

    #[test]
    fn matrix() {
        let alg = Levenshtein::new(AlgorithmType::MATRIX);
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3);
    }
}
