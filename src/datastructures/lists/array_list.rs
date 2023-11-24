use std::{ptr::{NonNull, self}, alloc::{Layout, self}, panic};

pub struct ArrayList<T> {
    buf: NonNull<T>,
    capacity: usize,
    layout: Layout,
    len: usize,
}

impl<T> ArrayList<T> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let layout = Layout::array::<T>(capacity).expect("Out of memory");
        let buf = unsafe {
            let buf_ptr = Box::from_raw(alloc::alloc(layout) as *mut T);
            NonNull::from(Box::leak(buf_ptr))
        };

        Self { buf, capacity, layout, len: 0 }
    }

    pub fn push(&mut self, value: T) {
        if self.len >= self.capacity {
            todo!("Implement realloc");
        }

        unsafe {
            let end = self.buf.as_ptr().add(self.len);
            // FIX: Write access when tag not exist in the borrow stack
            ptr::write(end, value);
            self.len += 1;
        }
    }
     pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        unsafe {
            self.len -= 1;
            Some(ptr::read(self.buf.as_ptr().add(self.len)))
        }
    }

    pub fn insert(&mut self, _index: usize, _value: T) {
        todo!()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            panic!("Index out of bounds.");
        }

        unsafe { self.buf.as_ptr().add(index).as_ref() }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let buf = self.buf.as_ptr();
            let raw_slice = ptr::slice_from_raw_parts(buf, self.len);

            raw_slice.as_ref().expect("Failed to get slice")
        }
    }
}

impl<T> Drop for ArrayList<T> {
    fn drop(&mut self) {
        unsafe {
            // FIX: Memory leak issue on drop
            let _ = Box::from_raw(self.buf.as_ptr()); 
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_push_and_pop() {
        let mut list = ArrayList::with_capacity(10);
        list.push(1);
        assert_eq!(list.as_slice(), &[1]);

        list.push(2);
        assert_eq!(list.as_slice(), &[1, 2]);

        list.push(3);
        assert_eq!(list.as_slice(), &[1, 2, 3]);

        list.pop();
        assert_eq!(list.as_slice(), &[1, 2]);

        list.pop();
        assert_eq!(list.as_slice(), &[1]);

        list.pop();
        assert_eq!(list.as_slice(), &[]);
    }
}
