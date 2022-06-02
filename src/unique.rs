use crate::{PrioContainer, SortedHeapIter};
use std::{cmp::Reverse, collections::HashSet, hash::Hash};

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct UniquePrioContainer<T> {
    container: PrioContainer<T>,
    hash: HashSet<T>,
}

impl<T: Ord + PartialEq + Clone + Hash> UniquePrioContainer<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = PrioContainer::new(capacity);
        let hash = HashSet::with_capacity(capacity);
        Self { container, hash }
    }

    pub fn insert(&mut self, item: T) -> bool {
        if self.hash.contains(&item) {
            if let Some(old) = self.container.heap.iter().find(|i| **i == item) {
                if item < *old {
                    // TODO: Find a more efficient way to replace existing ones
                    let out: Vec<_> = self.container.heap.drain().filter(|i| *i != item).collect();
                    self.container.heap.extend(out);
                    self.container.heap.push(item);
                    return true;
                }
            }
            return false;
        }

        if self.container.heap.len() < self.container.capacity {
            self.container.heap.push(item.clone());
            self.hash.insert(item);
            return true;
        }

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let min_item = unsafe { self.container.heap.peek().unwrap_unchecked() };
        if *min_item <= item {
            return false;
        }

        self.hash.insert(item.clone());
        *unsafe { self.container.heap.peek_mut().unwrap_unchecked() } = item;
        true
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
    pub fn contains(&self, item: &T) -> bool {
        self.container.heap.iter().any(|i| i == item)
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

impl<T: Ord + Hash> IntoIterator for UniquePrioContainer<T> {
    type Item = T;
    type IntoIter = SortedHeapIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        SortedHeapIter::new(self.container.heap)
    }
}

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct UniquePrioContainerMax<T> {
    container: UniquePrioContainer<Reverse<T>>,
}

impl<T: Ord + PartialEq + Clone + Hash> UniquePrioContainerMax<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = UniquePrioContainer::new(capacity);
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
}

impl<T: Ord + Clone + Hash> Extend<T> for UniquePrioContainerMax<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord + Hash> IntoIterator for UniquePrioContainerMax<T> {
    type Item = Reverse<T>;
    type IntoIter = SortedHeapIter<Reverse<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}
