use crate::error::SpatiumError;
use crate::normalize::normalize;
use std::cmp::Eq;

const DIFFERENT_LENGTHS: &str = "Arguments have different lengths.";

type Result<T> = std::result::Result<T, SpatiumError>;

/// Standart implementation of Hamming distance.
pub struct Hamming1 {
    /// Is result should be normalized?
    normalized: bool,
}

impl Default for Hamming1 {
    fn default() -> Self {
        Self { normalized: false }
    }
}

impl Hamming1 {
    fn calc_distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq,
    {
        if x.len() != y.len() {
            return Err(SpatiumError::ValueError(String::from(DIFFERENT_LENGTHS)));
        }
        let mut distance: f64 = 0.0;
        for (x_el, y_el) in x.iter().zip(y.iter()) {
            if x_el != y_el {
                distance += 1.0;
            }
        }
        Ok(distance)
    }

    /// Set normalization parameter
    ///
    /// If normailzed is True,
    /// the distance is normalized by dividing it
    pub fn normalize_result(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
    }

    /// Distance between sequences
    pub fn distance<T>(&self, x: &[T], y: &[T]) -> Result<f64>
    where
        T: Eq,
    {
        let distance = self.calc_distance(x, y);

        if self.normalized {
            distance.and_then(|dis| normalize(dis, x.len(), y.len()))
        } else {
            distance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Hamming1;

    #[test]
    fn error_text() {
        let x = [1, 2, 3];
        let y = [1, 2, 4, 5];
        assert_eq!(
            Hamming1::default()
                .calc_distance(&x, &y)
                .unwrap_err()
                .to_string(),
            "ValueError: Arguments have different lengths."
        );
    }

    #[test]
    fn on_slices() {
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = Hamming1::default().calc_distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn on_unicode_strings() {
        let x = "Привет";
        let y = "Привёт";
        let xc = x.chars().collect::<Vec<char>>();
        let yc = y.chars().collect::<Vec<char>>();
        let distance = Hamming1::default().calc_distance(&xc, &yc).unwrap();
        assert_eq!(distance, 1.0);
    }

    #[test]
    fn normalize_result() {
        let alg = Hamming1::default().normalize_result(true);
        let x = [1, 2, 3];
        let y = [1, 2, 4];
        let distance = alg.distance(&x, &y).unwrap();
        assert_eq!(distance, 1.0 / 3.0);
    }
}
