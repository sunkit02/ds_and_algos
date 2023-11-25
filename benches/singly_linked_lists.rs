use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ds_and_algos::datastructures::linked_lists::singly_linked_list;
use ds_and_algos::datastructures::linked_lists::unsafe_singly_linked_list;

pub fn safe_singly_linked_list(c: &mut Criterion) {
    let vec: Vec<_> = (0..1000).collect();
    c.bench_function("safe_singly_linked_list", |b| {
        b.iter(|| {
            black_box({
                let mut list = singly_linked_list::SinglyLinkedList::from_iter(vec.clone());
                for i in 0..list.len() {
                    let _ = list.get(i);
                }
                list.clear();
            });
        })
    });
}

pub fn unsafe_singly_linked_list(c: &mut Criterion) {
    let vec: Vec<_> = (0..1000).collect();
    c.bench_function("unsafe_singly_linked_list", |b| {
        b.iter(|| {
            black_box({
                let mut list = unsafe_singly_linked_list::SinglyLinkedList::from_iter(vec.clone());
                for i in 0..list.len() {
                    let _ = list.get(i);
                }
                list.clear();
            });
        })
    });
}

criterion_group!(benches, safe_singly_linked_list, unsafe_singly_linked_list);
criterion_main!(benches);
