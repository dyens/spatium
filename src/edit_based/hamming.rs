use crate::error::SpatiumError;
use crate::normalize::normalize;
use std::cmp::Eq;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// # Hamming distance
///
/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at
/// which two strings are different.
///
/// ## Examples
/// ```
/// use spatium::edit_based::hamming::Hamming;
///
/// let alg = Hamming::default();
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
/// let alg = Hamming::default().normalize_result(true);
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// let distance = alg.distance(&x, &y).unwrap();
/// assert_eq!(distance, 1.0 / 3.0);
/// ```
/// # References:
/// - [Wikipedia](https://en.wikipedia.org/wiki/Hamming_distance)
///
/// ## Some implementation
/// - [python](https://github.com/chrislit/abydos/blob/master/abydos/distance/_hamming.py)

#[derive(Default)]
pub struct Hamming {
    /// Is result should be normalized?
    normalized: bool,
}

impl Hamming {
    /// New object for calc distance
    pub fn new() -> Self {
        Self::default()
    }

    /// Calc normailzed distance.
    /// The distance is normalized by dividing it
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
        let distance = hamming(x, y);
        match self.normalized {
            false => distance,
            true => distance.and_then(|dis| normalize(dis, x.len(), y.len())),
        }
    }
}

fn hamming<T>(x: &[T], y: &[T]) -> Result<f64>
where
    T: Eq,
{
    if x.len() != y.len() {
        return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
    }
    let mut distance: f64 = 0.0;
    for (x_el, y_el) in x.into_iter().zip(y.iter()) {
        if x_el != y_el {
            distance += 1.0;
        }
    }
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::hamming;
    use super::Hamming;

    #[test]
    fn error_text() {
        let x = [1, 2, 3];
        let y = [1, 2, 4, 5];
        assert_eq!(
            hamming(&x, &y).unwrap_err().to_string(),
            "ValueError: Arguments have different lengths."
        );
    }

    #[test]
    fn on_slices() {
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = hamming(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_unicode_strings() {
        let x = "Привет";
        let y = "Привёт";
        let xc = x.chars().collect::<Vec<char>>();
        let yc = y.chars().collect::<Vec<char>>();
        let distance = hamming(&xc, &yc).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn default() {
        let alg = Hamming::default();
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn normalize_result() {
        let alg = Hamming::new().normalize_result(true);
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0 / 3.0);
    }
}
