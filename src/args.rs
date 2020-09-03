use std::iter::ExactSizeIterator;

// TODO: implement from for this struct
// For algs with iterations and index accessing
// For example x[0] == y[0]
// Or some complex algs count(x[i] == y[y.len() - i])
#[derive(Debug)]
pub struct SpatiumIterator<T>
where
    T: std::iter::Iterator,
    T::Item: std::cmp::Eq,
{
    _iterator: T,
}

impl<'a, T, const K: usize> From<&'a [T; K]> for SpatiumIterator<std::slice::Iter<'a, T>>
where
    T: std::cmp::Eq,
{
    fn from(obj: &'a [T; K]) -> Self {
        Self {
            _iterator: obj.into_iter(),
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

impl<'a, T> From<&'a Vec<T>> for SpatiumIterator<std::slice::Iter<'a, T>>
where
    T: std::cmp::Eq,
{
    fn from(obj: &'a Vec<T>) -> Self {
        Self {
            _iterator: obj.into_iter(),
        }
    }
}

// For algs with iterations and len
// For example we can raise error in hamming if
// lens not equals
#[derive(Debug)]
pub struct SpatiumIteratorWithLen<T> {
    _iterator: T,
    _len: usize,
}

impl<T> Iterator for SpatiumIteratorWithLen<T>
where
    T: std::iter::Iterator,
    T::Item: std::cmp::Eq,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self._iterator.next()
    }
}

impl<T> ExactSizeIterator for SpatiumIteratorWithLen<T>
where
    T: std::iter::Iterator,
    T::Item: std::cmp::Eq,
{
    fn len(&self) -> usize {
        self._len
    }
}

impl<'a, T, const K: usize> From<&'a [T; K]> for SpatiumIteratorWithLen<std::slice::Iter<'a, T>>
where
    T: std::cmp::Eq,
{
    fn from(obj: &'a [T; K]) -> Self {
        Self {
            _iterator: obj.into_iter(),
            _len: obj.len(),
        }
    }
}

impl<'a> From<&'a str> for SpatiumIteratorWithLen<std::str::Chars<'a>> {
    fn from(obj: &'a str) -> Self {
        Self {
            _iterator: obj.chars(),
            _len: obj.len(),
        }
    }
}

impl<'a> From<&'a String> for SpatiumIteratorWithLen<std::str::Chars<'a>> {
    fn from(obj: &'a String) -> Self {
        Self {
            _iterator: obj.chars(),
            _len: obj.len(),
        }
    }
}

impl<'a, T> From<&'a Vec<T>> for SpatiumIteratorWithLen<std::slice::Iter<'a, T>>
where
    T: std::cmp::Eq,
{
    fn from(obj: &'a Vec<T>) -> Self {
        Self {
            _iterator: obj.into_iter(),
            _len: obj.len(),
        }
    }
}

// TODO: implement from for this struct
// For algs with iterations and index accessing
// For example x[0] == y[0]
// Or some complex algs count(x[i] == y[y.len() - i])
#[derive(Debug)]
pub struct SpatiumVector<T> {
    _iterator: Vec<T>,
}
