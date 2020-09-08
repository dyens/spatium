use crate::core::SpatiumIteratorWithLen;
use crate::distance::IteratorsWithLenDistance;
use crate::error::SpatiumError;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// Standart implementation of Hamming distance.
pub struct Hamming1 {}

impl Default for Hamming1 {
    fn default() -> Self {
        Self {}
    }
}

impl IteratorsWithLenDistance for Hamming1 {
    fn spatium_distance<T>(
        &self,
        x: SpatiumIteratorWithLen<T>,
        y: SpatiumIteratorWithLen<T>,
    ) -> Result<f64>
    where
        T: std::iter::Iterator,
        T::Item: std::cmp::Eq,
    {
        let mut distance: f64 = 0.0;

        if x.len() != y.len() {
            return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
        }

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
    use super::Hamming1;
    use crate::distance::IteratorsWithLenDistance;

    #[test]
    fn error_text() {
        let x = vec![1, 2, 3];
        let y = vec![1, 2, 3, 4];
        assert_eq!(
            Hamming1::default()
                .distance(&x, &y)
                .unwrap_err()
                .to_string(),
            "ValueError: Arguments have different lengths."
        );
    }

    #[test]
    fn on_slices() {
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = Hamming1::default().distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_strs() {
        let x = "Привет";
        let y = "Привёт";
        let distance = Hamming1::default().distance(x, y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_strings() {
        let x = String::from("Привет");
        let y = String::from("Привёт");
        let distance = Hamming1::default().distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_vecs() {
        let x = vec![1, 2, 3];
        let y = vec![1, 3, 3];
        let distance = Hamming1::default().distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_unicode_strings() {
        let x = "Привет";
        let y = "Привёт";
        let xc = x.chars().collect::<Vec<char>>();
        let yc = y.chars().collect::<Vec<char>>();
        let distance = Hamming1::default().distance(&xc, &yc).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn normalized_result() {
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = Hamming1::default().normalized_distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0 / 3.0);
    }
}
