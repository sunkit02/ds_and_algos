use criterion::{criterion_group, criterion_main, Criterion};
use ds_and_algos::datastructures::lru_cache::LRUCache;

pub fn lru_cache_get(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get last item in LRU Cache");

    for limit in [1000, 10000, 100000] {
        let items: Vec<i32> = (1..limit).collect();
        let mut cache = LRUCache::new(limit as usize);

        items.iter().for_each(|n| cache.set(n, n));
        group.bench_function(format!("{} items", limit), |b| {
            b.iter(|| {
                for item in 1i32..500 {
                    cache.get(&item);
                }
            })
        });
    }

    group.finish();
}

pub fn lru_cache_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("Set last item in LRU Cache");

    for limit in [1000, 10000, 100000] {
        let items: Vec<i32> = (1..limit).collect();
        let mut cache = LRUCache::new(limit as usize);

        items.iter().for_each(|n| cache.set(n, *n));
        let first_500 = items.iter().take(500);
        let second_500 = items.iter().filter(|&&n| n >= 500).take(500);
        group.bench_function(format!("{} items", limit), |b| {
            b.iter(|| {
                for item in first_500.clone() {
                    cache.set(&item, *item);
                }

                for item in second_500.clone() {
                    cache.set(&item, *item);
                }
            })
        });
    }

    group.finish();
}

pub fn lru_cache_overcharge(c: &mut Criterion) {
    let mut group = c.benchmark_group("Set last item in LRU Cache");

    for limit in [1000, 10000, 100000] {
        let items: Vec<i32> = (1..limit).collect();
        let overcharge_items: Vec<i32> = (limit..500).collect();

        let mut cache = LRUCache::new(limit as usize);

        items.iter().for_each(|n| cache.set(n, *n));

        let overcharge_iter = overcharge_items.iter();
        group.bench_function(format!("{} items", limit), |b| {
            b.iter(|| {
                for item in overcharge_iter.clone() {
                    cache.set(&item, *item);
                }
            })
        });
    }

    group.finish();
}

criterion_group!(benches, lru_cache_get, lru_cache_set, lru_cache_overcharge);
criterion_main!(benches);
