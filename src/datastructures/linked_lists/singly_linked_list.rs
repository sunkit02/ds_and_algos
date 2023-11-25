use std::fmt::Debug;

#[derive(Clone)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut list = Self::new();
        let mut iter = iter.into_iter();

        while let Some(value) = iter.next() {
            list.push_back(value);
        }

        return list;
    }

    pub fn insert(&mut self, value: T, index: usize) {
        if index > self.len {
            panic!("Index out of bounds.");
        }

        if index == 0 {
            self.push_front(value);
        } else if index == self.len {
            self.push_back(value);
        } else {
            let mut i = 0;
            let mut curr_node = &mut self.head;
            while let Some(node) = curr_node {
                if i == index - 1 {
                    let new_node = Node {
                        value,
                        next: node.next.take(),
                    };
                    node.next = Some(Box::new(new_node));
                    break;
                }

                curr_node = &mut node.next;
                i += 1;
            }

            self.len += 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node { value, next: None };
        match &mut self.head {
            Some(_) => {
                new_node.next = self.head.take();
                self.head = Some(Box::new(new_node));
            }
            None => self.head = Some(Box::new(new_node)),
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Some(Box::new(Node { value, next: None }));
        if self.head.is_some() {
            let mut curr_node = &mut self.head;
            while let Some(node) = curr_node {
                if node.next.is_none() {
                    node.next = new_node;
                    break;
                }
                curr_node = &mut node.next;
            }
        } else {
            self.head = new_node;
        }

        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let next = node.next;
            self.head = next;
            node.value
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        // if self.len == 0 {
        //     return None;
        // } else if self.len == 1 {
        //     return self.pop_front();
        // } else {
        //     let mut curr_node = self.head;
        //     let mut i = 0;
        //     while let Some(node) = curr_node {
        //         if i == self.len() - 1 {
        //             let mut curr_node = curr_node.expect("Node should not be None");
        //             let back_node = curr_node.next.expect("Node should not be None");
        //             curr_node.next = back_node.next;
        //
        //             return Some(back_node.value);
        //         }
        //
        //         curr_node = node.next;
        //         i += 1;
        //     }
        //
        //     return None;
        //
        // }
        todo!("Not sure how to implement");
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut curr_node = &self.head;

        if index > 0 {
            let mut i = 0;
            while let Some(node) = curr_node {
                if i == index {
                    break;
                }

                curr_node = &node.next;
                i += 1;
            }
        }

        match curr_node.as_deref() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn get_mut(&self, _index: usize) -> Option<&mut T> {
        todo!("Don't know if possible")
    }

    pub fn clear(&mut self) {
        self.head.take();
        self.len = 0;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn front(&self) -> Option<&T> {
        self.get(0)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    pub fn back(&self) -> Option<&T> {
        if self.len() >= 1 {
            self.get(self.len() - 1)
        } else {
            None
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.len() >= 1 {
            self.get_mut(self.len() - 1)
        } else {
            None
        }
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len);
        let mut curr_node = self.head;
        while let Some(node) = curr_node {
            vec.push(node.value);

            curr_node = node.next;
        }

        return vec;
    }
}

impl<T: PartialEq> SinglyLinkedList<T> {
    pub fn contains(&self, value: T) -> bool {
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            if node.value == value {
                return true;
            }

            curr_node = &node.next;
        }

        return false;
    }
}

impl<T: Debug> Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            match &node.next {
                Some(_) => f.write_fmt(format_args!("{:?}->", node.value))?,
                None => f.write_fmt(format_args!("{:?}", node.value))?,
            }
            curr_node = &node.next;
        }

        f.write_str("]")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_from_iterator_and_to_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = SinglyLinkedList::from_iter(vec.clone());

        dbg!(&linked_list);

        assert_eq!(linked_list.to_vec(), vec);
    }

    #[test]
    fn can_push_front() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let mut linked_list = SinglyLinkedList::new();

        for n in vec.iter().rev() {
            linked_list.push_front(*n);
        }

        dbg!(&linked_list);

        assert_eq!(linked_list.to_vec(), vec);
    }

    #[test]
    fn can_get_value_by_index() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = SinglyLinkedList::from_iter(vec.clone());

        for (idx, n) in vec.iter().enumerate() {
            dbg!(idx);
            assert_eq!(linked_list.get(idx), Some(n));
        }
    }

    // #[test]
    // fn can_get_and_mutate_value_by_index() {
    //     let vec = vec![1, 2, 3, 4, 5, 6, 7];
    //     let linked_list = SinglyLinkedList::from_iter(vec.clone());
    //
    //     for idx in 0..linked_list.len()  {
    //         *linked_list.get_mut(idx).unwrap() += 1;
    //     }
    //
    //     for (idx, n) in vec.iter().enumerate() {
    //         assert_eq!(linked_list.get(idx), Some(&(n + 1)));
    //     }
    // }

    // #[test]
    // fn can_remove() {
    //     let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3, 4]);
    //
    //     assert_eq!(linked_list.remove(linked_list.len() - 1), Some(4));
    //     assert_eq!(linked_list.remove(linked_list.len() - 1), Some(3));
    //     assert_eq!(linked_list.remove(1), Some(2));
    //     assert_eq!(linked_list.remove(0), Some(1));
    //     assert_eq!(linked_list.remove(0), None);
    //     assert_eq!(linked_list.len(), 0);
    // }

    #[test]
    fn can_insert() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(1, 0); // [1]
        linked_list.insert(3, 1); // [1, 3]
        linked_list.insert(5, 2); // [1, 3, 5]

        linked_list.insert(2, 1); // [1, 2, 3, 5]
        linked_list.insert(4, 3); // [1, 2, 3, 4, 5]

        assert_eq!(linked_list.get(0), Some(&1));
        assert_eq!(linked_list.get(1), Some(&2));
        assert_eq!(linked_list.get(2), Some(&3));
        assert_eq!(linked_list.get(3), Some(&4));
        assert_eq!(linked_list.get(4), Some(&5));
        assert_eq!(linked_list.get(5), None);
    }

    #[test]
    fn can_get_and_pop_front() {
        let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3]);

        assert_eq!(linked_list.front(), Some(&1));
        assert_eq!(linked_list.pop_front(), Some(1));

        assert_eq!(linked_list.front(), Some(&2));
        assert_eq!(linked_list.pop_front(), Some(2));

        assert_eq!(linked_list.front(), Some(&3));
        assert_eq!(linked_list.pop_front(), Some(3));

        assert_eq!(linked_list.front(), None);
        assert_eq!(linked_list.pop_front(), None);
    }

    // #[test]
    // fn can_get_and_pop_back() {
    //     let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3]);
    //
    //     assert_eq!(linked_list.back(), Some(&3));
    //     assert_eq!(linked_list.pop_back(), Some(3));
    //     println!("3, {}", linked_list.len());
    //
    //     assert_eq!(linked_list.back(), Some(&2));
    //     assert_eq!(linked_list.pop_back(), Some(2));
    //     println!("3, {}", linked_list.len());
    //
    //     assert_eq!(linked_list.back(), Some(&1));
    //     assert_eq!(linked_list.pop_back(), Some(1));
    //     println!("3, {}", linked_list.len());
    //
    //     assert_eq!(linked_list.back(), None);
    //     assert_eq!(linked_list.pop_back(), None);
    //     println!("3, {}", linked_list.len());
    // }

    #[test]
    fn can_clear() {
        let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        linked_list.clear();

        assert_eq!(linked_list.front(), None);
        assert_eq!(linked_list.back(), None);
        assert_eq!(linked_list.len(), 0);
    }
}
