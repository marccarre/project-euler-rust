pub fn problem_0001() -> u32 {
    2 + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_problem_0001() {
        assert_eq!(problem_0001(), 4);
    }
}
