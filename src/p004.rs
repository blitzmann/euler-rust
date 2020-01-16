extern crate itertools;
use itertools::Itertools;

pub type Palindrome = u64;

// Test if a given number is a palindrome, MUCH faster than string testing
// Courtesy of https://exercism.io/tracks/rust/exercises/palindrome-products/solutions/1e0e95be77594b5fa22ac7ea79b1bf51
fn is_palindrome(mut number: u64) -> bool {
    // multiple of 10 cannot be a palindrome
    if number % 10 == 0 {
        return false;
    }
    // assemble the reverse of the right half of number
    let mut rebmun: u64 = 0;
    while rebmun < number {
        // push the rightmost digit onto the reverse
        rebmun = 10 * rebmun + number % 10;
        // pop the rightmost digit from the number
        number /= 10;
    }
    // match the left half with the reverse of the right
    // accounting for the possibility of an odd-length number
    number == rebmun || number == rebmun / 10
}

pub fn solve(min: u64, max: u64) -> u64 {
    (min..=max)
        .cartesian_product(min..=max)
        .map(|(j, k)| j * k) // get the product
        .filter(|i| is_palindrome(*i)) // determine if product is palindrome
        .max() // get the max
        .unwrap() // max() returns an Option, unwrap it
}

#[cfg(test)]
mod tests {

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(10, 99), 9009);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(100, 999), 906609);
    }
}
