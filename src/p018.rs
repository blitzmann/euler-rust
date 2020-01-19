extern crate num_bigint;
use english_numbers::convert_all_fmt;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::cast::ToPrimitive;
/*
    Instead of brute-force, I instead used a method of summing a trianglel described here:
    https://www.youtube.com/watch?v=ZSNWyyaxFYM
*/
const TRIANGLE: &str = "
75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

pub fn solve() -> u32 {
    let mut matrix = TRIANGLE
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .rev() // reverse since we are going to process the triangle from bottom-up
        .collect::<Vec<Vec<u32>>>();

    // couldn't get this to work, was sparring with the borrow checker
    // for (ir, row) in matrix.iter_mut().skip(1).enumerate() {
    //     println!("Before: {:?}", row);
    //     for (ic, cell) in row.iter_mut().enumerate() {
    //         *cell = std::cmp::max(*(mut matrix[ir-1][ic]), *(&mut matrix[ir-1][ic+1]) as u32)
    //     }
    //     // println!("After: {:?}", matrix[level]);
    // }

    for ir in (1..matrix.len()) {
        for ic in (0..matrix[ir].len()) {
            matrix[ir][ic] += std::cmp::max(matrix[ir - 1][ic], matrix[ir - 1][ic + 1])
        }
    }

    matrix.last().unwrap()[0]
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 1074);
    }
}
