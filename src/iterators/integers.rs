// Integers u32 "infinite" iterator.
pub struct Integers {
    i: u32,
}

impl Integers {
    pub fn new() -> Integers {
        Integers { i: 0 }
    }
}

impl Iterator for Integers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.i <= u32::max_value() {
            Some(self.i)
        } else {
            None
        };
        self.i += 1;
        return next;
    }
}
