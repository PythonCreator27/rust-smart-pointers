use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: Nobody else is concurrently mutating self.value (since UnsafeCell is !Sync)
        // SAFETY: Nobody can invalidate references, because none are given out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: Nobody else is modifying this value, since only this thread can mutate
        // (since !Sync), and it is executing this function instead
        unsafe { *self.value.get() }
    }
}
