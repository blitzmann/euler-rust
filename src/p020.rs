extern crate num_bigint;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;

fn factorial(n: u64) -> BigUint {
    (1..=n).map(|s| s.to_biguint().unwrap()).product()
}

pub fn solve() -> u32 {
    factorial(100)
    .to_str_radix(10)
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .sum::<u32>()
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 648);
    }
}
