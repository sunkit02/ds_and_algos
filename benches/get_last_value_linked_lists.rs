use criterion::{criterion_group, criterion_main, Criterion};
use ds_and_algos::datastructures::{unsafe_doubly_linked_list::DoublyLinkedList, unsafe_singly_linked_list::SinglyLinkedList};

pub fn get_last_safe_list(c: &mut Criterion) {
    let vec: Vec<usize> = (0..1000).collect();
    let list = ds_and_algos::datastructures::singly_linked_list::SinglyLinkedList::from_iter(vec);
    let last_index = list.len() - 1;
    c.bench_function("Getting last element from safe::SinglyLinkedList", |b| {
        b.iter(|| {
            list.get(last_index)
        })
    });
}

pub fn get_last_unsafe_list(c: &mut Criterion) {
    let vec: Vec<usize> = (0..1000).collect();
    let list = SinglyLinkedList::from_iter(vec);
    let last_index = list.len() - 1;
    c.bench_function("Getting last element from unsafe::SinglyLinkedList", |b| {
        b.iter(|| {
            list.get(last_index)
        })
    });
}

pub fn get_last_unsafe_doubly_list(c: &mut Criterion) {
    let vec: Vec<_> = (0..1000).collect();
    let list = DoublyLinkedList::from_iter(vec);
    let last_index = list.len() - 1;
    c.bench_function("Getting last element from unsafe::DoublyLinkedList", |b| {
        b.iter(|| {
            list.get(last_index)
        })
    });
}

criterion_group!(
    benches,
    get_last_safe_list,
    get_last_unsafe_list,
    get_last_unsafe_doubly_list
);
criterion_main!(benches);
