use crate::error::SpatiumError;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

// TODO: Adjacment distance???
/// #Damerau-Levenshtein algorithm
///
/// [Damerau-Levenshtein distance] (https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance)
///
/// The Damerau-Levenshtein distance is the minimum number of edit
/// operations necessary for transforming one sequence into the other.
/// The edit operations allowed are:
///     * deletion:      ABC -> BC, AC, AB
///     * insertion:     ABC -> ABCD, EABC, AEBC..
///     * substitution:  ABC -> ABE, ADC, FBC..
///     * transposition: ABC -> ACB, BAC
#[derive(Default)]
pub struct DamerauLevenshtein {}

impl DamerauLevenshtein {
    /// #New object for calc distance
    pub fn new() -> Self {
        Self {}
    }
    /// #Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<u64>
    where
        T: Eq,
    {
        damerau_levenshtein(x, y)
    }
}

/// #Damerau Levenshtein distance
pub fn damerau_levenshtein<T>(x: &[T], y: &[T]) -> Result<u64>
where
    T: Eq,
{
    let x_len = x.len();
    let y_len = y.len();

    // for all i and j, matrix[i,j] will hold the Damerau_Levenshtein distance between
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

            dbg!(i, j);
            if i > 1 && j > 1 && x[i - 1] == y[j - 2] && x[i - 2] == y[j - 1] {
                matrix[j][i] = min(matrix[j][i], matrix[j - 2][i - 2] + 1); // transposition
            }
        }
    }

    Ok(matrix[y_len][x_len] as u64)
}

#[cfg(test)]
mod tests {

    use super::damerau_levenshtein;
    use super::DamerauLevenshtein;

    macro_rules! damerau_levenshtein{
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

    damerau_levenshtein! {
    eq: (damerau_levenshtein, &[1], &[1], 0),
    deletion: (damerau_levenshtein, &[1], &[], 1),
    insertion: (damerau_levenshtein, &[0], &[1], 1),
    substitution: (damerau_levenshtein, &[1], &[2], 1),
    transposition: (damerau_levenshtein, &[1, 2, 3], &[1, 3, 2], 1),
    case1: (damerau_levenshtein, &[1, 5, 3], &[4, 5, 6, 7], 3),

    }

    #[test]
    fn default() {
        let alg = DamerauLevenshtein::default();
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 3);
    }
}
