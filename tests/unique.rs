use std::hash::Hash;

use priority_container::{unique::max::UniquePrioContainerMax, StableUniquePrioContainer};

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

#[test]
fn test_stability_simple_unique() {
    for _ in 0..1000 {
        println!("-------");
        let mut heap = StableUniquePrioContainer::new(10);

        for _ in 0..20 {
            heap.insert(UniqueItem::new("9", 2));
        }
        assert_eq!(heap.len(), 1);

        heap.insert(UniqueItem::new("9", 2));
        heap.insert(UniqueItem::new("9", 2));
        assert_eq!(heap.len(), 1);

        heap.insert(UniqueItem::new("8", 2));
        assert_eq!(heap.len(), 2);

        for _ in 0..4 {
            heap.insert(UniqueItem::new("7", 2));
            heap.insert(UniqueItem::new("a", 1));
            heap.insert(UniqueItem::new("b", 1));
            heap.insert(UniqueItem::new("c", 1));
            heap.insert(UniqueItem::new("d", 1));
        }
        heap.insert(UniqueItem::new("7", 2));
        heap.insert(UniqueItem::new("e", 0));
        heap.insert(UniqueItem::new("e", 0));

        let out = heap.into_iter().map(|i| i.item).collect::<Vec<_>>();
        assert_eq!(out, vec!["9", "8", "7", "a", "b", "c", "d", "e"]);
    }
}
