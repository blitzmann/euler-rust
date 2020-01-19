extern crate num_bigint;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;

pub fn solve(mut n: u32) -> u32 {
    let mut r = 2u32.pow(1).to_biguint().unwrap();
    let two = 2.to_biguint().unwrap();

    let t = (1..n).for_each(|x| r = &r * &two);

    r.to_str_radix(10)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(15), 26);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(1000), 1366);
    }
}
