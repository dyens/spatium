/// Spatium iterator
///
/// Argument with iterator support for spatium distances
#[derive(Debug)]
pub struct SpatiumIterator<T>
where
    T: std::iter::Iterator,
{
    _iterator: T,
}

impl<T> Iterator for SpatiumIterator<T>
where
    T: std::iter::Iterator,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self._iterator.next()
    }
}

impl<'a, T, const K: usize> From<&'a [T; K]> for SpatiumIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a [T; K]) -> Self {
        Self {
            _iterator: obj.iter(),
        }
    }
}

impl<'a, T> From<&'a [T]> for SpatiumIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a [T]) -> Self {
        Self {
            _iterator: obj.iter(),
        }
    }
}


impl<'a, T> From<&'a Vec<T>> for SpatiumIterator<std::slice::Iter<'a, T>>
{
    fn from(obj: &'a Vec<T>) -> Self {
        Self {
            _iterator: obj.iter(),
        }
    }
}




impl<'a> From<&'a str> for SpatiumIterator<std::str::Chars<'a>> {
    fn from(obj: &'a str) -> Self {
        Self {
            _iterator: obj.chars(),
        }
    }
}

impl<'a> From<&'a String> for SpatiumIterator<std::str::Chars<'a>> {
    fn from(obj: &'a String) -> Self {
        Self {
            _iterator: obj.chars(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::SpatiumIterator;

    #[test]
    fn spatium_iterator_from_array() {
	let x = [1,2,3];
	let y = SpatiumIterator::from(&x);
	assert_eq!(x.iter().eq(y), true);
	
    }

    #[test]
    fn spatium_iterator_from_slice() {
	let x = [1,2,3];
	let y = SpatiumIterator::from(&x[..]);
	assert_eq!(x.iter().eq(y), true);
    }

    #[test]
    fn spatium_iterator_from_vec() {
	let x = vec![1,2,3];
	let y = SpatiumIterator::from(&x);
	assert_eq!(x.iter().eq(y), true);
    }

    #[test]
    fn spatium_iterator_from_str() {
	let x = "Hello";
	let y = SpatiumIterator::from(x);
	assert_eq!(x.chars().eq(y), true);
    }

    #[test]
    fn spatium_iterator_from_string() {
	let x = String::from("Hello");
	let y = SpatiumIterator::from(&x);
	assert_eq!(x.chars().eq(y), true);
    }

}
 
