use crate::error::SpatiumError;
use crate::normalize::normalize;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Algorithmtype
pub enum AlgorithmType {
    /// Recursive algorith.
    /// Don't use this in real applications.
    Recursive,
    /// Wagner-Fisher algorithm.
    /// This is simple matrix algorithm from.
    WagnerFisher,
}

impl Default for AlgorithmType {
    fn default() -> Self {
        AlgorithmType::WagnerFisher
    }
}

/// # Levenshtein distance
///
/// The [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance) between two strings
/// In simple case number of operations for transforming one sequence to another.
/// The edit operations allowed are:
///
/// 1. deletion:      ABC -> BC, AC, AB
/// 2. insertion:     ABC -> ABCD, EABC, AEBC..
/// 3. substitution:  ABC -> ABE, ADC, FBC..
///
/// ## Examples
/// ```
/// use spatium::edit_based::levenshtein::Levenshtein;
///
/// let alg = Levenshtein::default();
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// let distance = alg.distance(&x, &y).unwrap();
/// assert_eq!(distance, 1.0);
///
/// // On &str.
/// let x = "Hello-МИР";
/// let y = "Hello-ПИР";
/// let xc = x.chars().collect::<Vec<char>>();
/// let yc = y.chars().collect::<Vec<char>>();
/// let distance = alg.distance(&xc, &yc).unwrap();
/// assert_eq!(distance, 1.0);
///
/// // With normaliztion (normalized distance = distance / x.len())
/// let alg = Levenshtein::default().normalize_result(true);
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// let distance = alg.distance(&x, &y).unwrap();
/// assert_eq!(distance, 1.0 / 3.0);
///
/// // Use recursive algorithm
///
/// use spatium::edit_based::levenshtein::AlgorithmType;
///
/// let alg = Levenshtein::new(AlgorithmType::Recursive);
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// let distance = alg.distance(&x, &y).unwrap();
/// assert_eq!(distance, 1.0);
/// ```
/// # References:
/// - [Wikipedia](https://en.wikipedia.org/wiki/Levenshtein_distance)
///
/// ## Some implementation
/// - [python](https://github.com/chrislit/abydos/blob/master/abydos/distance/_levenshtein.py)

#[derive(Default)]
pub struct Levenshtein {
    /// Type of algorithm
    algorithm_type: AlgorithmType,
    /// Is result should be normalized?
    normalized: bool,
}

impl Levenshtein {
    /// # New object for calc distance
    pub fn new(alg_type: AlgorithmType) -> Self {
        let mut new_alg = Self::default();
        new_alg.algorithm_type = alg_type;
        new_alg
    }

    /// Calc normailzed distance.
    /// The distance is normalized by dividing it
    /// by the lenght of sequence.
    pub fn normalize_result(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
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
        let distance = alg_function(x, y);
        match self.normalized {
            false => distance,
            true => distance.and_then(|dis| normalize(dis, x.len(), y.len())),
        }
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
    fn normalize_result() {
        let alg = Levenshtein::default().normalize_result(true);
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3.0 / 4.0);
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
