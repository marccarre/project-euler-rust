/// Fibonacci infinite sequence iterator.
pub struct Fibonacci {
    current: u32,
    next: u32,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    /// next always returns a value, as this iterator is infinite.
    fn next(&mut self) -> Option<u32> {
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}
