use std::fmt::Debug;

use super::errors::LinkedListError;

#[derive(Clone)]
pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn from_iter<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T>,
    {
        let mut list = Self::new();

        while let Some(value) = iter.next() {
            list.insert_back(value);
        }

        return list;
    }

    pub fn insert(&mut self, value: T, index: usize) -> Result<(), LinkedListError> {
        if index > self.length {
            return Err(LinkedListError::IndexOutOfBounds {
                max: self.length,
                given: index,
            });
        }

        if index == 0 {
            self.insert_front(value);
        } else if index == self.length {
            self.insert_back(value);
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

            self.length += 1;
        }

        Ok(())
    }

    pub fn insert_front(&mut self, value: T) {
        let mut new_node = Node { value, next: None };
        match &mut self.head {
            Some(_) => {
                new_node.next = self.head.take();
                self.head = Some(Box::new(new_node));
            }
            None => self.head = Some(Box::new(new_node)),
        }

        self.length += 1;
    }

    pub fn insert_back(&mut self, value: T) {
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

        self.length += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i = 0;
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            if i == index - 1 {
                break;
            }

            curr_node = &node.next;
            i += 1;
        }

        match curr_node.as_deref() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    pub fn clear(&mut self) {
        self.head.take();
        self.length = 0;
    }


    pub fn len(&self) -> usize {
        self.length
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
    use super::SinglyLinkedList;

    #[test]
    fn can_construct_from_iterator() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let iterator = vec.clone().into_iter();

        let linked_list = SinglyLinkedList::from_iter(iterator);

        dbg!(&linked_list);

        assert_eq!(linked_list.to_vec(), vec);
    }
}
