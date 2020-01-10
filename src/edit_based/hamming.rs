use crate::error::SpatiumError;
use std::cmp::Eq;

const DIFFERENT_LENGTHS: &'static str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// Base Hamming algorithm implementation
fn _hamming<T>(x: T, y: T) -> u64
where
    T: IntoIterator,
    T::Item: Eq,
{
    let mut distance = 0;
    for (x_el, y_el) in x.into_iter().zip(y) {
        if x_el != y_el {
            distance += 1;
        }
    }
    distance
}

/// Hamming on slices
fn hamming_on_slices<T>(x: &[T], y: &[T]) -> Result<u64>
where
    T: Eq,
{
    if x.len() != y.len() {
        return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
    }
    Ok(_hamming(x, y))
}

/// Hamming on string slices
fn hamming_on_str(x: &str, y: &str) -> Result<u64> {
    if x.chars().count() != y.chars().count() {
        return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
    }
    Ok(_hamming(x.chars(), y.chars()))
}

/// # Hamming distance
///
/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between two strings of equal length is the number of positions at
/// which two strings are different.
/// This returns an error of type error::SpatiumError::ValueError if the string arguments do not have equal length.
///
/// ## Examples
/// ```
/// use spatium::edit_based::hamming::Hamming;
///
/// // On slices [T: Eq].
/// let x = [1, 2, 3];
/// let y = [1, 2, 4];
/// assert_eq!(x.as_ref().hamming(&y.as_ref()).unwrap(), 1);
///
///
/// // On Vec<T: Eq>.
/// let x = vec![1, 2, 3];
/// let y = vec![1, 2, 4];
/// assert_eq!(x.hamming(&y).unwrap(), 1);
///
///
/// // On &str.
/// let x = "Some string";
/// let y = "Some strink";
/// assert_eq!(x.hamming(&y).unwrap(), 1);
///
/// // On String.
/// let x = "Some string".to_string();
/// let y = "Some strink".to_string();
/// assert_eq!(x.hamming(&y).unwrap(), 1);
/// ```
pub trait Hamming<Rhs = Self> {
    /// Calc Hamming distance
    fn hamming(&self, other: &Rhs) -> Result<u64>;
}

impl<T> Hamming for &[T]
where
    T: Eq,
{
    fn hamming(&self, other: &&[T]) -> Result<u64> {
        hamming_on_slices(&self, &other)
    }
}

impl Hamming for &str {
    fn hamming(&self, other: &&str) -> Result<u64> {
        hamming_on_str(self, other)
    }
}

impl<T> Hamming for Vec<T>
where
    T: Eq,
{
    fn hamming(&self, other: &Vec<T>) -> Result<u64> {
        let vec_slice: &[T] = self.as_ref();
        vec_slice.hamming(&other.as_ref())
    }
}

impl Hamming for String {
    fn hamming(&self, other: &String) -> Result<u64> {
        let string_slice: &str = self.as_ref();
        string_slice.hamming(&other.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::Hamming;

    #[test]
    fn test_error_text() {
        let x = [1, 2, 3];
        let y = [1, 2, 4, 5];
        assert_eq!(
            x.as_ref().hamming(&y.as_ref()).unwrap_err().to_string(),
            "ValueError: Arguments have different lengths."
        );
    }

    #[test]
    fn test_on_slices() {
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        assert_eq!((&x[..]).hamming(&&y[..]).unwrap(), 1);
        // OR
        assert_eq!(x.as_ref().hamming(&y.as_ref()).unwrap(), 1);
    }

    #[test]
    fn test_on_slices_diff_lens() {
        let x = [1, 2, 3];
        let y = [1, 2, 4, 5];
        assert_eq!(x.as_ref().hamming(&y.as_ref()).is_err(), true);
    }

    #[test]
    fn test_on_vecs() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2, 4];
        assert_eq!(x.hamming(&y).unwrap(), 1);
    }

    #[test]
    fn test_on_vecs_diff_lens() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2, 4, 5];
        assert_eq!(x.hamming(&y).is_err(), true);
    }

    #[test]
    fn test_str_slice() {
        let x = "abc";
        let y = "abd";
        assert_eq!(x.hamming(&y).unwrap(), 1);
    }

    #[test]
    fn test_str_slice_diff_lens() {
        let x = "abcc";
        let y = "abd";
        assert_eq!(x.hamming(&y).is_err(), true);
    }

    #[test]
    fn test_unicode_str_slice() {
        let x = "Привет";
        let y = "Пвивет";
        assert_eq!(x.hamming(&y).unwrap(), 1);
    }

    #[test]
    fn test_unicode_str_slice_diff_lens() {
        let x = "Привет";
        let y = "Приветт";
        assert_eq!(x.hamming(&y).is_err(), true);
    }

    #[test]
    fn test_str() {
        let x = "abc".to_string();
        let y = "abd".to_string();
        assert_eq!(x.hamming(&y).unwrap(), 1);
    }

    #[test]
    fn test_str_diff_lens() {
        let x = "abcc".to_string();
        let y = "abd".to_string();
        assert_eq!(x.hamming(&y).is_err(), true);
    }

    #[test]
    fn test_unicode_str() {
        let x = "Привет".to_string();
        let y = "Пвивет".to_string();
        assert_eq!(x.hamming(&y).unwrap(), 1);
    }

    #[test]
    fn test_unicode_str_diff_lens() {
        let x = "Привет".to_string();
        let y = "Приветт".to_string();
        assert_eq!(x.hamming(&y).is_err(), true);
    }
}
