use std::collections::HashSet;

use priority_container::{unique::UniquePrioContainerMax, *};
use rand::{thread_rng, Rng};

fn generate_data(inp_len: usize) -> Vec<usize> {
    let mut input = vec![0usize; inp_len];
    thread_rng().try_fill(&mut input[..]).unwrap();
    input
}

fn test_unique(inp_len: usize, capacity: usize) {
    let mut input = generate_data(inp_len)
        .into_iter()
        // unique items only
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // add duplicates
    for val in input.clone().into_iter().step_by(10) {
        input.push(val);
    }

    let mut expected = input.clone();
    expected.sort();
    expected.reverse();
    expected.truncate(capacity);

    let mut prio_container = UniquePrioContainerMax::new(capacity);
    prio_container.extend(input);

    let collected: Vec<_> = prio_container.into_iter().collect();
    let unique = collected.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), collected.len());
}

fn test_max_with_capacity(inp_len: usize, capacity: usize) {
    let input = generate_data(inp_len);
    let mut expected = input.clone();
    expected.sort();
    expected.reverse();
    expected.truncate(capacity);

    let mut prio_container = PrioContainerMax::new(capacity);
    prio_container.extend(input);
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
fn test_other() {
    let mut queue = PrioContainer::new(2);
    queue.insert(3);
    queue.insert(5);
    queue.insert(10);
    assert_eq!(queue.into_sorted_vec(), vec![3, 5]);
}

#[test]
fn test() {
    for inp_len in (0..2000).step_by(51) {
        for cap in (1..2000).step_by(61) {
            test_min_with_capacity(inp_len, cap);
            test_max_with_capacity(inp_len, cap);
            test_unique(inp_len, cap);
        }
    }
}

#[test]
fn test_stability_simple() {
    let mut heap = StablePrioContainer::new(10);

    heap.insert(UniqueItem::new("9", 3));
    heap.insert(UniqueItem::new("8", 2));
    heap.insert(UniqueItem::new("7", 2));
    heap.insert(UniqueItem::new("a", 1));
    heap.insert(UniqueItem::new("b", 1));
    heap.insert(UniqueItem::new("c", 1));
    heap.insert(UniqueItem::new("d", 1));
    heap.insert(UniqueItem::new("e", 0));

    let out = heap.into_iter().map(|i| i.item).collect::<Vec<_>>();
    assert_eq!(out, vec!["9", "8", "7", "a", "b", "c", "d", "e"]);
}

#[derive(Clone)]
struct UniqueItem<T> {
    item: T,
    val: u32,
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
