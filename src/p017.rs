extern crate num_bigint;
use english_numbers::convert_all_fmt;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;

fn convert(n: i64) -> usize {
    convert_all_fmt(n)
        .chars()
        .filter(|c| c.is_alphabetic())
        .count()
}

pub fn solve(mut n: u32) -> usize {
    let mut r = 0;
    (1..=n).map(|i| convert(i as i64)).sum()
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn number_test_342() {
        assert_eq!(super::convert(342), 23);
    }

    #[test]
    fn number_test_115() {
        assert_eq!(super::convert(115), 20);
    }

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(5), 19);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(1000), 21124);
    }
}
