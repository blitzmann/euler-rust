
extern crate test;

/*

*/

pub fn solve(max: u64) -> u64 {
    let mut multiple: u64 = 1;
    let mut number: u64 = multiple;
    let mut min: u64 = 1;

    'main: loop { // our main loop. This will check each number from 1 to x to see if it's divisible by 1..=max
        let mut break_for = false;
        'inner: for x in min..=max { // check each int from 1..=max to see if it can evenly divide our number
            if number % x != 0 { // if this number cannot be evenly divided, stop looking 
                break_for = true;
                break 'inner;
            } else {
                multiple = number;
                min = x+1;
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

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| {
        solve(20);
    });
}

#[bench]
fn bench_original(b: &mut test::Bencher) {
    b.iter(|| {
        original_iteration(20);
    });
}