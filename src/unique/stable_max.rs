use super::stable::StableUniquePrioContainer;
use crate::iter::StableHeapIterMax;
use std::{cmp::Reverse, hash::Hash};

/// A stable priority container max. This means equal elements are returned in inserted order
pub struct StableUniquePrioContainerMax<T> {
    heap: StableUniquePrioContainer<Reverse<T>>,
}

impl<T: Ord + Hash + Clone> StableUniquePrioContainerMax<T> {
    /// Creates a new StablePrioContainer with given max items. This value must not be smaller than 1
    ///
    /// # Panics
    /// Panics if `capacity` is 0
    pub fn new(capacity: usize) -> Self {
        let heap = StableUniquePrioContainer::new(capacity);
        StableUniquePrioContainerMax { heap }
    }

    /// Create a new StablePrioContainer with given preallocated size. `capacity` must not be smaller than 1
    ///
    /// # Panics
    /// Panics if `capacity` is 0
    pub fn new_allocated(capacity: usize, alloc_size: usize) -> Self {
        let heap = StableUniquePrioContainer::new_allocated(capacity, alloc_size);
        StableUniquePrioContainerMax { heap }
    }

    /// Pushes a new element into the PrioContainer
    pub fn insert(&mut self, item: T) -> bool {
        self.heap.insert(Reverse(item))
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        self.heap
            .container
            .heap
            .iter()
            .any(|i| i.as_ref().0 == *item)
    }

    /// Return a sorted vec of the prio container
    #[inline]
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }

    #[inline]
    pub fn inc_push(&mut self, delta: usize) {
        self.heap.inc_push(delta)
    }
}

impl<T> StableUniquePrioContainerMax<T> {
    /// Returns the amount of items currently stored in the PrioContainer
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns `true` if no items have been pushed onto the PrioContainer
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.heap.capacity()
    }

    #[inline]
    pub fn total_pushed(&self) -> usize {
        self.heap.total_pushed()
    }
}

impl<T: Ord> IntoIterator for StableUniquePrioContainerMax<T> {
    type Item = T;

    type IntoIter = StableHeapIterMax<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        StableHeapIterMax::new(self.heap.container.heap)
    }
}

impl<T: Ord + Hash + Clone> Extend<T> for StableUniquePrioContainerMax<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}
