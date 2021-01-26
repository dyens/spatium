use crate::types::iterator::SpatiumIterator;
use crate::types::exact_size_iterator::SpatiumExactSizeIterator;
use crate::error::SpatiumError;

type Result<T> = std::result::Result<T, SpatiumError>;

/// Distance trait on iterators
pub trait IteratorsDistance<T>
    where T: std::iter::Iterator
{

    /// Distance on spatium arguments
    fn iterators_distance(&self, x: SpatiumIterator<T>, y: SpatiumIterator<T>) -> Result<f64>;

    /// Distance between sequences
    fn distance<U>(&self, x: U, y: U) -> Result<f64>
    where
        U: Into<SpatiumIterator<T>>,
    {
        let x = x.into();
        let y = y.into();
        self.iterators_distance(x, y)
    }
}

/// Distance trait on const size iterators
pub trait ExactSizeIteratorsDistance<T>
    where T: std::iter::Iterator
{

    /// Distance on spatium arguments
    fn exact_size_iterators_distance(
	&self,
	x: SpatiumExactSizeIterator<T>,
	y: SpatiumExactSizeIterator<T>,
    ) -> Result<f64>;

    /// Distance between iteartors with exact size
    fn distance<U>(&self, x: U, y: U) -> Result<f64>
    where
        U: Into<SpatiumExactSizeIterator<T>>,
    {
        let x = x.into();
        let y = y.into();
        self.exact_size_iterators_distance(x, y)
    }

    /// Normalized distance between iterators with exact size
    fn normalized_distance<U>(&self, x: U, y: U) -> Result<f64>
    where
	U: Into<SpatiumExactSizeIterator<T>>,
    {
        let x = x.into();
        let y = y.into();

	let x_len = x.len();
	let y_len = y.len();

	let max_len = std::cmp::max(x_len, y_len) as f64;

        let distance = self.exact_size_iterators_distance(x, y)?;
	if distance != 0.0 && max_len == 0.0 {
	    return Err(SpatiumError::NormalizationError);
	}
	if distance == 0.0 {
	    return Ok(0.0);
	}

	Ok(distance / max_len)
    }

}

