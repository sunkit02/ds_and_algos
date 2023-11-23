use std::{fmt::Debug, rc::{Weak, Rc}, cell::RefCell, borrow::BorrowMut};
use std::cmp;

#[derive(Debug)]
pub struct BackPointingBinaryTree<T: Debug + PartialEq> {
    root: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

#[derive(Debug)]
struct Node<T: Debug + PartialEq> {
    value: Option<T>,
    parent: Option<Weak<RefCell<Node<T>>>>,
    left: Option<Rc<RefCell<Node<T>>>>,
    right: Option<Rc<RefCell<Node<T>>>>,
}

impl<T: Debug + PartialEq + PartialOrd> BackPointingBinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None, size: 0 }
    }

    pub fn insert(&mut self, value: T) {
        match &self.root {
            Some(root) => {
                Self::recur_insert(root.clone(), value);
            }
            None => {
                self.root = Some(Rc::new(RefCell::new(Node {
                    value: Some(value),
                    parent: None,
                    left: None,
                    right: None,

                })));
            }
        }

        self.size += 1;
    }

    fn recur_insert(root: Rc<RefCell<Node<T>>>, value: T) {
        let mut root_node = root.as_ref().borrow_mut();
        
        if let Some(root_node_value) = &root_node.value {
            if value <= *root_node_value {
                match root_node.left.as_ref().borrow_mut() {
                    Some(left) => Self::recur_insert(left.clone(), value),
                    None => {
                        root_node.left = Some(Rc::new(RefCell::new(Node {
                            value: Some(value),
                            parent: Some(Rc::downgrade(&root)),
                            left: None,
                            right: None,
                        })));
                    },
                }
            } else {
                match root_node.right.as_ref().borrow_mut() {
                    Some(right) => Self::recur_insert(right.clone(), value),
                    None => {
                        root_node.right = Some(Rc::new(RefCell::new(Node {
                            value: Some(value),
                            parent: Some(Rc::downgrade(&root)),
                            left: None,
                            right: None,
                        })));
                    },
                }
            }

        } else {
            panic!("Node.value should not be None!");
        }
    }

    pub fn remove(&mut self, value: T) {
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn height(&self) -> usize {
        match &self.root {
            Some(root) => {
                Self::height_recur(root.clone()) - 1
            }
            None => 0
        }
    }

    fn height_recur(root: Rc<RefCell<Node<T>>>) -> usize {
        let root = root.as_ref().borrow();

        let left_height = match &root.left {
            Some(left_node) =>  Self::height_recur(left_node.clone()),
            None => 0,
        };

        let right_height = match &root.right {
            Some(right_node) =>  Self::height_recur(right_node.clone()),
            None => 0,
        };

        return 1 + cmp::max(left_height, right_height);
    }

    pub fn to_vec_in_order(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.size);

        if let Some(root) = self.root {
            Self::to_vec_in_order_recur(root.clone(), &mut vec);
        }

        return vec;
    }

    fn to_vec_in_order_recur(root: Rc<RefCell<Node<T>>>, vec: &mut Vec<T>) {
        let mut root = root.as_ref().borrow_mut();
        if let Some(left_node) = &root.left {
            Self::to_vec_in_order_recur(left_node.clone(), vec);
        }

        vec.push(root.value.take().unwrap());

        if let Some(right_node) = &root.right {
            Self::to_vec_in_order_recur(right_node.clone(), vec);
        }
    }

    pub fn to_vec_pre_order(self) -> Vec<T> {
        todo!()
    }

    pub fn to_vec_post_order(self) -> Vec<T> {
        todo!()
    }

    pub fn from_iter<I>(iterator: I) -> Self 
    where 
        I: Iterator<Item = T>
    {
        let mut tree = BackPointingBinaryTree::new();
        for item in iterator {
            tree.insert(item);
        }

        return tree;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_in_order_to_vec() {
        let tree = BackPointingBinaryTree::from_iter(vec![6, 4, 8, 2, 5, 1, 3, 7, 9, 10].into_iter());

        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        assert_eq!(tree.to_vec_in_order(), expected);
    }

    #[test]
    fn can_get_height() {
        let tree = BackPointingBinaryTree::from_iter(vec![6, 4, 8, 2, 5, 1, 3, 7, 9, 10].into_iter());

        let expected = 3;

        assert_eq!(tree.height(), expected);
    }
}
