pub fn solve(max: u32) -> u32 {
    (1..=max).sum::<u32>().pow(2) - (1..=max).map(|i| i.pow(2)).sum::<u32>()
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(10), 2640);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(100), 25_164_150);
    }
}
