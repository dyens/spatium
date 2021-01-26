use crate::types::iterator::SpatiumIterator;
use crate::distance::IteratorsDistance;
use crate::error::SpatiumError;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// Standart implementation of Hamming distance on iterators.
pub struct Algorithm {}

impl Default for Algorithm {
    fn default() -> Self {
        Self {}
    }
}

impl<T> IteratorsDistance<T> for Algorithm
where T: std::iter::Iterator,
        T::Item: std::cmp::Eq,
{
    fn iterators_distance(
        &self,
        mut x: SpatiumIterator<T>,
        mut y: SpatiumIterator<T>,
    ) -> Result<f64>
    {
        let mut distance: f64 = 0.0;

	loop {
	    match (x.next(), y.next()) {
		(None, None) => {break;},
	 	(Some(x_el), Some(y_el)) => {
	 	    if x_el != y_el {
	 		distance += 1.0;
	 	    }
	 	},
	 	 (None, Some(_)) | (Some(_), None) => {
	 	     return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
	 	 },
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

