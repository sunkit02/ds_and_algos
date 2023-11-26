use std::fmt::Debug;
use std::ptr::NonNull;

pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T: Debug> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
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

    pub fn insert(&mut self, index: usize, value: T) {
        if index > self.len {
            panic!("Index out of bounds.");
        }

        if index == 0 {
            self.push_front(value);
        } else if index == self.len() {
            self.push_back(value);
        } else {
            let mut node = Node::new(value);
            if let Some(prev_node) = self.get_node(index - 1) {
                unsafe {
                    node.next = (*prev_node.as_ptr()).next;
                    let node = NonNull::from(Box::leak(Box::new(node)));
                    (*prev_node.as_ptr()).next = Some(node);
                }
            }

            self.len += 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node = NonNull::from(Box::leak(node));

        match self.head {
            Some(_) => unsafe {
                let node = node.as_ptr();
                (*node).next = self.head;
            },
            None => {
                self.tail = Some(node);
            }
        }

        self.head = Some(node);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::new(Node::new(value));
        let node = NonNull::from(Box::leak(node));

        match self.tail {
            Some(tail) => unsafe {
                (*tail.as_ptr()).next = Some(node);
            },
            None => self.head = Some(node),
        }

        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.get_node(index) {
            Some(node_ptr) => unsafe { Some(&(*node_ptr.as_ptr()).value) },
            None => None,
        }
    }

    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        match self.get_node(index) {
            Some(node_ptr) => unsafe { Some(&mut (*node_ptr.as_ptr()).value) },
            None => None,
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.unlink_node(index) {
            Some(node) => Some(node.value),
            None => None,
        }
    }

    pub fn clear(&mut self) {
        while self.pop_front_node().is_some() {}
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

    pub fn to_vec(mut self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len);

        while let Some(node) = self.pop_front_node() {
            vec.push(node.value);
        }

        return vec;
    }
}

// private helper functions
impl<T> SinglyLinkedList<T> {
    fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        if index >= self.len {
            return None;
        }

        let mut node = None;

        unsafe {
            if index == 0 {
                match self.head {
                    Some(head) => node = Some(head),
                    None => {}
                }
            } else if index == self.len - 1 {
                match self.tail {
                    Some(tail) => node = Some(tail),
                    None => {}
                }
            } else {
                let mut itr = self.head;
                let mut i = 0;
                while let Some(itr_node) = itr {
                    if i == index {
                        node = Some(itr_node);
                        break;
                    }

                    itr = (*itr_node.as_ptr()).next;
                    i += 1;
                }
            }
        }

        return node;
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|head| unsafe {
            let node = Box::from_raw(head.as_ptr());
            self.head = node.next;

            if let None = self.head {
                self.tail = None;
            }

            self.len -= 1;
            return node;
        })
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        // Take care of cases where `self.length - 2` will cause an integer underflow
        if self.len <= 1 {
            return self.pop_front_node();
        }

        match self.get_node(self.len - 2) {
            Some(prev_node) => unsafe {
                let back_node = (*prev_node.as_ptr()).next;
                (*prev_node.as_ptr()).next = None;
                self.tail = Some(prev_node);

                match back_node {
                    Some(node_ptr) => {
                        self.len -= 1;
                        Some(Box::from_raw(node_ptr.as_ptr()))
                    }
                    None => None,
                }
            },
            None => None,
        }
    }

    fn unlink_node(&mut self, index: usize) -> Option<Box<Node<T>>> {
        if index == 0 {
            return self.pop_front_node();
        } else if index == self.len - 1 {
            return self.pop_back_node();
        } else {
            match self.get_node(index - 1) {
                Some(prev_node) => unsafe {
                    let target_node = (*prev_node.as_ptr()).next.expect("Node should not be null");
                    let next_node = (*target_node.as_ptr()).next;
                    (*target_node.as_ptr()).next = None;
                    (*prev_node.as_ptr()).next = next_node;

                    self.len -= 1;
                    return Some(Box::from_raw(target_node.as_ptr()));
                },
                None => None,
            }
        }
    }
}

impl<T: Debug> Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[")?;
        unsafe {
            let mut curr_node = self.head;
            while let Some(node) = curr_node {
                let node = node.as_ptr();
                match (*node).next {
                    Some(_) => f.write_fmt(format_args!("{:?}->", (*node).value))?,
                    None => f.write_fmt(format_args!("{:?}", (*node).value))?,
                }
                curr_node = (*node).next;
            }
        }

        f.write_str("]")
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
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
            assert_eq!(linked_list.get(idx), Some(n));
        }
    }

    #[test]
    fn can_get_and_mutate_value_by_index() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = SinglyLinkedList::from_iter(vec.clone());

        for idx in 0..linked_list.len() {
            *linked_list.get_mut(idx).unwrap() += 1;
        }

        for (idx, n) in vec.iter().enumerate() {
            assert_eq!(linked_list.get(idx), Some(&(n + 1)));
        }
    }

    #[test]
    fn can_remove() {
        let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3, 4]);

        assert_eq!(linked_list.remove(linked_list.len() - 1), Some(4));
        assert_eq!(linked_list.remove(linked_list.len() - 1), Some(3));
        assert_eq!(linked_list.remove(1), Some(2));
        assert_eq!(linked_list.remove(0), Some(1));
        assert_eq!(linked_list.remove(0), None);
        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn can_insert() {
        let mut linked_list = SinglyLinkedList::new();
        linked_list.insert(0, 1); // [1]
        linked_list.insert(1, 3); // [1, 3]
        linked_list.insert(2, 5); // [1, 3, 5]

        linked_list.insert(1, 2); // [1, 2, 3, 5]
        linked_list.insert(3, 4); // [1, 2, 3, 4, 5]

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

    #[test]
    fn can_get_and_pop_back() {
        let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3]);

        assert_eq!(linked_list.back(), Some(&3));
        assert_eq!(linked_list.pop_back(), Some(3));
        println!("3, {}", linked_list.len());

        assert_eq!(linked_list.back(), Some(&2));
        assert_eq!(linked_list.pop_back(), Some(2));
        println!("3, {}", linked_list.len());

        assert_eq!(linked_list.back(), Some(&1));
        assert_eq!(linked_list.pop_back(), Some(1));
        println!("3, {}", linked_list.len());

        assert_eq!(linked_list.back(), None);
        assert_eq!(linked_list.pop_back(), None);
        println!("3, {}", linked_list.len());
    }

    #[test]
    fn can_clear() {
        let mut linked_list = SinglyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        linked_list.clear();

        assert_eq!(linked_list.front(), None);
        assert_eq!(linked_list.back(), None);
        assert_eq!(linked_list.len(), 0);
    }
}
