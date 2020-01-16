extern crate test;
use primal;

pub fn solve(n: usize) -> usize {
    primal::StreamingSieve::nth_prime(n)
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(6), 13);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(10_001), 104743);
    }
}
