use crate::error::SpatiumError;
use crate::normalize::normalize;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

/// [Wagner-Fisher](https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm)
/// implementation for Levenshtein distance.
pub struct WagnerFisher {
    /// Is result should be normalized?
    normalized: bool,
}

impl Default for WagnerFisher {
    fn default() -> Self {
        Self { normalized: false }
    }
}

impl WagnerFisher {
    fn calc_distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
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

    /// Set normalization parameter
    ///
    /// If normailzed is True,
    /// the distance is normalized by dividing it
    /// by the lenght of sequence.
    pub fn normalize_result(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
    }

    /// Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq,
    {
        let distance = self.calc_distance(x, y);
        match self.normalized {
            false => distance,
            true => distance.and_then(|dis| normalize(dis, x.len(), y.len())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WagnerFisher;

    macro_rules! check_params {
	($($name:ident: $value:expr,)*) => {
	$(
	    #[test]
	    fn $name() {
		let (x, y, distance) = $value;
		let dist = WagnerFisher::default().distance(x, y).unwrap();
		assert_eq!(distance, dist);
	    }
	)*
	}
    }

    check_params! {
    eq: (&[1], &[1], 0.0),
    deletion: (&[1], &[], 1.0),
    insertion: (&[0], &[1], 1.0),
    substitution: (&[1], &[2], 1.0),
    case1: (&[1, 5, 3], &[4, 5, 6, 7], 3.0),
    }

    #[test]
    fn normalize_result() {
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];

        let distance = WagnerFisher::default()
            .normalize_result(true)
            .distance(&x, &y)
            .unwrap();
        assert_eq!(distance, 3.0 / 4.0);
    }
}
