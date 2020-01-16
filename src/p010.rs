pub fn solve(n: usize) -> u64 {
    primal::Sieve::new(n)
        .primes_from(0)
        .take_while(|x| *x < n) // upper bound can be slightly larger than our limit, so we enforce the limit here
        .map(|n| n as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn euler_test() {
        assert_eq!(super::solve(10), 17);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(2_000_000), 142_913_828_922);
    }
}
