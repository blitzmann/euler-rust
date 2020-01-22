// Day of weeek algorithm
// https://www.hackerearth.com/blog/developers/how-to-find-the-day-of-a-week
fn dayofweek(y: i32, m: i32, d: i32) -> i32 {
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = y + if m < 3 {-1} else {0};
    (y + y/4 - y/100 + y/400 + t[(m-1) as usize] + d) % 7
}

pub fn solve() -> u32 {
    let mut r = 0;
    for y in 1901..=2000 {
        for m in 1..=12 {
            if dayofweek(y, m, 1) == 0 {
                r += 1
            }
        }
    }
    r
}

#[cfg(test)]
mod tests {
    extern crate test;

    #[test]
    fn answer_test() {
        assert_eq!(super::solve(), 171);
    }
}
