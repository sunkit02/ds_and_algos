use criterion::{criterion_group, criterion_main, Criterion};
use ds_and_algos::datastructures::linked_lists::unsafe_singly_linked_list;
use ds_and_algos::datastructures::linked_lists::unsafe_doubly_linked_list;

pub fn traverse_singly_linked_list(c: &mut Criterion) {
    let vec: Vec<usize> = (0..10000).collect();
    let list = unsafe_singly_linked_list::SinglyLinkedList::from_iter(vec.clone());
    c.bench_function("Traverse singly_linked_list", |b| b.iter(|| {
        let mut n = 0;
        for i in 0..list.len() {
            n += list.get(i).unwrap();
        }
    }));
}

pub fn traverse_doubly_linked_list(c: &mut Criterion) {
    let vec: Vec<_> = (0..10000).collect();
    let list = unsafe_doubly_linked_list::DoublyLinkedList::from_iter(vec.clone());
    c.bench_function("Traverse doubly_linked_list", |b| b.iter(|| {
        let mut n = 0;
        for i in 0..list.len() {
            n += list.get(i).unwrap();
        }
    }));
}

criterion_group!(benches, traverse_singly_linked_list, traverse_doubly_linked_list);
criterion_main!(benches);
