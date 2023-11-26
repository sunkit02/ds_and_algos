use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct DoublyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }

    fn new_as_ptr(value: T) -> NonNull<Self> {
        return NonNull::from(Box::leak(Box::new(Self::new(value))));
    }
}

impl<T> DoublyLinkedList<T> {
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
            if let Some(prev_node) = self.get_node(index - 1) {
                unsafe {
                    let node = Node::new_as_ptr(value);
                    let next_node = (*prev_node.as_ptr()).next;

                    (*node.as_ptr()).prev = Some(prev_node);
                    (*node.as_ptr()).next = next_node;
                    (*prev_node.as_ptr()).next = Some(node);
                    if let Some(next_node) = next_node {
                        (*next_node.as_ptr()).prev = Some(node);
                    }
                }
            }

            self.len += 1;
        }
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Node::new_as_ptr(value);
        match self.head {
            Some(head) => self.head = Some(Self::link_node_before(head, new_node)),
            None => {
                self.head = Some(new_node);
                self.tail = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Node::new_as_ptr(value);
        match self.tail {
            Some(tail) => self.tail = Some(Self::link_node_after(tail, new_node)),
            None => {
                self.tail = Some(new_node);
                self.head = Some(new_node);
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node()
            .map(|node| unsafe { Box::from_raw(node.as_ptr()).value })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node()
            .map(|node| unsafe { Box::from_raw(node.as_ptr()).value })
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
        match self.unlink_node_at(index) {
            Some(node) => Some(unsafe { Box::from_raw(node.as_ptr()).value }),
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
            vec.push(unsafe { Box::from_raw(node.as_ptr()).value });
        }

        return vec;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next_node: self.head,
            next_back_node: self.tail,
            len: self.len,
            phantom_data: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next_node: self.head,
            next_back_node: self.tail,
            len: self.len,
            phantom_data: PhantomData,
        }
    }
}

// Helper methods
impl<T> DoublyLinkedList<T> {
    fn get_node(&self, index: usize) -> Option<NonNull<Node<T>>> {
        if index >= self.len {
            return None;
        }

        let mut node = None;

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
            // Start traversing from the end closer to the target index
            if index > self.len / 2 {
                let mut itr = self.head;
                let mut i = 0;
                while let Some(itr_node) = itr {
                    if i == index {
                        node = Some(itr_node);
                        break;
                    }

                    itr = unsafe { (*itr_node.as_ptr()).next };
                    i += 1;
                }
            } else {
                let mut itr = self.tail;
                let mut i = self.len - 1;
                while let Some(itr_node) = itr {
                    if i == index {
                        node = Some(itr_node);
                        break;
                    }

                    itr = unsafe { (*itr_node.as_ptr()).prev };
                    i -= 1;
                }
            }
        }

        return node;
    }

    fn pop_front_node(&mut self) -> Option<NonNull<Node<T>>> {
        self.head.map(|prev_head| unsafe {
            if let None = (*prev_head.as_ptr()).next {
                self.tail = None;
            }
            self.head = (*prev_head.as_ptr()).next;
            self.len -= 1;

            return Self::unlink_node(prev_head);
        })
    }

    fn pop_back_node(&mut self) -> Option<NonNull<Node<T>>> {
        self.tail.map(|prev_tail| unsafe {
            if let None = (*prev_tail.as_ptr()).prev {
                self.head = None;
            }
            self.tail = (*prev_tail.as_ptr()).prev;
            self.len -= 1;

            return Self::unlink_node(prev_tail);
        })
    }

    fn unlink_node_at(&mut self, index: usize) -> Option<NonNull<Node<T>>> {
        if index == 0 {
            return self.pop_front_node();
        } else if index == self.len - 1 {
            return self.pop_back_node();
        } else {
            match self.get_node(index) {
                Some(node) => {
                    self.len -= 1;
                    return Some(Self::unlink_node(node));
                }
                None => None,
            }
        }
    }
}

// Helper functions
impl<T> DoublyLinkedList<T> {
    fn unlink_node(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            let prev_node = (*node.as_ptr()).prev;
            let next_node = (*node.as_ptr()).next;

            if let Some(prev) = prev_node {
                (*prev.as_ptr()).next = next_node;
                (*node.as_ptr()).prev = None;
            }

            if let Some(next) = next_node {
                (*next.as_ptr()).prev = prev_node;
                (*node.as_ptr()).next = None;
            }

            return node;
        }
    }

    /// Links `new_node` after node `after` and returns a `NonNull` pointer to `new_node`
    fn link_node_after(after: NonNull<Node<T>>, new_node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            let next_node = (*after.as_ptr()).next;

            if let Some(next) = next_node {
                (*next.as_ptr()).prev = Some(new_node);
            }

            (*new_node.as_ptr()).next = next_node;
            (*new_node.as_ptr()).prev = Some(after);

            (*after.as_ptr()).next = Some(new_node);
        }

        return new_node;
    }

    /// Links `new_node` before node `before` and returns a `NonNull` pointer to `new_node`
    fn link_node_before(before: NonNull<Node<T>>, new_node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        unsafe {
            let prev_node = (*before.as_ptr()).prev;

            if let Some(prev) = prev_node {
                (*prev.as_ptr()).next = Some(new_node);
            }

            (*new_node.as_ptr()).prev = prev_node;
            (*new_node.as_ptr()).next = Some(before);

            (*before.as_ptr()).prev = Some(new_node);
        }

        return new_node;
    }
}

impl<T: Debug> Debug for DoublyLinkedList<T> {
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

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front_node().is_some() {}
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type Item = T;

    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(mut self) -> Self::IntoIter {
        return Self::IntoIter {
            next_node: self.head.take(),
            next_back_node: self.tail.take(),
            len: self.len,
        };
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    next_node: Option<NonNull<Node<T>>>,
    next_back_node: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_node.map(|node| unsafe {
            self.next_node = (*node.as_ptr()).next;
            self.len -= 1;
            return Box::from_raw(node.as_ptr()).value;
        });
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn last(mut self) -> Option<T> {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_back_node.map(|node| unsafe {
            self.next_back_node = (*node.as_ptr()).prev;
            self.len -= 1;
            return Box::from_raw(node.as_ptr()).value;
        });
    }
}

pub struct Iter<'a, T> {
    next_node: Option<NonNull<Node<T>>>,
    next_back_node: Option<NonNull<Node<T>>>,
    len: usize,
    phantom_data: PhantomData<&'a DoublyLinkedList<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_node.map(|node| unsafe {
            self.next_node = (*node.as_ptr()).next;
            self.len -= 1;
            return &node.as_ref().value;
        });
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_back_node.map(|node| unsafe {
            self.next_back_node = (*node.as_ptr()).prev;
            self.len -= 1;
            return &node.as_ref().value;
        });
    }
}

pub struct IterMut<'a, T> {
    next_node: Option<NonNull<Node<T>>>,
    next_back_node: Option<NonNull<Node<T>>>,
    len: usize,
    phantom_data: PhantomData<&'a mut DoublyLinkedList<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_node.map(|mut node| unsafe {
            self.next_node = (*node.as_ptr()).next;
            self.len -= 1;
            return &mut node.as_mut().value;
        });
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn last(mut self) -> Option<&'a mut T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }

        return self.next_back_node.map(|mut node| unsafe {
            self.next_back_node = (*node.as_ptr()).prev;
            self.len -= 1;
            return &mut node.as_mut().value;
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_from_iterator_and_to_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = DoublyLinkedList::from_iter(vec.clone());

        can_walk_forward_and_back(&linked_list);

        assert_eq!(linked_list.to_vec(), vec);
    }

    #[test]
    fn can_push_front() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let mut linked_list = DoublyLinkedList::new();

        for n in vec.iter().rev() {
            linked_list.push_front(*n);
        }

        can_walk_forward_and_back(&linked_list);

        assert_eq!(linked_list.to_vec(), vec);
    }

    #[test]
    fn can_get_value_by_index() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = DoublyLinkedList::from_iter(vec.clone());

        for (idx, n) in vec.iter().enumerate() {
            assert_eq!(linked_list.get(idx), Some(n));
        }
    }

    #[test]
    fn can_get_and_mutate_value_by_index() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let linked_list = DoublyLinkedList::from_iter(vec.clone());

        for idx in 0..linked_list.len() {
            *linked_list.get_mut(idx).unwrap() += 1;
        }

        for (idx, n) in vec.iter().enumerate() {
            assert_eq!(linked_list.get(idx), Some(&(n + 1)));
        }
    }

    #[test]
    fn can_remove() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5, 6]);

        // Call pop_back_node()
        assert_eq!(linked_list.remove(5), Some(6)); // Result: [1, 2, 3, 4, 5]
        can_walk_forward_and_back(&linked_list);

        // Call unlink_node()
        assert_eq!(linked_list.remove(3), Some(4)); // Result: [1, 2, 3, 5]
        can_walk_forward_and_back(&linked_list);

        // Call pop_back_node()
        assert_eq!(linked_list.remove(3), Some(5)); // Result: [1, 2, 3]

        // Call unlink_node()
        assert_eq!(linked_list.remove(1), Some(2)); // Result: [1, 3]

        // Call pop_back_node()
        assert_eq!(linked_list.remove(1), Some(3)); // Result: [1]

        // Call pop_front_node()
        assert_eq!(linked_list.remove(0), Some(1)); // Result: []

        // Empty
        assert_eq!(linked_list.remove(0), None);
        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn can_insert() {
        let mut linked_list = DoublyLinkedList::new();
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

        can_walk_forward_and_back(&linked_list);
    }

    #[test]
    fn can_get_and_pop_front() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3]);

        assert_eq!(linked_list.front(), Some(&1));
        assert_eq!(linked_list.pop_front(), Some(1));
        can_walk_forward_and_back(&linked_list);

        assert_eq!(linked_list.front(), Some(&2));
        assert_eq!(linked_list.pop_front(), Some(2));

        assert_eq!(linked_list.front(), Some(&3));
        assert_eq!(linked_list.pop_front(), Some(3));

        assert_eq!(linked_list.front(), None);
        assert_eq!(linked_list.pop_front(), None);
    }

    #[test]
    fn can_get_and_pop_back() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3]);

        assert_eq!(linked_list.back(), Some(&3));
        assert_eq!(linked_list.pop_back(), Some(3));
        can_walk_forward_and_back(&linked_list);

        assert_eq!(linked_list.back(), Some(&2));
        assert_eq!(linked_list.pop_back(), Some(2));

        assert_eq!(linked_list.back(), Some(&1));
        assert_eq!(linked_list.pop_back(), Some(1));

        assert_eq!(linked_list.back(), None);
        assert_eq!(linked_list.pop_back(), None);
    }

    #[test]
    fn can_clear() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        linked_list.clear();

        assert_eq!(linked_list.front(), None);
        assert_eq!(linked_list.back(), None);
        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn can_into_iter_when_empty() {
        let linked_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut into_iter = linked_list.into_iter();

        assert_eq!(into_iter.next(), None);
        assert_eq!(into_iter.next_back(), None);
    }

    #[test]
    fn can_into_iter_from_the_front() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut into_iter = linked_list.into_iter();

        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), Some(4));
        assert_eq!(into_iter.next(), Some(5));
        assert_eq!(into_iter.next(), None);
    }

    #[test]
    fn can_into_iter_from_the_back() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut into_iter = linked_list.into_iter();

        assert_eq!(into_iter.next_back(), Some(5));
        assert_eq!(into_iter.next_back(), Some(4));
        assert_eq!(into_iter.next_back(), Some(3));
        assert_eq!(into_iter.next_back(), Some(2));
        assert_eq!(into_iter.next_back(), Some(1));
        assert_eq!(into_iter.next_back(), None);
    }

    #[test]
    fn can_into_iter_from_both_sides() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        // [1, 2, 3, 4, 5]
        //  F           B
        let mut into_iter = linked_list.into_iter();

        // [1, 2, 3, 4, 5]
        //  X  F        B
        assert_eq!(into_iter.next(), Some(1));

        // [1, 2, 3, 4, 5]
        //  X  X  F     B
        assert_eq!(into_iter.next(), Some(2));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  B
        assert_eq!(into_iter.next(), Some(3));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  X
        //           B
        assert_eq!(into_iter.next_back(), Some(5));

        // [1, 2, 3, 4, 5]
        //  X  X  X  X  X
        //           F
        //           B
        assert_eq!(into_iter.next_back(), Some(4));

        assert_eq!(into_iter.next_back(), None);
        assert_eq!(into_iter.next(), None);
    }

    #[test]
    fn can_iter_when_empty() {
        let linked_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut iter = linked_list.iter();

        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn can_iter_from_the_front() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter = linked_list.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iter_from_the_back() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter = linked_list.iter();

        assert_eq!(iter.next_back(), Some(&5));
        assert_eq!(iter.next_back(), Some(&4));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn can_iter_from_both_sides() {
        let linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        // [1, 2, 3, 4, 5]
        //  F           B
        let mut iter = linked_list.iter();

        // [1, 2, 3, 4, 5]
        //  X  F        B
        assert_eq!(iter.next(), Some(&1));

        // [1, 2, 3, 4, 5]
        //  X  X  F     B
        assert_eq!(iter.next(), Some(&2));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  B
        assert_eq!(iter.next(), Some(&3));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  X
        //           B
        assert_eq!(iter.next_back(), Some(&5));

        // [1, 2, 3, 4, 5]
        //  X  X  X  X  X
        //           F
        //           B
        assert_eq!(iter.next_back(), Some(&4));

        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iter_mut_when_empty() {
        let mut linked_list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let mut iter_mut = linked_list.iter_mut();

        assert_eq!(iter_mut.next(), None);
        assert_eq!(iter_mut.next_back(), None);
    }

    #[test]
    fn can_iter_mut_from_the_front() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter_mut = linked_list.iter_mut();

        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 4));
        assert_eq!(iter_mut.next(), Some(&mut 5));
        assert_eq!(iter_mut.next(), None);
    }

    #[test]
    fn can_iter_mut_from_the_back() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
        let mut iter_mut = linked_list.iter_mut();

        assert_eq!(iter_mut.next_back(), Some(&mut 5));
        assert_eq!(iter_mut.next_back(), Some(&mut 4));
        assert_eq!(iter_mut.next_back(), Some(&mut 3));
        assert_eq!(iter_mut.next_back(), Some(&mut 2));
        assert_eq!(iter_mut.next_back(), Some(&mut 1));
        assert_eq!(iter_mut.next_back(), None);
    }

    #[test]
    fn can_iter_mut_from_both_sides() {
        let mut linked_list = DoublyLinkedList::from_iter(vec![1, 2, 3, 4, 5]);

        // [1, 2, 3, 4, 5]
        //  F           B
        let mut iter_mut = linked_list.iter_mut();

        // [1, 2, 3, 4, 5]
        //  X  F        B
        assert_eq!(iter_mut.next(), Some(&mut 1));

        // [1, 2, 3, 4, 5]
        //  X  X  F     B
        assert_eq!(iter_mut.next(), Some(&mut 2));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  B
        assert_eq!(iter_mut.next(), Some(&mut 3));

        // [1, 2, 3, 4, 5]
        //  X  X  X  F  X
        //           B
        assert_eq!(iter_mut.next_back(), Some(&mut 5));

        // [1, 2, 3, 4, 5]
        //  X  X  X  X  X
        //           F
        //           B
        assert_eq!(iter_mut.next_back(), Some(&mut 4));

        assert_eq!(iter_mut.next_back(), None);
        assert_eq!(iter_mut.next(), None);
    }

    /// Checks the integrity of all pointers in the linked list
    /// by walking from `self.head` to `self.tail` and ensuring that
    /// the value in all nodes match between each pass
    fn can_walk_forward_and_back<T>(linked_list: &DoublyLinkedList<T>)
    where
        T: Debug + PartialEq,
    {
        let mut values = Vec::with_capacity(linked_list.len);

        // Walk from head to tail
        let mut curr_node = &linked_list.head;
        while let Some(node) = curr_node {
            unsafe {
                values.push(&(*node.as_ptr()).value);
                if (*node.as_ptr()).next.is_some() {
                    curr_node = &(*node.as_ptr()).next;
                } else {
                    break;
                }
            }
        }

        // Assert curr_node is tail
        unsafe {
            assert_eq!(
                (*curr_node.unwrap().as_ptr()),
                (*linked_list.tail.unwrap().as_ptr())
            );
        }

        // Walk from tail to head
        let mut iter = values.into_iter();
        while let Some(node) = curr_node {
            unsafe {
                // verifiy each value
                assert_eq!(
                    &(*node.as_ptr()).value,
                    iter.next_back().expect("Should have value")
                );

                if (*node.as_ptr()).prev.is_some() {
                    curr_node = &(*node.as_ptr()).prev;
                } else {
                    break;
                }
            }
        }

        // Assert curr_node is head
        assert_eq!(*curr_node, linked_list.head);
    }
}
