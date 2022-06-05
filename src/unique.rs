use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    hash::Hash,
};

use crate::stable_item::HeapItem;

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`.
/// This PrioContainer is stable
pub struct UniquePrioContainer<T> {
    container: BinaryHeap<HeapItem<T>>,
    hash: HashSet<T>,
    total_pushed: usize,
    capacity: usize,
}

impl<T: Ord + PartialEq + Clone + Hash> UniquePrioContainer<T> {
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
            let need_replace = self
                .container
                .iter()
                .find(|i| *i.as_ref() == item && item < *i.as_ref())
                .map(|i| i.counter);

            if let Some(old_counter) = need_replace {
                let add: Vec<_> = self
                    .container
                    .drain()
                    .filter(|i| *i.as_ref() != item)
                    .collect();

                self.container.extend(add);
                self.container.push(HeapItem::new(item, old_counter));
            }

            return false;
        }

        self.hash.insert(item.clone());

        if self.container.len() < self.capacity {
            self.container.push(HeapItem::new(item, self.total_pushed));
            self.total_pushed += 1;
            return true;
        }

        let new_item = HeapItem::new(item, self.total_pushed);

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let min_item = unsafe { self.container.peek().unwrap_unchecked() };
        if *min_item <= new_item {
            self.total_pushed += 1;
            return false;
        }

        *unsafe { self.container.peek_mut().unwrap_unchecked() } = new_item;
        self.total_pushed += 1;

        true
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        self.container.iter().any(|i| i.as_ref() == item)
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

/// Iterator over a binary heap sorted
pub struct SortedHeapIter<T> {
    inner: BinaryHeap<HeapItem<T>>,
}

impl<T> SortedHeapIter<T> {
    #[inline]
    fn new(heap: BinaryHeap<HeapItem<T>>) -> Self {
        Self { inner: heap }
    }
}

impl<T: Ord> Iterator for SortedHeapIter<T> {
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

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
/// This PrioContainer is stable
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

#[cfg(test)]
mod tests {
    use std::hash::Hash;

    use super::UniquePrioContainerMax;

    fn make_invariant_test(len: usize, max: usize) -> UniquePrioContainerMax<UniqueItem<usize>> {
        let mut heap = UniquePrioContainerMax::new(max);

        let to_add = (0..len)
            .into_iter()
            .map(|i| UniqueItem::new(i, i as u32))
            .collect::<Vec<_>>();
        heap.extend(to_add);

        heap
    }

    #[test]
    fn test1() {
        for i in (10..100).step_by(10) {
            let container = make_invariant_test(i, 100);
            let expected = (0..i).collect::<Vec<_>>();
            assert_eq!(
                container.into_iter().map(|i| i.item).collect::<Vec<_>>(),
                expected
            );
        }
    }

    struct UniqueItem<T> {
        item: T,
        val: u32,
    }

    impl<T: Hash> Hash for UniqueItem<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.item.hash(state);
        }
    }

    impl<T> UniqueItem<T> {
        fn new(item: T, val: u32) -> Self {
            Self { item, val }
        }
    }

    impl<T> PartialEq for UniqueItem<T> {
        #[inline]
        fn eq(&self, other: &Self) -> bool {
            self.val == other.val
        }
    }

    impl<T> Eq for UniqueItem<T> {}

    impl<T> PartialOrd for UniqueItem<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.val.partial_cmp(&other.val)
        }
    }

    impl<T> Ord for UniqueItem<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.val.cmp(&other.val)
        }
    }

    impl<T: Clone> Clone for UniqueItem<T> {
        #[inline]
        fn clone(&self) -> Self {
            Self {
                item: self.item.clone(),
                val: self.val,
            }
        }
    }
}
