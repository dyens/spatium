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

/// Distance trait
pub trait Distance {
    /// Distance between sequeces
    fn distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq;

    /// Normalized distance between sequeces
    /// The distance is normalized by dividing it
    /// by the greater of x_len or y_len (lenghths of sequeces).
    fn normalized_distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq,
    {
        self.distance(&x, &y)
            .and_then(|dis| normalize(dis, x.len(), y.len()))
    }
}
