use crate::error::SpatiumError;
use std::cmp::Eq;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// # Hamming distance

/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at
/// which two strings are different.
/// This returns an error of type error::SpatiumError::ValueError if the string arguments do not have equal length.
///
#[derive(Default)]
pub struct Hamming {}

impl Hamming {
    /// #New object for calc distance
    pub fn new() -> Self {
        Self {}
    }
    /// #Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<u64>
    where
        T: Eq,
    {
        hamming(x, y)
    }
}

/// # Hamming distance
/// ## Examples
/// ```
/// use spatium::edit_based::hamming::hamming;
///
/// // On slices [T: Eq].
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// let distance = hamming(&x, &y).unwrap();
/// assert_eq!(distance, 1);
///
/// // On Vec<T: Eq>.
/// let x = vec![1, 2, 3];
/// let y = vec![1, 2, 4];
/// let distance = hamming(&x, &y).unwrap();
/// assert_eq!(distance, 1);
///
/// // On &str.
/// let x = "Привет";
/// let y = "Привёт";
/// let xc = x.chars().collect::<Vec<char>>();
/// let yc = y.chars().collect::<Vec<char>>();
/// let distance = hamming(&xc, &yc).unwrap();
/// assert_eq!(distance, 1);
/// ```
pub fn hamming<T>(x: &[T], y: &[T]) -> Result<u64>
where
    T: Eq,
{
    if x.len() != y.len() {
        return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
    }
    let mut distance = 0;
    for (x_el, y_el) in x.into_iter().zip(y.iter()) {
        if x_el != y_el {
            distance += 1;
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
        assert_eq!(distance, 1);
    }

    #[test]
    fn on_unicode_strings() {
        let x = "Привет";
        let y = "Привёт";
        let xc = x.chars().collect::<Vec<char>>();
        let yc = y.chars().collect::<Vec<char>>();
        let distance = hamming(&xc, &yc).unwrap();
        assert_eq!(distance, 1);
    }

    #[test]
    fn default() {
        let alg = Hamming::default();
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 1);
    }
}
