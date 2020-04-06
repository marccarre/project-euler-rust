// Make Fibonacci visible as part of the iterators module:
mod fibonacci;
pub use self::fibonacci::Fibonacci;
// Make Primes visible as part of the iterators module:
mod primes;
pub use self::primes::Primes;
// Make Integers visible as part of the iterators module:
mod integers;
pub use self::integers::Integers;
