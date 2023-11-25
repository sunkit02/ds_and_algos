use std::collections::VecDeque;
use std::fmt::Debug;

// TODO: Implement my own `VecDeque` after finishing `ArrayList`
pub struct RingBuffer<T> {
    inner: VecDeque<T>,
}

impl<T> RingBuffer<T> {
    pub fn new() -> Self {
        return Self {
            inner: VecDeque::new(),
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        return Self {
            inner: VecDeque::from_iter(iter)
        };
    }

    pub fn push_front(&mut self, value: T) {
        self.inner.push_front(value);
    }

    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        return self.inner.pop_front();
    }

    pub fn pop_back(&mut self) -> Option<T> {
        return self.inner.pop_back();
    }

    pub fn front(&self) -> Option<&T> {
        return self.inner.front();
    }

    pub fn back(&self) -> Option<&T> {
        return self.inner.back();
    }

    pub fn get(&self, index: usize) -> Option<&T>{
        return self.inner.get(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T>{
        return self.inner.get_mut(index);
    }

    pub fn insert(&mut self, index: usize, value: T) {
        self.inner.insert(index, value);
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        return self.inner.remove(index);
    }

    pub fn clear(&mut self) {
        self.inner.clear()
    }

    pub fn len(&self) -> usize {
        return self.inner.len();
    }

    pub fn to_vec(self) -> Vec<T> {
        return self.inner.into_iter().collect();
    }
}

impl<T: Debug> Debug for RingBuffer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.inner))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_from_iterator_and_to_vec() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let buffer = RingBuffer::from_iter(vec.clone());

        dbg!(&buffer);

        assert_eq!(buffer.to_vec(), vec);
    }

    #[test]
    fn can_push_front() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let mut buffer = RingBuffer::new();

        for n in vec.iter().rev() {
            buffer.push_front(*n);
        }

        dbg!(&buffer);

        assert_eq!(buffer.to_vec(), vec);
    }

    #[test]
    fn can_get_value_by_index() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7];
        let buffer = RingBuffer::from_iter(vec.clone());

        for (idx, n) in vec.iter().enumerate() {
            dbg!(idx);
            assert_eq!(buffer.get(idx), Some(n));
        }
    }

    // #[test]
    // fn can_get_and_mutate_value_by_index() {
    //     let vec = vec![1, 2, 3, 4, 5, 6, 7];
    //     let buffer = RingBuffer::from_iter(vec.clone());
    //
    //     for idx in 0..buffer.len()  {
    //         *buffer.get_mut(idx).unwrap() += 1;
    //     }
    //
    //     for (idx, n) in vec.iter().enumerate() {
    //         assert_eq!(buffer.get(idx), Some(&(n + 1)));
    //     }
    // }

    // #[test]
    // fn can_remove() {
    //     let mut buffer = RingBuffer::from_iter(vec![1, 2, 3, 4]);
    //
    //     assert_eq!(buffer.remove(buffer.len() - 1), Some(4));
    //     assert_eq!(buffer.remove(buffer.len() - 1), Some(3));
    //     assert_eq!(buffer.remove(1), Some(2));
    //     assert_eq!(buffer.remove(0), Some(1));
    //     assert_eq!(buffer.remove(0), None);
    //     assert_eq!(buffer.len(), 0);
    // }

    #[test]
    fn can_insert() {
        let mut buffer = RingBuffer::from_iter(vec![1, 3, 5]);

        buffer.insert(1, 2); // [1, 2, 3, 5]
        buffer.insert(3, 4); // [1, 2, 3, 4, 5]

        assert_eq!(buffer.get(0), Some(&1));
        assert_eq!(buffer.get(1), Some(&2));
        assert_eq!(buffer.get(2), Some(&3));
        assert_eq!(buffer.get(3), Some(&4));
        assert_eq!(buffer.get(4), Some(&5));
        assert_eq!(buffer.get(5), None);
    }

    #[test]
    fn can_get_and_pop_front() {
        let mut buffer = RingBuffer::from_iter(vec![1, 2, 3]);

        assert_eq!(buffer.front(), Some(&1));
        assert_eq!(buffer.pop_front(), Some(1));

        assert_eq!(buffer.front(), Some(&2));
        assert_eq!(buffer.pop_front(), Some(2));

        assert_eq!(buffer.front(), Some(&3));
        assert_eq!(buffer.pop_front(), Some(3));

        assert_eq!(buffer.front(), None);
        assert_eq!(buffer.pop_front(), None);
    }

    // #[test]
    // fn can_get_and_pop_back() {
    //     let mut buffer = RingBuffer::from_iter(vec![1, 2, 3]);
    //
    //     assert_eq!(buffer.back(), Some(&3));
    //     assert_eq!(buffer.pop_back(), Some(3));
    //     println!("3, {}", buffer.len());
    //
    //     assert_eq!(buffer.back(), Some(&2));
    //     assert_eq!(buffer.pop_back(), Some(2));
    //     println!("3, {}", buffer.len());
    //
    //     assert_eq!(buffer.back(), Some(&1));
    //     assert_eq!(buffer.pop_back(), Some(1));
    //     println!("3, {}", buffer.len());
    //
    //     assert_eq!(buffer.back(), None);
    //     assert_eq!(buffer.pop_back(), None);
    //     println!("3, {}", buffer.len());
    // }

    #[test]
    fn can_clear() {
        let mut buffer = RingBuffer::from_iter(vec![1, 2, 3, 4, 5]);

        buffer.clear();

        assert_eq!(buffer.front(), None);
        assert_eq!(buffer.back(), None);
        assert_eq!(buffer.len(), 0);
    }
}
