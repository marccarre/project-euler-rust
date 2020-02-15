extern crate num_traits;

use num_traits::One;
use std::mem;
use std::ops::Add;

/// Fibonacci infinite sequence iterator.
pub struct Fibonacci<T> {
    current: T,
    next: T,
}

impl<T: One> Fibonacci<T> {
    pub fn new() -> Fibonacci<T> {
        Fibonacci {
            current: One::one(),
            next: One::one(),
        }
    }
}

impl<T: Add<T, Output = T> + Clone> Iterator for Fibonacci<T> {
    type Item = T;

    /// next always returns a value, as this iterator is infinite.
    fn next(&mut self) -> Option<T> {
        let new_next = self.current.clone() + self.next.clone();
        let new_current = mem::replace(&mut self.next, new_next);
        let current = mem::replace(&mut self.current, new_current);
        Some(current)
    }
}
