pub fn solve(n: u32) -> u32 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(10), 23);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(1000), 233168);
    }
}
