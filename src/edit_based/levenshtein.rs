use crate::error::SpatiumError;
use std::cmp::Eq;

use std::cmp::min;

// type Result<T> = std::result::Result<T, SpatiumError>;

/// #Levenshtein recursive algorithm
///
/// This is a straightforward, but inefficient, recursive
/// implementation of a Levenshtein distance.
pub fn levenshtein_recursive<T>(x: &[T], y: &[T]) -> u64
where
    T: Eq,
{
    let x_len = x.len();
    let y_len = y.len();
    if x_len == 0 {
        return y_len as u64;
    }
    if y_len == 0 {
        return x_len as u64;
    }

    let mut cost = 1;
    if x[x_len - 1] == y[y_len - 1] {
        cost = 0;
    }

    return min(
        min(
            levenshtein_recursive(&x[..x_len - 1], &y) + 1,
            levenshtein_recursive(&x, &y[..y_len - 1]) + 1,
        ),
        levenshtein_recursive(&x[..x_len - 1], &y[..y_len - 1]) + cost,
    );
}

#[cfg(test)]
mod tests {
    use super::levenshtein_recursive;

    #[test]
    fn test_error_text() {
        let x = [1, 5, 3];
        let y = [4, 5, 6, 7];

        assert_eq!(levenshtein_recursive(&x, &y), 3);
    }
}
