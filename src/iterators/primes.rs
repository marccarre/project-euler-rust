/// Primes sequence iterator.
pub struct Primes {
    is_prime: Vec<bool>,
    i: usize,
}

fn sqrt(n: u64) -> usize {
    (n as f64).sqrt() as usize
}

fn primes(bound: usize) -> Vec<bool> {
    let mut is_prime: Vec<bool> = vec![true; bound];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..bound {
        for k in (2 * i..bound).step_by(i) {
            is_prime[k] = false;
        }
    }
    return is_prime;
}

impl Primes {
    pub fn new(n: u64) -> Primes {
        Primes {
            is_prime: self::primes(self::sqrt(n) + 1),
            i: 0,
        }
    }
}

impl Iterator for Primes {
    type Item = u64;

    /// next returns the next prime number if any.
    fn next(&mut self) -> Option<u64> {
        // Move to the next prime number if any:
        while self.i < self.is_prime.len() && !self.is_prime[self.i] {
            self.i += 1;
        }
        if self.i == self.is_prime.len() {
            None
        } else {
            let prime = self.i as u64;
            self.i += 1; // Consume the current prime number.
            Some(prime)
        }
    }
}
