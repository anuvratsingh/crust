use std::cell::UnsafeCell;
// Cell<T> gives ability ti mutate immutabke values
// +Interior Mutability
// -Size Increase
// -Performance
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// impl<T> !Sync for Cell<T> {} //  Implied by UnsafeCell
unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            // SAFETY: None of the references are being invalidated, becuase we never give any out
            // SAFETY: No one can concurrently mutate self.value because !Sync
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: No one modify's this value, since only this thread can mutate (because !Synce), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        std::thread::spawn(move || {
            x1.set(43);
        });
        let x2 = Arc::clone(&x);
        std::thread::spawn(move || {
            x2.set(44);
        });
    }
    // #[test]
    // fn bad2() {
    //     let x = Cell::new(String::from("Hello")); // Allocated Hello
    //     let first = x.get(); // Get the pointer to x pointing at Hello
    //     x.set(String::from("World")); // Change the Hello to World, same pointer points to World now
    //     eprintln!("{}", first);
    // }
}
