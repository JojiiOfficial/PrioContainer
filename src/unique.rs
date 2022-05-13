use crate::{PrioContainer, SortedHeapIter};
use std::cmp::Reverse;

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct UniquePrioContainer<T> {
    container: PrioContainer<T>,
}

impl<T: Ord + PartialEq> UniquePrioContainer<T> {
    /// Create a new Unique PrioContainer
    #[inline]
    pub fn new(capacity: usize) -> Self {
        let container = PrioContainer::new(capacity);
        Self { container }
    }

    #[inline]
    pub fn insert(&mut self, item: T) -> bool {
        if self.container.heap.len() < self.container.capacity {
            if self.contains(&item) {
                return false;
            }
            self.container.pushed += 1;
            self.container.heap.push(item);
            return true;
        }

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let min_item = unsafe { self.container.heap.peek().unwrap_unchecked() };
        let contains = self.contains(&item);
        if *min_item <= item || contains {
            if !contains {
                self.container.pushed += 1;
            }
            return false;
        }

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

    /// Returns the total amount of pushed items. Ignores duplicates
    #[inline]
    pub fn total_pushed(&self) -> usize {
        self.container.total_pushed()
    }
}

impl<T: Ord> Extend<T> for UniquePrioContainer<T> {
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
        SortedHeapIter::new(self.container.heap)
    }
}

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct UniquePrioContainerMax<T> {
    container: UniquePrioContainer<Reverse<T>>,
}

impl<T: Ord + PartialEq> UniquePrioContainerMax<T> {
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

    /// Returns the total amount of pushed items. Ignores duplicates
    #[inline]
    pub fn total_pushed(&self) -> usize {
        self.container.total_pushed()
    }
}

impl<T: Ord> Extend<T> for UniquePrioContainerMax<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter.into_iter() {
            self.insert(i);
        }
    }
}

impl<T: Ord> IntoIterator for UniquePrioContainerMax<T> {
    type Item = Reverse<T>;
    type IntoIter = SortedHeapIter<Reverse<T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.container.into_iter()
    }
}
