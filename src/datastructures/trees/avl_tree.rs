use std::fmt::Debug;
use std::mem;
use std::{marker::PhantomData, ptr::NonNull};

#[derive(Debug)]
pub struct AVLTree<T>
where
    T: PartialEq + PartialOrd,
{
    root: Option<NonNull<Node<T>>>,
    len: usize,
    mark: PhantomData<T>,
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq + PartialOrd,
{
    value: T,
    height: u64,
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialEq + PartialOrd,
{
    fn new(value: T) -> Self {
        Self {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn new_as_ptr(value: T) -> NonNull<Self> {
        NonNull::from(Box::leak(Box::new(Self::new(value))))
    }

    fn is_leaf(&self) -> bool {
        return self.left.is_none() && self.right.is_none();
    }
}

/// Public methods
impl<T> AVLTree<T>
where
    T: PartialEq + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            root: None,
            len: 0,
            mark: PhantomData,
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut tree = Self::new();
        iter.into_iter().for_each(|value| tree.insert(value));
        return tree;
    }

    pub fn insert(&mut self, value: T) {
        if let Some(root) = self.root.take() {
            unsafe {
                self.root = Some(Self::insert_node(root, Node::new_as_ptr(value)));
            };
        } else {
            self.root = Some(Node::new_as_ptr(value));
        }

        self.len += 1;
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        if let Some(root) = self.root.take() {
            let (new_root, value) = unsafe {
                let (new_root, value) = Self::remove_node(root, value);
                (new_root, value)
            };
            self.root = new_root;
            self.len -= 1;
            value
        } else {
            None
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        match self.root {
            Some(root) => unsafe {
                match Self::find_node(root, value) {
                    Some(_) => true,
                    None => false,
                }
            },
            None => false,
        }
    }

    pub fn height(&self) -> u64 {
        if let Some(root) = self.root {
            unsafe { root.as_ref().height }
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn inorder_to_vec(mut self) -> Vec<T> {
        unsafe fn inorder_to_vec_recur<T>(node: NonNull<Node<T>>, vec: &mut Vec<T>)
        where
            T: PartialEq + PartialOrd,
        {
            let node = Box::from_raw(node.as_ptr());

            if let Some(left) = node.left {
                inorder_to_vec_recur(left, vec);
            }

            vec.push(node.value);

            if let Some(right) = node.right {
                inorder_to_vec_recur(right, vec);
            }
        }

        let mut vec = Vec::with_capacity(self.len());
        if let Some(root) = self.root.take() {
            unsafe { inorder_to_vec_recur(root, &mut vec) };
        }
        return vec;
    }

    pub fn preorder_to_vec(mut self) -> Vec<T> {
        unsafe fn preorder_to_vec_recur<T>(node: NonNull<Node<T>>, vec: &mut Vec<T>)
        where
            T: PartialEq + PartialOrd,
        {
            let node = Box::from_raw(node.as_ptr());

            vec.push(node.value);

            if let Some(left) = node.left {
                preorder_to_vec_recur(left, vec);
            }

            if let Some(right) = node.right {
                preorder_to_vec_recur(right, vec);
            }
        }

        let mut vec = Vec::with_capacity(self.len());
        if let Some(root) = self.root.take() {
            unsafe { preorder_to_vec_recur(root, &mut vec) };
        }
        return vec;
    }
}

impl<T> AVLTree<T>
where
    T: PartialOrd + PartialEq + Debug,
{
    pub fn debug_print(&self) {
        unsafe fn print_tree_recur<T>(node: NonNull<Node<T>>)
        where
            T: PartialOrd + PartialEq + Debug,
        {
            let value = &(*node.as_ptr()).value;

            let left_value = if let Some(left) = (*node.as_ptr()).left {
                Some(&(*left.as_ptr()).value)
            } else {
                None
            };

            let right_value = if let Some(right) = (*node.as_ptr()).right {
                Some(&(*right.as_ptr()).value)
            } else {
                None
            };

            let height = (*node.as_ptr()).height;
            let balance = AVLTree::get_balance_factor(node);

            println!(
                "(value: {:?}, height: {:?}, balance: {:?}, left: {:?}, right: {:?})",
                value, height, balance, left_value, right_value
            );

            if let Some(left) = (*node.as_ptr()).left {
                print_tree_recur(left);
            }
            if let Some(right) = (*node.as_ptr()).right {
                print_tree_recur(right);
            }
        }

        if let Some(root) = self.root {
            unsafe { print_tree_recur(root) };
        } else {
            println!("Tree is empty");
        }
    }
}

// Helper functions
impl<T> AVLTree<T>
where
    T: PartialEq + PartialOrd,
{
    unsafe fn insert_node(root: NonNull<Node<T>>, node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        if (*node.as_ptr()).value < (*root.as_ptr()).value {
            match (*root.as_ptr()).left {
                Some(left) => {
                    (*root.as_ptr()).left = Some(Self::insert_node(left, node));
                }
                None => (*root.as_ptr()).left = Some(node),
            }
        } else if (*node.as_ptr()).value > (*root.as_ptr()).value {
            match (*root.as_ptr()).right {
                Some(right) => {
                    (*root.as_ptr()).right = Some(Self::insert_node(right, node));
                }
                None => (*root.as_ptr()).right = Some(node),
            }
        } else {
            return node;
        }

        Self::update_height(root);
        return Self::apply_rotation(root);
    }
    /// Returns the new `node` in place of the old `node` and the value of the old node in a tuple
    unsafe fn remove_node(
        root: NonNull<Node<T>>,
        value: &T,
    ) -> (Option<NonNull<Node<T>>>, Option<T>)
    where
        T: PartialEq + PartialOrd,
    {
        let mut ret_value = None;
        if value < &(*root.as_ptr()).value {
            if let Some(left) = (*root.as_ptr()).left {
                let (new_root, value) = Self::remove_node(left, value);
                ret_value = value;
                (*root.as_ptr()).left = new_root;
            }
        } else if value > &(*root.as_ptr()).value {
            if let Some(right) = (*root.as_ptr()).right {
                let (new_root, value) = Self::remove_node(right, value);
                ret_value = value;
                (*root.as_ptr()).right = new_root;
            }
        } else {
            // One child or leaf node
            if (*root.as_ptr()).left.is_none() {
                let root = Box::from_raw(root.as_ptr());
                return (root.right, Some(root.value));
            } else if (*root.as_ptr()).right.is_none() {
                let root = Box::from_raw(root.as_ptr());
                return (root.left, Some(root.value));
            }

            // Two children
            let mut succ_parent = root;
            let mut succ = (*root.as_ptr()).right;

            while let Some(succ_node) = succ {
                if let Some(left) = (*succ_node.as_ptr()).left {
                    succ_parent = succ_node;
                    succ = Some(left);
                } else {
                    break;
                }
            }

            if let Some(succ_node) = succ {
                let succ_node = Box::from_raw(succ_node.as_ptr());
                (*succ_parent.as_ptr()).left = None;
                ret_value = Some(mem::replace(&mut (*root.as_ptr()).value, succ_node.value));
            } else {
                let root = Box::from_raw(root.as_ptr());
                return (None, Some(root.value));
            }
        }

        println!("1");
        Self::update_height(root);
        println!("2");
        let new_root = Self::apply_rotation(root);
        (Some(new_root), ret_value)
    }

    /// Remove the node with the given `value` in the subtree of the given `root` node
    unsafe fn find_node(root: NonNull<Node<T>>, value: &T) -> Option<NonNull<Node<T>>> {
        let root_value = &(*root.as_ptr()).value;

        if value < root_value {
            if let Some(left) = (*root.as_ptr()).left {
                Self::find_node(left, value)
            } else {
                None
            }
        } else if value > root_value {
            if let Some(right) = (*root.as_ptr()).right {
                Self::find_node(right, value)
            } else {
                None
            }
        } else if value == root_value {
            Some(root)
        } else {
            None
        }
    }

    fn update_height(node: NonNull<Node<T>>) {
        fn update_height_recur<T>(node: NonNull<Node<T>>) -> u64
        where
            T: PartialOrd + PartialEq,
        {
            let node = unsafe { node.as_ref() };

            if node.is_leaf() {
                return 1;
            }

            let left_height = if let Some(left) = node.left {
                update_height_recur(left)
            } else {
                0
            };

            let right_height = if let Some(right) = node.right {
                update_height_recur(right)
            } else {
                0
            };

            left_height.max(right_height) + 1
        }

        unsafe {
            (*node.as_ptr()).height = update_height_recur(node);
        }
    }

    unsafe fn apply_rotation(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        // Balancing
        let balance_factor = Self::get_balance_factor(node);

        if balance_factor > 1 {
            // Left heavy, rotate right
            if let Some(left) = (*node.as_ptr()).left {
                if Self::get_balance_factor(left) < 0 {
                    (*node.as_ptr()).left = Some(Self::rotate_left(left));
                }
            }
            return Self::rotate_right(node);
        } else if balance_factor < -1 {
            // Right heavy, rotate left
            if let Some(right) = (*node.as_ptr()).right {
                if Self::get_balance_factor(right) > 0 {
                    (*node.as_ptr()).right = Some(Self::rotate_right(right));
                }
            }
            return Self::rotate_left(node);
        }

        return node;
    }

    unsafe fn rotate_left(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        let right = (*node.as_ptr()).right.expect("Right node should exist");
        let center = (*right.as_ptr()).left;

        (*right.as_ptr()).left = Some(node);
        (*node.as_ptr()).right = center;

        Self::update_height(node);
        Self::update_height(right);

        return right;
    }

    unsafe fn rotate_right(node: NonNull<Node<T>>) -> NonNull<Node<T>> {
        let left = (*node.as_ptr()).left.expect("Left node should exit");
        let center = (*left.as_ptr()).right;

        (*left.as_ptr()).right = Some(node);
        (*node.as_ptr()).left = center;

        Self::update_height(node);
        Self::update_height(left);

        return left;
    }

    unsafe fn get_balance_factor(root: NonNull<Node<T>>) -> i64 {
        unsafe {
            let left_height = if let Some(left) = (*root.as_ptr()).left {
                (*left.as_ptr()).height
            } else {
                0
            };

            let right_height = if let Some(right) = (*root.as_ptr()).right {
                (*right.as_ptr()).height
            } else {
                0
            };

            left_height as i64 - right_height as i64
        }
    }
}

impl<T> Drop for AVLTree<T>
where
    T: PartialEq + PartialOrd,
{
    fn drop(&mut self) {
        unsafe fn drop_recur<T>(node: NonNull<Node<T>>)
        where
            T: PartialEq + PartialOrd,
        {
            if let Some(left) = (*node.as_ptr()).left {
                drop_recur(left);
            }

            if let Some(right) = (*node.as_ptr()).right {
                drop_recur(right);
            }

            let _ = Box::from_raw(node.as_ptr());
        }

        match self.root.take() {
            Some(root) => unsafe { drop_recur(root) },
            None => return,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_insert() {
        let mut nums = [1, 2, 3, 4, 5, 6, -1, -2];
        let tree = AVLTree::from_iter(nums.clone());

        tree.debug_print();

        unsafe {
            assert_eq!((*tree.root.unwrap().as_ptr()).value, 4);
        }

        assert_eq!(tree.height(), 4);

        // Inorder traversal should yield elements sorted
        nums.sort();
        assert_eq!(tree.inorder_to_vec(), nums);
    }

    #[test]
    fn can_check_contains() {
        let nums = [1, 2, 3, 4];
        let tree = AVLTree::from_iter(nums.clone());

        for num in nums {
            assert_eq!(tree.contains(&num), true);
        }

        assert_eq!(tree.contains(&0), false);
    }

    #[test]
    fn can_remove_1() {
        let mut tree = AVLTree::from_iter([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        tree.remove(&4);

        tree.debug_print();

        unsafe {
            assert_eq!((*tree.root.unwrap().as_ptr()).value, 5);
        }

        assert_eq!(tree.len(), 8);
        assert_eq!(tree.preorder_to_vec(), [5, 2, 1, 3, 6, 8, 7, 9]);
    }

    #[test]
    fn can_remove_2() {
        let tree = AVLTree::from_iter([9, 5, 10, 0, 6, 11, -1, 1, 2]);
        tree.debug_print();
        assert_eq!(tree.preorder_to_vec(), [9, 1, 0, -1, 5, 2, 6, 10, 11]);

        let mut tree = AVLTree::from_iter([9, 5, 10, 0, 6, 11, -1, 1, 2]);
        tree.remove(&10);
        assert_eq!(tree.preorder_to_vec(), [1, 0, -1, 9, 5, 2, 6, 11])
    }

    #[test]
    fn can_remove_3() {
        let mut tree = AVLTree::from_iter([9, 8, 15, 7, 13, 20, 10]);
        tree.debug_print();
        tree.remove(&8);

        println!();
        tree.debug_print();

        assert_eq!(tree.preorder_to_vec(), [13, 9, 7, 10, 15, 20])
    }
}
