use criterion::{criterion_group, criterion_main, Criterion};
use priority_container::UniquePrioContainerMax;
use rand::prelude::*;

fn overlapping(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(1000);

    let mut prio_queue = UniquePrioContainerMax::<u64>::new(1000);

    for _ in 0..10000 {
        prio_queue.insert(rng.next_u64());
    }

    c.bench_function("similarity", |b| {
        let nr = rng.next_u64();
        b.iter(|| {
            prio_queue.insert(nr);
        })
    });
}

criterion_group!(benches, overlapping);
criterion_main!(benches);
