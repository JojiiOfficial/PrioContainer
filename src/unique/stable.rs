use crate::{iter::StableHeapIter, stable::item::HeapItem, StablePrioContainer};
use std::{collections::HashSet, hash::Hash};

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`.
/// This PrioContainer is stable
pub struct StableUniquePrioContainer<T> {
    pub(crate) container: StablePrioContainer<T>,
    pub(crate) hash: HashSet<T>,
}

impl<T: Ord + Clone + Hash> StableUniquePrioContainer<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = StablePrioContainer::new(capacity);
        let hash = HashSet::new();
        Self { container, hash }
    }

    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new_allocated(capacity: usize, alloc_size: usize) -> Self {
        let container = StablePrioContainer::new_allocated(capacity, alloc_size);
        let hash = HashSet::with_capacity(alloc_size);
        Self { container, hash }
    }

    /// Inserts a new intem into the StableUniquePrioContainer
    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        if self.hash.contains(&item) {
            self.replace_eq(item);
            return false;
        }

        self.hash.insert(item.clone());
        self.container.insert(item)
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        self.container.contains(item)
    }

    /// Replaces an already pushed item with `item` if their hashes are equal
    /// and `item`'s relevance is bigger
    fn replace_eq(&mut self, item: T) {
        let need_replace = self
            .container
            .heap
            .iter()
            .find(|i| *i.as_ref() == item && item < *i.as_ref())
            .map(|i| i.counter);

        if let Some(old_counter) = need_replace {
            let add: Vec<_> = self
                .container
                .heap
                .drain()
                .filter(|i| *i.as_ref() != item)
                .collect();

            self.container.heap.extend(add);
            self.container.heap.push(HeapItem::new(item, old_counter));
        }
    }
}

impl<T> StableUniquePrioContainer<T> {
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
        self.container.total_pushed
    }
}

impl<T: Ord + Clone + Hash> Extend<T> for StableUniquePrioContainer<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord + Clone + Hash> IntoIterator for StableUniquePrioContainer<T> {
    type Item = T;

    type IntoIter = StableHeapIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        StableHeapIter::new(self.container.heap)
    }
}
