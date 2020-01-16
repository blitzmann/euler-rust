extern crate test;
use primal;

pub fn solve(n: usize) -> usize {
    primal::StreamingSieve::nth_prime(n)
}
