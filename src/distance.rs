use crate::core::SpatiumIterator;
use crate::core::SpatiumIteratorWithLen;
use crate::error::SpatiumError;
use std::cmp::max;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Normalize distance by len of sequences.
///
/// The distance is normalized by dividing it
/// by the greater of x_len or y_len (lenghths of sequeces).
fn normalize(distance: f64, x_len: usize, y_len: usize) -> Result<f64> {
    let max_len = max(x_len, y_len) as f64;
    if distance != 0.0 && max_len == 0.0 {
        return Err(SpatiumError::NormalizationError);
    }
    if distance == 0.0 {
        return Ok(0.0);
    }
    Ok(distance / max_len)
}

/// Distance trait on iterators
pub trait IteratorsDistance {
    /// Distance between sequeces
    fn distance<T, U>(&self, x: T, y: T) -> Result<f64>
    where
        T: Into<SpatiumIterator<U>>,
        U: std::iter::Iterator,
        U::Item: std::cmp::Eq;
}

/// Distance trait on iterators with len
pub trait IteratorsWithLenDistance {
    /// Distance on spatium arguments
    fn spatium_distance<T>(
        &self,
        x: SpatiumIteratorWithLen<T>,
        y: SpatiumIteratorWithLen<T>,
    ) -> Result<f64>
    where
        T: std::iter::Iterator,
        T::Item: std::cmp::Eq;

    /// Distance between sequeces
    fn distance<T, U>(&self, x: T, y: T) -> Result<f64>
    where
        T: Into<SpatiumIteratorWithLen<U>>,
        U: std::iter::Iterator,
        U::Item: std::cmp::Eq,
    {
        let x = x.into();
        let y = y.into();
        self.spatium_distance(x, y)
    }

    /// Normalized distance between sequeces
    fn normalized_distance<T, U>(&self, x: T, y: T) -> Result<f64>
    where
        T: Into<SpatiumIteratorWithLen<U>>,
        U: std::iter::Iterator,
        U::Item: std::cmp::Eq,
    {
        let x = x.into();
        let y = y.into();
        let x_len = x.len();
        let y_len = y.len();
        let distance = self.spatium_distance(x, y)?;
        normalize(distance, x_len, y_len)
    }
}
