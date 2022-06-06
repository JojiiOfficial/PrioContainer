use crate::stable::item::HeapItem;
use std::{cmp::Reverse, collections::BinaryHeap};

/// Iterator over a binary heap sorted
pub struct SortedHeapIter<T> {
    inner: BinaryHeap<T>,
}

impl<T: Ord> SortedHeapIter<T> {
    #[inline]
    pub(crate) fn new(heap: BinaryHeap<T>) -> Self {
        Self { inner: heap }
    }
}

impl<T: Ord> Iterator for SortedHeapIter<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// Iterator over a binary heap sorted
pub struct SortedHeapIterMax<T> {
    inner: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> SortedHeapIterMax<T> {
    #[inline]
    pub(crate) fn new(heap: BinaryHeap<Reverse<T>>) -> Self {
        Self { inner: heap }
    }
}

impl<T: Ord> Iterator for SortedHeapIterMax<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.inner.pop().map(|i| i.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// Iterator over a binary heap sorted
pub struct StableHeapIter<T> {
    inner: BinaryHeap<HeapItem<T>>,
}

impl<T> StableHeapIter<T> {
    #[inline]
    pub(crate) fn new(heap: BinaryHeap<HeapItem<T>>) -> Self {
        Self { inner: heap }
    }
}

impl<T: Ord> Iterator for StableHeapIter<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.inner.pop().map(|i| i.into_inner())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}

/// Iterator over a binary heap sorted
pub struct StableHeapIterMax<T> {
    inner: BinaryHeap<HeapItem<Reverse<T>>>,
}

impl<T> StableHeapIterMax<T> {
    #[inline]
    pub(crate) fn new(heap: BinaryHeap<HeapItem<Reverse<T>>>) -> Self {
        Self { inner: heap }
    }
}

impl<T: Ord> Iterator for StableHeapIterMax<T> {
    type Item = T;

    #[inline(always)]
    fn next(&mut self) -> Option<T> {
        self.inner.pop().map(|i| i.into_inner().0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.inner.len();
        (exact, Some(exact))
    }
}
