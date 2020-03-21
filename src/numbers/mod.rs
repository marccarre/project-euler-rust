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
