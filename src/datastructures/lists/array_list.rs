use std::{
    alloc::{self, Layout},
    ptr::{self, NonNull},
};

const INITIAL_CAPACITY: usize = 4;

pub struct ArrayList<T> {
    ptr: NonNull<T>,
    capacity: usize,
    len: usize,
}

impl<T> ArrayList<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Out of memory");
        let ptr = unsafe { alloc::alloc(layout.clone()) as *mut T };
        let ptr = NonNull::new(ptr).expect("Failed to allocate memory");

        Self {
            ptr,
            capacity,
            len: 0,
        }
    }

    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let mut list = Self::new();

        while let Some(item) = iter.next() {
            list.push(item);
        }

        return list;
    }

    pub fn push(&mut self, value: T) {
        // This will catch the case where len and capcity are both 0
        if self.len >= self.capacity {
            self.grow();
        }

        debug_assert!(self.len < self.capacity);
        unsafe {
            self.ptr.as_ptr().add(self.len).write(value);
            self.len += 1;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            Some(ptr::read(self.ptr.as_ptr().add(self.len)))
        }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        if self.len >= self.capacity {
            self.grow();
        }

        unsafe {
            // Shift all elements down by one index
            let src = self.ptr.as_ptr().add(index);
            let dst = src.add(1);
            let count = self.len - index;
            ptr::copy(src, dst, count);

            src.write(value);
        }

        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        if index >= self.len {
            panic!("Index out of bounds. Len: {}, Got: {}.", self.len, index);
        } else {
            unsafe {
                let ptr = self.ptr.as_ptr().add(index);
                let target_value = ptr::read(ptr);

                let count = self.len - index - 1;
                ptr::copy::<T>(ptr.add(1), ptr, count);

                self.len -= 1;
                return target_value;
            }
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        debug_assert!(self.len >= 1, "Should have at least one element");
        unsafe { self.ptr.as_ptr().add(index).as_ref() }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        debug_assert!(self.len >= 1, "Should have at least one element");
        unsafe { self.ptr.as_ptr().add(index).as_mut() }
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn len(&self) -> usize {
        return self.len;
    }

    pub fn is_empty(&self) -> bool {
        return self.len == 0;
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let raw_slice = ptr::slice_from_raw_parts(self.ptr.as_ptr(), self.len);

            match raw_slice.as_ref() {
                Some(slice) => slice,
                None => &[],
            }
        }
    }
}

impl<T> ArrayList<T> {
    fn grow(&mut self) {
        let (new_capacity, new_layout) = if self.capacity == 0 {
            (
                INITIAL_CAPACITY,
                Layout::array::<T>(INITIAL_CAPACITY).unwrap(),
            )
        } else {
            let new_capacity = self.capacity * 2;
            let new_layout = Layout::array::<T>(new_capacity).unwrap();
            (new_capacity, new_layout)
        };

        let new_ptr = if self.capacity == 0 {
            unsafe { alloc::alloc(new_layout) as *mut T }
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) as *mut T }
        };

        self.ptr = match NonNull::new(new_ptr) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.capacity = new_capacity;
    }
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        unsafe {
            // Ensure that the `ptr` is not dangling by only dropping if
            // capacity is greater than 0
            if self.capacity > 0 {
                ptr::drop_in_place(ptr::slice_from_raw_parts_mut(self.ptr.as_ptr(), self.len));

                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.capacity).unwrap(),
                );
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_push_and_pop() {
        let mut list = ArrayList::new();
        list.push(1);
        assert_eq!(list.as_slice(), &[1]);

        list.push(2);
        assert_eq!(list.as_slice(), &[1, 2]);

        list.push(3);
        assert_eq!(list.as_slice(), &[1, 2, 3]);

        list.push(4);
        assert_eq!(list.as_slice(), &[1, 2, 3, 4]);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.as_slice(), &[1, 2, 3]);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.as_slice(), &[1, 2]);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.as_slice(), &[1]);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.as_slice(), &[]);

        assert_eq!(list.pop(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.capacity(), 4);
    }

    #[test]
    fn can_grow_capacity() {
        let mut list = ArrayList::new();
        assert_eq!(list.capacity(), 0);

        list.push(1);
        assert_eq!(list.capacity(), 4);
        list.push(2);
        assert_eq!(list.capacity(), 4);
        list.push(3);
        assert_eq!(list.capacity(), 4);
        list.push(4);
        assert_eq!(list.capacity(), 4);

        list.push(5);
        assert_eq!(list.capacity(), 8);
    }

    #[test]
    fn can_get() {
        let list = ArrayList::from_iter([1, 2, 3]);

        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), None);
    }

    #[test]
    fn can_get_mut() {
        let mut list = ArrayList::from_iter([1, 2, 3]);

        assert_eq!(list.get_mut(0), Some(&mut 1));
        assert_eq!(list.get_mut(1), Some(&mut 2));
        assert_eq!(list.get_mut(2), Some(&mut 3));
        assert_eq!(list.get_mut(3), None);

        *list.get_mut(0).unwrap() = 3;
        *list.get_mut(1).unwrap() = 2;
        *list.get_mut(2).unwrap() = 1;

        assert_eq!(list.as_slice(), &[3, 2, 1]);
    }

    #[test]
    fn can_insert() {
        let mut list = ArrayList::from_iter([1, 3, 5]);

        // Insert front
        list.insert(0, 0);
        assert_eq!(list.as_slice(), &[0, 1, 3, 5]);

        // Insert middle
        list.insert(2, 2);
        assert_eq!(list.as_slice(), &[0, 1, 2, 3, 5]);

        // Insert middle
        list.insert(4, 4);
        assert_eq!(list.as_slice(), &[0, 1, 2, 3, 4, 5]);

        // Insert back
        list.insert(6, 6);
        assert_eq!(list.as_slice(), &[0, 1, 2, 3, 4, 5, 6]);

        assert_eq!(list.len(), 7);
    }

    #[test]
    fn can_remove() {
        let mut list = ArrayList::from_iter([1, 2, 3, 4, 5]);

        assert_eq!(list.remove(0), 1); // Remove first
        assert_eq!(list.remove(3), 5); // Remove last
        assert_eq!(list.remove(1), 3); // Remove middle
        assert_eq!(list.remove(0), 2); // Remove first
        assert_eq!(list.remove(0), 4); // Remove first

        assert!(list.is_empty());
        assert_eq!(list.capacity(), 8);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds. Len: 0, Got: 0.")]
    fn can_panic_when_removing_from_empty_list() {
        let mut list = ArrayList::<i32>::new();
        list.remove(0);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds. Len: 3, Got: 3.")]
    fn can_panic_when_removing_out_of_bounds() {
        let mut list = ArrayList::from_iter([1, 2, 3]);
        list.remove(3);
    }
}
