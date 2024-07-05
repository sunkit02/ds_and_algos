use std::{
    hash::{BuildHasher, Hash, RandomState},
    mem,
};

use crate::datastructures::{array_list::ArrayList, unsafe_doubly_linked_list::DoublyLinkedList};

pub const DEFAULT_LOAD_FACTOR: LoadFactor = LoadFactor(0.8);

/// A HashMap that only works on 64 bit systems because the Hasher produces 64 bit hashers
pub struct HashMap<K, V> {
    buckets: ArrayList<Bucket<K, V>>,
    len: usize,
    load_factor: LoadFactor,
    state: RandomState,

    #[cfg(debug_assertions)]
    collisions: u64,
}

impl<K, V> HashMap<K, V>
where
    K: Hash + PartialEq,
{
    pub fn new() -> Self {
        Self {
            buckets: ArrayList::new(),
            len: 0,
            load_factor: DEFAULT_LOAD_FACTOR,
            state: RandomState::default(),

            #[cfg(debug_assertions)]
            collisions: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        let mut buckets = ArrayList::with_capacity(cap);
        for _ in 0..buckets.capacity() {
            buckets.push(Bucket::Empty);
        }

        Self {
            buckets,
            len: 0,
            load_factor: DEFAULT_LOAD_FACTOR,
            state: RandomState::default(),

            #[cfg(debug_assertions)]
            collisions: 0,
        }
    }

    pub fn with_capacity_and_load_factor(cap: usize, load_factor: LoadFactor) -> Self {
        let mut zelf = Self::with_capacity(cap);
        zelf.load_factor = load_factor;

        zelf
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.check_capacity();

        let hash = self.state.hash_one(&key);
        let bucket_index = hash as usize % self.capacity();

        let bucket = &mut self.buckets[bucket_index];

        match bucket {
            Bucket::Filled(EntryBucket { entries, hash: _ }) => {
                #[cfg(debug_assertions)]
                {
                    self.collisions += 1;
                }

                let old_entry = entries
                    .iter_mut()
                    .filter(|entry| entry.key == key)
                    .take(1)
                    .next();

                let new_entry = Entry { key, value };
                match old_entry {
                    Some(old_entry) => {
                        let old_entry = mem::replace(old_entry, new_entry);
                        Some(old_entry.value)
                    }
                    None => {
                        entries.push_back(new_entry);
                        None
                    }
                }
            }
            Bucket::Empty => {
                let new_bucket = {
                    let mut entries = DoublyLinkedList::new();
                    entries.push_front(Entry { key, value });
                    Bucket::Filled(EntryBucket { entries, hash })
                };
                let _ = mem::replace(bucket, new_bucket);
                None
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.state.hash_one(key);
        let bucket_index = hash as usize % self.capacity();

        let bucket = &self.buckets[bucket_index];

        let Bucket::Filled(EntryBucket { entries, hash: _ }) = bucket else {
            return None;
        };

        entries
            .iter()
            .find(|entry| entry.key == *key)
            .map(|entry| &entry.value)
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let hash = self.state.hash_one(key);
        let bucket_index = hash as usize % self.capacity();

        let bucket = &mut self.buckets[bucket_index];

        let Bucket::Filled(EntryBucket { entries, hash: _ }) = bucket else {
            return None;
        };

        entries
            .iter_mut()
            .find(|entry| entry.key == *key)
            .map(|entry| &mut entry.value)
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let hash = self.state.hash_one(key);
        let bucket_index = hash as usize % self.capacity();

        let bucket = &self.buckets[bucket_index];

        let Bucket::Filled(EntryBucket { entries, hash: _ }) = bucket else {
            return false;
        };

        entries.iter().any(|entry| entry.key == *key)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn load_factor(&self) -> LoadFactor {
        self.load_factor
    }

    pub fn set_load_factor(&mut self, new_load_factor: LoadFactor) {
        self.load_factor = new_load_factor;
    }

    pub fn capacity(&self) -> usize {
        self.buckets.capacity()
    }

    fn check_capacity(&mut self) {
        match self.capacity() {
            0 => self.grow(),
            cap => {
                if !((self.len() as f32 / cap as f32) < self.load_factor.get()) {
                    self.grow()
                }
            }
        }
    }

    fn grow(&mut self) {
        unimplemented!();
    }
}

pub enum Bucket<K, V> {
    Filled(EntryBucket<K, V>),
    Empty,
}

pub struct EntryBucket<K, V> {
    entries: DoublyLinkedList<Entry<K, V>>,
    hash: u64,
}

pub struct Entry<K, V> {
    key: K,
    value: V,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LoadFactor(f32);

impl LoadFactor {
    pub fn new(value: f32) -> Result<Self, ()> {
        if Self::is_valid(value) {
            return Err(());
        }
        Ok(Self(value))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, new_value: f32) -> Result<(), ()> {
        if !Self::is_valid(new_value) {
            return Err(());
        }

        self.0 = new_value;
        Ok(())
    }

    fn is_valid(value: f32) -> bool {
        value < 0.1 || value > 1.0
    }
}

impl TryFrom<f32> for LoadFactor {
    type Error = ();

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert_and_get_from_simple_hash_map() {
        let mut map = HashMap::with_capacity(100);
        (0..100).for_each(|num| {
            map.insert(num, num);
        });

        (0..100).for_each(|num| {
            assert_eq!(Some(&num), map.get(&num));
        });

        dbg!(map.collisions);
    }
}
