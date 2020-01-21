use crate::error::SpatiumError;
use crate::normalize::normalize;
use std::cmp::Eq;

use std::cmp::min;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Recursive implementation Levenshtein distance.
///
/// Dont use it in real live!
/// This is a straightforward, but inefficient, recursive
/// implementation of a Levenshtein distance.
pub struct Recursive {
    /// Is result should be normalized?
    normalized: bool,
}

impl Default for Recursive {
    fn default() -> Self {
        Self { normalized: false }
    }
}

impl Recursive {
    fn calc_distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
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

        let cost = if x[x_len - 1] == y[y_len - 1] { 0 } else { 1 };

        Ok(min(
            min(
                self.calc_distance(&x[..x_len - 1], &y).unwrap() as u32 + 1,
                self.calc_distance(&x, &y[..y_len - 1]).unwrap() as u32 + 1,
            ),
            self.calc_distance(&x[..x_len - 1], &y[..y_len - 1])
                .unwrap() as u32
                + cost,
        ) as f64)
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

        if self.normalized {
            distance.and_then(|dis| normalize(dis, x.len(), y.len()))
        } else {
            distance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Recursive;

    macro_rules! check_params {
	($($name:ident: $value:expr,)*) => {
	$(
	    #[test]
	    fn $name() {
		let (x, y, distance) = $value;
		let dist = Recursive::default().distance(x, y).unwrap();
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

        let distance = Recursive::default()
            .normalize_result(true)
            .distance(&x, &y)
            .unwrap();
        assert_eq!(distance, 3.0 / 4.0);
    }
}
