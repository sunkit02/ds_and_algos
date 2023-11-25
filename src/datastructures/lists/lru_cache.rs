use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use super::ring_buffer::RingBuffer;

// TODO: Make `get` and `set` both O(1)
pub struct LRUCache<K, V>
where
    K: Eq + PartialEq + Hash + Clone,
{
    values: RingBuffer<(K, V)>,
    keys: HashSet<K>,
    max_size: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + PartialEq + Hash + Clone + Debug,
    V: PartialEq,
{
    pub fn new(max_size: usize) -> Self {
        return Self {
            values: RingBuffer::with_capacity(max_size + 1),
            keys: HashSet::with_capacity(max_size),
            max_size,
        };
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.keys.contains(key) {
            let index = self.values.find(|(k, _)| k == key).unwrap();
            let value = self.values.remove(index).unwrap();
            self.values.push_front(value);

            return self.values.front().map(|(_key, value)| value);
        } else {
            return None;
        }
    }

    pub fn set(&mut self, key: &K, value: V) {
        if self.keys.contains(key) {
            let index = self.values.find(|(k, _)| k == key).unwrap();
            let (k, _) = self.values.remove(index).expect("Value should exist");
            self.values.push_front((k, value));
        } else {
            if self.size() >= self.max_size {
                self.values.pop_back();
            }

            self.keys.insert(key.clone());
            self.values.push_front((key.clone(), value));
        }
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.keys.clear();
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn max_size(&self) -> usize {
        return self.max_size;
    }

    pub fn as_slice(&mut self) -> &[(K, V)] {
        return self.values.make_contiguous();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_get_and_set() {
        let mut cache: LRUCache<i32, char> = LRUCache::new(3);

        assert_eq!(cache.get(&1), None);

        cache.set(&1, 'a');
        assert_eq!(cache.as_slice(), &[(1, 'a')]);

        cache.set(&2, 'b');
        assert_eq!(cache.as_slice(), &[(2, 'b'), (1, 'a')]);

        cache.set(&3, 'c');
        assert_eq!(cache.as_slice(), &[(3, 'c'), (2, 'b'), (1, 'a')]);

        assert_eq!(cache.get(&2), Some(&'b'));
        assert_eq!(cache.as_slice(), &[(2, 'b'), (3, 'c'), (1, 'a')]);

        cache.set(&4, 'd');
        assert_eq!(cache.as_slice(), &[(4, 'd'), (2, 'b'), (3, 'c')]);

        assert_eq!(cache.get(&4), Some(&'d'));
        assert_eq!(cache.as_slice(), &[(4, 'd'), (2, 'b'), (3, 'c')]);

        assert_eq!(cache.get(&3), Some(&'c'));
        assert_eq!(cache.as_slice(), &[(3, 'c'), (4, 'd'), (2, 'b')]);
    }
}
