use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ptr::NonNull;

use crate::datastructures::unsafe_doubly_linked_list::{DoublyLinkedList, Iter, Node};

pub struct LRUCache<'a, K, V>
where
    K: Hash,
{
    values: DoublyLinkedList<(&'a K, V)>,
    map: HashMap<K, NonNull<Node<(&'a K, V)>>>,
    max_size: usize,
}

impl<'a, K, V> LRUCache<'a, K, V>
where
    K: Eq + PartialEq + Hash + Clone + Debug,
    V: PartialEq,
{
    pub fn new(max_size: usize) -> Self {
        return Self {
            values: DoublyLinkedList::new(),
            map: HashMap::with_capacity(max_size),
            max_size,
        };
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            let node = self.map.get(key).unwrap();
            unsafe { self.values.unlink_node_unchecked(*node) };
            self.values.push_front_node(*node);

            return self.values.front().map(|(_, v)| v);
        } else {
            return None;
        }
    }

    pub fn set(&mut self, key: &'a K, value: V) {
        if self.map.contains_key(key) {
            unsafe {
                let node = self.map.get(key).unwrap();
                self.values.unlink_node_unchecked(*node);
                self.values.push_front_node(*node);

                (*node.as_ptr()).value.1 = value;
            }
        } else {
            if self.size() >= self.max_size {
                match self.values.pop_back() {
                    Some((k, _)) => {
                        self.map.remove(k);
                    }
                    None => {}
                }
            }

            let node = Node::new_as_ptr((key, value));
            self.map.insert(key.clone(), node);
            self.values.push_front_node(node);
        }
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.map.clear();
    }

    pub fn size(&self) -> usize {
        return self.values.len();
    }

    pub fn max_size(&self) -> usize {
        return self.max_size;
    }

    pub fn iter(&self) -> Iter<(&K, V)> {
        return self.values.iter();
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
        assert_eq!(
            cache.iter().collect::<Vec<_>>(),
            [(&1, 'a')].iter().collect::<Vec<_>>()
        );

        cache.set(&2, 'b');
        assert!(cache.iter().eq([(&2, 'b'), (&1, 'a')].iter()));

        cache.set(&3, 'c');
        assert!(cache.iter().eq([(&3, 'c'), (&2, 'b'), (&1, 'a')].iter()));

        assert_eq!(cache.get(&2), Some(&'b'));
        assert!(cache.iter().eq([(&2, 'b'), (&3, 'c'), (&1, 'a')].iter()));

        cache.set(&4, 'd');
        assert!(cache.iter().eq([(&4, 'd'), (&2, 'b'), (&3, 'c')].iter()));

        assert_eq!(cache.get(&4), Some(&'d'));
        assert!(cache.iter().eq([(&4, 'd'), (&2, 'b'), (&3, 'c')].iter()));

        assert_eq!(cache.get(&3), Some(&'c'));
        assert!(cache.iter().eq([(&3, 'c'), (&4, 'd'), (&2, 'b')].iter()));
    }
}
