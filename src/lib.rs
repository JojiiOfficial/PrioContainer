pub mod iter;
pub mod stable;
pub mod unique;

pub use stable::{max::StablePrioContainerMax, StablePrioContainer};
pub use unique::{
    max::UniquePrioContainerMax, stable::StableUniquePrioContainer,
    stable_max::StableUniquePrioContainerMax, UniquePrioContainer,
};

use iter::SortedHeapIter;
use std::{cmp::Reverse, collections::BinaryHeap};

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct PrioContainerMax<T> {
    container: PrioContainer<Reverse<T>>,
}

impl<T: Ord> PrioContainerMax<T> {
    /// Create a new Max PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = PrioContainer::new(capacity);
        Self { container }
    }

    /// Create a new Max PrioContainer
    #[inline]
    pub fn new_allocated(capacity: usize) -> Self {
        let container = PrioContainer::new_allocated(capacity);
        Self { container }
    }

    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        self.container.insert(Reverse(item))
    }

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

impl<T: Ord> Extend<T> for PrioContainerMax<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord> IntoIterator for PrioContainerMax<T> {
    type Item = Reverse<T>;

    type IntoIter = SortedHeapIter<Reverse<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SortedHeapIter::new(self.container.heap)
    }
}

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct PrioContainer<T> {
    heap: BinaryHeap<T>,
    /// Max amount of items that will be returned in the end
    capacity: usize,
    pushed: usize,
}

impl<T: Ord> PrioContainer<T> {
    /// Create a new PrioContainerMin with `capacity`
    ///
    /// # Panics
    /// Panics if `capacity` is zero
    #[inline]
    pub fn new(capacity: usize) -> Self {
        if capacity == 0 {
            panic!("Capacity can't be zero");
        }
        let heap = BinaryHeap::new();
        Self {
            heap,
            capacity,
            pushed: 0,
        }
    }

    /// Create a new PrioContainerMin with already allocated spaces
    ///
    /// # Panics
    /// Panics if `capacity` is zero
    #[inline]
    pub fn new_allocated(capacity: usize) -> Self {
        let mut queue = Self::new(capacity);
        queue.heap.reserve(capacity);
        queue
    }

    /// Inserts a new Item into the queue.
    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        self.pushed += 1;
        if self.heap.len() < self.capacity {
            self.heap.push(item);
            return true;
        }

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let mut max_item = unsafe { self.heap.peek_mut().unwrap_unchecked() };
        if *max_item <= item {
            return false;
        }

        *max_item = item;
        true
    }

    /// Returns the amount of items in the container. This value
    /// is always smaller or equal to `capacity`
    #[inline]
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns `true` if there is no item in the container
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the prio container's capacity
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the total amount of items pushed into the prio container
    #[inline]
    pub fn total_pushed(&self) -> usize {
        self.pushed
    }

    /// Return a sorted vec of the prio container
    #[inline]
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.heap.into_sorted_vec()
    }
}

impl<T: Ord> Extend<T> for PrioContainer<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord> IntoIterator for PrioContainer<T> {
    type Item = T;

    type IntoIter = SortedHeapIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SortedHeapIter::new(self.heap)
    }
}
