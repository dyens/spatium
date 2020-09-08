use std::error;
use std::fmt;

#[derive(Debug)]
/// # SpatiumError
pub enum SpatiumError {
    /// # Value Error variant.
    ValueError(String),

    /// # Error of normalization.
    NormalizationError,
}

impl fmt::Display for SpatiumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            SpatiumError::ValueError(reason) => write!(f, "ValueError: {}", reason),
            SpatiumError::NormalizationError => write!(f, "NormalizationError"),
        }
    }
}

impl error::Error for SpatiumError {
    #[allow(clippy::match_single_binding)]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}
