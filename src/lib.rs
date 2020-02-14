/**
 * Problem 1: Multiples of 3 and 5
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we
 * get 3, 5, 6 and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */
pub fn problem_0001(n: u32) -> u32 {
    let mut sum = 0;
    for k in 1..n {
        if k % 3 == 0 || k % 5 == 0 {
            sum += k
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_problem_0001() {
        assert_eq!(problem_0001(1000), 233168);
    }
}
