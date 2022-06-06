pub mod max;
pub mod stable;
pub mod stable_max;

use std::{
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use crate::iter::SortedHeapIter;

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`.
/// This PrioContainer is stable
pub struct UniquePrioContainer<T> {
    container: BinaryHeap<T>,
    hash: HashSet<T>,
    total_pushed: usize,
    capacity: usize,
}

impl<T: Ord + Clone + Hash> UniquePrioContainer<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = BinaryHeap::new();
        let hash = HashSet::new();

        Self {
            container,
            hash,
            total_pushed: 0,
            capacity,
        }
    }

    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new_allocated(capacity: usize) -> Self {
        let container = BinaryHeap::with_capacity(capacity);
        let hash = HashSet::with_capacity(capacity);

        Self {
            container,
            hash,
            total_pushed: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, item: T) -> bool {
        if self.hash.contains(&item) {
            self.replace_eq(item);
            return false;
        }

        self.hash.insert(item.clone());

        if self.container.len() < self.capacity {
            self.container.push(item);
            self.total_pushed += 1;
            return true;
        }

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let min_item = unsafe { self.container.peek().unwrap_unchecked() };
        if *min_item <= item {
            self.total_pushed += 1;
            return false;
        }

        *unsafe { self.container.peek_mut().unwrap_unchecked() } = item;
        self.total_pushed += 1;

        true
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        self.container.iter().any(|i| *i == *item)
    }

    /// Replaces an already pushed item with `item` if their hashes are equal
    /// and `item`'s relevance is bigger
    fn replace_eq(&mut self, item: T) {
        let need_replace = self.container.iter().any(|i| *i == item && item < *i);

        if need_replace {
            let add: Vec<_> = self.container.drain().filter(|i| *i != item).collect();
            self.container.extend(add);
            self.container.push(item);
        }
    }
}

impl<T> UniquePrioContainer<T> {
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
        self.total_pushed
    }
}

impl<T: Ord + Clone + Hash> Extend<T> for UniquePrioContainer<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord> IntoIterator for UniquePrioContainer<T> {
    type Item = T;
    type IntoIter = SortedHeapIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SortedHeapIter::new(self.container)
    }
}
