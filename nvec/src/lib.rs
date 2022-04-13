use std::{
    alloc::{self, Layout},
    ptr::NonNull,
};

pub struct NVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> NVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { &*self.ptr.as_ptr().add(index) })
    }

    pub fn push(&mut self, to_push: T) {
        assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");

        if self.capacity == 0 {
            let layout = Layout::array::<T>(4).expect("Not Allocated");
            // SAFETY: The layout is hardcoded to be 4* size_of<T>.
            let ptr = unsafe { alloc::alloc(layout) as *mut T };
            let ptr = NonNull::new(ptr).expect("Couldn't allocate memory");
            // SAFETY: `ptr` is non-null as already enough memory is allocated.
            unsafe { ptr.as_ptr().write(to_push) };
            self.ptr = ptr;
            self.capacity = 4;
            self.len += 1;
        } else if self.len == self.capacity {
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let size = std::mem::size_of::<T>() * self.capacity;
            let align = std::mem::align_of::<T>();
            size.checked_add(size % align).expect("Can't allocate");
            let ptr = unsafe {
                let layout = Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");
                ptr.as_ptr().add(self.len).write(to_push);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory locaiton");
            assert!(self.len <= (isize::MAX / 2).try_into().unwrap()); // Wrap around check
            assert!(offset < isize::MAX as usize); // Wrap around check
                                                   // SAFETY: Memory aloocated is greater than current in used and cannot wrap around
            unsafe { self.ptr.as_ptr().add(self.len).write(to_push) }
            self.len += 1;
        }
    }
}

impl<T> Drop for NVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::NVec;

    #[test]
    fn base_test() {
        let mut vec = NVec::new();
        for i in 1..=20 {
            vec.push(i);
            assert_eq!(vec.get(i - 1), Some(&i));
        }

        assert_eq!(vec.capacity(), 32); // Allocated on the heap of size 2^n
        assert_eq!(vec.len(), 20);
    }

    // Test for zero sizzed struct
}
