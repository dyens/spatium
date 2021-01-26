use crate::types::exact_size_iterator::SpatiumExactSizeIterator;
use crate::distance::ExactSizeIteratorsDistance;
use crate::error::SpatiumError;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// Standart implementation of Hamming distance on const  size iterators.
pub struct Algorithm {}

impl Default for Algorithm {
    fn default() -> Self {
        Self {}
    }
}

impl<T> ExactSizeIteratorsDistance<T> for Algorithm
where T: std::iter::Iterator,
        T::Item: std::cmp::Eq,
{
    fn exact_size_iterators_distance(
        &self,
        x: SpatiumExactSizeIterator<T>,
        y: SpatiumExactSizeIterator<T>,
    ) -> Result<f64>
    {
	if x.len() != y.len() {
	    return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
	}

        let mut distance: f64 = 0.0;
	for (x_el, y_el) in x.zip(y) {
	    if x_el != y_el {
		distance += 1.0;
	    }
	}
        Ok(distance)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_text() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2, 3, 4];
        assert_eq!(
            Algorithm::default()
                .distance(&x, &y)
                .unwrap_err()
                .to_string(),
            "ValueError: Arguments have different lengths."
        );
    }

    #[test]
    fn on_str() {
        let x = "Привет";
        let y = "Прирет";
        assert_eq!(
            Algorithm::default().distance(x, y).unwrap(), 1.0
        );
    }

    #[test]
    fn on_strings() {
        let x = String::from("Привет");
        let y = String::from("Прирет");
        assert_eq!(
            Algorithm::default().distance(&x, &y).unwrap(), 1.0
        );
    }


    #[test]
    fn on_slices() {
        let x = [1, 2, 3];
        let y = [1, 3, 3];
        assert_eq!(
            Algorithm::default().distance(&x, &y).unwrap(), 1.0
        );
    }

}

