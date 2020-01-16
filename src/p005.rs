pub fn solve(max: u64) -> u64 {
    improved_version(max)
}

pub fn improved_version(max: u64) -> u64 {
    let mut multiple: u64 = 1;
    let mut number: u64 = multiple;
    let mut min: u64 = 1;

    'main: loop {
        // our main loop. This will check each number from 1 to x to see if it's divisible by 1..=max
        let mut break_for = false;
        'inner: for x in min..=max {
            // check each int from 1..=max to see if it can evenly divide our number
            if number % x != 0 {
                // if this number cannot be evenly divided, stop looking
                break_for = true;
                break 'inner;
            } else {
                multiple = number;
                min = x + 1;
            }
        }

        if !break_for {
            break 'main;
        }
        number += multiple;
    }
    number
}

// Original iteration based on my previous solve in Python:
// https://github.com/blitzmann/euler/blob/master/problem5.py
pub fn original_iteration(max: u64) -> u64 {
    /*
       There's a trick to this one. We already know that 2520 is the lowest number
       evenly divided by 1-10. We use this fact for two purposes:
       - Since our result must be evenly divided by 1-10 (and 11-20), the result must
           be a multiple of 2520 (otherwise it would not satisfy 1-10 condition).
       - Since our tests are multiples of 2520, we do not need to test 1-10 as they are
           inherently valid
       Both of these facts greatly speed up the process when compared to brute-forcing
       it.
    */
    let mut multiple = 2520;
    let mut i = multiple;

    'main: loop {
        let mut break_for = false;
        'inner: for x in 10..=max {
            if i % x != 0 {
                break_for = true;
                break 'inner;
            }
        }

        if !break_for {
            break 'main;
        }
        i += multiple;
    }
    i
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn euler_test() {
        assert_eq!(super::solve(10), 2520);
    }

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(20), 232_792_560);
    }

    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| super::solve(20));
    }

    #[bench]
    fn bench_original(b: &mut test::Bencher) {
        b.iter(|| super::original_iteration(20));
    }
}
