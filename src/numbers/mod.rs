pub fn is_palindrome(n: &u32) -> bool {
    let mut k = *n;
    let mut reversed = 0;
    while k > 0 {
        let digit = k % 10;
        reversed = reversed * 10 + digit;
        k /= 10;
    }
    return *n == reversed;
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    if (5..=sqrt(n) + 1).step_by(2).any(|k| n % k == 0) {
        return false;
    }
    return true;
}

fn sqrt(n: u32) -> u32 {
    (n as f32).sqrt() as u32
}
