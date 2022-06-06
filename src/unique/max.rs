use super::UniquePrioContainer;
use crate::iter::SortedHeapIterMax;
use std::{cmp::Reverse, hash::Hash};

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
/// This PrioContainer is stable
pub struct UniquePrioContainerMax<T> {
    container: UniquePrioContainer<Reverse<T>>,
}

impl<T: Ord + Clone + Hash> UniquePrioContainerMax<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = UniquePrioContainer::new(capacity);
        Self { container }
    }

    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new_allocated(capacity: usize) -> Self {
        let container = UniquePrioContainer::new_allocated(capacity);
        Self { container }
    }

    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        self.container.insert(Reverse(item))
    }
}

impl<T: Ord> UniquePrioContainerMax<T> {
    #[inline]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.container.into_iter().map(|i| i.0)
    }
}

impl<T> UniquePrioContainerMax<T> {
    #[inline]
    pub fn len(&self) -> usize {
        self.container.len()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.container.capacity()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.container.is_empty()
    }

    #[inline]
    pub fn total_pushed(&self) -> usize {
        self.container.total_pushed()
    }
}

impl<T: Ord + Clone + Hash> Extend<T> for UniquePrioContainerMax<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord + Clone + Hash> IntoIterator for UniquePrioContainerMax<T> {
    type Item = T;

    type IntoIter = SortedHeapIterMax<T>;

    fn into_iter(self) -> Self::IntoIter {
        SortedHeapIterMax::new(self.container.container)
    }
}
