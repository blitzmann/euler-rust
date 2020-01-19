extern crate num_bigint;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;

fn factorial(n: u64) -> BigUint {
    (1..=n).map(|s| s.to_biguint().unwrap()).product()
}

fn binomial_coefficient(n: u64, k: u64) -> BigUint {
    factorial(n) / (factorial(k) * factorial(n - k))
}

pub fn solve() -> u64 {
    let x = 20; // find 20,20
    binomial_coefficient(x + x, x).to_u64().unwrap()
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 137_846_528_820);
    }
}
