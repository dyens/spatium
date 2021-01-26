/// Spatium exact size iterator
///
/// Argument with exact size iterator support for spatium distances
#[derive(Debug)]
pub struct SpatiumExactSizeIterator<T>
where
    T: std::iter::Iterator,
{
    _iterator: T,
    _len: usize,
}

impl<T> Iterator for SpatiumExactSizeIterator<T>
where
    T: std::iter::Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self._iterator.next()
    }
}

impl<T> ExactSizeIterator for SpatiumExactSizeIterator<T>
where
    T: std::iter::Iterator,
{
    fn len(&self) -> usize {
        self._len
    }
}


impl<'a, T, const K: usize> From<&'a [T; K]> for SpatiumExactSizeIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a [T; K]) -> Self {
        Self {
            _iterator: obj.iter(),
	    _len: obj.len(),
        }
    }
}

impl<'a, T> From<&'a [T]> for SpatiumExactSizeIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a [T]) -> Self {
        Self {
            _iterator: obj.iter(),
	    _len: obj.len(),
        }
    }
}


impl<'a, T> From<&'a Vec<T>> for SpatiumExactSizeIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a Vec<T>) -> Self {
        Self {
            _iterator: obj.iter(),
	    _len: obj.len(),
        }
    }
}




/// This is O(n) operation !
impl<'a> From<&'a str> for SpatiumExactSizeIterator<std::str::Chars<'a>> {
    fn from(obj: &'a str) -> Self {
        Self {
            _iterator: obj.chars(),
	    _len: obj.len(),
        }
    }
}

/// This is O(n) operation !
impl<'a> From<&'a String> for SpatiumExactSizeIterator<std::str::Chars<'a>> {
    fn from(obj: &'a String) -> Self {
        Self {
            _iterator: obj.chars(),
	    _len: obj.len(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::SpatiumExactSizeIterator;

    #[test]
    fn spatium_exact_size_iterator_from_array() {
	let x = [1,2,3];
	let y = SpatiumExactSizeIterator::from(&x);
	assert_eq!(x.iter().eq(y), true);
	
    }

    #[test]
    fn spatium_exact_size_iterator_from_slice() {
	let x = [1,2,3];
	let y = SpatiumExactSizeIterator::from(&x[..]);
	assert_eq!(x.iter().eq(y), true);
    }

    #[test]
    fn spatium_exact_size_iterator_from_vec() {
	let x = vec![1,2,3];
	let y = SpatiumExactSizeIterator::from(&x);
	assert_eq!(x.iter().eq(y), true);
    }

    #[test]
    fn spatium_exact_size_iterator_from_str() {
	let x = "Hello";
	let y = SpatiumExactSizeIterator::from(x);
	assert_eq!(x.chars().eq(y), true);
    }

    #[test]
    fn spatium_exact_soze_iterator_from_string() {
	let x = String::from("Hello");
	let y = SpatiumExactSizeIterator::from(&x);
	assert_eq!(x.chars().eq(y), true);
    }

}
 
