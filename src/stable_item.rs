use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

pub struct HeapItem<T> {
    pub inner: T,
    pub counter: usize,
}

impl<T: Ord> HeapItem<T> {
    #[inline]
    pub fn new(inner: T, pos: usize) -> Self {
        HeapItem {
            inner,
            counter: pos,
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner
    }

    /*
    /// Get a reference to the heap item's inner.
    #[inline]
    pub fn inner(&self) -> &T {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Get a mutable reference to the heap item's counter.
    pub fn counter_mut(&mut self) -> &mut usize {
        &mut self.counter
    }
    */
}

impl<T> AsRef<T> for HeapItem<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> Deref for HeapItem<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for HeapItem<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Ord + PartialEq> PartialEq for HeapItem<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.counter == other.counter && self.inner == other.inner
    }
}

impl<T: Ord + PartialEq> Eq for HeapItem<T> {}

impl<T: Ord + PartialEq> PartialOrd for HeapItem<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let cmp = self.inner.cmp(&other.inner);
        if cmp == Ordering::Equal {
            return Some(self.counter.cmp(&other.counter).reverse());
        }

        Some(cmp)
    }
}

impl<T: Ord + PartialEq> Ord for HeapItem<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
