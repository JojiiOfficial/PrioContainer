pub mod item;
pub mod max;

use self::item::HeapItem;
use crate::iter::StableHeapIter;
use std::collections::BinaryHeap;

/// A stable priority container. This means equal elements are returned in inserted order
pub struct StablePrioContainer<T> {
    pub(crate) heap: BinaryHeap<HeapItem<T>>,
    pub(crate) total_pushed: usize,
    pub(crate) capacity: usize,
}

impl<T: Ord> StablePrioContainer<T> {
    /// Creates a new StablePrioContainer with given max items. This value must not be smaller than 1
    ///
    /// # Panics
    /// Panics if `capacity` is 0
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        let heap = BinaryHeap::new();

        StablePrioContainer {
            heap,
            total_pushed: 0,
            capacity,
        }
    }

    /// Create a new StablePrioContainer with given preallocated size. `capacity` must not be smaller than 1
    ///
    /// # Panics
    /// Panics if `capacity` is 0
    pub fn new_allocated(capacity: usize, alloc_size: usize) -> Self {
        assert!(capacity > 0);

        // We'll never allocate more items than `capacity` so prevent stupid input
        // doin stupid things
        let alloc_size = alloc_size.min(capacity);

        let heap = BinaryHeap::with_capacity(alloc_size);

        StablePrioContainer {
            heap,
            total_pushed: 0,
            capacity,
        }
    }

    /// Pushes a new element into the PrioContainer
    pub fn insert(&mut self, item: T) -> bool {
        self.total_pushed += 1;
        if self.heap.len() < self.capacity {
            self.heap.push(HeapItem::new(item, self.total_pushed));
            return true;
        }

        let new_item = HeapItem::new(item, self.total_pushed);

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let min_item = unsafe { self.heap.peek().unwrap_unchecked() };
        if *min_item <= new_item {
            return false;
        }

        *unsafe { self.heap.peek_mut().unwrap_unchecked() } = new_item;

        true
    }

    #[inline]
    pub fn inc_push(&mut self, delta: usize) {
        self.total_pushed += delta;
    }

    #[inline]
    pub fn contains(&self, item: &T) -> bool {
        self.heap.iter().any(|i| *i.as_ref() == *item)
    }

    /// Return a sorted vec of the prio container
    #[inline]
    pub fn into_sorted_vec(self) -> Vec<T> {
        self.into_iter().collect()
    }
}

impl<T> StablePrioContainer<T> {
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
        self.total_pushed
    }
}

impl<T: Ord> IntoIterator for StablePrioContainer<T> {
    type Item = T;

    type IntoIter = StableHeapIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        StableHeapIter::new(self.heap)
    }
}

impl<T: Ord> Extend<T> for StablePrioContainer<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for i in iter {
            self.insert(i);
        }
    }
}
