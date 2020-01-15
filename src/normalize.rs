use crate::error::SpatiumError;
use std::cmp::max;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Normalize distance by len of sequences.
///
/// The distance is normalized by dividing it
/// by the greater of x_len or y_len (lenghths of sequeces).
pub fn normalize(distance: f64, x_len: usize, y_len: usize) -> Result<f64> {
    let max_len = max(x_len, y_len) as f64;
    if distance != 0.0 && max_len == 0.0 {
        return Err(SpatiumError::NormalizationError);
    }
    if distance == 0.0 {
        return Ok(0.0);
    }
    Ok(distance / max_len)
}
