use std::collections::BinaryHeap;

/// Priority container storing max `capacity` amount of items. Can be used to find
/// `n` smallest items within an iterator or a set of items that implement `Ord`
pub struct PrioContainer<T> {
    heap: BinaryHeap<T>,
    /// Max amount of items that will be returned in the end
    capacity: usize,
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
        let heap = BinaryHeap::with_capacity(capacity);
        Self { heap, capacity }
    }

    /// Inserts a new Item into the queue.
    #[inline]
    pub fn insert(&mut self, item: T) {
        if self.heap.len() < self.capacity {
            self.heap.push(item);
            return;
        }

        // Safety:
        //
        // heap.len() >= n without elements is impossible for n>0 which is enforced in `PrioContainer::new()`
        let mut min_item = unsafe { self.heap.peek_mut().unwrap_unchecked() };
        if *min_item > item {
            *min_item = item;
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{thread_rng, Rng};
    use std::cmp::Reverse;

    fn generate_data(inp_len: usize) -> Vec<usize> {
        let mut input = vec![0usize; inp_len];
        thread_rng().try_fill(&mut input[..]).unwrap();
        input
    }

    fn test_max_with_capacity(inp_len: usize, capacity: usize) {
        let input = generate_data(inp_len);
        let mut expected = input.clone();
        expected.sort();
        expected.reverse();
        expected.truncate(capacity);

        let mut prio_container = PrioContainer::new(capacity);
        prio_container.extend(input.into_iter().map(|i| Reverse(i)));
        let mut out = prio_container.into_iter().map(|i| i.0).collect::<Vec<_>>();
        out.reverse();
        assert_eq!(out, expected);
    }

    fn test_min_with_capacity(inp_len: usize, capacity: usize) {
        let input = generate_data(inp_len);
        let mut expected = input.clone();
        expected.sort();
        expected.truncate(capacity);

        let mut prio_container = PrioContainer::new(capacity);
        prio_container.extend(input);
        let mut out = prio_container.into_iter().collect::<Vec<_>>();
        out.reverse();
        assert_eq!(out, expected);
    }

    #[test]
    fn test() {
        for inp_len in (0..10000).step_by(300) {
            for cap in (1..10000).step_by(400) {
                test_min_with_capacity(inp_len, cap);
                test_max_with_capacity(inp_len, cap);
            }
        }
    }
}

/// Iterator over a binary heap sorted
pub struct SortedHeapIter<T> {
    inner: BinaryHeap<T>,
}

impl<T: Ord> SortedHeapIter<T> {
    #[inline]
    fn new(heap: BinaryHeap<T>) -> Self {
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
