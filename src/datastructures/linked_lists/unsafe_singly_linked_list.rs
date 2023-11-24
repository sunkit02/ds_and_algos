use std::ptr::NonNull;
use std::fmt::Debug;

pub struct SinglyLinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self { value, next: None }
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn from_iter<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let mut list = Self::new();

        while let Some(value) = iter.next() {
            list.push_back(value);
        }

        return list;
    }

    pub fn insert(&mut self, _value: T, index: usize) {
        if index > self.length {
            panic!("Index out of bounds.");
        }
        todo!()
    }

    pub fn push_front(&mut self, value: T) {
        let node_ptr = Box::leak(Box::new(Node::new(value)));
        let node = NonNull::new(node_ptr);

        match self.head {
            Some(_) => unsafe {
                let node = node.unwrap().as_ptr();
                (*node).next = self.head;
            }
            None => {
                self.tail = node;
            }
        }

        self.head = node;
        self.length += 1;

    }

    pub fn push_back(&mut self, value: T) {
        let node_ptr = Box::leak(Box::new(Node::new(value)));
        let node = NonNull::new(node_ptr);

        match self.tail {
            Some(tail) => unsafe {
                (*tail.as_ptr()).next = node;
            }
            None => {
                self.head = node;
            }
        }

        self.tail = node;
        self.length += 1;
    }

    pub fn get(&self, _index: usize) -> Option<&T> {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }


    pub fn len(&self) -> usize {
        todo!()
    }

    pub fn front(&self) -> Option<&T> {
        todo!()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn back(&self) -> &T {
        todo!()
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.length);
        let mut curr_node = self.head;
        while let Some(node) = curr_node {
            unsafe {
                let node = Box::from_raw(node.as_ptr());
                vec.push(node.value);

                curr_node = node.next;
            }
        }

        return vec;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_from_iterator_and_to_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let iterator = vec.clone().into_iter();

        let linked_list = SinglyLinkedList::from_iter(iterator);

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
}
