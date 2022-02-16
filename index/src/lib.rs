use std::fmt;
use std::marker::PhantomData;
use std::vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Idx<T> {
    idx: usize,
    _marker: PhantomData<T>,
}

impl<T> Idx<T> {
    #[inline]
    fn new(idx: usize) -> Self {
        Idx {
            idx: idx,
            _marker: PhantomData,
        }
    }

    #[inline]
    fn index(self) -> usize {
        self.idx
    }
}

impl<T> fmt::Display for Idx<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.idx)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexVec<T> {
    raw: Vec<T>,
}

impl<T> IndexVec<T> {
    #[inline]
    pub fn new() -> Self {
        IndexVec { raw: Vec::new() }
    }

    #[inline]
    pub fn push(&mut self, d: T) -> Idx<T> {
        let idx = Idx::new(self.raw.len());
        self.raw.push(d);

        idx
    }

    #[inline]
    pub fn get(&self, idx: Idx<T>) -> &T {
        &self.raw[idx.index()]
    }
}

impl<T> IntoIterator for IndexVec<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.raw.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_get() {
        let mut v = IndexVec::new();

        let i0 = v.push(10);
        let i1 = v.push(11);
        let i2 = v.push(12);

        assert_eq!(v.get(i0), &10);
        assert_eq!(v.get(i1), &11);
        assert_eq!(v.get(i2), &12);
    }
}